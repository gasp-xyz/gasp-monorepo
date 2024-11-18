use std::time::Duration;

use alloy::sol_types::SolValue;
use futures::StreamExt;
use primitive_types::H256;
use hex::encode as hex_encode;
use tokio::time::timeout;

use crate::l1::{L1Error, L1Interface, types as l1types};
use crate::l2::{types as l2types, HeaderStream, L2Error, L2Interface, PendingUpdate};

pub struct Sequencer<L1, L2> {
    l1: L1,
    l2: L2,
    chain: l2types::Chain,
    limit: u128,
    address: [u8; 20],
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("L1 error")]
    L1Error(#[from] L1Error),
    #[error("L2 error")]
    L2Error(#[from] L2Error),
    #[error("Cannot deserialize the Cancel prtocol message")]
    CancelDeserializationFailure,
    #[error("Update submission failed")]
    UpdateSubmissionFailure,
    #[error("L2Request does not exists")]
    L2RequestDoesNotExists(u128),
}

impl<L1, L2> Sequencer<L1, L2>
where
    L1: L1Interface,
    L2: L2Interface,
{
    pub fn new(l1: L1, l2: L2, chain: l2types::Chain, limit: u128) -> Self {
        let address = l2.address();
        Self { l1, l2, chain, limit , address }
    }

    // consume all items that are available instantely, return on first item 
    // that required more than 1s of waiting
    async fn consume_stream_with_timeout(mut stream: HeaderStream) -> HeaderStream
    {
        loop {
            let result = timeout(Duration::from_secs(1), stream.next()).await;
            match result {
                Ok(Some(Ok((number, hash)))) => { 
                    tracing::debug!("#{} : {} - skipping", number, hex_encode(hash));
                },
                _ => {
                    return stream;
                }
            }
        }
    }

    pub async fn run(&self) -> Result<(), Error> {


        let mut stream = self.l2.finalized_header_stream().await?;
        loop {
            let (number, block_hash) = stream.next().await.expect("infinite stream")?;
            let at = block_hash;

            tracing::info!("#{} : block hash {}", number, hex_encode(block_hash));

            if self.has_cancel_rights_available().await? {
                if let Some(update) = self.find_malicious_update(at).await? {
                    tracing::info!("Found malicious update: {}", update);
                    self.cancel_update(update).await?;
                    stream = Self::consume_stream_with_timeout(stream).await;
                    continue;
                }
            }

            if let Some(closable) = self.find_closable_cancel_resolutions(at).await?.first() {
                tracing::info!("Found pending cancel ready to close : {}", closable);
                self.close_cancel(*closable, at).await?;
                continue;
            }

            if self.has_read_rights_available().await? && self.is_selected_sequencer().await? {
                if let Some((update_hash, update)) = self.get_pending_update(block_hash).await? {
                    tracing::info!("Found update to submit: {:?}", update);
                    let result = self.l2.update_l1_from_l2(update, update_hash).await?;
                    if !result {
                        tracing::error!("update submission failed");
                        return Err(Error::UpdateSubmissionFailure);
                    }else{
                        stream = Self::consume_stream_with_timeout(stream).await;
                    }
                }
            }

        }
    }

    pub async fn close_cancel(&self, request_id: u128, at: H256) -> Result<(), Error> {
        let (merkle_root, range ) = self.l1.get_merkle_root(request_id).await?;
        let proof = self.l2.get_merkle_proof(request_id, range, self.chain.clone(), at).await?;
        let cancel_bytes = self.l2.get_abi_encoded_request(request_id, self.chain.clone(), at).await?;
        let cancel = l1types::Cancel::abi_decode(cancel_bytes.as_ref(), true).or_else(|_| Err(Error::CancelDeserializationFailure))?; 
        self.l1.close_cancel(cancel, merkle_root.into(), proof).await?;
        Ok(())
    }

    pub async fn find_malicious_update(&self, at: H256) -> Result<Option<u128>, Error> {
        let updates = self.l2.get_pending_updates(at).await?;
        let mut updates = updates
            .into_iter()
            .map(|(update_id, update, update_hash)| -> PendingUpdate {
                let min_deposit_id = update
                    .pendingDeposits
                    .iter()
                    .map(|elem| elem.requestId.id)
                    .min()
                    .unwrap_or(u128::MAX);

                let max_deposit_id = update
                    .pendingDeposits
                    .iter()
                    .map(|elem| elem.requestId.id)
                    .max()
                    .unwrap_or(0u128);

                let min_cancel_id = update
                    .pendingCancelResolutions
                    .iter()
                    .map(|elem| elem.requestId.id)
                    .min()
                    .unwrap_or(u128::MAX);

                let max_cancel_id = update
                    .pendingCancelResolutions
                    .iter()
                    .map(|elem| elem.requestId.id)
                    .max()
                    .unwrap_or(0u128);

                PendingUpdate {
                    update_id,
                    chain: update.chain,
                    range: (
                        std::cmp::min(min_deposit_id, min_cancel_id),
                        std::cmp::max(max_deposit_id, max_cancel_id),
                    ),
                    hash: update_hash,
                }
            })
            .filter(|update| update.chain == self.chain)
            .collect::<Vec<_>>();
        updates.sort_by_key(|update| update.update_id);

        let l1handle = &self.l1;

        let mut verified = futures::stream::iter(updates).map(|update| async {
            match l1handle
                .get_update_hash(update.range.0, update.range.1)
                .await{
                    Ok(correct_hash) => Ok((correct_hash, update)),
                    Err(L1Error::InvalidRange) => Ok((H256::zero(), update)),
                    Err(e) => Err(Error::from(e)),
            }
        });

        while let Some(result) = verified.next().await {
            let (correct_hash, update) = result.await?;
            if correct_hash != update.hash {
                return Ok(Some(update.update_id));
            }
        }

        Ok(None)
    }

    pub async fn cancel_update(&self, update_id: u128) -> Result<bool, Error> {
        Ok(self
            .l2
            .cancel_pending_request(update_id, self.chain.clone())
            .await?)
    }

    #[tracing::instrument(skip(self))]
    pub async fn has_read_rights_available(&self) -> Result<bool, Error> {
        let at = self.get_latest_block_hash().await?;
        let read_rights = self.l2.get_read_rights(self.chain.clone(), at).await?;
        tracing::trace!("read rights: {}", read_rights);
        Ok(read_rights > 0)
    }

    #[tracing::instrument(skip(self))]
    pub async fn is_selected_sequencer(&self) -> Result<bool, Error> {
        let at = self.get_latest_block_hash().await?;
        match self.l2.get_selected_sequencer(self.chain.clone(), at).await? {
            Some(selected) if selected == self.address => {
                tracing::debug!("i am selected");
                Ok(true)
            }
            Some(selected) => {
                tracing::debug!("im not the selected sequencer selected({}) vs me({})", hex_encode(selected), hex_encode(self.address));
                Ok(false)
            }
            None =>{
                tracing::debug!("no selcted sequencer");
                Ok(false)
            }
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn has_cancel_rights_available(&self) -> Result<bool, Error> {
        let at = self.get_latest_block_hash().await?;
        let cancel_rights = self.l2.get_cancel_rights(self.chain.clone(), at).await?;
        tracing::trace!("cancel rights: {}", cancel_rights);
        Ok(cancel_rights> 0)
    }

    pub async fn get_latest_block_hash(&self) -> Result<H256, Error> {
        Ok(self.l2.header_stream()
            .await?
            .next()
            .await
            .ok_or(L2Error::HeaderSubscriptionFailed)?
            .map(|(_, hash)| hash)?)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_pending_update(
        &self,
        at: H256,
    ) -> Result<Option<(H256, l2types::L1Update)>, Error> {
        let latest_processed_on_l2 = self.l2.get_latest_processed_request_id(self.chain.clone(), at).await?;
        let latest_request_l1 = self.l1.get_latest_reqeust_id().await?;

        tracing::debug!("latest available on L1: {:?} latest processed on L2 {}", latest_request_l1, latest_processed_on_l2);

        match latest_request_l1 {
            Some(latest_request_l1) if latest_request_l1 > latest_processed_on_l2 => {
                let start = latest_processed_on_l2.saturating_add(1u128);
                let end = std::cmp::min(latest_request_l1, start.saturating_add(self.limit));
                tracing::info!("new requests availabla, fetching range {}..{}", start, end);

                let update = self.l1.get_update(start, end).await?;
                let update_hash = self.l1.get_update_hash(start, end).await?;
                let native_update = self
                    .l2
                    .deserialize_sequencer_update(update.abi_encode())
                    .await?;
                Ok(Some((update_hash, native_update)))
            }
            Some(_) => {
                tracing::debug!("all requests are processed");
                Ok(None)
            }
            _ => {
                tracing::debug!("no requests available yet");
                Ok(None)
            },
        }
    }

    pub async fn find_closable_cancel_resolutions(&self, at: H256) -> Result<Vec<u128>, Error> {
        let latest_closable_request_id = self.l1.get_latest_finalized_request_id().await?;
        if let Some(latest_closable_request_id) = latest_closable_request_id {
            let cancels = self
                .l2
                .get_pending_cancels(self.chain.clone(), at).await?
                .into_iter()
                .filter(|&cancel_request_id| cancel_request_id <= latest_closable_request_id)
                .collect::<Vec<_>>();

            let stream = futures::stream::iter(cancels)
                .map(|cancel_request_id:u128| async move {
                        let hash = self.l2.get_l2_request_hash(cancel_request_id, self.chain.clone(), at).await?
                            .ok_or(Error::L2RequestDoesNotExists(cancel_request_id))?;
                        Ok::<_, Error>((self.l1.is_closed(hash).await?, cancel_request_id))
                }).collect::<Vec<_>>().await;

            let result : Result<Vec<_>, Error> = futures::future::join_all(stream).await.into_iter().collect();
            Ok(result?.into_iter().filter_map(|(closed, request_id)| if !closed { Some(request_id) } else { None }).collect())
        } else {
            Ok(vec![])
        }
    }
}

#[cfg(test)]
pub (crate) mod test {
    use super::*;
    use crate::l1::types as l1types;
    use crate::l2::{types as l2types, PendingUpdateWithKeys, HeaderStream};
    use hex_literal::hex;
    use mockall;
    use mockall::predicate::eq;
    use parity_scale_codec::Decode;
    use primitive_types::H256;
    

    mockall::mock! {
        pub L1 {}

        impl L1Interface for L1{
            async fn get_latest_reqeust_id(&self) -> Result<Option<u128>, L1Error>;
            async fn get_update(&self, start: u128, end: u128) ->  Result<l1types::L1Update, L1Error>;
            async fn get_update_hash(&self, start: u128, end: u128) ->  Result<H256, L1Error>;
            async fn close_cancel(&self, cancel: l1types::Cancel, merkle_root:H256, proof: Vec<H256>) -> Result<H256, L1Error>;
            async fn get_latest_finalized_request_id(&self) -> Result<Option<u128>, L1Error>;
            async fn get_merkle_root(&self, request_id: u128) -> Result<([u8; 32], (u128, u128)), L1Error>;
            async fn is_closed(&self, request_hash: H256) -> Result<bool, L1Error>;
        }
    }

    mockall::mock! {
        pub L2 {}

        impl L2Interface for L2{
            fn address(&self) -> [u8; 20];
            async fn get_latest_processed_request_id(&self, chain: l2types::Chain, at: H256) -> Result<u128, L2Error>;
            async fn get_read_rights(&self, chain: l2types::Chain, at: H256) -> Result<u128, L2Error>;
            async fn get_cancel_rights(&self, chain: l2types::Chain, at: H256) -> Result<u128, L2Error>;
            async fn get_pending_updates(&self, at: H256) -> Result<Vec<PendingUpdateWithKeys>, L2Error>;
            async fn deserialize_sequencer_update(&self, data: Vec<u8>) -> Result<l2types::L1Update, L2Error>;
            async fn cancel_pending_request(&self, request_id: u128, chain: l2types::Chain) -> Result<bool, L2Error>;
            async fn update_l1_from_l2(&self, update: l2types::L1Update, hash: H256) -> Result<bool, L2Error>;
            async fn get_pending_cancels( &self, chain: l2types::Chain, at: H256) -> Result<Vec<u128>, L2Error>;
            async fn get_merkle_proof( &self, request_id: u128, range : (u128, u128), chain: l2types::Chain, at: H256) -> Result<Vec<H256>, L2Error>;
            async fn get_l2_request_hash( &self, request_id: u128, chain: l2types::Chain, at: H256) -> Result<Option<H256>, L2Error>;
            async fn header_stream( &self) -> Result<HeaderStream, L2Error>;
            async fn finalized_header_stream( &self) -> Result<HeaderStream, L2Error>;
            async fn get_selected_sequencer( &self, chain: l2types::Chain, at: H256) -> Result<Option<[u8; 20]>, L2Error>;
            async fn get_abi_encoded_request( &self, request_id : u128, chain: l2types::Chain, at: H256) -> Result<Vec<u8>, L2Error>;
        }
    }

    const DUMMY_ADDRESS: [u8; 20] = hex!("0000000000000000000000000000000000000000");
    const ETHEREUM: l2types::Chain = l2types::Chain::Ethereum;
    const ARBITRUM: l2types::Chain = l2types::Chain::Arbitrum;

    #[allow(dead_code)]
    pub enum Request {
        Deposit(l2types::Deposit),
        Cancel(l2types::CancelResolution),
    }

    impl From<l2types::Deposit> for Request {
        fn from(d: l2types::Deposit) -> Self {
            Self::Deposit(d)
        }
    }

    pub struct UpdateBuilder(Vec<Request>);

    pub fn to_u256(value: u128) -> l2types::bindings::runtime_types::primitive_types::U256 {
        let x = primitive_types::U256::from(value);
        let data = x.to_big_endian();
        l2types::bindings::runtime_types::primitive_types::U256::decode(&mut &data[..]).unwrap()
    }

    impl UpdateBuilder {
        pub fn new() -> Self {
            Self(vec![])
        }

        pub fn with_dummy_deposit(self, rid: u128) -> Self {
            self.with_request(
                l2types::Deposit {
                    requestId: l2types::RequestId {
                        origin: l2types::Origin::L1,
                        id: rid,
                    },
                    depositRecipient: DUMMY_ADDRESS,
                    tokenAddress: DUMMY_ADDRESS,
                    amount: to_u256(100u128),
                    timeStamp: to_u256(0u128),
                    ferryTip: to_u256(0u128),
                }
                .into(),
            )
        }

        pub fn with_request(mut self, r: Request) -> Self {
            self.0.push(r);
            self
        }

        pub fn build(self, chain: l2types::Chain) -> l2types::L1Update {
            let mut result = l2types::L1Update {
                chain,
                pendingDeposits: vec![],
                pendingCancelResolutions: vec![],
            };

            for elem in self.0.into_iter() {
                match elem {
                    Request::Deposit(d) => {
                        result.pendingDeposits.push(d);
                    }
                    Request::Cancel(c) => {
                        result.pendingCancelResolutions.push(c);
                    }
                }
            }

            result
        }
    }

    #[tokio::test]
    async fn test_find_malicious_update_ignores_valid_updates() {
        let update_hash = H256::zero();
        let correct_hash = update_hash.clone();

        let update = UpdateBuilder::new().with_dummy_deposit(1u128).build(ETHEREUM);
        let pending: PendingUpdateWithKeys = (1u128, update, update_hash);

        let mut l1mock = MockL1::new();
        l1mock
            .expect_get_update_hash()
            .with(eq(1u128), eq(1u128))
            .times(1)
            .returning(move |_, _| Ok(correct_hash));

        let mut l2mock = MockL2::new();
        l2mock.expect_address().return_const(DUMMY_ADDRESS);
        l2mock
            .expect_get_pending_updates()
            .times(1)
            .return_once(move |_| Ok(vec![pending]));

        let sequencer = Sequencer::new(l1mock, l2mock, ETHEREUM, 100u128);

        assert_eq!(
            sequencer.find_malicious_update(H256::zero()).await.unwrap(),
            None
        );
    }

    #[tokio::test]
    async fn test_find_malicious_update_ignores_updates_from_other_chains() {
        let update_hash = H256::zero();
        let update = UpdateBuilder::new().with_dummy_deposit(1u128).build(ARBITRUM);
        let pending: PendingUpdateWithKeys = (1u128, update, update_hash);

        let mut l1mock = MockL1::new();
        l1mock.expect_get_update_hash().times(0);

        let mut l2mock = MockL2::new();
        l2mock.expect_address().return_const(DUMMY_ADDRESS);
        l2mock
            .expect_get_pending_updates()
            .times(1)
            .return_once(move |_| Ok(vec![pending]));

        let sequencer = Sequencer::new(l1mock, l2mock, ETHEREUM, 100u128);

        assert_eq!(
            sequencer.find_malicious_update(H256::zero()).await.unwrap(),
            None
        );
    }

    #[tokio::test]
    async fn test_find_malicious_update_works() {
        let update_hash = H256::from(hex!(
            "1111111111111111111111111111111111111111111111111111111111111111"
        ));
        let correct_hash = H256::zero();

        let update = UpdateBuilder::new().with_dummy_deposit(1u128).build(ETHEREUM);
        let pending: PendingUpdateWithKeys = (1u128, update, update_hash);

        let mut l1mock = MockL1::new();
        l1mock
            .expect_get_update_hash()
            .with(eq(1u128), eq(1u128))
            .times(1)
            .returning(move |_, _| Ok(correct_hash));

        let mut l2mock = MockL2::new();
        l2mock.expect_address().return_const(DUMMY_ADDRESS);
        l2mock
            .expect_get_pending_updates()
            .times(1)
            .return_once(move |_| Ok(vec![pending]));

        let sequencer = Sequencer::new(l1mock, l2mock, ETHEREUM, 100u128);

        assert_eq!(
            sequencer.find_malicious_update(H256::zero()).await.unwrap(),
            Some(1u128)
        );
    }

    #[tokio::test]
    async fn test_find_pending_cancels_to_close() {
        let mut l1mock = MockL1::new();
        l1mock
            .expect_get_latest_finalized_request_id()
            .return_once(|| Ok(Some(1u128)));
        l1mock.expect_is_closed().returning(|_| Ok(false));

        let mut l2mock = MockL2::new();
        l2mock.expect_address().return_const(DUMMY_ADDRESS);
        l2mock
            .expect_get_pending_cancels()
            .return_once(|_, _| Ok(vec![1u128, 2u128]));
        l2mock.expect_get_l2_request_hash().returning(|_,_,_| Ok(Some(H256::zero())));

        let sequencer = Sequencer::new(l1mock, l2mock, ETHEREUM, 100u128);
        let result = sequencer
            .find_closable_cancel_resolutions(H256::zero())
            .await;

        assert_eq!(result.unwrap(), vec![1u128]);
    }

    #[tokio::test]
    async fn test_find_pending_cancels_to_close2() {
        let mut l1mock = MockL1::new();
        l1mock
            .expect_get_latest_finalized_request_id()
            .return_once(|| Ok(Some(10u128)));
        l1mock.expect_is_closed().returning(|_| Ok(false));

        let pending_cancels = vec![1u128, 2u128, 10u128];
        let cancels = pending_cancels.clone();

        let mut l2mock = MockL2::new();
        l2mock.expect_address().return_const(DUMMY_ADDRESS);
        l2mock.expect_get_l2_request_hash().returning(|_,_,_| Ok(Some(H256::zero())));
        l2mock
            .expect_get_pending_cancels()
            .return_once(|_, _| Ok(cancels));

        let sequencer = Sequencer::new(l1mock, l2mock, ETHEREUM, 100u128);
        let result = sequencer
            .find_closable_cancel_resolutions(H256::zero())
            .await;

        assert_eq!(result.unwrap(), pending_cancels);
    }

    #[tokio::test]
    async fn test_find_pending_cancels_ignores_closed_cancels() {
        let at = H256::zero();
        let first_request_hash = H256::from(hex!("0000000000000000000000000000000000000000000000000000000000000001"));
        let second_request_hash = H256::from(hex!("0000000000000000000000000000000000000000000000000000000000000002"));
        let third_request_hash = H256::from(hex!("0000000000000000000000000000000000000000000000000000000000000003"));

        let mut l1mock = MockL1::new();
        l1mock
            .expect_get_latest_finalized_request_id()
            .return_once(|| Ok(Some(10u128)));
        l1mock.expect_is_closed().with(eq(first_request_hash.clone())).returning(|_| Ok(true));
        l1mock.expect_is_closed().with(eq(second_request_hash.clone())).returning(|_| Ok(false));
        l1mock.expect_is_closed().with(eq(third_request_hash.clone())).returning(|_| Ok(true));

        let pending_cancels = vec![1u128, 2u128, 10u128];
        let cancels = pending_cancels.clone();

        let mut l2mock = MockL2::new();
        l2mock.expect_address().return_const(DUMMY_ADDRESS);
        l2mock.expect_get_l2_request_hash()
            .with(eq(pending_cancels[0]), eq(ETHEREUM), eq(at))
            .returning(move |_,_,_| Ok(Some(first_request_hash)));
        l2mock.expect_get_l2_request_hash()
            .with(eq(pending_cancels[1]), eq(ETHEREUM), eq(at))
            .returning(move |_,_,_| Ok(Some(second_request_hash)));
        l2mock.expect_get_l2_request_hash()
            .with(eq(pending_cancels[2]), eq(ETHEREUM), eq(at))
            .returning(move |_,_,_| Ok(Some(third_request_hash)));

        l2mock
            .expect_get_pending_cancels()
            .return_once(|_, _| Ok(cancels));

        let sequencer = Sequencer::new(l1mock, l2mock, ETHEREUM, 100u128);
        let result = sequencer
            .find_closable_cancel_resolutions(at)
            .await;

        assert_eq!(result.unwrap(), vec![2u128]);
    }


    #[tokio::test]
    async fn test_find_pending_cancels_to_close_when_there_is_no_merkle_root_provided_to_l1() {
        let mut l1mock = MockL1::new();
        l1mock
            .expect_get_latest_finalized_request_id()
            .return_once(|| Ok(None));

        let mut l2mock = MockL2::new();
        l2mock.expect_address().return_const(DUMMY_ADDRESS);
        l2mock.expect_get_pending_cancels().times(0);

        let sequencer = Sequencer::new(l1mock, l2mock, ETHEREUM, 100u128);
        let result = sequencer
            .find_closable_cancel_resolutions(H256::zero())
            .await;

        assert_eq!(result.unwrap(), vec![]);
    }

    #[tokio::test]
    async fn test_get_pending_update_when_there_are_no_requests() {
        let mut l1mock = MockL1::new();
        l1mock
            .expect_get_latest_reqeust_id()
            .return_once(|| Ok(None));

        let mut l2mock = MockL2::new();
        l2mock.expect_address().return_const(DUMMY_ADDRESS);
        l2mock.expect_get_latest_processed_request_id()
            .return_once(|_,_| Ok(0u128));

        l1mock.expect_get_update().times(0);

        let sequencer = Sequencer::new(l1mock, l2mock, ETHEREUM, 100u128);

        let update = sequencer.get_pending_update(H256::zero()).await;
        assert!(matches!(update, Ok(None)));
    }

    #[tokio::test]
    async fn test_get_pending_update_when_there_are_requests() {
        let mut l1mock = MockL1::new();
        l1mock
            .expect_get_latest_reqeust_id()
            .return_once(|| Ok(Some(10u128)));

        let mut l2mock = MockL2::new();
        l2mock.expect_address().return_const(DUMMY_ADDRESS);
        l2mock.expect_get_latest_processed_request_id()
            .return_once(|_,_| Ok(0u128));

        let update = l1types::L1Update{
            chain: Default::default(),
            pendingDeposits : vec![],
            pendingCancelResolutions: vec![],
        };


        l1mock.expect_get_update()
            .times(1)
            .with(eq(1u128), eq(10u128))
            .return_once(|_, _| Ok(update));

        l1mock.expect_get_update_hash()
            .times(1)
            .with(eq(1u128), eq(10u128))
            .return_once(|_, _| Ok(H256::zero()));

        l2mock.expect_deserialize_sequencer_update()
            .times(1)
            .return_once(|_| Ok(UpdateBuilder::new().build(ETHEREUM)));

        let sequencer = Sequencer::new(l1mock, l2mock, ETHEREUM, 100u128);

        sequencer.get_pending_update(H256::zero()).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_pending_update_when_there_are_too_many_requests_for_single_update() {

        let mut l1mock = MockL1::new();
        l1mock
            .expect_get_latest_reqeust_id()
            .return_once(|| Ok(Some(1000u128)));

        let mut l2mock = MockL2::new();
        l2mock.expect_address().return_const(DUMMY_ADDRESS);
        l2mock.expect_get_latest_processed_request_id()
            .return_once(|_,_| Ok(0u128));

        let update = l1types::L1Update{
            chain: Default::default(),
            pendingDeposits : vec![],
            pendingCancelResolutions: vec![],
        };


        l1mock.expect_get_update()
            .times(1)
            .with(eq(1u128), eq(101u128))
            .return_once(|_, _| Ok(update));

        l1mock.expect_get_update_hash()
            .times(1)
            .with(eq(1u128), eq(101u128))
            .return_once(|_, _| Ok(H256::zero()));

        l2mock.expect_deserialize_sequencer_update()
            .times(1)
            .return_once(|_| Ok(UpdateBuilder::new().build(ETHEREUM)));

        let sequencer = Sequencer::new(l1mock, l2mock, ETHEREUM, 100u128);

        sequencer.get_pending_update(H256::zero()).await.unwrap();
    }


    #[tokio::test]
    async fn test_find_malicious_update_with_invalid_range_works() {
        let update_hash = H256::from(hex!(
            "1111111111111111111111111111111111111111111111111111111111111111"
        ));
        let update = UpdateBuilder::new().with_dummy_deposit(1u128).build(ETHEREUM);
        let pending: PendingUpdateWithKeys = (33u128, update, update_hash);

        let mut l1mock = MockL1::new();
        l1mock
            .expect_get_update_hash()
            .with(eq(1u128), eq(1u128))
            .times(1)
            .returning(move |_, _| Err(L1Error::InvalidRange));

        let mut l2mock = MockL2::new();
        l2mock.expect_address().return_const(DUMMY_ADDRESS);
        l2mock
            .expect_get_pending_updates()
            .times(1)
            .return_once(move |_| Ok(vec![pending]));

        let sequencer = Sequencer::new(l1mock, l2mock, ETHEREUM, 100u128);

        assert_eq!(
            sequencer.find_malicious_update(H256::zero()).await.unwrap(),
            Some(33u128)
        );
    }




}

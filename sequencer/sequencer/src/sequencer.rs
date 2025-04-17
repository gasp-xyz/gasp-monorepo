use std::time::Duration;

use alloy::sol_types::SolValue;
use futures::{FutureExt, StreamExt};
use gasp_types::PendingUpdate;
use hex::encode as hex_encode;
use primitive_types::H256;
use tokio::sync::mpsc::Sender;
use tokio::time::timeout;

use l1api::{types::RequestStatus, L1Error, L1Interface};
use l2api::{HeaderStream, L2Error, L2Interface};
// use crate::l1::{types as l1types, L1Error, L1Interface};
// use crate::l2::{types as l2types, HeaderStream, L2Error, L2Interface, PendingUpdate};

const ALERT_ERROR: &str = "ALERT::ERROR";
const ALERT_WARNING: &str = "ALERT::WARNING";
const ALERT_INFO: &str = "ALERT::INFO";

use prometheus::{opts, register_counter_vec, CounterVec};

lazy_static::lazy_static! {
    static ref SEQUENCER_ACTION: CounterVec = register_counter_vec!(
        opts!("action", "number of sequencer actions"),
        &["type"]
    )
    .unwrap();
}

pub struct Sequencer<L1, L2> {
    l1: L1,
    l2: L2,
    chain: gasp_types::Chain,
    limit: u128,
    l1_account_address: [u8; 20],
    l2_account_address: [u8; 20],
    tx_cost: Option<u128>,
}

#[derive(Debug)]
pub enum ActionStatus {
    Performed,
    Skipped,
    NothingToDo,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("L1 error")]
    L1(#[from] L1Error),
    #[error("L2 error")]
    L2(#[from] L2Error),
    #[error("Cannot deserialize the Cancel prtocol message")]
    CancelDeserializationFailure,
    #[error("Update submission failed")]
    UpdateSubmissionFailure,
    #[error("L2Request does not exists")]
    L2RequestDoesNotExists(u128),
    #[error("This account is not a sequener")]
    NotASequencer,
    #[error("Account balance dropped below required minimum")]
    NotEnoughtBalance,
    #[error("Unknown merkle root")]
    UnknownMerkleRoot,
    #[error("Reqeust `{request_id:?}` not found for chain `{chain:?}`")]
    CancelIdDoesNotExistsOnL2 {
        request_id: gasp_types::U256,
        chain: gasp_types::Chain,
    },
}

impl<L1, L2> Sequencer<L1, L2>
where
    L1: L1Interface,
    L2: L2Interface,
{
    pub fn new(
        l1: L1,
        l2: L2,
        chain: gasp_types::Chain,
        l1_account: [u8; 20],
        l2_account: [u8; 20],
        limit: u128,
        tx_cost: Option<u128>,
    ) -> Self {
        // TODO: pass addresses in ctor
        Self {
            l1,
            l2,
            chain,
            limit,
            l1_account_address: l1_account,
            l2_account_address: l2_account,
            tx_cost,
        }
    }

    // consume all items that are available instantely, return on first item
    // that required more than 1s of waiting
    async fn consume_stream_with_timeout(mut stream: HeaderStream) -> HeaderStream {
        loop {
            let result = timeout(Duration::from_secs(1), stream.next()).await;
            match result {
                Ok(Some(Ok((number, hash)))) => {
                    tracing::debug!("#{} : {} - skipping", number, hex_encode(hash));
                }
                _ => {
                    return stream;
                }
            }
        }
    }

    pub async fn cancel_malicious_update_if_any(&self, at: H256) -> Result<ActionStatus, Error> {
        if let Some(update) = self.find_malicious_update(at).await? {
            tracing::warn!("{ALERT_ERROR} Found malicious update: {}", update);
            if self.has_cancel_rights_available().await? {
                SEQUENCER_ACTION
                    .with_label_values(&["cancel_malicious_update"])
                    .inc();
                self.cancel_update(update)
                    .await
                    .inspect(|result| {
                        if *result {
                            tracing::warn!("{ALERT_ERROR} cancel malicious update has succeded");
                        } else {
                            tracing::error!("{ALERT_ERROR} cancel malicious update has failed");
                        }
                    })
                    .inspect_err(|_| {
                        tracing::error!("{ALERT_ERROR} cancel malicious update has failed");
                    })?;
                return Ok(ActionStatus::Performed);
            } else {
                tracing::error!(
                    "{ALERT_ERROR} there are no cancel rights available to cancel malicous update"
                );
                return Ok(ActionStatus::Skipped);
            }
        }
        Ok(ActionStatus::NothingToDo)
    }

    pub async fn should_send_update_already(
        &self,
        now: u128,
        sequencers_count: usize,
        dispute_period_length: u128,
        last_update: Option<u128>,
    ) -> Result<bool, L1Error> {
        let sequencers_count: u128 = sequencers_count as u128;
        match (last_update, sequencers_count) {
            (None, _) => {
                tracing::info!("there are no pending updates, proceeding...");
                Ok(true)
            }
            (Some(_), ..=1) => {
                tracing::info!("there is just one sequencer, proceeding...");
                Ok(true)
            }
            (Some(latest), _)
                if now.saturating_sub(latest) > (dispute_period_length / sequencers_count) =>
            {
                tracing::info!("previous update was long enough ago, proceeding...");
                Ok(true)
            }
            _ => {
                tracing::info!("previous pending update found, no need to provide update yet");
                Ok(false)
            }
        }
    }

    pub async fn send_sequencer_update(
        &self,
        block_number: u32,
        at: H256,
    ) -> Result<ActionStatus, Error> {
        let has_read_rights = self.has_read_rights_available().await?;
        let is_selected_sequencer = self.is_selected_sequencer().await?;
        if !has_read_rights || !is_selected_sequencer {
            return Ok(ActionStatus::Skipped);
        }

        let dispute_period = self.l2.get_dispute_period(self.chain, at).await?;
        let sequencers_count = self.l2.get_active_sequencers(self.chain, at).await?.len();
        let latest_update_block_time = self.find_latest_correct_update_block_submission(at).await?;

        let should_send_update = self
            .should_send_update_already(
                block_number.into(),
                sequencers_count,
                dispute_period,
                latest_update_block_time,
            )
            .await?;

        if !should_send_update {
            return Ok(ActionStatus::Skipped);
        }

        if let Some((update_hash, update)) = self.get_pending_update(at).await? {
            tracing::info!("Found update to submit: {:?}", update);
            SEQUENCER_ACTION
                .with_label_values(&["sequencer_update"])
                .inc();
            let result = self.l2.update_l1_from_l2(update, update_hash).await?;
            if !result {
                tracing::error!("{ALERT_WARNING} update submission failed");
                Err(Error::UpdateSubmissionFailure)
            } else {
                tracing::info!("{ALERT_INFO} update submission succeded");
                Ok(ActionStatus::Performed)
            }
        } else {
            Ok(ActionStatus::Skipped)
        }
    }

    pub async fn retrieve_cancel_rights(&self, at: H256) -> Result<ActionStatus, Error> {
        let balance = self.get_my_balance().await?;
        match (
            self.tx_cost,
            self.find_closable_cancel_resolutions(at).await?.first(),
        ) {
            (Some(tx_cost), Some(closable)) if balance > tx_cost => {
                tracing::info!("Found pending cancel ready to close : {}", closable);
                SEQUENCER_ACTION.with_label_values(&["close_cancel"]).inc();
                self.close_cancel(*closable, at).await?;
                Ok(ActionStatus::Performed)
            }
            (Some(tx_cost), Some(closable)) => {
                tracing::error!("Found pending cancel ready to close : {}, but not enought funds available({}) vs required({})", closable, balance, tx_cost);
                Err(Error::NotEnoughtBalance)
            }
            (None, Some(closable)) => {
                tracing::warn!(
                    "Found pending cancel ready to close : {}, but tx closing is disabled",
                    closable
                );
                Ok(ActionStatus::Skipped)
            }
            _ => Ok(ActionStatus::Skipped),
        }
    }

    pub async fn run(&self, sender: Sender<()>) -> Result<(), Error> {
        let mut stream = self
            .l2
            .header_stream(l2api::Finalization::Finalized)
            .await?;
        loop {
            sender.send(()).await.expect("send error");
            let (number, block_hash) = stream.next().await.expect("infinite stream")?;
            let at = block_hash;

            tracing::info!("#{} : block hash {}", number, hex_encode(block_hash));

            if !self.is_active_sequencer().await? {
                tracing::error!(
                    "{} is not a sequencer for {:?}",
                    hex_encode(self.l2_account_address),
                    self.chain
                );
                return Err(Error::NotASequencer);
            }

            match (
                self.cancel_malicious_update_if_any(at).await?,
                self.send_sequencer_update(number, at).await?,
                self.retrieve_cancel_rights(at).await?,
            ) {
                (ActionStatus::Performed, _, _)
                | (_, ActionStatus::Performed, _)
                | (_, _, ActionStatus::Performed) => {
                    // ignore blocks produced while action was beeing performed
                    stream = Self::consume_stream_with_timeout(stream).await;
                    continue;
                }
                _ => {}
            }
        }
    }

    pub async fn close_cancel(&self, request_id: u128, at: H256) -> Result<(), Error> {
        let chain = self.chain;
        let (merkle_root, range) = self
            .l1
            .get_merkle_root(request_id)
            .await?
            .ok_or(Error::UnknownMerkleRoot)?;
        let proof = self
            .l2
            .get_merkle_proof(request_id, range, self.chain, at)
            .await?;

        let cancel = self
            .l2
            .get_l2_request(self.chain, request_id, at)
            .await?
            .and_then(|elem| match elem {
                gasp_types::L2Request::Cancel(cancel) => Some(cancel),
                _ => None,
            })
            .ok_or(Error::CancelIdDoesNotExistsOnL2 {
                request_id: request_id.into(),
                chain,
            })?;
        self.l1
            .close_cancel(cancel, merkle_root.into(), proof)
            .await?;
        Ok(())
    }

    async fn get_pending_updates(&self, at: H256) -> Result<Vec<PendingUpdate>, Error> {
        let updates = self.l2.get_pending_updates(at).await?;
        let mut updates = updates
            .into_iter()
            .filter(|update| update.chain == self.chain)
            .collect::<Vec<_>>();
        updates.sort_by_key(|update| update.update_id);
        Ok(updates)
    }

    pub async fn find_malicious_update(&self, at: H256) -> Result<Option<u128>, Error> {
        let updates = self.get_pending_updates(at).await?;
        let l1handle = &self.l1;

        let mut verified = futures::stream::iter(updates).map(|update| async {
            match l1handle
                .get_update_hash(update.range.0, update.range.1)
                .await
            {
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

    pub async fn find_latest_correct_update_block_submission(
        &self,
        at: H256,
    ) -> Result<Option<u128>, Error> {
        let updates = self.get_pending_updates(at).await?;
        let dispute_period_length = self.l2.get_dispute_period(self.chain, at).await?;
        let l1handle = &self.l1;

        let mut verified = futures::stream::iter(updates.into_iter().rev()).map(|update| async {
            match l1handle
                .get_update_hash(update.range.0, update.range.1)
                .await
            {
                Ok(correct_hash) => Ok((correct_hash, update)),
                Err(e) => Err(Error::from(e)),
            }
        });

        while let Some(result) = verified.next().await {
            let (correct_hash, update) = result.await?;
            if correct_hash == update.hash {
                return Ok(Some(update.update_id.saturating_sub(dispute_period_length)));
            }
        }
        Ok(None)
    }

    pub async fn cancel_update(&self, update_id: u128) -> Result<bool, Error> {
        Ok(self
            .l2
            .cancel_pending_request(update_id, self.chain)
            .await?)
    }

    #[tracing::instrument(skip(self))]
    pub async fn has_read_rights_available(&self) -> Result<bool, Error> {
        let at = self.get_latest_block_hash().await?;
        let read_rights = self.l2.get_read_rights(self.chain, at).await?;
        tracing::trace!("read rights: {}", read_rights);
        Ok(read_rights > 0)
    }

    #[tracing::instrument(skip(self))]
    pub async fn is_selected_sequencer(&self) -> Result<bool, Error> {
        let at = self.get_latest_block_hash().await?;
        match self.l2.get_selected_sequencer(self.chain, at).await? {
            Some(selected) if selected == self.l2_account_address => {
                tracing::debug!("i am selected");
                Ok(true)
            }
            Some(selected) => {
                tracing::debug!(
                    "im not the selected sequencer selected({}) vs me({})",
                    hex_encode(selected),
                    hex_encode(self.l2_account_address)
                );
                Ok(false)
            }
            None => {
                tracing::debug!("no selcted sequencer");
                Ok(false)
            }
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn has_cancel_rights_available(&self) -> Result<bool, Error> {
        let at = self.get_latest_block_hash().await?;
        let cancel_rights = self.l2.get_cancel_rights(self.chain, at).await?;
        tracing::trace!("cancel rights: {}", cancel_rights);
        Ok(cancel_rights > 0)
    }

    #[tracing::instrument(skip(self))]
    pub async fn is_active_sequencer(&self) -> Result<bool, Error> {
        let at = self.get_latest_block_hash().await?;
        let active = self.l2.get_active_sequencers(self.chain, at).await?;

        Ok(active.iter().any(|e| e == &(self.l2_account_address)))
    }

    pub async fn get_latest_block_hash(&self) -> Result<H256, Error> {
        Ok(self
            .l2
            .header_stream(l2api::Finalization::Finalized)
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
    ) -> Result<Option<(H256, gasp_types::L1Update)>, Error> {
        let latest_accepted_on_l2 = self
            .l2
            .get_latest_accepted_request_id(self.chain, at)
            .await?;
        let latest_request_l1 = self.l1.get_latest_reqeust_id().await?;

        tracing::debug!(
            "latest available on L1: {:?} latest accepted on L2 {}",
            latest_request_l1,
            latest_accepted_on_l2
        );

        match latest_request_l1 {
            Some(latest_request_l1) if latest_request_l1 > latest_accepted_on_l2 => {
                let start = latest_accepted_on_l2.saturating_add(1u128);
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
            }
        }
    }

    pub async fn find_closable_cancel_resolutions(&self, at: H256) -> Result<Vec<u128>, Error> {
        let latest_closable_request_id = self.l1.get_latest_finalized_request_id().await?;
        if let Some(latest_closable_request_id) = latest_closable_request_id {
            let cancels = self
                .l2
                .get_pending_cancels(self.chain, at)
                .await?
                .into_iter()
                .filter(|&cancel_request_id| cancel_request_id <= latest_closable_request_id)
                .collect::<Vec<_>>();

            let stream = futures::stream::iter(cancels)
                .map(|cancel_request_id: u128| async move {
                    let hash = self
                        .l2
                        .get_l2_request_hash(cancel_request_id, self.chain, at)
                        .await?
                        .ok_or(Error::L2RequestDoesNotExists(cancel_request_id))?;
                    let status = self.l1.get_status(hash).map(|e| {
                        e.map(|elem| (matches!(elem, RequestStatus::Closed), cancel_request_id))
                    });
                    Ok::<_, Error>(status.await?)
                })
                .collect::<Vec<_>>()
                .await;

            let result: Result<Vec<_>, Error> = futures::future::join_all(stream)
                .await
                .into_iter()
                .collect();
            Ok(result?
                .into_iter()
                .filter_map(|(closed, request_id)| if !closed { Some(request_id) } else { None })
                .collect())
        } else {
            Ok(vec![])
        }
    }

    pub async fn get_my_balance(&self) -> Result<u128, Error> {
        Ok(self.l1.native_balance(self.l1_account_address).await?)
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;
    use hex_literal::hex;

    use gasp_types::{Chain, PendingUpdate};
    use l1api::mock::MockL1;
    use l2api::mock::MockL2;
    use mockall::predicate::eq;

    use primitive_types::H256;

    const DUMMY_ADDRESS: [u8; 20] = hex!("0000000000000000000000000000000000000000");
    const ETHEREUM: Chain = Chain::Ethereum;
    const ARBITRUM: Chain = Chain::Arbitrum;

    #[tokio::test]
    async fn test_find_malicious_update_ignores_valid_updates() {
        let update_hash = H256::zero();
        let correct_hash = update_hash;

        let pending = PendingUpdate {
            chain: ETHEREUM,
            update_id: 1u128,
            range: (1u128, 1u128),
            hash: update_hash,
        };

        let mut l1mock = l1api::mock::MockL1::new();
        l1mock
            .expect_get_update_hash()
            .with(eq(1u128), eq(1u128))
            .times(1)
            .returning(move |_, _| Ok(correct_hash));

        let mut l2mock = MockL2::new();
        l2mock
            .expect_get_pending_updates()
            .times(1)
            .return_once(move |_| Ok(vec![pending]));

        let sequencer = Sequencer::new(
            l1mock,
            l2mock,
            ETHEREUM,
            DUMMY_ADDRESS,
            DUMMY_ADDRESS,
            100u128,
            None,
        );

        assert_eq!(
            sequencer.find_malicious_update(H256::zero()).await.unwrap(),
            None
        );
    }

    #[tokio::test]
    async fn test_find_malicious_update_ignores_updates_from_other_chains() {
        let update_hash = H256::zero();

        let pending = PendingUpdate {
            chain: ARBITRUM,
            update_id: 1u128,
            range: (1u128, 1u128),
            hash: update_hash,
        };

        let mut l1mock = MockL1::new();
        l1mock.expect_get_update_hash().times(0);

        let mut l2mock = MockL2::new();
        l2mock
            .expect_get_pending_updates()
            .times(1)
            .return_once(move |_| Ok(vec![pending]));

        let sequencer = Sequencer::new(
            l1mock,
            l2mock,
            ETHEREUM,
            DUMMY_ADDRESS,
            DUMMY_ADDRESS,
            100u128,
            None,
        );

        assert_eq!(
            sequencer.find_malicious_update(H256::zero()).await.unwrap(),
            None
        );
    }

    #[tokio::test]
    async fn test_find_malicious_update_works() {
        let update_hash = H256::from_slice(
            &hex::decode("1111111111111111111111111111111111111111111111111111111111111111")
                .unwrap(),
        );
        let correct_hash = H256::zero();

        let pending = PendingUpdate {
            chain: ETHEREUM,
            update_id: 1u128,
            range: (1u128, 1u128),
            hash: update_hash,
        };

        let mut l1mock = MockL1::new();
        l1mock
            .expect_get_update_hash()
            .with(eq(1u128), eq(1u128))
            .times(1)
            .returning(move |_, _| Ok(correct_hash));

        let mut l2mock = MockL2::new();
        l2mock
            .expect_get_pending_updates()
            .times(1)
            .return_once(move |_| Ok(vec![pending]));

        let sequencer = Sequencer::new(
            l1mock,
            l2mock,
            ETHEREUM,
            DUMMY_ADDRESS,
            DUMMY_ADDRESS,
            100u128,
            None,
        );

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
        l1mock
            .expect_get_status()
            .returning(|_| Ok(RequestStatus::Pending));

        let mut l2mock = MockL2::new();
        l2mock
            .expect_get_pending_cancels()
            .return_once(|_, _| Ok(vec![1u128, 2u128]));
        l2mock
            .expect_get_l2_request_hash()
            .returning(|_, _, _| Ok(Some(H256::zero())));

        let sequencer = Sequencer::new(
            l1mock,
            l2mock,
            ETHEREUM,
            DUMMY_ADDRESS,
            DUMMY_ADDRESS,
            100u128,
            None,
        );
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
        l1mock
            .expect_get_status()
            .returning(|_| Ok(RequestStatus::Pending));

        let pending_cancels = vec![1u128, 2u128, 10u128];
        let cancels = pending_cancels.clone();

        let mut l2mock = MockL2::new();
        l2mock
            .expect_get_l2_request_hash()
            .returning(|_, _, _| Ok(Some(H256::zero())));
        l2mock
            .expect_get_pending_cancels()
            .return_once(|_, _| Ok(cancels));

        let sequencer = Sequencer::new(
            l1mock,
            l2mock,
            ETHEREUM,
            DUMMY_ADDRESS,
            DUMMY_ADDRESS,
            100u128,
            None,
        );
        let result = sequencer
            .find_closable_cancel_resolutions(H256::zero())
            .await;

        assert_eq!(result.unwrap(), pending_cancels);
    }

    #[tokio::test]
    async fn test_find_pending_cancels_ignores_closed_cancels() {
        let at = H256::zero();
        let first_request_hash = H256::from(hex!(
            "0000000000000000000000000000000000000000000000000000000000000001"
        ));
        let second_request_hash = H256::from(hex!(
            "0000000000000000000000000000000000000000000000000000000000000002"
        ));
        let third_request_hash = H256::from(hex!(
            "0000000000000000000000000000000000000000000000000000000000000003"
        ));

        let mut l1mock = MockL1::new();
        l1mock
            .expect_get_latest_finalized_request_id()
            .return_once(|| Ok(Some(10u128)));
        l1mock
            .expect_get_status()
            .with(eq(first_request_hash))
            .returning(|_| Ok(RequestStatus::Closed));
        l1mock
            .expect_get_status()
            .with(eq(second_request_hash))
            .returning(|_| Ok(RequestStatus::Pending));
        l1mock
            .expect_get_status()
            .with(eq(third_request_hash))
            .returning(|_| Ok(RequestStatus::Closed));

        let pending_cancels = vec![1u128, 2u128, 10u128];
        let cancels = pending_cancels.clone();

        let mut l2mock = MockL2::new();
        l2mock
            .expect_get_l2_request_hash()
            .with(eq(pending_cancels[0]), eq(ETHEREUM), eq(at))
            .returning(move |_, _, _| Ok(Some(first_request_hash)));
        l2mock
            .expect_get_l2_request_hash()
            .with(eq(pending_cancels[1]), eq(ETHEREUM), eq(at))
            .returning(move |_, _, _| Ok(Some(second_request_hash)));
        l2mock
            .expect_get_l2_request_hash()
            .with(eq(pending_cancels[2]), eq(ETHEREUM), eq(at))
            .returning(move |_, _, _| Ok(Some(third_request_hash)));

        l2mock
            .expect_get_pending_cancels()
            .return_once(|_, _| Ok(cancels));

        let sequencer = Sequencer::new(
            l1mock,
            l2mock,
            ETHEREUM,
            DUMMY_ADDRESS,
            DUMMY_ADDRESS,
            100u128,
            None,
        );
        let result = sequencer.find_closable_cancel_resolutions(at).await;

        assert_eq!(result.unwrap(), vec![2u128]);
    }

    #[tokio::test]
    async fn test_find_pending_cancels_to_close_when_there_is_no_merkle_root_provided_to_l1() {
        let mut l1mock = MockL1::new();
        l1mock
            .expect_get_latest_finalized_request_id()
            .return_once(|| Ok(None));

        let mut l2mock = MockL2::new();
        l2mock.expect_get_pending_cancels().times(0);

        let sequencer = Sequencer::new(
            l1mock,
            l2mock,
            ETHEREUM,
            DUMMY_ADDRESS,
            DUMMY_ADDRESS,
            100u128,
            None,
        );
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
        l2mock
            .expect_get_latest_processed_request_id()
            .return_once(|_, _| Ok(0u128));

        l1mock.expect_get_update().times(0);

        let sequencer = Sequencer::new(
            l1mock,
            l2mock,
            ETHEREUM,
            DUMMY_ADDRESS,
            DUMMY_ADDRESS,
            100u128,
            None,
        );

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
        l2mock
            .expect_get_latest_processed_request_id()
            .return_once(|_, _| Ok(0u128));

        let eth_update = l1api::types::abi::L1Update {
            chain: Into::<l1api::types::abi::ChainId>::into(ETHEREUM).into(),
            pendingDeposits: vec![],
            pendingCancelResolutions: vec![],
        };

        let gasp_update = gasp_types::L1Update {
            chain: ETHEREUM.into(),
            pendingDeposits: vec![],
            pendingCancelResolutions: vec![],
        };

        l1mock
            .expect_get_update()
            .times(1)
            .with(eq(1u128), eq(10u128))
            .return_once(|_, _| Ok(eth_update));

        l1mock
            .expect_get_update_hash()
            .times(1)
            .with(eq(1u128), eq(10u128))
            .return_once(|_, _| Ok(H256::zero()));

        l2mock
            .expect_deserialize_sequencer_update()
            .times(1)
            .return_once(|_| Ok(gasp_update));

        let sequencer = Sequencer::new(
            l1mock,
            l2mock,
            ETHEREUM,
            DUMMY_ADDRESS,
            DUMMY_ADDRESS,
            100u128,
            None,
        );

        sequencer.get_pending_update(H256::zero()).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_pending_update_when_there_are_too_many_requests_for_single_update() {
        let mut l1mock = MockL1::new();
        l1mock
            .expect_get_latest_reqeust_id()
            .return_once(|| Ok(Some(1000u128)));

        let mut l2mock = MockL2::new();
        l2mock
            .expect_get_latest_processed_request_id()
            .return_once(|_, _| Ok(0u128));

        let eth_update = l1api::types::abi::L1Update {
            chain: Into::<l1api::types::abi::ChainId>::into(ETHEREUM).into(),
            pendingDeposits: vec![],
            pendingCancelResolutions: vec![],
        };

        let gasp_update = gasp_types::L1Update {
            chain: ETHEREUM.into(),
            pendingDeposits: vec![],
            pendingCancelResolutions: vec![],
        };

        l1mock
            .expect_get_update()
            .times(1)
            .with(eq(1u128), eq(101u128))
            .return_once(|_, _| Ok(eth_update));

        l1mock
            .expect_get_update_hash()
            .times(1)
            .with(eq(1u128), eq(101u128))
            .return_once(|_, _| Ok(H256::zero()));

        l2mock
            .expect_deserialize_sequencer_update()
            .times(1)
            .return_once(|_| Ok(gasp_update));

        let sequencer = Sequencer::new(
            l1mock,
            l2mock,
            ETHEREUM,
            DUMMY_ADDRESS,
            DUMMY_ADDRESS,
            100u128,
            None,
        );

        sequencer.get_pending_update(H256::zero()).await.unwrap();
    }

    #[tokio::test]
    async fn test_find_malicious_update_with_invalid_range_works() {
        let update_hash = H256::from(hex!(
            "1111111111111111111111111111111111111111111111111111111111111111"
        ));

        let pending = PendingUpdate {
            chain: ETHEREUM,
            update_id: 33u128,
            range: (1u128, 1u128),
            hash: update_hash,
        };

        let mut l1mock = MockL1::new();
        l1mock
            .expect_get_update_hash()
            .with(eq(1u128), eq(1u128))
            .times(1)
            .returning(move |_, _| Err(L1Error::InvalidRange));

        let mut l2mock = MockL2::new();
        l2mock
            .expect_get_pending_updates()
            .times(1)
            .return_once(move |_| Ok(vec![pending]));

        let sequencer = Sequencer::new(
            l1mock,
            l2mock,
            ETHEREUM,
            DUMMY_ADDRESS,
            DUMMY_ADDRESS,
            100u128,
            None,
        );

        assert_eq!(
            sequencer.find_malicious_update(H256::zero()).await.unwrap(),
            Some(33u128)
        );
    }

    #[tokio::test]
    async fn test_find_latest_correct_update() {
        let dispute_period = 10u128;
        let update_executed_at = 33u128;
        let update_hash = H256::zero();

        let pending = PendingUpdate {
            chain: ETHEREUM,
            update_id: update_executed_at,
            range: (1u128, 1u128),
            hash: update_hash,
        };

        let mut l1mock = MockL1::new();
        l1mock
            .expect_get_update_hash()
            .times(1)
            .with(eq(1u128), eq(1u128))
            .return_once(move |_, _| Ok(update_hash));

        let mut l2mock = MockL2::new();
        l2mock
            .expect_get_pending_updates()
            .times(1)
            .return_once(move |_| Ok(vec![pending]));

        l2mock
            .expect_get_dispute_period()
            .times(1)
            .with(eq(ETHEREUM), eq(H256::zero()))
            .return_once(move |_, _| Ok(dispute_period));

        let sequencer = Sequencer::new(
            l1mock,
            l2mock,
            ETHEREUM,
            DUMMY_ADDRESS,
            DUMMY_ADDRESS,
            100u128,
            None,
        );

        assert_eq!(
            sequencer
                .find_latest_correct_update_block_submission(H256::zero())
                .await
                .unwrap(),
            Some(update_executed_at - dispute_period)
        );
    }

    #[tokio::test]
    async fn test_find_latest_correct_update_prefers_latest() {
        let dispute_period = 10u128;
        let latest_update_executed_at = 33u128;
        let old_update_executed_at = 23u128;
        let update_hash = H256::zero();

        let latest_pending = PendingUpdate {
            chain: ETHEREUM,
            update_id: latest_update_executed_at,
            range: (1u128, 1u128),
            hash: update_hash,
        };

        let old_pending = PendingUpdate {
            chain: ETHEREUM,
            update_id: old_update_executed_at,
            range: (1u128, 1u128),
            hash: update_hash,
        };

        let mut l1mock = MockL1::new();
        l1mock
            .expect_get_update_hash()
            .times(1)
            .with(eq(1u128), eq(1u128))
            .return_once(move |_, _| Ok(update_hash));

        let mut l2mock = MockL2::new();
        l2mock
            .expect_get_pending_updates()
            .times(1)
            .return_once(move |_| Ok(vec![old_pending, latest_pending]));

        l2mock
            .expect_get_dispute_period()
            .times(1)
            .with(eq(Chain::Ethereum), eq(H256::zero()))
            .return_once(move |_, _| Ok(dispute_period));

        let sequencer = Sequencer::new(
            l1mock,
            l2mock,
            ETHEREUM,
            DUMMY_ADDRESS,
            DUMMY_ADDRESS,
            100u128,
            None,
        );

        assert_eq!(
            sequencer
                .find_latest_correct_update_block_submission(H256::zero())
                .await
                .unwrap(),
            Some(latest_update_executed_at - dispute_period)
        );
    }

    #[tokio::test]
    async fn test_find_latest_correct_update_ignores_invalid_updates() {
        let dispute_period = 10u128;
        let valid_update_executed_at = 33u128;
        let invalid_update_executed_at = 43u128;
        let update_hash = H256::from(hex!(
            "1111111111111111111111111111111111111111111111111111111111111111"
        ));
        let invalid_hash = H256::from(hex!(
            "2222222222222222222222222222222222222222222222222222222222222222"
        ));

        let valid_pending = PendingUpdate {
            chain: ETHEREUM,
            update_id: valid_update_executed_at,
            range: (1u128, 1u128),
            hash: update_hash,
        };

        let invalid_pending = PendingUpdate {
            chain: ETHEREUM,
            update_id: invalid_update_executed_at,
            range: (1u128, 3u128),
            hash: update_hash,
        };

        let mut l1mock = MockL1::new();
        l1mock
            .expect_get_update_hash()
            .times(1)
            .with(eq(1u128), eq(1u128))
            .return_once(move |_, _| Ok(update_hash));
        l1mock
            .expect_get_update_hash()
            .times(1)
            .with(eq(1u128), eq(3u128))
            .return_once(move |_, _| Ok(invalid_hash));

        let mut l2mock = MockL2::new();
        l2mock
            .expect_get_pending_updates()
            .times(1)
            .return_once(move |_| Ok(vec![invalid_pending, valid_pending]));

        l2mock
            .expect_get_dispute_period()
            .times(1)
            .with(eq(Chain::Ethereum), eq(H256::zero()))
            .return_once(move |_, _| Ok(dispute_period));

        let sequencer = Sequencer::new(
            l1mock,
            l2mock,
            ETHEREUM,
            DUMMY_ADDRESS,
            DUMMY_ADDRESS,
            100u128,
            None,
        );

        assert_eq!(
            sequencer
                .find_latest_correct_update_block_submission(H256::zero())
                .await
                .unwrap(),
            Some(valid_update_executed_at - dispute_period)
        );
    }
}

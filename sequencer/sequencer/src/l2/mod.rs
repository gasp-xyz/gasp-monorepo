use std::collections::HashMap;
use std::pin::Pin;

use futures::Stream;
use hex::encode as hex_encode;

use futures::StreamExt;
use primitive_types::H256;
// use subxt::config::Header;
use subxt::ext::subxt_core;
use subxt::ext::subxt_core::storage::address::StorageHashers;
use subxt::storage::StorageKey;
use subxt::Config;
use subxt::OnlineClient;

mod signer;
use signer::Keypair;

mod gasp;
use gasp::{GaspAddress, GaspConfig, GaspSignature};

pub mod types {
    use super::gasp;

    // NOTE: this alias is used in multiple other files to make code more readable
    #[allow(unused_imports)]
    pub use gasp::api as bindings;
    pub use gasp::api::runtime_types::pallet_rolldown::messages::L1Update;
    #[allow(unused_imports)]
    pub use gasp::api::runtime_types::pallet_rolldown::messages::{
        Chain, Origin, RequestId, Deposit, CancelResolution
    };
}

pub type PendingUpdateWithKeys = (u128, types::L1Update, H256);
pub type HeaderStream = Pin<Box<dyn Stream<Item = Result<(u32, H256), L2Error>> + Send + 'static>>;

#[derive(Debug)]
pub struct PendingUpdate {
    pub chain: types::Chain,
    pub update_id: u128,
    pub range: (u128, u128),
    pub hash: H256,
}

use gasp::api::runtime_types::frame_system::EventRecord;
use gasp::api::runtime_types::rollup_runtime::RuntimeEvent;
pub type L2Event = EventRecord<RuntimeEvent, H256>;

pub trait L2Interface {
    fn address(&self) -> [u8; 20];
    async fn get_latest_processed_request_id(
        &self,
        chain: types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<u128, L2Error>;
    async fn get_read_rights(
        &self,
        chain: types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<u128, L2Error>;
    async fn get_selected_sequencer(
        &self,
        chain: types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<Option<[u8; 20]>, L2Error>;
    async fn get_cancel_rights(
        &self,
        chain: types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<u128, L2Error>;
    async fn get_pending_updates(
        &self,
        at: HashOf<GaspConfig>,
    ) -> Result<Vec<PendingUpdateWithKeys>, L2Error>;
    async fn deserialize_sequencer_update(&self, data: Vec<u8>)
        -> Result<types::L1Update, L2Error>;
    async fn cancel_pending_request(
        &self,
        request_id: u128,
        chain: types::Chain,
    ) -> Result<bool, L2Error>;
    async fn update_l1_from_l2(&self, update: types::L1Update, hash: H256)
        -> Result<bool, L2Error>;

    async fn get_pending_cancels(
        &self,
        chain: types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<Vec<u128>, L2Error>;

    async fn get_merkle_proof(
        &self,
        request_id: u128,
        range: (u128, u128),
        chain: types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<Vec<H256>, L2Error>;

    async fn get_l2_request_hash(
        &self,
        request_id: u128,
        chain: types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<Option<H256>, L2Error>;

    async fn header_stream( &self,) -> Result<HeaderStream, L2Error>;

    async fn finalized_header_stream(&self) -> Result<HeaderStream, L2Error>;

    async fn get_abi_encoded_request(
        &self,
        request_id: u128,
        chain: types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<Vec<u8>, L2Error>;
}

pub struct Gasp {
    client: OnlineClient<GaspConfig>,
    keypair: Keypair,
}

#[derive(Debug, thiserror::Error)]
pub enum L2Error {
    #[error("tx inclusion block does not exist")]
    TxInclusionBlockDoesNotExits,
    #[error("tx included but not executed")]
    TxIncludedButNotExecuted,
    #[error("block fetch error")]
    BlockFetchError,
    #[error("unknown error")]
    Subxt(#[from] subxt::Error),
    #[error("unknown error")]
    SubxtExt(#[from] subxt::ext::subxt_core::Error),
    #[error("cannot fetch sequencer rights")]
    CanNotFetchRights,
    #[error("runtime api call failed")]
    SequencerUpdateConversionError,
    #[error("cannot fetch last processed request id")]
    CanNotFetchLatestProcessedRequestId,
    #[error("unknown tx status")]
    UnknownTxStatus,
    #[error("cannot subscribe to block headers")]
    HeaderSubscriptionFailed,
    #[error("awaiting cancel resolution fetch error")]
    PendingCancelFetchError,
}

pub type HashOf<T> = <T as Config>::Hash;

impl Gasp {
    pub async fn new(uri: &str, secret_key: [u8; 32]) -> Result<Self, L2Error> {
        let client = OnlineClient::<GaspConfig>::from_url(uri).await?;

        Ok(Self {
            client,
            keypair: Keypair::from_secret_key(secret_key),
        })
    }

    async fn get_events(&self, at: HashOf<GaspConfig>) -> Result<Vec<L2Event>, L2Error> {
        let storage = gasp::api::storage().system().events();
        Ok(self
            .client
            .storage()
            .at(at)
            .fetch(&storage)
            .await?
            .unwrap_or_default())
    }

    #[tracing::instrument(skip(self))]
    async fn wait_for_tx_execution(&self, tx_hash: HashOf<GaspConfig>) -> Result<bool, L2Error> {
        let mut stream = self.finalized_header_stream().await?;

        while let Some(header) = stream.next().await {
            let (number, hash) = header?;

            tracing::debug!(
                "checking block #{} {}",
                number,
                hex_encode(tx_hash),
            );

            let block = self.client.blocks().at(hash.clone()).await?;
            let extrinsics = block.extrinsics().await?;

            if let Some((id, _)) = extrinsics
                .iter()
                .enumerate()
                .find(|(_, extrinsic)| extrinsic.hash() == tx_hash)
            {
                let events = self.get_events(hash.into()).await?;
                let events = events
                    .iter()
                    .filter(|elem| {
                        matches!(
                            elem.phase,
                            gasp::api::runtime_types::frame_system::Phase::ApplyExtrinsic(pos) if pos == id as u32
                        )
                    })
                    .collect::<Vec<_>>();

                let status = events.iter().find(|elem | {
                        matches!(elem.event, RuntimeEvent::System(gasp::api::runtime_types::frame_system::pallet::Event::ExtrinsicSuccess{..})) ||
                        matches!(elem.event, RuntimeEvent::System(gasp::api::runtime_types::frame_system::pallet::Event::ExtrinsicFailed{..}))
                });

                let elem = status.ok_or(L2Error::UnknownTxStatus)?;

                let status = match elem.event {
                    RuntimeEvent::System(
                        gasp::api::runtime_types::frame_system::pallet::Event::ExtrinsicSuccess {
                            ..
                        },
                    ) => Ok(true),
                    RuntimeEvent::System(
                        gasp::api::runtime_types::frame_system::pallet::Event::ExtrinsicFailed {
                            ..
                        },
                    ) => Ok(false),
                    _ => Err(L2Error::UnknownTxStatus),
                };

                tracing::debug!("execution status: {:?}", status);
                return status;
            }
        }

        Err(L2Error::UnknownTxStatus)
    }

    async fn sign_and_send(&self, call: impl subxt::tx::Payload) -> Result<bool, L2Error> {
        let tx = self.client.tx();

        let partial_signed = tx
            .create_partial_signed(&call, &self.keypair.address().into(), Default::default())
            .await?;

        tracing::trace!("tx: {}", hex_encode(partial_signed.signer_payload()));

        let signed = partial_signed.sign(&self.keypair);

        tracing::trace!("signed tx: {}", hex_encode(signed.encoded()));

        let tx_hash = signed.submit().await?;
        Ok(self.wait_for_tx_execution(tx_hash).await?)
    }

    #[cfg(test)]
    async fn update_l1_from_l2_unsafe(
        &self,
        update: gasp::api::runtime_types::pallet_rolldown::messages::L1Update,
    ) -> Result<bool, L2Error> {
        let call = gasp::api::tx().rolldown().update_l2_from_l1_unsafe(update);
        self.sign_and_send(call).await
    }

    #[cfg(test)]
    pub async fn wait_for_next_block(&self) -> Result<(), L2Error> {
        let (best, _header) = self.latest_block().await?;
        let mut stream = self.header_stream().await?;

        while let Some(item) = stream.next().await {
            let (number, _hash) = item?;
            if number > best {
                break;
            }
        }
        Ok(())
    }

    #[cfg(test)]
    pub async fn latest_block(&self) -> Result<(u32, HashOf<GaspConfig>), L2Error> {
        let mut stream = self.header_stream().await?;
        stream
            .next()
            .await
            .ok_or(L2Error::HeaderSubscriptionFailed)?
    }

    #[cfg(test)]
    pub async fn withdraw(
        &self,
        chain: types::Chain,
        recipient: [u8; 20],
        token: [u8; 20],
        amount: u128,
        ferry_tip: Option<u128>,
    ) -> Result<bool, L2Error> {
        let call = gasp::api::tx()
            .rolldown()
            .withdraw(chain, recipient, token, amount, ferry_tip);
        self.sign_and_send(call).await
    }
}

impl L2Interface for Gasp {
    fn address(&self) -> [u8; 20] {
        self.keypair.address().into_inner()
    }

    #[tracing::instrument(skip(self))]
    async fn get_latest_processed_request_id(
        &self,
        chain: types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<u128, L2Error> {
        let storage = gasp::api::storage()
            .rolldown()
            .last_processed_request_on_l2(chain);
        Ok(self
            .client
            .storage()
            .at(at)
            .fetch(&storage)
            .await?
            .unwrap_or_default())
    }

    #[tracing::instrument(skip(self))]
    async fn get_read_rights(
        &self,
        chain: types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<u128, L2Error> {
        use gasp::api::runtime_types::pallet_rolldown::pallet::SequencerRights;

        let storage = gasp::api::storage().rolldown().sequencers_rights(chain);
        let rights: HashMap<GaspAddress, SequencerRights> = self
            .client
            .storage()
            .at(at)
            .fetch(&storage)
            .await?
            .unwrap_or_default()
            .into_iter()
            .map(|(k, v)| (k.0.into(), v))
            .collect();

        rights
            .get(&self.keypair.address())
            .map(|elem| elem.read_rights)
            .ok_or(L2Error::CanNotFetchRights)
    }

    #[tracing::instrument(skip(self))]
    async fn get_selected_sequencer(
        &self,
        chain: types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<Option<[u8; 20]>, L2Error>{
        let storage = gasp::api::storage().sequencer_staking().selected_sequencer();
        let selected = self
            .client
            .storage()
            .at(at)
            .fetch(&storage)
            .await?
            .unwrap_or_default();

        let selected = selected.iter()
            .find(|(c, _)| c == &chain)
            .map(|(_, account)| account.0);

        if let Some(selected) = &selected {
            tracing::trace!("selected : {}", hex_encode(selected));
        } else {
            tracing::warn!("no sequencer selected");
        }
        Ok(selected)
    }

    #[tracing::instrument(skip(self))]
    async fn get_cancel_rights(
        &self,
        chain: types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<u128, L2Error> {
        use gasp::api::runtime_types::pallet_rolldown::pallet::SequencerRights;

        let storage = gasp::api::storage().rolldown().sequencers_rights(chain);
        let rights: HashMap<GaspAddress, SequencerRights> = self
            .client
            .storage()
            .at(at)
            .fetch(&storage)
            .await?
            .unwrap_or_default()
            .into_iter()
            .map(|(k, v)| (k.0.into(), v))
            .collect();

        rights
            .get(&self.keypair.address())
            .map(|elem| elem.cancel_rights)
            .ok_or(L2Error::CanNotFetchRights)
    }

    #[tracing::instrument(skip(self))]
    async fn deserialize_sequencer_update(
        &self,
        payload: Vec<u8>,
    ) -> Result<gasp::api::runtime_types::pallet_rolldown::messages::L1Update, L2Error> {
        let call = gasp::api::runtime_apis::rolldown_runtime_api::RolldownRuntimeApi
            .get_native_sequencer_update(payload);

        let update = self
            .client
            .runtime_api()
            .at_latest()
            .await?
            .call(call)
            .await?;

        update.ok_or(L2Error::SequencerUpdateConversionError)
    }

    #[tracing::instrument(skip(self, update))]
    async fn update_l1_from_l2(
        &self,
        update: gasp::api::runtime_types::pallet_rolldown::messages::L1Update,
        hash: H256,
    ) -> Result<bool, L2Error> {
        let call = gasp::api::tx().rolldown().update_l2_from_l1(update, hash);
        self.sign_and_send(call).await
    }

    #[tracing::instrument(skip(self))]
    async fn cancel_pending_request(
        &self,
        request_id: u128,
        chain: types::Chain,
    ) -> Result<bool, L2Error> {
        let call = gasp::api::tx()
            .rolldown()
            .cancel_requests_from_l1(chain, request_id);
        self.sign_and_send(call).await
    }

    #[tracing::instrument(skip(self))]
    async fn get_pending_cancels(
        &self,
        chain: types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<Vec<u128>, L2Error> {
        let storage_entry = gasp::api::storage()
            .rolldown()
            .awaiting_cancel_resolution(chain);

        let result = self
            .client
            .storage()
            .at(at)
            .fetch(&storage_entry)
            .await?
            .unwrap_or_default()
            .into_iter()
            .filter(|(account, _request_id, _role)| account.0 == self.keypair.address().into_inner())
            .map(|(_account, request_id, _role)| request_id)
            .collect::<Vec<_>>();

        tracing::debug!(
            "found {} pending cancels associated with account: {}",
            result.len(),
            hex_encode(self.keypair.address())
        );

        Ok(result)
    }

    #[tracing::instrument(skip(self))]
    async fn get_pending_updates(
        &self,
        at: HashOf<GaspConfig>,
    ) -> Result<Vec<PendingUpdateWithKeys>, L2Error> {
        use ::subxt::ext::subxt_core::storage::address::StaticStorageKey;
        use gasp::api::rolldown::storage::types as gasp_types;

        let metadata = self.client.metadata();
        let (_pallet, entry) = subxt_core::storage::lookup_storage_entry_details(
            "Rolldown",
            "PendingSequencerUpdates",
            &metadata,
        )?;

        let hashers = StorageHashers::new(entry.entry_type(), metadata.types())?;

        let iter = gasp::api::storage()
            .rolldown()
            .pending_sequencer_updates_iter();
        let result: Vec<Result<_, L2Error>> = self
            .client
            .storage()
            .at(at)
            .iter(iter)
            .await?
            .map(|result| -> Result<_, L2Error> {
                let storage_kv = result?;
                let (_acc, update, hash) = storage_kv.value;

                let keys = <(
                    StaticStorageKey<gasp_types::pending_sequencer_updates::Param0>,
                    StaticStorageKey<gasp_types::pending_sequencer_updates::Param1>,
                )>::decode_storage_key(
                    &mut &storage_kv.key_bytes[32..],
                    &mut hashers.iter(),
                    metadata.types(),
                )?;

                Ok((keys.0.decoded()?, update, hash))
            })
            .collect()
            .await;

        result.into_iter().collect()
    }

    #[tracing::instrument(skip(self))]
    async fn get_merkle_proof(
        &self,
        request_id: u128,
        range: (u128, u128),
        chain: types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<Vec<H256>, L2Error> {
        // let range = types::Range{ start, end };
        let call = gasp::api::runtime_apis::rolldown_runtime_api::RolldownRuntimeApi
            .get_merkle_proof_for_tx(chain, range, request_id);

        let proof = self
            .client
            .runtime_api()
            .at_latest()
            .await?
            .call(call)
            .await?;

        proof.iter().enumerate().for_each(|(id, elem)| {
            tracing::trace!("proof[{id}] elem: {}", hex_encode(elem));
        });

        Ok(proof)
    }

    #[tracing::instrument(skip(self))]
    async fn get_abi_encoded_request(
        &self,
        request_id: u128,
        chain: types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<Vec<u8>, L2Error> {
        let call = gasp::api::runtime_apis::rolldown_runtime_api::RolldownRuntimeApi
            .get_abi_encoded_l2_request(chain, request_id);

        let abi_encoded_request = self
            .client
            .runtime_api()
            .at_latest()
            .await?
            .call(call)
            .await?;


        Ok(abi_encoded_request)
    }

    #[tracing::instrument(skip(self))]
    async fn get_l2_request_hash(
        &self,
        request_id: u128,
        chain: types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<Option<H256>, L2Error> {
        let req = types::RequestId {
            origin: types::Origin::L2,
            id: request_id,
        };

        let storage = gasp::api::storage().rolldown().l2_requests(chain, req);
        let reqeust_hash = self
            .client
            .storage()
            .at(at)
            .fetch(&storage)
            .await?
            .map(|elem| elem.1);

        if let Some(request_hash) = &reqeust_hash {
            tracing::trace!("request hash {}", hex_encode(request_hash));
        } else {
            tracing::warn!("request hash unknown");
        }

        Ok(reqeust_hash)
    }

    async fn header_stream(
        &self,
    ) -> Result<HeaderStream, L2Error>
    {
        Ok(self
            .client
            .backend()
            .stream_best_block_headers()
            .await?
            .map(|elem| {
                elem.map(|(header, hash)| (header.number, hash.hash()))
                    .map_err(|err| L2Error::from(err))
            })
            .boxed())
    }

    async fn finalized_header_stream(
        &self,
    ) -> Result<HeaderStream, L2Error>
    {
        Ok(self
            .client
            .backend()
            .stream_finalized_block_headers()
            .await?
            .map(|elem| {
                elem.map(|(header, hash)| (header.number, hash.hash()))
                    .map_err(|err| L2Error::from(err))
            })
            .boxed())
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sequencer::test::{to_u256, Request, UpdateBuilder};
    use hex_literal::hex;
    use serial_test::serial;

    //TODO: adcd test for L2Interace::deserialize_sequencer_update

    const URI: &'static str = "ws://localhost:9944";
    const DUMMY_PKEY: [u8; 32] =
        hex!("b9d2ea9a615f3165812e8d44de0d24da9bbd164b65c4f0573e1ce2c8dbd9c8df");
    const DUMMY_ADDR: [u8; 20] = hex!("0000000000000000000000000000000000000000");
    const ALITH_PKEY: [u8; 32] =
        hex!("5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133");
    const BALTATHAR_PKEY: [u8; 32] =
        hex!("8075991ce870b93a8870eca0c0f91913d12f47948ca0fd25b49c6fa7cdbeee8b");
    const TEST_TOKEN: [u8; 20] = hex!("FD471836031dc5108809D173A067e8486B9047A3");
    const ETHEREUM: types::Chain = types::Chain::Ethereum;

    #[serial]
    #[tokio::test]
    async fn test_can_connect() {
        Gasp::new(URI, BALTATHAR_PKEY)
            .await
            .expect("can connect to gasp");
    }

    #[serial]
    #[tokio::test]
    async fn test_can_submit_multiple_tx_in_a_row() {
        use tracing::level_filters::LevelFilter;
        let filter = tracing_subscriber::EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy()
            .add_directive("sequencer=trace".parse().expect("proper directive"));
        tracing_subscriber::fmt().with_env_filter(filter).init();


        let gasp = Gasp::new(URI, BALTATHAR_PKEY)
            .await
            .expect("can connect to gasp");
        gasp.withdraw(ETHEREUM, DUMMY_ADDR, TEST_TOKEN, 100, None)
            .await
            .expect("can submit withdrawal");
        gasp.withdraw(ETHEREUM, DUMMY_ADDR, TEST_TOKEN, 100, None)
            .await
            .expect("can submit withdrawal");
    }

    #[serial]
    #[tokio::test]
    async fn can_subscribe_to_new_blocks() {
        let gasp = Gasp::new(URI, BALTATHAR_PKEY)
            .await
            .expect("can connect to gasp");
        let mut stream = gasp.header_stream().await.unwrap();
        let (number1, hash1) = stream.next().await.expect("can fetch next block").unwrap();
        let (number2, hash2) = stream.next().await.expect("can fetch next block").unwrap();
        assert!(number2 > number1);
        assert!(hash1 != hash2);
    }

    #[serial]
    #[tokio::test]
    async fn test_can_fetch_rights() {
        let baltathat_gasp = Gasp::new(URI, BALTATHAR_PKEY)
            .await
            .expect("can connect to gasp");
        let dummy_gasp = Gasp::new(URI, DUMMY_PKEY)
            .await
            .expect("can connect to gasp");

        let at = baltathat_gasp.latest_block().await.unwrap().1;

        let dummy_read_rights = dummy_gasp.get_read_rights(types::Chain::Ethereum, at).await;
        let dummy_cancel_rights = dummy_gasp
            .get_cancel_rights(types::Chain::Ethereum, at)
            .await;

        let baltathar_read_rights = baltathat_gasp
            .get_read_rights(types::Chain::Ethereum, at)
            .await;
        let baltathar_cancel_rights = baltathat_gasp
            .get_cancel_rights(types::Chain::Ethereum, at)
            .await;

        assert!(matches!(dummy_read_rights, Err(L2Error::CanNotFetchRights)),);
        assert!(matches!(
            dummy_cancel_rights,
            Err(L2Error::CanNotFetchRights)
        ),);

        assert_eq!(baltathar_read_rights.unwrap(), 1u128);

        baltathar_cancel_rights.unwrap();
    }

    #[serial]
    #[tokio::test]
    async fn test_fetch_merkle_proof() {
        let gasp = Gasp::new(URI, ALITH_PKEY)
            .await
            .expect("can connect to gasp");
        gasp.wait_for_next_block().await.unwrap();

        assert_eq!(
            gasp.withdraw(ETHEREUM, DUMMY_ADDR, TEST_TOKEN, 100, None)
                .await
                .expect("can submit withdrawal"),
            true
        );

        gasp.wait_for_next_block().await.unwrap();
        gasp.wait_for_next_block().await.unwrap();
        gasp.wait_for_next_block().await.unwrap();

        assert_eq!(
            gasp.withdraw(ETHEREUM, DUMMY_ADDR, TEST_TOKEN, 100, None)
                .await
                .expect("can submit withdrawal"),
            true
        );

        let at = gasp.latest_block().await.unwrap().1;
        let hash1 = gasp
            .get_l2_request_hash(1u128, ETHEREUM, at)
            .await
            .expect("can fetch l2 request hash")
            .expect("can fetch l2 request hash");

        let hash2 = gasp
            .get_l2_request_hash(2u128, ETHEREUM, at)
            .await
            .expect("can fetch l2 request hash")
            .expect("can fetch l2 request hash");

        assert!(hash1 != H256::zero());
        assert!(hash2 != H256::zero());
        assert!(hash1 != hash2);

        let proofs = gasp
            .get_merkle_proof(1u128, (1u128, 2u128), ETHEREUM, at)
            .await
            .expect("can fetch l2 request hash");

        assert_eq!(proofs.len(), 1 as usize);
    }


    #[serial]
    #[tokio::test]
    async fn test_can_fetch_pending_cancels() {
        let gasp = Gasp::new(URI, BALTATHAR_PKEY)
            .await
            .expect("can connect to gasp");
        let at = gasp.latest_block().await.unwrap().1;
        gasp.get_pending_cancels(ETHEREUM, at).await.expect("can fetch pending cancels");
    }

    #[serial]
    #[tokio::test]
    async fn test_can_cancel_pending_update() {
        let gasp = Gasp::new(URI, BALTATHAR_PKEY)
            .await
            .expect("can connect to gasp");

        let result = gasp.cancel_pending_request(u128::MAX, ETHEREUM).await.expect("can fetch pending cancels");
        assert_eq!(false, result);
    }

    #[serial]
    #[tokio::test]
    async fn test_can_submit_and_fetch_udates() {
        let gasp = Gasp::new(URI, ALITH_PKEY)
            .await
            .expect("can connect to gasp");
        let at = gasp.latest_block().await.unwrap().1;
        let latest_req_id = gasp.get_latest_processed_request_id(ETHEREUM, at).await.unwrap();
        let next_req_id = latest_req_id.saturating_add(1u128);
        let update = UpdateBuilder::new()
            .with_request(Request::Deposit(types::Deposit {
                requestId: types::RequestId {
                    origin: types::Origin::L1,
                    id: next_req_id,
                },
                depositRecipient: DUMMY_ADDR,
                tokenAddress: DUMMY_ADDR,
                amount: to_u256(100u128),
                timeStamp: to_u256(0u128),
                ferryTip: to_u256(0u128),
            }))
            .build(ETHEREUM);

        let status = gasp.update_l1_from_l2_unsafe(update)
            .await
            .expect("can submit update");

        assert_eq!(status, false);

    }
}

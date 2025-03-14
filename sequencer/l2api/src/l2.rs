use gasp_types::L2Request;
use std::collections::HashMap;

use hex::encode as hex_encode;

use crate::types::Finalization;
use crate::L2Error;
use futures::StreamExt;
use primitive_types::H256;
use subxt::OnlineClient;

use crate::signer::Keypair;
use crate::HashOf;
use crate::HeaderStream;
use crate::L2Interface;

use gasp_bindings::api::runtime_types::frame_system::EventRecord;
use gasp_bindings::api::runtime_types::rollup_runtime::RuntimeEvent;
use gasp_bindings::{GaspAddress, GaspConfig};



pub type L2Event = EventRecord<RuntimeEvent, H256>;

pub struct Gasp {
    client: OnlineClient<GaspConfig>,
    keypair: Keypair,
}

#[allow(dead_code)]
impl Gasp {
    pub async fn new(uri: &str, secret_key: [u8; 32]) -> Result<Self, L2Error> {
        let client = OnlineClient::<GaspConfig>::from_insecure_url(uri).await?;

        Ok(Self {
            client,
            keypair: Keypair::from_secret_key(secret_key),
        })
    }

    async fn get_events(&self, at: HashOf<GaspConfig>) -> Result<Vec<L2Event>, L2Error> {
        let storage = gasp_bindings::api::storage().system().events();
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
        let mut stream = self.header_stream(Finalization::Finalized).await?;

        while let Some(header) = stream.next().await {
            let (number, hash) = header?;

            tracing::debug!("checking block #{} {}", number, hex_encode(tx_hash),);

            let block = self.client.blocks().at(hash).await?;
            let extrinsics = block.extrinsics().await?;

            if let Some((id, _)) = extrinsics
                .iter()
                .enumerate()
                .find(|(_, extrinsic)| extrinsic.hash() == tx_hash)
            {
                let events = self.get_events(hash).await?;
                let events = events
                    .iter()
                    .filter(|elem| {
                        matches!(
                            elem.phase,
                            gasp_bindings::api::runtime_types::frame_system::Phase::ApplyExtrinsic(pos) if pos == id as u32
                        )
                    })
                    .collect::<Vec<_>>();

                let status = events.iter().find(|elem | {
                        matches!(elem.event, RuntimeEvent::System(gasp_bindings::api::runtime_types::frame_system::pallet::Event::ExtrinsicSuccess{..})) ||
                        matches!(elem.event, RuntimeEvent::System(gasp_bindings::api::runtime_types::frame_system::pallet::Event::ExtrinsicFailed{..}))
                });

                let elem = status.ok_or(L2Error::UnknownTxStatus)?;

                let status = match elem.event {
                    RuntimeEvent::System(
                        gasp_bindings::api::runtime_types::frame_system::pallet::Event::ExtrinsicSuccess {
                            ..
                        },
                    ) => Ok(true),
                    RuntimeEvent::System(
                        gasp_bindings::api::runtime_types::frame_system::pallet::Event::ExtrinsicFailed {
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
            .create_partial_signed(&call, &self.keypair.address(), Default::default())
            .await?;

        tracing::trace!("tx: {}", hex_encode(partial_signed.signer_payload()));

        let signed = partial_signed.sign(&self.keypair);

        tracing::trace!("signed tx: {}", hex_encode(signed.encoded()));

        let tx_hash = signed.submit().await?;
        self.wait_for_tx_execution(tx_hash).await
    }

    #[cfg(test)]
    async fn update_l1_from_l2_unsafe(
        &self,
        update: gasp_bindings::api::runtime_types::pallet_rolldown::messages::L1Update,
    ) -> Result<bool, L2Error> {
        let call = gasp_bindings::api::tx()
            .rolldown()
            .update_l2_from_l1_unsafe(update);
        self.sign_and_send(call).await
    }

    #[cfg(test)]
    pub async fn wait_for_next_block(&self) -> Result<(), L2Error> {
        let (best, _header) = self.latest_block().await?;
        let mut stream = self.header_stream(Finalization::Best).await?;

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
        let mut stream = self.header_stream(Finalization::Best).await?;
        stream
            .next()
            .await
            .ok_or(L2Error::HeaderSubscriptionFailed)?
    }

    #[cfg(test)]
    pub async fn withdraw(
        &self,
        chain: gasp_types::Chain,
        recipient: [u8; 20],
        token: [u8; 20],
        amount: u128,
        ferry_tip: Option<u128>,
    ) -> Result<bool, L2Error> {
        let chain: crate::types::subxt::Chain = chain.into();
        let call = gasp_bindings::api::tx()
            .rolldown()
            .withdraw(chain, recipient, token, amount, ferry_tip);
        self.sign_and_send(call).await
    }
}

impl L2Interface for Gasp {
    fn account_address(&self) -> [u8; 20] {
        self.keypair.address().into_inner()
    }

    #[tracing::instrument(skip(self), ret)]
    async fn get_l2_request(
        &self,
        chain: gasp_types::Chain,
        request_id: u128,
        at: HashOf<GaspConfig>,
    ) -> Result<Option<L2Request>, L2Error> {
        pub use gasp_bindings::api::runtime_types::pallet_rolldown::pallet::L2Request as SubxtL2Request;

        let request_id = crate::types::subxt::RequestId {
            origin: crate::types::subxt::Origin::L2,
            id: request_id,
        };
        let chain: crate::types::subxt::Chain = chain.into();
        let storage = gasp_bindings::api::storage()
            .rolldown()
            .l2_requests(chain, request_id);

        Ok(self.client.storage().at(at).fetch(&storage).await?.map(
            |(request, _hash)| match request {
                SubxtL2Request::FailedDepositResolution(_deposit) => {
                    todo!()
                }
                SubxtL2Request::Withdrawal(withdrawal) => L2Request::Withdrawal(withdrawal.into()),

                SubxtL2Request::Cancel(cancel) => {
                    L2Request::Cancel(gasp_types::Cancel::from(cancel))
                }
            },
        ))
    }

    #[tracing::instrument(skip(self))]
    async fn get_latest_processed_request_id(
        &self,
        chain: gasp_types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<u128, L2Error> {
        let chain: crate::types::subxt::Chain = chain.into();
        let storage = gasp_bindings::api::storage()
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
        chain: gasp_types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<u128, L2Error> {
        use gasp_bindings::api::runtime_types::pallet_rolldown::pallet::SequencerRights;

        let chain: crate::types::subxt::Chain = chain.into();
        let storage = gasp_bindings::api::storage()
            .rolldown()
            .sequencers_rights(chain);
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
        chain: gasp_types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<Option<[u8; 20]>, L2Error> {
        let storage = gasp_bindings::api::storage()
            .sequencer_staking()
            .selected_sequencer();
        let selected = self
            .client
            .storage()
            .at(at)
            .fetch(&storage)
            .await?
            .unwrap_or_default();

        let chain: crate::types::subxt::Chain= chain.into();
        let selected = selected
            .iter()
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
        chain: gasp_types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<u128, L2Error> {
        use gasp_bindings::api::runtime_types::pallet_rolldown::pallet::SequencerRights;

        let chain: crate::types::subxt::Chain = chain.into();
        let storage = gasp_bindings::api::storage()
            .rolldown()
            .sequencers_rights(chain);
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
    ) -> Result<gasp_types::L1Update, L2Error> {
        let call = gasp_bindings::api::runtime_apis::rolldown_runtime_api::RolldownRuntimeApi
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
        update: gasp_bindings::api::runtime_types::pallet_rolldown::messages::L1Update,
        hash: H256,
    ) -> Result<bool, L2Error> {
        let call = gasp_bindings::api::tx()
            .rolldown()
            .update_l2_from_l1(update, hash);
        self.sign_and_send(call).await
    }

    #[tracing::instrument(skip(self))]
    async fn cancel_pending_request(
        &self,
        request_id: u128,
        chain: gasp_types::Chain,
    ) -> Result<bool, L2Error> {
        let chain: crate::types::subxt::Chain = chain.into();
        let call = gasp_bindings::api::tx()
            .rolldown()
            .cancel_requests_from_l1(chain, request_id);
        self.sign_and_send(call).await
    }

    #[tracing::instrument(skip(self))]
    async fn get_pending_cancels(
        &self,
        chain: gasp_types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<Vec<u128>, L2Error> {
        let chain: crate::types::subxt::Chain = chain.into();
        let storage_entry = gasp_bindings::api::storage()
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
            .filter(|(account, _request_id, _role)| {
                account.0 == self.keypair.address().into_inner()
            })
            .map(|(_account, request_id, _role)| request_id)
            .collect::<Vec<_>>();

        tracing::debug!(
            "found {} pending cancels associated with account: {}",
            result.len(),
            hex_encode(self.keypair.address())
        );

        Ok(result)
    }

    // #[tracing::instrument(skip(self))]
    // async fn get_pending_updates(
    //     &self,
    //     at: HashOf<GaspConfig>,
    // ) -> Result<Vec<PendingUpdateWithKeys>, L2Error> {
    //     use ::subxt::ext::subxt_core::storage::address::StaticStorageKey;
    //     use gasp_bindings::api::rolldown::storage::types as gasp_types;
    //
    //     let metadata = self.client.metadata();
    //     let (_pallet, entry) = subxt_core::storage::lookup_storage_entry_details(
    //         "Rolldown",
    //         "PendingSequencerUpdates",
    //         &metadata,
    //     )?;
    //
    //     let hashers = StorageHashers::new(entry.entry_type(), metadata.types())?;
    //
    //     let iter = gasp_bindings::api::storage()
    //         .rolldown()
    //         .pending_sequencer_updates_iter();
    //
    //     let result = self
    //         .client
    //         .storage()
    //         .at(at)
    //         .iter(iter)
    //         .await?
    //         .map(|result| async {
    //             let storage_kv = result?;
    //             let update_metadata = storage_kv.value;
    //
    //             let keys = <(
    //                 StaticStorageKey<gasp_types::pending_sequencer_updates::Param0>,
    //                 StaticStorageKey<gasp_types::pending_sequencer_updates::Param1>,
    //             )>::decode_storage_key(
    //                 &mut &storage_kv.key_bytes[32..],
    //                 &mut hashers.iter(),
    //                 metadata.types(),
    //             )?;
    //             let end_dispute_period = keys.0.decoded()?;
    //             let chain = keys.1.decoded()?;
    //             let update_hash = hex_encode(update_metadata.update_hash);
    //             tracing::debug!("update found chain:{chain:?} end_dispute_period:{end_dispute_period} hash:{update_hash} update_metadata:{update_metadata:?}");
    //             Ok::<_, L2Error>((keys.0.decoded()?, chain, update_metadata))
    //         })
    //         .collect::<Vec<_>>()
    //         .await;
    //
    //     join_all(result)
    //         .await
    //         .into_iter()
    //         .collect::<Result<Vec<_>, _>>()
    // }

    #[tracing::instrument(skip(self))]
    async fn get_merkle_proof(
        &self,
        request_id: u128,
        range: (u128, u128),
        chain: gasp_types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<Vec<H256>, L2Error> {
        // let range = types::Range{ start, end };
        let chain: crate::types::subxt::Chain = chain.into();
        let call = gasp_bindings::api::runtime_apis::rolldown_runtime_api::RolldownRuntimeApi
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
        chain: gasp_types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<Vec<u8>, L2Error> {
        let chain: crate::types::subxt::Chain = chain.into();
        let call = gasp_bindings::api::runtime_apis::rolldown_runtime_api::RolldownRuntimeApi
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
        chain: gasp_types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<Option<H256>, L2Error> {
        let req = crate::types::subxt::RequestId {
            origin: crate::types::subxt::Origin::L2,
            id: request_id,
        };

        let chain: crate::types::subxt::Chain = chain.into();
        let storage = gasp_bindings::api::storage()
            .rolldown()
            .l2_requests(chain, req);
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
        finalization_type: Finalization,
    ) -> Result<HeaderStream, L2Error> {
        Ok(match finalization_type {
            Finalization::Best => self.client.backend().stream_best_block_headers(),
            Finalization::Finalized => self.client.backend().stream_finalized_block_headers(),
        }
        .await?
        .map(|elem| {
            elem.map(|(header, hash)| (header.number, hash.hash()))
                .map_err(L2Error::from)
        })
        .boxed())
    }

    #[tracing::instrument(skip(self))]
    async fn get_active_sequencers(
        &self,
        chain: gasp_types::Chain,
        at: H256,
    ) -> Result<Vec<[u8; 20]>, L2Error> {
        let storage = gasp_bindings::api::storage()
            .sequencer_staking()
            .active_sequencers();

        let chain: crate::types::subxt::Chain = chain.into();
        let active = self
            .client
            .storage()
            .at(at)
            .fetch(&storage)
            .await?
            .unwrap_or_default();

        let active = active
            .into_iter()
            .find(|(c, _)| c == &chain)
            .map(|(_, account)| account.0)
            .unwrap_or_default();

        Ok(active.into_iter().map(|elem| elem.0).collect())
    }

    #[tracing::instrument(skip(self))]
    async fn get_dispute_period(
        &self,
        chain: gasp_types::Chain,
        at: H256,
    ) -> Result<u128, L2Error> {
        let chain: crate::types::subxt::Chain = chain.into();
        let storage = gasp_bindings::api::storage()
            .rolldown()
            .dispute_period(chain.clone());
        let active = self.client.storage().at(at).fetch(&storage).await?;

        tracing::trace!("dispute period: {active:?}");
        active.ok_or(L2Error::UnknownDisputePeriodLength(chain.into()))
    }

    #[tracing::instrument(skip(self), ret)]
    async fn get_latest_created_request_id(
        &self,
        chain: gasp_types::Chain,
        at: HashOf<GaspConfig>,
    ) -> Result<Option<u128>, L2Error> {

        let storage = gasp_bindings::api::storage()
            .rolldown()
            .l2_origin_request_id();
        let latest = self.client.storage()
            .at(at)
            .fetch(&storage).await?
            .map(|elem| elem.into_iter()
                .map(|(chain, id)| (gasp_types::Chain::from(chain), id))
                .collect::<HashMap<_, _>>()
            ).unwrap_or_default();

        // tracing::trace!("dispute period: {active:?}");
        // active.ok_or(L2Error::UnknownDisputePeriodLength(chain.into()))
        // todo!()
        Ok(latest.get(&chain).cloned())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use gasp_types::L1Update;
    use hex_literal::hex;
    use serial_test::serial;
    use tracing_test::traced_test;

    //TODO: adcd test for L2Interace::deserialize_sequencer_update

    const URI: &str = "ws://localhost:9944";
    const DUMMY_PKEY: [u8; 32] =
        hex!("b9d2ea9a615f3165812e8d44de0d24da9bbd164b65c4f0573e1ce2c8dbd9c8df");
    const DUMMY_ADDR: [u8; 20] = hex!("0000000000000000000000000000000000000000");
    const ALITH_PKEY: [u8; 32] =
        hex!("5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133");
    const BALTATHAR_PKEY: [u8; 32] =
        hex!("8075991ce870b93a8870eca0c0f91913d12f47948ca0fd25b49c6fa7cdbeee8b");
    const TEST_TOKEN: [u8; 20] = hex!("c351628eb244ec633d5f21fbd6621e1a683b1181");
    const ETHEREUM: gasp_types::Chain = gasp_types::Chain::Ethereum;

    #[serial]
    #[tokio::test]
    async fn test_can_connect() {
        Gasp::new(URI, BALTATHAR_PKEY)
            .await
            .expect("can connect to gasp");
    }

    #[serial]
    #[tokio::test]
    #[traced_test]
    async fn test_can_submit_multiple_tx_in_a_row() {
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
        let mut stream = gasp.header_stream(Finalization::Best).await.unwrap();
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

        let dummy_read_rights = dummy_gasp
            .get_read_rights(gasp_types::Chain::Ethereum, at)
            .await;
        let dummy_cancel_rights = dummy_gasp
            .get_cancel_rights(gasp_types::Chain::Ethereum, at)
            .await;

        let baltathar_read_rights = baltathat_gasp
            .get_read_rights(gasp_types::Chain::Ethereum, at)
            .await;
        let baltathar_cancel_rights = baltathat_gasp
            .get_cancel_rights(gasp_types::Chain::Ethereum, at)
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

        assert!(gasp
            .withdraw(ETHEREUM, DUMMY_ADDR, TEST_TOKEN, 100, None)
            .await
            .expect("can submit withdrawal"));

        gasp.wait_for_next_block().await.unwrap();
        gasp.wait_for_next_block().await.unwrap();
        gasp.wait_for_next_block().await.unwrap();

        assert!(gasp
            .withdraw(ETHEREUM, DUMMY_ADDR, TEST_TOKEN, 100, None)
            .await
            .expect("can submit withdrawal"));

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

        assert_eq!(proofs.len(), 1_usize);
    }

    #[serial]
    #[tokio::test]
    async fn test_can_fetch_pending_cancels() {
        let gasp = Gasp::new(URI, BALTATHAR_PKEY)
            .await
            .expect("can connect to gasp");
        let at = gasp.latest_block().await.unwrap().1;
        gasp.get_pending_cancels(ETHEREUM, at)
            .await
            .expect("can fetch pending cancels");
    }

    #[serial]
    #[tokio::test]
    async fn test_can_cancel_pending_update() {
        let gasp = Gasp::new(URI, BALTATHAR_PKEY)
            .await
            .expect("can connect to gasp");

        let result = gasp
            .cancel_pending_request(u128::MAX, ETHEREUM)
            .await
            .expect("can fetch pending cancels");
        assert!(!result);
    }

    #[serial]
    #[tokio::test]
    async fn test_can_submit_and_fetch_udates() {
        let gasp = Gasp::new(URI, ALITH_PKEY)
            .await
            .expect("can connect to gasp");
        let at = gasp.latest_block().await.unwrap().1;
        let latest_req_id = gasp
            .get_latest_processed_request_id(ETHEREUM, at)
            .await
            .unwrap();

        let next_req_id = latest_req_id.saturating_add(1u128);

        let update = L1Update {
            chain: crate::types::subxt::Chain::Ethereum,
            pendingDeposits: vec![crate::types::subxt::Deposit {
                requestId: crate::types::subxt::RequestId {
                    origin: crate::types::subxt::Origin::L1,
                    id: next_req_id,
                },
                depositRecipient: DUMMY_ADDR,
                tokenAddress: DUMMY_ADDR,
                amount: gasp_types::into_l2_u256(gasp_types::U256::from(100u128)),
                timeStamp: gasp_types::into_l2_u256(gasp_types::U256::from(0u128)),
                ferryTip: gasp_types::into_l2_u256(gasp_types::U256::from(0u128)),
            }],
            pendingCancelResolutions: vec![]
        };

        let status = gasp
            .update_l1_from_l2_unsafe(update)
            .await
            .expect("can submit update");

        assert!(!status);
    }

    #[serial]
    #[tokio::test]
    async fn test_can_fetch_active_sequencers() {
        let gasp = Gasp::new(URI, ALITH_PKEY)
            .await
            .expect("can connect to gasp");
        let at = gasp.latest_block().await.unwrap().1;

        let active_sequencers = gasp.get_active_sequencers(ETHEREUM, at).await.unwrap();

        assert!(!active_sequencers.is_empty());
    }
}

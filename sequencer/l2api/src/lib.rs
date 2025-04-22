use futures::{Stream, StreamExt};
use gasp_types::{Chain, L2Request};
use primitive_types::H256;
use std::pin::Pin;
use subxt::Config;

mod l2;
pub mod mock;
pub mod signer;
use gasp_types::PendingUpdate;
pub type HashOf<T> = <T as Config>::Hash;
pub type EndDisputePeriod = u128;
pub type PendingUpdateWithKeys = (EndDisputePeriod, Chain, gasp_types::PendingUpdateMetadata);
pub type HeaderStream = Pin<Box<dyn Stream<Item = Result<(u32, H256), L2Error>> + Send + 'static>>;
pub enum Finalization {
    Best,
    Finalized,
}
pub use l2::Gasp;

pub mod types {
    // NOTE: this alias is used in multiple other files to make code more readable
    // #[allow(unused_imports)]
    // pub use gasp::api as bindings;

    pub mod subxt {
        pub use gasp_bindings::api::runtime_types::pallet_rolldown::messages::L1Update;
        #[allow(unused_imports)]
        pub use gasp_bindings::api::runtime_types::pallet_rolldown::messages::{
            CancelResolution, Chain, Deposit, Origin, RequestId,
        };
        pub use gasp_bindings::api::runtime_types::pallet_rolldown::pallet::UpdateMetadata;
        pub use gasp_bindings::api::runtime_types::sp_runtime::account::AccountId20;
    }

    pub use gasp_types::{Chain, L2Request};
    pub use primitive_types::H256;

    pub use super::Finalization;
    pub use super::HeaderStream;
    pub type BatchId = u128;

    #[derive(Clone, Copy, Debug)]
    pub struct BatchInfo {
        pub batch_id: BatchId,
        pub range: (u128, u128),
        pub merkle_root: H256,
    }
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
    #[error("unknown dispute period length")]
    UnknownDisputePeriodLength(Chain),
    #[error("unknown pending update `{0}`")]
    UnknownPendingUpdate(H256),
    #[error("missing batch with id `{0}`")]
    MissingBatch(u128),
    #[error("missing batch with id `{0}`")]
    MissingBatchForRequestId(u128),
}

#[allow(async_fn_in_trait)]
pub trait L2Interface {
    fn account_address(&self) -> [u8; 20];

    async fn get_balance(
        &self,
        chain: Chain,
        token: [u8; 20],
        account: [u8; 20],
        at: H256,
    ) -> Result<u128, L2Error>;

    async fn get_l2_request(
        &self,
        chain: Chain,
        range: u128,
        at: H256,
    ) -> Result<Option<L2Request>, L2Error>;

    async fn get_latest_batch(
        &self,
        request_id: u128,
        chain: gasp_types::Chain,
        at: H256,
    ) -> Result<Option<(types::BatchId, (u128, u128))>, L2Error>;

    async fn get_batch_range(
        &self,
        batch_id: u128,
        chain: gasp_types::Chain,
        at: H256,
    ) -> Result<Option<(u128, u128)>, L2Error>;

    async fn bisect_find_batch(
        &self,
        request_id: u128,
        chain: gasp_types::Chain,
        at: H256,
    ) -> Result<Option<types::BatchInfo>, L2Error>;

    async fn get_latest_created_request_id(
        &self,
        chain: Chain,
        at: H256,
    ) -> Result<Option<u128>, L2Error>;

    async fn get_latest_processed_request_id(
        &self,
        chain: Chain,
        at: H256,
    ) -> Result<u128, L2Error>;

    async fn get_latest_accepted_request_id(&self, chain: Chain, at: H256)
        -> Result<u128, L2Error>;

    async fn get_read_rights(&self, chain: Chain, at: H256) -> Result<u128, L2Error>;

    async fn get_selected_sequencer(
        &self,
        chain: Chain,
        at: H256,
    ) -> Result<Option<[u8; 20]>, L2Error>;

    async fn get_cancel_rights(&self, chain: Chain, at: H256) -> Result<u128, L2Error>;

    async fn deserialize_sequencer_update(
        &self,
        data: Vec<u8>,
    ) -> Result<gasp_types::L1Update, L2Error>;

    async fn cancel_pending_request(&self, request_id: u128, chain: Chain)
        -> Result<bool, L2Error>;

    async fn update_l1_from_l2(
        &self,
        update: gasp_types::L1Update,
        hash: H256,
    ) -> Result<bool, L2Error>;

    async fn get_pending_cancels(&self, chain: Chain, at: H256) -> Result<Vec<u128>, L2Error>;

    async fn get_merkle_proof(
        &self,
        request_id: u128,
        range: (u128, u128),
        chain: Chain,
        at: H256,
    ) -> Result<Vec<H256>, L2Error>;

    async fn get_merkle_root(
        &self,
        range: (u128, u128),
        chain: Chain,
        at: H256,
    ) -> Result<H256, L2Error>;

    async fn get_l2_request_hash(
        &self,
        request_id: u128,
        chain: Chain,
        at: H256,
    ) -> Result<Option<H256>, L2Error>;

    async fn header_stream(&self, finalization_type: Finalization)
        -> Result<HeaderStream, L2Error>;

    async fn get_best_block(&self) -> Result<(u32, H256), L2Error> {
        self.header_stream(Finalization::Best)
            .await?
            .next()
            .await
            .expect("infinite stream")
    }

    async fn get_latest_finalized_block(&self) -> Result<(u32, H256), L2Error> {
        self.header_stream(Finalization::Finalized)
            .await?
            .next()
            .await
            .expect("infinite stream")
    }

    async fn get_abi_encoded_request(
        &self,
        request_id: u128,
        chain: Chain,
        at: H256,
    ) -> Result<Vec<u8>, L2Error>;

    async fn get_active_sequencers(&self, chain: Chain, at: H256)
        -> Result<Vec<[u8; 20]>, L2Error>;

    async fn get_dispute_period(&self, chain: Chain, at: H256) -> Result<u128, L2Error>;

    async fn get_pending_updates(&self, at: H256) -> Result<Vec<PendingUpdate>, L2Error>;

    async fn ferry_deposit(
        &self,
        chain: Chain,
        deposit: gasp_types::Deposit,
    ) -> Result<bool, L2Error>;
}

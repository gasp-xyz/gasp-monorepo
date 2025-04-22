use crate::types::Chain;
use crate::types::Finalization;
use crate::types::L2Request;
use crate::L2Error;
use gasp_types::H256;

mockall::mock! {

    pub L2 {}


#[allow(clippy::type_complexity)]
impl crate::L2Interface for L2{

    async fn get_best_block(&self) -> Result<(u32, H256), L2Error>;

    async fn get_latest_finalized_block(&self) -> Result<(u32, H256), L2Error>;

    async fn get_latest_batch(
        &self,
        request_id: u128,
        chain: gasp_types::Chain,
        at: H256,
    ) -> Result<Option<(crate::types::BatchId, (u128, u128))>, L2Error>;

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
    ) -> Result<Option<crate::types::BatchInfo>, L2Error> ;


    async fn get_balance(&self, chain: gasp_types::Chain, token: [u8; 20], account: [u8; 20], at: H256) -> Result<u128, L2Error>;

    async fn ferry_deposit(&self, chain: Chain, deposit: gasp_types::Deposit) -> Result<bool, L2Error>;

    fn account_address(&self) -> [u8; 20];

    async fn get_l2_request(
        &self,
        chain: Chain,
        request_id: u128,
        at: H256,
    ) -> Result<Option<L2Request>, L2Error>;

    //TODO: rename
    async fn get_latest_created_request_id(&self, chain: Chain, at: H256) -> Result<Option<u128>, L2Error>;

    //TODO: rename
    async fn get_latest_processed_request_id(
        &self,
        chain: Chain,
        at: H256,
    ) -> Result<u128, L2Error>;

    // TODO: maybe rename
    async fn get_latest_accepted_request_id(
        &self,
        chain: Chain,
        at: H256,
    ) -> Result<u128, L2Error>;

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
        -> Result<crate::types::HeaderStream, L2Error>;

    async fn get_abi_encoded_request(
        &self,
        request_id: u128,
        chain: Chain,
        at: H256,
    ) -> Result<Vec<u8>, L2Error>;

    async fn get_active_sequencers(&self, chain: Chain, at: H256)
        -> Result<Vec<[u8; 20]>, L2Error>;

    async fn get_dispute_period(&self, chain: Chain, at: H256) -> Result<u128, L2Error>;

    async fn get_pending_updates(&self, at: H256) -> Result<Vec<gasp_types::PendingUpdate>, L2Error>;
}

}

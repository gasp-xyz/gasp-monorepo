use crate::types::Chain;
use crate::types::Finalization;
use crate::types::L2Request;
use crate::L2Error;
use gasp_types::H256;

mockall::mock! {

    pub L2 {}


impl crate::L2Interface for L2{
    fn account_address(&self) -> [u8; 20];

    async fn get_l2_request(
        &self,
        chain: Chain,
        range: u128,
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

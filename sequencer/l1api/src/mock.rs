use crate::L1Error;
use futures::stream::BoxStream;
use gasp_types::H256;

mockall::mock! {

    pub L1 {}

    #[allow(clippy::type_complexity)]
    impl crate::L1Interface for L1{
        async fn subscribe_deposit<'a>(&'a self) -> Result<BoxStream<'a, u128>, L1Error>;
        async fn subscribe_new_batch<'a>(&'a self) -> Result<BoxStream<'a, (H256, (u128, u128))>, L1Error>;
        async fn get_deposit(&self, request_id: u128) -> Result<Option<crate::types::Deposit>, L1Error>;
        async fn erc20_balance(&self, token: [u8; 20], account: [u8; 20]) -> Result<u128, L1Error>;
        async fn ferry_withdrawal(&self, withdrawal: crate::types::Withdrawal) -> Result<H256, L1Error>;
        async fn native_balance(&self, account: [u8; 20]) -> Result<u128, L1Error>;
        async fn get_status(&self, request_hash: H256) -> Result<crate::types::RequestStatus, L1Error>;
        async fn get_update(&self, start: u128, end: u128) -> Result<crate::types::abi::L1Update, L1Error>;
        async fn get_update_hash(&self, start: u128, end: u128) -> Result<H256, L1Error>;
        async fn get_latest_reqeust_id(&self) -> Result<Option<u128>, L1Error>;
        async fn get_merkle_root(
            &self,
            request_id: u128,
        ) -> Result<Option<([u8; 32], (u128, u128))>, L1Error>;
        async fn get_latest_finalized_request_id(&self) -> Result<Option<u128>, L1Error>;

        async fn close_cancel(
            &self,
            cancel: gasp_types::Cancel,
            merkle_root: H256,
            proof: Vec<H256>,
        ) -> Result<H256, L1Error>;

        async fn close_withdrawal(
            &self,
            withdrawal: gasp_types::Withdrawal,
            merkle_root: H256,
            proof: Vec<H256>,
        ) -> Result<H256, L1Error>;

    }
}

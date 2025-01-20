use alloy::providers::PendingTransactionError;
use primitive_types::H256;

mod lru;
mod rolldown_contract;

pub use lru::CachedL1Interface;
pub use rolldown_contract::RolldownContract;

pub mod types {
    pub use bindings::rolldown::IRolldownPrimitives::Cancel;
    pub use bindings::rolldown::IRolldownPrimitives::L1Update;
}

#[derive(Debug, thiserror::Error)]
pub enum L1Error {
    #[error("Invalid range")]
    InvalidRange,
    #[error("Overflow error")]
    OverflowError,
    #[error("alloy error")]
    Alloy(#[from] alloy::contract::Error),
    #[error("alloy error")]
    TransportAlloy(#[from] alloy::transports::TransportError),
    #[error("transaction error")]
    TxSendError(#[from] PendingTransactionError),
}

pub trait L1Interface {
    fn account_address(&self) -> [u8; 20];
    async fn get_native_balance(&self, address: [u8; 20]) -> Result<u128, L1Error>;
    async fn is_closed(&self, request_hash: H256) -> Result<bool, L1Error>;
    async fn get_update(&self, start: u128, end: u128) -> Result<types::L1Update, L1Error>;
    async fn get_update_hash(&self, start: u128, end: u128) -> Result<H256, L1Error>;
    async fn get_latest_reqeust_id(&self) -> Result<Option<u128>, L1Error>;
    async fn get_merkle_root(&self, request_id: u128) -> Result<([u8; 32], (u128, u128)), L1Error>;
    async fn get_latest_finalized_request_id(&self) -> Result<Option<u128>, L1Error>;
    async fn estimate_gas_in_wei(&self) -> Result<(u128, u128), L1Error>;
    async fn close_cancel(
        &self,
        cancel: types::Cancel,
        merkle_root: H256,
        proof: Vec<H256>,
    ) -> Result<H256, L1Error>;
}

#[cfg(test)]
mod test {
    use super::*;
    use hex_literal::hex;
    use serial_test::serial;

    const URI: &str = "http://localhost:8545";
    const ROLLDOWN_ADDRESS: [u8; 20] = hex!("1429859428C0aBc9C2C47C8Ee9FBaf82cFA0F20f");
    const ALICE_PKEY: [u8; 32] =
        hex!("dbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97");

    #[serial]
    #[tokio::test]
    async fn test_can_connect() {
        RolldownContract::new(URI, ROLLDOWN_ADDRESS, ALICE_PKEY)
            .await
            .unwrap();
    }

    #[serial]
    #[tokio::test]
    async fn test_can_latest_request_id() {
        let rolldown = RolldownContract::new(URI, ROLLDOWN_ADDRESS, ALICE_PKEY)
            .await
            .unwrap();
        rolldown.deposit(1000, 10).await.unwrap();
        rolldown
            .get_latest_reqeust_id()
            .await
            .expect("can fetch request");
    }

    #[serial]
    #[tokio::test]
    async fn test_can_fetch_balance() {
        let rolldown = RolldownContract::new(URI, ROLLDOWN_ADDRESS, ALICE_PKEY)
            .await
            .unwrap();

        let balance = rolldown
            .get_native_balance(hex!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266"))
            .await
            .unwrap();
        assert!(balance > 0u128);
    }
}

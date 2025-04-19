use primitive_types::H256;
use serde::Deserialize;
use serde_query::{DeserializeQuery, Query};

#[derive(Debug, thiserror::Error)]
pub enum StashError {
    #[error("Http error")]
    HttpError(#[from] reqwest::Error),
    #[error("Deserialization error")]
    DeserializationError(#[from] serde_json::Error),
}

pub struct Stash {
    uri: String,
}

impl Stash {
    pub fn new(uri: String) -> Stash {
        Self { uri }
    }
}

impl StashInterface for Stash {
    #[tracing::instrument(skip(self))]
    async fn get_withdrawal_status(
        &self,
        withdrawal_hash: H256,
    ) -> Result<WithdrawalStatus, StashError> {
        let uri = format!(
            "{uri}/tracing/type/withdrawal/tx/{withdrawal_hash}",
            uri = self.uri
        );
        let raw_response = reqwest::get(uri).await?.text().await?;
        tracing::trace!("stash response: {raw_response}");
        let status: WithdrawalStatus =
            serde_json::from_str::<Query<WithdrawalStatus>>(&raw_response)?.into();
        Ok(status)
    }
}

#[allow(async_fn_in_trait)]
pub trait StashInterface {
    async fn get_withdrawal_status(
        &self,
        withdrawal_hash: H256,
    ) -> Result<WithdrawalStatus, StashError>;
}

#[derive(Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub enum CreatedBy {
    Frontend,
    Other,
}

#[derive(Deserialize, DeserializeQuery, Debug, Clone)]
pub struct WithdrawalStatus {
    #[query(".transaction.createdBy")]
    pub created_by: CreatedBy,
}

pub mod mock {
    use super::*;
    mockall::mock! {

        pub Stash {}

        #[allow(clippy::type_complexity)]
        impl StashInterface for Stash{
            async fn get_withdrawal_status(&self, withdrawal_hash: H256) -> Result<WithdrawalStatus, StashError>;

        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_parse_created_by_frontend() {
        let text_response = r#"
        {
            "transaction": {
                "requestId": 1,
                "txHash": "0x0fbe055def1246edc5527effcd148121384ec0980f7dada0e1f86e2e005e89af",
                "address": "0xf525b86742d20ca26069d39c0c5cbc7fcd6148ed",
                "created": 1736776886000,
                "updated": 1736776891649,
                "status": "BatchedForL1",
                "type": "withdrawal",
                "chain": "Ethereum",
                "amount": "0.3",
                "asset_chainId": "unknown",
                "asset_address": "0x2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3",
                "proof": "0x00",
                "calldata": "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000928f1040adb982d3ab32a62dc8eda57e9b81b4dd000000000000000000000000c351628eb244ec633d5f21fbd6621e1a683b118100000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000",
                "createdBy": "frontend",
                "closedBy": null
            }
        }"#;

        let status: WithdrawalStatus =
            serde_json::from_str::<Query<WithdrawalStatus>>(text_response)
                .unwrap()
                .into();
        assert_eq!(status.created_by, CreatedBy::Frontend);
    }

    #[test]
    fn test_can_parse_created_by_other() {
        let text_response = r#"
        {
            "transaction": {
                "requestId": 1,
                "txHash": "0x0fbe055def1246edc5527effcd148121384ec0980f7dada0e1f86e2e005e89af",
                "address": "0xf525b86742d20ca26069d39c0c5cbc7fcd6148ed",
                "created": 1736776886000,
                "updated": 1736776891649,
                "status": "BatchedForL1",
                "type": "withdrawal",
                "chain": "Ethereum",
                "amount": "0.3",
                "asset_chainId": "unknown",
                "asset_address": "0x2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3",
                "proof": "0x00",
                "calldata": "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000928f1040adb982d3ab32a62dc8eda57e9b81b4dd000000000000000000000000c351628eb244ec633d5f21fbd6621e1a683b118100000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000",
                "createdBy": "other",
                "closedBy": null
            }
        }"#;

        let status: WithdrawalStatus =
            serde_json::from_str::<Query<WithdrawalStatus>>(text_response)
                .unwrap()
                .into();
        assert_eq!(status.created_by, CreatedBy::Other);
    }
}

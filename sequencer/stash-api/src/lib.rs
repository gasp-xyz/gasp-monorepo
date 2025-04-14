use primitive_types::H256;
use serde::Deserialize;
use serde::Serialize;

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
        let status: WithdrawalStatus = serde_json::from_str(&raw_response)?;
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

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub enum CreatedBy {
    Frontend,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WithdrawalStatus {
    pub created_by: CreatedBy,
}

#[cfg(test)]
mod mock {
    use super::*;
    mockall::mock! {

        pub Stash {}

        #[allow(clippy::type_complexity)]
        impl StashInterface for Stash{
            async fn get_withdrawal_status(&self, withdrawal_hash: H256) -> Result<WithdrawalStatus, StashError>;

        }
    }
}

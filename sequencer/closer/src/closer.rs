#![allow(dead_code)]

use gasp_types::Withdrawal;
use l1api::types::RequestStatus;
use l1api::{L1Error, L1Interface};
use l2api::types::{BatchInfo, Chain, H256};
use l2api::{L2Error, L2Interface};
use tokio::sync::mpsc;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("L1 error")]
    L1(#[from] L1Error),
    #[error("L2 error")]
    L2(#[from] L2Error),
    #[error("Could not find batch for request id `{0}`")]
    NoBatchForL2RequestId(u128),
    #[error("Could not find batch for request id `{0}`")]
    L2RequestDoesNotExists(u128),
    #[error("Could not find batch for request id `{0}`")]
    L2RequestIsNotWithdrawal(u128),
}

pub type CloserResult<T> = Result<T, Error>;

pub struct Closer<L1, L2> {
    l1: L1,
    l2: L2,
    chain: Chain,
    input: mpsc::Receiver<Withdrawal>,
}

impl<L1, L2> Closer<L1, L2>
where
    L1: L1Interface,
    L2: L2Interface,
{
    pub fn new(chain: Chain, l1: L1, l2: L2, input: mpsc::Receiver<Withdrawal>) -> Self {
        Closer {
            l1,
            l2,
            chain,
            input,
        }
    }

    async fn close_single_withdrawal(&self, withdrawal: Withdrawal, at: H256) -> Result<(), Error> {
        let request_id = withdrawal.request_id.id.try_into().unwrap();
        let (batch, proof) = self.get_batch_and_proof(request_id, at).await?;

        let close_withdrawal_fut = self
            .l1
            .close_withdrawal(withdrawal, batch.merkle_root, proof);

        let get_status_after_close_fut = self.l1.get_status(withdrawal.withdrawal_hash());

        match (
            close_withdrawal_fut.await,
            get_status_after_close_fut.await?,
        ) {
            (Ok(hash), _) => {
                tracing::info!(
                    "successfully closed withdrawal {} tx hash {}",
                    request_id,
                    hash
                );
                Ok(())
            }
            (Err(_), RequestStatus::Closed) => {
                tracing::info!(
                    "failed to close withdrawal {} - someone else closed it ",
                    request_id,
                );
                Ok(())
            }
            (Err(err), _) => {
                tracing::error!("failed to close withdrawal {request_id} err: {err}",);
                Err(err.into())
            }
        }
    }

    async fn close_withdrawals(&self, withdrawals: Vec<Withdrawal>) -> Result<(), Error> {
        let (_, at) = self.l2.get_best_block().await?;
        for w in withdrawals {
            self.close_single_withdrawal(w, at).await?;
        }
        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        let mut buffer = Vec::<Withdrawal>::with_capacity(25);

        loop {
            buffer.clear();
            let count = self.input.recv_many(&mut buffer, 25).await;
            if count > 0 {
                tracing::debug!("found {count} withdrawals to close");
                self.close_withdrawals(buffer.clone()).await?;
            }
        }
    }

    async fn get_batch_and_proof(
        &self,
        l2_request_id: u128,
        at: H256,
    ) -> CloserResult<(BatchInfo, Vec<H256>)> {
        let batch = self
            .l2
            .bisect_find_batch(l2_request_id, self.chain, at)
            .await?
            .ok_or(Error::NoBatchForL2RequestId(l2_request_id))?;
        let proof = self
            .l2
            .get_merkle_proof(l2_request_id, batch.range, self.chain, at)
            .await?;
        Ok((batch, proof))
    }
}

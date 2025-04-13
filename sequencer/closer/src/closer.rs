#![allow(dead_code)]
use gasp_types::Withdrawal;
use l1api::{L1Error, L1Interface};
use l2api::{L2Error, L2Interface};
use l2api::types::{BatchId, BatchInfo, Chain, L2Request, H256};
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
    input: mpsc::Receiver<Withdrawal>
}

impl<L1, L2> Closer<L1, L2>
where
    L1: L1Interface,
    L2: L2Interface,
{

    pub fn new(chain: Chain, l1: L1, l2: L2, input: mpsc::Receiver<Withdrawal>) -> Self {
        Closer { l1, l2, chain, input }
    }

    pub async fn run(& mut self) -> Result<(), Error> {
        while let Some(w) = self.input.recv().await {
            let trace_span = tracing::trace_span!("closer::run", withdrawal = format!("{}", w));
            let _trace_guard= trace_span.enter();
            match self.l1.get_status(w.withdrawal_hash()).await?{
                l1api::types::RequestStatus::Pending => {
                    let (_, at) = self.l2.get_best_block().await?;
                    match self.close_withdrawal(w.request_id.id.try_into().unwrap(), at).await{
                        Ok(hash) => {
                            tracing::info!("successfully closed withdrawal {} tx hash {}", w.request_id.id, hash);
                        },
                        Err(err) => {
                            match self.l1.get_status(w.withdrawal_hash()).await?{
                                l1api::types::RequestStatus::Pending => {
                                    tracing::error!("couldnt close withdrawal {}", w.request_id.id);
                                    return Err(err);
                                },
                                l1api::types::RequestStatus::Ferried(addr) => {
                                    tracing::error!("couldnt close withdrawal {} - it got ferried by {}", w.request_id.id, hex::encode(addr));
                                },
                                l1api::types::RequestStatus::Closed => {
                                    tracing::error!("couldnt close withdrawal {} - it got closed in meantime", w.request_id.id);
                                },
                            }
                        },
                    }
                },
                l1api::types::RequestStatus::Ferried(addr) => {
                    tracing::info!("withdrawal is ferried by {} - skipping", hex::encode(addr));
                },
                l1api::types::RequestStatus::Closed => {
                    tracing::info!("withdrawal is already closed");
                },
            }

        }
        Ok(())

    }

    async fn get_batch_and_proof(&self, l2_request_id: u128, at: H256) -> CloserResult<(BatchInfo, Vec<H256>)> {
        let batch = self.l2.bisect_find_batch(l2_request_id, self.chain, at).await?
            .ok_or(Error::NoBatchForL2RequestId(l2_request_id))?;
        let proof = self.l2.get_merkle_proof(l2_request_id, batch.range, self.chain, at).await?;
        Ok((batch, proof))
    }

    pub async fn close_withdrawal(&self, l2_request_id: u128, at: H256) -> CloserResult<H256> {
        let withdrawal = match self.l2.get_l2_request(self.chain, l2_request_id, at).await?{
            Some(L2Request::Withdrawal(w)) => {
                Ok(w)
            }, 
            Some(req) => {
                Err(Error::L2RequestIsNotWithdrawal(l2_request_id))
            }, 
            None => {
                Err(Error::L2RequestDoesNotExists(l2_request_id))
            }
        }?;
        let batch = self.l2.bisect_find_batch(withdrawal.request_id.id.try_into().unwrap(), self.chain, at).await?
            .ok_or(Error::NoBatchForL2RequestId(l2_request_id))?;

        let proof = self.l2.get_merkle_proof(l2_request_id, batch.range, self.chain, at).await?;
        Ok(self.l1.close_withdrawal(withdrawal, batch.merkle_root, proof).await?)
    }

}


#![allow(dead_code)]
use l1api::{L1Error, L1Interface};
use l2api::{L2Error, L2Interface};
use l2api::types::{BatchId, BatchInfo, Chain, L2Request, H256};

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
}

impl<L1, L2> Closer<L1, L2>
where
    L1: L1Interface,
    L2: L2Interface,
{

    pub fn new(chain: Chain, l1: L1, l2: L2) -> Self {
        Closer { l1, l2, chain }
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


use futures::StreamExt;
use gasp_types::{Chain, L2Request, H256};
use gasp_types::{Withdrawal, U256};
use l1api::types::RequestStatus;
use l1api::L1Interface;
use l2api::L2Interface;
use tokio::sync::mpsc;

use crate::ferry::FerryAction;

#[derive(thiserror::Error, Debug)]
pub enum CleanerError {
    #[error("L1Error `{0}`")]
    L1Error(#[from] l1api::L1Error),

    #[error("L2Error `{0}`")]
    L2Error(#[from] l2api::L2Error),

    #[error("Reqeust `{request_id:?}` not found for chain `{chain:?}`")]
    RequestIdDoesNotExistsOnL2 { request_id: U256, chain: Chain },

    #[error("Unknown merkle root for finalized request id `{0}`")]
    UnknownMerkleRootForFinalizedRequestId(U256),

    #[error("Reqeust `{request_id:?}` not found for chain `{chain:?}`")]
    WithdrawalIdDoesNotExistsOnL2 { request_id: U256, chain: Chain },

    #[error("Sink send error")]
    SinkSendError(#[from] tokio::sync::mpsc::error::SendError<FerryAction>),
}

pub type CleanerResult<T> = Result<T, CleanerError>;

//TODO: rename to Cleaner
pub struct FerryCleaner<L1, L2> {
    chain: gasp_types::Chain,
    l1: L1,
    l2: L2,
    sink: mpsc::Sender<FerryAction>,
    latest_processed: u128,
    account: [u8; 20],
}

impl<L1, L2> FerryCleaner<L1, L2>
where
    L1: L1Interface,
    L2: L2Interface,
{
    pub fn new(
        chain: gasp_types::Chain,
        l1: L1,
        l2: L2,
        account: [u8; 20],
        sink: mpsc::Sender<FerryAction>,
        offset: u64,
    ) -> Self {
        FerryCleaner {
            chain,
            l1,
            l2,
            sink,
            account,
            latest_processed: offset as u128,
        }
    }

    pub async fn get_ferried_withdrawal(
        &self,
        id: u128,
        at: H256,
    ) -> CleanerResult<Option<Withdrawal>> {
        match self.l2.get_l2_request(self.chain, id, at).await? {
            Some(L2Request::Withdrawal(w)) => {
                match self.l1.get_status(w.withdrawal_hash()).await? {
                    RequestStatus::Ferried(addr) if addr == self.account => {
                        Ok::<_, CleanerError>(Some(w))
                    }
                    _ => Ok(None),
                }
            }
            _ => Ok(None),
        }
    }

    pub async fn run(&mut self) -> CleanerResult<()> {
        let mut stream = self.l2.header_stream(l2api::Finalization::Best).await?;

        while let Some(elem) = stream.next().await {
            let (_nr, at) = elem?;

            if let Some(range_end) = self.l1.get_latest_finalized_request_id().await? {
                if range_end <= self.latest_processed {
                    continue;
                }
                let mut latest = self.latest_processed;
                let range_start = std::cmp::min(latest, range_end);

                let chunks = crate::utils::get_chunks(range_start, range_end, 25);
                for (id, range) in chunks.iter().enumerate() {
                    tokio::time::sleep(std::time::Duration::from_secs_f64(0.25)).await;
                    tracing::info!(
                        "looking for ferried withdrawals {range:?} batch {id} / {chunks_count} (last : {range_end})",
                        id = id + 1,
                        chunks_count = chunks.len()
                    );
                    let queries = (range.0..range.1)
                        .map(|elem| self.get_ferried_withdrawal(elem, at))
                        .collect::<Vec<_>>();
                    for withdrawal in futures::future::try_join_all(queries)
                        .await?
                        .into_iter()
                        .filter_map(|elem| elem)
                    {
                        self.sink
                            .send(FerryAction::CloseFerriedWithdrawal { withdrawal })
                            .await?;
                        latest = withdrawal.request_id.id.try_into().unwrap();
                    }
                }
                self.latest_processed = latest;
            }
        }
        tracing::warn!("header stream ended");
        Ok(())
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_generate_chunks() {}
}

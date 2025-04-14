use std::time::Duration;

use futures::StreamExt;
use gasp_types::Withdrawal;
use gasp_types::{L2Request, H256};
use l1api::types::RequestStatus;
use l1api::L1Interface;
use l2api::L2Interface;
use tokio::sync::mpsc;

#[derive(thiserror::Error, Debug)]
pub enum HunterError {
    #[error("L1Error `{0}`")]
    L1(#[from] l1api::L1Error),

    #[error("L2Error `{0}`")]
    L2(#[from] l2api::L2Error),

    #[error("Sink send error")]
    SinkSendError(#[from] tokio::sync::mpsc::error::SendError<Withdrawal>),
}

pub type HunterResult<T> = Result<T, HunterError>;

//TODO: rename to Hunter
pub struct FerryHunter<L1, L2> {
    chain: gasp_types::Chain,
    l1: L1,
    l2: L2,
    sink: mpsc::Sender<Withdrawal>,
    latest_processed: u128,
    chunk_size: usize,
    time_between_queries: Duration,
    replica_id: u128,
    replica_count: u128,
}

#[allow(clippy::too_many_arguments)]
impl<L1, L2> FerryHunter<L1, L2>
where
    L1: L1Interface,
    L2: L2Interface,
{
    pub fn new(
        chain: gasp_types::Chain,
        l1: L1,
        l2: L2,
        chunk_size: usize,
        offset: u128,
        time_between_queries: Duration,
        sink: mpsc::Sender<Withdrawal>,
        replica_id: u128,
        replica_count: u128,
    ) -> Self {
        FerryHunter {
            chain,
            l1,
            l2,
            sink,
            latest_processed: offset,
            chunk_size,
            time_between_queries,
            replica_id,
            replica_count,
        }
    }

    #[tracing::instrument(level = "debug", skip(self, at))]
    pub async fn get_pending_withdrawal(
        &self,
        id: u128,
        at: H256,
    ) -> HunterResult<Option<Withdrawal>> {
        match self.l2.get_l2_request(self.chain, id, at).await? {
            Some(L2Request::Withdrawal(w)) => {
                let status = self.l1.get_status(w.withdrawal_hash()).await?;
                tracing::info!("{status}");
                match status {
                    RequestStatus::Pending => Ok::<_, HunterError>(Some(w)),
                    _ => Ok(None),
                }
            }
            _ => Ok(None),
        }
    }

    pub async fn run(&mut self) -> HunterResult<()> {
        let mut stream = self.l2.header_stream(l2api::Finalization::Best).await?;

        let (_nr, at) = stream.next().await.expect("infinite stream")?;
        let first = std::cmp::max(1, self.latest_processed);

        match self.l1.get_latest_finalized_request_id().await? {
            Some(latest_closable) if latest_closable < first => {
                tracing::info!(
                    "offset:{first} is higher than latest available request id:{latest_closable}"
                );
            }
            Some(latest_closable) => {
                let chunks = common::get_chunks(first, latest_closable, self.chunk_size);

                for (id, range) in chunks.iter().enumerate() {
                    tokio::time::sleep(self.time_between_queries).await;
                    tracing::info!(
                        "looking for pending withdrawals {range:?} batch {id} / {chunks_count} (last : {latest_closable})",
                        id = id + 1,
                        chunks_count = chunks.len()
                    );
                    let queries = (range.0..=range.1)
                        .filter(|id| id % self.replica_count == self.replica_id)
                        .map(|elem| self.get_pending_withdrawal(elem, at))
                        .collect::<Vec<_>>();
                    for w in futures::future::try_join_all(queries)
                        .await?
                        .into_iter()
                        .flatten()
                    {
                        self.sink.send(w).await?;
                    }
                }
            }
            None => {
                tracing::info!("no withdrawals yet found");
            }
        }
        tracing::info!("finished");
        Ok(())
    }
}

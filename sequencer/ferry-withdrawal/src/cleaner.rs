use futures::{FutureExt, StreamExt};
use gasp_types::{Chain, L2Request, Origin, RequestId, H256};
use gasp_types::{Withdrawal, U256};
use itertools::Itertools;
use l1api::types::RequestStatus;
use l1api::L1Interface;
use l2api::L2Interface;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Mutex;
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

    #[tracing::instrument(skip_all, ret)]
    async fn get_requests_to_ferry(&self, l2_state: H256) -> CleanerResult<Option<(u128, u128)>> {
        let latest_request_id_on_l1 = self.l1.get_latest_finalized_request_id().await?;
        let latest_request_id_on_l2 = self
            .l2
            .get_latest_created_request_id(self.chain, l2_state)
            .await?;
        Ok(match (latest_request_id_on_l1, latest_request_id_on_l2) {
            (Some(l1_request_id), Some(l2_request_id)) if l2_request_id > l1_request_id => {
                Some((l1_request_id + 1, l2_request_id))
            }
            (None, Some(l2_request_id)) => Some((1, l2_request_id)),
            _ => None,
        })
    }

    #[tracing::instrument(skip_all)]
    async fn fetch_l2_requests(&self, range: Vec<u128>, at: H256) -> CleanerResult<Vec<L2Request>> {
        tracing::trace!(
            "fetching l2 requests for range {first:?} .. {last:?}",
            first = range.first(),
            last = range.last()
        );
        let chain = self.chain;
        let futures = range
            .into_iter()
            .filter(|id| *id > self.latest_processed)
            .map(|id| {
                self.l2.get_l2_request(self.chain, id, at).map(move |elem| {
                    elem.map(|elem| {
                        elem.ok_or(CleanerError::RequestIdDoesNotExistsOnL2 {
                            request_id: id.into(),
                            chain,
                        })
                    })
                })
            })
            .collect::<Vec<_>>();

        Ok(futures::future::join_all(futures)
            .await
            .into_iter()
            .collect::<Result<Result<Vec<_>, _>, _>>()??)
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


    pub fn get_chunks(&self, start: u128, end: u128, chunk_size: usize) -> Vec<(u128, u128)> {
        (start..end)
            .chunks(chunk_size)
            .into_iter()
            .map(|elem| {
                let mut x = elem.into_iter();
                let first = x.nth(0).expect("at least on element in chunk");
                let last = x.last().unwrap_or(first);
                (start, last)
            })
            .collect::<Vec<_>>()
    }

    pub async fn run(&mut self) -> CleanerResult<()> {
        let mut stream = self.l2.header_stream(l2api::Finalization::Best).await?;
        //TODO: replace with wait for the next block
        while let Some(elem) = stream.next().await {
            let (_nr, at) = elem?;

            if let Some(range_end) = self.l1.get_latest_finalized_request_id().await? {
                let mut latest = self.latest_processed;
                let range_start = std::cmp::min(latest, range_end);

                let chunks =self.get_chunks(range_start, range_end, 50);
                for (id, range) in chunks.iter().enumerate() {
                    tracing::info!( "fetching l2 requests for range {range:?} batch {id} / {chunks_count}", id = id + 1, chunks_count = chunks.len());
                    let queries= (range.0..range.1).map(|elem| self.get_ferried_withdrawal(elem, at)).collect::<Vec<_>>();
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

use futures::StreamExt;
use gasp_types::{Chain, L2Request, Withdrawal, H256};
use l1api::{types::RequestStatus, L1Error, L1Interface, Subscription};
use l2api::{L2Error, L2Interface};
use tokio::sync::mpsc;
use tokio::time::{timeout, Duration, Timeout};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("L1 error")]
    L1(#[from] L1Error),
    #[error("L2 error")]
    L2(#[from] L2Error),
    #[error("Sink send error")]
    NoBatchForL2RequestId(#[from] mpsc::error::SendError<Withdrawal>),
    #[error("Withdrawals subscription failed")]
    SubscriptionFailed,
}

pub struct WithdrawalSubscriber<L1, L2> {
    l1: L1,
    l2: L2,
    chain: Chain,
    sink: mpsc::Sender<Withdrawal>,
    chunk_size: usize,
}

impl<L1, L2> WithdrawalSubscriber<L1, L2>
where
    L1: L1Interface,
    L2: L2Interface,
{
    pub fn new(
        chain: Chain,
        l1: L1,
        l2: L2,
        sink: mpsc::Sender<Withdrawal>,
        chunk_size: usize,
    ) -> Self {
        WithdrawalSubscriber {
            l1,
            l2,
            chain,
            sink,
            chunk_size,
        }
    }

    #[tracing::instrument(level = "debug", skip(self, at), ret)]
    pub async fn get_pending_withdrawal(
        &self,
        id: u128,
        at: H256,
    ) -> Result<Option<Withdrawal>, Error> {
        match self.l2.get_l2_request(self.chain, id, at).await? {
            Some(L2Request::Withdrawal(w)) => {
                match self.l1.get_status(w.withdrawal_hash()).await? {
                    RequestStatus::Pending => Ok::<_, Error>(Some(w)),
                    _ => Ok(None),
                }
            }
            _ => Ok(None),
        }
    }

    pub async fn run(&mut self, subscription: Subscription) -> Result<(), Error> {
        let mut subscription = self.l1.subscribe_new_batch(subscription).await?;

        loop {
            let elem = timeout(Duration::from_secs_f64(30.0), subscription.next()).await;
            match elem {
                Err(Timeout ) => {
                    tracing::info!("keep alive");
                },
                Ok(None) => {
                    return Err(Error::SubscriptionFailed);
                }
                Ok(Some((merkle_root, range))) => {
                    tracing::info!("new batch found {merkle_root} with request {range:?}");
                    let (_, at) = self.l2.get_best_block().await?;
                    for (start, end) in common::get_chunks(range.0, range.1, self.chunk_size) {
                        let queries = (start..=end)
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
            }
        }
        Ok(())
    }
}

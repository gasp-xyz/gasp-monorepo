use futures::StreamExt;
use gasp_types::{Chain, L2Request, Withdrawal, H256};
use l1api::{types::RequestStatus, L1Error, L1Interface};
use l2api::{L2Error, L2Interface};
use tokio::sync::mpsc;
use tokio::time::{timeout, Duration};

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
    #[error("Request does not exists `{0}`")]
    L2RequestDoesNotExists(u128),
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
            Some(_) => Ok(None),
            None => Err(Error::L2RequestDoesNotExists(id)),
        }
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        let mut subscription = self.l1.subscribe_new_batch().await?;

        loop {
            let elem = timeout(Duration::from_secs_f64(30.0), subscription.next()).await;
            match elem {
                Err(_timeout) => {
                    tracing::info!("keep alive");
                }
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
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use futures::stream::{self, StreamExt};
    use gasp_types::RequestId;
    use l1api::mock::MockL1;
    use l2api::mock::MockL2;
    use mockall::predicate::{always, eq};
    use std::iter::once;

    const CHAIN: gasp_types::Chain = gasp_types::Chain::Ethereum;

    fn dummy_withdrawal(id: u128) -> Withdrawal {
        Withdrawal {
            request_id: RequestId {
                origin: gasp_types::Origin::L2,
                id: id.into(),
            },
            recipient: [0u8; 20],
            token_address: [1u8; 20],
            amount: 123u128.into(),
            ferry_tip: 23u128.into(),
        }
    }

    fn dummy_withdrawal_request(id: u128) -> L2Request {
        L2Request::Withdrawal(dummy_withdrawal(id))
    }

    #[tokio::test]
    async fn test_extracts_withdrawals_from_batch_subscription() {
        let mut l1mock = MockL1::new();
        let mut l2mock = MockL2::new();

        let batch = (H256::default(), (1u128, 5u128));
        let (sender, mut receiver) = mpsc::channel(100);

        l1mock
            .expect_subscribe_new_batch()
            .returning(move || Ok(stream::iter(once(batch)).boxed()));

        l1mock
            .expect_get_status()
            .returning(|_| Ok(RequestStatus::Pending));

        l2mock
            .expect_get_best_block()
            .returning(|| Ok((1, H256::zero())));
        l2mock
            .expect_get_l2_request()
            .returning(|_, id, _| Ok(Some(dummy_withdrawal_request(id))));

        let _t = tokio::spawn(async move {
            WithdrawalSubscriber::new(CHAIN, l1mock, l2mock, sender, 10)
                .run()
                .await
                .unwrap();
        });

        assert_eq!(receiver.recv().await, Some(dummy_withdrawal(1u128)));
        assert_eq!(receiver.recv().await, Some(dummy_withdrawal(2u128)));
        assert_eq!(receiver.recv().await, Some(dummy_withdrawal(3u128)));
        assert_eq!(receiver.recv().await, Some(dummy_withdrawal(4u128)));
        assert_eq!(receiver.recv().await, Some(dummy_withdrawal(5u128)));
    }

    #[tokio::test]
    async fn test_ignores_closed_withdrawals() {
        let mut l1mock = MockL1::new();
        let mut l2mock = MockL2::new();

        let batch = (H256::default(), (1u128, 5u128));
        let (sender, mut receiver) = mpsc::channel(100);

        l1mock
            .expect_subscribe_new_batch()
            .returning(move || Ok(stream::iter(once(batch)).boxed()));

        l1mock
            .expect_get_status()
            .with(eq(dummy_withdrawal(2).withdrawal_hash()))
            .returning(|_| Ok(RequestStatus::Closed));
        l1mock
            .expect_get_status()
            .with(eq(dummy_withdrawal(4).withdrawal_hash()))
            .returning(|_| Ok(RequestStatus::Ferried([1u8; 20])));
        l1mock
            .expect_get_status()
            .returning(|_| Ok(RequestStatus::Pending));

        l2mock
            .expect_get_best_block()
            .returning(|| Ok((1, H256::zero())));
        l2mock
            .expect_get_l2_request()
            .returning(|_, id, _| Ok(Some(dummy_withdrawal_request(id))));

        let _t = tokio::spawn(async move {
            WithdrawalSubscriber::new(CHAIN, l1mock, l2mock, sender, 10)
                .run()
                .await
                .unwrap();
        });

        assert_eq!(receiver.recv().await, Some(dummy_withdrawal(1u128)));
        assert_eq!(receiver.recv().await, Some(dummy_withdrawal(3u128)));
        assert_eq!(receiver.recv().await, Some(dummy_withdrawal(5u128)));
    }

    #[tokio::test]
    async fn test_fails_when_receives_batch_with_unexisting_request() {
        let mut l1mock = MockL1::new();
        let mut l2mock = MockL2::new();

        let batch = (H256::default(), (1u128, 5u128));
        let (sender, _receiver) = mpsc::channel(100);

        l1mock
            .expect_subscribe_new_batch()
            .returning(move || Ok(stream::iter(once(batch)).boxed()));
        l1mock
            .expect_get_status()
            .returning(|_| Ok(RequestStatus::Pending));

        l2mock
            .expect_get_best_block()
            .returning(|| Ok((1, H256::zero())));

        l2mock
            .expect_get_l2_request()
            .with(always(), eq(3u128), always())
            .returning(|_, _, _| Ok(None));

        l2mock
            .expect_get_l2_request()
            .returning(|_, id, _| Ok(Some(dummy_withdrawal_request(id))));

        assert!(matches!(
            WithdrawalSubscriber::new(CHAIN, l1mock, l2mock, sender, 10)
                .run()
                .await,
            Err(Error::L2RequestDoesNotExists(3u128))
        ));
    }
}

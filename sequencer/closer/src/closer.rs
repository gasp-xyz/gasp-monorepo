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
            if self.is_pending(w).await? {
                self.close_single_withdrawal(w, at).await?;
            }
        }
        Ok(())
    }

    async fn is_pending(&self, withdrawal: Withdrawal) -> Result<bool, Error> {
        if let RequestStatus::Pending = self.l1.get_status(withdrawal.withdrawal_hash()).await? {
            Ok(true)
        } else {
            tracing::info!("skipping withdrawal id:{}", withdrawal.request_id.id);
            Ok(false)
        }
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
            if self.input.is_closed() {
                break;
            }
        }
        Ok(())
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

#[cfg(test)]
mod test {
    use super::*;
    use common::TryReceiveAsync;
    use gasp_types::RequestId;
    use l1api::mock::MockL1;
    use l2api::mock::MockL2;
    use l2api::types::BatchInfo;
    use mockall::Sequence;
    use tokio::time::Duration;
    use tracing_test::traced_test;

    const CHAIN: gasp_types::Chain = gasp_types::Chain::Ethereum;
    const TIMEOUT: Duration = Duration::from_secs(1);

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

    #[tokio::test]
    async fn test_close_withdrawal() {
        let (signal, is_closed) = oneshot::channel();
        let mut l1mock = MockL1::new();
        let mut l2mock = MockL2::new();

        l1mock
            .expect_get_status()
            .returning(|_| Ok(RequestStatus::Pending));
        l1mock.expect_close_withdrawal().return_once(|_, _, _| {
            signal.send(()).unwrap();
            Ok(H256::default())
        });

        l2mock
            .expect_get_best_block()
            .returning(|| Ok((1, H256::zero())));
        l2mock.expect_bisect_find_batch().returning(|_, _, _| {
            Ok(Some(BatchInfo {
                batch_id: 1u128,
                range: (1u128, 1u128),
                merkle_root: H256::default(),
            }))
        });
        l2mock
            .expect_get_merkle_proof()
            .returning(|_, _, _, _| Ok(Default::default()));

        let (sender, receiver) = mpsc::channel(100);

        let _t = tokio::spawn(async move {
            let mut closer = Closer::new(CHAIN, l1mock, l2mock, receiver);
            closer.run().await
        });

        sender.send(dummy_withdrawal(1u128)).await.unwrap();

        is_closed.recv_timeout_async(TIMEOUT).await.unwrap();
    }

    #[tokio::test]
    async fn test_missing_batch() {
        let mut l1mock = MockL1::new();
        let mut l2mock = MockL2::new();

        l1mock
            .expect_get_status()
            .returning(|_| Ok(RequestStatus::Pending));

        l2mock
            .expect_get_best_block()
            .returning(|| Ok((1, H256::zero())));
        l2mock
            .expect_bisect_find_batch()
            .returning(|_, _, _| Ok(None));
        l2mock
            .expect_get_merkle_proof()
            .returning(|_, _, _, _| Ok(Default::default()));

        let (sender, receiver) = mpsc::channel(100);
        sender.send(dummy_withdrawal(1u128)).await.unwrap();

        let mut closer = Closer::new(CHAIN, l1mock, l2mock, receiver);
        assert!(matches!(
            closer.run().await,
            Err(Error::NoBatchForL2RequestId(1u128))
        ));
    }

    #[tokio::test]
    #[traced_test]
    async fn test_frontruned_close_withdrawal_can_recover() {
        let (signal, is_closed) = oneshot::channel();
        let mut l1mock = MockL1::new();
        let mut l2mock = MockL2::new();

        let mut seq = Sequence::new();
        l1mock
            .expect_get_status()
            // .with(eq(withdrawal.withdrawal_hash()))
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_| Ok(RequestStatus::Pending));

        l1mock
            .expect_get_status()
            // .with(eq(withdrawal.withdrawal_hash()))
            .times(1)
            .in_sequence(&mut seq)
            .return_once(|_| {
                signal.send(()).unwrap();
                Ok(RequestStatus::Closed)
            });
        l1mock
            .expect_close_withdrawal()
            .return_once(|_, _, _| Err(L1Error::TxReverted(H256::zero())));

        l2mock
            .expect_get_best_block()
            .returning(|| Ok((1, H256::zero())));
        l2mock.expect_bisect_find_batch().returning(|_, _, _| {
            Ok(Some(BatchInfo {
                batch_id: 1u128,
                range: (1u128, 1u128),
                merkle_root: H256::default(),
            }))
        });
        l2mock
            .expect_get_merkle_proof()
            .returning(|_, _, _, _| Ok(Default::default()));

        let (sender, receiver) = mpsc::channel(100);

        let t = tokio::spawn(async move {
            let mut closer = Closer::new(CHAIN, l1mock, l2mock, receiver);
            closer.run().await.unwrap()
        });

        sender.send(dummy_withdrawal(1u128)).await.unwrap();
        is_closed.recv_timeout_async(TIMEOUT).await.unwrap();

        std::mem::drop(sender);
        assert!(tokio::time::timeout(TIMEOUT, t).await.is_ok());
    }
}

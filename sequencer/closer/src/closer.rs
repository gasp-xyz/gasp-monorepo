#![allow(dead_code)]

use gasp_types::Withdrawal;
use l1api::types::RequestStatus;
use l1api::{L1Error, L1Interface};
use l2api::types::{BatchInfo, Chain, H256};
use l2api::{L2Error, L2Interface};
use tokio::sync::mpsc;

use crate::metrics;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("L1 error")]
    L1(#[from] L1Error),
    #[error("L2 error")]
    L2(#[from] L2Error),
    #[error("Could not find batch for request id `{0}`")]
    NoBatchForL2RequestId(u128),
    #[error("Withdrawal does not exists on l2 `{0}`")]
    DoesntExistsOnL2(u128),
}

pub type CloserResult<T> = Result<T, Error>;

pub struct Closer<L1, L2> {
    l1: L1,
    l2: L2,
    chain: Chain,
    input: mpsc::Receiver<Withdrawal>,
    close_withdrawals_in_batches: bool,
}

impl<L1, L2> Closer<L1, L2>
where
    L1: L1Interface,
    L2: L2Interface,
{
    pub fn new(
        chain: Chain,
        l1: L1,
        l2: L2,
        input: mpsc::Receiver<Withdrawal>,
        close_withdrawals_in_batches: bool,
    ) -> Self {
        Closer {
            l1,
            l2,
            chain,
            input,
            close_withdrawals_in_batches,
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
                metrics::CLOSED_WITHDRAWAL.inc();
                Ok(())
            }
            (Err(_), RequestStatus::Closed) => {
                tracing::info!(
                    "failed to close withdrawal {} - someone else closed it ",
                    request_id,
                );
                metrics::FAILED_CLOSE_ATTEMPTS.inc();
                Ok(())
            }
            (Err(err), _) => {
                tracing::error!("failed to close withdrawal {request_id} err: {err}",);
                Err(err.into())
            }
        }
    }

    async fn assert_withdrawals_exists(
        &self,
        withdrawals: &[Withdrawal],
        at: H256,
    ) -> Result<(), Error> {
        futures::future::try_join_all(withdrawals.iter().map(|elem| self.check_exists(*elem, at)))
            .await?;
        Ok(())
    }

    async fn close_withdrawals(&self, withdrawals: Vec<Withdrawal>) -> Result<(), Error> {
        let (_, at) = self.l2.get_best_block().await?;
        self.assert_withdrawals_exists(&withdrawals, at).await?;

        if self.close_withdrawals_in_batches {
            let withdrawals_with_proofs = futures::future::try_join_all(
                withdrawals
                    .iter()
                    .map(|elem| self.get_batch_proof_and_status(*elem, at)),
            )
            .await?;

            let closable_withdrawals = withdrawals_with_proofs
                .into_iter()
                .filter_map(|(w, status, batch, proof)| {
                    if status == RequestStatus::Pending {
                        Some((w, batch.merkle_root, proof))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            let closable_withdrawals_ids: Vec<u128> = closable_withdrawals
                .iter()
                .map(|(w, _, _)| w.request_id.id.try_into().unwrap())
                .collect();

            tracing::info!("closing withdrawals with ids : {closable_withdrawals_ids:?}");
            let amount = closable_withdrawals.len();
            let tx = self
                .l1
                .close_withdrawals_at_once(closable_withdrawals)
                .await?;
            metrics::CLOSED_WITHDRAWAL.add(amount as f64);

            tracing::info!("batch closed successfully {tx}");
        } else {
            for w in withdrawals {
                if self.is_pending(w).await? {
                    self.close_single_withdrawal(w, at).await?;
                }
            }
        }
        Ok(())
    }

    async fn is_pending(&self, withdrawal: Withdrawal) -> Result<bool, Error> {
        if let RequestStatus::Pending = self.l1.get_status(withdrawal.withdrawal_hash()).await? {
            Ok(true)
        } else {
            tracing::info!(
                "skipping withdrawal id:{} - its not pending anymore",
                withdrawal.request_id.id
            );
            Ok(false)
        }
    }

    async fn check_exists(&self, withdrawal: Withdrawal, at: H256) -> Result<(), Error> {
        let id = withdrawal.request_id.id.try_into().unwrap();
        let hash = self
            .l2
            .get_l2_request_hash(id, self.chain, at)
            .await?
            .ok_or(Error::DoesntExistsOnL2(id))?;
        let expected_hash = withdrawal.withdrawal_hash();
        if expected_hash == hash {
            Ok(())
        } else {
            tracing::error!("withdrawal_hash {hash} doesnt match expected one {expected_hash} - did reorg happend?");
            Err(Error::DoesntExistsOnL2(id))
        }
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        let mut buffer = Vec::<Withdrawal>::with_capacity(25);

        loop {
            buffer.clear();
            let queue_size = self.input.len();
            crate::metrics::PENDING_WITHDRAWALS.set(queue_size as f64);

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

    async fn get_batch_proof_and_status(
        &self,
        withdrawal: Withdrawal,
        at: H256,
    ) -> CloserResult<(Withdrawal, RequestStatus, BatchInfo, Vec<H256>)> {
        let (batch, range) = self
            .get_batch_and_proof(withdrawal.request_id.id.try_into().unwrap(), at)
            .await?;
        let status = self.l1.get_status(withdrawal.withdrawal_hash()).await?;
        Ok((withdrawal, status, batch, range))
    }

    #[tracing::instrument(level = "debug", skip(self, at), ret)]
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
    use mockall::predicate::eq;
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
    async fn test_error_on_non_existing_withdrawal() {
        let mut l1mock = MockL1::new();
        let mut l2mock = MockL2::new();

        l1mock
            .expect_get_status()
            .returning(|_| Ok(RequestStatus::Pending));
        l1mock
            .expect_close_withdrawal()
            .return_once(|_, _, _| Ok(H256::default()));

        l2mock
            .expect_get_best_block()
            .returning(|| Ok((1, H256::zero())));
        l2mock
            .expect_get_l2_request_hash()
            .returning(|_, _, _| Ok(None));
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
            let mut closer = Closer::new(CHAIN, l1mock, l2mock, receiver, false);
            closer.run().await
        });

        sender.send(dummy_withdrawal(1u128)).await.unwrap();
        assert!(matches!(
            t.await.unwrap(),
            Err(Error::DoesntExistsOnL2(1u128))
        ));
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
        l2mock
            .expect_get_l2_request_hash()
            .returning(|id, _, _| Ok(Some(dummy_withdrawal(id).withdrawal_hash())));
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
            let mut closer = Closer::new(CHAIN, l1mock, l2mock, receiver, false);
            closer.run().await
        });

        sender.send(dummy_withdrawal(1u128)).await.unwrap();

        is_closed.recv_timeout_async(TIMEOUT).await.unwrap();
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_close_multiple_withdrawals_at_once_ignoring_non_pending_withdrawals() {
        let (signal, is_closed) = oneshot::channel();
        let mut l1mock = MockL1::new();
        let mut l2mock = MockL2::new();

        let w1 = dummy_withdrawal(1u128);
        let w2 = dummy_withdrawal(2u128);
        let w3 = dummy_withdrawal(3u128);

        l1mock
            .expect_get_status()
            .with(eq(w1.withdrawal_hash()))
            .returning(|_| Ok(RequestStatus::Pending));
        l1mock
            .expect_get_status()
            .with(eq(w2.withdrawal_hash()))
            .returning(|_| Ok(RequestStatus::Closed));
        l1mock
            .expect_get_status()
            .with(eq(w3.withdrawal_hash()))
            .returning(|_| Ok(RequestStatus::Pending));

        let closable_withdrawals =
            vec![(w1, H256::default(), vec![]), (w3, H256::default(), vec![])];
        l1mock
            .expect_close_withdrawals_at_once()
            .with(eq(closable_withdrawals))
            .return_once(|_| {
                signal.send(()).unwrap();
                Ok(H256::default())
            });

        l2mock
            .expect_get_best_block()
            .returning(|| Ok((1, H256::zero())));
        l2mock
            .expect_get_l2_request_hash()
            .returning(|id, _, _| Ok(Some(dummy_withdrawal(id).withdrawal_hash())));

        l2mock.expect_bisect_find_batch().returning(move |_, _, _| {
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

        sender.send(w1).await.unwrap();
        sender.send(w2).await.unwrap();
        sender.send(w3).await.unwrap();

        let _t = tokio::spawn(async move {
            let mut closer = Closer::new(CHAIN, l1mock, l2mock, receiver, true);
            closer.run().await
        });

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
        l2mock
            .expect_get_l2_request_hash()
            .returning(|id, _, _| Ok(Some(dummy_withdrawal(id).withdrawal_hash())));

        let (sender, receiver) = mpsc::channel(100);
        sender.send(dummy_withdrawal(1u128)).await.unwrap();

        let mut closer = Closer::new(CHAIN, l1mock, l2mock, receiver, false);
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
        l2mock
            .expect_get_l2_request_hash()
            .returning(|id, _, _| Ok(Some(dummy_withdrawal(id).withdrawal_hash())));

        let (sender, receiver) = mpsc::channel(100);

        let t = tokio::spawn(async move {
            let mut closer = Closer::new(CHAIN, l1mock, l2mock, receiver, false);
            closer.run().await.unwrap()
        });

        sender.send(dummy_withdrawal(1u128)).await.unwrap();
        is_closed.recv_timeout_async(TIMEOUT).await.unwrap();

        std::mem::drop(sender);
        assert!(tokio::time::timeout(TIMEOUT, t).await.is_ok());
    }
}

use futures::{FutureExt, StreamExt};
use gasp_types::{Chain, Withdrawal, U256};
use l1api::{types::RequestStatus, L1Error, L1Interface};
use l2api::L2Interface;
use std::collections::{HashMap, VecDeque};
use tokio::sync::mpsc;
use tokio::time::timeout;

#[derive(Debug, PartialEq, Clone)]
pub enum FerryAction {
    Ferry { withdrawal: Withdrawal, prio: U256 },
    CloseFerriedWithdrawal { withdrawal: Withdrawal },
}

#[derive(Debug, thiserror::Error)]
pub enum FerryError {
    #[error("Could not find merkle root for request id {0}")]
    UnknownMerkleRoot(U256),

    #[error("L1 error: {0}")]
    L1Error(#[from] l1api::L1Error),

    #[error("L2 error: {0}")]
    L2Error(#[from] l2api::L2Error),
}

pub struct Ferry<L1, L2> {
    l1: L1,
    l2: L2,
    chain: Chain,
    account: [u8; 20],
    input: mpsc::Receiver<FerryAction>,
    closable_withdrawals: VecDeque<Withdrawal>,
    ferryable_withdrawals: HashMap<U256, (U256, Withdrawal)>,
    balances: HashMap<[u8; 20], U256>,
    tx_cost: U256,
}

impl<L1, L2> Ferry<L1, L2>
where
    L1: L1Interface,
    L2: L2Interface,
{
    pub fn new(
        l1: L1,
        l2: L2,
        account: [u8; 20],
        chain: Chain,
        tx_cost: u128,
        input: mpsc::Receiver<FerryAction>,
    ) -> Self {
        Self {
            l1,
            l2,
            chain,
            input,
            closable_withdrawals: Default::default(),
            ferryable_withdrawals: Default::default(),
            balances: Default::default(),
            account,
            tx_cost: tx_cost.into(),
        }
    }

    pub async fn ferry_withdrawal(&mut self) -> Result<(), FerryError> {
        let balances = self.balances.clone();
        let latest_finalized = self
            .l1
            .get_latest_finalized_request_id()
            .await?
            .unwrap_or_default();
        let len = self.ferryable_withdrawals.len();
        self.ferryable_withdrawals
            .retain(|request_id, _| request_id > &latest_finalized.into());

        tracing::debug!(
            "removed {n} withdrawals that are already finalized",
            n = len - self.ferryable_withdrawals.len()
        );

        let req_to_ferrry = self
            .ferryable_withdrawals
            .iter()
            .filter(|(_, (_, w))| {
                let required_tokens_amount = if w.token_address == l1api::NATIVE_TOKEN_ADDRESS {
                    w.amount - w.ferry_tip
                } else {
                    (w.amount - w.ferry_tip).saturating_add(self.tx_cost)
                };
                let balance = balances.get(&w.token_address).cloned().unwrap_or_default();
                balance >= required_tokens_amount
            })
            .max_by_key(|(_, (prio, _))| prio)
            .map(|(_, (_, w))| *w);

        if let Some(w) = req_to_ferrry {
            if let RequestStatus::Pending = self.l1.get_status(w.withdrawal_hash()).await? {
                tracing::trace!("tracing withdrawal {w}");
                let status = self.l1.ferry_withdrawal(w).await;
                match status {
                    Ok(hash) => {
                        tracing::info!("withdrawal ferried successfully {hash}");
                    }
                    Err(L1Error::TxReverted(hash)) => {
                        tracing::warn!("withdrawal ferried unsuccessfully {hash}");
                    }
                    Err(e) => {
                        return Err(e.into());
                    }
                }
                self.ferryable_withdrawals.remove(&w.request_id.id);
            }
        }

        Ok(())
    }

    pub async fn close_withdrawal(&self, withdrawal: Withdrawal) -> Result<(), FerryError> {
        if let RequestStatus::Ferried(_) = self.l1.get_status(withdrawal.withdrawal_hash()).await? {
            let req_id = withdrawal.request_id.id.try_into().unwrap();
            let (root, range) = self
                .l1
                .get_merkle_root(req_id)
                .await?
                .ok_or(FerryError::UnknownMerkleRoot(withdrawal.request_id.id))?;

            let mut stream = self.l2.header_stream(l2api::Finalization::Best).await?;
            let (_, at) = stream.next().await.expect("infinite stream")?;
            let proof = self
                .l2
                .get_merkle_proof(req_id, range, self.chain, at)
                .await?;
            let _result = self
                .l1
                .close_withdrawal(withdrawal, root.into(), proof)
                .await?;
            Ok(())
        } else {
            tracing::debug!("skipping already closed withdrawal {withdrawal}");
            Ok(())
        }
    }

    pub async fn close_all_withdrawals(&mut self) -> Result<(), FerryError> {
        while let Some(w) = self.closable_withdrawals.pop_back() {
            let result = self.close_withdrawal(w).await;
            match result {
                Ok(_) => {
                    tracing::debug!("withdrawal closed successfully {w}");
                }
                Err(e) => {
                    if let RequestStatus::Closed = self.l1.get_status(w.withdrawal_hash()).await? {
                        tracing::debug!("withdrawal closed by someone else");
                    } else {
                        tracing::warn!("failed to close withdrawal {w} {e}");
                        return Err(e);
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn get_balance(&self, token_address: [u8; 20]) -> Result<U256, FerryError> {
        let balance = if token_address == l1api::NATIVE_TOKEN_ADDRESS {
            self.l1.native_balance(self.account).await?
        } else {
            self.l1.erc20_balance(token_address, self.account).await?
        };
        Ok(balance.into())
    }

    #[tracing::instrument(skip_all)]
    pub async fn refresh_balances(&mut self) -> Result<(), FerryError> {
        let tokens = self
            .balances
            .keys()
            .map(|token| {
                self.get_balance(*token)
                    .map(|result| result.map(|balance| (*token, balance)))
            })
            .collect::<Vec<_>>();

        let balances = futures::future::join_all(tokens)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        for (token, balance) in balances {
            self.balances.insert(token, balance);
        }

        Ok(())
    }

    pub async fn track_balance(&mut self, token_address: [u8; 20]) -> Result<(), FerryError> {
        if self.balances.get(&token_address).is_none() {
            let balance = self.get_balance(token_address).await?;
            self.balances.insert(token_address, balance);
        }
        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), FerryError> {
        loop {
            loop {
                // fetch as many elemenets from queue at once as possible
                match timeout(
                    tokio::time::Duration::from_secs_f64(0.25),
                    self.input.recv(),
                )
                .await
                {
                    Ok(Some(FerryAction::Ferry { withdrawal, prio })) => {
                        tracing::info!("received fery request");
                        self.track_balance(withdrawal.token_address).await?;
                        self.ferryable_withdrawals
                            .insert(withdrawal.request_id.id, (prio, withdrawal));
                    }
                    Ok(Some(FerryAction::CloseFerriedWithdrawal { withdrawal })) => {
                        self.closable_withdrawals.push_front(withdrawal);
                    }
                    Err(_elapsed) => {
                        tracing::debug!("no new messages found yet");
                        break;
                    }
                    Ok(None) => {
                        tracing::info!("input channel closed, bye!");
                        return Ok(());
                    }
                }
            }

            if !self.closable_withdrawals.is_empty() {
                tracing::info!(
                    "found {n} closable withdrawals",
                    n = self.closable_withdrawals.len()
                );
                self.close_all_withdrawals().await?;
                self.refresh_balances().await?;
                continue;
            }

            if !self.ferryable_withdrawals.is_empty() {
                tracing::info!(
                    "found {n} withdrawals ready to ferry",
                    n = self.ferryable_withdrawals.len()
                );
                self.ferry_withdrawal().await?;
                self.refresh_balances().await?;
                continue;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use futures::stream;
    use gasp_types::{Origin, RequestId, H256};
    use l1api::mock::MockL1;
    use l2api::mock::MockL2;
    use mockall::{
        predicate::{always, eq},
        Sequence,
    };
    use tracing_test::traced_test;

    use std::{
        sync::{atomic::AtomicBool, Arc},
        time::Duration,
    };

    const ACCOUNT: [u8; 20] = [1; 20];
    const ENABLED_TOKEN1: [u8; 20] = [2; 20];
    const NATIVE_TOKEN: [u8; 20] = hex_literal::hex!("0000000000000000000000000000000000000001");
    const RECIPIENT: [u8; 20] = [5; 20];

    #[traced_test]
    #[tokio::test]
    async fn works_fine_when_there_is_nothing_to_process() {
        let l1 = MockL1::new();
        let l2 = MockL2::new();

        let (input, output) = mpsc::channel(100);
        let mut ferry = Ferry::new(l1, l2, ACCOUNT, Chain::Ethereum, 0, output);

        let handle = tokio::spawn(async move {
            ferry.run().await.unwrap();
        });

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        drop(input);
        handle.await.unwrap();
    }

    #[traced_test]
    #[tokio::test]
    async fn test_ferry_ignores_requests_that_can_not_affort() {
        let mut l1 = MockL1::new();
        let l2 = MockL2::new();

        let (input, output) = mpsc::channel(100);
        l1.expect_erc20_balance().returning(|_, _| Ok(10u128));
        l1.expect_get_latest_finalized_request_id()
            .returning(|| Ok(None));
        l1.expect_ferry_withdrawal().times(0);

        input
            .send(FerryAction::Ferry {
                withdrawal: Withdrawal {
                    request_id: RequestId {
                        id: 1.into(),
                        origin: Origin::L2,
                    },
                    token_address: ENABLED_TOKEN1,
                    amount: 100.into(),
                    ferry_tip: 10.into(),
                    recipient: RECIPIENT,
                },
                prio: 10.into(),
            })
            .await
            .unwrap();

        let mut ferry = Ferry::new(l1, l2, ACCOUNT, Chain::Ethereum, 0, output);

        let handle = tokio::spawn(async move {
            ferry.run().await.unwrap();
        });

        drop(input);
        handle.await.unwrap();
    }

    #[traced_test]
    #[tokio::test]
    async fn test_ferry_ignores_native_requests_that_can_not_affort() {
        let mut l1 = MockL1::new();
        let l2 = MockL2::new();

        let signal = Arc::new(AtomicBool::new(false));
        let notify = signal.clone();
        let (input, output) = mpsc::channel(100);
        l1.expect_native_balance().returning(|_| Ok(500));
        l1.expect_get_latest_finalized_request_id()
            .returning(|| Ok(None));

        l1.expect_get_status()
            .returning(|_| Ok(RequestStatus::Pending));

        l1.expect_ferry_withdrawal().times(1).return_once(move |_| {
            notify.store(true, std::sync::atomic::Ordering::Relaxed);
            Ok(H256::default())
        });

        input
            .send(FerryAction::Ferry {
                withdrawal: Withdrawal {
                    request_id: RequestId {
                        id: 1.into(),
                        origin: Origin::L2,
                    },
                    token_address: NATIVE_TOKEN,
                    amount: 100.into(),
                    ferry_tip: 10.into(),
                    recipient: RECIPIENT,
                },
                prio: 10.into(),
            })
            .await
            .unwrap();

        let mut ferry = Ferry::new(l1, l2, ACCOUNT, Chain::Ethereum, 0, output);

        let handle = tokio::spawn(async move {
            ferry.run().await.unwrap();
        });

        drop(input);
        handle.await.unwrap();
    }

    #[traced_test]
    #[tokio::test]
    async fn test_ferry_acutally_ferries_withdrawal() {
        let mut l1 = MockL1::new();
        let l2 = MockL2::new();

        let (input, output) = mpsc::channel(100);

        let withdrawal = Withdrawal {
            request_id: RequestId {
                id: 1.into(),
                origin: Origin::L2,
            },
            token_address: ENABLED_TOKEN1,
            amount: 100.into(),
            ferry_tip: 10.into(),
            recipient: RECIPIENT,
        };

        l1.expect_erc20_balance().returning(|_, _| Ok(100u128));
        l1.expect_get_latest_finalized_request_id()
            .returning(|| Ok(None));
        l1.expect_get_status()
            .returning(|_| Ok(RequestStatus::Pending));
        l1.expect_ferry_withdrawal()
            .times(1)
            .return_once(|_| Ok(H256::default()));

        input
            .send(FerryAction::Ferry {
                withdrawal,
                prio: 10.into(),
            })
            .await
            .unwrap();

        let mut ferry = Ferry::new(l1, l2, ACCOUNT, Chain::Ethereum, 0, output);

        let handle = tokio::spawn(async move {
            ferry.run().await.unwrap();
        });

        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        drop(input);
        handle.await.unwrap();
    }

    pub async fn wait_for(mtx: Arc<AtomicBool>, duration: Duration) -> bool {
        let now = tokio::time::Instant::now();
        while !mtx.load(std::sync::atomic::Ordering::Relaxed) {
            if now.elapsed() > duration {
                return false;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        true
    }

    #[traced_test]
    #[tokio::test]
    async fn test_ferry_withdrawals_by_priority() {
        let mut l1 = MockL1::new();
        let l2 = MockL2::new();
        let signal = Arc::new(AtomicBool::new(false));
        let (input, output) = mpsc::channel(100);

        let low_prio_withdrawal = Withdrawal {
            request_id: RequestId {
                id: 2.into(),
                origin: Origin::L2,
            },
            token_address: ENABLED_TOKEN1,
            amount: 100.into(),
            ferry_tip: 10.into(),
            recipient: RECIPIENT,
        };

        let high_prio_withdrawal = Withdrawal {
            request_id: RequestId {
                id: 3.into(),
                origin: Origin::L2,
            },
            token_address: ENABLED_TOKEN1,
            amount: 100.into(),
            ferry_tip: 50.into(),
            recipient: RECIPIENT,
        };

        tracing::info!("hello world");
        l1.expect_erc20_balance().returning(|_, _| Ok(100u128));
        l1.expect_get_latest_finalized_request_id()
            .returning(|| Ok(None));
        l1.expect_get_status()
            .returning(|_| Ok(RequestStatus::Pending));

        let mut seq = Sequence::new();
        l1.expect_ferry_withdrawal()
            .times(1)
            .in_sequence(&mut seq)
            .with(eq(high_prio_withdrawal))
            .returning(|_| Ok(H256::default()));

        let notify = signal.clone();
        l1.expect_ferry_withdrawal()
            .times(1)
            .in_sequence(&mut seq)
            .with(eq(low_prio_withdrawal))
            .return_once(move |_| {
                notify.store(true, std::sync::atomic::Ordering::Relaxed);
                Ok(H256::default())
            });

        input
            .send(FerryAction::Ferry {
                withdrawal: low_prio_withdrawal,
                prio: 10.into(),
            })
            .await
            .unwrap();

        input
            .send(FerryAction::Ferry {
                withdrawal: high_prio_withdrawal,
                prio: 50.into(),
            })
            .await
            .unwrap();

        let mut ferry = Ferry::new(l1, l2, ACCOUNT, Chain::Ethereum, 0, output);
        let handle = tokio::spawn(async move {
            ferry.run().await.unwrap();
        });
        assert!(wait_for(signal, Duration::from_secs(5)).await);
        drop(input);
        handle.await.unwrap();
    }

    #[traced_test]
    #[tokio::test]
    async fn prioriorize_closing_over_ferrying() {
        let mut l1 = MockL1::new();
        let mut l2 = MockL2::new();
        let signal = Arc::new(AtomicBool::new(false));
        let (input, output) = mpsc::channel(100);

        let closable_withdrawal = Withdrawal {
            request_id: RequestId {
                id: 1.into(),
                origin: Origin::L2,
            },
            token_address: ENABLED_TOKEN1,
            amount: 100.into(),
            ferry_tip: 10.into(),
            recipient: RECIPIENT,
        };
        let ferryable_withdrawal = Withdrawal {
            request_id: RequestId {
                id: 2.into(),
                origin: Origin::L2,
            },
            token_address: ENABLED_TOKEN1,
            amount: 100.into(),
            ferry_tip: 10.into(),
            recipient: RECIPIENT,
        };

        input
            .send(FerryAction::Ferry {
                withdrawal: ferryable_withdrawal,
                prio: 50.into(),
            })
            .await
            .unwrap();

        input
            .send(FerryAction::CloseFerriedWithdrawal {
                withdrawal: closable_withdrawal,
            })
            .await
            .unwrap();

        l1.expect_erc20_balance().returning(|_, _| Ok(100u128));
        l1.expect_get_latest_finalized_request_id()
            .returning(|| Ok(Some(1)));

        l1.expect_get_status()
            .with(eq(ferryable_withdrawal.withdrawal_hash()))
            .returning(|_| Ok(RequestStatus::Pending));
        l1.expect_get_status()
            .with(eq(closable_withdrawal.withdrawal_hash()))
            .returning(|_| Ok(RequestStatus::Ferried(ACCOUNT)));
        l2.expect_header_stream()
            .return_once(|_| Ok(Box::pin(stream::iter(vec![Ok((1u32, [0u8; 32].into()))]))));

        l1.expect_get_merkle_root()
            .returning(|_| Ok(Some(([0u8; 32], (1, 1)))));
        l2.expect_get_merkle_proof()
            .returning(|_, _, _, _| Ok(Vec::default()));

        let notify = signal.clone();
        let mut seq = Sequence::new();
        l1.expect_close_withdrawal()
            .times(1)
            .in_sequence(&mut seq)
            .with(eq(closable_withdrawal), always(), always())
            .returning(|_, _, _| Ok(H256::default()));
        l1.expect_ferry_withdrawal()
            .times(1)
            .in_sequence(&mut seq)
            .with(eq(ferryable_withdrawal))
            .return_once(move |_| {
                notify.store(true, std::sync::atomic::Ordering::Relaxed);
                Ok(H256::default())
            });

        let mut ferry = Ferry::new(l1, l2, ACCOUNT, Chain::Ethereum, 0, output);
        let handle = tokio::spawn(async move {
            ferry.run().await.unwrap();
        });
        assert!(wait_for(signal, Duration::from_secs(5)).await);
        drop(input);
        handle.await.unwrap();
    }
}

use futures::FutureExt;
use gasp_types::{Chain, Deposit, U256};
use l1api::L1Interface;
use l2api::L2Interface;
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::{timeout, Instant};

use crate::metrics;

pub type Priority = U256;
pub type DepositId = U256;

#[derive(Debug, thiserror::Error)]
pub enum FerryError {
    #[error("request does not exists on L1 {0}")]
    DoesntExistsOnL1(u128),

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
    input: mpsc::Receiver<(Priority, Deposit)>,
    ferryable_deposits: HashMap<DepositId, (Priority, Deposit)>,
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
        input: mpsc::Receiver<(Priority, Deposit)>,
    ) -> Self {
        Self {
            l1,
            l2,
            chain,
            input,
            ferryable_deposits: Default::default(),
            balances: Default::default(),
            account,
            tx_cost: tx_cost.into(),
        }
    }

    async fn assert_exists(&self, deposit: Deposit) -> Result<(), FerryError> {
        let request_id = deposit.request_id.id.try_into().unwrap();
        let d = self
            .l1
            .get_deposit(request_id)
            .await?
            .ok_or(FerryError::DoesntExistsOnL1(request_id))?;
        if d == deposit {
            Ok(())
        } else {
            Err(FerryError::DoesntExistsOnL1(request_id))
        }
    }

    pub async fn ferry_deposit(&mut self) -> Result<(), FerryError> {
        let balances = self.balances.clone();
        let (nr, at) = self.l2.get_best_block().await?;
        let latest_finalized = self
            .l2
            .get_latest_processed_request_id(self.chain, at)
            .await?;

        let ferryable_deposits_amount_before = self.ferryable_deposits.len();
        self.ferryable_deposits
            .retain(|request_id, _| *request_id > latest_finalized.into());
        if ferryable_deposits_amount_before > self.ferryable_deposits.len() {
            tracing::info!(
                "removed {n} that are already finalized",
                n = ferryable_deposits_amount_before - self.ferryable_deposits.len()
            );
        }

        let affordable_deposits = self
            .ferryable_deposits
            .iter()
            .filter(|(_, (_, deposit))| {
                let required_tokens_amount = if deposit.token_address == l1api::NATIVE_TOKEN_ADDRESS
                {
                    (deposit.amount - deposit.ferry_tip).saturating_add(self.tx_cost)
                } else {
                    deposit.amount - deposit.ferry_tip
                };
                let balance = balances
                    .get(&deposit.token_address)
                    .cloned()
                    .unwrap_or_default();
                tracing::trace!("required balance {balance} available balance {required_tokens_amount} token {}", hex::encode(deposit.token_address));
                balance >= required_tokens_amount
            }).collect::<Vec<_>>();

        tracing::debug!(
            "{affordable} / {all} affordable ferries found",
            affordable = affordable_deposits.len(),
            all = self.ferryable_deposits.len()
        );

        let req_to_ferrry = affordable_deposits
            .into_iter()
            .max_by_key(|(_, (prio, _))| prio)
            .map(|(_, (_, w))| *w);

        if let Some(deposit) = req_to_ferrry {
            self.assert_exists(deposit).await?;
            let ferried_amount: u128 = (deposit.amount - deposit.ferry_tip).try_into().unwrap();

            tracing::info!("#{nr} ferring deposit rid : {}", deposit.request_id.id);
            if self.l2.ferry_deposit(self.chain, deposit).await? {
                tracing::info!("deposit ferried successfully {}", deposit.request_id.id);
                metrics::FERRIED
                    .with_label_values(&[&hex::encode(deposit.token_address)])
                    .inc();
                metrics::FERRIED_VOLUME
                    .with_label_values(&[&hex::encode(deposit.token_address)])
                    .inc_by(ferried_amount as f64);
                metrics::FERRIED.with_label_values(&["total"]).inc();
                metrics::FERRIED_VOLUME
                    .with_label_values(&["total"])
                    .inc_by(ferried_amount as f64);
            } else {
                tracing::warn!("failed to ferry deposit {}", deposit.request_id.id);
                metrics::FAILED_FERRY_ATTEMPTS.inc();
            }
            self.ferryable_deposits.remove(&deposit.request_id.id);
        }

        Ok(())
    }

    #[tracing::instrument(skip_all)]
    pub async fn refresh_balances(&mut self) -> Result<(), FerryError> {
        let (_, at) = self.l2.get_best_block().await?;
        let chain = self.chain;
        let me = self.account;
        let keys = self.balances.keys().cloned().collect::<Vec<_>>();
        let tokens = keys
            .into_iter()
            .map(|token| {
                self.l2
                    .get_balance(chain, token, me, at)
                    .map(move |result| result.map(move |balance| (token, balance)))
            })
            .collect::<Vec<_>>();

        let balances = futures::future::join_all(tokens)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        for (token, balance) in balances {
            metrics::TOKEN_BALANCE
                .with_label_values(&[&hex::encode(token)])
                .set(balance as f64);
            self.balances.insert(token, balance.into());
        }

        Ok(())
    }

    pub async fn track_balance(&mut self, token_address: [u8; 20]) -> Result<(), FerryError> {
        if let std::collections::hash_map::Entry::Vacant(e) = self.balances.entry(token_address) {
            let (_, at) = self.l2.get_best_block().await?;
            let balance = self
                .l2
                .get_balance(self.chain, token_address, self.account, at)
                .await?;
            e.insert(balance.into());
        }
        Ok(())
    }

    pub async fn run(mut self) -> Result<(), FerryError> {
        let mut now = Instant::now();
        loop {
            loop {
                // fetch as many elemenets from queue at once as possible
                match timeout(
                    tokio::time::Duration::from_secs_f64(0.25),
                    self.input.recv(),
                )
                .await
                {
                    Ok(Some((prio, deposit))) => {
                        tracing::info!("pending deposit received rid : {}", deposit.request_id.id);
                        self.track_balance(deposit.token_address).await?;
                        self.ferryable_deposits
                            .insert(deposit.request_id.id, (prio, deposit));
                    }
                    Err(_elapsed) => {
                        break;
                    }
                    Ok(None) => {
                        tracing::debug!("stream dropped");
                        return Ok(());
                    }
                }
            }

            if Instant::now() - now > Duration::from_secs(30) {
                now = Instant::now();
                tracing::info!("keep alive");
            }

            if !self.ferryable_deposits.is_empty() {
                self.refresh_balances().await?;
                self.ferry_deposit().await?;
                continue;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use common::TryReceiveAsync;

    use gasp_types::{Origin, RequestId, H256};
    use l1api::mock::MockL1;
    use l2api::mock::MockL2;
    use mockall::{
        predicate::{always, eq},
        Sequence,
    };
    use tracing_test::traced_test;

    const ACCOUNT: [u8; 20] = [1; 20];
    const ENABLED_TOKEN1: [u8; 20] = [2; 20];
    const ENABLED_TOKEN2: [u8; 20] = [3; 20];
    const NATIVE_TOKEN: [u8; 20] = hex_literal::hex!("0000000000000000000000000000000000000001");
    const RECIPIENT: [u8; 20] = [5; 20];

    #[traced_test]
    #[tokio::test]
    async fn works_fine_when_there_is_nothing_to_process() {
        let l1 = MockL1::new();
        let l2 = MockL2::new();

        let (input, output) = mpsc::channel(100);
        let ferry = Ferry::new(l1, l2, ACCOUNT, Chain::Ethereum, 0, output);

        let handle = tokio::spawn(async move {
            ferry.run().await.unwrap();
        });

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        drop(input);
        handle.await.unwrap();
    }

    #[traced_test]
    #[tokio::test]
    async fn test_error_on_inexisting_deposit() {
        let mut l1 = MockL1::new();
        let mut l2 = MockL2::new();
        let (input, output) = mpsc::channel(100);

        let deposit = Deposit {
            request_id: RequestId {
                id: 2.into(),
                origin: Origin::L2,
            },
            token_address: ENABLED_TOKEN1,
            amount: 100.into(),
            ferry_tip: 10.into(),
            recipient: RECIPIENT,
            timestamp: 0.into(),
        };

        l2.expect_get_balance().returning(|_, _, _, _| Ok(90u128));
        l2.expect_get_latest_processed_request_id()
            .returning(|_, _| Ok(0u128));
        l2.expect_get_best_block()
            .returning(|| Ok((1, H256::zero())));

        l1.expect_get_deposit()
            .with(eq(2u128))
            .returning(move |_| Ok(None));

        input.send((10.into(), deposit)).await.unwrap();

        let handle = tokio::spawn(async move {
            let ferry = Ferry::new(l1, l2, ACCOUNT, Chain::Ethereum, 0, output);
            ferry.run().await
        });
        assert!(matches!(
            handle.await.unwrap(),
            Err(FerryError::DoesntExistsOnL1(2u128))
        ));
    }

    #[traced_test]
    #[tokio::test]
    async fn test_ferry_withdrawal_that_can_afford() {
        let mut l1 = MockL1::new();
        let mut l2 = MockL2::new();
        let (input, output) = mpsc::channel(100);
        let (signal, is_ferried) = oneshot::channel();

        let deposit = Deposit {
            request_id: RequestId {
                id: 2.into(),
                origin: Origin::L2,
            },
            token_address: ENABLED_TOKEN1,
            amount: 100.into(),
            ferry_tip: 10.into(),
            recipient: RECIPIENT,
            timestamp: 0.into(),
        };

        l2.expect_get_balance().returning(|_, _, _, _| Ok(90u128));
        l2.expect_get_latest_processed_request_id()
            .returning(|_, _| Ok(0u128));
        l2.expect_get_best_block()
            .returning(|| Ok((1, H256::zero())));

        l2.expect_ferry_deposit().times(1).return_once(move |_, _| {
            signal.send(()).unwrap();
            Ok(true)
        });
        l1.expect_get_deposit()
            .with(eq(2u128))
            .returning(move |_| Ok(Some(deposit)));

        input.send((10.into(), deposit)).await.unwrap();

        let handle = tokio::spawn(async move {
            let ferry = Ferry::new(l1, l2, ACCOUNT, Chain::Ethereum, 0, output);
            ferry.run().await.unwrap();
        });
        is_ferried
            .recv_timeout_async(tokio::time::Duration::from_millis(1000))
            .await
            .unwrap();
        drop(input);
        handle.await.unwrap();
    }

    #[traced_test]
    #[tokio::test]
    async fn test_ferry_withdrawals_by_priority() {
        let mut l1 = MockL1::new();
        let mut l2 = MockL2::new();
        let (input, output) = mpsc::channel(100);
        let (signal, is_ferried) = oneshot::channel();

        let low_prio_deposit = Deposit {
            request_id: RequestId {
                id: 2.into(),
                origin: Origin::L2,
            },
            token_address: ENABLED_TOKEN1,
            amount: 100.into(),
            ferry_tip: 10.into(),
            recipient: RECIPIENT,
            timestamp: 0.into(),
        };

        let high_prio_deposit = Deposit {
            request_id: RequestId {
                id: 3.into(),
                origin: Origin::L2,
            },
            token_address: ENABLED_TOKEN1,
            amount: 100.into(),
            ferry_tip: 50.into(),
            recipient: RECIPIENT,
            timestamp: 0.into(),
        };

        l1.expect_get_deposit()
            .with(eq(2u128))
            .returning(move |_| Ok(Some(low_prio_deposit)));
        l1.expect_get_deposit()
            .with(eq(3u128))
            .returning(move |_| Ok(Some(high_prio_deposit)));

        l2.expect_get_balance().returning(|_, _, _, _| Ok(100u128));
        l2.expect_get_latest_processed_request_id()
            .returning(|_, _| Ok(0u128));
        l2.expect_get_best_block()
            .returning(|| Ok((1, H256::zero())));

        let mut seq = Sequence::new();
        l2.expect_ferry_deposit()
            .times(1)
            .in_sequence(&mut seq)
            .with(always(), eq(high_prio_deposit))
            .returning(|_, _| Ok(true));

        l2.expect_ferry_deposit()
            .times(1)
            .in_sequence(&mut seq)
            .with(always(), eq(low_prio_deposit))
            .return_once(move |_, _| {
                signal.send(()).unwrap();
                Ok(true)
            });

        input.send((10.into(), low_prio_deposit)).await.unwrap();
        input.send((100.into(), high_prio_deposit)).await.unwrap();

        let handle = tokio::spawn(async move {
            let ferry = Ferry::new(l1, l2, ACCOUNT, Chain::Ethereum, 0, output);
            ferry.run().await.unwrap();
        });
        is_ferried
            .recv_timeout_async(tokio::time::Duration::from_millis(1000))
            .await
            .unwrap();
        drop(input);
        handle.await.unwrap();
    }

    #[traced_test]
    #[tokio::test]
    async fn test_ferry_ignores_requests_that_can_not_affort() {
        let mut l1 = MockL1::new();
        let mut l2 = MockL2::new();
        let (input, output) = mpsc::channel(100);
        let (signal, is_ferried) = oneshot::channel();

        let affordable_deposit = Deposit {
            request_id: RequestId {
                id: 2.into(),
                origin: Origin::L2,
            },
            token_address: ENABLED_TOKEN1,
            amount: 100.into(),
            ferry_tip: 10.into(),
            recipient: RECIPIENT,
            timestamp: 0.into(),
        };
        l2.expect_get_balance()
            .with(always(), eq(ENABLED_TOKEN1), always(), always())
            .returning(|_, _, _, _| Ok(90u128));

        let non_affordable_deposit = Deposit {
            request_id: RequestId {
                id: 3.into(),
                origin: Origin::L2,
            },
            token_address: ENABLED_TOKEN2,
            amount: 100.into(),
            ferry_tip: 10.into(),
            recipient: RECIPIENT,
            timestamp: 0.into(),
        };

        l1.expect_get_deposit()
            .with(eq(2u128))
            .returning(move |_| Ok(Some(affordable_deposit)));
        l2.expect_get_balance()
            .with(always(), eq(ENABLED_TOKEN2), always(), always())
            .returning(|_, _, _, _| Ok(89u128));

        l2.expect_get_latest_processed_request_id()
            .returning(|_, _| Ok(0u128));
        l2.expect_get_best_block()
            .returning(|| Ok((1, H256::zero())));

        l2.expect_ferry_deposit()
            .times(1)
            .with(always(), eq(affordable_deposit))
            .return_once(move |_, _| {
                signal.send(()).unwrap();
                Ok(true)
            });

        input
            .send((100.into(), non_affordable_deposit))
            .await
            .unwrap();
        input.send((10.into(), affordable_deposit)).await.unwrap();

        let handle = tokio::spawn(async move {
            let ferry = Ferry::new(l1, l2, ACCOUNT, Chain::Ethereum, 0, output);
            ferry.run().await.unwrap();
        });
        is_ferried
            .recv_timeout_async(tokio::time::Duration::from_millis(1000))
            .await
            .unwrap();
        drop(input);
        handle.await.unwrap();
    }

    #[traced_test]
    #[tokio::test]
    async fn test_ferry_ignores_native_requests_that_can_not_affort_because_of_tx_cost() {
        let mut l1 = MockL1::new();
        let mut l2 = MockL2::new();
        let (input, output) = mpsc::channel(100);
        let (signal, is_ferried) = oneshot::channel();

        let affordable_deposit = Deposit {
            request_id: RequestId {
                id: 2.into(),
                origin: Origin::L2,
            },
            token_address: ENABLED_TOKEN1,
            amount: 100.into(),
            ferry_tip: 10.into(),
            recipient: RECIPIENT,
            timestamp: 0.into(),
        };

        l1.expect_get_deposit()
            .with(eq(2u128))
            .returning(move |_| Ok(Some(affordable_deposit)));
        l2.expect_get_balance()
            .with(always(), eq(ENABLED_TOKEN1), always(), always())
            .returning(|_, _, _, _| Ok(90u128));

        let tx_cost = 1u128;
        let non_affordable_deposit = Deposit {
            request_id: RequestId {
                id: 3.into(),
                origin: Origin::L2,
            },
            token_address: NATIVE_TOKEN,
            amount: 100.into(),
            ferry_tip: 10.into(),
            recipient: RECIPIENT,
            timestamp: 0.into(),
        };
        l2.expect_get_balance()
            .with(always(), eq(NATIVE_TOKEN), always(), always())
            .returning(|_, _, _, _| Ok(90u128));

        l2.expect_get_latest_processed_request_id()
            .returning(|_, _| Ok(0u128));
        l2.expect_get_best_block()
            .returning(|| Ok((1, H256::zero())));

        l2.expect_ferry_deposit()
            .times(1)
            .with(always(), eq(affordable_deposit))
            .return_once(move |_, _| {
                signal.send(()).unwrap();
                Ok(true)
            });

        input
            .send((100.into(), non_affordable_deposit))
            .await
            .unwrap();
        input.send((10.into(), affordable_deposit)).await.unwrap();

        let handle = tokio::spawn(async move {
            let ferry = Ferry::new(l1, l2, ACCOUNT, Chain::Ethereum, tx_cost, output);
            ferry.run().await.unwrap();
        });
        is_ferried
            .recv_timeout_async(tokio::time::Duration::from_millis(1000))
            .await
            .unwrap();
        drop(input);
        handle.await.unwrap();
    }
}

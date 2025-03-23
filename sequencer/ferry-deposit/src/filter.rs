use gasp_types::{Deposit, U256};
use std::collections::HashMap;
use tokio::sync::mpsc;

pub async fn filter_deposits(
    mut input: mpsc::Receiver<Deposit>,
    output: mpsc::Sender<(U256, Deposit)>,
    enabled: HashMap<[u8; 20], u128>,
) {
    while let Some(deposit) = input.recv().await {
        if let Some(min_profit) = enabled.get(&deposit.token_address) {
            if deposit.ferry_tip > (*min_profit).into() {
                output
                    .send((deposit.ferry_tip, deposit))
                    .await
                    .expect("infinite");
            }
        } 
    }
    tracing::info!("closing filter service");
}

#[cfg(test)]
mod test {
    use super::*;
    use gasp_types::{Origin, RequestId};
    use l1api::mock::MockL1;
    use l2api::mock::MockL2;
    use tokio::sync::mpsc;

    const ENABLED_TOKEN1: [u8; 20] = [1; 20];
    const ENABLED_TOKEN2: [u8; 20] = [2; 20];
    const DISABLED_TOKEN: [u8; 20] = [3; 20];

    #[tokio::test]
    async fn test_picks_only_enabled_tokens() {
        let l1mock = MockL1::new();
        let l2mock = MockL2::new();

        let (filter_input, input) = mpsc::channel(10);
        let (output, mut filter_output) = mpsc::channel(10);

        let task = tokio::spawn(async move {
            filter_deposits(
                input,
                output,
                vec![(ENABLED_TOKEN1, 1u128), (ENABLED_TOKEN2, 1u128)]
                    .into_iter()
                    .collect(),
            )
            .await;
        });

        let enabled_deposit1 = Deposit {
            request_id: RequestId {
                id: 123u128.into(),
                origin: Origin::L2,
            },
            recipient: [0u8; 20],
            token_address: ENABLED_TOKEN1,
            amount: 100u128.into(),
            ferry_tip: 11u128.into(),
        };

        let enabled_deposit2 = Deposit {
            request_id: RequestId {
                id: 123u128.into(),
                origin: Origin::L2,
            },
            recipient: [0u8; 20],
            token_address: ENABLED_TOKEN2,
            amount: 100u128.into(),
            ferry_tip: 10u128.into(),
        };

        let disabled_deposit = Deposit {
            request_id: RequestId {
                id: 123u128.into(),
                origin: Origin::L2,
            },
            recipient: [0u8; 20],
            token_address: DISABLED_TOKEN,
            amount: 100u128.into(),
            ferry_tip: 10u128.into(),
        };

        filter_input.send(disabled_deposit).await.unwrap();
        filter_input.send(enabled_deposit1).await.unwrap();
        filter_input.send(enabled_deposit2).await.unwrap();
        filter_input.send(disabled_deposit).await.unwrap();

        drop(filter_input);
        task.await.unwrap();

        assert_eq!(
            (11u128.into(), enabled_deposit1),
            filter_output.recv().await.unwrap(),
        );

        assert_eq!(
            (10u128.into(), enabled_deposit2),
            filter_output.recv().await.unwrap(),
        );

        assert!(filter_output.recv().await.is_none(),);
    }
}

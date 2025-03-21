use crate::ferry::FerryAction;
use gasp_types::Withdrawal;
use l1api::L1Interface;
use l2api::L2Interface;
use std::collections::{HashMap, HashSet};
use tokio::sync::mpsc;

pub async fn filter_deposits(
    mut input: mpsc::Receiver<Withdrawal>,
    output: mpsc::Sender<FerryAction>,
    enabled: HashMap<[u8; 20], u128>,
) {
    while let Some(w) = input.recv().await {
        if let Some(min_profit) = enabled.get(&w.token_address) {
            if w.ferry_tip > (*min_profit).into() {
                output
                    .send(FerryAction::Ferry {
                        withdrawal: w,
                        prio: w.ferry_tip,
                    })
                    .await
                    .expect("infinite");
            }
        } else {
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

        let enabled_withdrawal1 = Withdrawal {
            request_id: RequestId {
                id: 123u128.into(),
                origin: Origin::L2,
            },
            recipient: [0u8; 20],
            token_address: ENABLED_TOKEN1,
            amount: 100u128.into(),
            ferry_tip: 11u128.into(),
        };

        let enabled_withdrawal2 = Withdrawal {
            request_id: RequestId {
                id: 123u128.into(),
                origin: Origin::L2,
            },
            recipient: [0u8; 20],
            token_address: ENABLED_TOKEN2,
            amount: 100u128.into(),
            ferry_tip: 10u128.into(),
        };

        let disabled_withdrawal = Withdrawal {
            request_id: RequestId {
                id: 123u128.into(),
                origin: Origin::L2,
            },
            recipient: [0u8; 20],
            token_address: DISABLED_TOKEN,
            amount: 100u128.into(),
            ferry_tip: 10u128.into(),
        };

        filter_input.send(disabled_withdrawal).await.unwrap();
        filter_input.send(enabled_withdrawal1).await.unwrap();
        filter_input.send(enabled_withdrawal2).await.unwrap();
        filter_input.send(disabled_withdrawal).await.unwrap();

        drop(filter_input);
        task.await.unwrap();

        assert_eq!(
            FerryAction::Ferry {
                withdrawal: enabled_withdrawal1,
                prio: 11u128.into(),
            },
            filter_output.recv().await.unwrap(),
        );

        assert_eq!(
            FerryAction::Ferry {
                withdrawal: enabled_withdrawal2,
                prio: 10u128.into(),
            },
            filter_output.recv().await.unwrap(),
        );

        assert!(filter_output.recv().await.is_none(),);
    }
}

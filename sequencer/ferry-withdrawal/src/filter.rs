use crate::ferry::FerryAction;
use gasp_types::{Withdrawal, U256};
use l1api::L1Interface;
use l2api::L2Interface;
use std::collections::HashSet;
use tokio::sync::mpsc;

pub struct Filter<L1, L2> {
    l1: L1,
    l2: L2,
    input: mpsc::Receiver<Withdrawal>,
    output: mpsc::Sender<FerryAction>,
    enabled: HashSet<[u8; 20]>,
}

impl<L1, L2> Filter<L1, L2>
where
    L1: L1Interface,
    L2: L2Interface,
{
    pub fn new(
        l1: L1,
        l2: L2,
        input: mpsc::Receiver<Withdrawal>,
        output: mpsc::Sender<FerryAction>,
        enabled: Vec<[u8; 20]>,
    ) -> Self {
        Self {
            l1,
            l2,
            input,
            output,
            enabled: enabled.into_iter().collect(),
        }
    }

    pub async fn run(&mut self) {
        while let Some(w) = self.input.recv().await {
            let is_enabled = self.enabled.contains(&w.token_address);
            //TODO: add more sophisticated logic here
            let profit = w.ferry_tip;
            if is_enabled && !profit.is_zero() {
                self.output
                    .send(FerryAction::Ferry {
                        withdrawal: w,
                        prio: profit,
                    })
                    .await.expect("infinite");
            }
        }
        tracing::info!("closing filter service");
    }
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
        let mut filter = Filter::new(
            l1mock,
            l2mock,
            input,
            output,
            vec![ENABLED_TOKEN1, ENABLED_TOKEN2],
        );


        let task = tokio::spawn(async move {
            filter.run().await;
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
            FerryAction::Ferry{
                withdrawal: enabled_withdrawal1,
                prio: 11u128.into(),
            },
            filter_output.recv().await.unwrap(),
        );

        assert_eq!(
            FerryAction::Ferry{
                withdrawal: enabled_withdrawal2,
                prio: 10u128.into(),
            },
            filter_output.recv().await.unwrap(),
        );

        assert!(
            filter_output.recv().await.is_none(),
        );

    }
}

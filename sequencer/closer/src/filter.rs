use gasp_types::Withdrawal;
use stash_api::{CreatedBy, StashInterface};
use tokio::sync::mpsc;

pub async fn filter_deposits_created_by_frontend(
    stash: impl StashInterface,
    mut input: mpsc::Receiver<Withdrawal>,
    output: mpsc::Sender<Withdrawal>,
) {
    while let Some(withdrawal) = input.recv().await {
        match stash
            .get_withdrawal_status(withdrawal.withdrawal_hash())
            .await
        {
            Ok(status) if status.created_by == CreatedBy::Frontend => {
                tracing::debug!(
                    "withdrawal rid : {} was created by frontend",
                    withdrawal.request_id.id
                );
                output.send(withdrawal).await.expect("infinite");
            }
            Ok(_) => {
                tracing::warn!(
                    "ignoring withdrawal rid : {} - not initated by frontend",
                    withdrawal.request_id.id
                );
            }
            Err(e) => {
                tracing::warn!(
                    "Stash error: {e}, ignoring withdrawal {}",
                    withdrawal.request_id.id
                );
            }
        };
    }
    tracing::info!("closing filter service");
}

pub async fn filter_deposits_without_fee(
    mut input: mpsc::Receiver<Withdrawal>,
    output: mpsc::Sender<Withdrawal>,
) {
    while let Some(withdrawal) = input.recv().await {
        let rid = withdrawal.request_id.id;
        if withdrawal.ferry_tip > 0u128.into() {
            tracing::debug!("withdrawal {rid} meets ferry criteria");
            output.send(withdrawal).await.expect("infinite");
        } else {
            tracing::debug!("withdrawal {rid} meets ferry criteria");
        }
    }
    tracing::info!("closing filter service");
}

#[cfg(test)]
mod test {
    use super::*;
    use gasp_types::RequestId;
    use mockall::predicate::eq;
    use stash_api::mock::MockStash;
    use stash_api::WithdrawalStatus;
    use tokio::time::Duration;

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
    async fn test_forwards_request_created_by_frontend() {
        let (sender, input) = mpsc::channel(100);
        let (output, mut receiver) = mpsc::channel(100);

        let mut stash = MockStash::new();
        let non_frontend_withdrawal = dummy_withdrawal(1);
        let frontend_withdrawal = dummy_withdrawal(2);

        stash
            .expect_get_withdrawal_status()
            .with(eq(non_frontend_withdrawal.withdrawal_hash()))
            .return_once(|_| {
                Ok(WithdrawalStatus {
                    created_by: CreatedBy::Other,
                })
            });
        stash
            .expect_get_withdrawal_status()
            .with(eq(frontend_withdrawal.withdrawal_hash()))
            .return_once(|_| {
                Ok(WithdrawalStatus {
                    created_by: CreatedBy::Frontend,
                })
            });

        let _t =
            tokio::spawn(
                async move { filter_deposits_created_by_frontend(stash, input, output).await },
            );

        sender.send(non_frontend_withdrawal).await.unwrap();
        sender.send(frontend_withdrawal).await.unwrap();

        assert_eq!(receiver.recv().await, Some(frontend_withdrawal));
        assert!(
            tokio::time::timeout(Duration::from_secs_f32(0.1), receiver.recv())
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn test_forwards_request_with_fee() {
        let (sender, input) = mpsc::channel(100);
        let (output, mut receiver) = mpsc::channel(100);

        let mut withdrawal_without_fee = dummy_withdrawal(1);
        withdrawal_without_fee.ferry_tip = 0u128.into();

        let withdrawal_with_fee = dummy_withdrawal(2);

        let _t = tokio::spawn(async move { filter_deposits_without_fee(input, output).await });

        sender.send(withdrawal_without_fee).await.unwrap();
        sender.send(withdrawal_with_fee).await.unwrap();

        assert_eq!(receiver.recv().await, Some(withdrawal_with_fee));
        assert!(
            tokio::time::timeout(Duration::from_secs_f32(0.1), receiver.recv())
                .await
                .is_err()
        );
    }
}

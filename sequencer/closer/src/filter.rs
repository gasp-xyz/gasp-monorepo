use gasp_types::Withdrawal;
use stash_api::{CreatedBy, StashInterface};
use tokio::sync::mpsc;

pub async fn filter_deposits(
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
                output
                    .send(withdrawal)
                    .await
                    .expect("infinite");
            },
            Ok(_) => {
                tracing::warn!("ignoring withdrawal {withdrawal} - not initated by frontend");
            },
            Err(e) => {
                tracing::warn!("err {e}, ignoring withdrawal {withdrawal}");
            }
        };
    }
    tracing::info!("closing filter service");
}

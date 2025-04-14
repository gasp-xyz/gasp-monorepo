use std::collections::HashSet;

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
                output.send(withdrawal).await.expect("infinite");
            }
            Ok(_) => {
                tracing::warn!("ignoring withdrawal {withdrawal} - not initated by frontend");
            }
            Err(e) => {
                tracing::warn!("err {e}, ignoring withdrawal {withdrawal}");
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
        if withdrawal.ferry_tip > 0u128.into() {
            output.send(withdrawal).await.expect("infinite");
        }
    }
    tracing::info!("closing filter service");
}

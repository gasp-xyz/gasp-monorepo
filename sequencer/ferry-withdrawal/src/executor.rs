use futures::{FutureExt, StreamExt};
use gasp_types::{Chain, Withdrawal, U256};
use l1api::{types::RequestStatus, L1Error, L1Interface};
use l2api::L2Interface;
use std::collections::{BTreeMap, HashMap, VecDeque};
use tokio::sync::mpsc;

#[derive(Debug, PartialEq)]
pub enum L1Action {
    Ferry { withdrawal: Withdrawal, prio: U256 },
    CloseFerriedWithdrawal { withdrawal: Withdrawal },
}

#[derive(Debug, thiserror::Error)]
pub enum ExecutorError {
    #[error("Could not find merkle root for request id {0}")]
    UnknownMerkleRoot(U256),

    #[error("L1 error: {0}")]
    L1Error(#[from] l1api::L1Error),

    #[error("L2 error: {0}")]
    L2Error(#[from] l2api::L2Error),
}

pub struct Executor<L1, L2> {
    l1: L1,
    l2: L2,
    chain: Chain,
    account: [u8; 20],
    input: mpsc::Receiver<L1Action>,
    closable_withdrawals: VecDeque<Withdrawal>,
    ferryable_withdrawals: HashMap<U256, (U256, Withdrawal)>,
    balances: HashMap<[u8; 20], U256>,
}

impl<L1, L2> Executor<L1, L2>
where
    L1: L1Interface,
    L2: L2Interface,
{
    pub fn new(
        l1: L1,
        l2: L2,
        account: [u8; 20],
        chain: Chain,
        input: mpsc::Receiver<L1Action>,
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
        }
    }

    pub async fn ferry_withdrawal(&mut self) -> Result<(), ExecutorError> {
        let balances = self.balances.clone();
        let latest_finalized = self.l1.get_latest_finalized_request_id().await?.unwrap_or_default();
        let len = self.ferryable_withdrawals.len();
        self.ferryable_withdrawals.retain(|request_id ,_| request_id > &latest_finalized.into());

        tracing::debug!("removed {n} withdrawals that are already finalized", n = len - self.ferryable_withdrawals.len());

        let req_to_ferrry = self.ferryable_withdrawals.iter().filter(|(_, (_, w))| {
            let required_tokens_amount = w.amount - w.ferry_tip;
            let balance = balances.get(&w.token_address).cloned().unwrap_or_default();
            balance > required_tokens_amount
        })
            .max_by_key(|(_, (prio, _))| prio).clone()
            .map(|(_, (_, w))| *w);

        if let Some(w) = req_to_ferrry {
            if let RequestStatus::Pending = self.l1.get_status(w.withdrawal_hash()).await?{
                let status = self.l1.ferry_withdrawal(w.clone()).await;
                match status {
                    Ok(hash) => {
                        tracing::info!("withdrawal ferried successfully {hash}");
                    },
                    Err(L1Error::TxReverted(hash)) => {
                        tracing::warn!("withdrawal ferried unsuccessfully {hash}");
                    },
                    Err(e) => {
                        return Err(e.into());
                    }
                }
                self.ferryable_withdrawals.remove(&w.request_id.id);
            }
        }

        Ok(())
    }


    pub async fn close_withdrawal(&self, withdrawal: Withdrawal) -> Result<(), ExecutorError> {
        if let RequestStatus::Ferried(_) = self.l1.get_status(withdrawal.withdrawal_hash()).await? {
            let req_id = withdrawal.request_id.id.try_into().unwrap();
            let (root, range) = self
                .l1
                .get_merkle_root(req_id)
                .await?
                .ok_or(ExecutorError::UnknownMerkleRoot(withdrawal.request_id.id))?;

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

    pub async fn close_all_withdrawals(& mut self) -> Result<(), ExecutorError> {
        while let Some(w) = self.closable_withdrawals.pop_back() {
            let result = self.close_withdrawal(w).await;
            let post_status = self.l1.get_status(w.withdrawal_hash()).await?;
            match (result, post_status) {
                (Ok(_), _) => {
                    tracing::debug!("withdrawal closed successfully {w}");
                }
                (Err(e), RequestStatus::Closed) => {
                    tracing::warn!("withdrawal closed by someone else");
                }
                (Err(e), _) => {
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    pub async fn get_balance(&self, token_address: [u8;20]) -> Result<U256, ExecutorError> {
        let balance = if (token_address == l1api::NATIVE_TOKEN_ADDRESS) {
            self.l1.native_balance(self.account).await?
        }else{
            self.l1.erc20_balance(token_address, self.account).await?
        };
        Ok(balance.into())
    }


    #[tracing::instrument(skip_all)]
    pub async fn refresh_balances(&mut self) -> Result<(), ExecutorError> {
        let tokens = self.balances.iter().map(|(token,amount)| 
            self.get_balance(*token)
            .map(|result| result.map(|balance| (*token, balance)))
        ).collect::<Vec<_>>();

        let balances = futures::future::join_all(tokens)
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

        for (token, balance) in balances {
            self.balances.insert(token, balance);
        }

        Ok(())
    }

    pub async fn track_balance(&mut self, token_address: [u8;20]) -> Result<(), ExecutorError> {
        if let None = self.balances.get(&token_address) {
            let balance = self.get_balance(token_address).await?;
            self.balances.insert(token_address, balance.into());
        }
        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), ExecutorError> {
        loop {

            while let Ok(req) = self.input.try_recv(){
                match req {
                    L1Action::Ferry { withdrawal, prio } => {
                        self.track_balance(withdrawal.token_address).await?;
                        self.ferryable_withdrawals.insert(withdrawal.request_id.id, (prio, withdrawal));
                    },
                    L1Action::CloseFerriedWithdrawal { withdrawal } => {
                        self.closable_withdrawals.push_front(withdrawal);
                    },
                }
            }

            if self.closable_withdrawals.len() > 0 {
                tracing::info!("found {n} closable withdrawals", n = self.closable_withdrawals.len());
                self.close_all_withdrawals().await?;
                self.refresh_balances().await?;
                continue;
            }

            if self.ferryable_withdrawals.len() > 0 {
                tracing::info!("found {n} withdrawals ready to ferry", n = self.ferryable_withdrawals.len());
                self.ferry_withdrawal().await?;
                self.refresh_balances().await?;
                continue;
            }
        }
    }
}

#[cfg(test)]
mod test{

    fn works_fine_when_there_is_nothing_to_process(){

    }

}

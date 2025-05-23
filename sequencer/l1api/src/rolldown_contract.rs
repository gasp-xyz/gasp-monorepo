use super::{types, L1Error};
use crate::types::RequestStatus;
use crate::utils::simulate_send_and_wait_for_result;
use crate::NATIVE_TOKEN_ADDRESS;
use alloy::consensus::BlockHeader;
use alloy::contract::{CallBuilder, CallDecoder};
use futures::{Stream, StreamExt};
use lazy_static::lazy_static;
use tokio::time::sleep;
use tokio::time::Duration;

use alloy::network::Network;
use alloy::providers::{Provider, WalletProvider};
use alloy::transports::Transport;
use primitive_types::H256;

#[cfg(test)]
use alloy::network::NetworkWallet;

use prometheus::{opts, register_counter_vec, CounterVec};

lazy_static! {
    static ref ETH_CALL_COUNTER: CounterVec = register_counter_vec!(
        opts!("eth_call", "Number of HTTP requests made."),
        &["method"]
    )
    .unwrap();
}

#[derive(Clone)]
pub struct RolldownContract<T, P, N> {
    contract_handle: contract_bindings::rolldown::Rolldown::RolldownInstance<T, P, N>,
    dry_run: bool,
}

impl<T, P, N> RolldownContract<T, P, N>
where
    T: Transport + Clone,
    P: Provider<T, N> + WalletProvider<N>,
    N: Network,
{
    async fn execute_tx<D>(&self, call: CallBuilder<T, &P, D, N>) -> Result<H256, L1Error>
    where
        D: CallDecoder + Send + Sync + Unpin,
    {
        if self.dry_run {
            tracing::warn!("running in dry run mode - tx is not beeing executed");
            Ok(H256::default())
        } else {
            simulate_send_and_wait_for_result::<T, P, D, N>(self.contract_handle.provider(), call)
                .await
        }
    }

    #[tracing::instrument(skip(self, cancel))]
    pub async fn send_close_cancel_tx(
        &self,
        cancel: types::abi::Cancel,
        merkle_root: H256,
        proof: Vec<H256>,
    ) -> Result<H256, L1Error> {
        let proof = proof.into_iter().map(|elem| elem.0.into()).collect();
        let call = self
            .contract_handle
            .close_cancel(cancel, merkle_root.0.into(), proof);
        self.execute_tx(call).await
    }

    #[tracing::instrument(skip(self, withdrawal))]
    pub async fn hash(&self, withdrawal: gasp_types::Withdrawal) -> Result<H256, L1Error> {
        let call = self.contract_handle.hashWithdrawal(withdrawal.into());
        Ok(call.call().await?._0.0.into())
    }

    #[tracing::instrument(skip(self, withdrawal))]
    pub async fn send_ferry_withdrawal(
        &self,
        withdrawal: gasp_types::Withdrawal,
    ) -> Result<H256, L1Error> {
        let amount = withdrawal.amount - withdrawal.ferry_tip;
        let call = self.contract_handle.ferryWithdrawal(withdrawal.into());
        if withdrawal.token_address == NATIVE_TOKEN_ADDRESS {
            self.execute_tx(call.value(gasp_types::into_l1_u256(amount)))
                .await
        } else {
            self.execute_tx(call).await
        }
    }

    #[tracing::instrument(skip(self, request_id))]
    pub async fn get_deposit(
        &self,
        request_id: u128,
    ) -> Result<crate::types::abi::Deposit, L1Error> {
        let call = self
            .contract_handle
            .deposits(gasp_types::into_l1_u256(request_id.into()));
        let deposit = call.call().await?;
        Ok(crate::types::abi::Deposit {
            requestId: deposit.requestId,
            depositRecipient: deposit.depositRecipient,
            tokenAddress: deposit.tokenAddress,
            amount: deposit.amount,
            timeStamp: deposit.timeStamp,
            ferryTip: deposit.ferryTip,
        })
    }

    #[tracing::instrument(skip(self, withdrawal))]
    pub async fn send_close_withdrawal_tx(
        &self,
        withdrawal: gasp_types::Withdrawal,
        merkle_root: H256,
        proof: Vec<H256>,
    ) -> Result<H256, L1Error> {
        let proof = proof.into_iter().map(|elem| elem.0.into()).collect();
        let native_withdrawal: types::abi::Withdrawal = withdrawal.into();
        let call =
            self.contract_handle
                .closeWithdrawal(native_withdrawal, merkle_root.0.into(), proof);
        self.execute_tx(call).await
    }

    #[cfg(test)]
    #[tracing::instrument(skip_all)]
    pub(crate) async fn deploy(provider: P) -> Result<Self, L1Error> {
        let sender = provider.wallet().default_signer_address();
        let contract_handle =
            contract_bindings::rolldown::Rolldown::RolldownInstance::<T, P, N>::deploy(provider)
                .await?;
        tracing::info!("contract deployed");

        let call = contract_handle.initialize(sender, 0, sender);
        simulate_send_and_wait_for_result(contract_handle.provider(), call).await?;
        tracing::info!("contract initialized");

        Ok(RolldownContract {
            contract_handle,
            dry_run: false,
        })
    }

    #[cfg(test)]
    #[tracing::instrument(skip_all)]
    pub(crate) async fn deposit_native(
        &self,
        amount: u128,
        ferry_tip: u128,
    ) -> Result<(), L1Error> where {
        let call = self
            .contract_handle
            .deposit_native_1(alloy::primitives::U256::from(ferry_tip))
            .value(alloy::primitives::U256::from(amount));

        self.execute_tx(call).await?;
        Ok(())
    }

    #[cfg(test)]
    #[tracing::instrument(skip_all)]
    pub(crate) async fn submit_merkle_root(
        &self,
        root: [u8; 32],
        range: (u128, u128),
    ) -> Result<(), L1Error> where {
        let range = contract_bindings::rolldown::IRolldownPrimitives::Range {
            start: gasp_types::into_l1_u256(gasp_types::U256::from(range.0)),
            end: gasp_types::into_l1_u256(gasp_types::U256::from(range.1)),
        };

        let call = self.contract_handle.update_l1_from_l2(root.into(), range);

        self.execute_tx(call).await?;
        Ok(())
    }
}

use async_stream::stream;

#[tracing::instrument(skip_all)]
fn dedup_stream<S>(mut input: S) -> impl Stream<Item = S::Item>
where
    S: Stream + Unpin,
    S::Item: PartialEq + Clone,
{
    stream! {
        let mut last = None;

        while let Some(item) = input.next().await {
            if Some(&item) != last.as_ref() {
                tracing::debug!("received new unique elem");
                last = Some(item.clone());
                yield item;
            }else{
                tracing::debug!("skipping previosly yielded element");
            }
        }
    }
}

impl<T, P, N> RolldownContract<T, P, N>
where
    T: Transport + Clone,
    P: Provider<T, N> + Clone,
    N: Network,
{
    #[tracing::instrument(skip(self))]
    pub async fn subscribe_blocks(
        &self,
    ) -> Result<impl Stream<Item = Result<u128, L1Error>>, L1Error> {
        let p = self.contract_handle.provider();
        let subscription = p.subscribe_blocks().await?;
        Ok(subscription
            .into_stream()
            .map(|elem| Ok(elem.number() as u128)))
    }

    #[tracing::instrument(skip(self))]
    pub async fn subscribe_blocks_polling(
        &self,
        timeout: Duration,
    ) -> Result<impl Stream<Item = Result<u128, L1Error>>, L1Error> {
        let p = self.contract_handle.provider().clone();
        let mut latest = p.get_block_number().await?;

        let stream = stream! {
            loop {
                    tokio::time::sleep(timeout).await;
                    match p.get_block_number().await
                {
                        Ok(elem) if elem > latest => {
                            for nr in (latest+1)..=elem{
                                latest = nr;
                                yield Ok(nr as u128);
                            }
                        },
                        Err(err) => {
                            tracing::error!("could not get latest block number {err}");
                            yield Err(err.into());
                        },
                        _ => {}
                    }
            }
        };
        Ok(stream)
    }

    #[tracing::instrument(skip(self))]
    pub async fn subscribe_new_batch(
        &self,
    ) -> Result<impl Stream<Item = (H256, (u128, u128))>, L1Error> {
        let p = self.contract_handle.provider();
        let filter = self.contract_handle.L2UpdateAccepted_filter().filter;
        Ok(p.subscribe_logs(&filter)
            .await
            .unwrap()
            .into_stream()
            .map(|elem| {
                let log = elem
                    .log_decode::<crate::types::abi::L2UpdateAccepted>()
                    .expect("can decode subscribed log");
                let merkle_root = log.data().root.0;
                let start = gasp_types::from_l1_u256(log.data().range.start);
                let end = gasp_types::from_l1_u256(log.data().range.end);
                (
                    merkle_root.into(),
                    (start.try_into().unwrap(), end.try_into().unwrap()),
                )
            }))
    }

    #[allow(clippy::type_complexity)]
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_latest_batch(&self) -> Result<Option<(H256, u128, (u128, u128))>, L1Error> {
        let amount = self.get_amount_of_merkle_roots().await?;
        if amount > 0 {
            let batch_id = amount - 1;
            self.get_batch(batch_id)
                .await
                .map(|elem| elem.map(|(merkle_root, range)| (merkle_root, batch_id, range)))
        } else {
            Ok(None)
        }
    }

    #[tracing::instrument(level = "debug", skip(self), ret)]
    #[allow(clippy::type_complexity)]
    pub async fn get_batch(&self, batch_id: u128) -> Result<Option<(H256, (u128, u128))>, L1Error> {
        let root = self.get_merkle_root_by_id(batch_id).await?;
        let range = self.get_request_range_from_merkle_root(root).await?;
        Ok(Some((root.into(), range)))
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_deposit_if_exists(
        &self,
        request_id: u128,
    ) -> Result<Option<types::abi::Deposit>, L1Error> {
        let deposit = self
            .contract_handle
            .deposits(gasp_types::into_l1_u256(request_id.into()))
            .call()
            .await?;
        if deposit.requestId.id > gasp_types::into_l1_u256(0u128.into()) {
            Ok(Some(types::abi::Deposit {
                requestId: deposit.requestId,
                depositRecipient: deposit.depositRecipient,
                tokenAddress: deposit.tokenAddress,
                amount: deposit.amount,
                timeStamp: deposit.timeStamp,
                ferryTip: deposit.ferryTip,
            }))
        } else {
            Ok(None)
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_latest_deposit(&self) -> Result<Option<types::abi::Deposit>, L1Error> {
        match self.get_latest_reqeust_id().await? {
            Some(mut id) => {
                while id > 0u128 {
                    if let Some(deposit) = self.get_deposit_if_exists(id).await? {
                        return Ok(Some(deposit));
                    } else {
                        id -= 1;
                    }
                }
                Ok(None)
            }
            None => Ok(None),
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_latest_reqeust_id(&self) -> Result<Option<u128>, L1Error> {
        let next_request_id = self.get_next_request_id().await?;
        if next_request_id == 1 {
            Ok(None)
        } else {
            Ok(next_request_id.checked_sub(1u128))
        }
    }

    #[tracing::instrument(skip(self, timeout))]
    pub async fn subscribe_new_batch_polling<'a>(
        &'a self,
        timeout: Duration,
    ) -> Result<impl Stream<Item = (H256, (u128, u128))> + 'a, L1Error> {
        let batch = self.get_latest_batch().await?;
        tracing::info!("starting batch subscription - latest batch {batch:?}");

        let stream = stream! {

            let mut last_sent = None;

            if let Some((root, batch_id ,range)) = batch{
                last_sent = Some(batch_id);
                yield (root, range);
            }

            loop {
                match (self.get_latest_batch().await, last_sent){
                    (Ok(Some((_, batch_id, range))), Some(start)) if batch_id > start=> {
                        tracing::debug!("new batch found {batch_id} {range:?}");
                        for id in start..=batch_id{
                            match self.get_batch(id).await {
                                Ok(Some((root, range))) => {
                                    last_sent = Some(id);
                                    yield (root, range);
                                }
                                Ok(None) => {
                                    tracing::warn!("could not fetch batch, retrying");
                                }
                                Err(e) => {
                                    tracing::warn!("could not fetch batch with err {e}");
                                    break;
                                },
                            }
                        }
                    },
                    (Ok(Some((_root, batch_id, range))), None) => {
                        tracing::debug!("new batch found {batch_id} {range:?}");
                        for id in 0..=batch_id{
                            match self.get_batch(id).await {
                                Ok(Some((root, range))) => {
                                    last_sent = Some(batch_id);
                                    yield (root, range);
                                }
                                Ok(None) => {
                                    tracing::warn!("could not fetch batch, retrying");
                                }
                                Err(e) => {
                                    tracing::warn!("could not fetch batch with err {e}");
                                    break;
                                },
                            }
                        }
                    },
                    (Err(err), _) => {
                        tracing::warn!("could not get batch - err {err}");
                        sleep(timeout).await;
                    },
                    _ => {
                        tracing::debug!("sleeping for {timeout:?}");
                        sleep(timeout).await;
                    }
                }
            }
        };

        Ok(dedup_stream(stream.boxed()))
    }

    #[tracing::instrument(skip(self))]
    pub async fn subscribe_deposit(&self) -> Result<impl Stream<Item = u128>, L1Error> {
        let p = self.contract_handle.provider();
        let filter = self
            .contract_handle
            .DepositAcceptedIntoQueue_filter()
            .filter;
        Ok(p.subscribe_logs(&filter)
            .await?
            .into_stream()
            .map(|elem| {
                let log = elem
                    .log_decode::<crate::types::abi::DepositAcceptedIntoQueue>()
                    .expect("can decode subscribed log");
                gasp_types::from_l1_u256(log.data().requestId)
                    .try_into()
                    .unwrap()
            })
            .map(|request_id| request_id))
    }

    #[tracing::instrument(skip(self))]
    pub async fn subscribe_deposit_polling<'a>(
        &'a self,
        timeout: Duration,
    ) -> Result<impl Stream<Item = u128> + 'a, L1Error> {
        let get_id = |elem: types::abi::Deposit| {
            gasp_types::from_l1_u256(elem.requestId.id)
                .try_into()
                .unwrap()
        };
        let mut last_sent: u128 = self
            .get_latest_deposit()
            .await?
            .map(get_id)
            .unwrap_or_default();

        let stream = stream! {
            if last_sent > 0 {
                yield last_sent;
            }

            loop {
                sleep(timeout).await;
                match self.get_latest_deposit().await.map(|e| e.map(get_id)){
                    Ok(Some(latest_id)) => {
                        let start = last_sent;
                        for id in start..=latest_id{
                            if let Ok(Some(deposit_id)) = self.get_deposit_if_exists(id).await.map(|e| e.map(get_id)){
                                if deposit_id > last_sent {
                                    last_sent = deposit_id;
                                    yield deposit_id;
                                }
                            }
                        }
                    },
                    Err(err) => {
                        tracing::warn!("could not get deposit - err {err}");
                        sleep(timeout).await;
                    }
                    _ => {}
                }
            }
        };

        Ok(dedup_stream(stream.boxed()))
    }

    pub fn address(&self) -> [u8; 20] {
        (*self.contract_handle.address()).into()
    }

    pub async fn is_admin(&self, address: [u8; 20]) -> Result<bool, L1Error> {
        let admin_role = self.contract_handle.DEFAULT_ADMIN_ROLE().call().await?._0;
        Ok(self
            .contract_handle
            .hasRole(admin_role, address.into())
            .call()
            .await?
            ._0)
    }

    pub async fn is_updater(&self, address: [u8; 20]) -> Result<bool, L1Error> {
        let updater_role = self.contract_handle.UPDATER_ROLE().call().await?._0;
        Ok(self
            .contract_handle
            .hasRole(updater_role, address.into())
            .call()
            .await?
            ._0)
    }

    pub fn new(provider: P, address: [u8; 20]) -> Self {
        RolldownContract {
            contract_handle: contract_bindings::rolldown::Rolldown::RolldownInstance::new(
                address.into(),
                provider,
            ),
            dry_run: false,
        }
    }

    pub fn new_dry_run(provider: P, address: [u8; 20]) -> Self {
        RolldownContract {
            contract_handle: contract_bindings::rolldown::Rolldown::RolldownInstance::new(
                address.into(),
                provider,
            ),
            dry_run: true,
        }
    }

    pub async fn find_l2_batch(&self, request_id: u128) -> Result<[u8; 32], L1Error> {
        ETH_CALL_COUNTER.with_label_values(&["find_l2_batch"]).inc();
        let request_id = alloy::primitives::U256::from(request_id);
        let call = self.contract_handle.find_l2_batch(request_id);
        Ok(call.call().await?._0.0)
    }

    pub async fn get_request_range_from_merkle_root(
        &self,
        merkle_root: [u8; 32],
    ) -> Result<(u128, u128), L1Error> {
        ETH_CALL_COUNTER
            .with_label_values(&["merkleRootRange"])
            .inc();
        let call = self.contract_handle.merkleRootRange(merkle_root.into());
        let range = call.call().await?;

        let range_start = range.start.try_into().map_err(|_| L1Error::OverflowError)?;
        let range_end = range.end.try_into().map_err(|_| L1Error::OverflowError)?;
        Ok((range_start, range_end))
    }

    pub async fn get_next_request_id(&self) -> Result<u128, L1Error> {
        ETH_CALL_COUNTER.with_label_values(&["counter"]).inc();
        let call = self.contract_handle.counter();
        let result = call.call().await?;
        result._0.try_into().map_err(|_| L1Error::OverflowError)
    }

    pub async fn get_amount_of_merkle_roots(&self) -> Result<u128, L1Error> {
        ETH_CALL_COUNTER
            .with_label_values(&["getMerkleRootsLength"])
            .inc();
        let call = self.contract_handle.getMerkleRootsLength();
        let result = call.call().await?;
        result._0.try_into().map_err(|_| L1Error::OverflowError)
    }

    pub async fn get_merkle_root_by_id(&self, merkle_root_id: u128) -> Result<[u8; 32], L1Error> {
        ETH_CALL_COUNTER.with_label_values(&["roots"]).inc();
        let merkle_root_id = alloy::primitives::U256::from(merkle_root_id);
        let latest_root = self.contract_handle.roots(merkle_root_id).call().await?._0;
        Ok(latest_root.0)
    }

    pub async fn get_update_impl(
        &self,
        start: u128,
        end: u128,
    ) -> Result<types::abi::L1Update, L1Error> {
        ETH_CALL_COUNTER
            .with_label_values(&["getPendingRequests"])
            .inc();
        let range_start = alloy::primitives::U256::from(start);
        let range_end = alloy::primitives::U256::from(end);
        let call = self
            .contract_handle
            .getPendingRequests(range_start, range_end);
        Ok(call.call().await?._0)
    }

    pub async fn get_request_status(&self, request_hash: H256) -> Result<RequestStatus, L1Error> {
        ETH_CALL_COUNTER
            .with_label_values(&["processedL2Requests"])
            .inc();
        let request_hash = request_hash.0.into();
        let request_status = self
            .contract_handle
            .processedL2Requests(request_hash)
            .call()
            .await
            .map(|elem| elem._0)?;

        let closed = self.contract_handle.CLOSED().call().await?._0;
        if request_status == closed {
            Ok(RequestStatus::Closed)
        } else if request_status.0 == [0u8; 20] {
            Ok(RequestStatus::Pending)
        } else {
            Ok(RequestStatus::Ferried(request_status.into()))
        }
    }

    pub async fn is_request_closed(&self, request_hash: H256) -> Result<bool, L1Error> {
        ETH_CALL_COUNTER
            .with_label_values(&["processedL2Requests"])
            .inc();
        let request_hash = request_hash.0.into();
        let request_status = self
            .contract_handle
            .processedL2Requests(request_hash)
            .call()
            .await
            .map(|elem| elem._0)?;

        let closed = self.contract_handle.CLOSED().call().await?._0;
        let is_closed = request_status == closed;
        Ok(is_closed)
    }

    pub async fn is_request_ferried(&self, request_hash: H256) -> Result<bool, L1Error> {
        ETH_CALL_COUNTER
            .with_label_values(&["processedL2Requests"])
            .inc();
        let request_hash = request_hash.0.into();
        let request_status = self
            .contract_handle
            .processedL2Requests(request_hash)
            .call()
            .await
            .map(|elem| elem._0)?;

        let closed = self.contract_handle.CLOSED().call().await?._0;
        let is_closed = request_status == closed;
        Ok(is_closed)
    }
}

use gasp_types::{Withdrawal, U256};
use itertools::Itertools;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Mutex;
use tokio::sync::mpsc;
use futures::{FutureExt, StreamExt};
use gasp_types::{Chain, L2Request, Origin, RequestId, H256};
use l1api::L1Interface;
use l2api::L2Interface;
use l1api::types::RequestStatus;

#[derive(thiserror::Error, Debug)]
pub enum FerryError {
    #[error("L1Error `{0}`")]
    L1Error(#[from] l1api::L1Error),

    #[error("L2Error `{0}`")]
    L2Error(#[from] l2api::L2Error),

    #[error("Reqeust `{request_id:?}` not found for chain `{chain:?}`")]
    RequestIdDoesNotExistsOnL2 { request_id: U256, chain: Chain },

    #[error("Unknown merkle root for finalized request id `{0}`")]
    UnknownMerkleRootForFinalizedRequestId(U256),

    #[error("Reqeust `{request_id:?}` not found for chain `{chain:?}`")]
    WithdrawalIdDoesNotExistsOnL2 { request_id: U256, chain: Chain },
}

pub type FerryResult<T> = Result<T, FerryError>;

//TODO: rename to Hunter
pub struct FerryWithdrawal<L1, L2> {
    chain: gasp_types::Chain,
    l1: L1,
    l2: L2,
    sink: mpsc::Sender<Withdrawal>,
    latest_processed: U256,
    // tokens_to_ferry: BTreeSet<[u8; 20]>,
    // l2_requests_cache: Mutex<RefCell<BTreeMap<U256, L2Request>>>,
}

impl<L1, L2> FerryWithdrawal<L1, L2>
where
    L1: L1Interface,
    L2: L2Interface,
{
    pub fn new(chain: gasp_types::Chain, l1: L1, l2: L2,sink: mpsc::Sender<Withdrawal>) -> Self {
        FerryWithdrawal {
            chain,
            l1,
            l2,
            sink,
            latest_processed: 0u128.into(),
        }
    }

    #[tracing::instrument(skip_all, ret)]
    async fn get_requests_to_ferry(&self, l2_state: H256) -> FerryResult<Option<(u128, u128)>> {
        let latest_request_id_on_l1 = self.l1.get_latest_finalized_request_id().await?;
        let latest_request_id_on_l2 = self
            .l2
            .get_latest_created_request_id(self.chain, l2_state)
            .await?;
        Ok(match (latest_request_id_on_l1, latest_request_id_on_l2) {
            (Some(l1_request_id), Some(l2_request_id)) if l2_request_id > l1_request_id => {
                Some((l1_request_id + 1, l2_request_id))
            }
            (None, Some(l2_request_id)) => Some((1, l2_request_id)),
            _ => None,
        })
    }

    // fn remove_old_requests_from_cache(&mut self, latest_request_id: u128) {
    //     let guard = self.l2_requests_cache.lock().unwrap();
    //     let mut l2_requests_cache = guard.borrow_mut();
    //     let initial_len = l2_requests_cache.len();
    //     l2_requests_cache.retain(|key, _| *key >= latest_request_id.into());
    //     let removed = initial_len - l2_requests_cache.len();
    //     tracing::trace!("removed {removed} / {initial_len} requests from cache");
    // }

    #[tracing::instrument(skip_all)]
    async fn fetch_l2_requests(&self, range: Vec<u128>, at: H256) -> FerryResult<Vec<L2Request>> {
        tracing::trace!("fetching l2 requests for range {first:?} .. {last:?}", first = range.first(), last = range.last());
        let chain = self.chain;
        // let futures = {
        //     let guard = self.l2_requests_cache.lock().unwrap();
        //     range
        //         .iter()
        //         .filter(|elem| !guard.borrow().contains_key(&(**elem).into())).cloned().collect::<Vec<_>>()
        // };

        let futures = range.into_iter()
            .filter(|id| U256::from(*id) > self.latest_processed)
            .map(|id| {
                self.l2.get_l2_request(self.chain, id, at).map(move |elem| {
                    elem.map(|elem| {
                        elem.ok_or(FerryError::RequestIdDoesNotExistsOnL2 {
                            request_id: id.into(),
                            chain,
                        })
                    })
                })
            })
            .collect::<Vec<_>>();

         Ok(futures::future::join_all(futures)
            .await
            .into_iter()
            .collect::<Result<Result<Vec<_>, _>, _>>()??)

         // let guard = self.l2_requests_cache.lock().unwrap();
         // let mut cache = guard.borrow_mut();
         // cache.extend(result.into_iter().map(|req| (req.request_id(), req)));
         // range
         //     .into_iter()
         //     .map(|id| {
         //         cache
         //             .get(&id.into())
         //             .cloned()
         //             .ok_or(FerryError::RequestIdDoesNotExistsOnL2 {
         //                 request_id: id.into(),
         //                 chain,
         //             })
         //     })
         //     .collect::<Result<Vec<_>, _>>()
    }

    // pub async fn close_ferried_withdrawal(&mut self, at: H256) -> FerryResult<()> {
    //     // // if self.pending_ferry_requests.is_empty(){
    //     // //     return Ok(());
    //     // // }
    //     //
    //     // if let Some(latest_closable_req_id) = self.l1.get_latest_finalized_request_id().await? {
    //     //     let closable_ferried_txs = self.pending_ferry_requests
    //     //         .iter()
    //     //         .cloned()
    //     //         .take_while(|elem| (*elem) <= latest_closable_req_id.into())
    //     //         .collect::<Vec<_>>();
    //     //
    //     //     for id in closable_ferried_txs{
    //     //         let request_id = id.try_into().unwrap();
    //     //         let req = self.l2.get_l2_request(self.chain, request_id, at).await?
    //     //             .ok_or(FerryError::RequestIdDoesNotExistsOnL2 { request_id: id, chain: self.chain })?;
    //     //
    //     //         let withdrawal = if let L2Request::Withdrawal(w) = req { 
    //     //             Ok(w) 
    //     //         } else { 
    //     //             Err(FerryError::WithdrawalIdDoesNotExistsOnL2 { request_id: id, chain: self.chain }) 
    //     //         }?;
    //     //
    //     //         if let RequestStatus::Ferried(_) = self.l1.get_status(withdrawal.withdrawal_hash()).await?{
    //     //                 let (merkle_root, range) = self.l1.get_merkle_root(request_id).await?.ok_or(FerryError::UnknownMerkleRootForFinalizedRequestId(id.into()))?;
    //     //                 let proof = self.l2.get_merkle_proof(request_id, range, self.chain, at).await?;
    //     //                 self.l1.close_withdrawal(withdrawal, merkle_root.into(), proof).await?;
    //     //         }
    //     //         self.pending_ferry_requests.remove(&id);
    //     //     }
    //     // }
    //     // Ok(())
    // }

    pub async fn run(&mut self) -> FerryResult<()> {
        let mut stream = self.l2.header_stream(l2api::Finalization::Best).await?;
        //TODO replace with wait for the next block
        while let Some(elem) = stream.next().await {
            let (block_nr, at) = elem?;
            // self.close_ferried_withdrawal(at).await?;
            tracing::info!("#{block_nr} Looking for ferry requests at block {at}");


            if let Some((start, end)) = self.get_requests_to_ferry(at).await? {
                // self.remove_old_requests_from_cache(start);


                let range = start..=end;
                let chunks: Vec<Vec<_>> = range
                    .into_iter()
                    .chunks(50)
                    .into_iter()
                    .map(|range| range.collect())
                    .collect();

                // let mut requests = vec![];
                for chunk in chunks {

                    let reqs = self.fetch_l2_requests(chunk, at).await?.into_iter().filter_map(|elem| match elem{
                        L2Request::Withdrawal(withdrawal) => Some(withdrawal),
                        _ => None,
                    });
                    for r in reqs {
                        self.latest_processed = r.request_id.id.clone();
                        self.sink.send(r).await.expect("infinite queue");
                    }
                }

                // tracing::info!("found {len} requests to ferry", len = requests.len());
                //
                // let mut withdrawals = requests
                //     .into_iter()
                //     .filter_map(|elem| 
                //         match elem{
                //             L2Request::Withdrawal(w) if self.tokens_to_ferry.contains(&w.token_address) && !w.ferry_tip.is_zero() && !w.amount.is_zero() => {
                //                 tracing::info!("found {w:?} profitable withdrawal");
                //                 Some(w)
                //             } 
                //             _ => {
                //                 tracing::info!("ignoring request {elem:?}");
                //                 None
                //             }
                //         }
                //     ).collect::<Vec<_>>();
                //
                //  withdrawals.sort_by(|lhs, rhs| {
                //     let profit = |w: &Withdrawal| {
                //        let amount  = TryInto::<u128>::try_into(w.amount).unwrap() as f64;
                //         let tip  = TryInto::<u128>::try_into(w.ferry_tip).unwrap() as f64;
                //         tip / amount
                //     };
                //     if lhs.ferry_tip == rhs.ferry_tip {
                //         let lhs_profit = profit(lhs);
                //         let rhs_profit = profit(rhs);
                //         tracing::debug!("lhs_profit: {lhs_profit} rhs_profit: {rhs_profit}", lhs_profit = lhs_profit, rhs_profit = rhs_profit);
                //         lhs_profit.partial_cmp(&rhs_profit).unwrap_or(std::cmp::Ordering::Equal)
                //     } else {
                //         lhs.ferry_tip
                //         .partial_cmp(&rhs.ferry_tip)
                //         .unwrap_or(std::cmp::Ordering::Equal)
                //     }
                // });

                // for req in withdrawals.into_iter().rev(){
                //     if let RequestStatus::Pending = self.l1.get_status(req.withdrawal_hash()).await?{
                //         let result = self.l1.ferry_withdrawal(req).await?;
                //         self.pending_ferry_requests.insert(req.request_id.id);
                //         break;
                //     }
                // }
            }
        }
        tracing::warn!("header stream ended");
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(Some(100u128), None, None; "no l2 requests yet")]
    #[test_case(Some(100u128), Some(100u128), None; "everything already processed")]
    #[test_case(Some(110u128), Some(100u128), None; "invalid state")]
    #[test_case(Some(100u128), Some(101u128), Some((101u128, 101u128)); "single request available")]
    #[test_case(Some(100u128), Some(110u128), Some((101u128, 110u128)); "multiple request available")]
    #[test_case(None, Some(110u128), Some((1u128, 110u128)); "no finalized requests yet")]
    #[tokio::test]
    async fn test_get_requests_to_ferry(
        l1_req: Option<u128>,
        l2_req: Option<u128>,
        range: Option<(u128, u128)>,
    ) {
        let mut l1mock = l1api::mock::MockL1::default();
        let mut l2mock = l2api::mock::MockL2::default();

        l1mock
            .expect_get_latest_finalized_request_id()
            .return_once(move || Ok(l1_req.clone()));
        l2mock
            .expect_get_latest_created_request_id()
            .return_once(move |_, _| Ok(l2_req.clone()));

        let ferry = FerryWithdrawal::new(gasp_types::Chain::Ethereum, l1mock, l2mock, mpsc::channel(10000).0);

        assert_eq!(
            ferry.get_requests_to_ferry(H256::default()).await.unwrap(),
            range
        );
    }
}

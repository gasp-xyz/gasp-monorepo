use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

use futures::{FutureExt, StreamExt};
use gasp_types::{Chain, L2Request, Origin, RequestId, H256};
use l1api::L1Interface;
use l2api::L2Interface;

#[derive(thiserror::Error, Debug)]
pub enum FerryError {
    #[error("L1Error `{0}`")]
    L1Error(#[from] l1api::L1Error),

    #[error("L2Error `{0}`")]
    L2Error(#[from] l2api::L2Error),

    #[error("Reqeust `{request_id:?}` not found for chain `{chain:?}`")]
    RequestIdDoesNotExistsOnL2 { request_id: RequestId, chain: Chain },
}

pub type FerryResult<T> = Result<T, FerryError>;

pub struct FerryWithdrawal<L1, L2> {
    chain: gasp_types::Chain,
    l1: L1,
    l2: L2,
    pending_ferry_requests: BTreeSet<u128>,
    tokens_to_ferry: BTreeSet<[u8; 20]>,
}

impl<L1, L2> FerryWithdrawal<L1, L2>
where
    L1: L1Interface,
    L2: L2Interface,
{
    pub fn new(chain: gasp_types::Chain, l1: L1, l2: L2, tokens: Vec<[u8; 20]>) -> Self {
        FerryWithdrawal {
            chain,
            l1,
            l2,
            pending_ferry_requests: Default::default(),
            tokens_to_ferry: tokens.into_iter().collect(),
        }
    }

    #[tracing::instrument(skip_all,ret)]
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

    pub async fn run(&self) -> FerryResult<()> {
        let mut stream = self.l2.header_stream(l2api::Finalization::Best).await?;
        //TODO replace with wait for the next block
        while let Some(elem) = stream.next().await {
            let (block_nr, at) = elem?;
            tracing::info!("#{block_nr} Looking for ferry requests at block {at}");
            if let Some((start, end)) = self.get_requests_to_ferry(at).await? {
                let chunks: Vec<Vec<_>> = (start..=end)
                    .into_iter()
                    .chunks(50)
                    .into_iter()
                    .map(|chunk| chunk.collect())
                    .collect();
                for chunk in chunks {
                    let futures = chunk
                        .iter()
                        .map(|id| {
                            self.l2
                                .get_l2_request(self.chain, *id, at)
                                .map(|elem| elem.map(|elem| (*id, elem)))
                        })
                        .collect::<Vec<_>>();

                    let result = futures::future::join_all(futures)
                        .await
                        .into_iter()
                        .collect::<Result<Vec<_>, _>>()?;

                    let (existing_reqs, non_exising_reqs): (Vec<_>, Vec<_>) = result.into_iter().partition(|(id, maybe_req)| maybe_req.is_some());

                    non_exising_reqs.iter().for_each(|(id, _)| {
                        tracing::warn!("could not find request with id == {id}");
                    });

                    let x = existing_reqs.into_iter()
                        .filter_map(|(_, req)| match req{
                            Some(L2Request::Withdrawal(w)) if self.tokens_to_ferry.contains(&w.token_address) => Some(w),
                            _ => None
                        })
                        .collect::<Vec<_>>();

                    if let Some(w) = x.first(){
                        tracing::info!("ferry withdrawal with request_id == {id}", id = w.request_id.id);
                        let result = self.l1.ferry_withdrawal((*w).into()).await?;
                        break;
                    }

                    // if let Some((id, _)) = result.iter().find(|(_, maybe_req)| maybe_req.is_none()){
                    //     tracing::warn!("could not find request with id == {id}");
                    //     return Err(FerryError::RequestIdDoesNotExistsOnL2 {
                    //         request_id: RequestId {
                    //             origin: Origin::L2,
                    //             id: (*id).into(),
                    //         },
                    //         chain: self.chain,
                    //     });
                    // }else{
                    //
                    // }
                    //
                    // .ok_or(FerryError::RequestIdDoesNotExistsOnL2 {
                    //     request_id: RequestId {
                    //         origin: Origin::L2
                    //         id: *chunk.last().unwrap(),
                    //     },
                    //     chain: self.chain,
                    // })?;
                }
            }
        }
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

        let ferry = FerryWithdrawal::new(gasp_types::Chain::Ethereum, l1mock, l2mock, vec![]);

        assert_eq!(
            ferry.get_requests_to_ferry(H256::default()).await.unwrap(),
            range
        );
    }
}

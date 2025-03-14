use futures::sink::With;
use futures::{FutureExt, StreamExt};
use gasp_types::{Chain, L2Request, Origin, RequestId, H256};
use gasp_types::{Withdrawal, U256};
use itertools::Itertools;
use l1api::types::RequestStatus;
use l1api::L1Interface;
use l2api::L2Interface;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Mutex;
use tokio::sync::mpsc;

#[derive(thiserror::Error, Debug)]
pub enum HunterError {
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

    #[error("Sink send error")]
    SinkSendError(#[from] tokio::sync::mpsc::error::SendError<Withdrawal>),
}

pub type HunterResult<T> = Result<T, HunterError>;

//TODO: rename to Hunter
pub struct FerryHunter<L1, L2> {
    chain: gasp_types::Chain,
    l1: L1,
    l2: L2,
    sink: mpsc::Sender<Withdrawal>,
    latest_processed: u128,
}

impl<L1, L2> FerryHunter<L1, L2>
where
    L1: L1Interface,
    L2: L2Interface,
{
    pub fn new(chain: gasp_types::Chain, l1: L1, l2: L2, sink: mpsc::Sender<Withdrawal>) -> Self {
        FerryHunter {
            chain,
            l1,
            l2,
            sink,
            latest_processed: 0u128,
        }
    }

    #[tracing::instrument(skip_all, ret)]
    async fn get_requests_to_ferry(&self, l2_state: H256) -> HunterResult<Option<(u128, u128)>> {
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

    #[tracing::instrument(skip_all, ret)]
    pub async fn get_pending_withdrawal(
        &self,
        id: u128,
        at: H256,
    ) -> HunterResult<Option<Withdrawal>> {
        match self.l2.get_l2_request(self.chain, id, at).await? {
            Some(L2Request::Withdrawal(w)) => {
                match self.l1.get_status(w.withdrawal_hash()).await? {
                    RequestStatus::Pending => Ok::<_, HunterError>(Some(w)),
                    _ => Ok(None),
                }
            }
            _ => Ok(None),
        }
    }

    pub fn get_chunks(&self, start: u128, end: u128, chunk_size: usize) -> Vec<(u128, u128)> {
        (start..=end)
            .chunks(chunk_size)
            .into_iter()
            .map(|elem| {
                let mut x = elem.into_iter();
                let first = x.nth(0).expect("at least on element in chunk");
                let last = x.last().unwrap_or(first);
                (start, last)
            })
            .collect::<Vec<_>>()
    }

    pub async fn run(&mut self) -> HunterResult<()> {
        let mut stream = self.l2.header_stream(l2api::Finalization::Best).await?;
        //TODO replace with wait for the next block
        while let Some(elem) = stream.next().await {
            let (block_nr, at) = elem?;
            tracing::info!("#{block_nr} Looking for ferry requests at block {at}");

            let mut latest = self.latest_processed;
            if let Some((start, end)) = self.get_requests_to_ferry(at).await? {
                tracing::info!(
                    "start:{start} end:{end} latest{latest}",
                    latest = self.latest_processed
                );

                if end <= self.latest_processed {
                    continue;
                }
                let chunks =
                    self.get_chunks(std::cmp::max(start, self.latest_processed + 1), end, 50);
                for (id, range) in chunks.iter().enumerate() {
                    tracing::info!(
                        "fetching l2 requests for range {range:?} batch {id} / {chunks_count}",
                        id = id + 1,
                        chunks_count = chunks.len()
                    );
                    let queries = (range.0..=range.1)
                        .map(|elem| self.get_pending_withdrawal(elem, at))
                        .collect::<Vec<_>>();
                    for w in futures::future::try_join_all(queries)
                        .await?
                        .into_iter()
                        .filter_map(|elem| elem)
                    {
                        self.sink.send(w).await?;
                        latest = w.request_id.id.try_into().unwrap();
                    }
                }
            }
            self.latest_processed = latest;
        }
        tracing::warn!("header stream ended");
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use futures::stream;
    use gasp_types::Chain;
    use l1api::L1Error;
    use l2api::{HeaderStream, L2Error};
    use mockall::{predicate::{always, eq}, Sequence};
    use test_case::test_case;
    use tracing_test::traced_test;

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

        let ferry = FerryHunter::new(
            gasp_types::Chain::Ethereum,
            l1mock,
            l2mock,
            mpsc::channel(10000).0,
        );

        assert_eq!(
            ferry.get_requests_to_ferry(H256::default()).await.unwrap(),
            range
        );
    }

    #[tokio::test]
    async fn test_ferry_services_when_no_requests_to_ferry() {
        let mut l1mock = l1api::mock::MockL1::default();
        let mut l2mock = l2api::mock::MockL2::default();

        l2mock
            .expect_header_stream()
            .return_once(|_| Ok(Box::pin(stream::iter(vec![Ok((1u32, [0u8; 32].into()))]))));

        l1mock
            .expect_get_latest_finalized_request_id()
            .return_once(move || Ok(None));
        l2mock
            .expect_get_latest_created_request_id()
            .return_once(move |_, _| Ok(None));

        let (sender, receiver) = mpsc::channel(100);
        let handle = tokio::spawn(async move {
            FerryHunter::new(gasp_types::Chain::Ethereum, l1mock, l2mock, sender)
                .run()
                .await
                .unwrap();
        });

        handle.await.unwrap();
    }

    fn withdrawal_stream() -> impl Iterator<Item = Withdrawal> {
        std::iter::repeat(()).enumerate().map(|(id, _)| Withdrawal {
            request_id: RequestId {
                origin: gasp_types::Origin::L2,
                id: (id + 1).into(),
            },
            recipient: [0u8; 20],
            token_address: [0u8; 20],
            amount: 100u128.into(),
            ferry_tip: 1u128.into(),
        })
    }

    fn nth_withdrawal(id: usize) -> Withdrawal {
        withdrawal_stream().nth(id - 1).unwrap()
    }

    fn ret_nth_withdrawal(
        _chain: Chain,
        id: u128,
        _at: H256,
    ) -> Result<Option<L2Request>, L2Error> {
        Ok(Some(nth_withdrawal(id as usize).into()))
    }

    fn block_hash(id: usize) -> H256 {
        U256::from(id).to_big_endian().into()
    }

    fn header_stream() -> impl Iterator<Item = Result<(u32, H256), L2Error>> {
        std::iter::repeat(())
            .enumerate()
            .skip(1)
            .map(|(id, _)| Ok::<_, L2Error>((id as u32, block_hash(id))))
    }

    #[tokio::test]
    #[traced_test]
    async fn test_fetches_single_request() {
        let mut l1mock = l1api::mock::MockL1::default();
        let mut l2mock = l2api::mock::MockL2::default();

        l2mock
            .expect_header_stream()
            .returning(|_| Ok(Box::pin(stream::iter(header_stream().take(1)))));
        l1mock
            .expect_get_latest_finalized_request_id()
            .returning(|| Ok(None));
        l2mock
            .expect_get_latest_created_request_id()
            .returning(|_, _| Ok(Some(1u128)));
        l2mock
            .expect_get_l2_request()
            .with(always(), eq(1u128), always())
            .returning(ret_nth_withdrawal);
        l1mock
            .expect_get_status()
            .returning(|_| Ok(l1api::types::RequestStatus::Pending));

        let (sender, mut receiver) = mpsc::channel(100);
        let handle = tokio::spawn(async move {
            FerryHunter::new(gasp_types::Chain::Ethereum, l1mock, l2mock, sender)
                .run()
                .await
                .unwrap();
        });

        handle.await.unwrap();

        receiver.recv().await.unwrap();
        assert!(receiver.recv().await.is_none());
    }

    #[tokio::test]
    #[traced_test]
    async fn test_ferry_fetches_single_request_only_once() {
        let mut l1mock = l1api::mock::MockL1::default();
        let mut l2mock = l2api::mock::MockL2::default();

        l2mock
            .expect_header_stream()
            .returning(|_| Ok(Box::pin(stream::iter(header_stream().take(3)))));

        l1mock
            .expect_get_latest_finalized_request_id()
            .returning(|| Ok(None));

        l2mock
            .expect_get_latest_created_request_id()
            .with(always(), eq(block_hash(1)))
            .return_once(|_, _| Ok(Some(1u128)));
        l2mock
            .expect_get_latest_created_request_id()
            .with(always(), eq(block_hash(2)))
            .return_once(|_, _| Ok(Some(2u128)));
        l2mock
            .expect_get_latest_created_request_id()
            .with(always(), eq(block_hash(3)))
            .return_once(|_, _| Ok(Some(2u128)));

        l2mock
            .expect_get_l2_request()
            .times(1)
            .with(always(), eq(1u128), always())
            .returning(ret_nth_withdrawal);
        l2mock
            .expect_get_l2_request()
            .times(1)
            .with(always(), eq(2u128), always())
            .returning(ret_nth_withdrawal);

        l1mock
            .expect_get_status()
            .returning(|_| Ok(l1api::types::RequestStatus::Pending));
        let (sender, mut receiver) = mpsc::channel(100);
        let handle = tokio::spawn(async move {
            FerryHunter::new(gasp_types::Chain::Ethereum, l1mock, l2mock, sender)
                .run()
                .await
                .unwrap();
        });
        handle.await.unwrap();
        tracing::info!("ferry finished");
        assert_eq!(receiver.recv().await.unwrap(), nth_withdrawal(1));
        assert_eq!(receiver.recv().await.unwrap(), nth_withdrawal(2));
        assert!(receiver.recv().await.is_none());
    }
}

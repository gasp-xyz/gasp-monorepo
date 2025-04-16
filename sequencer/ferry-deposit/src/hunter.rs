use futures::StreamExt;
use gasp_types::H256;
use l1api::types::Deposit;
use l1api::L1Interface;
use l2api::L2Interface;
use tokio::sync::mpsc;

#[derive(thiserror::Error, Debug)]
pub enum HunterError {
    #[error("L1Error `{0}`")]
    L1(#[from] l1api::L1Error),

    #[error("L2Error `{0}`")]
    L2(#[from] l2api::L2Error),

    #[error("Sink send error")]
    SinkSendError(#[from] tokio::sync::mpsc::error::SendError<Deposit>),
}

pub type HunterResult<T> = Result<T, HunterError>;

//TODO: rename to Hunter
pub struct FerryHunter<L1, L2> {
    chain: gasp_types::Chain,
    l1: L1,
    l2: L2,
    sink: mpsc::Sender<Deposit>,
    latest_processed: u128,
}

impl<L1, L2> FerryHunter<L1, L2>
where
    L1: L1Interface,
    L2: L2Interface,
{
    pub fn new(chain: gasp_types::Chain, l1: L1, l2: L2, sink: mpsc::Sender<Deposit>) -> Self {
        FerryHunter {
            chain,
            l1,
            l2,
            sink,
            latest_processed: 0u128,
        }
    }

    async fn get_requests_to_ferry(&self, l2_state: H256) -> HunterResult<Option<(u128, u128)>> {
        let latest_available_request = self.l1.get_latest_reqeust_id().await?;
        let latest_processed = self
            .l2
            .get_latest_processed_request_id(self.chain, l2_state)
            .await?;

        Ok(match latest_available_request {
            Some(latest_avilable) if latest_avilable > latest_processed => {
                Some((latest_processed + 1, latest_avilable))
            }
            _ => None,
        })
    }

    #[tracing::instrument(level = "debug", skip(self), ret)]
    pub async fn get_deposit(&self, id: u128) -> HunterResult<Option<Deposit>> {
        Ok(self.l1.get_deposit(id).await?)
    }

    pub async fn run(&mut self) -> HunterResult<()> {
        let mut stream = self.l1.subscribe_deposit().await?;
        // while let Some(_elem) = stream.next().await
        loop {
            let (block_nr, at) = self.l2.get_best_block().await?;
            tracing::info!("#{block_nr} Looking for ferry requests at block {at}");

            let mut latest = self.latest_processed;
            if let Some((start, end)) = self.get_requests_to_ferry(at).await? {
                if end <= self.latest_processed {
                    continue;
                }
                let chunks =
                    common::get_chunks(std::cmp::max(start, self.latest_processed + 1), end, 25);
                for (id, range) in chunks.iter().enumerate() {
                    tokio::time::sleep(std::time::Duration::from_secs_f64(0.25)).await;
                    tracing::info!(
                        "looking for pending deposit {range:?} batch {id} / {chunks_count} (last : {end})",
                        id = id + 1,
                        chunks_count = chunks.len()
                    );
                    let queries = (range.0..=range.1)
                        .map(|elem| self.get_deposit(elem))
                        .collect::<Vec<_>>();

                    for w in futures::future::try_join_all(queries)
                        .await?
                        .into_iter()
                        .flatten()
                    {
                        self.sink.send(w).await?;
                        latest = w.request_id.id.try_into().unwrap();
                    }
                }
            }
            self.latest_processed = latest;
            // block until notified
            if stream.next().await.is_none() {
                break;
            }
        }
        tracing::warn!("header stream ended");
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use futures::stream;
    use gasp_types::RequestId;

    use l1api::L1Error;
    use mockall::{predicate::eq, Sequence};
    use test_case::test_case;
    use tracing_test::traced_test;

    #[test_case(100u128, None, None; "no l2 requests yet")]
    #[test_case(100u128, Some(100u128), None; "everything already processed")]
    #[test_case(110u128, Some(100u128), None; "invalid state")]
    #[test_case(100u128, Some(101u128), Some((101u128, 101u128)); "single request available")]
    #[test_case(100u128, Some(110u128), Some((101u128, 110u128)); "multiple request available")]
    #[test_case(0u128, Some(110u128), Some((1u128, 110u128)); "no finalized requests yet")]
    #[tokio::test]
    async fn test_get_requests_to_ferry(
        latest_processeed: u128,
        latest_available: Option<u128>,
        range: Option<(u128, u128)>,
    ) {
        let mut l1mock = l1api::mock::MockL1::default();
        let mut l2mock = l2api::mock::MockL2::default();

        l1mock
            .expect_get_latest_reqeust_id()
            .return_once(move || Ok(latest_available));
        l2mock
            .expect_get_latest_processed_request_id()
            .return_once(move |_, _| Ok(latest_processeed));

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
        let l2mock = l2api::mock::MockL2::default();

        l1mock
            .expect_subscribe_deposit()
            .return_once(|| Ok(stream::pending().boxed()));

        let (sender, __receiver) = mpsc::channel(100);
        let handle = tokio::spawn(async move {
            FerryHunter::new(gasp_types::Chain::Ethereum, l1mock, l2mock, sender)
                .run()
                .await
                .unwrap();
        });

        tokio::time::timeout(tokio::time::Duration::from_secs_f32(0.1), handle)
            .await
            .unwrap_err();
    }

    fn deposit_stream() -> impl Iterator<Item = Deposit> {
        std::iter::repeat(()).enumerate().map(|(id, _)| Deposit {
            request_id: RequestId {
                origin: gasp_types::Origin::L2,
                id: (id + 1).into(),
            },
            recipient: [0u8; 20],
            token_address: [0u8; 20],
            amount: 100u128.into(),
            ferry_tip: 1u128.into(),
            timestamp: 0u128.into(),
        })
    }

    fn nth_deposit(id: usize) -> Deposit {
        deposit_stream().nth(id - 1).unwrap()
    }

    fn ret_nth_deposit(id: u128) -> Result<Option<Deposit>, L1Error> {
        Ok(Some(nth_deposit(id as usize)))
    }

    #[tokio::test]
    #[traced_test]
    async fn test_fetches_single_request() {
        let mut l1mock = l1api::mock::MockL1::default();
        let mut l2mock = l2api::mock::MockL2::default();

        l1mock
            .expect_subscribe_deposit()
            .return_once(|| Ok(futures::stream::iter(std::iter::once(1u128)).boxed()));

        l2mock
            .expect_get_best_block()
            .return_once(|| Ok((1, H256::default())));

        l1mock
            .expect_get_latest_reqeust_id()
            .returning(|| Ok(Some(1u128)));
        l2mock
            .expect_get_latest_processed_request_id()
            .returning(|_, _| Ok(0u128));
        l1mock
            .expect_get_deposit()
            .with(eq(1u128))
            .returning(ret_nth_deposit);
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

        l1mock
            .expect_subscribe_deposit()
            .return_once(|| Ok(futures::stream::iter([1, 2, 3]).boxed()));
        l2mock
            .expect_get_best_block()
            .returning(|| Ok((1, H256::default())));

        l2mock
            .expect_get_latest_processed_request_id()
            .returning(|_, _| Ok(0u128));

        let mut seq = Sequence::new();
        l1mock
            .expect_get_latest_reqeust_id()
            .times(1)
            .in_sequence(&mut seq)
            .return_once(|| Ok(Some(1u128)));

        l1mock
            .expect_get_latest_reqeust_id()
            .times(2)
            .in_sequence(&mut seq)
            .returning(|| Ok(Some(2u128)));

        l1mock
            .expect_get_deposit()
            .with(eq(1u128))
            .returning(ret_nth_deposit);
        l1mock
            .expect_get_deposit()
            .with(eq(2u128))
            .returning(ret_nth_deposit);

        let (sender, mut receiver) = mpsc::channel(100);
        let handle = tokio::spawn(async move {
            FerryHunter::new(gasp_types::Chain::Ethereum, l1mock, l2mock, sender)
                .run()
                .await
                .unwrap();
        });
        handle.await.unwrap();
        tracing::info!("ferry finished");
        assert_eq!(receiver.recv().await.unwrap(), nth_deposit(1));
        assert_eq!(receiver.recv().await.unwrap(), nth_deposit(2));
        assert!(receiver.recv().await.is_none());
    }
}

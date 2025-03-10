use l1api::mock::MockL1;
use l2api::mock::MockL2;
use gasp_types::{H256, L2Request, Withdrawal, RequestId};
use futures::stream;
use crate::ferry::FerryWithdrawal;
use mockall::predicate::eq;
use mockall::predicate::always;

#[cfg(test)]
mod test{
    use super::*;
    use test_case::test_case;
    use tracing_test::traced_test;

    #[tokio::test]
    async fn test_ferry_services_when_no_requests_to_ferry() {
        let mut l1mock = l1api::mock::MockL1::default();
        let mut l2mock = l2api::mock::MockL2::default();

        l2mock.expect_header_stream().return_once(|_| Ok(Box::pin(stream::iter( vec![ Ok((1u32, [0u8; 32].into())), ]))));

        l1mock.expect_get_latest_finalized_request_id().return_once(move || Ok(None));
        l2mock.expect_get_latest_created_request_id().return_once(move |_,_| Ok(None));


        let handle = tokio::spawn(async move {
            FerryWithdrawal::new(gasp_types::Chain::Ethereum, l1mock, l2mock, vec![]).run().await.unwrap();
        });

        handle.await.unwrap();
    }

    #[tokio::test]
    #[traced_test]
    async fn test_ferry_single_request() {
        let mut l1mock = l1api::mock::MockL1::default();
        let mut l2mock = l2api::mock::MockL2::default();

        let withdrawal = Withdrawal {
            request_id: RequestId { origin: gasp_types::Origin::L2, id: 1u128.into() },
            recipient: [0u8; 20],
            token_address: [0u8; 20],
            amount: 100u128.into(),
            ferry_tip: 1u128.into(),
        };

        l2mock.expect_header_stream().return_once(|_| Ok(Box::pin(stream::iter( vec![ Ok((1u32, [0u8; 32].into())), ]))));

        l1mock.expect_get_latest_finalized_request_id().return_once(move || Ok(None));
        l2mock.expect_get_latest_created_request_id().return_once(move |_,_| Ok(Some(1u128)));

        l2mock.expect_get_l2_request()
            .times(1)
            .with(always(), eq(1u128), always())
            .return_once(move |_,_,_| Ok(Some(withdrawal.into())));

        l1mock.expect_ferry_withdrawal().return_once(move |_| Ok(H256::default()));

        let handle = tokio::spawn(async move {
            FerryWithdrawal::new(gasp_types::Chain::Ethereum, l1mock, l2mock, vec![withdrawal.token_address]).run().await.unwrap();
        });

        handle.await.unwrap();
    }
}


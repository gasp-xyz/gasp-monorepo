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
    use mockall::Sequence;
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

        l1mock.expect_get_status().return_once(|_| Ok(l1api::types::RequestStatus::Pending));
        l1mock.expect_ferry_withdrawal().return_once(move |_| Ok(H256::default()));

        let handle = tokio::spawn(async move {
            FerryWithdrawal::new(gasp_types::Chain::Ethereum, l1mock, l2mock, vec![withdrawal.token_address]).run().await.unwrap();
        });

        handle.await.unwrap();
    }

    #[tokio::test]
    #[traced_test]
    async fn test_ferry_fetches_single_request_only_once() {
        let mut l1mock = l1api::mock::MockL1::default();
        let mut l2mock = l2api::mock::MockL2::default();

        let withdrawal = Withdrawal {
            request_id: RequestId { origin: gasp_types::Origin::L2, id: 1u128.into() },
            recipient: [0u8; 20],
            token_address: [0u8; 20],
            amount: 100u128.into(),
            ferry_tip: 1u128.into(),
        };

        let withdrawal2 = Withdrawal {
            request_id: RequestId { origin: gasp_types::Origin::L2, id: 2u128.into() },
            recipient: [0u8; 20],
            token_address: [0u8; 20],
            amount: 100u128.into(),
            ferry_tip: 1u128.into(),
        };


        let first_block_hash = [0u8; 32].into();
        let second_block_hash = [1u8; 32].into();
        l2mock.expect_header_stream().return_once(move |_| Ok(Box::pin(stream::iter( vec![
            Ok((1u32, first_block_hash)), 
            Ok((2u32, second_block_hash)), 
        ]))));

        l1mock.expect_get_latest_finalized_request_id().returning(|| Ok(None));
        l2mock.expect_get_latest_created_request_id().with(always(), eq(first_block_hash)).return_once(|_,_| Ok(Some(1u128)));
        l2mock.expect_get_latest_created_request_id().with(always(), eq(second_block_hash)).return_once(|_,_| Ok(Some(2u128)));

        l2mock.expect_get_l2_request()
            .times(1)
            .with(always(), eq(1u128), always())
            .return_once(move |_,_,_| Ok(Some(withdrawal.into())));

        l2mock.expect_get_l2_request()
            .times(1)
            .with(always(), eq(2u128), always())
            .return_once(move |_,_,_| Ok(Some(withdrawal2.into())));

        let handle = tokio::spawn(async move {
            FerryWithdrawal::new(gasp_types::Chain::Ethereum, l1mock, l2mock, vec![]).run().await.unwrap();
        });

        handle.await.unwrap();
    }

    #[tokio::test]
    #[traced_test]
    async fn test_withdrawal_to_ferry_is_selected_based_on_tip() {
        let mut l1mock = l1api::mock::MockL1::default();
        let mut l2mock = l2api::mock::MockL2::default();

        let withdrawal = Withdrawal {
            request_id: RequestId { origin: gasp_types::Origin::L2, id: 1u128.into() },
            recipient: [0u8; 20],
            token_address: [0u8; 20],
            amount: 100u128.into(),
            ferry_tip: 2u128.into(),
        };

        let withdrawal2 = Withdrawal {
            request_id: RequestId { origin: gasp_types::Origin::L2, id: 2u128.into() },
            recipient: [0u8; 20],
            token_address: [0u8; 20],
            amount: 10u128.into(),
            ferry_tip: 2u128.into(),
        };


        let first_block_hash = [0u8; 32].into();
        l2mock.expect_header_stream().return_once(move |_| Ok(Box::pin(stream::iter( vec![
            Ok((2u32, first_block_hash)), 
        ]))));

        l1mock.expect_get_latest_finalized_request_id().returning(|| Ok(None));
        l2mock.expect_get_latest_created_request_id().returning(|_,_| Ok(Some(2u128)));

        l2mock.expect_get_l2_request()
            .times(1)
            .with(always(), eq(1u128), always())
            .return_once(move |_,_,_| Ok(Some(withdrawal.into())));

        l2mock.expect_get_l2_request()
            .times(1)
            .with(always(), eq(2u128), always())
            .return_once(move |_,_,_| Ok(Some(withdrawal2.into())));

        l1mock.expect_get_status().returning(|_| Ok(l1api::types::RequestStatus::Pending));
        l1mock.expect_ferry_withdrawal().with(eq(withdrawal2)).times(1).return_once(move |_| Ok(H256::default()));
        l1mock.expect_ferry_withdrawal().with(eq(withdrawal)).times(0).return_once(move |_| Ok(H256::default()));

        let handle = tokio::spawn(async move {
            FerryWithdrawal::new(gasp_types::Chain::Ethereum, l1mock, l2mock, vec![withdrawal.token_address]).run().await.unwrap();
        });

        handle.await.unwrap();
    }

    #[tokio::test]
    #[traced_test]
    async fn test_ferry_ignores_already_ferried_withdrawals() {
        let mut l1mock = l1api::mock::MockL1::default();
        let mut l2mock = l2api::mock::MockL2::default();

        let withdrawal = Withdrawal {
            request_id: RequestId { origin: gasp_types::Origin::L2, id: 1u128.into() },
            recipient: [0u8; 20],
            token_address: [0u8; 20],
            amount: 100u128.into(),
            ferry_tip: 2u128.into(),
        };

        let withdrawal2 = Withdrawal {
            request_id: RequestId { origin: gasp_types::Origin::L2, id: 2u128.into() },
            recipient: [0u8; 20],
            token_address: [0u8; 20],
            amount: 10u128.into(),
            ferry_tip: 2u128.into(),
        };


        let first_block_hash = [0u8; 32].into();
        l2mock.expect_header_stream().return_once(move |_| Ok(Box::pin(stream::iter( vec![
            Ok((2u32, first_block_hash)), 
        ]))));

        l1mock.expect_get_latest_finalized_request_id().returning(|| Ok(None));
        l2mock.expect_get_latest_created_request_id().returning(|_,_| Ok(Some(2u128)));

        l2mock.expect_get_l2_request()
            .times(1)
            .with(always(), eq(1u128), always())
            .return_once(move |_,_,_| Ok(Some(withdrawal.into())));

        l1mock.expect_get_status().with(eq(withdrawal.withdrawal_hash())).return_once(|_| Ok(l1api::types::RequestStatus::Ferried([1u8; 20])));
        l1mock.expect_get_status().with(eq(withdrawal2.withdrawal_hash())).return_once(|_| Ok(l1api::types::RequestStatus::Pending));

        l2mock.expect_get_l2_request()
            .times(1)
            .with(always(), eq(2u128), always())
            .return_once(move |_,_,_| Ok(Some(withdrawal2.into())));

        l1mock.expect_ferry_withdrawal().with(eq(withdrawal)).times(0).return_once(move |_| Ok(H256::default()));
        l1mock.expect_ferry_withdrawal().with(eq(withdrawal2)).times(1).return_once(move |_| Ok(H256::default()));

        let handle = tokio::spawn(async move {
            FerryWithdrawal::new(gasp_types::Chain::Ethereum, l1mock, l2mock, vec![withdrawal.token_address]).run().await.unwrap();
        });

        handle.await.unwrap();
    }


    #[tokio::test]
    #[traced_test]
    async fn test_ferry_closes_his_withdrawals() {
        let mut l1mock = l1api::mock::MockL1::default();
        let mut l2mock = l2api::mock::MockL2::default();

        let withdrawal = Withdrawal {
            request_id: RequestId { origin: gasp_types::Origin::L2, id: 1u128.into() },
            recipient: [0u8; 20],
            token_address: [0u8; 20],
            amount: 100u128.into(),
            ferry_tip: 2u128.into(),
        };

        l2mock.expect_header_stream().return_once(move |_| Ok(Box::pin(stream::iter( vec![
            Ok((1u32, [0u8; 32].into())), 
            Ok((2u32, [1u8; 32].into())), 
        ]))));

        l1mock.expect_get_latest_finalized_request_id().returning(|| Ok(None));
        l2mock.expect_get_latest_created_request_id().returning(|_,_| Ok(Some(2u128)));

        l2mock.expect_get_l2_request()
            .times(1)
            .with(always(), eq(1u128), always())
            .return_once(move |_,_,_| Ok(Some(withdrawal.into())));
        l1mock.expect_get_status().with(eq(withdrawal.withdrawal_hash())).return_once(|_| Ok(l1api::types::RequestStatus::Ferried([1u8; 20])));

        l1mock.expect_ferry_withdrawal().with(eq(withdrawal)).times(0).return_once(move |_| Ok(H256::default()));
        l1mock.expect_close_withdrawal().with(eq(withdrawal), always(), always()).times(0).return_once(move |_,_,_| Ok(H256::default()));

        let handle = tokio::spawn(async move {
            FerryWithdrawal::new(gasp_types::Chain::Ethereum, l1mock, l2mock, vec![withdrawal.token_address]).run().await.unwrap();
        });


        handle.await.unwrap();
    }

}


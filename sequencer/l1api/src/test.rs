use super::*;
use crate::utils::test_utils::DevToken;
use crate::{create_provider, types::RequestStatus};
use futures::StreamExt;
use gasp_types::{Origin, RequestId, U256};
use hex_literal::hex;
use serial_test::serial;
use tracing_test::traced_test;

const URI: &str = "http://localhost:8545";
const WS_URI: &str = "ws://localhost:8545";
const ALICE_PKEY: [u8; 32] =
    hex!("dbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97");
const ALICE_ADDRESS: [u8; 20] = hex!("23618e81E3f5cdF7f54C3d65f7FBc0aBf5B21E8f");

#[serial]
#[tokio::test]
async fn test_can_deploy() {
    let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
    let rolldown = RolldownContract::deploy(provider).await.unwrap();

    assert!(rolldown.is_admin(ALICE_ADDRESS).await.unwrap());
    assert!(rolldown.is_updater(ALICE_ADDRESS).await.unwrap());
}

#[serial]
#[tokio::test]
async fn test_get_request_status_pending() {
    let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
    let rolldown = RolldownContract::deploy(provider.clone()).await.unwrap();
    let l1 = L1::new(rolldown.clone(), provider);

    assert_eq!(
        l1.get_status(
            hex!("1111111111111111111111111111111111111111111111111111111111111111").into()
        )
        .await
        .unwrap(),
        RequestStatus::Pending
    );
}

#[serial]
#[traced_test]
#[tokio::test]
async fn test_withdrawal_hash() {
    let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
    let rolldown = RolldownContract::deploy(provider.clone()).await.unwrap();

    let withdrawal = gasp_types::Withdrawal {
        request_id: RequestId {
            id: 123u128.into(),
            origin: Origin::L2,
        },
        recipient: hex!("ffffffffffffffffffffffffffffffffffffffff"),
        token_address: hex!("1f1f1f1f1f1f1f1f1f1f1f1f1f1f1f1f1f1f1f1f"),
        amount: 123456u128.into(),
        ferry_tip: 465789u128.into(),
    };

    let withdrawal_hash = withdrawal.withdrawal_hash();
    let contract_calculated_hash = rolldown.hash(withdrawal).await.unwrap();

    assert_eq!(withdrawal_hash, contract_calculated_hash);
}

#[serial]
#[traced_test]
#[tokio::test]
async fn test_ferry_withdrawal() {
    let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
    let rolldown = RolldownContract::deploy(provider.clone()).await.unwrap();
    let dev_token = DevToken::deploy(provider.clone()).await.unwrap();
    let l1 = L1::new(rolldown.clone(), provider);

    let withdrawal = gasp_types::Withdrawal {
        request_id: RequestId {
            id: 1u128.into(),
            origin: Origin::L2,
        },
        recipient: hex!("1111111111111111111111111111111111111111"),
        token_address: dev_token.address(),
        amount: 100u128.into(),
        ferry_tip: 1u128.into(),
    };
    let withdrawal_hash = withdrawal.withdrawal_hash();

    dev_token.mint(ALICE_ADDRESS, 100u128).await.unwrap();
    dev_token
        .approve(rolldown.address(), 100u128)
        .await
        .unwrap();

    l1.ferry_withdrawal(withdrawal).await.unwrap();

    assert_eq!(
        l1.get_status(withdrawal_hash).await.unwrap(),
        RequestStatus::Ferried(ALICE_ADDRESS)
    );
}

#[serial]
#[tokio::test]
async fn test_close_withdrawal() {
    let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
    let rolldown = RolldownContract::deploy(provider.clone()).await.unwrap();
    let dev_token = DevToken::deploy(provider.clone()).await.unwrap();
    let l1 = L1::new(rolldown.clone(), provider);

    let withdrawal = gasp_types::Withdrawal {
        request_id: RequestId {
            id: 1u128.into(),
            origin: Origin::L2,
        },
        recipient: hex!("1111111111111111111111111111111111111111"),
        token_address: dev_token.address(),
        amount: 100u128.into(),
        ferry_tip: 1u128.into(),
    };

    dev_token.mint(rolldown.address(), 100u128).await.unwrap();
    // dev_token.approve(rolldown.address(), 100u128).await.unwrap();

    assert_eq!(
        l1.get_status(withdrawal.withdrawal_hash()).await.unwrap(),
        RequestStatus::Pending
    );

    rolldown
        .submit_merkle_root(
            withdrawal.withdrawal_hash().into(),
            (
                withdrawal.request_id.id.try_into().unwrap(),
                withdrawal.request_id.id.try_into().unwrap(),
            ),
        )
        .await
        .unwrap();

    l1.close_withdrawal(withdrawal, withdrawal.withdrawal_hash(), vec![])
        .await
        .unwrap();

    assert_eq!(
        l1.get_status(withdrawal.withdrawal_hash()).await.unwrap(),
        RequestStatus::Closed
    );
}

#[serial]
#[tokio::test]
async fn test_close_cancel() {
    let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
    let rolldown = RolldownContract::deploy(provider.clone()).await.unwrap();
    let l1 = L1::new(rolldown.clone(), provider);

    rolldown.deposit_native(1_000u128, 1u128).await.unwrap();

    let cancel = gasp_types::Cancel {
        request_id: RequestId {
            id: 1u128.into(),
            origin: Origin::L2,
        },
        range: (U256::from(1u128), U256::from(1u128)),
        hash: hex!("1111111111111111111111111111111111111111111111111111111111111111"),
    };

    rolldown
        .submit_merkle_root(
            cancel.cancel_hash().into(),
            (
                cancel.request_id.id.try_into().unwrap(),
                cancel.request_id.id.try_into().unwrap(),
            ),
        )
        .await
        .unwrap();

    l1.close_cancel(cancel, cancel.cancel_hash(), vec![])
        .await
        .unwrap();

    assert_eq!(
        l1.get_status(cancel.cancel_hash()).await.unwrap(),
        RequestStatus::Closed
    );
}

#[serial]
#[traced_test]
#[tokio::test]
async fn test_get_latest_request_id() {
    let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
    let rolldown = RolldownContract::deploy(provider.clone()).await.unwrap();

    let l1 = L1::new(rolldown.clone(), provider);
    assert_eq!(l1.get_latest_reqeust_id().await.unwrap(), None);

    rolldown.deposit_native(1_000u128, 1u128).await.unwrap();
    assert_eq!(l1.get_latest_reqeust_id().await.unwrap(), Some(1));

    rolldown.deposit_native(1_000u128, 1u128).await.unwrap();
    assert_eq!(l1.get_latest_reqeust_id().await.unwrap(), Some(2));
}

#[serial]
#[traced_test]
#[tokio::test]
async fn test_get_update_and_update_hash() {
    let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
    let rolldown = RolldownContract::deploy(provider.clone()).await.unwrap();
    let l1 = L1::new(rolldown.clone(), provider);
    assert!(matches!(
        l1.get_update(1u128, 1u128).await,
        Err(L1Error::InvalidRange)
    ));
    assert!(matches!(
        l1.get_update_hash(1u128, 1u128).await,
        Err(L1Error::InvalidRange)
    ));

    rolldown.deposit_native(1_000u128, 1u128).await.unwrap();
    l1.get_update(1u128, 1u128).await.unwrap();
    l1.get_update_hash(1u128, 1u128).await.unwrap();
}

const DUMMY_MERKLE_ROOT: [u8; 32] =
    hex!("1111111111111111111111111111111122222222222222222222222222222222");
const DUMMY_MERKLE_RANGE: (u128, u128) = (1u128, 170u128);

#[serial]
#[traced_test]
#[tokio::test]
async fn test_get_merkle_root() {
    let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
    let rolldown = RolldownContract::deploy(provider.clone()).await.unwrap();
    let l1 = L1::new(rolldown.clone(), provider);

    l1.get_merkle_root(DUMMY_MERKLE_RANGE.0).await.unwrap();
    assert!(l1
        .get_merkle_root(DUMMY_MERKLE_RANGE.0)
        .await
        .unwrap()
        .is_none());
    rolldown
        .submit_merkle_root(DUMMY_MERKLE_ROOT, DUMMY_MERKLE_RANGE)
        .await
        .unwrap();
    assert_eq!(
        (DUMMY_MERKLE_ROOT, DUMMY_MERKLE_RANGE),
        l1.get_merkle_root(DUMMY_MERKLE_RANGE.0)
            .await
            .unwrap()
            .unwrap()
    );
    assert_eq!(
        (DUMMY_MERKLE_ROOT, DUMMY_MERKLE_RANGE),
        l1.get_merkle_root(DUMMY_MERKLE_RANGE.1)
            .await
            .unwrap()
            .unwrap()
    );
}

#[serial]
#[traced_test]
#[tokio::test]
async fn test_get_latest_finalized_request_id() {
    let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
    let rolldown = RolldownContract::deploy(provider.clone()).await.unwrap();
    let l1 = L1::new(rolldown.clone(), provider);

    assert_eq!(l1.get_latest_finalized_request_id().await.unwrap(), None);

    rolldown
        .submit_merkle_root(DUMMY_MERKLE_ROOT, DUMMY_MERKLE_RANGE)
        .await
        .unwrap();

    assert_eq!(
        l1.get_latest_finalized_request_id().await.unwrap(),
        Some(DUMMY_MERKLE_RANGE.1)
    );
}

#[serial]
#[traced_test]
#[tokio::test]
async fn test_get_native_balance() {
    let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
    let rolldown = RolldownContract::deploy(provider.clone()).await.unwrap();
    let l1 = L1::new(rolldown.clone(), provider);

    assert!(l1.native_balance(ALICE_ADDRESS).await.unwrap() > 0u128);
}

#[serial]
#[traced_test]
#[tokio::test]
async fn test_get_erc20_balance() {
    let dummy_address = hex!("3333333333333333333333333333333333333333");
    let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
    let dev_token = DevToken::deploy(provider.clone()).await.unwrap();
    let rolldown = RolldownContract::deploy(provider.clone()).await.unwrap();
    let l1 = L1::new(rolldown.clone(), provider);

    assert_eq!(
        l1.erc20_balance(dev_token.address(), dummy_address)
            .await
            .unwrap(),
        0u128
    );
    dev_token.mint(dummy_address, 100u128).await.unwrap();
    assert_eq!(
        l1.erc20_balance(dev_token.address(), dummy_address)
            .await
            .unwrap(),
        100u128
    );
}

#[serial]
#[traced_test]
#[tokio::test]
async fn test_subscribe() {
    let provider = create_provider(WS_URI, ALICE_PKEY).await.unwrap();
    let rolldown = RolldownContract::deploy(provider.clone()).await.unwrap();

    let rolldown_handle = rolldown.clone();
    let dummy_hash = hex!("0000000000000000000000000000000000000000000000000000000000000000");

    let x = tokio::spawn(async move {
        let mut id = 0u128;
        loop {
            id+=1;
            tracing::info!("sending merkle root");
            tokio::time::sleep(tokio::time::Duration::from_secs_f64(1.0)).await;
            rolldown_handle
                .submit_merkle_root(
                    dummy_hash,
                    (
                        id,
                        id,
                    ),
                )
                .await
                .unwrap();
        }
    });

    let mut subscription = rolldown.subscribe_new_batch().await.unwrap().take(3);
    assert_eq!(subscription.next().await, Some((dummy_hash.into() ,(1u128, 1u128))));
    assert_eq!(subscription.next().await, Some((dummy_hash.into() ,(2u128, 2u128))));
    assert_eq!(subscription.next().await, Some((dummy_hash.into() ,(3u128, 3u128))));
    assert_eq!(subscription.next().await, None);

}

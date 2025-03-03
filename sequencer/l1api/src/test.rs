use super::*;
use crate::create_provider;
use crate::utils::test_utils::DevToken;
use hex_literal::hex;
use serial_test::serial;
use tracing_test::traced_test;
use warp::filters::sse::keep_alive;

const URI: &str = "http://localhost:8545";
const ROLLDOWN_ADDRESS: [u8; 20] = hex!("1429859428C0aBc9C2C47C8Ee9FBaf82cFA0F20f");
const ALICE_PKEY: [u8; 32] =
    hex!("dbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97");
const ALICE_ADDRESS: [u8; 20] = hex!("23618e81E3f5cdF7f54C3d65f7FBc0aBf5B21E8f");
const ANVIL_TEST_ACCOUNT: [u8; 32] =
    hex!("7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6");

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

    let withdrawal = L1Withdrawal {
        request_id: 123u128,
        recipient: hex!("ffffffffffffffffffffffffffffffffffffffff"),
        token_address: hex!("1f1f1f1f1f1f1f1f1f1f1f1f1f1f1f1f1f1f1f1f"),
        amount: 123456u128,
        ferry_tip: 465789u128,
    };

    let withdrawal_hash = withdrawal.hash();
    let contract_calculated_hash = rolldown.hash(withdrawal.clone()).await.unwrap();

    assert_eq!(
        withdrawal_hash, 
        contract_calculated_hash
    );
}

#[serial]
#[traced_test]
#[tokio::test]
async fn test_ferry_withdrawal() {
    let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
    let rolldown = RolldownContract::deploy(provider.clone()).await.unwrap();
    let dev_token = DevToken::deploy(provider.clone()).await.unwrap();
    let l1 = L1::new(rolldown.clone(), provider);

    let withdrawal = L1Withdrawal {
        request_id: 1u128,
        recipient: hex!("1111111111111111111111111111111111111111"),
        token_address: dev_token.address(),
        amount: 100u128,
        ferry_tip: 1u128,
    };
    let withdrawal_hash = withdrawal.hash();

    dev_token.mint(ALICE_ADDRESS, 100u128).await.unwrap();
    dev_token.approve(rolldown.address(), 100u128).await.unwrap();

    l1.ferry_withdrawal(withdrawal).await.unwrap();

    assert_eq!(
        l1.get_status(
            withdrawal_hash.into()
        )
        .await
        .unwrap(),
        RequestStatus::Ferried(ALICE_ADDRESS)
    );

    // l1.close_withdrawal(withdrawal, merkle_root, proof)
}

#[serial]
#[tokio::test]
async fn test_close_withdrawal() {
    let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
    let rolldown = RolldownContract::deploy(provider.clone()).await.unwrap();
    let dev_token = DevToken::deploy(provider.clone()).await.unwrap();
    let l1 = L1::new(rolldown.clone(), provider);

    let withdrawal = L1Withdrawal {
        request_id: 1u128,
        recipient: hex!("1111111111111111111111111111111111111111"),
        token_address: dev_token.address(),
        amount: 100u128,
        ferry_tip: 1u128,
    };

    dev_token.mint(rolldown.address(), 100u128).await.unwrap();
    // dev_token.approve(rolldown.address(), 100u128).await.unwrap();

    assert_eq!(
        l1.get_status(
            withdrawal.hash()
        )
        .await
        .unwrap(),
        RequestStatus::Pending
    );

    rolldown
        .submit_merkle_root(withdrawal.hash().into(), (withdrawal.request_id, withdrawal.request_id))
        .await
        .unwrap();

    l1.close_withdrawal(withdrawal, withdrawal.hash(), vec![]).await.unwrap();


    assert_eq!(
        l1.get_status(
            withdrawal.hash()
        )
        .await
        .unwrap(),
        RequestStatus::Closed
    );

}

#[serial]
#[tokio::test]
async fn test_get_request_status_closed() {
    let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
    let rolldown = RolldownContract::deploy(provider.clone()).await.unwrap();
    let l1 = L1::new(rolldown.clone(), provider);

    assert_eq!(
        l1.get_status(
            hex!("1111111111111111111111111111111111111111111111111111111111111111").into()
        )
        .await
        .unwrap(),
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
    assert!(matches!(
        l1.get_merkle_root(DUMMY_MERKLE_RANGE.0).await.unwrap(),
        None
    ));
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
async fn test_foo() {
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

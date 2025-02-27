
use super::*;
use hex_literal::hex;
use serial_test::serial;
use warp::filters::sse::keep_alive;
use tracing_test::traced_test;
use crate::create_provider;

const URI: &str = "http://localhost:8545";
const ROLLDOWN_ADDRESS: [u8; 20] = hex!("1429859428C0aBc9C2C47C8Ee9FBaf82cFA0F20f");
const ALICE_PKEY: [u8; 32] =
hex!("dbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97");
const ANVIL_TEST_ACCOUNT: [u8; 32] = hex!("7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6");


#[serial]
#[tokio::test]
async fn test_can_deploy() {
    let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
    RolldownContract::deploy(provider).await.unwrap();
}

#[serial]
#[tokio::test]
async fn test_is_request_closed() {
    let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
    let rolldown = RolldownContract::deploy(provider).await.unwrap();
    assert_eq!(
        rolldown.is_request_closed(hex!("1111111111111111111111111111111111111111111111111111111111111111").into()).await.unwrap(),
        false
    );
}

#[serial]
#[traced_test]
#[tokio::test]
async fn test_get_latest_request_id() {
    let provider = create_provider(URI, ANVIL_TEST_ACCOUNT).await.unwrap();
    let rolldown = RolldownContract::deploy(provider).await.unwrap();

    let l1 = L1::new(rolldown.clone());
    assert_eq!(l1.get_latest_reqeust_id().await.unwrap(), None);

    rolldown.deposit_native(1_000u128, 1u128).await.unwrap();
    assert_eq!(l1.get_latest_reqeust_id().await.unwrap(), Some(1));

    rolldown.deposit_native(1_000u128, 1u128).await.unwrap();
    assert_eq!(l1.get_latest_reqeust_id().await.unwrap(), Some(2));
}

// #[serial]
// #[tokio::test]
// async fn test_get_update() {
//     let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
//     let rolldown = RolldownContract::deploy(provider).await.unwrap();
//     assert_eq!(
//         rolldown.get_update(hex!("1111111111111111111111111111111111111111111111111111111111111111").into()).await.unwrap(),
//         false
//     );
// }


// #[serial]
// #[tokio::test]
// async fn test_can_latest_request_id() {
//     let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
//     let rolldown = RolldownContract::from_provider(ROLLDOWN_ADDRESS, provider);
//     rolldown.deposit(1000, 10).await.unwrap();
//     rolldown
//         .get_latest_reqeust_id()
//         .await
//         .expect("can fetch request");
// }
//
// #[serial]
// #[tokio::test]
// async fn test_can_fetch_balance() {
//     let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
//     let rolldown = RolldownContract::from_provider(ROLLDOWN_ADDRESS, provider);
//
//     let balance = rolldown
//         .get_native_balance(hex!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266"))
//         .await
//         .unwrap();
//     assert!(balance > 0u128);
// }
//
// #[serial]
// #[tokio::test]
// async fn test_builder() {
//     let foo = FooBuilder {
//         uri: URI,
//         pkey: ALICE_PKEY,
//         address: ROLLDOWN_ADDRESS,
//     }.build().await.unwrap();
//
//     let d = foo.deposit(100u128, 1u128).await.unwrap();
//
// }

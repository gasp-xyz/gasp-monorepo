
use super::*;
use hex_literal::hex;
use serial_test::serial;
use crate::create_provider;

const URI: &str = "http://localhost:8545";
const ROLLDOWN_ADDRESS: [u8; 20] = hex!("1429859428C0aBc9C2C47C8Ee9FBaf82cFA0F20f");
const ALICE_PKEY: [u8; 32] =
hex!("dbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97");


#[serial]
#[tokio::test]
async fn test_can_deploy() {
    let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
    RolldownContract::deploy(provider).await.unwrap();
}

#[serial]
#[tokio::test]
async fn test_can_deploy() {
    let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
    RolldownContract::deploy(provider).await.unwrap();
}


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

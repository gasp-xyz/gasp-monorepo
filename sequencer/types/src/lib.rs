#![allow(clippy::from_over_into)]
use core::fmt;
use std::fmt::{Display, Formatter};

use alloy::sol_types::SolValue;
use gasp_bindings::api::runtime_types::sp_runtime::account::AccountId20;
use hex_literal::hex;
use parity_scale_codec::{Decode, Encode};
pub use primitive_types::{H256, U256};
use sha3::{Digest, Keccak256};

// since its only used in l2api lets just use it instead of defining own types
pub type PendingUpdateMetadata =
    gasp_bindings::api::runtime_types::pallet_rolldown::pallet::UpdateMetadata<AccountId20>;
pub use gasp_bindings::api::runtime_types::pallet_rolldown::messages::L1Update;

#[derive(Debug)]
pub struct PendingUpdate {
    pub chain: Chain,
    pub update_id: u128,
    pub range: (u128, u128),
    pub hash: H256,
}

mod l2types {
    pub use gasp_bindings::api::runtime_types::sp_runtime::account::AccountId20;
    // pub use gasp_bindings::api::runtime_types::pallet_rolldown::messages::Cancel;
    pub use gasp_bindings::api::runtime_types::pallet_rolldown::messages::Chain;
    pub use gasp_bindings::api::runtime_types::pallet_rolldown::messages::Origin;
    pub use gasp_bindings::api::runtime_types::pallet_rolldown::messages::RequestId;
    pub use gasp_bindings::api::runtime_types::pallet_rolldown::messages::Withdrawal;
    pub use gasp_bindings::api::runtime_types::primitive_types::U256;
    pub type Cancel =
        gasp_bindings::api::runtime_types::pallet_rolldown::messages::Cancel<AccountId20>;
}

mod l1types {
    pub use alloy::primitives::U256;
    pub use contract_bindings::rolldown::IRolldownPrimitives::Cancel;
    pub use contract_bindings::rolldown::IRolldownPrimitives::ChainId as Chain;
    pub use contract_bindings::rolldown::IRolldownPrimitives::Deposit;
    pub use contract_bindings::rolldown::IRolldownPrimitives::Origin;
    pub use contract_bindings::rolldown::IRolldownPrimitives::Range;
    pub use contract_bindings::rolldown::IRolldownPrimitives::RequestId;
    pub use contract_bindings::rolldown::IRolldownPrimitives::Withdrawal;
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Origin {
    L1,
    L2,
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub enum Chain {
    Ethereum,
    Arbitrum,
    Base,
    Monad,
    MegaEth,
    Sonic,
}

#[derive(thiserror::Error, Debug)]
pub enum ChainParseError {
    #[error("Unsupported chain id {0}")]
    UnsupportedChainId(u32),
}

impl TryFrom<u32> for Chain {
    type Error = ChainParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 | 17000 | 31337 => Ok(Chain::Ethereum),
            42161 | 421614 | 31338 => Ok(Chain::Arbitrum),
            8453 | 84532 | 31339 => Ok(Chain::Arbitrum),
            146 => Ok(Chain::Sonic),
            10143 => Ok(Chain::Monad),
            6342 => Ok(Chain::MegaEth),
            _ => Err(ChainParseError::UnsupportedChainId(value)),
        }
    }
}

impl From<l2types::Chain> for Chain {
    fn from(value: l2types::Chain) -> Self {
        match value {
            l2types::Chain::Ethereum => Chain::Ethereum,
            l2types::Chain::Arbitrum => Chain::Arbitrum,
            l2types::Chain::Base => Chain::Base,
            l2types::Chain::Monad => Chain::Monad,
            l2types::Chain::MegaEth => Chain::MegaEth,
            l2types::Chain::Sonic => Chain::Sonic,
        }
    }
}

const ETHEREUM_CHAIN_ID: u8 = 0;
const ARBITRUM_CHAIN_ID: u8 = 1;
const BASE_CHAIN_ID: u8 = 2;
const MONAD_CHAIN_ID: u8 = 3;
const MEGAETH_CHAIN_ID: u8 = 4;
const SONIC_CHAIN_ID: u8 = 5;

impl From<l1types::Chain> for Chain {
    fn from(value: l1types::Chain) -> Self {
        match value.into() {
            ETHEREUM_CHAIN_ID => Chain::Ethereum,
            ARBITRUM_CHAIN_ID => Chain::Arbitrum,
            BASE_CHAIN_ID => Chain::Base,
            MONAD_CHAIN_ID => Chain::Monad,
            MEGAETH_CHAIN_ID => Chain::MegaEth,
            SONIC_CHAIN_ID => Chain::Sonic,
            _ => panic!("unknown chain"),
        }
    }
}

impl Into<l1types::Chain> for Chain {
    fn into(self) -> l1types::Chain {
        match self {
            Chain::Arbitrum => l1types::Chain::from(ARBITRUM_CHAIN_ID),
            Chain::Ethereum => l1types::Chain::from(ETHEREUM_CHAIN_ID),
            Chain::Base => l1types::Chain::from(BASE_CHAIN_ID),
            Chain::Monad => l1types::Chain::from(MONAD_CHAIN_ID),
            Chain::MegaEth => l1types::Chain::from(MEGAETH_CHAIN_ID),
            Chain::Sonic => l1types::Chain::from(SONIC_CHAIN_ID),
        }
    }
}

impl Into<l2types::Chain> for Chain {
    fn into(self) -> l2types::Chain {
        match self {
            Chain::Ethereum => l2types::Chain::Ethereum,
            Chain::Arbitrum => l2types::Chain::Arbitrum,
            Chain::Base => l2types::Chain::Base,
            Chain::Monad => l2types::Chain::Monad,
            Chain::MegaEth => l2types::Chain::MegaEth,
            Chain::Sonic => l2types::Chain::Sonic,
        }
    }
}

impl From<l2types::Origin> for Origin {
    fn from(origin: l2types::Origin) -> Self {
        match origin {
            l2types::Origin::L1 => Origin::L1,
            l2types::Origin::L2 => Origin::L2,
        }
    }
}

impl Into<l2types::Origin> for Origin {
    fn into(self) -> l2types::Origin {
        match self {
            Origin::L1 => l2types::Origin::L1,
            Origin::L2 => l2types::Origin::L2,
        }
    }
}

impl From<l1types::Origin> for Origin {
    fn from(origin: l1types::Origin) -> Self {
        match origin.into() {
            0u8 => Origin::L1,
            1u8 => Origin::L2,
            _ => panic!("unknown origin"),
        }
    }
}

impl Into<l1types::Origin> for Origin {
    fn into(self) -> l1types::Origin {
        match self {
            Origin::L1 => l1types::Origin::from(0u8),
            Origin::L2 => l1types::Origin::from(1u8),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct RequestId {
    pub origin: Origin,
    pub id: U256,
}

impl From<l2types::RequestId> for RequestId {
    fn from(value: l2types::RequestId) -> Self {
        RequestId {
            origin: value.origin.into(),
            id: U256::from(value.id),
        }
    }
}

impl From<l1types::RequestId> for RequestId {
    fn from(value: l1types::RequestId) -> Self {
        let origin: l1types::Origin = l1types::Origin::from(value.origin);

        RequestId {
            origin: Origin::from(origin),
            id: from_l1_u256(value.id),
        }
    }
}

impl Into<l2types::RequestId> for RequestId {
    fn into(self) -> l2types::RequestId {
        l2types::RequestId {
            origin: self.origin.into(),
            id: self.id.try_into().unwrap(),
        }
    }
}

impl Into<l1types::RequestId> for RequestId {
    fn into(self) -> l1types::RequestId {
        let origin: l1types::Origin = self.origin.into();
        l1types::RequestId {
            origin: origin.into(),
            id: into_l1_u256(self.id),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Cancel {
    pub request_id: RequestId,
    pub range: (U256, U256),
    pub hash: [u8; 32],
}

impl Cancel {
    pub fn cancel_hash(&self) -> H256 {
        let encoded = Into::<l1types::Cancel>::into(*self).abi_encode();
        let prefix = hex!("0000000000000000000000000000000000000000000000000000000000000001");
        let data = prefix.into_iter().chain(encoded);
        let hash: [u8; 32] = Keccak256::digest(&data.collect::<Vec<_>>()).into();
        hash.into()
    }
}

#[derive(PartialEq, Copy, Clone)]
pub struct Withdrawal {
    pub request_id: RequestId,
    pub recipient: [u8; 20],
    pub token_address: [u8; 20],
    pub amount: U256,
    pub ferry_tip: U256,
}

impl std::fmt::Debug for Withdrawal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Withdrawal{{request_id: {:?}, recipient: {}, token_address: {}, amount: {:?}, ferry_tip: {:?}}}",
            self.request_id, hex::encode(self.recipient), hex::encode(self.token_address), self.amount, self.ferry_tip
        )
    }
}

impl Display for Withdrawal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Withdrawal: request_id: {:?}, recipient: {}, token_address: {}, amount: {:?}, ferry_tip: {:?}",
            self.request_id, hex::encode(self.recipient), hex::encode(self.token_address), self.amount, self.ferry_tip
        )
    }
}

#[derive(PartialEq, Copy, Clone)]
pub struct Deposit {
    pub request_id: RequestId,
    pub recipient: [u8; 20],
    pub token_address: [u8; 20],
    pub amount: U256,
    pub timestamp: U256,
    pub ferry_tip: U256,
}

impl Into<l1types::Deposit> for Deposit {
    fn into(self) -> l1types::Deposit {
        l1types::Deposit {
            requestId: self.request_id.into(),
            depositRecipient: self.recipient.into(),
            tokenAddress: self.token_address.into(),
            timeStamp: into_l1_u256(self.timestamp),
            amount: into_l1_u256(self.amount),
            ferryTip: into_l1_u256(self.ferry_tip),
        }
    }
}

impl From<l1types::Deposit> for Deposit {
    fn from(deposit: l1types::Deposit) -> Self {
        Self {
            request_id: deposit.requestId.into(),
            recipient: deposit.depositRecipient.into(),
            token_address: deposit.tokenAddress.into(),
            timestamp: from_l1_u256(deposit.timeStamp),
            amount: from_l1_u256(deposit.amount),
            ferry_tip: from_l1_u256(deposit.ferryTip),
        }
    }
}

impl Deposit {
    pub fn deposit_hash(&self) -> H256 {
        let encoded = Into::<l1types::Deposit>::into(*self).abi_encode();
        let hash: [u8; 32] = Keccak256::digest(encoded.as_ref()).into();
        hash.into()
    }
}

impl std::fmt::Debug for Deposit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Deposit: request_id: {:?}, recipient: {}, token_address: {}, amount: {:?}, ferry_tip: {:?}",
            self.request_id, hex::encode(self.recipient), hex::encode(self.token_address), self.amount, self.ferry_tip
        )
    }
}

impl Display for Deposit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Deposit: request_id: {:?}, recipient: {}, token_address: {}, amount: {:?}, ferry_tip: {:?}",
            self.request_id, hex::encode(self.recipient), hex::encode(self.token_address), self.amount, self.ferry_tip
        )
    }
}

impl Withdrawal {
    pub fn withdrawal_hash(&self) -> H256 {
        let encoded = Into::<l1types::Withdrawal>::into(*self).abi_encode();
        let data = [0u8; 32].into_iter().chain(encoded);
        let hash: [u8; 32] = Keccak256::digest(&data.collect::<Vec<_>>()).into();
        hash.into()
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum L2Request {
    Cancel(Cancel),
    Withdrawal(Withdrawal),
}

impl L2Request {
    pub fn request_id(&self) -> U256 {
        match self {
            L2Request::Cancel(cancel) => cancel.request_id.id,
            L2Request::Withdrawal(withdrawal) => withdrawal.request_id.id,
        }
    }
}

impl From<Cancel> for L2Request {
    fn from(cancel: Cancel) -> Self {
        L2Request::Cancel(cancel)
    }
}

impl From<Withdrawal> for L2Request {
    fn from(withdrawal: Withdrawal) -> Self {
        L2Request::Withdrawal(withdrawal)
    }
}

pub fn from_l2_u256(value: l2types::U256) -> U256 {
    let encoded = value.encode();
    U256::from_little_endian(encoded.as_ref())
}

pub fn into_l2_u256(value: U256) -> l2types::U256 {
    let data = value.to_little_endian();
    l2types::U256::decode(&mut &data[..]).unwrap()
}

pub fn from_l1_u256(value: l1types::U256) -> U256 {
    let bytes = value.as_le_slice();
    U256::from_little_endian(bytes)
}

pub fn into_l1_u256(value: U256) -> l1types::U256 {
    let data = value.to_big_endian();
    l1types::U256::from_be_slice(&data)
}

impl From<l2types::Withdrawal> for Withdrawal {
    fn from(value: l2types::Withdrawal) -> Self {
        Withdrawal {
            request_id: value.requestId.into(),
            recipient: value.withdrawalRecipient,
            token_address: value.tokenAddress,
            amount: from_l2_u256(value.amount),
            ferry_tip: from_l2_u256(value.ferryTip),
        }
    }
}

impl From<l1types::Withdrawal> for Withdrawal {
    fn from(value: l1types::Withdrawal) -> Self {
        Withdrawal {
            request_id: value.requestId.into(),
            recipient: value.recipient.0 .0,
            token_address: value.tokenAddress.0 .0,
            amount: from_l1_u256(value.amount),
            ferry_tip: from_l1_u256(value.ferryTip),
        }
    }
}

impl Into<l2types::Withdrawal> for Withdrawal {
    fn into(self) -> l2types::Withdrawal {
        l2types::Withdrawal {
            requestId: self.request_id.into(),
            withdrawalRecipient: self.recipient,
            tokenAddress: self.token_address,
            amount: into_l2_u256(self.amount),
            ferryTip: into_l2_u256(self.ferry_tip),
        }
    }
}

impl Into<l1types::Withdrawal> for Withdrawal {
    fn into(self) -> l1types::Withdrawal {
        l1types::Withdrawal {
            requestId: self.request_id.into(),
            recipient: self.recipient.into(),
            tokenAddress: self.token_address.into(),
            amount: into_l1_u256(self.amount),
            ferryTip: into_l1_u256(self.ferry_tip),
        }
    }
}

impl From<l2types::Cancel> for Cancel {
    fn from(value: l2types::Cancel) -> Self {
        Cancel {
            request_id: value.requestId.into(),
            range: (U256::from(value.range.start), U256::from(value.range.end)),
            hash: value.hash.into(),
        }
    }
}

impl From<l1types::Cancel> for Cancel {
    fn from(value: l1types::Cancel) -> Self {
        Cancel {
            request_id: value.requestId.into(),
            range: (
                from_l1_u256(value.range.start),
                from_l1_u256(value.range.end),
            ),
            hash: value.hash.into(),
        }
    }
}

impl Into<l1types::Cancel> for Cancel {
    fn into(self) -> l1types::Cancel {
        l1types::Cancel {
            requestId: self.request_id.into(),
            range: l1types::Range {
                start: into_l1_u256(self.range.0),
                end: into_l1_u256(self.range.1),
            },
            hash: self.hash.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use alloy::sol_types::SolValue;
    use hex_literal::hex;

    #[test]
    fn test_from_l2_u256() {
        let payload = hex!("ffffff0000000000000000000000000000000000000000000000000000000000");
        let input = l2types::U256::decode(&mut &payload[..]).unwrap();
        assert_eq!(from_l2_u256(input), U256::from(16777215u128))
    }

    #[test]
    fn test_into_l2_u256() {
        let l2_u256 = into_l2_u256(U256::from(16777215u128));
        assert_eq!(
            l2_u256.encode(),
            hex!("ffffff0000000000000000000000000000000000000000000000000000000000").to_vec()
        )
    }

    #[test]
    fn test_from_l1_u256() {
        let input = l1types::U256::from(16777215u128);
        assert_eq!(from_l1_u256(input), U256::from(16777215u128))
    }

    #[test]
    fn test_into_l1_u256() {
        let l1_u256 = into_l1_u256(U256::from(16777215u128));
        assert_eq!(
            l1_u256.abi_encode(),
            hex!("0000000000000000000000000000000000000000000000000000000000ffffff").to_vec()
        )
    }
}

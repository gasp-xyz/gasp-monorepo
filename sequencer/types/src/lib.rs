use parity_scale_codec::{Decode, Encode};
use primitive_types::U256;

mod l2types {
    pub use gasp_bindings::api::runtime_types::sp_runtime::account::AccountId20;
    // pub use gasp_bindings::api::runtime_types::pallet_rolldown::messages::Cancel;
    pub use gasp_bindings::api::runtime_types::pallet_rolldown::messages::Origin;
    pub use gasp_bindings::api::runtime_types::pallet_rolldown::messages::RequestId;
    pub use gasp_bindings::api::runtime_types::pallet_rolldown::messages::Withdrawal;
    pub use gasp_bindings::api::runtime_types::primitive_types::U256;
    pub use gasp_bindings::api::runtime_types::pallet_rolldown::messages::Chain;
    pub type Cancel = gasp_bindings::api::runtime_types::pallet_rolldown::messages::Cancel<AccountId20>;
}

mod l1types {
    pub use alloy::primitives::U256;
    pub use contract_bindings::rolldown::IRolldownPrimitives::ChainId as Chain;
    pub use contract_bindings::rolldown::IRolldownPrimitives::Cancel;
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Chain {
    Ethereum,
    Arbitrum,
    Base,
}

impl From<l2types::Chain> for Chain{
    fn from(value: l2types::Chain) -> Self {
        match value{
            l2types::Chain::Ethereum => Chain::Ethereum,
            l2types::Chain::Arbitrum => Chain::Arbitrum,
            l2types::Chain::Base => Chain::Base,
        }
    }
}


const ETHEREUM_CHAIN_ID: u8 = 0;
const ARBITRUM_CHAIN_ID: u8 = 1;
const BASE_CHAIN_ID: u8 = 2;
impl From<l1types::Chain> for Chain{
    fn from(value: l1types::Chain) -> Self {
        match value.into(){
            ETHEREUM_CHAIN_ID => Chain::Ethereum,
            ARBITRUM_CHAIN_ID => Chain::Arbitrum,
            BASE_CHAIN_ID => Chain::Base,
            _ => panic!("unknown chain"),
        }
    }
}

impl Into<l1types::Chain> for Chain{
    fn into(self) -> l1types::Chain {
        match self{
            Chain::Ethereum => l1types::Chain::from(ETHEREUM_CHAIN_ID),
            Chain::Arbitrum => l1types::Chain::from(ARBITRUM_CHAIN_ID),
            Chain::Base => l1types::Chain::from(BASE_CHAIN_ID),
        }
    }
}

impl Into<l2types::Chain> for Chain{
    fn into(self) -> l2types::Chain {
        match self{
            Chain::Ethereum => l2types::Chain::Ethereum,
            Chain::Arbitrum => l2types::Chain::Arbitrum,
            Chain::Base => l2types::Chain::Base,
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
    origin: Origin,
    request_id: U256,
}

impl From<l2types::RequestId> for RequestId {
    fn from(value: l2types::RequestId) -> Self {
        RequestId {
            origin: value.origin.into(),
            request_id: U256::from(value.id),
        }
    }
}

impl From<l1types::RequestId> for RequestId {
    fn from(value: l1types::RequestId) -> Self {
        let origin: l1types::Origin = l1types::Origin::from(value.origin);

        RequestId {
            origin: Origin::from(origin),
            request_id: from_l1_u256(value.id),
        }
    }
}

impl Into<l2types::RequestId> for RequestId {
    fn into(self) -> l2types::RequestId {
        l2types::RequestId {
            origin: self.origin.into(),
            id: U256::from(self.request_id).try_into().unwrap(),
        }
    }
}

impl Into<l1types::RequestId> for RequestId {
    fn into(self) -> l1types::RequestId {
        let origin: l1types::Origin = self.origin.into();
        l1types::RequestId {
            origin: origin.into(),
            id: into_l1_u256(self.request_id),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Cancel {
    pub request_id: RequestId,
    pub range: (U256, U256),
    pub hash: [u8; 32],
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Withdrawal {
    pub request_id: RequestId,
    pub recipient: [u8; 20],
    pub token_address: [u8; 20],
    pub amount: U256,
    pub ferry_tip: U256,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum L2Request {
    Cancel(Cancel),
    Withdrawal(Withdrawal),
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
            withdrawalRecipient: self.recipient.into(),
            tokenAddress: self.token_address.into(),
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
mod test{
    use super::*;
    use alloy::sol_types::SolValue;
    use hex_literal::hex;

    #[test]
    fn test_from_l2_u256() {
        let payload = hex!("ffffff0000000000000000000000000000000000000000000000000000000000");
        let input = l2types::U256::decode(&mut &payload[..]).unwrap();
        assert_eq!(
            from_l2_u256(input),
            U256::from(16777215u128)
        )
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
        assert_eq!(
            from_l1_u256(input),
            U256::from(16777215u128)
        )
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

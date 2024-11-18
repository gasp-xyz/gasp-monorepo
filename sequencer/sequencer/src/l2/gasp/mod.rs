use std::fmt::{self, Display};

use parity_scale_codec::{Decode, Encode};

mod gasp_bindings;
pub use gasp_bindings::api;

mod header;
pub use header::GaspHeader;

use primitive_types::H256;
use subxt::{
    config::{signed_extensions, substrate::BlakeTwo256},
    Config,
};

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct GaspConfig;

pub type GaspExtrinsicParams<T> = signed_extensions::AnyOf<
    T,
    (
        signed_extensions::CheckSpecVersion,
        signed_extensions::CheckTxVersion,
        signed_extensions::CheckGenesis<T>,
        signed_extensions::CheckMortality<T>,
        signed_extensions::CheckNonce,
        signed_extensions::ChargeTransactionPayment,
    ),
>;

impl Config for GaspConfig {
    type Hash = H256;
    type AccountId = GaspAddress;
    type Address = GaspAddress;
    type Signature = GaspSignature;
    type Hasher = BlakeTwo256;
    type Header = GaspHeader<u32, BlakeTwo256>;
    type ExtrinsicParams = GaspExtrinsicParams<Self>;
    type AssetId = u32;
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct GaspSignature([u8; 65]);

impl From<[u8; 65]> for GaspSignature {
    fn from(data: [u8; 65]) -> Self {
        GaspSignature(data)
    }
}

impl Display for GaspSignature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct GaspPublicKey([u8; 33]);

impl From<[u8; 33]> for GaspPublicKey {
    fn from(data: [u8; 33]) -> Self {
        GaspPublicKey(data)
    }
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, Hash)]
pub struct GaspAddress([u8; 20]);

impl From<[u8; 20]> for GaspAddress {
    fn from(data: [u8; 20]) -> Self {
        GaspAddress(data)
    }
}

impl GaspAddress {
    pub fn into_inner(&self) -> [u8; 20] {
        self.0
    }
}

impl AsRef<[u8]> for GaspAddress {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Display for GaspAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

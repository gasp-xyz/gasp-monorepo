use parity_scale_codec::{Decode, Encode};
use primitive_types::{H256, H512, U256};
use serde::{Deserialize, Serialize};
use subxt::config::{
    substrate::{Digest, NumberOrHex},
    Hasher, Header,
};

fn serialize_number<S, T: Copy + Into<U256>>(val: &T, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let u256: U256 = (*val).into();
    serde::Serialize::serialize(&u256, s)
}

fn deserialize_number<'a, D, T: TryFrom<U256>>(d: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'a>,
{
    // At the time of writing, Smoldot gives back block numbers in numeric rather
    // than hex format. So let's support deserializing from both here:
    let number_or_hex = NumberOrHex::deserialize(d)?;
    let u256 = number_or_hex.into_u256();
    TryFrom::try_from(u256).map_err(|_| serde::de::Error::custom("Try from failed"))
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct ShufflingSeed {
    /// shuffling seed for the previous block
    pub seed: H256,
    /// seed signature
    pub proof: H512,
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GaspHeader<N: Copy + Into<U256> + TryFrom<U256>, H: Hasher> {
    /// The parent hash.
    pub parent_hash: H::Output,
    /// The block number.
    #[serde(
        serialize_with = "serialize_number",
        deserialize_with = "deserialize_number"
    )]
    #[codec(compact)]
    pub number: N,
    /// The state trie merkle root
    pub state_root: H::Output,
    /// The merkle root of the extrinsics.
    pub extrinsics_root: H::Output,
    /// A chain-specific digest of data useful for light clients or referencing auxiliary data.
    pub digest: Digest,

    pub seed: ShufflingSeed,

    pub count: N,
}

impl<N, H> Header for GaspHeader<N, H>
where
    N: Copy + Into<u64> + Into<U256> + TryFrom<U256> + Encode,
    H: Hasher + Encode,
    GaspHeader<N, H>: Encode + Decode,
{
    type Number = N;
    type Hasher = H;
    fn number(&self) -> Self::Number {
        self.number
    }
}

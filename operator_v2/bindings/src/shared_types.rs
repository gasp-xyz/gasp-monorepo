///`G1Point(uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct G1Point {
    pub x: ::ethers::core::types::U256,
    pub y: ::ethers::core::types::U256,
}
///`G2Point(uint256[2],uint256[2])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct G2Point {
    pub x: [::ethers::core::types::U256; 2],
    pub y: [::ethers::core::types::U256; 2],
}
///`NonSignerStakesAndSignature(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct NonSignerStakesAndSignature {
    pub non_signer_quorum_bitmap_indices: ::std::vec::Vec<u32>,
    pub non_signer_pubkeys: ::std::vec::Vec<G1Point>,
    pub quorum_apks: ::std::vec::Vec<G1Point>,
    pub apk_g2: G2Point,
    pub sigma: G1Point,
    pub quorum_apk_indices: ::std::vec::Vec<u32>,
    pub total_stake_indices: ::std::vec::Vec<u32>,
    pub non_signer_stake_indices: ::std::vec::Vec<::std::vec::Vec<u32>>,
}
///`QuorumStakeTotals(uint96[],uint96[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct QuorumStakeTotals {
    pub signed_stake_for_quorum: ::std::vec::Vec<u128>,
    pub total_stake_for_quorum: ::std::vec::Vec<u128>,
}
///`Task(uint256,uint32,bytes,uint32)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Task {
    pub block_number: ::ethers::core::types::U256,
    pub task_created_block: u32,
    pub quorum_numbers: ::ethers::core::types::Bytes,
    pub quorum_threshold_percentage: u32,
}
///`TaskResponse(uint32,bytes32)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct TaskResponse {
    pub reference_task_index: u32,
    pub storage_root: [u8; 32],
}
///`TaskResponseMetadata(uint32,bytes32)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct TaskResponseMetadata {
    pub task_responsed_block: u32,
    pub hash_of_non_signers: [u8; 32],
}

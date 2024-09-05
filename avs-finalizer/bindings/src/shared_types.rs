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
    Hash,
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
    Hash,
)]
pub struct G2Point {
    pub x: [::ethers::core::types::U256; 2],
    pub y: [::ethers::core::types::U256; 2],
}
///`ApkUpdate(bytes24,uint32,uint32)`
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
    Hash,
)]
pub struct ApkUpdate {
    pub apk_hash: [u8; 24],
    pub update_block_number: u32,
    pub next_update_block_number: u32,
}
///`PubkeyRegistrationParams((uint256,uint256),(uint256,uint256),(uint256[2],uint256[2]))`
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
    Hash,
)]
pub struct PubkeyRegistrationParams {
    pub pubkey_registration_signature: G1Point,
    pub pubkey_g1: G1Point,
    pub pubkey_g2: G2Point,
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
    Hash,
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
    Hash,
)]
pub struct QuorumStakeTotals {
    pub signed_stake_for_quorum: ::std::vec::Vec<u128>,
    pub total_stake_for_quorum: ::std::vec::Vec<u128>,
}
///`OperatorDetails(address,address,uint32)`
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
    Hash,
)]
pub struct OperatorDetails {
    pub earnings_receiver: ::ethers::core::types::Address,
    pub delegation_approver: ::ethers::core::types::Address,
    pub staker_opt_out_window_blocks: u32,
}
///`QueuedWithdrawalParams(address[],uint256[],address)`
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
    Hash,
)]
pub struct QueuedWithdrawalParams {
    pub strategies: ::std::vec::Vec<::ethers::core::types::Address>,
    pub shares: ::std::vec::Vec<::ethers::core::types::U256>,
    pub withdrawer: ::ethers::core::types::Address,
}
///`Withdrawal(address,address,address,uint256,uint32,address[],uint256[])`
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
    Hash,
)]
pub struct Withdrawal {
    pub staker: ::ethers::core::types::Address,
    pub delegated_to: ::ethers::core::types::Address,
    pub withdrawer: ::ethers::core::types::Address,
    pub nonce: ::ethers::core::types::U256,
    pub start_block: u32,
    pub strategies: ::std::vec::Vec<::ethers::core::types::Address>,
    pub shares: ::std::vec::Vec<::ethers::core::types::U256>,
}
///`OpTask(uint32,uint32,uint32,bytes,uint32,bytes,uint32)`
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
    Hash,
)]
pub struct OpTask {
    pub task_num: u32,
    pub task_created_block: u32,
    pub last_completed_op_task_created_block: u32,
    pub quorum_numbers: ::ethers::core::types::Bytes,
    pub quorum_threshold_percentage: u32,
    pub last_completed_op_task_quorum_numbers: ::ethers::core::types::Bytes,
    pub last_completed_op_task_quorum_threshold_percentage: u32,
}
///`OpTaskResponse(uint32,bytes32,bytes32)`
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
    Hash,
)]
pub struct OpTaskResponse {
    pub reference_task_index: u32,
    pub reference_task_hash: [u8; 32],
    pub operators_state_info_hash: [u8; 32],
}
///`RdTask(uint32,uint8,uint32,uint32,uint32,bytes,uint32)`
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
    Hash,
)]
pub struct RdTask {
    pub task_num: u32,
    pub chain_id: u8,
    pub batch_id: u32,
    pub task_created_block: u32,
    pub last_completed_op_task_created_block: u32,
    pub last_completed_op_task_quorum_numbers: ::ethers::core::types::Bytes,
    pub last_completed_op_task_quorum_threshold_percentage: u32,
}
///`RdTaskResponse(uint32,bytes32,uint8,uint32,bytes32,uint256,uint256,address)`
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
    Hash,
)]
pub struct RdTaskResponse {
    pub reference_task_index: u32,
    pub reference_task_hash: [u8; 32],
    pub chain_id: u8,
    pub batch_id: u32,
    pub rd_update: [u8; 32],
    pub range_start: ::ethers::core::types::U256,
    pub range_end: ::ethers::core::types::U256,
    pub updater: ::ethers::core::types::Address,
}
///`TaskResponseMetadata(uint32,bytes32,uint96[],uint96[])`
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
    Hash,
)]
pub struct TaskResponseMetadata {
    pub task_responsed_block: u32,
    pub hash_of_non_signers: [u8; 32],
    pub quroum_stake_totals: ::std::vec::Vec<u128>,
    pub quroum_stake_signed: ::std::vec::Vec<u128>,
}
///`OperatorStateInfo(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[])`
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
    Hash,
)]
pub struct OperatorStateInfo {
    pub operators_state_changed: bool,
    pub quorums_removed: ::std::vec::Vec<u8>,
    pub quorums_added: ::std::vec::Vec<QuorumsAdded>,
    pub quorums_stake_update: ::std::vec::Vec<QuorumsStakeUpdate>,
    pub quorums_apk_update: ::std::vec::Vec<QuorumsApkUpdate>,
    pub operators_removed: ::std::vec::Vec<[u8; 32]>,
    pub operators_added: ::std::vec::Vec<OperatorsAdded>,
    pub operators_stake_update: ::std::vec::Vec<OperatorsStakeUpdate>,
    pub operators_quorum_count_update: ::std::vec::Vec<OperatorsQuorumCountUpdate>,
}
///`OperatorsAdded(bytes32,uint8[],uint96[],uint8)`
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
    Hash,
)]
pub struct OperatorsAdded {
    pub operator_id: [u8; 32],
    pub quorum_for_stakes: ::std::vec::Vec<u8>,
    pub quorum_stakes: ::std::vec::Vec<u128>,
    pub quorum_count: u8,
}
///`OperatorsQuorumCountUpdate(bytes32,uint8)`
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
    Hash,
)]
pub struct OperatorsQuorumCountUpdate {
    pub operator_id: [u8; 32],
    pub quorum_count: u8,
}
///`OperatorsStakeUpdate(bytes32,uint8[],uint96[])`
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
    Hash,
)]
pub struct OperatorsStakeUpdate {
    pub operator_id: [u8; 32],
    pub quorum_for_stakes: ::std::vec::Vec<u8>,
    pub quorum_stakes: ::std::vec::Vec<u128>,
}
///`QuorumsAdded(uint8,uint96,(uint256,uint256))`
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
    Hash,
)]
pub struct QuorumsAdded {
    pub quorum_number: u8,
    pub quorum_stake: u128,
    pub quorum_apk: G1Point,
}
///`QuorumsApkUpdate(uint8,(uint256,uint256))`
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
    Hash,
)]
pub struct QuorumsApkUpdate {
    pub quorum_number: u8,
    pub quorum_apk: G1Point,
}
///`QuorumsStakeUpdate(uint8,uint96)`
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
    Hash,
)]
pub struct QuorumsStakeUpdate {
    pub quorum_number: u8,
    pub quorum_stake: u128,
}
///`OperatorInfo(bytes32,uint8)`
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
    Hash,
)]
pub struct OperatorInfo {
    pub operator_id: [u8; 32],
    pub status: u8,
}
///`OperatorSetParam(uint32,uint16,uint16)`
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
    Hash,
)]
pub struct OperatorSetParam {
    pub max_operator_count: u32,
    pub kick_bi_ps_of_operator_stake: u16,
    pub kick_bi_ps_of_total_stake: u16,
}
///`QuorumBitmapUpdate(uint32,uint32,uint192)`
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
    Hash,
)]
pub struct QuorumBitmapUpdate {
    pub update_block_number: u32,
    pub next_update_block_number: u32,
    pub quorum_bitmap: ::ethers::core::types::U256,
}
///`SignatureWithExpiry(bytes,uint256)`
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
    Hash,
)]
pub struct SignatureWithExpiry {
    pub signature: ::ethers::core::types::Bytes,
    pub expiry: ::ethers::core::types::U256,
}
///`SignatureWithSaltAndExpiry(bytes,bytes32,uint256)`
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
    Hash,
)]
pub struct SignatureWithSaltAndExpiry {
    pub signature: ::ethers::core::types::Bytes,
    pub salt: [u8; 32],
    pub expiry: ::ethers::core::types::U256,
}
///`StakeUpdate(uint32,uint32,uint96)`
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
    Hash,
)]
pub struct StakeUpdate {
    pub update_block_number: u32,
    pub next_update_block_number: u32,
    pub stake: u128,
}
///`StrategyParams(address,uint96)`
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
    Hash,
)]
pub struct StrategyParams {
    pub strategy: ::ethers::core::types::Address,
    pub multiplier: u128,
}
///`DeprecatedStructQueuedWithdrawal(address[],uint256[],address,(address,uint96),uint32,address)`
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
    Hash,
)]
pub struct DeprecatedStructQueuedWithdrawal {
    pub strategies: ::std::vec::Vec<::ethers::core::types::Address>,
    pub shares: ::std::vec::Vec<::ethers::core::types::U256>,
    pub staker: ::ethers::core::types::Address,
    pub withdrawer_and_nonce: DeprecatedStructWithdrawerAndNonce,
    pub withdrawal_start_block: u32,
    pub delegated_address: ::ethers::core::types::Address,
}
///`DeprecatedStructWithdrawerAndNonce(address,uint96)`
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
    Hash,
)]
pub struct DeprecatedStructWithdrawerAndNonce {
    pub withdrawer: ::ethers::core::types::Address,
    pub nonce: u128,
}
///`FuzzInterface(address,string[])`
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
    Hash,
)]
pub struct FuzzInterface {
    pub addr: ::ethers::core::types::Address,
    pub artifacts: ::std::vec::Vec<::std::string::String>,
}
///`FuzzSelector(address,bytes4[])`
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
    Hash,
)]
pub struct FuzzSelector {
    pub addr: ::ethers::core::types::Address,
    pub selectors: ::std::vec::Vec<[u8; 4]>,
}

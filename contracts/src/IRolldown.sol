// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.13;

import "./IRolldownPrimitives.sol";

interface IRolldown is IRolldownPrimitives {
    function setUpdater(address updater) external;

    function depositNative() external payable;

    function depositNative(uint256 ferryTip) external payable;

    function deposit(address tokenAddress, uint256 amount) external;

    function deposit(address tokenAddress, uint256 amount, uint256 ferryTip) external;

    function depositERC20(address tokenAddress, uint256 amount) external;

    function depositERC20(address tokenAddress, uint256 amount, uint256 ferryTip) external;

    function ferryWithdrawal(Withdrawal calldata withdrawal) external payable;

    function closeWithdrawal(Withdrawal calldata withdrawal, bytes32 merkle_root, bytes32[] calldata proof) external;

    function closeCancel(Cancel calldata cancel, bytes32 merkle_root, bytes32[] calldata proof) external;

    function closeDepositRefund(
        FailedDepositResolution calldata failedDeposit,
        bytes32 merkleRoot,
        bytes32[] calldata proof
    ) external;

    function updateL1FromL2(bytes32 merkle_root, Range calldata range) external;

    function findL2Batch(uint256 requestId) external view returns (Range memory);

    function getUpdateForL2() external view returns (L1Update memory);

    function getMerkleRootsLength() external view returns (uint256);

    function getPendingRequests(uint256 start, uint256 end) external view returns (L1Update memory);

    function calculateRoot(bytes32 leaveHash, uint32 leaveIdx, bytes32[] calldata proof, uint32 leaveCount) external pure returns (bytes32);

    function calculateRootImpl(
        uint32 level,
        uint32 pos,
        bytes32 hash,
        bytes32[] calldata proofs,
        uint32 proofIdx,
        uint32 maxIdx
    ) external pure returns (bytes32);
}

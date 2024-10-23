// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import {IRolldownPrimitives} from "./IRolldownPrimitives.sol";

interface IRolldown is IRolldownPrimitives {
    function setUpdater(address updater) external;

    function deposit_native() external payable;

    function depositNative() external payable;

    function deposit_native(uint256 ferryTip) external payable;

    function depositNative(uint256 ferryTip) external payable;

    function deposit(address tokenAddress, uint256 amount) external;

    function deposit(address tokenAddress, uint256 amount, uint256 ferryTip) external;

    function deposit_erc20(address tokenAddress, uint256 amount) external;

    function depositERC20(address tokenAddress, uint256 amount) external;

    function deposit_erc20(address tokenAddress, uint256 amount, uint256 ferryTip) external;

    function depositERC20(address tokenAddress, uint256 amount, uint256 ferryTip) external;

    function getUpdateForL2() external view returns (L1Update memory);

    function ferry_withdrawal(Withdrawal calldata withdrawal) external payable;

    function ferryWithdrawal(Withdrawal calldata withdrawal) external payable;

    function update_l1_from_l2(bytes32 merkleRoot, Range calldata range) external;

    function find_l2_batch(uint256 requestId) external view returns (bytes32);

    function findL2Batch(uint256 requestId) external view returns (bytes32);

    function updateL1FromL2(bytes32 merkleRoot, Range calldata range) external;

    function close_cancel(Cancel calldata cancel, bytes32 merkleRoot, bytes32[] calldata proof) external;

    function closeCancel(Cancel calldata cancel, bytes32 merkleRoot, bytes32[] calldata proof) external;

    function close_withdrawal(Withdrawal calldata withdrawal, bytes32 merkleRoot, bytes32[] calldata proof) external;

    function closeWithdrawal(Withdrawal calldata withdrawal, bytes32 merkleRoot, bytes32[] calldata proof) external;

    function close_deposit_refund(
        FailedDepositResolution calldata failedDeposit,
        bytes32 merkleRoot,
        bytes32[] calldata proof
    ) external;

    function closeDepositRefund(
        FailedDepositResolution calldata failedDeposit,
        bytes32 merkleRoot,
        bytes32[] calldata proof
    ) external;

    function getPendingRequests(uint256 start, uint256 end) external view returns (L1Update memory);

    function getMerkleRootsLength() external view returns (uint256);

    function hashWithdrawal(Withdrawal calldata withdrawal) external pure returns (bytes32);

    function hashCancel(Cancel calldata cancel) external pure returns (bytes32);

    function hashFailedDepositResolution(FailedDepositResolution calldata failedDeposit)
        external
        pure
        returns (bytes32);

    function calculateRoot(bytes32 leaveHash, uint32 leaveIdx, bytes32[] calldata proof, uint32 leaveCount)
        external
        pure
        returns (bytes32);

    function calculateRootImpl(
        uint32 level,
        uint32 pos,
        bytes32 hash,
        bytes32[] calldata proofs,
        uint32 proofIdx,
        uint32 maxIdx
    ) external pure returns (bytes32);
}

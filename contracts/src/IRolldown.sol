// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

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

    function getUpdateForL2() external view returns (L1Update memory);

    function getMerkleRootsLength() external view returns (uint256);

    function getPendingRequests(uint256 start, uint256 end) external view returns (L1Update memory);

    function hashWithdrawal(Withdrawal calldata withdrawal) external pure returns (bytes32);

    function hashCancel(Cancel calldata cancel) external pure returns (bytes32);

    function hashFailedDepositResolution(FailedDepositResolution calldata failedDeposit)
        external
        pure
        returns (bytes32);

    function counter() external view returns (uint256);

    function lastProcessedUpdate_origin_l1() external view returns (uint256);

    function lastProcessedUpdate_origin_l2() external view returns (uint256);

    function chain() external view returns (ChainId);

    function updaterAccount() external view returns (address);

    function processedL2Requests(bytes32 requestId) external view returns (address);
}

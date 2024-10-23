// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import {IRolldownPrimitives} from "./IRolldownPrimitives.sol";

interface IRolldown is IRolldownPrimitives {
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

    function update_l1_from_l2(bytes32 merkle_root, Range calldata range) external;

    function updateL1FromL2(bytes32 merkle_root, Range calldata range) external;

    function close_cancel(Cancel calldata cancel, bytes32 merkle_root, bytes32[] calldata proof) external;

    function closeCancel(Cancel calldata cancel, bytes32 merkle_root, bytes32[] calldata proof) external;

    function close_withdrawal(Withdrawal calldata withdrawal, bytes32 merkle_root, bytes32[] calldata proof) external;

    function closeWithdrawal(Withdrawal calldata withdrawal, bytes32 merkle_root, bytes32[] calldata proof) external;

    function getPendingRequests(uint256 start, uint256 end) external view returns (L1Update memory);
}

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "./IRolldownPrimitives.sol";

interface IRolldown is IRolldownPrimitives {

    function withdraw_pending_eth(uint256 amount) external;

    function deposit_native() external payable;

    function deposit(address tokenAddress, uint256 amount) external;

    function deposit_erc20(address tokenAddress, uint256 amount) external;

    function getUpdateForL2() external view returns (L1Update memory) ;

    function update_l1_from_l2(bytes32 merkle_root, Range calldata range ) external;

    function close_cancel(Cancel calldata cancel, bytes32 merkle_root, bytes32[] calldata proof) external;

    function close_withdrawal(Withdrawal calldata withdrawal, bytes32 merkle_root, bytes32[] calldata proof) external;

    function getPendingRequests(
        uint256 start,
        uint256 end
    ) external view returns (L1Update memory);

}

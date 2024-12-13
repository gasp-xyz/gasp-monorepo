// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

interface IGaspToken is IERC20 {
    event UniswapPoolSet(address indexed uniswapPool_);

    event AllowTransfersSet(bool allowTransfers_);

    event AddedToWhitelist(address indexed account);

    event RemovedFromWhitelist(address indexed account);

    error ZeroL1Council();

    error ZeroUniswapPool();

    error ZeroWhitelistAccount();

    error TransfersAlreadyAllowed();

    error AccountAlreadyWhitelisted(address account);

    error AccountNotWhitelisted(address account);

    error OperationForbidden(bytes32 selector);

    function setUniswapPool(address uniswapPool_) external;

    function setAllowTransfers(bool allowTransfers_) external;

    function addToWhitelist(address account) external;

    function removeFromWhitelist(address account) external;

    function increaseAllowance(address spender, uint256 addedAmount) external returns (bool);

    function decreaseAllowance(address spender, uint256 subtractedAmount) external returns (bool);

    function whitelist(address account) external view returns (bool enabled);

    function uniswapPool() external view returns (address);

    function allowTransfers() external view returns (bool);
}

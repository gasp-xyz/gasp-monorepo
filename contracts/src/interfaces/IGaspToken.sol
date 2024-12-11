// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

interface IGaspToken is IERC20 {
    event UniswapPoolSet(address indexed uniswapPool_);

    event AllowTransfersSet(bool allowTransfers_);

    event AddedToWhitelist(address indexed address_);

    event RemovedFromWhitelist(address indexed address_);

    error ZeroL1Council();

    error ZeroUniswapPool();

    error ZeroWhitelistAccount();

    error TransfersAlreadyAllowed();

    error AccountAlreadyWhitelisted(address addr);

    error AccountNotWhitelisted(address addr);

    error OperationForbidden(bytes32 selector);

    function setUniswapPool(address uniswapPool_) external;

    function setAllowTransfers(bool allowTransfers_) external;

    function addToWhitelist(address addr) external;

    function removeFromWhitelist(address addr) external;
}

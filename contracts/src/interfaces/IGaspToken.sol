// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

interface IGaspToken is IERC20 {
    event AllowTransfersSet(bool allowTransfers_);

    event AddedToWhitelist(address indexed addr);

    event RemovedFromWhitelist(address indexed addr);

    error ZeroL1Council();

    error ZeroUniswapPool();

    error ZeroWhitelistAddress();

    error TransfersAlreadyAllowed();

    error AddressAlreadyWhitelisted(address addr);

    error AddressNotWhitelisted(address addr);

    error OperationForbidden(bytes32 selector);

    function setAllowTransfers(bool allowTransfers_) external;

    function addToWhitelist(address addr) external;

    function removeFromWhitelist(address addr) external;
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

interface IGaspToken is IERC20 {
    event AllowTransfersSet(bool allowTransfers_);

    event AddedToWhitelist(address indexed account);

    event RemovedFromWhitelist(address indexed account);

    error ZeroL1Council();

    error ZeroWhitelistAccount();

    error TransfersAlreadyAllowed();

    error AccountAlreadyWhitelisted(address account);

    error AccountNotWhitelisted(address account);

    error OperationForbidden(bytes32 selector);

    function setAllowTransfers(bool allowTransfers_) external;

    function addToWhitelist(address account) external;

    function removeFromWhitelist(address account) external;

    function whitelist(address account) external view returns (bool enabled);

    function allowTransfers() external view returns (bool);
}

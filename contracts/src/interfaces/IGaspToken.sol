// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

interface IGaspToken is IERC20 {
    event AllowTransfersSet(bool allowTransfers_);

    event AddedToSenderWhitelist(address indexed account);

    event RemovedFromSenderWhitelist(address indexed account);

    error ZeroL1Council();

    error ZeroWhitelistAccount();

    error TransfersAlreadyAllowed();

    error AccountAlreadyWhitelisted(address account);

    error AccountNotWhitelisted(address account);

    error OperationForbidden(bytes32 selector);

    function setAllowTransfers(bool allowTransfers_) external;

    function addToSenderWhitelist(address account) external;

    function removeFromSenderWhitelist(address account) external;

    function senderWhitelist(address account) external view returns (bool enabled);

    function recipientWhitelist(address account) external view returns (bool enabled);

    function allowTransfers() external view returns (bool);
}

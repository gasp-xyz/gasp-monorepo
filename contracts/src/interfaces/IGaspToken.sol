// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

interface IGaspToken is IERC20 {
    event AllowTransfersSet(bool allowTransfers_);

    event SenderWhitelisted(address indexed sender);

    event RecipientWhitelisted(address indexed recipient);

    event SenderDewhitelisted(address indexed sender);

    event RecipientDewhitelisted(address indexed recipient);

    error ZeroL1Council();

    error ZeroSender();

    error ZeroRecipient();

    error SenderAlreadyWhitelisted(address sender);

    error RecipientAlreadyWhitelisted(address recipient);

    error SenderNotWhitelisted(address sender);

    error RecipientNotWhitelisted(address recipient);

    error OperationForbidden(bytes32 selector);

    function setAllowTransfers(bool allowTransfers_) external;

    function whitelistSender(address sender) external;

    function whitelistRecipient(address recipient) external;

    function dewhitelistSender(address sender) external;

    function dewhitelistRecipient(address recipient) external;

    function getSenderWhitelist() external view returns (address[] memory);

    function getRecipientWhitelist() external view returns (address[] memory);
}

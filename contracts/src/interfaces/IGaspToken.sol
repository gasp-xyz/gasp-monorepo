// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

interface IGaspToken is IERC20 {
    event AllowTransfersSet(bool allowTransfers_);

    event AddressWhitelisted(address indexed addr);

    event AddressDewhitelisted(address indexed addr);

    error ZeroL1Council();

    error ZeroAddress();

    error TransfersAlreadyAllowed();

    error AddressAlreadyWhitelisted(address addr);

    error AddressNotWhitelisted(address addr);

    error OperationForbidden(bytes32 selector);

    function setAllowTransfers(bool allowTransfers_) external;

    function whitelist(address addr) external;

    function dewhitelist(address addr) external;

    function getWhitelist() external view returns (address[] memory);
}

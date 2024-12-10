// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

interface IGaspToken is IERC20 {
    event AllowTransfersSet(bool allowTransfers_);

    event AddedToWhitelist(address indexed address_);

    event RemoveFromWhitelist(address indexed address_);

    error ZeroL1Council();

    error ZeroAddress();

    error TransfersAlreadyAllowed();

    error AddressAlreadyWhitelisted(address address_);

    error AddressNotWhitelisted(address address_);

    error OperationForbidden(bytes32 selector_);

    function setAllowTransfers(bool allowTransfers_) external;

    function addToWhitelist(address address_) external;

    function removeFromWhitelist(address address_) external;
}

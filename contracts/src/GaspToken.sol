// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {Context} from "@openzeppelin/contracts/utils/Context.sol";
import {IGaspToken} from "./interfaces/IGaspToken.sol";

contract GaspToken is Context, Ownable, ERC20, IGaspToken {
    uint256 private constant _TOTAL_SUPPLY = 1_000_000_000 * 10 ** 18;
    string private constant _NAME = "GASP";
    string private constant _SYMBOL = "GASP";

    mapping (address => bool) public whitelist;
    bool public allowTransfers;

    constructor(address l1Council) Ownable() ERC20(_NAME, _SYMBOL) {
        if (l1Council == address(0)) {
            revert ZeroL1Council();
        }

        _transferOwnership(l1Council);
        _mint(l1Council, _TOTAL_SUPPLY);

        whitelist[l1Council] = true;
    }

    function setAllowTransfers(bool allowTransfers_) external override onlyOwner {
        if (allowTransfers) {
            revert TransfersAlreadyAllowed();
        }

        allowTransfers = allowTransfers_;
        emit AllowTransfersSet(allowTransfers_);
    }

    function addToWhitelist(address address_) external override onlyOwner {
        if (address_ == address(0)) {
            revert ZeroAddress();
        }

        if (whitelist[address_]) {
            revert AddressAlreadyWhitelisted(address_);
        }
        whitelist[address_] = true;

        emit AddedToWhitelist(address_);
    }

    function removeFromWhitelist(address address_) external override onlyOwner {
        if (address_ == address(0)) {
            revert ZeroAddress();
        }

        if (!whitelist[address_]) {
            revert AddressNotWhitelisted(address_);
        }
        delete whitelist[address_];

        emit RemoveFromWhitelist(address_);
    }

    function approve(address spender, uint256 amount)
        public
        override(ERC20, IERC20)
        returns (bool)
    {
        if (!allowTransfers && !_isWhitelisted([_msgSender(), spender])) {
            revert OperationForbidden(IERC20.approve.selector);
        }
        return super.approve(spender, amount);
    }

    function transfer(address recipient, uint256 amount)
        public
        override(ERC20, IERC20)
        returns (bool)
    {
        if (!allowTransfers && !_isWhitelisted([_msgSender(), recipient])) {
            revert OperationForbidden(IERC20.transfer.selector);
        }
        return super.transfer(recipient, amount);
    }

    function transferFrom(address owner, address recipient, uint256 amount)
        public
        override(ERC20, IERC20)
        returns (bool)
    {
        if (!allowTransfers && (!_isWhitelisted([owner, _msgSender(), recipient]))) {
            revert OperationForbidden(IERC20.transferFrom.selector);
        }
        return super.transferFrom(owner, recipient, amount);
    }

    function _isWhitelisted(address[2] memory addresses) private view returns (bool) {
        return (whitelist[addresses[0]] || whitelist[addresses[1]]);
    }

    function _isWhitelisted(address[3] memory addresses) private view returns (bool) {
        return (whitelist[addresses[0]] || whitelist[addresses[1]] || whitelist[addresses[2]]);
    }
}

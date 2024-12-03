// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {Context} from "@openzeppelin/contracts/utils/Context.sol";
import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {IGaspToken} from "./interfaces/IGaspToken.sol";

contract GaspToken is Context, Ownable, ERC20, IGaspToken {
    using EnumerableSet for EnumerableSet.AddressSet;

    uint256 private constant _TOTAL_SUPPLY = 1_000_000_000 * 10 ** 18;
    string private constant _NAME = "GASP";
    string private constant _SYMBOL = "GASP";

    bool public allowTransfers;
    EnumerableSet.AddressSet private _senderWhitelist;
    EnumerableSet.AddressSet private _recipientWhitelist;

    modifier isApproveOperationForbidden(address owner, address spender) {
        if (!allowTransfers && !_checkOwnerAndSpenderWhitelisted(owner, spender)) {
            revert OperationForbidden(IERC20.approve.selector);
        }
        _;
    }

    modifier isTransferOperationForbidden(address sender, address recipient) {
        if (!allowTransfers && !_checkSenderAndRecipientWhitelisted(sender, recipient)) {
            revert OperationForbidden(IERC20.transfer.selector);
        }
        _;
    }

    modifier isTransferFromOperationForbidden(address owner, address spender, address recipient) {
        if (
            !allowTransfers
                && (
                    !_checkOwnerAndSpenderWhitelisted(owner, spender)
                        || !_checkSenderAndRecipientWhitelisted(owner, recipient)
                )
        ) {
            revert OperationForbidden(IERC20.transferFrom.selector);
        }
        _;
    }

    constructor(address l1Council) Ownable() ERC20(_NAME, _SYMBOL) {
        if (l1Council == address(0)) {
            revert ZeroL1Council();
        }

        _transferOwnership(l1Council);
        _mint(l1Council, _TOTAL_SUPPLY);

        _senderWhitelist.add(l1Council);
        _recipientWhitelist.add(l1Council);
    }

    function setAllowTransfers(bool allowTransfers_) external override onlyOwner {
        if (allowTransfers) {
            revert TransfersAlreadyAllowed();
        }

        allowTransfers = allowTransfers_;
        emit AllowTransfersSet(allowTransfers_);
    }

    function whitelistSender(address sender) external override onlyOwner {
        if (sender == address(0)) {
            revert ZeroSender();
        }

        bool isAdded = _senderWhitelist.add(sender);
        if (!isAdded) {
            revert SenderAlreadyWhitelisted(sender);
        }

        emit SenderWhitelisted(sender);
    }

    function whitelistRecipient(address recipient) external override onlyOwner {
        if (recipient == address(0)) {
            revert ZeroRecipient();
        }

        bool isAdded = _recipientWhitelist.add(recipient);
        if (!isAdded) {
            revert RecipientAlreadyWhitelisted(recipient);
        }

        emit RecipientWhitelisted(recipient);
    }

    function dewhitelistSender(address sender) external override onlyOwner {
        if (sender == address(0)) {
            revert ZeroSender();
        }

        bool isRemoved = _senderWhitelist.remove(sender);
        if (!isRemoved) {
            revert SenderNotWhitelisted(sender);
        }

        emit SenderDewhitelisted(sender);
    }

    function dewhitelistRecipient(address recipient) external override onlyOwner {
        if (recipient == address(0)) {
            revert ZeroRecipient();
        }

        bool isRemoved = _recipientWhitelist.remove(recipient);
        if (!isRemoved) {
            revert RecipientNotWhitelisted(recipient);
        }

        emit RecipientDewhitelisted(recipient);
    }

    function getSenderWhitelist() external view override returns (address[] memory) {
        return _senderWhitelist.values();
    }

    function getRecipientWhitelist() external view override returns (address[] memory) {
        return _recipientWhitelist.values();
    }

    function approve(address spender, uint256 amount)
        public
        override(ERC20, IERC20)
        isApproveOperationForbidden(_msgSender(), spender)
        returns (bool)
    {
        return super.approve(spender, amount);
    }

    function transfer(address recipient, uint256 amount)
        public
        override(ERC20, IERC20)
        isTransferOperationForbidden(_msgSender(), recipient)
        returns (bool)
    {
        return super.transfer(recipient, amount);
    }

    function transferFrom(address owner, address recipient, uint256 amount)
        public
        override(ERC20, IERC20)
        isTransferFromOperationForbidden(owner, _msgSender(), recipient)
        returns (bool)
    {
        return super.transferFrom(owner, recipient, amount);
    }

    function allowance(address owner, address spender) public view override(ERC20, IERC20) returns (uint256) {
        return !allowTransfers && !_checkOwnerAndSpenderWhitelisted(owner, spender) ? 0 : super.allowance(owner, spender);
    }

    function _checkOwnerAndSpenderWhitelisted(address owner, address spender) private view returns (bool) {
        return (_senderWhitelist.contains(owner) && _senderWhitelist.contains(spender));
    }

    function _checkSenderAndRecipientWhitelisted(address sender, address recipient) private view returns (bool) {
        return (_senderWhitelist.contains(sender) && _recipientWhitelist.contains(recipient));
    }
}

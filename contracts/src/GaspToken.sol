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

    mapping(address => bool) public override senderWhitelist;
    mapping(address => bool) public override recipientWhitelist;
    bool public override allowTransfers;

    modifier isWhitelisted(address sender, address recipient, bytes4 selector) {
        if (!allowTransfers && !senderWhitelist[sender] && !recipientWhitelist[recipient]) {
            revert OperationForbidden(selector);
        }
        _;
    }

    constructor(address l1Council) Ownable() ERC20(_NAME, _SYMBOL) {
        if (l1Council == address(0)) {
            revert ZeroL1Council();
        }

        _transferOwnership(l1Council);
        _mint(l1Council, _TOTAL_SUPPLY);

        senderWhitelist[l1Council] = true;
        recipientWhitelist[l1Council] = true;
    }

    function setAllowTransfers(bool allowTransfers_) external override onlyOwner {
        if (allowTransfers) {
            revert TransfersAlreadyAllowed();
        }

        allowTransfers = allowTransfers_;
        emit AllowTransfersSet(allowTransfers_);
    }

    function addToSenderWhitelist(address account) external override onlyOwner {
        if (account == address(0)) {
            revert ZeroWhitelistedAccount();
        }
        if (senderWhitelist[account]) {
            revert AccountAlreadyWhitelisted(account);
        }

        senderWhitelist[account] = true;
        emit AddedToSenderWhitelist(account);
    }

    function addToRecipientWhitelist(address account) external override onlyOwner {
        if (account == address(0)) {
            revert ZeroWhitelistedAccount();
        }
        if (recipientWhitelist[account]) {
            revert AccountAlreadyWhitelisted(account);
        }

        recipientWhitelist[account] = true;
        emit AddedToRecipientWhitelist(account);
    }

    function removeFromSenderWhitelist(address account) external override onlyOwner {
        if (account == address(0)) {
            revert ZeroWhitelistedAccount();
        }
        if (!senderWhitelist[account]) {
            revert AccountNotWhitelisted(account);
        }

        delete senderWhitelist[account];
        emit RemovedFromSenderWhitelist(account);
    }

    function removeFromRecipientWhitelist(address account) external override onlyOwner {
        if (account == address(0)) {
            revert ZeroWhitelistedAccount();
        }
        if (!recipientWhitelist[account]) {
            revert AccountNotWhitelisted(account);
        }

        delete recipientWhitelist[account];
        emit RemovedFromRecipientWhitelist(account);
    }

    function transfer(address recipient, uint256 amount)
        public
        override(ERC20, IERC20)
        isWhitelisted(_msgSender(), recipient, IERC20.transfer.selector)
        returns (bool)
    {
        return super.transfer(recipient, amount);
    }

    function transferFrom(address owner, address recipient, uint256 amount)
        public
        override(ERC20, IERC20)
        isWhitelisted(owner, recipient, IERC20.transferFrom.selector)
        returns (bool)
    {
        return super.transferFrom(owner, recipient, amount);
    }
}

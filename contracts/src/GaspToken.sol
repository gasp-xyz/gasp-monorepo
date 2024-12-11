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

    mapping(address => bool) public whitelist;
    address public uniswapPool;
    bool public allowTransfers;

    modifier isWhitelisted(address[2] memory addresses, bytes4 selector) {
        if (!allowTransfers) {
            if (!whitelist[addresses[0]] && !whitelist[addresses[1]] || addresses[1] == uniswapPool) {
                revert OperationForbidden(selector);
            }
        }
        _;
    }

    constructor(address l1Council_, address uniswapPool_, address rolldown_) Ownable() ERC20(_NAME, _SYMBOL) {
        if (l1Council_ == address(0)) {
            revert ZeroL1Council();
        }
        if (uniswapPool_ == address(0)) {
            revert ZeroUniswapPool();
        }
        if (rolldown_ == address(0)) {
            revert ZeroRolldown();
        }

        _transferOwnership(l1Council_);
        _mint(l1Council_, _TOTAL_SUPPLY);

        whitelist[l1Council_] = true;
        whitelist[uniswapPool_] = true;
        whitelist[rolldown_] = true;

        uniswapPool = uniswapPool_;
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
            revert ZeroWhitelistAddress();
        }
        if (whitelist[address_]) {
            revert AddressAlreadyWhitelisted(address_);
        }

        whitelist[address_] = true;
        emit AddedToWhitelist(address_);
    }

    function removeFromWhitelist(address address_) external override onlyOwner {
        if (address_ == address(0)) {
            revert ZeroWhitelistAddress();
        }
        if (!whitelist[address_]) {
            revert AddressNotWhitelisted(address_);
        }

        delete whitelist[address_];
        emit RemovedFromWhitelist(address_);
    }

    function approve(address spender, uint256 amount)
        public
        override(ERC20, IERC20)
        isWhitelisted([_msgSender(), spender], IERC20.approve.selector)
        returns (bool)
    {
        return super.approve(spender, amount);
    }

    function transfer(address recipient, uint256 amount)
        public
        override(ERC20, IERC20)
        isWhitelisted([_msgSender(), recipient], IERC20.transfer.selector)
        returns (bool)
    {
        return super.transfer(recipient, amount);
    }

    function transferFrom(address owner, address recipient, uint256 amount)
        public
        override(ERC20, IERC20)
        isWhitelisted([_msgSender(), recipient], IERC20.transferFrom.selector)
        returns (bool)
    {
        return super.transferFrom(owner, recipient, amount);
    }
}

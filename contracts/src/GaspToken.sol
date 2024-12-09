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
    EnumerableSet.AddressSet private _whitelist;

    modifier isApproveOperationForbidden(address owner, address spender) {
        if (!allowTransfers && !_checkWhitelisted(owner, spender)) {
            revert OperationForbidden(IERC20.approve.selector);
        }
        _;
    }

    modifier isTransferOperationForbidden(address sender, address recipient) {
        if (!allowTransfers && !_checkWhitelisted(sender, recipient)) {
            revert OperationForbidden(IERC20.transfer.selector);
        }
        _;
    }

    modifier isTransferFromOperationForbidden(address owner, address spender, address recipient) {
        if (!allowTransfers
                && (!_checkWhitelisted(owner, spender))
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

        _whitelist.add(l1Council);
    }

    function setAllowTransfers(bool allowTransfers_) external override onlyOwner {
        if (allowTransfers) {
            revert TransfersAlreadyAllowed();
        }

        allowTransfers = allowTransfers_;
        emit AllowTransfersSet(allowTransfers_);
    }

    function whitelist(address addr) external override onlyOwner {
        if (addr == address(0)) {
            revert ZeroAddress();
        }

        bool isAdded = _whitelist.add(addr);
        if (!isAdded) {
            revert AddressAlreadyWhitelisted(addr);
        }

        emit AddressWhitelisted(addr);
    }

    function dewhitelist(address addr) external override onlyOwner {
        if (addr == address(0)) {
            revert ZeroAddress();
        }

        bool isRemoved = _whitelist.remove(addr);
        if (!isRemoved) {
            revert AddressNotWhitelisted(addr);
        }

        emit AddressDewhitelisted(addr);
    }

    function getWhitelist() external view override returns (address[] memory) {
        return _whitelist.values();
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
        return !allowTransfers && !_checkWhitelisted(owner, spender) ? 0 : super.allowance(owner, spender);
    }

    function _checkWhitelisted(address sender, address recipient) private view returns (bool) {
        return (_whitelist.contains(sender) || _whitelist.contains(recipient));
    }
}

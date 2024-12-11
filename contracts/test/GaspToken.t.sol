// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {Test} from "forge-std/Test.sol";
import {GaspToken} from "../src/GaspToken.sol";
import {Rolldown} from "../src/Rolldown.sol";


contract GaspTokenTest is Test {
    struct Accounts {
        address deployer;
        address l1Council;
        address holder;
        address sender;
        address recipient;
        address uniswapPool;
        address rolldown;
    }

    uint8 public constant DECIMALS = 18;
    uint256 public constant TOTAL_SUPPLY = 1_000_000_000 * 10 ** DECIMALS;
    string public constant NAME = "GASP";
    string public constant SYMBOL = "GASP";

    Accounts public accounts;
    GaspToken public gaspToken;

    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event AllowTransfersSet(bool allowTransfers_);
    event AddedToWhitelist(address indexed address_);
    event RemovedFromWhitelist(address indexed address_);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    event Transfer(address indexed from, address indexed to, uint256 value);

    error ZeroL1Council();
    error ZeroUniswapPool();
    error ZeroRolldown();
    error ZeroWhitelistAddress();
    error TransfersAlreadyAllowed();
    error AddressAlreadyWhitelisted(address addr);
    error AddressNotWhitelisted(address addr);
    error OperationForbidden(bytes32 selector);

    function setUp() public virtual {
        accounts = Accounts({
            deployer: makeAddr("deployer"),
            l1Council: makeAddr("l1Council"),
            holder: makeAddr("holder"),
            sender: makeAddr("sender"),
            recipient: makeAddr("recipient"),
            uniswapPool: makeAddr("uniswapPool"),
            rolldown: makeAddr("rolldown")
        });

        deal(payable(accounts.deployer), 100 ether);
        deal(payable(accounts.l1Council), 100 ether);
        deal(payable(accounts.holder), 100 ether);
        deal(payable(accounts.sender), 100 ether);
        deal(payable(accounts.recipient), 100 ether);

        vm.prank(accounts.deployer);
        gaspToken = new GaspToken(accounts.l1Council, accounts.uniswapPool, accounts.rolldown);
    }
}

contract Deploy is GaspTokenTest {
    function test_GetOwner() external view {
        assertEq(gaspToken.owner(), accounts.l1Council);
    }

    function test_GetName() external view {
        assertEq(gaspToken.name(), NAME);
    }

    function test_GetSymbol() external view {
        assertEq(gaspToken.symbol(), SYMBOL);
    }

    function test_GetDecimals() external view {
        assertEq(gaspToken.decimals(), DECIMALS);
    }

    function test_GetTotalSupply() external view {
        assertEq(gaspToken.totalSupply(), TOTAL_SUPPLY);
    }

    function test_GetAllowTransfers() external view {
        assertFalse(gaspToken.allowTransfers());
    }

    function test_GetWhitelist() external view {
        assertTrue(gaspToken.whitelist(accounts.l1Council));
        assertTrue(gaspToken.whitelist(accounts.uniswapPool));
        assertTrue(gaspToken.whitelist(accounts.rolldown));
    }

    function test_GetL1CouncilBalance() external view {
        assertEq(gaspToken.balanceOf(accounts.l1Council), TOTAL_SUPPLY);
    }

    function test_RevertIf_ZeroL1Council() external {
        vm.prank(accounts.deployer);
        vm.expectRevert(ZeroL1Council.selector);
        gaspToken = new GaspToken(address(0), accounts.uniswapPool, accounts.rolldown);
    }

    function test_RevertIf_ZeroUniswapPool() external {
        vm.prank(accounts.deployer);
        vm.expectRevert(ZeroUniswapPool.selector);
        gaspToken = new GaspToken(accounts.l1Council, address(0), accounts.rolldown);
    }

    function test_RevertIf_ZeroRolldown() external {
        vm.prank(accounts.deployer);
        vm.expectRevert(ZeroRolldown.selector);
        gaspToken = new GaspToken(accounts.l1Council, accounts.uniswapPool, address(0));
    }
}

contract TransferOwnership is GaspTokenTest {
    function test_EmitOwnershipTransferred() external {
        vm.prank(accounts.l1Council);
        vm.expectEmit();
        emit OwnershipTransferred(accounts.l1Council, accounts.deployer);
        gaspToken.transferOwnership(accounts.deployer);
    }

    function test_GetOwner() external {
        vm.prank(accounts.l1Council);
        gaspToken.transferOwnership(accounts.deployer);
        assertEq(gaspToken.owner(), accounts.deployer);
    }

    function test_RevertIf_NotOwner() external {
        vm.prank(accounts.deployer);
        vm.expectRevert("Ownable: caller is not the owner");
        gaspToken.transferOwnership(accounts.deployer);
    }
}

contract SetAllowTransfers is GaspTokenTest {
    function test_EmitAllowTransfersSet_IfAllowed() external {
        bool allowTransfers = true;

        vm.prank(accounts.l1Council);
        vm.expectEmit();
        emit AllowTransfersSet(allowTransfers);
        gaspToken.setAllowTransfers(allowTransfers);
    }

    function test_EmitAllowTransfersSet_IfNotAllowed() external {
        bool allowTransfers = false;

        vm.prank(accounts.l1Council);
        vm.expectEmit();
        emit AllowTransfersSet(allowTransfers);
        gaspToken.setAllowTransfers(allowTransfers);
    }

    function test_GetAllowTransfers() external {
        vm.prank(accounts.l1Council);
        gaspToken.setAllowTransfers(true);
        assertTrue(gaspToken.allowTransfers());
    }

    function test_RevertIf_TransfersAlreadyAllowed() external {
        vm.startPrank(accounts.l1Council);

        gaspToken.setAllowTransfers(true);

        vm.expectRevert(TransfersAlreadyAllowed.selector);
        gaspToken.setAllowTransfers(false);

        vm.stopPrank();
    }

    function test_RevertIf_NotOwner() external {
        vm.prank(accounts.sender);
        vm.expectRevert("Ownable: caller is not the owner");
        gaspToken.setAllowTransfers(false);
    }
}

contract AddToWhitelist is GaspTokenTest {
    function test_EmitAddedToWhitelist() external {
        vm.prank(accounts.l1Council);
        vm.expectEmit();
        emit AddedToWhitelist(accounts.sender);
        gaspToken.addToWhitelist(accounts.sender);
    }

    function test_GetWhitelist() external {
        vm.prank(accounts.l1Council);
        gaspToken.addToWhitelist(accounts.sender);
        assertTrue(gaspToken.whitelist(accounts.sender));
    }

    function test_RevertIf_NotOwner() external {
        vm.prank(accounts.sender);
        vm.expectRevert("Ownable: caller is not the owner");
        gaspToken.addToWhitelist(accounts.sender);
    }

    function test_RevertIf_ZeroWhitelistAddress() external {
        vm.prank(accounts.l1Council);
        vm.expectRevert(ZeroWhitelistAddress.selector);
        gaspToken.addToWhitelist(address(0));
    }

    function test_RevertIf_AddressAlreadyWhitelisted() external {
        vm.startPrank(accounts.l1Council);

        gaspToken.addToWhitelist(accounts.sender);

        vm.expectRevert(abi.encodeWithSelector(AddressAlreadyWhitelisted.selector, accounts.sender));
        gaspToken.addToWhitelist(accounts.sender);

        vm.stopPrank();
    }
}

contract RemoveFromWhitelist is GaspTokenTest {
    function test_EmitRemovedFromWhitelist() external {
        vm.prank(accounts.l1Council);
        vm.expectEmit();
        emit RemovedFromWhitelist(accounts.l1Council);
        gaspToken.removeFromWhitelist(accounts.l1Council);
    }

    function test_GetWhitelist() external {
        vm.startPrank(accounts.l1Council);

        gaspToken.addToWhitelist(accounts.sender);
        gaspToken.removeFromWhitelist(accounts.sender);

        vm.stopPrank();

        assertFalse(gaspToken.whitelist(accounts.sender));
    }

    function test_RevertIf_NotOwner() external {
        vm.prank(accounts.sender);
        vm.expectRevert("Ownable: caller is not the owner");
        gaspToken.removeFromWhitelist(accounts.sender);
    }

    function test_RevertIf_ZeroWhitelistAddress() external {
        vm.prank(accounts.l1Council);
        vm.expectRevert(ZeroWhitelistAddress.selector);
        gaspToken.removeFromWhitelist(address(0));
    }

    function test_RevertIf_AddressNotWhitelisted() external {
        vm.prank(accounts.l1Council);
        vm.expectRevert(abi.encodeWithSelector(AddressNotWhitelisted.selector, accounts.sender));
        gaspToken.removeFromWhitelist(accounts.sender);
    }
}

contract Approve is GaspTokenTest {
    uint256 public amount = 1 * 10 ** DECIMALS;

    function test_EmitApproval_IfAccountWhitelisted() external {
        vm.startPrank(accounts.l1Council);

        gaspToken.addToWhitelist(accounts.sender);

        vm.expectEmit();
        emit Approval(accounts.l1Council, accounts.sender, amount);
        gaspToken.approve(accounts.sender, amount);

        vm.stopPrank();
    }

    function test_EmitApproval_IfTransfersAllowed() external {
        vm.prank(accounts.l1Council);
        gaspToken.setAllowTransfers(true);

        vm.prank(accounts.sender);
        vm.expectEmit();
        emit Approval(accounts.sender, accounts.rolldown, amount);
        gaspToken.approve(accounts.rolldown, amount);

        vm.stopPrank();
    }

    function test_GetAllowance() external {
        vm.prank(accounts.l1Council);
        gaspToken.approve(accounts.rolldown, amount);
        assertEq(gaspToken.allowance(accounts.l1Council, accounts.rolldown), amount);
    }

    function test_RevertIf_OperationForbidden_IfAccountNotWhitelisted() external {
        vm.prank(accounts.sender);
        vm.expectRevert(abi.encodeWithSelector(OperationForbidden.selector, IERC20.approve.selector));
        gaspToken.approve(accounts.recipient, amount);
    }

    function test_RevertIf_OperationForbidden_IfIsUniswapPool() external {
        vm.prank(accounts.l1Council);
        vm.expectRevert(abi.encodeWithSelector(OperationForbidden.selector, IERC20.approve.selector));
        gaspToken.approve(accounts.uniswapPool, amount);
    }

    function test_RevertIf_ApproveFromZeroAddress() external {
        vm.prank(accounts.l1Council);
        gaspToken.setAllowTransfers(true);

        vm.prank(address(0));
        vm.expectRevert("ERC20: approve from the zero address");
        gaspToken.approve(accounts.rolldown, amount);
    }

    function test_RevertIf_ApproveToZeroAddress() external {
        vm.prank(accounts.l1Council);
        gaspToken.setAllowTransfers(true);

        vm.expectRevert("ERC20: approve to the zero address");
        gaspToken.approve(address(0), amount);
    }
}

contract TransferToken is GaspTokenTest {
    uint256 public amount = 1 * 10 ** DECIMALS;

    function test_EmitTransfer_IfAccountWhitelisted() external {
        vm.startPrank(accounts.l1Council);
        gaspToken.addToWhitelist(accounts.recipient);

        vm.expectEmit();
        emit Transfer(accounts.l1Council, accounts.recipient, amount);
        gaspToken.transfer(accounts.recipient, amount);

        vm.stopPrank();
    }

    function test_EmitTransfer_IfTransfersAllowed() external {
        vm.startPrank(accounts.l1Council);

        gaspToken.setAllowTransfers(true);
        gaspToken.transfer(accounts.sender, amount);

        vm.stopPrank();

        vm.prank(accounts.sender);
        vm.expectEmit();
        emit Transfer(accounts.sender, accounts.recipient, amount);
        gaspToken.transfer(accounts.recipient, amount);
    }

    function test_ChangeBalances() external {
        uint256 initialL1CouncilBalance = gaspToken.balanceOf(accounts.l1Council);
        uint256 initialRecipientBalance = gaspToken.balanceOf(accounts.recipient);

        vm.startPrank(accounts.l1Council);

        gaspToken.addToWhitelist(accounts.recipient);
        gaspToken.transfer(accounts.recipient, amount);

        vm.stopPrank();

        uint256 currentL1CouncilBalance = gaspToken.balanceOf(accounts.l1Council);
        assertEq(currentL1CouncilBalance, initialL1CouncilBalance - amount);

        uint256 currentRecipientBalance = gaspToken.balanceOf(accounts.recipient);
        assertEq(currentRecipientBalance, initialRecipientBalance + amount);
    }

    function test_RevertIf_OperationForbidden_IfAccountNotWhitelisted() external {
        vm.prank(accounts.sender);
        vm.expectRevert(abi.encodeWithSelector(OperationForbidden.selector, IERC20.transfer.selector));
        gaspToken.transfer(accounts.recipient, amount);
    }

    function test_RevertIf_OperationForbidden_IfIsUniswapPool() external {
        vm.prank(accounts.sender);
        vm.expectRevert(abi.encodeWithSelector(OperationForbidden.selector, IERC20.transfer.selector));
        gaspToken.transfer(accounts.uniswapPool, amount);
    }

    function test_RevertIf_TransferFromZeroAddress() external {
        vm.prank(accounts.l1Council);
        gaspToken.setAllowTransfers(true);

        vm.prank(address(0));
        vm.expectRevert("ERC20: transfer from the zero address");
        gaspToken.transfer(accounts.recipient, amount);
    }

    function test_RevertIf_TransferToZeroAddress() external {
        vm.startPrank(accounts.l1Council);

        gaspToken.setAllowTransfers(true);

        vm.expectRevert("ERC20: transfer to the zero address");
        gaspToken.transfer(address(0), amount);

        vm.stopPrank();
    }

    function test_RevertIf_InsufficientBalance() external {
        vm.startPrank(accounts.l1Council);

        gaspToken.addToWhitelist(accounts.sender);
        gaspToken.addToWhitelist(accounts.recipient);

        vm.stopPrank();

        vm.prank(accounts.sender);
        vm.expectRevert("ERC20: transfer amount exceeds balance");
        gaspToken.transfer(accounts.recipient, amount);
    }
}

contract TransferTokenFrom is GaspTokenTest {
    uint256 public amount = 1 * 10 ** DECIMALS;

    function test_EmitTransfer_IfAccountWhitelisted() external {
        vm.startPrank(accounts.l1Council);

        gaspToken.addToWhitelist(accounts.sender);
        gaspToken.approve(accounts.sender, amount);

        vm.stopPrank();

        vm.prank(accounts.sender);
        vm.expectEmit();
        emit Transfer(accounts.l1Council, accounts.recipient, amount);
        gaspToken.transferFrom(accounts.l1Council, accounts.recipient, amount);
    }

    function test_EmitTransfer_IfTransfersAllowed() external {
        vm.startPrank(accounts.l1Council);

        gaspToken.setAllowTransfers(true);
        gaspToken.transfer(accounts.holder, amount);

        vm.stopPrank();

        vm.prank(accounts.holder);
        gaspToken.approve(accounts.sender, amount);

        vm.prank(accounts.sender);
        vm.expectEmit();
        emit Transfer(accounts.holder, accounts.recipient, amount);
        gaspToken.transferFrom(accounts.holder, accounts.recipient, amount);
    }

    function test_ChangeBalances() external {
        vm.startPrank(accounts.l1Council);

        gaspToken.addToWhitelist(accounts.sender);
        gaspToken.approve(accounts.sender, amount);

        vm.stopPrank();

        uint256 initialL1CouncilBalance = gaspToken.balanceOf(accounts.l1Council);
        uint256 initialRecipientBalance = gaspToken.balanceOf(accounts.recipient);

        vm.prank(accounts.sender);
        gaspToken.transferFrom(accounts.l1Council, accounts.recipient, amount);

        uint256 currentL1CouncilBalance = gaspToken.balanceOf(accounts.l1Council);
        assertEq(currentL1CouncilBalance, initialL1CouncilBalance - amount);

        uint256 currentSpenderBalance = gaspToken.balanceOf(accounts.sender);
        assertEq(currentSpenderBalance, 0);

        uint256 currentRecipientBalance = gaspToken.balanceOf(accounts.recipient);
        assertEq(currentRecipientBalance, initialRecipientBalance + amount);
    }

    function test_GetAllowance() external {
        vm.startPrank(accounts.l1Council);

        gaspToken.addToWhitelist(accounts.sender);
        gaspToken.approve(accounts.sender, amount);

        vm.stopPrank();

        vm.prank(accounts.sender);
        gaspToken.transferFrom(accounts.l1Council, accounts.recipient, amount);

        assertEq(gaspToken.allowance(accounts.l1Council, accounts.rolldown), 0);
    }

    function test_RevertIf_OperationForbidden_IfAccountNotWhitelisted() external {
        vm.prank(accounts.sender);
        vm.expectRevert(abi.encodeWithSelector(OperationForbidden.selector, IERC20.transferFrom.selector));
        gaspToken.transferFrom(accounts.sender, accounts.recipient, amount);
    }

    function test_RevertIf_OperationForbidden_IfIsUniswapPool() external {
        vm.prank(accounts.l1Council);
        vm.expectRevert(abi.encodeWithSelector(OperationForbidden.selector, IERC20.transferFrom.selector));
        gaspToken.transferFrom(accounts.l1Council, accounts.uniswapPool, amount);
    }

    function test_RevertIf_TransferToZeroAddress() external {
        vm.startPrank(accounts.l1Council);

        gaspToken.setAllowTransfers(true);
        gaspToken.approve(accounts.rolldown, amount);

        vm.stopPrank();

        vm.prank(accounts.rolldown);
        vm.expectRevert("ERC20: transfer to the zero address");
        gaspToken.transferFrom(accounts.l1Council, address(0), amount);
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {Test} from "forge-std/Test.sol";
import {GaspToken} from "../src/GaspToken.sol";

contract GaspTokenTest is Test {
    struct Users {
        address deployer;
        address l1Council;
        address sender;
        address recipient;
        address spender;
    }

    uint256 public constant TOTAL_SUPPLY = 1_000_000_000 * 10 ** 18;
    string public constant NAME = "GASP";
    string public constant SYMBOL = "GASP";

    Users public users;
    GaspToken public gaspToken;

    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event AllowTransfersSet(bool allowTransfers_);
    event AddedToWhitelist(address indexed addr);
    event RemoveFromWhitelist(address indexed addr);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    event Transfer(address indexed from, address indexed to, uint256 value);

    error ZeroL1Council();
    error ZeroAddress();
    error TransfersAlreadyAllowed();
    error AddressAlreadyWhitelisted(address addr);
    error AddressNotWhitelisted(address addr);
    error OperationForbidden(bytes32 selector);

    function setUp() public virtual {
        users = Users({
            deployer: makeAddr("deployer"),
            l1Council: makeAddr("l1Council"),
            sender: makeAddr("sender"),
            recipient: makeAddr("recipient"),
            spender: makeAddr("spender")
        });

        deal(payable(users.deployer), 100 ether);
        deal(payable(users.l1Council), 100 ether);
        deal(payable(users.sender), 100 ether);
        deal(payable(users.recipient), 100 ether);
        deal(payable(users.spender), 100 ether);

        vm.prank(users.deployer);
        gaspToken = new GaspToken(users.l1Council);
    }
}

contract Deploy is GaspTokenTest {
    function test_GetOwner() external view {
        address owner = gaspToken.owner();
        assertEq(owner, users.l1Council);
    }

    function test_GetName() external view {
        string memory name = gaspToken.name();
        assertEq(name, NAME);
    }

    function test_GetSymbol() external view {
        string memory symbol = gaspToken.symbol();
        assertEq(symbol, SYMBOL);
    }

    function test_GetTotalSupply() external view {
        uint256 totalSupply = gaspToken.totalSupply();
        assertEq(totalSupply, TOTAL_SUPPLY);
    }

    function test_GetAllowTransfers() external view {
        bool allowTransfers = gaspToken.allowTransfers();
        assertFalse(allowTransfers);
    }

    function test_GetWhitelist() external view {
        assertTrue(gaspToken.whitelist(users.l1Council));
    }

    function test_GetL1CouncilBalance() external view {
        uint256 currentL1CouncilBalance = gaspToken.balanceOf(users.l1Council);
        assertEq(currentL1CouncilBalance, TOTAL_SUPPLY);
    }

    function test_RevertIf_ZeroL1Council() external {
        vm.prank(users.deployer);
        vm.expectRevert(ZeroL1Council.selector);
        gaspToken = new GaspToken(address(0));
    }
}

contract TransferOwnership is GaspTokenTest {
    function test_EmitOwnershipTransferred() external {
        vm.prank(users.l1Council);
        vm.expectEmit();
        emit OwnershipTransferred(users.l1Council, users.deployer);
        gaspToken.transferOwnership(users.deployer);
    }

    function test_GetOwner() external {
        vm.prank(users.l1Council);
        gaspToken.transferOwnership(users.deployer);

        address owner = gaspToken.owner();
        assertEq(owner, users.deployer);
    }

    function test_RevertIf_NotOwner() external {
        vm.prank(users.deployer);
        vm.expectRevert("Ownable: caller is not the owner");
        gaspToken.transferOwnership(users.deployer);
    }
}

contract SetAllowTransfers is GaspTokenTest {
    function test_EmitAllowTransfersSet_IfAllowed() external {
        bool allowTransfers = true;

        vm.prank(users.l1Council);
        vm.expectEmit();
        emit AllowTransfersSet(allowTransfers);
        gaspToken.setAllowTransfers(allowTransfers);
    }

    function test_EmitAllowTransfersSet_IfNotAllowed() external {
        bool allowTransfers = false;

        vm.prank(users.l1Council);
        vm.expectEmit();
        emit AllowTransfersSet(allowTransfers);
        gaspToken.setAllowTransfers(allowTransfers);
    }

    function test_GetAllowTransfers() external {
        vm.prank(users.l1Council);
        gaspToken.setAllowTransfers(true);

        bool allowTransfers = gaspToken.allowTransfers();
        assertTrue(allowTransfers);
    }

    function test_RevertIf_TransfersAlreadyAllowed() external {
        vm.startPrank(users.l1Council);

        gaspToken.setAllowTransfers(true);

        vm.expectRevert(TransfersAlreadyAllowed.selector);
        gaspToken.setAllowTransfers(false);

        vm.stopPrank();
    }

    function test_RevertIf_NotOwner() external {
        vm.prank(users.sender);
        vm.expectRevert("Ownable: caller is not the owner");
        gaspToken.setAllowTransfers(false);
    }
}

contract Whitelist is GaspTokenTest {
    function test_EmitAddressWhitelisted() external {
        vm.prank(users.l1Council);
        vm.expectEmit();
        emit AddedToWhitelist(users.sender);
        gaspToken.addToWhitelist(users.sender);
    }

    function test_GetWhitelist() external {
        vm.prank(users.l1Council);
        gaspToken.addToWhitelist(users.sender);

        assertTrue(gaspToken.whitelist(users.l1Council));
        assertTrue(gaspToken.whitelist(users.sender));
    }

    function test_RevertIf_NotOwner() external {
        vm.prank(users.sender);
        vm.expectRevert("Ownable: caller is not the owner");
        gaspToken.addToWhitelist(users.sender);
    }

    function test_RevertIf_ZeroAddress() external {
        vm.prank(users.l1Council);
        vm.expectRevert(ZeroAddress.selector);
        gaspToken.addToWhitelist(address(0));
    }

    function test_RevertIf_AddressAlreadyWhitelisted() external {
        vm.startPrank(users.l1Council);

        gaspToken.addToWhitelist(users.sender);

        vm.expectRevert(abi.encodeWithSelector(AddressAlreadyWhitelisted.selector, users.sender));
        gaspToken.addToWhitelist(users.sender);

        vm.stopPrank();
    }
}

contract Dewhitelist is GaspTokenTest {
    function test_EmitAddressDewhitelisted() external {
        vm.prank(users.l1Council);
        vm.expectEmit();
        emit RemoveFromWhitelist(users.l1Council);
        gaspToken.removeFromWhitelist(users.l1Council);
    }

    function test_GetWhitelist() external {
        vm.startPrank(users.l1Council);

        gaspToken.addToWhitelist(users.sender);
        gaspToken.removeFromWhitelist(users.sender);

        vm.stopPrank();

        assertFalse(gaspToken.whitelist(users.sender));
    }

    function test_RevertIf_NotOwner() external {
        vm.prank(users.sender);
        vm.expectRevert("Ownable: caller is not the owner");
        gaspToken.removeFromWhitelist(users.sender);
    }

    function test_RevertIf_ZeroAddress() external {
        vm.prank(users.l1Council);
        vm.expectRevert(ZeroAddress.selector);
        gaspToken.removeFromWhitelist(address(0));
    }

    function test_RevertIf_AddressNotWhitelisted() external {
        vm.prank(users.l1Council);
        vm.expectRevert(abi.encodeWithSelector(AddressNotWhitelisted.selector, users.sender));
        gaspToken.removeFromWhitelist(users.sender);
    }
}

contract Approve is GaspTokenTest {
    uint256 public amount = 1 * 10 ** 18;

    function test_EmitApproval_IfAllowTransfers() external {
        vm.startPrank(users.l1Council);

        gaspToken.setAllowTransfers(true);

        vm.expectEmit();
        emit Approval(users.l1Council, users.spender, amount);
        gaspToken.approve(users.spender, amount);

        vm.stopPrank();
    }

    function test_EmitApproval_IfRecipientWhitelisted() external {
        vm.startPrank(users.l1Council);

        gaspToken.addToWhitelist(users.spender);

        vm.expectEmit();
        emit Approval(users.l1Council, users.spender, amount);
        gaspToken.approve(users.spender, amount);

        vm.stopPrank();
    }

    function test_GetAllowance_IfAllowTransfers() external {
        vm.startPrank(users.l1Council);

        gaspToken.setAllowTransfers(true);
        gaspToken.approve(users.spender, amount);

        vm.stopPrank();

        uint256 allowance = gaspToken.allowance(users.l1Council, users.spender);
        assertEq(allowance, amount);
    }

    function test_GetAllowance_IfWhitelisted() external {
        vm.startPrank(users.l1Council);

        gaspToken.addToWhitelist(users.spender);
        gaspToken.approve(users.spender, amount);

        vm.stopPrank();

        uint256 allowance = gaspToken.allowance(users.l1Council, users.spender);
        assertEq(allowance, amount);
    }

    function test_GetAllowance_IfNotWhitelisted() external {
        vm.prank(users.l1Council);
        gaspToken.transfer(users.spender, amount);

        uint256 allowance = gaspToken.allowance(users.spender, users.recipient);
        assertEq(allowance, 0);
    }

    function test_RevertIf_OperationForbidden() external {
        vm.prank(users.sender);
        vm.expectRevert(abi.encodeWithSelector(OperationForbidden.selector, IERC20.approve.selector));
        gaspToken.approve(users.spender, amount);
    }

    function test_RevertIf_ApproveFromZeroAddress() external {
        vm.prank(users.l1Council);
        gaspToken.setAllowTransfers(true);

        vm.prank(address(0));
        vm.expectRevert("ERC20: approve from the zero address");
        gaspToken.approve(users.spender, amount);
    }

    function test_RevertIf_ApproveToZeroAddress() external {
        vm.prank(users.l1Council);
        gaspToken.setAllowTransfers(true);

        vm.expectRevert("ERC20: approve to the zero address");
        gaspToken.approve(address(0), amount);
    }
}

contract TransferToken is GaspTokenTest {
    uint256 public amount = 1 * 10 ** 18;

    function test_EmitTransfer_IfAllowTransfers() external {
        vm.startPrank(users.l1Council);

        gaspToken.setAllowTransfers(true);

        vm.expectEmit();
        emit Transfer(users.l1Council, users.recipient, amount);
        gaspToken.transfer(users.recipient, amount);

        vm.stopPrank();
    }

    function test_EmitTransfer_IfRecipientWhitelisted() external {
        vm.startPrank(users.l1Council);
        gaspToken.addToWhitelist(users.recipient);

        vm.expectEmit();
        emit Transfer(users.l1Council, users.recipient, amount);
        gaspToken.transfer(users.recipient, amount);

        vm.stopPrank();
    }

    function test_ChangeBalances() external {
        uint256 initialL1CouncilBalance = gaspToken.balanceOf(users.l1Council);
        uint256 initialRecipientBalance = gaspToken.balanceOf(users.recipient);

        vm.startPrank(users.l1Council);

        gaspToken.addToWhitelist(users.recipient);
        gaspToken.transfer(users.recipient, amount);

        vm.stopPrank();

        uint256 currentL1CouncilBalance = gaspToken.balanceOf(users.l1Council);
        assertEq(currentL1CouncilBalance, initialL1CouncilBalance - amount);

        uint256 currentRecipientBalance = gaspToken.balanceOf(users.recipient);
        assertEq(currentRecipientBalance, initialRecipientBalance + amount);
    }

    function test_RevertIf_OperationForbidden_IfNotWhitelisted() external {
        vm.prank(users.sender);
        vm.expectRevert(abi.encodeWithSelector(OperationForbidden.selector, IERC20.transfer.selector));
        gaspToken.transfer(users.recipient, amount);
    }

    function test_RevertIf_TransferFromZeroAddress() external {
        vm.prank(users.l1Council);
        gaspToken.setAllowTransfers(true);

        vm.prank(address(0));
        vm.expectRevert("ERC20: transfer from the zero address");
        gaspToken.transfer(users.recipient, amount);
    }

    function test_RevertIf_TransferToZeroAddress() external {
        vm.startPrank(users.l1Council);

        gaspToken.setAllowTransfers(true);

        vm.expectRevert("ERC20: transfer to the zero address");
        gaspToken.transfer(address(0), amount);

        vm.stopPrank();
    }

    function test_RevertIf_InsufficientBalance() external {
        vm.startPrank(users.l1Council);

        gaspToken.addToWhitelist(users.sender);
        gaspToken.addToWhitelist(users.recipient);

        vm.stopPrank();

        vm.prank(users.sender);
        vm.expectRevert("ERC20: transfer amount exceeds balance");
        gaspToken.transfer(users.recipient, amount);
    }
}

contract TransferTokenFrom is GaspTokenTest {
    uint256 public amount = 1 * 10 ** 18;

    function test_EmitTransfer_IfAllowTransfers() external {
        vm.startPrank(users.l1Council);

        gaspToken.setAllowTransfers(true);
        gaspToken.approve(users.spender, amount);

        vm.stopPrank();

        vm.prank(users.spender);
        vm.expectEmit();
        emit Transfer(users.l1Council, users.recipient, amount);
        gaspToken.transferFrom(users.l1Council, users.recipient, amount);
    }

    function test_EmitTransfer_IfSenderAndRecipientWhitelisted() external {
        vm.startPrank(users.l1Council);

        gaspToken.addToWhitelist(users.spender);
        gaspToken.approve(users.spender, amount);

        gaspToken.addToWhitelist(users.recipient);

        vm.stopPrank();

        vm.prank(users.spender);
        vm.expectEmit();
        emit Transfer(users.l1Council, users.recipient, amount);
        gaspToken.transferFrom(users.l1Council, users.recipient, amount);
    }

    function test_ChangeBalances() external {
        vm.startPrank(users.l1Council);

        gaspToken.addToWhitelist(users.spender);
        gaspToken.approve(users.spender, amount);

        gaspToken.addToWhitelist(users.recipient);

        vm.stopPrank();

        uint256 initialL1CouncilBalance = gaspToken.balanceOf(users.l1Council);
        uint256 initialRecipientBalance = gaspToken.balanceOf(users.recipient);

        vm.prank(users.spender);
        gaspToken.transferFrom(users.l1Council, users.recipient, amount);

        uint256 currentL1CouncilBalance = gaspToken.balanceOf(users.l1Council);
        assertEq(currentL1CouncilBalance, initialL1CouncilBalance - amount);

        uint256 currentRecipientBalance = gaspToken.balanceOf(users.recipient);
        assertEq(currentRecipientBalance, initialRecipientBalance + amount);

        uint256 currentSpenderBalance = gaspToken.balanceOf(users.spender);
        assertEq(currentSpenderBalance, 0);
    }

    function test_GetAllowance() external {
        vm.startPrank(users.l1Council);

        gaspToken.addToWhitelist(users.spender);
        gaspToken.approve(users.spender, amount);

        gaspToken.addToWhitelist(users.recipient);

        vm.stopPrank();

        vm.prank(users.spender);
        gaspToken.transferFrom(users.l1Council, users.recipient, amount);

        uint256 allowance = gaspToken.allowance(users.l1Council, users.spender);
        assertEq(allowance, 0);
    }

    function test_RevertIf_OperationForbidden_IfNotWhitelisted() external {
        vm.prank(users.spender);
        vm.expectRevert(abi.encodeWithSelector(OperationForbidden.selector, IERC20.transferFrom.selector));
        gaspToken.transferFrom(users.spender, users.recipient, amount);
    }

    function test_RevertIf_TransferToZeroAddress() external {
        vm.startPrank(users.l1Council);

        gaspToken.setAllowTransfers(true);
        gaspToken.approve(users.spender, amount);

        vm.stopPrank();

        vm.prank(users.spender);
        vm.expectRevert("ERC20: transfer to the zero address");
        gaspToken.transferFrom(users.l1Council, address(0), amount);
    }
}

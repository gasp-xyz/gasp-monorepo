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
    event SenderWhitelisted(address indexed sender);
    event RecipientWhitelisted(address indexed recipient);
    event SenderDewhitelisted(address indexed sender);
    event RecipientDewhitelisted(address indexed recipient);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    event Transfer(address indexed from, address indexed to, uint256 value);

    error ZeroL1Council();
    error ZeroSender();
    error ZeroRecipient();
    error TransfersAlreadyAllowed();
    error SenderAlreadyWhitelisted(address sender);
    error RecipientAlreadyWhitelisted(address recipient);
    error SenderNotWhitelisted(address sender);
    error RecipientNotWhitelisted(address recipient);
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

contract Initialize is GaspTokenTest {
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

    function test_GetSenderWhitelist() external view {
        address[] memory senderWhitelist = gaspToken.getSenderWhitelist();
        assertEq(senderWhitelist.length, 1);
        assertEq(senderWhitelist[0], users.l1Council);
    }

    function test_GetRecipientWhitelist() external view {
        address[] memory recipientWhitelist = gaspToken.getRecipientWhitelist();
        assertEq(recipientWhitelist.length, 1);
        assertEq(recipientWhitelist[0], users.l1Council);
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

contract WhitelistSender is GaspTokenTest {
    function test_EmitSenderWhitelisted() external {
        vm.prank(users.l1Council);
        vm.expectEmit();
        emit SenderWhitelisted(users.sender);
        gaspToken.whitelistSender(users.sender);
    }

    function test_GetSenderWhitelist() external {
        vm.prank(users.l1Council);
        gaspToken.whitelistSender(users.sender);

        address[] memory senderWhitelist = gaspToken.getSenderWhitelist();
        assertEq(senderWhitelist.length, 2);
        assertEq(senderWhitelist[0], users.l1Council);
        assertEq(senderWhitelist[1], users.sender);
    }

    function test_RevertIf_NotOwner() external {
        vm.prank(users.sender);
        vm.expectRevert("Ownable: caller is not the owner");
        gaspToken.whitelistSender(users.sender);
    }

    function test_RevertIf_ZeroSender() external {
        vm.prank(users.l1Council);
        vm.expectRevert(ZeroSender.selector);
        gaspToken.whitelistSender(address(0));
    }

    function test_RevertIf_SenderAlreadyWhitelisted() external {
        vm.startPrank(users.l1Council);

        gaspToken.whitelistSender(users.sender);

        vm.expectRevert(abi.encodeWithSelector(SenderAlreadyWhitelisted.selector, users.sender));
        gaspToken.whitelistSender(users.sender);

        vm.stopPrank();
    }
}

contract WhitelistRecipient is GaspTokenTest {
    function test_EmitRecipientWhitelisted() external {
        vm.prank(users.l1Council);
        vm.expectEmit();
        emit RecipientWhitelisted(users.sender);
        gaspToken.whitelistRecipient(users.sender);
    }

    function test_GetRecipientWhitelist() external {
        vm.prank(users.l1Council);
        gaspToken.whitelistRecipient(users.recipient);

        address[] memory recipientWhitelist = gaspToken.getRecipientWhitelist();
        assertEq(recipientWhitelist.length, 2);
        assertEq(recipientWhitelist[0], users.l1Council);
        assertEq(recipientWhitelist[1], users.recipient);
    }

    function test_RevertIf_NotOwner() external {
        vm.prank(users.sender);
        vm.expectRevert("Ownable: caller is not the owner");
        gaspToken.whitelistRecipient(users.sender);
    }

    function test_RevertIf_ZeroRecipient() external {
        vm.prank(users.l1Council);
        vm.expectRevert(ZeroRecipient.selector);
        gaspToken.whitelistRecipient(address(0));
    }

    function test_RevertIf_RecipientAlreadyWhitelisted() external {
        vm.startPrank(users.l1Council);

        gaspToken.whitelistRecipient(users.recipient);

        vm.expectRevert(abi.encodeWithSelector(RecipientAlreadyWhitelisted.selector, users.recipient));
        gaspToken.whitelistRecipient(users.recipient);

        vm.stopPrank();
    }
}

contract DewhitelistSender is GaspTokenTest {
    function test_EmitSenderDewhitelisted() external {
        vm.prank(users.l1Council);
        vm.expectEmit();
        emit SenderDewhitelisted(users.l1Council);
        gaspToken.dewhitelistSender(users.l1Council);
    }

    function test_GetSenderWhitelist() external {
        vm.startPrank(users.l1Council);

        gaspToken.whitelistSender(users.sender);
        gaspToken.dewhitelistSender(users.sender);

        vm.stopPrank();

        address[] memory senderWhitelist = gaspToken.getSenderWhitelist();
        assertEq(senderWhitelist.length, 1);
        assertNotEq(senderWhitelist[0], users.sender);
    }

    function test_RevertIf_NotOwner() external {
        vm.prank(users.sender);
        vm.expectRevert("Ownable: caller is not the owner");
        gaspToken.dewhitelistSender(users.sender);
    }

    function test_RevertIf_ZeroSender() external {
        vm.prank(users.l1Council);
        vm.expectRevert(ZeroSender.selector);
        gaspToken.dewhitelistSender(address(0));
    }

    function test_RevertIf_SenderNotWhitelisted() external {
        vm.prank(users.l1Council);
        vm.expectRevert(abi.encodeWithSelector(SenderNotWhitelisted.selector, users.sender));
        gaspToken.dewhitelistSender(users.sender);
    }
}

contract DewhitelistRecipient is GaspTokenTest {
    function test_EmitSenderDewhitelisted() external {
        vm.prank(users.l1Council);
        vm.expectEmit();
        emit RecipientDewhitelisted(users.l1Council);
        gaspToken.dewhitelistRecipient(users.l1Council);
    }

    function test_GetRecipientWhitelist() external {
        vm.startPrank(users.l1Council);

        gaspToken.whitelistRecipient(users.recipient);
        gaspToken.dewhitelistRecipient(users.recipient);

        vm.stopPrank();

        address[] memory recipientWhitelist = gaspToken.getRecipientWhitelist();
        assertEq(recipientWhitelist.length, 1);
        assertNotEq(recipientWhitelist[0], users.recipient);
    }

    function test_RevertIf_NotOwner() external {
        vm.prank(users.sender);
        vm.expectRevert("Ownable: caller is not the owner");
        gaspToken.dewhitelistRecipient(users.l1Council);
    }

    function test_RevertIf_ZeroRecipient() external {
        vm.prank(users.l1Council);
        vm.expectRevert(ZeroRecipient.selector);
        gaspToken.dewhitelistRecipient(address(0));
    }

    function test_RevertIf_RecipientNotWhitelisted() external {
        vm.prank(users.l1Council);
        vm.expectRevert(abi.encodeWithSelector(RecipientNotWhitelisted.selector, users.recipient));
        gaspToken.dewhitelistRecipient(users.recipient);
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

        gaspToken.whitelistSender(users.spender);

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

    function test_GetAllowance_IfNotAllowTransfers() external {
        vm.startPrank(users.l1Council);

        gaspToken.whitelistSender(users.spender);
        gaspToken.approve(users.spender, amount);

        gaspToken.dewhitelistSender(users.spender);

        vm.stopPrank();

        uint256 allowance = gaspToken.allowance(users.l1Council, users.spender);
        assertEq(allowance, 0);
    }

    function test_GetAllowance_Whitelisted() external {
        vm.startPrank(users.l1Council);

        gaspToken.whitelistSender(users.spender);
        gaspToken.approve(users.spender, amount);

        vm.stopPrank();

        uint256 allowance = gaspToken.allowance(users.l1Council, users.spender);
        assertEq(allowance, amount);
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

        gaspToken.whitelistRecipient(users.recipient);

        vm.expectEmit();
        emit Transfer(users.l1Council, users.recipient, amount);
        gaspToken.transfer(users.recipient, amount);

        vm.stopPrank();
    }

    function test_ChangeBalances() external {
        uint256 initialL1CouncilBalance = gaspToken.balanceOf(users.l1Council);
        uint256 initialRecipientBalance = gaspToken.balanceOf(users.recipient);

        vm.startPrank(users.l1Council);

        gaspToken.whitelistRecipient(users.recipient);
        gaspToken.transfer(users.recipient, amount);

        vm.stopPrank();

        uint256 currentL1CouncilBalance = gaspToken.balanceOf(users.l1Council);
        assertEq(currentL1CouncilBalance, initialL1CouncilBalance - amount);

        uint256 currentRecipientBalance = gaspToken.balanceOf(users.recipient);
        assertEq(currentRecipientBalance, initialRecipientBalance + amount);
    }

    function test_RevertIf_OperationForbidden_IfSenderNotWhitelisted() external {
        vm.prank(users.l1Council);
        gaspToken.whitelistRecipient(users.recipient);

        vm.prank(users.sender);
        vm.expectRevert(abi.encodeWithSelector(OperationForbidden.selector, IERC20.transfer.selector));
        gaspToken.transfer(users.recipient, amount);
    }

    function test_RevertIf_OperationForbidden_IfRecipientNotWhitelisted() external {
        vm.prank(users.l1Council);
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

        gaspToken.whitelistSender(users.sender);
        gaspToken.whitelistRecipient(users.recipient);

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

        gaspToken.whitelistSender(users.spender);
        gaspToken.approve(users.spender, amount);

        gaspToken.whitelistRecipient(users.recipient);

        vm.stopPrank();

        vm.prank(users.spender);
        vm.expectEmit();
        emit Transfer(users.l1Council, users.recipient, amount);
        gaspToken.transferFrom(users.l1Council, users.recipient, amount);
    }

    function test_ChangeBalances() external {
        vm.startPrank(users.l1Council);

        gaspToken.whitelistSender(users.spender);
        gaspToken.approve(users.spender, amount);

        gaspToken.whitelistRecipient(users.recipient);

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

        gaspToken.whitelistSender(users.spender);
        gaspToken.approve(users.spender, amount);

        gaspToken.whitelistRecipient(users.recipient);

        vm.stopPrank();

        vm.prank(users.spender);
        gaspToken.transferFrom(users.l1Council, users.recipient, amount);

        uint256 allowance = gaspToken.allowance(users.l1Council, users.spender);
        assertEq(allowance, 0);
    }

    function test_RevertIf_OperationForbidden_IfSpenderNotWhitelisted() external {
        vm.prank(users.spender);
        vm.expectRevert(abi.encodeWithSelector(OperationForbidden.selector, IERC20.transferFrom.selector));
        gaspToken.transferFrom(users.l1Council, users.recipient, amount);
    }

    function test_RevertIf_OperationForbidden_IfRecipientNotWhitelisted() external {
        vm.prank(users.l1Council);
        gaspToken.whitelistSender(users.spender);

        vm.prank(users.spender);
        vm.expectRevert(abi.encodeWithSelector(OperationForbidden.selector, IERC20.transferFrom.selector));
        gaspToken.transferFrom(users.l1Council, users.recipient, amount);
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

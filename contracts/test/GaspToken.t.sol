// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { Test } from "forge-std/Test.sol";
import { GaspToken } from "../src/GaspToken.sol";

contract GaspTokenTest is Test {
    struct Users {
        address l1Council;
        address sender;
        address recipient;
        address spender;
    }

    uint256 public constant TOTAL_SUPPLY = 1_000_000_000 * 10 ** 18;
    string public constant NAME = "Gasp Token";
    string public constant SYMBOL = "GASP";

    Users public users;
    GaspToken public gaspToken;

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
    error SenderAlreadyWhitelisted(address sender);
    error RecipientAlreadyWhitelisted(address recipient);
    error SenderNotWhitelisted(address sender);
    error RecipientNotWhitelisted(address recipient);
    error OperationForbidden(bytes32 selector);

    function setUp() public virtual {
        users = Users({
            l1Council: makeAddr("l1Council"),
            sender: makeAddr("sender"),
            recipient: makeAddr("recipient"),
            spender: makeAddr("spender")
        });

        deal(payable(users.l1Council), 100 ether);
        deal(payable(users.sender), 100 ether);
        deal(payable(users.recipient), 100 ether);
        deal(payable(users.spender), 100 ether);

        vm.prank(users.l1Council);
        gaspToken = new GaspToken(users.l1Council);
    }

    function _addAddressesToWhiteLists() internal {
        vm.startPrank(users.l1Council);

        gaspToken.whiltelistSender(users.sender);
        gaspToken.whiltelistSender(users.recipient);
        gaspToken.whitelistRecipient(users.sender);
        gaspToken.whitelistRecipient(users.recipient);

        vm.stopPrank();
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
        uint256 l1CouncilBalance = gaspToken.balanceOf(users.l1Council);
        assertEq(l1CouncilBalance, TOTAL_SUPPLY);
    }

    function test_RevertIf_ZeroL1Council() external {
        vm.expectRevert(ZeroL1Council.selector);
        gaspToken = new GaspToken(address(0));
    }
}

contract SetAllowTransfers is GaspTokenTest {
    function test_EmitAllowTransfersSet() external {
        bool allowTransfers = true;

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
        gaspToken.whiltelistSender(users.sender);
    }

    function test_GetSenderWhitelist() external {
        vm.prank(users.l1Council);
        gaspToken.whiltelistSender(users.sender);

        address[] memory senderWhitelist = gaspToken.getSenderWhitelist();
        assertEq(senderWhitelist.length, 2);
        assertEq(senderWhitelist[0], users.l1Council);
        assertEq(senderWhitelist[1], users.sender);
    }

    function test_RevertIf_NotOwner() external {
        vm.prank(users.sender);
        vm.expectRevert("Ownable: caller is not the owner");
        gaspToken.whiltelistSender(users.sender);
    }

    function test_RevertIf_ZeroSender() external {
        vm.prank(users.l1Council);
        vm.expectRevert(ZeroSender.selector);
        gaspToken.whiltelistSender(address(0));
    }

    function test_RevertIf_SenderAlreadyWhitelisted() external {
        vm.startPrank(users.l1Council);

        gaspToken.whiltelistSender(users.sender);

        vm.expectRevert(abi.encodeWithSelector(SenderAlreadyWhitelisted.selector, users.sender));
        gaspToken.whiltelistSender(users.sender);

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

        gaspToken.whiltelistSender(users.sender);
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

    function test_EmitApproval_AllowTransfers() external {
        vm.startPrank(users.l1Council);

        gaspToken.setAllowTransfers(true);

        vm.expectEmit();
        emit Approval(users.l1Council, users.spender, amount);
        gaspToken.approve(users.spender, amount);

        vm.stopPrank();
    }

    function test_EmitApproval_NotAllowTransfers() external {
        vm.startPrank(users.l1Council);

        gaspToken.whitelistRecipient(users.spender);

        vm.expectEmit();
        emit Approval(users.l1Council, users.spender, amount);
        gaspToken.approve(users.spender, amount);

        vm.stopPrank();
    }

    function test_GetAllowance_AllowTransfers() external {
        vm.startPrank(users.l1Council);

        gaspToken.setAllowTransfers(true);
        gaspToken.approve(users.spender, amount);

        vm.stopPrank();

        uint256 allowance = gaspToken.allowance(users.l1Council, users.spender);
        assertEq(allowance, amount);
    }

    function test_GetAllowance_NotAllowTransfers() external {
        vm.startPrank(users.l1Council);

        gaspToken.setAllowTransfers(true);
        gaspToken.approve(users.spender, amount);
        gaspToken.setAllowTransfers(false);

        vm.stopPrank();

        uint256 allowance = gaspToken.allowance(users.l1Council, users.spender);
        assertEq(allowance, 0);
    }

    function test_GetAllowance_Whitelisted() external {
        vm.startPrank(users.l1Council);

        gaspToken.whitelistRecipient(users.spender);
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

pragma solidity ^0.8.0;
import {RollDown} from "../src/rolldown.sol";
import {stdStorage, StdStorage, Test} from "forge-std/Test.sol";
import "forge-std/console.sol";
import {Utilities, MyERC20} from "./utils/Utilities.sol";

contract RollDownTest is Test {
    RollDown public rollDown;
    Utilities internal utils;
    address payable[] internal users;
    MyERC20 internal token;

    function setUp() public {
        utils = new Utilities();
        users = utils.createUsers(1);
        rollDown = new RollDown();
    }

    function beforeEach() public {}

    function testExecuteDeposit() public {
        // Arrange
        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 10;
        deal(tokenAddress, alice, 100 ether);
        uint256 aliceBalanceBefore = token.balanceOf(alice);
        uint256 contractBalanceBefore = token.balanceOf(address(rollDown));

        // Act
        vm.startPrank(alice);
        token.approve(address(rollDown), amount);
        vm.expectEmit(true, true, true, true);
        emit RollDown.DepositAcceptedIntoQueue(1, alice, tokenAddress, amount);
        rollDown.deposit(tokenAddress, 10);
        vm.stopPrank();

        RollDown.L1Update memory l1Update = rollDown.getUpdateForL2();
        uint256 aliceBalanceAfter = token.balanceOf(alice);
        uint256 contractBalanceAfter = token.balanceOf(address(rollDown));

        // Assert
        assertEq(l1Update.order.length, 1);
        assert(l1Update.order[0] == RollDown.PendingRequestType.DEPOSIT);
        assertEq(l1Update.pendingDeposits.length, 1);
        assertEq(l1Update.pendingDeposits[0].depositRecipient, alice);
        assertEq(l1Update.pendingDeposits[0].tokenAddress, tokenAddress);
        assertEq(l1Update.pendingDeposits[0].amount, amount);
        assertEq(aliceBalanceBefore - aliceBalanceAfter, 10);
        assertEq(contractBalanceAfter - contractBalanceBefore, 10);
    }

    function testAcceptL2Update() public {
        // Arrange
        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 10;
        deal(tokenAddress, alice, 100 ether);
        vm.startPrank(alice);
        token.approve(address(rollDown), amount);
        rollDown.deposit(tokenAddress, 10);
        vm.stopPrank();

        RollDown.L1Update memory l1Update = rollDown.getUpdateForL2();
        assertEq(l1Update.order.length, 1);
        assert(l1Update.order[0] == RollDown.PendingRequestType.DEPOSIT);
        assertEq(l1Update.offset, 1);
        assertEq(l1Update.lastProccessedRequestOnL1, 0);
        assertEq(l1Update.lastAcceptedRequestOnL1, 1);

        RollDown.L2Update memory l2Update;
        l2Update.cancels = new RollDown.Cancel[](0);
        l2Update.results = new RollDown.RequestResult[](1);
        l2Update.results[0] = RollDown.RequestResult({
            requestId: 1,
            updateType: RollDown.UpdateType.DEPOSIT,
            status: false
        });

        // Act
        rollDown.update_l1_from_l2(l2Update);

        l1Update = rollDown.getUpdateForL2();
        assertEq(l1Update.order.length, 1);
        assert(
            l1Update.order[0] ==
                RollDown.PendingRequestType.L2_UPDATES_TO_REMOVE
        );
        assertEq(l1Update.offset, 2);
        assertEq(l1Update.lastProccessedRequestOnL1, 1);
        assertEq(l1Update.lastAcceptedRequestOnL1, 2);

        RollDown.L2Update memory l2Update2;
        l2Update2.cancels = new RollDown.Cancel[](0);
        l2Update2.results = new RollDown.RequestResult[](1);
        l2Update2.results[0] = RollDown.RequestResult({
            requestId: 2,
            updateType: RollDown.UpdateType.INDEX_UPDATE,
            status: true
        });
        rollDown.update_l1_from_l2(l2Update2);

        l1Update = rollDown.getUpdateForL2();
        assertEq(l1Update.order.length, 1);
        assertEq(l1Update.offset, 3);
        assertEq(l1Update.lastProccessedRequestOnL1, 2);
        assertEq(l1Update.lastAcceptedRequestOnL1, 3);

        RollDown.L2Update memory l2Update3;
        l2Update3.cancels = new RollDown.Cancel[](0);
        l2Update3.results = new RollDown.RequestResult[](1);
        l2Update3.results[0] = RollDown.RequestResult({
            requestId: 3,
            updateType: RollDown.UpdateType.INDEX_UPDATE,
            status: true
        });
        rollDown.update_l1_from_l2(l2Update3);

        // Assert
        l1Update = rollDown.getUpdateForL2();
        assertEq(l1Update.order.length, 1);
        assertEq(l1Update.offset, 4);
        assertEq(l1Update.lastProccessedRequestOnL1, 3);
        assertEq(l1Update.lastAcceptedRequestOnL1, 4);
    }

    function testIgnoreDuplicatedUpdates() public {
        // Arrange

        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 10;
        deal(tokenAddress, alice, 100 ether);
        vm.startPrank(alice);
        token.approve(address(rollDown), amount);
        rollDown.deposit(tokenAddress, 10);
        vm.stopPrank();

        RollDown.L2Update memory l2Update;
        l2Update.cancels = new RollDown.Cancel[](0);
        l2Update.results = new RollDown.RequestResult[](1);
        l2Update.results[0] = RollDown.RequestResult({
            requestId: 1,
            updateType: RollDown.UpdateType.DEPOSIT,
            status: false
        });

        // Act
        // make sure that executing same request does not alter the state
        rollDown.update_l1_from_l2(l2Update);
        RollDown.L1Update memory update1 = rollDown.getUpdateForL2();
        rollDown.update_l1_from_l2(l2Update);
        rollDown.update_l1_from_l2(l2Update);
        rollDown.update_l1_from_l2(l2Update);
        rollDown.update_l1_from_l2(l2Update);
        rollDown.update_l1_from_l2(l2Update);
        RollDown.L1Update memory update2 = rollDown.getUpdateForL2();

        // Assert
        assertEq(
            update1.lastProccessedRequestOnL1,
            update2.lastProccessedRequestOnL1
        );
        assertEq(
            update1.lastAcceptedRequestOnL1,
            update2.lastAcceptedRequestOnL1
        );
        assertEq(update1.offset, update2.offset);
        assertEq(update1.order.length, update2.order.length);
        assertEq(
            update1.pendingDeposits.length,
            update2.pendingDeposits.length
        );
        assertEq(
            update1.pendingCancelResultions.length,
            update2.pendingCancelResultions.length
        );
        assertEq(
            update1.pendingL2UpdatesToRemove.length,
            update2.pendingL2UpdatesToRemove.length
        );
    }

    function testHashCompatibilityWithMangataNode() public {
        RollDown.L1Update memory l1Update;
        l1Update.offset = 1;
        l1Update.pendingDeposits = new RollDown.Deposit[](1);
        l1Update.pendingDeposits[0] = RollDown.Deposit({
            depositRecipient: 0x0000000000000000000000000000000000000004,
            tokenAddress: 0x2CD2188119797153892438E57364D95B32975560,
            amount: 1000000,
            blockHash: 0x0000000000000000000000000000000000000000000000000000000000000000
        });
        l1Update.order = new RollDown.PendingRequestType[](1);
        l1Update.order[0] = RollDown.PendingRequestType.DEPOSIT;

        bytes32 l2Hash = 0x17df588b43dd6f217debd726704178a1ed63aa47522066e8c5283a7797f57e8c;
        assertEq(keccak256(abi.encode(l1Update)), l2Hash);
    }

    function testCancelResolutionWithNonMatchingHashResultsWithUnjustifiedStatus()
        public
    {
        // Arrange
        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 1000;
        deal(tokenAddress, alice, amount);
        vm.startPrank(alice);
        token.approve(address(rollDown), amount);
        rollDown.deposit(tokenAddress, amount);
        vm.stopPrank();

        RollDown.L2Update memory l2Update;
        l2Update.results = new RollDown.RequestResult[](0);
        l2Update.cancels = new RollDown.Cancel[](1);
        l2Update.cancels[0] = RollDown.Cancel({
            l2RequestId: 170141183460469231731687303715884105728,
            lastProccessedRequestOnL1: 0,
            lastAcceptedRequestOnL1: 1,
            hash: bytes32(uint256(0))
        });

        vm.startPrank(alice);
        vm.expectEmit(true, true, true, true);
        emit RollDown.DisputeResolutionAcceptedIntoQueue(170141183460469231731687303715884105728, false);
        rollDown.update_l1_from_l2(l2Update);
        vm.stopPrank();
    }

    function testCancelResolutionWithMatchingHashResultsWithJustifiedStatus()
        public
    {
        // Arrange
        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 1000;
        deal(tokenAddress, alice, amount);

        // Act
        vm.startPrank(alice);
        token.approve(address(rollDown), amount);
        rollDown.deposit(tokenAddress, amount);
        vm.stopPrank();

        RollDown.L1Update memory l1Update = rollDown.getUpdateForL2();

        RollDown.L2Update memory l2Update;
        l2Update.results = new RollDown.RequestResult[](0);
        l2Update.cancels = new RollDown.Cancel[](1);
        l2Update.cancels[0] = RollDown.Cancel({
            l2RequestId: 170141183460469231731687303715884105728,
            lastAcceptedRequestOnL1: 1,
            lastProccessedRequestOnL1: 0,
            hash: bytes32(keccak256(abi.encode(l1Update)))
        });

        vm.startPrank(alice);
        vm.expectEmit(true, true, true, true);
        emit RollDown.DisputeResolutionAcceptedIntoQueue(170141183460469231731687303715884105728, true);
        rollDown.update_l1_from_l2(l2Update);
        vm.stopPrank();
    }

    function testProcessWithdraw() public {
        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 10;

        // Arrange
        deal(tokenAddress, alice, 100 ether);

        // Act
        vm.startPrank(alice);
        token.transfer(address(rollDown), amount);
        vm.stopPrank();
        uint256 aliceBalanceBefore = token.balanceOf(alice);
        uint256 contractBalanceBefore = token.balanceOf(address(rollDown));

        RollDown.L2Update memory l2Update;
        l2Update.results = new RollDown.RequestResult[](0);
        l2Update.withdraws = new RollDown.Withdraw[](1);
        l2Update.withdraws[0] = RollDown.Withdraw({
            requestId: 170141183460469231731687303715884105728, //u128max/2+1
            withdrawRecipient: alice,
            tokenAddress: tokenAddress,
            amount: amount
        });

        rollDown.update_l1_from_l2(l2Update);
        uint256 aliceBalanceAfter = token.balanceOf(alice);
        uint256 contractBalanceAfter = token.balanceOf(address(rollDown));

        assertEq(aliceBalanceBefore + amount, aliceBalanceAfter);
        assertEq(contractBalanceBefore - amount, contractBalanceAfter);
    }
}

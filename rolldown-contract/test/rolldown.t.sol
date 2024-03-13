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
        assertEq(l1Update.pendingDeposits.length, 1);
        assertEq(l1Update.pendingCancelResultions.length, 0);
        assertEq(l1Update.pendingL2UpdatesToRemove.length, 0);
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
        assertEq(l1Update.pendingDeposits.length, 1);
        assertEq(l1Update.pendingL2UpdatesToRemove.length, 0);
        assertEq(l1Update.pendingCancelResultions.length, 0);
        assertEq(l1Update.pendingDeposits[0].requestId.id, 1);

        RollDown.L2Update memory l2Update;
        l2Update.cancles = new RollDown.Cancel[](0);
        l2Update.results = new RollDown.RequestResult[](1);
        l2Update.results[0] = RollDown.RequestResult({
            requestId: RollDown.RequestId({id: 1, origin: RollDown.Origin.L2}),
            originRequestId: 1,
            updateType: RollDown.UpdateType.DEPOSIT,
            status: false
        });

        // Act
        rollDown.update_l1_from_l2(l2Update);

        l1Update = rollDown.getUpdateForL2();
        assertEq(l1Update.pendingL2UpdatesToRemove.length, 1);
        assertEq(l1Update.pendingDeposits.length, 0);
        assertEq(l1Update.pendingCancelResultions.length, 0);
        assertEq(l1Update.pendingL2UpdatesToRemove[0].requestId.id, 2);

        RollDown.L2Update memory l2Update2;
        l2Update2.cancles = new RollDown.Cancel[](0);
        l2Update2.results = new RollDown.RequestResult[](1);
        l2Update2.results[0] = RollDown.RequestResult({
            requestId: RollDown.RequestId({id: 2, origin: RollDown.Origin.L2}),
            originRequestId: 1,
            updateType: RollDown.UpdateType.INDEX_UPDATE,
            status: true
        });
        rollDown.update_l1_from_l2(l2Update2);

        l1Update = rollDown.getUpdateForL2();
        assertEq(l1Update.pendingL2UpdatesToRemove.length, 1);
        assertEq(l1Update.pendingDeposits.length, 0);
        assertEq(l1Update.pendingCancelResultions.length, 0);
        assertEq(l1Update.pendingL2UpdatesToRemove[0].requestId.id, 3);

        RollDown.L2Update memory l2Update3;
        l2Update3.cancles = new RollDown.Cancel[](0);
        l2Update3.results = new RollDown.RequestResult[](1);
        l2Update3.results[0] = RollDown.RequestResult({
            requestId: RollDown.RequestId({id: 3, origin: RollDown.Origin.L2}),
            originRequestId: 2,
            updateType: RollDown.UpdateType.INDEX_UPDATE,
            status: true
        });
        rollDown.update_l1_from_l2(l2Update3);

        // Assert
        l1Update = rollDown.getUpdateForL2();
        assertEq(l1Update.pendingL2UpdatesToRemove.length, 1);
        assertEq(l1Update.pendingDeposits.length, 0);
        assertEq(l1Update.pendingCancelResultions.length, 0);
        assertEq(l1Update.pendingL2UpdatesToRemove[0].requestId.id, 4);
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
        l2Update.cancles = new RollDown.Cancel[](0);
        l2Update.results = new RollDown.RequestResult[](1);
        l2Update.results[0] = RollDown.RequestResult({
            requestId: RollDown.RequestId({id: 1, origin: RollDown.Origin.L2}),
            updateType: RollDown.UpdateType.DEPOSIT,
            originRequestId: 1,
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
            update1.pendingL2UpdatesToRemove[0].l2UpdatesToRemove,
            update2.pendingL2UpdatesToRemove[0].l2UpdatesToRemove
        );
    }

    function testHashCompatibilityWithMangataNode() public {
        RollDown.L1Update memory l1Update;
        l1Update.pendingDeposits = new RollDown.Deposit[](1);
        l1Update.pendingL2UpdatesToRemove = new RollDown.L2UpdatesToRemove[](1);
        l1Update.pendingCancelResultions = new RollDown.CancelResolution[](1);

        l1Update.pendingDeposits[0] = RollDown.Deposit({
            requestId: RollDown.RequestId({id: 1, origin: RollDown.Origin.L1}),
            depositRecipient: 0x0000000000000000000000000000000000000004,
            tokenAddress: 0x0000000000000000000000000000000000000004,
            amount: 1000000
        });

        l1Update.pendingCancelResultions[0] = RollDown.CancelResolution({
            requestId: RollDown.RequestId({id: 2, origin: RollDown.Origin.L1}),
            l2RequestId: 3,
            cancelJustified: true
        });

        // Request
        // RollDown.RequestId[] memory l2UpdatesToRemove = new RollDown.RequestId[](1);
        // l2UpdatesToRemove[0] = RollDown.RequestId({id: 1, origin: RollDown.Origin.L2});
        // l1Update.pendingL2UpdatesToRemove[0] = RollDown.L2UpdatesToRemove({
        //     requestId: RollDown.RequestId({id: 1, origin: RollDown.Origin.L1}),
        //     l2UpdatesToRemove: l2UpdatesToRemove
        // });
        //

        bytes32 l2Hash =  0x134d8fb03451305234f1d783a3923bffd5a6b276a646386d0a18fc953c403637;
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
        l2Update.cancles = new RollDown.Cancel[](1);
        l2Update.cancles[0] = RollDown.Cancel({
            requestId: RollDown.RequestId({id: 50000, origin: RollDown.Origin.L2}),
            range: RollDown.Range({start: 1, end: 1}),
            hash: bytes32(uint256(0))
        });


        vm.startPrank(alice);
        vm.expectEmit(true, true, true, true);
        emit RollDown.DisputeResolutionAcceptedIntoQueue(50000, false);
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
        l2Update.cancles = new RollDown.Cancel[](1);
        l2Update.cancles[0] = RollDown.Cancel({
            requestId: RollDown.RequestId({id: 50000, origin: RollDown.Origin.L2}),
            range: RollDown.Range({start: 1, end: 1}),
            hash: bytes32(keccak256(abi.encode(l1Update)))
        });

        vm.startPrank(alice);
        vm.expectEmit(true, true, true, true);
        emit RollDown.DisputeResolutionAcceptedIntoQueue(50000, true);
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
        l2Update.withdrawals = new RollDown.Withdrawal[](1);
        l2Update.withdrawals[0] = RollDown.Withdrawal({
            requestId: RollDown.RequestId({id: 1, origin: RollDown.Origin.L2}),
            withdrawalRecipient: alice,
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

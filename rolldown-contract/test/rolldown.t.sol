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
        l2Update.results = new RollDown.RequestResult[](1);
        l2Update.results[0] = RollDown.RequestResult({
            requestId: RollDown.RequestId({id: 1, origin: RollDown.Origin.L2}),
            originRequestId: 1,
            updateType: RollDown.UpdateType.DEPOSIT,
            status: false
        });

        // Act
        rollDown.update_l1_from_l2(l2Update);
        //
        l1Update = rollDown.getUpdateForL2();
        assertEq(l1Update.pendingL2UpdatesToRemove.length, 1);
        assertEq(l1Update.pendingDeposits.length, 0);
        assertEq(l1Update.pendingCancelResultions.length, 0);
        assertEq(l1Update.pendingL2UpdatesToRemove[0].requestId.id, 2);
        assertEq(
            l1Update.pendingL2UpdatesToRemove[0].l2UpdatesToRemove.length,
            1
        );
        assertEq(l1Update.pendingL2UpdatesToRemove[0].l2UpdatesToRemove[0], 1);

        RollDown.L2Update memory l2Update2;
        l2Update2.cancels = new RollDown.Cancel[](0);
        l2Update2.results = new RollDown.RequestResult[](1);
        l2Update2.results[0] = RollDown.RequestResult({
            requestId: RollDown.RequestId({id: 2, origin: RollDown.Origin.L2}),
            originRequestId: 2,
            updateType: RollDown.UpdateType.INDEX_UPDATE,
            status: true
        });
        rollDown.update_l1_from_l2(l2Update2);

        l1Update = rollDown.getUpdateForL2();
        assertEq(l1Update.pendingL2UpdatesToRemove.length, 1);
        assertEq(l1Update.pendingDeposits.length, 0);
        assertEq(l1Update.pendingCancelResultions.length, 0);
        assertEq(l1Update.pendingL2UpdatesToRemove[0].requestId.id, 3);
        assertEq(
            l1Update.pendingL2UpdatesToRemove[0].l2UpdatesToRemove.length,
            1
        );
        assertEq(l1Update.pendingL2UpdatesToRemove[0].l2UpdatesToRemove[0], 2);

        RollDown.L2Update memory l2Update3;
        l2Update3.cancels = new RollDown.Cancel[](0);
        l2Update3.results = new RollDown.RequestResult[](1);
        l2Update3.results[0] = RollDown.RequestResult({
            requestId: RollDown.RequestId({id: 3, origin: RollDown.Origin.L2}),
            originRequestId: 3,
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
        assertEq(
            l1Update.pendingL2UpdatesToRemove[0].l2UpdatesToRemove.length,
            1
        );
        assertEq(l1Update.pendingL2UpdatesToRemove[0].l2UpdatesToRemove[0], 3);
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
            requestId: RollDown.RequestId({id: 1, origin: RollDown.Origin.L2}),
            originRequestId: 1,
            updateType: RollDown.UpdateType.DEPOSIT,
            status: false
        });

        // Act
        // make sure that executing same request does not alter the state
        rollDown.update_l1_from_l2(l2Update);
        vm.expectRevert("Invalid L2Update");
        rollDown.update_l1_from_l2(l2Update);
    }

    function testL1UpdateHashCompatibilityWithMangataNode() public {
        RollDown.L1Update memory l1Update;
        l1Update.pendingDeposits = new RollDown.Deposit[](1);
        l1Update.pendingL2UpdatesToRemove = new RollDown.L2UpdatesToRemove[](1);
        l1Update.pendingCancelResultions = new RollDown.CancelResolution[](1);
        l1Update
            .pendingWithdrawalResolutions = new RollDown.WithdrawalResolution[](
            1
        );

        l1Update.pendingDeposits[0] = RollDown.Deposit({
            requestId: RollDown.RequestId({id: 1, origin: RollDown.Origin.L1}),
            depositRecipient: 0x0000000000000000000000000000000000000002,
            tokenAddress: 0x0000000000000000000000000000000000000003,
            amount: 4,
            blockHash: 0x0000000000000000000000000000000000000000000000000000000000000005
        });

        l1Update.pendingCancelResultions[0] = RollDown.CancelResolution({
            requestId: RollDown.RequestId({id: 6, origin: RollDown.Origin.L1}),
            l2RequestId: 7,
            cancelJustified: true,
            blockHash: 0x0000000000000000000000000000000000000000000000000000000000000008
        });

        l1Update.pendingWithdrawalResolutions[0] = RollDown
            .WithdrawalResolution({
                requestId: RollDown.RequestId({
                    id: 9,
                    origin: RollDown.Origin.L1
                }),
                l2RequestId: 10,
                status: true,
                blockHash: 0x000000000000000000000000000000000000000000000000000000000000000b
            });

        uint256[] memory l2UpdatesToRemove = new uint256[](1);
        l2UpdatesToRemove[0] = 13;
        l1Update.pendingL2UpdatesToRemove[0] = RollDown.L2UpdatesToRemove({
            requestId: RollDown.RequestId({id: 12, origin: RollDown.Origin.L1}),
            l2UpdatesToRemove: l2UpdatesToRemove,
            blockHash: 0x000000000000000000000000000000000000000000000000000000000000000e
        });

        assertEq(
            keccak256(abi.encode(l1Update)),
            0x5129c9a6605d367397902fa839ef429af9abed97f0dd36e3b1973939817d40dc
        );
    }

    function testL2UpdateHashCompatibilityWithMangataNode() public {
        // TODO: add such  a test on substrate side
        RollDown.L2Update memory l2Update;
        l2Update.cancels = new RollDown.Cancel[](1);
        l2Update.withdrawals = new RollDown.Withdrawal[](1);
        l2Update.results = new RollDown.RequestResult[](1);

        l2Update.cancels[0] = RollDown.Cancel({
            requestId: RollDown.RequestId({id: 1, origin: RollDown.Origin.L2}),
            range: RollDown.Range({start: 2, end: 3}),
            hash: 0x0000000000000000000000000000000000000000000000000000000000000004
        });

        l2Update.withdrawals[0] = RollDown.Withdrawal({
            requestId: RollDown.RequestId({id: 5, origin: RollDown.Origin.L2}),
            withdrawalRecipient: 0x0000000000000000000000000000000000000006,
            tokenAddress: 0x0000000000000000000000000000000000000007,
            amount: 8
        });

        l2Update.results[0] = RollDown.RequestResult({
            requestId: RollDown.RequestId({id: 9, origin: RollDown.Origin.L2}),
            originRequestId: 10,
            updateType: RollDown.UpdateType.INDEX_UPDATE,
            status: true
        });

        bytes32 l2Hash = 0x86056ca0e2dd30bb80627000335bd10a2a57699f532e665260ffebb55047544f;
        assertEq(keccak256(abi.encode(l2Update)), l2Hash);
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
            requestId: RollDown.RequestId({id: 1, origin: RollDown.Origin.L2}),
            range: RollDown.Range({start: 1, end: 1}),
            hash: bytes32(uint256(0))
        });

        vm.startPrank(alice);
        vm.expectEmit(true, true, true, true);
        emit RollDown.DisputeResolutionAcceptedIntoQueue(1, false);
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
            requestId: RollDown.RequestId({id: 1, origin: RollDown.Origin.L2}),
            range: RollDown.Range({start: 1, end: 1}),
            hash: bytes32(keccak256(abi.encode(l1Update)))
        });

        vm.startPrank(alice);
        vm.expectEmit(true, true, true, true);
        emit RollDown.DisputeResolutionAcceptedIntoQueue(1, true);
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

    function testSuccessfulWithdrawalRequest() public {
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
        l2Update.withdrawals = new RollDown.Withdrawal[](1);
        l2Update.withdrawals[0] = RollDown.Withdrawal({
            requestId: RollDown.RequestId({id: 1, origin: RollDown.Origin.L2}),
            withdrawalRecipient: alice,
            tokenAddress: tokenAddress,
            amount: 500
        });

        vm.startPrank(alice);
        vm.expectEmit(true, true, true, true);
        emit RollDown.WithdrawalResolutionAcceptedIntoQueue(2, true);
        emit RollDown.FundsWithdrawn(alice, tokenAddress, 500);
        rollDown.update_l1_from_l2(l2Update);
        vm.stopPrank();

        RollDown.L1Update memory l1Update = rollDown.getUpdateForL2();
        assertEq(l1Update.pendingWithdrawalResolutions.length, 1);
    }

    function testUnsuccessfulWithdrawalRequest() public {
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
        l2Update.withdrawals = new RollDown.Withdrawal[](1);
        l2Update.withdrawals[0] = RollDown.Withdrawal({
            requestId: RollDown.RequestId({id: 1, origin: RollDown.Origin.L2}),
            withdrawalRecipient: alice,
            tokenAddress: tokenAddress,
            amount: 1001
        });

        vm.startPrank(alice);
        vm.expectEmit(true, true, true, true);
        emit RollDown.WithdrawalResolutionAcceptedIntoQueue(2, false);
        rollDown.update_l1_from_l2(l2Update);
        vm.stopPrank();
    }

    function testRejectL2UpdateWhenSubmittedSecondTime() public {
        // Arrange
        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 1000;
        deal(tokenAddress, alice, 2 * amount);
        vm.startPrank(alice);
        token.approve(address(rollDown), 2 * amount);
        rollDown.deposit(tokenAddress, amount);
        rollDown.deposit(tokenAddress, amount);
        vm.stopPrank();

        RollDown.L2Update memory l2Update;
        l2Update.results = new RollDown.RequestResult[](0);
        l2Update.withdrawals = new RollDown.Withdrawal[](1);
        l2Update.withdrawals[0] = RollDown.Withdrawal({
            requestId: RollDown.RequestId({id: 1, origin: RollDown.Origin.L2}),
            withdrawalRecipient: alice,
            tokenAddress: tokenAddress,
            amount: 1000
        });

        rollDown.update_l1_from_l2(l2Update);

        vm.expectRevert("Invalid L2Update");
        rollDown.update_l1_from_l2(l2Update);
    }

    function testDoNotRejectPartialyKnownL2Update() public {
        // Arrange
        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 1000;
        deal(tokenAddress, alice, 2 * amount);
        vm.startPrank(alice);
        token.approve(address(rollDown), 2 * amount);
        rollDown.deposit(tokenAddress, amount);
        rollDown.deposit(tokenAddress, amount);
        vm.stopPrank();

        RollDown.L2Update memory l2Update;
        l2Update.results = new RollDown.RequestResult[](0);
        l2Update.withdrawals = new RollDown.Withdrawal[](1);
        l2Update.withdrawals[0] = RollDown.Withdrawal({
            requestId: RollDown.RequestId({id: 1, origin: RollDown.Origin.L2}),
            withdrawalRecipient: alice,
            tokenAddress: tokenAddress,
            amount: 1000
        });
        rollDown.update_l1_from_l2(l2Update);

        RollDown.L2Update memory partiallyKnownUpdate;
        partiallyKnownUpdate.results = new RollDown.RequestResult[](0);
        partiallyKnownUpdate.withdrawals = new RollDown.Withdrawal[](2);
        partiallyKnownUpdate.withdrawals[0] = RollDown.Withdrawal({
            requestId: RollDown.RequestId({id: 1, origin: RollDown.Origin.L2}),
            withdrawalRecipient: alice,
            tokenAddress: tokenAddress,
            amount: 1000
        });
        partiallyKnownUpdate.withdrawals[1] = RollDown.Withdrawal({
            requestId: RollDown.RequestId({id: 2, origin: RollDown.Origin.L2}),
            withdrawalRecipient: alice,
            tokenAddress: tokenAddress,
            amount: 1000
        });

        rollDown.update_l1_from_l2(partiallyKnownUpdate);
    }

    function testReproduce() public {
        // Arrange
        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 1000;
        deal(tokenAddress, alice, 2 * amount);
        vm.startPrank(alice);
        token.approve(address(rollDown), 2 * amount);
        rollDown.deposit(tokenAddress, amount);
        rollDown.deposit(tokenAddress, amount);
        vm.stopPrank();

        RollDown.L2Update memory l2Update;
        l2Update.results = new RollDown.RequestResult[](2);
        l2Update.results[0] = RollDown.RequestResult({
            requestId: RollDown.RequestId({id: 1, origin: RollDown.Origin.L2}),
            originRequestId: 1,
            updateType: RollDown.UpdateType.DEPOSIT,
            status: true
        });
        l2Update.results[1] = RollDown.RequestResult({
            requestId: RollDown.RequestId({id: 2, origin: RollDown.Origin.L2}),
            originRequestId: 2,
            updateType: RollDown.UpdateType.DEPOSIT,
            status: false
        });

        rollDown.update_l1_from_l2(l2Update);

        RollDown.L2Update memory l2Update2;
        l2Update2.withdrawals = new RollDown.Withdrawal[](1);
        l2Update2.withdrawals[0] = RollDown.Withdrawal({
            requestId: RollDown.RequestId({id: 3, origin: RollDown.Origin.L2}),
            withdrawalRecipient: alice,
            tokenAddress: tokenAddress,
            amount: 1000
        });

        l2Update2.results = new RollDown.RequestResult[](1);
        l2Update2.results[0] = RollDown.RequestResult({
            requestId: RollDown.RequestId({id: 4, origin: RollDown.Origin.L2}),
            originRequestId: 3,
            updateType: RollDown.UpdateType.INDEX_UPDATE,
            status: true
        });

        rollDown.update_l1_from_l2(l2Update2);
    }

    function testOverlapping() public {
        // Arrange
        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 1000;
        deal(tokenAddress, alice, 2 * amount);
        vm.startPrank(alice);
        token.approve(address(rollDown), 2 * amount);
        rollDown.deposit(tokenAddress, amount);
        rollDown.deposit(tokenAddress, amount);
        vm.stopPrank();

        RollDown.L2Update memory l2Update;
        l2Update.results = new RollDown.RequestResult[](2);
        l2Update.results[0] = RollDown.RequestResult({
            requestId: RollDown.RequestId({id: 1, origin: RollDown.Origin.L2}),
            originRequestId: 1,
            updateType: RollDown.UpdateType.DEPOSIT,
            status: true
        });
        l2Update.results[1] = RollDown.RequestResult({
            requestId: RollDown.RequestId({id: 2, origin: RollDown.Origin.L2}),
            originRequestId: 2,
            updateType: RollDown.UpdateType.DEPOSIT,
            status: false
        });

        rollDown.update_l1_from_l2(l2Update);

        RollDown.L2Update memory l2Update2;
        l2Update2.withdrawals = new RollDown.Withdrawal[](1);
        l2Update2.withdrawals[0] = RollDown.Withdrawal({
            requestId: RollDown.RequestId({id: 3, origin: RollDown.Origin.L2}),
            withdrawalRecipient: alice,
            tokenAddress: tokenAddress,
            amount: 1000
        });

        l2Update2.results = new RollDown.RequestResult[](2);
        l2Update2.results[0] = RollDown.RequestResult({
            requestId: RollDown.RequestId({id: 1, origin: RollDown.Origin.L2}),
            originRequestId: 1,
            updateType: RollDown.UpdateType.DEPOSIT,
            status: true
        });
        l2Update2.results[1] = RollDown.RequestResult({
            requestId: RollDown.RequestId({id: 2, origin: RollDown.Origin.L2}),
            originRequestId: 2,
            updateType: RollDown.UpdateType.DEPOSIT,
            status: false
        });

        rollDown.update_l1_from_l2(l2Update2);
    }

    function testReproduce2() public {
        // Arrange
        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 1000;
        deal(tokenAddress, alice, 2 * amount);
        vm.startPrank(alice);
        token.approve(address(rollDown), 2 * amount);
        rollDown.deposit(tokenAddress, amount);
        rollDown.deposit(tokenAddress, amount);
        vm.stopPrank();

        RollDown.L2Update memory l2Update;
        l2Update.results = new RollDown.RequestResult[](2);
        l2Update.results[0] = RollDown.RequestResult({
            requestId: RollDown.RequestId({id: 1, origin: RollDown.Origin.L2}),
            originRequestId: 1,
            updateType: RollDown.UpdateType.DEPOSIT,
            status: true
        });
        l2Update.results[1] = RollDown.RequestResult({
            requestId: RollDown.RequestId({id: 2, origin: RollDown.Origin.L2}),
            originRequestId: 2,
            updateType: RollDown.UpdateType.DEPOSIT,
            status: false
        });

        rollDown.update_l1_from_l2(l2Update);

        RollDown.L2Update memory l2Update2;
        l2Update2.withdrawals = new RollDown.Withdrawal[](1);
        l2Update2.withdrawals[0] = RollDown.Withdrawal({
            requestId: RollDown.RequestId({id: 3, origin: RollDown.Origin.L2}),
            withdrawalRecipient: alice,
            tokenAddress: tokenAddress,
            amount: 1000
        });

        l2Update2.results = new RollDown.RequestResult[](1);
        l2Update2.results[0] = RollDown.RequestResult({
            requestId: RollDown.RequestId({id: 4, origin: RollDown.Origin.L2}),
            originRequestId: 3,
            updateType: RollDown.UpdateType.DEPOSIT,
            status: true
        });

        rollDown.update_l1_from_l2(l2Update2);
    }
}

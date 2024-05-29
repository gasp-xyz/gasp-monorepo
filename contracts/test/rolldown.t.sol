pragma solidity ^0.8.9;
import {Rolldown} from "../src/Rolldown.sol";
import {stdStorage, StdStorage, Test} from "forge-std/Test.sol";
import "forge-std/console.sol";
import {Utilities, MyERC20} from "./utils/Utilities.sol";
import {IRolldownPrimitives} from "../src/IRolldownPrimitives.sol";
import "@eigenlayer/contracts/permissions/PauserRegistry.sol";

contract RolldownTest is Test, IRolldownPrimitives {
    using stdStorage for StdStorage;
    Rolldown public rolldown;
    Utilities internal utils;
    address payable[] internal users;
    MyERC20 internal token;
    address payable internal ETH_TOKEN_ADDRESS;

    function setUp() public {
        address payable[] memory admins;

        utils = new Utilities();
        admins = utils.createUsers(1);

        PauserRegistry avsPauserReg;
        address avsOwner = admins[0];
        address unpauseMultisig = avsOwner;
        {
            address[] memory pausers = new address[](2);
            pausers[0] = avsOwner;
            pausers[1] = unpauseMultisig;
            avsPauserReg = new PauserRegistry(
                pausers,
                unpauseMultisig
            );
        }

        users = utils.createUsers(1);
        rolldown = new Rolldown();
        rolldown.initialize(avsPauserReg, avsOwner, ChainId.Ethereum);
        ETH_TOKEN_ADDRESS = payable(0x5748395867463837537395739375937493733457);
    }

    function beforeEach() public {}

    function testExecuteDepositEth() public {
        // Arrange
        address payable alice = users[0];
        uint256 amount = 10;
        address payable tokenAddress = payable(ETH_TOKEN_ADDRESS);
        address payable contract_address = payable(address(rolldown));
        deal(alice, 100 ether);
        uint256 aliceBalanceBefore = alice.balance;
        uint256 contractBalanceBefore = contract_address.balance;

        // Act
        vm.startPrank(alice);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.DepositAcceptedIntoQueue(1, alice, tokenAddress, amount);
        rolldown.deposit_eth{value: amount}();
        vm.stopPrank();

        Rolldown.L1Update memory l1Update = rolldown.getUpdateForL2();
        uint256 aliceBalanceAfter = alice.balance;
        uint256 contractBalanceAfter = contract_address.balance;

        // Assert
        assertEq(l1Update.pendingDeposits.length, 1);
        assertEq(l1Update.pendingCancelResolutions.length, 0);
        assertEq(l1Update.pendingL2UpdatesToRemove.length, 0);
        assertEq(l1Update.pendingDeposits[0].depositRecipient, alice);
        assertEq(l1Update.pendingDeposits[0].tokenAddress, tokenAddress);
        assertEq(l1Update.pendingDeposits[0].amount, amount);
        assertEq(aliceBalanceBefore - aliceBalanceAfter, 10);
        assertEq(contractBalanceAfter - contractBalanceBefore, 10);
    }

    function testExecuteWithdrawEth() public {
        // Arrange
        address payable alice = users[0];
        uint256 amount = 1000;
        address payable tokenAddress = payable(ETH_TOKEN_ADDRESS);
        address payable contract_address = payable(address(rolldown));
        deal(alice, 10000 ether);
        uint256 aliceBalanceBefore = alice.balance;
        uint256 contractBalanceBefore = contract_address.balance;

        // Act
        vm.startPrank(alice);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.DepositAcceptedIntoQueue(1, alice, tokenAddress, amount);
        rolldown.deposit_eth{value: amount}();
        vm.stopPrank();

        Rolldown.L1Update memory l1Update = rolldown.getUpdateForL2();
        uint256 aliceBalanceAfterDeposit = alice.balance;
        uint256 contractBalanceAfterDeposit = contract_address.balance;

        // Assert
        assertEq(l1Update.pendingDeposits.length, 1);
        assertEq(l1Update.pendingCancelResolutions.length, 0);
        assertEq(l1Update.pendingL2UpdatesToRemove.length, 0);
        assertEq(l1Update.pendingDeposits[0].depositRecipient, alice);
        assertEq(l1Update.pendingDeposits[0].tokenAddress, tokenAddress);
        assertEq(l1Update.pendingDeposits[0].amount, amount);
        assertEq(aliceBalanceBefore - aliceBalanceAfterDeposit, amount);
        assertEq(contractBalanceAfterDeposit - contractBalanceBefore, amount);

        uint withdraw_amount = 500;
        Rolldown.L2Update memory l2Update;
        l2Update.results = new Rolldown.RequestResult[](0);
        l2Update.withdrawals = new Rolldown.Withdrawal[](1);
        l2Update.withdrawals[0] = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            withdrawalRecipient: alice,
            tokenAddress: tokenAddress,
            amount: withdraw_amount
        });

        vm.startPrank(alice);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.WithdrawalResolutionAcceptedIntoQueue(2, true);
        vm.expectEmit(true, true, false, true);
        emit IRolldownPrimitives.EthWithdrawPending(alice, 500);
        rolldown.update_l1_from_l2(l2Update);
        vm.stopPrank();

        Rolldown.L1Update memory l1UpdateAfterWithdraw = rolldown.getUpdateForL2();
        assertEq(l1UpdateAfterWithdraw.pendingWithdrawalResolutions.length, 1);
        assertEq(stdstore
                    .target(address(rolldown))
                    .sig(rolldown.pendingEthWithdrawals.selector)
                    .with_key(alice)
                    .read_uint(),
                withdraw_amount);

        uint withdraw_from_pending = 200;

        vm.startPrank(alice);
        vm.expectEmit(true, true, false, true);
        emit IRolldownPrimitives.PendingEthWithdrawn(alice, withdraw_from_pending);
        rolldown.withdraw_pending_eth(withdraw_from_pending);
        vm.stopPrank();

        assertEq(stdstore
                    .target(address(rolldown))
                    .sig(rolldown.pendingEthWithdrawals.selector)
                    .with_key(alice)
                    .read_uint(),
                withdraw_amount - withdraw_from_pending);
        assertEq(alice.balance - aliceBalanceAfterDeposit, withdraw_from_pending);
        assertEq(contractBalanceAfterDeposit - contract_address.balance, withdraw_from_pending);
    }

    function testExecuteDeposit() public {
        // Arrange
        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 10;
        deal(tokenAddress, alice, 100 ether);
        uint256 aliceBalanceBefore = token.balanceOf(alice);
        uint256 contractBalanceBefore = token.balanceOf(address(rolldown));

        // Act
        vm.startPrank(alice);
        token.approve(address(rolldown), amount);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.DepositAcceptedIntoQueue(1, alice, tokenAddress, amount);
        rolldown.deposit_erc20(tokenAddress, 10);
        vm.stopPrank();

        Rolldown.L1Update memory l1Update = rolldown.getUpdateForL2();
        uint256 aliceBalanceAfter = token.balanceOf(alice);
        uint256 contractBalanceAfter = token.balanceOf(address(rolldown));

        // Assert
        assertEq(l1Update.pendingDeposits.length, 1);
        assertEq(l1Update.pendingCancelResolutions.length, 0);
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
        token.approve(address(rolldown), amount);
        rolldown.deposit_erc20(tokenAddress, 10);
        vm.stopPrank();

        Rolldown.L1Update memory l1Update = rolldown.getUpdateForL2();
        assertEq(l1Update.pendingDeposits.length, 1);
        assertEq(l1Update.pendingL2UpdatesToRemove.length, 0);
        assertEq(l1Update.pendingCancelResolutions.length, 0);
        assertEq(l1Update.pendingDeposits[0].requestId.id, 1);

        Rolldown.L2Update memory l2Update;
        l2Update.results = new Rolldown.RequestResult[](1);
        l2Update.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: false
        });

        // Act
        rolldown.update_l1_from_l2(l2Update);
        //
        l1Update = rolldown.getUpdateForL2();
        assertEq(l1Update.pendingL2UpdatesToRemove.length, 1);
        assertEq(l1Update.pendingDeposits.length, 0);
        assertEq(l1Update.pendingCancelResolutions.length, 0);
        assertEq(l1Update.pendingL2UpdatesToRemove[0].requestId.id, 2);
        assertEq(
            l1Update.pendingL2UpdatesToRemove[0].l2UpdatesToRemove.length,
            1
        );
        assertEq(l1Update.pendingL2UpdatesToRemove[0].l2UpdatesToRemove[0], 1);

        Rolldown.L2Update memory l2Update2;
        l2Update2.cancels = new Rolldown.Cancel[](0);
        l2Update2.results = new Rolldown.RequestResult[](1);
        l2Update2.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 2, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 2,
            updateType: IRolldownPrimitives.UpdateType.INDEX_UPDATE,
            status: true
        });
        rolldown.update_l1_from_l2(l2Update2);

        l1Update = rolldown.getUpdateForL2();
        assertEq(l1Update.pendingL2UpdatesToRemove.length, 1);
        assertEq(l1Update.pendingDeposits.length, 0);
        assertEq(l1Update.pendingCancelResolutions.length, 0);
        assertEq(l1Update.pendingL2UpdatesToRemove[0].requestId.id, 3);
        assertEq(
            l1Update.pendingL2UpdatesToRemove[0].l2UpdatesToRemove.length,
            1
        );
        assertEq(l1Update.pendingL2UpdatesToRemove[0].l2UpdatesToRemove[0], 2);

        Rolldown.L2Update memory l2Update3;
        l2Update3.cancels = new Rolldown.Cancel[](0);
        l2Update3.results = new Rolldown.RequestResult[](1);
        l2Update3.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 3, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 3,
            updateType: IRolldownPrimitives.UpdateType.INDEX_UPDATE,
            status: true
        });
        rolldown.update_l1_from_l2(l2Update3);

        // Assert
        l1Update = rolldown.getUpdateForL2();
        assertEq(l1Update.pendingL2UpdatesToRemove.length, 1);
        assertEq(l1Update.pendingDeposits.length, 0);
        assertEq(l1Update.pendingCancelResolutions.length, 0);
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
        token.approve(address(rolldown), amount);
        rolldown.deposit_erc20(tokenAddress, 10);
        vm.stopPrank();

        Rolldown.L2Update memory l2Update;
        l2Update.cancels = new Rolldown.Cancel[](0);
        l2Update.results = new Rolldown.RequestResult[](1);
        l2Update.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: false
        });

        // Act
        // make sure that executing same request does not alter the state
        rolldown.update_l1_from_l2(l2Update);
        vm.expectRevert("Invalid L2Update");
        rolldown.update_l1_from_l2(l2Update);
    }

    function testL1UpdateHashCompatibilityWithMangataNode() public {
        Rolldown.L1Update memory l1Update;
        l1Update.pendingDeposits = new Rolldown.Deposit[](1);
        l1Update.pendingL2UpdatesToRemove = new Rolldown.L2UpdatesToRemove[](1);
        l1Update.pendingCancelResolutions = new Rolldown.CancelResolution[](1);
        l1Update
            .pendingWithdrawalResolutions = new Rolldown.WithdrawalResolution[](
            1
        );

        l1Update.pendingDeposits[0] = IRolldownPrimitives.Deposit({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L1}),
            depositRecipient: 0x0000000000000000000000000000000000000002,
            tokenAddress: 0x0000000000000000000000000000000000000003,
            amount: 4,
            timeStamp: 1
        });

        l1Update.pendingCancelResolutions[0] = IRolldownPrimitives.CancelResolution({
            requestId: IRolldownPrimitives.RequestId({id: 6, origin: IRolldownPrimitives.Origin.L1}),
            l2RequestId: 7,
            cancelJustified: true,
            timeStamp: 2
        });

        l1Update.pendingWithdrawalResolutions[0] = IRolldownPrimitives
            .WithdrawalResolution({
                requestId: IRolldownPrimitives.RequestId({
                    id: 9,
                    origin: IRolldownPrimitives.Origin.L1
                }),
                l2RequestId: 10,
                status: true,
                timeStamp: 3
            });

        uint256[] memory l2UpdatesToRemove = new uint256[](1);
        l2UpdatesToRemove[0] = 13;
        l1Update.pendingL2UpdatesToRemove[0] = IRolldownPrimitives.L2UpdatesToRemove({
            requestId: IRolldownPrimitives.RequestId({id: 12, origin: IRolldownPrimitives.Origin.L1}),
            l2UpdatesToRemove: l2UpdatesToRemove,
            timeStamp: 4
        });

        assertEq(
            keccak256(abi.encode(l1Update)),
            0x6ebab65d2a7e2e2ac74b0415ccb2943ed7818bec57609986ab154b6880311c89
        );
    }

    function testL2UpdateHashCompatibilityWithMangataNode() public {
        // TODO: add such  a test on substrate side
        Rolldown.L2Update memory l2Update;
        l2Update.cancels = new Rolldown.Cancel[](1);
        l2Update.withdrawals = new Rolldown.Withdrawal[](1);
        l2Update.results = new Rolldown.RequestResult[](1);

        l2Update.cancels[0] = IRolldownPrimitives.Cancel({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            range: IRolldownPrimitives.Range({start: 2, end: 3}),
            hash: 0x0000000000000000000000000000000000000000000000000000000000000004
        });

        l2Update.withdrawals[0] = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 5, origin: IRolldownPrimitives.Origin.L2}),
            withdrawalRecipient: 0x0000000000000000000000000000000000000006,
            tokenAddress: 0x0000000000000000000000000000000000000007,
            amount: 8
        });

        l2Update.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 9, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 10,
            updateType: IRolldownPrimitives.UpdateType.INDEX_UPDATE,
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
        token.approve(address(rolldown), amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        vm.stopPrank();

        Rolldown.L2Update memory l2Update;
        l2Update.results = new Rolldown.RequestResult[](0);
        l2Update.cancels = new Rolldown.Cancel[](1);
        l2Update.cancels[0] = IRolldownPrimitives.Cancel({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            range: IRolldownPrimitives.Range({start: 1, end: 1}),
            hash: bytes32(uint256(0))
        });

        vm.startPrank(alice);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.DisputeResolutionAcceptedIntoQueue(1, false);
        rolldown.update_l1_from_l2(l2Update);
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
        token.approve(address(rolldown), amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        vm.stopPrank();

        Rolldown.L1Update memory l1Update = rolldown.getUpdateForL2();

        Rolldown.L2Update memory l2Update;
        l2Update.results = new Rolldown.RequestResult[](0);
        l2Update.cancels = new Rolldown.Cancel[](1);
        l2Update.cancels[0] = IRolldownPrimitives.Cancel({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            range: IRolldownPrimitives.Range({start: 1, end: 1}),
            hash: bytes32(keccak256(abi.encode(l1Update)))
        });

        vm.startPrank(alice);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.DisputeResolutionAcceptedIntoQueue(1, true);
        rolldown.update_l1_from_l2(l2Update);
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
        token.transfer(address(rolldown), amount);
        vm.stopPrank();
        uint256 aliceBalanceBefore = token.balanceOf(alice);
        uint256 contractBalanceBefore = token.balanceOf(address(rolldown));

        Rolldown.L2Update memory l2Update;
        l2Update.results = new Rolldown.RequestResult[](0);
        l2Update.withdrawals = new Rolldown.Withdrawal[](1);
        l2Update.withdrawals[0] = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            withdrawalRecipient: alice,
            tokenAddress: tokenAddress,
            amount: amount
        });

        rolldown.update_l1_from_l2(l2Update);
    }

    function testSuccessfulWithdrawalRequest() public {
        // Arrange
        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 1000;
        deal(tokenAddress, alice, amount);
        vm.startPrank(alice);
        token.approve(address(rolldown), amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        vm.stopPrank();

        Rolldown.L2Update memory l2Update;
        l2Update.results = new Rolldown.RequestResult[](0);
        l2Update.withdrawals = new Rolldown.Withdrawal[](1);
        l2Update.withdrawals[0] = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            withdrawalRecipient: alice,
            tokenAddress: tokenAddress,
            amount: 500
        });

        vm.startPrank(alice);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.WithdrawalResolutionAcceptedIntoQueue(2, true);
        emit IRolldownPrimitives.FundsWithdrawn(alice, tokenAddress, 500);
        rolldown.update_l1_from_l2(l2Update);
        vm.stopPrank();

        Rolldown.L1Update memory l1Update = rolldown.getUpdateForL2();
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
        token.approve(address(rolldown), amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        vm.stopPrank();

        Rolldown.L2Update memory l2Update;
        l2Update.results = new Rolldown.RequestResult[](0);
        l2Update.withdrawals = new Rolldown.Withdrawal[](1);
        l2Update.withdrawals[0] = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            withdrawalRecipient: alice,
            tokenAddress: tokenAddress,
            amount: 1001
        });

        vm.startPrank(alice);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.WithdrawalResolutionAcceptedIntoQueue(2, false);
        rolldown.update_l1_from_l2(l2Update);
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
        token.approve(address(rolldown), 2 * amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        vm.stopPrank();

        Rolldown.L2Update memory l2Update;
        l2Update.results = new Rolldown.RequestResult[](0);
        l2Update.withdrawals = new Rolldown.Withdrawal[](1);
        l2Update.withdrawals[0] = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            withdrawalRecipient: alice,
            tokenAddress: tokenAddress,
            amount: 1000
        });

        rolldown.update_l1_from_l2(l2Update);

        vm.expectRevert("Invalid L2Update");
        rolldown.update_l1_from_l2(l2Update);
    }

    function testDoNotRejectPartialyKnownL2Update() public {
        // Arrange
        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 1000;
        deal(tokenAddress, alice, 2 * amount);
        vm.startPrank(alice);
        token.approve(address(rolldown), 2 * amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        vm.stopPrank();

        Rolldown.L2Update memory l2Update;
        l2Update.results = new Rolldown.RequestResult[](0);
        l2Update.withdrawals = new Rolldown.Withdrawal[](1);
        l2Update.withdrawals[0] = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            withdrawalRecipient: alice,
            tokenAddress: tokenAddress,
            amount: 1000
        });
        rolldown.update_l1_from_l2(l2Update);

        Rolldown.L2Update memory partiallyKnownUpdate;
        partiallyKnownUpdate.results = new Rolldown.RequestResult[](0);
        partiallyKnownUpdate.withdrawals = new Rolldown.Withdrawal[](2);
        partiallyKnownUpdate.withdrawals[0] = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            withdrawalRecipient: alice,
            tokenAddress: tokenAddress,
            amount: 1000
        });
        partiallyKnownUpdate.withdrawals[1] = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 2, origin: IRolldownPrimitives.Origin.L2}),
            withdrawalRecipient: alice,
            tokenAddress: tokenAddress,
            amount: 1000
        });

        rolldown.update_l1_from_l2(partiallyKnownUpdate);
    }

    function testAcceptConsecutiveUpdates() public {
        // Arrange
        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 1000;
        deal(tokenAddress, alice, 2 * amount);
        vm.startPrank(alice);
        token.approve(address(rolldown), 2 * amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        vm.stopPrank();

        Rolldown.L2Update memory l2Update;
        l2Update.results = new Rolldown.RequestResult[](2);
        l2Update.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: true
        });
        l2Update.results[1] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 2, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 2,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: false
        });

        rolldown.update_l1_from_l2(l2Update);

        Rolldown.L2Update memory l2Update2;
        l2Update2.withdrawals = new Rolldown.Withdrawal[](1);
        l2Update2.withdrawals[0] = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 3, origin: IRolldownPrimitives.Origin.L2}),
            withdrawalRecipient: alice,
            tokenAddress: tokenAddress,
            amount: 1000
        });

        l2Update2.results = new Rolldown.RequestResult[](1);
        l2Update2.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 4, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 3,
            updateType: IRolldownPrimitives.UpdateType.INDEX_UPDATE,
            status: true
        });

        rolldown.update_l1_from_l2(l2Update2);
    }

    function testOverlapping() public {
        // Arrange
        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 1000;
        deal(tokenAddress, alice, 2 * amount);
        vm.startPrank(alice);
        token.approve(address(rolldown), 2 * amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        vm.stopPrank();

        Rolldown.L2Update memory l2Update;
        l2Update.results = new Rolldown.RequestResult[](2);
        l2Update.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: true
        });
        l2Update.results[1] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 2, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 2,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: false
        });

        rolldown.update_l1_from_l2(l2Update);

        Rolldown.L2Update memory l2Update2;
        l2Update2.withdrawals = new Rolldown.Withdrawal[](1);
        l2Update2.withdrawals[0] = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 3, origin: IRolldownPrimitives.Origin.L2}),
            withdrawalRecipient: alice,
            tokenAddress: tokenAddress,
            amount: 1000
        });

        l2Update2.results = new Rolldown.RequestResult[](2);
        l2Update2.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: true
        });
        l2Update2.results[1] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 2, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 2,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: false
        });

        rolldown.update_l1_from_l2(l2Update2);
    }

    function testWithdrawalAlongIndexUpdateWhenWithdrawalProcessedBeforeIndexUpdate()
        public
    {
        // Arrange
        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 1000;
        deal(tokenAddress, alice, 2 * amount);
        vm.startPrank(alice);
        token.approve(address(rolldown), 2 * amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        vm.stopPrank();

        Rolldown.L2Update memory l2Update;
        l2Update.results = new Rolldown.RequestResult[](2);
        l2Update.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: true
        });
        l2Update.results[1] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 2, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 2,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: false
        });

        rolldown.update_l1_from_l2(l2Update);

        Rolldown.L2Update memory l2Update2;
        l2Update2.withdrawals = new Rolldown.Withdrawal[](1);
        l2Update2.withdrawals[0] = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 3, origin: IRolldownPrimitives.Origin.L2}),
            withdrawalRecipient: alice,
            tokenAddress: tokenAddress,
            amount: 1000
        });

        l2Update2.results = new Rolldown.RequestResult[](1);
        l2Update2.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 4, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 3,
            updateType: IRolldownPrimitives.UpdateType.INDEX_UPDATE,
            status: true
        });

        rolldown.update_l1_from_l2(l2Update2);
        Rolldown.L1Update memory update = rolldown.getUpdateForL2();
        assertEq(update.pendingWithdrawalResolutions.length, 1);
        assert(
            update.pendingWithdrawalResolutions[0].requestId.origin ==
                IRolldownPrimitives.Origin.L1
        );
        assertEq(update.pendingWithdrawalResolutions[0].requestId.id, 4);
    }

    function testWithdrawalAlongIndexUpdateWhenWithdrawalProcessedAfterIndexUpdate()
        public
    {
        // Arrange
        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 1000;
        deal(tokenAddress, alice, 2 * amount);
        vm.startPrank(alice);
        token.approve(address(rolldown), 2 * amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        vm.stopPrank();

        Rolldown.L2Update memory l2Update;
        l2Update.results = new Rolldown.RequestResult[](2);
        l2Update.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: true
        });
        l2Update.results[1] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 2, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 2,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: false
        });

        rolldown.update_l1_from_l2(l2Update);

        Rolldown.L2Update memory l2Update2;
        l2Update2.withdrawals = new Rolldown.Withdrawal[](1);
        l2Update2.withdrawals[0] = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 4, origin: IRolldownPrimitives.Origin.L2}),
            withdrawalRecipient: alice,
            tokenAddress: tokenAddress,
            amount: 1000
        });

        l2Update2.results = new Rolldown.RequestResult[](1);
        l2Update2.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 3, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 3,
            updateType: IRolldownPrimitives.UpdateType.INDEX_UPDATE,
            status: true
        });

        rolldown.update_l1_from_l2(l2Update2);
        Rolldown.L1Update memory update = rolldown.getUpdateForL2();
        assertEq(update.pendingWithdrawalResolutions.length, 1);
        assert(
            update.pendingWithdrawalResolutions[0].requestId.origin ==
                IRolldownPrimitives.Origin.L1
        );
        assertEq(update.pendingWithdrawalResolutions[0].requestId.id, 4);
    }

    function testReproduceWithdrawalHandling() public {
        // Arrange
        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 1000;
        deal(tokenAddress, alice, 2 * amount);
        vm.startPrank(alice);
        token.approve(address(rolldown), 2 * amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        vm.stopPrank();

        Rolldown.L2Update memory l2Update;
        l2Update.results = new Rolldown.RequestResult[](2);
        l2Update.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: true
        });
        l2Update.results[1] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 2, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 2,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: false
        });
        rolldown.update_l1_from_l2(l2Update);

        Rolldown.L2Update memory l2Update2;
        l2Update2.results = new Rolldown.RequestResult[](1);
        l2Update2.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 3, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 3,
            updateType: IRolldownPrimitives.UpdateType.INDEX_UPDATE,
            status: true
        });
        rolldown.update_l1_from_l2(l2Update2);

        Rolldown.L2Update memory l2Update3;
        l2Update3.results = new Rolldown.RequestResult[](1);
        l2Update3.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 3, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 3,
            updateType: IRolldownPrimitives.UpdateType.INDEX_UPDATE,
            status: true
        });
        l2Update3.withdrawals = new Rolldown.Withdrawal[](1);
        l2Update3.withdrawals[0] = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 4, origin: IRolldownPrimitives.Origin.L2}),
            withdrawalRecipient: alice,
            tokenAddress: tokenAddress,
            amount: 1000
        });
        rolldown.update_l1_from_l2(l2Update3);

        Rolldown.L2Update memory l2Update4;
        l2Update4.results = new Rolldown.RequestResult[](2);
        l2Update4.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 5, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 4,
            updateType: IRolldownPrimitives.UpdateType.INDEX_UPDATE,
            status: true
        });
        l2Update4.results[1] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 6, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 5,
            updateType: IRolldownPrimitives.UpdateType.WITHDRAWAL_RESOLUTION,
            status: true
        });
        rolldown.update_l1_from_l2(l2Update4);
    }

    function testReproduceWithdrawalHandling2() public {
        // Arrange
        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 1000;
        deal(tokenAddress, alice, 2 * amount);
        vm.startPrank(alice);
        token.approve(address(rolldown), 2 * amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        vm.stopPrank();

        Rolldown.L2Update memory l2Update;
        l2Update.results = new Rolldown.RequestResult[](2);
        l2Update.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: true
        });
        l2Update.results[1] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 2, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 2,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: false
        });
        rolldown.update_l1_from_l2(l2Update);

        Rolldown.L2Update memory l2Update2;
        l2Update2.results = new Rolldown.RequestResult[](2);
        l2Update2.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: true
        });
        l2Update2.results[1] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 2, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 2,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: false
        });
        l2Update2.withdrawals = new Rolldown.Withdrawal[](1);
        l2Update2.withdrawals[0] = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 3, origin: IRolldownPrimitives.Origin.L2}),
            withdrawalRecipient: alice,
            tokenAddress: tokenAddress,
            amount: 1000
        });
        rolldown.update_l1_from_l2(l2Update2);

        Rolldown.L2Update memory l2Update3;
        l2Update3.results = new Rolldown.RequestResult[](2);
        l2Update3.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 4, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 3,
            updateType: IRolldownPrimitives.UpdateType.WITHDRAWAL_RESOLUTION,
            status: true
        });
        l2Update3.results[1] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 5, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 2,
            updateType: IRolldownPrimitives.UpdateType.INDEX_UPDATE,
            status: false
        });
        rolldown.update_l1_from_l2(l2Update3);
    }

    function testEveryRequestResultIsIncludedInSingleL2UpdatesToRemoveForOverlappingL2Updates()
        public
    {
        // Arrange
        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 1000;
        deal(tokenAddress, alice, 2 * amount);
        vm.startPrank(alice);
        token.approve(address(rolldown), 2 * amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        vm.stopPrank();

        Rolldown.L2Update memory l2Update;
        l2Update.results = new Rolldown.RequestResult[](1);
        l2Update.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: true
        });
        rolldown.update_l1_from_l2(l2Update);
        Rolldown.L1Update memory l1Update = rolldown.getUpdateForL2();
        assertEq(l1Update.pendingL2UpdatesToRemove.length, 1);
        assertEq(
            l1Update.pendingL2UpdatesToRemove[0].l2UpdatesToRemove.length,
            1
        );
        assertEq(l1Update.pendingL2UpdatesToRemove[0].l2UpdatesToRemove[0], 1);

        Rolldown.L2Update memory l2Update2;
        l2Update2.results = new Rolldown.RequestResult[](2);
        l2Update2.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: true
        });
        l2Update2.results[1] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 2, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 2,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: false
        });
        rolldown.update_l1_from_l2(l2Update2);
        rolldown.getUpdateForL2();

        l1Update = rolldown.getUpdateForL2();
        assertEq(l1Update.pendingL2UpdatesToRemove.length, 2);
        assertEq(
            l1Update.pendingL2UpdatesToRemove[0].l2UpdatesToRemove.length,
            1
        );
        assertEq(l1Update.pendingL2UpdatesToRemove[0].l2UpdatesToRemove[0], 1);

        assertEq(
            l1Update.pendingL2UpdatesToRemove[1].l2UpdatesToRemove.length,
            1
        );
        assertEq(l1Update.pendingL2UpdatesToRemove[1].l2UpdatesToRemove[0], 2);
    }

    function testUpdateWithWithdrawalAndRequestResult() public {
        // Arrange
        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 1000;
        deal(tokenAddress, alice, 2 * amount);
        vm.startPrank(alice);
        token.approve(address(rolldown), 2 * amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        vm.stopPrank();

        Rolldown.L2Update memory l2Update;
        l2Update.results = new Rolldown.RequestResult[](2);
        l2Update.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: true
        });
        l2Update.results[1] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 2, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 2,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: false
        });
        rolldown.update_l1_from_l2(l2Update);

        Rolldown.L2Update memory l2Update2;
        l2Update2.results = new Rolldown.RequestResult[](2);
        l2Update2.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 3, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: true
        });
        l2Update2.results[1] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 5, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: true
        });

        l2Update2.withdrawals = new Rolldown.Withdrawal[](1);
        l2Update2.withdrawals[0] = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 4, origin: IRolldownPrimitives.Origin.L2}),
            withdrawalRecipient: 0x0000000000000000000000000000000000000006,
            tokenAddress: tokenAddress,
            amount: 8
        });
        rolldown.update_l1_from_l2(l2Update2);
    }

    function testNonsuccessfullDepositHandling() public {
        // Arrange
        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 10;
        deal(tokenAddress, alice, 100 ether);
        uint256 aliceBalanceBefore = token.balanceOf(alice);
        uint256 contractBalanceBefore = token.balanceOf(address(rolldown));

        // Act
        vm.startPrank(alice);
        token.approve(address(rolldown), amount);
        rolldown.deposit(tokenAddress, 10);
        uint256 aliceBalanceAfterDeposit = token.balanceOf(alice);
        uint256 contractAfterDeposit = token.balanceOf(address(rolldown));

        Rolldown.L2Update memory l2Update;
        l2Update.results = new Rolldown.RequestResult[](1);
        l2Update.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: false
        });
        rolldown.update_l1_from_l2(l2Update);
        uint256 aliceBalanceAfterDepositUpdate = token.balanceOf(alice);
        uint256 contractAfterDepositUpdate = token.balanceOf(address(rolldown));
        vm.stopPrank();

        assertEq(aliceBalanceBefore - aliceBalanceAfterDeposit, 10);
        assertEq(contractAfterDeposit - contractBalanceBefore, 10);
        assertEq(aliceBalanceBefore - aliceBalanceAfterDepositUpdate, 0);
        assertEq(contractBalanceBefore - contractAfterDepositUpdate, 0);
    }

    function testSuccessfullDepositHandling() public {
        // Arrange
        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 10;
        deal(tokenAddress, alice, 100 ether);
        uint256 aliceBalanceBefore = token.balanceOf(alice);
        uint256 contractBalanceBefore = token.balanceOf(address(rolldown));

        // Act
        vm.startPrank(alice);
        token.approve(address(rolldown), amount);
        rolldown.deposit(tokenAddress, 10);
        uint256 aliceBalanceAfterDeposit = token.balanceOf(alice);
        uint256 contractAfterDeposit = token.balanceOf(address(rolldown));

        Rolldown.L2Update memory l2Update;
        l2Update.results = new Rolldown.RequestResult[](1);
        l2Update.results[0] = IRolldownPrimitives.RequestResult({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            updateType: IRolldownPrimitives.UpdateType.DEPOSIT,
            status: true
        });
        rolldown.update_l1_from_l2(l2Update);
        uint256 aliceBalanceAfterDepositUpdate = token.balanceOf(alice);
        uint256 contractAfterDepositUpdate = token.balanceOf(address(rolldown));
        vm.stopPrank();

        assertEq(aliceBalanceBefore - aliceBalanceAfterDeposit, 10);
        assertEq(contractAfterDeposit - contractBalanceBefore, 10);
        assertEq(aliceBalanceBefore - aliceBalanceAfterDepositUpdate, 10);
        assertEq(contractAfterDepositUpdate - contractBalanceBefore, 10);
    }
}

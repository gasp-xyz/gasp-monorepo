// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import "@eigenlayer/contracts/permissions/PauserRegistry.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "forge-std/console.sol";
import "forge-std/StdJson.sol";
import {stdStorage, StdStorage, Test} from "forge-std/Test.sol";
import {IRolldownPrimitives} from "../src/IRolldownPrimitives.sol";
import {Rolldown} from "../src/Rolldown.sol";
import {LMerkleTree} from "../src/LMerkleTree.sol";
import {Utilities, MyERC20} from "./utils/Utilities.sol";

contract RolldownTest is Test, IRolldownPrimitives {
    using stdStorage for StdStorage;

    Rolldown public rolldown;
    Utilities internal utils;
    address payable[] internal users;
    address payable ALICE;
    address payable BOB;
    address payable CHARLIE;
    MyERC20 internal token;
    address payable internal NATIVE_TOKEN_ADDRESS;

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
            avsPauserReg = new PauserRegistry(pausers, unpauseMultisig);
        }

        users = utils.createUsers(3);
        ALICE = users[0];
        BOB = users[1];
        CHARLIE = users[2];
        rolldown = new Rolldown();
        rolldown.initialize(avsPauserReg, avsOwner, ChainId.Ethereum, users[0]);
        NATIVE_TOKEN_ADDRESS = payable(0x0000000000000000000000000000000000000001);
        token = new MyERC20();
    }

    function beforeEach() public {}

    function testExecuteDepositEth() public {
        // Arrange
        address payable alice = users[0];
        uint256 amount = 10;
        uint256 fee = 0;
        address payable tokenAddress = payable(NATIVE_TOKEN_ADDRESS);
        address payable contract_address = payable(address(rolldown));
        deal(alice, 100 ether);
        uint256 aliceBalanceBefore = alice.balance;
        uint256 contractBalanceBefore = contract_address.balance;

        // Act
        vm.startPrank(alice);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.DepositAcceptedIntoQueue(1, alice, tokenAddress, amount, fee);
        rolldown.deposit_native{value: amount}();
        vm.stopPrank();

        Rolldown.L1Update memory l1Update = rolldown.getUpdateForL2();
        uint256 aliceBalanceAfter = alice.balance;
        uint256 contractBalanceAfter = contract_address.balance;

        // Assert
        assertEq(l1Update.pendingDeposits.length, 1);
        assertEq(l1Update.pendingCancelResolutions.length, 0);
        assertEq(l1Update.pendingDeposits[0].depositRecipient, alice);
        assertEq(l1Update.pendingDeposits[0].tokenAddress, tokenAddress);
        assertEq(l1Update.pendingDeposits[0].amount, amount);
        assertEq(aliceBalanceBefore - aliceBalanceAfter, 10);
        assertEq(contractBalanceAfter - contractBalanceBefore, 10);
    }

    function deposit_native_emits_event() public {
        address payable alice = users[0];
        uint256 amount = 1000;
        uint256 fee = 0;
        address payable tokenAddress = payable(NATIVE_TOKEN_ADDRESS);

        // Act
        vm.startPrank(alice);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.DepositAcceptedIntoQueue(1, alice, tokenAddress, amount, fee);
        rolldown.deposit_native{value: amount}();
        vm.stopPrank();
    }

    function deposit_erc20_emits_event() public {
        address payable alice = users[0];
        uint256 amount = 1000;
        address payable tokenAddress = payable(NATIVE_TOKEN_ADDRESS);
        uint256 fee = 0;

        // Act
        vm.startPrank(alice);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.DepositAcceptedIntoQueue(1, alice, tokenAddress, amount, fee);
        rolldown.deposit_erc20(tokenAddress, amount);
        vm.stopPrank();
    }

    function testUpdateL1FromL2TriggersEvent() public {
        address recipient = 0x0000000000000000000000000000000000000006;
        uint256 amount = 123456;
        token.mint(address(rolldown));

        Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            recipient: recipient,
            tokenAddress: address(token),
            amount: amount,
            ferryTip: 0
        });

        bytes32 merkleRoot = keccak256(abi.encode(withdrawal));

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.L2UpdateAccepted(merkleRoot, Range({start: 1, end: 1}));
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkleRoot, range);
        vm.stopPrank();
    }

    function testExecuteWithdrawErc20() public {
        address recipient = 0x0000000000000000000000000000000000000006;
        uint256 amount = 123456;
        token.mint(address(rolldown));

        Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            recipient: recipient,
            tokenAddress: address(token),
            amount: amount,
            ferryTip: 0
        });

        vm.startPrank(ALICE);
        // merkleRoot of tree with single element is just that single element
        bytes32 merkleRoot = rolldown.hashWithdrawal(withdrawal);
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkleRoot, range);
        vm.stopPrank();

        bytes32[] memory proofs = new bytes32[](0);
        assertEq(token.balanceOf(recipient), 0);

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.ERC20TokensWithdrawn(recipient, address(token), amount);
        emit IRolldownPrimitives.WithdrawalClosed(1, keccak256(abi.encode(withdrawal)));
        rolldown.close_withdrawal(withdrawal, merkleRoot, proofs);
        vm.stopPrank();
        assertEq(token.balanceOf(recipient), amount);
    }

    function testExecuteWithdrawErc20WithWrongHash() public {
        address recipient = 0x0000000000000000000000000000000000000006;
        uint256 amount = 123456;
        token.mint(address(rolldown));

        Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            recipient: recipient,
            tokenAddress: address(token),
            amount: amount,
            ferryTip: 0
        });

        vm.startPrank(ALICE);
        // merkleRoot of tree with single element is just that single element
        bytes32 merkleRoot = keccak256(abi.encode(withdrawal));
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkleRoot, range);
        vm.stopPrank();

        bytes32[] memory proofs = new bytes32[](0);
        withdrawal.amount += 1;

        vm.startPrank(ALICE);
        vm.expectRevert(abi.encodeWithSelector(IRolldownPrimitives.InvalidRequestProof.selector, merkleRoot));
        rolldown.close_withdrawal(withdrawal, merkleRoot, proofs);
        vm.stopPrank();
    }

    function testExecuteDeposit() public {
        // Arrange
        address payable alice = users[0];
        token = new MyERC20();
        address tokenAddress = address(token);
        uint256 amount = 10;
        uint256 fee = 0;
        deal(tokenAddress, alice, 100 ether);
        uint256 aliceBalanceBefore = token.balanceOf(alice);
        uint256 contractBalanceBefore = token.balanceOf(address(rolldown));

        // Act
        vm.startPrank(alice);
        token.approve(address(rolldown), amount);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.DepositAcceptedIntoQueue(1, alice, tokenAddress, amount, fee);
        rolldown.deposit_erc20(tokenAddress, 10);
        vm.stopPrank();

        Rolldown.L1Update memory l1Update = rolldown.getUpdateForL2();
        uint256 aliceBalanceAfter = token.balanceOf(alice);
        uint256 contractBalanceAfter = token.balanceOf(address(rolldown));

        // Assert
        assertEq(l1Update.pendingDeposits.length, 1);
        assertEq(l1Update.pendingCancelResolutions.length, 0);
        assertEq(l1Update.pendingDeposits[0].depositRecipient, alice);
        assertEq(l1Update.pendingDeposits[0].tokenAddress, tokenAddress);
        assertEq(l1Update.pendingDeposits[0].amount, amount);
        assertEq(aliceBalanceBefore - aliceBalanceAfter, 10);
        assertEq(contractBalanceAfter - contractBalanceBefore, 10);
    }

    function testAcceptUpdateWithMultipleWithdrawals() public {
        uint256 amount = 12345;
        token.mint(ALICE);

        token.mint(address(rolldown));
        vm.startPrank(ALICE);
        token.approve(address(rolldown), amount);
        rolldown.deposit_erc20(address(token), amount);
        vm.stopPrank();

        L1Update memory l1Update = rolldown.getPendingRequests(1, 1);

        Withdrawal memory withdrawalBob1 = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            recipient: BOB,
            tokenAddress: address(token),
            amount: amount,
            ferryTip: 0
        });

        Withdrawal memory withdrawalBob2 = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 2, origin: IRolldownPrimitives.Origin.L2}),
            recipient: BOB,
            tokenAddress: address(token),
            amount: amount,
            ferryTip: 0
        });

        Withdrawal memory withdrawalCharlie = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 3, origin: IRolldownPrimitives.Origin.L2}),
            recipient: CHARLIE,
            tokenAddress: address(token),
            amount: amount,
            ferryTip: 0
        });

        Cancel memory cancel = IRolldownPrimitives.Cancel({
            requestId: IRolldownPrimitives.RequestId({id: 4, origin: IRolldownPrimitives.Origin.L1}),
            range: Range({start: 1, end: 1}),
            hash: keccak256(abi.encode(l1Update))
        });

        //                             hash(hash(0, 1), hash(2, 3)) // AKA Merkle Root
        //                     /                                        \
        //                    /                                          \
        //                   /                                            \
        //               hash(0, 1)                                  hash(2, 3)
        //           /             \                             /               \
        //          /               \                           /                 \
        //         /                 \                         /                   \
        //       (0)                 (1)                      (2)                  (3)
        // withdrawalBob1     withdrawalBob2            withdrawalCharlie         cancel

        // manualy compure merkle root
        bytes32 node_0 = rolldown.hashWithdrawal(withdrawalBob1);
        bytes32 node_1 = rolldown.hashWithdrawal(withdrawalBob2);
        bytes32 node_2 = rolldown.hashWithdrawal(withdrawalCharlie);
        bytes32 node_3 = rolldown.hashCancel(cancel);

        bytes32 node_01 = keccak256(abi.encodePacked(node_0, node_1));
        bytes32 node_23 = keccak256(abi.encodePacked(node_2, node_3));

        bytes32 merkleRoot = keccak256(abi.encodePacked(node_01, node_23));

        // manualy compure merkle proofs
        bytes32[] memory proof_withdrawalBob1 = new bytes32[](2);
        proof_withdrawalBob1[0] = node_1;
        proof_withdrawalBob1[1] = node_23;

        bytes32[] memory proof_withdrawalBob2 = new bytes32[](2);
        proof_withdrawalBob2[0] = node_0;
        proof_withdrawalBob2[1] = node_23;

        bytes32[] memory proof_withdrawalCharlie = new bytes32[](2);
        proof_withdrawalCharlie[0] = node_3;
        proof_withdrawalCharlie[1] = node_01;

        vm.startPrank(ALICE);
        rolldown.update_l1_from_l2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 4}));
        vm.stopPrank();

        vm.startPrank(CHARLIE);
        vm.expectRevert(abi.encodeWithSelector(IRolldownPrimitives.InvalidRequestProof.selector, merkleRoot));
        rolldown.close_withdrawal(withdrawalBob1, merkleRoot, proof_withdrawalCharlie);
        vm.stopPrank();

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.ERC20TokensWithdrawn(CHARLIE, address(token), 12345);
        rolldown.close_withdrawal(withdrawalCharlie, merkleRoot, proof_withdrawalCharlie);
        vm.stopPrank();

        vm.startPrank(CHARLIE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.ERC20TokensWithdrawn(BOB, address(token), 12345);
        rolldown.close_withdrawal(withdrawalBob1, merkleRoot, proof_withdrawalBob1);
        vm.stopPrank();

        vm.startPrank(CHARLIE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.DisputeResolutionAcceptedIntoQueue(
            cancel.requestId.id, false, rolldown.hashCancel(cancel)
        );
        rolldown.close_cancel(cancel, merkleRoot, toBytes32Array([node_2, node_01]));
        vm.stopPrank();
    }

    function testCloseSameWithdrawalTwiceFails() public {
        address recipient = 0x0000000000000000000000000000000000000006;
        uint256 amount = 123456;
        token.mint(address(rolldown));

        Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            recipient: recipient,
            tokenAddress: address(token),
            amount: amount,
            ferryTip: 0
        });

        vm.startPrank(ALICE);
        // merkleRoot of tree with single element is just that single element
        bytes32 merkleRoot = rolldown.hashWithdrawal(withdrawal);
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkleRoot, range);
        vm.stopPrank();

        bytes32[] memory proofs = new bytes32[](0);
        rolldown.close_withdrawal(withdrawal, merkleRoot, proofs);

        vm.startPrank(ALICE);
        vm.expectRevert(abi.encodeWithSelector(IRolldownPrimitives.L2RequestAlreadyProcessed.selector, merkleRoot));
        rolldown.close_withdrawal(withdrawal, merkleRoot, proofs);
        vm.stopPrank();

        assertEq(token.balanceOf(recipient), 123456);
        assertEq(rolldown.lastProcessedUpdate_origin_l2(), 1);
        address status = rolldown.processedL2Requests(merkleRoot);
        assertTrue(status == rolldown.CLOSED());
    }

    function testCancelWithNonMatchingHashResultsWithUnjustifiedStatus() public {
        uint256 amount = 123456;
        token.mint(ALICE);

        Cancel memory cancel = IRolldownPrimitives.Cancel({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            range: IRolldownPrimitives.Range({start: 1, end: 1}),
            hash: 0x0000000000000000000000000000000000000000000000000000000000000000
        });

        vm.startPrank(ALICE);
        token.approve(address(rolldown), amount);
        rolldown.deposit_erc20(address(token), amount);
        vm.stopPrank();

        vm.startPrank(ALICE);
        // merkleRoot of tree with single element is just that single element
        bytes32 merkleRoot = rolldown.hashCancel(cancel);
        bytes32[] memory proofs = new bytes32[](0);
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkleRoot, range);
        vm.stopPrank();

        L1Update memory l1UpdateBefore = rolldown.getPendingRequests(1, 1);
        assertEq(l1UpdateBefore.pendingDeposits.length, 1);
        assertEq(l1UpdateBefore.pendingCancelResolutions.length, 0);

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.DisputeResolutionAcceptedIntoQueue(1, true, rolldown.hashCancel(cancel));
        rolldown.close_cancel(cancel, merkleRoot, proofs);
        vm.stopPrank();

        //validate pendingL2requests
        L1Update memory l1UpdateAfter = rolldown.getPendingRequests(1, 2);
        assertEq(l1UpdateAfter.pendingDeposits.length, 1);
        assertEq(l1UpdateAfter.pendingCancelResolutions.length, 1);
        assertEq(l1UpdateAfter.pendingCancelResolutions[0].l2RequestId, 1);
        assertEq(l1UpdateAfter.pendingCancelResolutions[0].cancelJustified, true);
        address status = rolldown.processedL2Requests(merkleRoot);
        assertTrue(status == rolldown.CLOSED());
    }

    function testCancelResolutionWithMatchingHashResultsWithJustifiedStatus() public {
        uint256 amount = 123456;
        token.mint(ALICE);

        vm.startPrank(ALICE);
        token.approve(address(rolldown), amount);
        rolldown.deposit_erc20(address(token), amount);
        vm.stopPrank();

        L1Update memory l1Update = rolldown.getPendingRequests(1, 1);

        Cancel memory cancel = IRolldownPrimitives.Cancel({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            range: IRolldownPrimitives.Range({start: 1, end: 1}),
            hash: keccak256(abi.encode(l1Update))
        });

        vm.startPrank(ALICE);
        // merkleRoot of tree with single element is just that single element
        bytes32 merkleRoot = rolldown.hashCancel(cancel);
        bytes32[] memory proofs = new bytes32[](0);
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkleRoot, range);
        vm.stopPrank();

        L1Update memory l1UpdateBefore = rolldown.getPendingRequests(1, 1);
        assertEq(l1UpdateBefore.pendingDeposits.length, 1);
        assertEq(l1UpdateBefore.pendingCancelResolutions.length, 0);

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.DisputeResolutionAcceptedIntoQueue(1, false, rolldown.hashCancel(cancel));
        rolldown.close_cancel(cancel, merkleRoot, proofs);
        vm.stopPrank();

        //validate pendingL2requests
        L1Update memory l1UpdateAfter = rolldown.getPendingRequests(1, 2);
        assertEq(l1UpdateAfter.pendingDeposits.length, 1);
        assertEq(l1UpdateAfter.pendingCancelResolutions.length, 1);
        assertEq(l1UpdateAfter.pendingCancelResolutions[0].l2RequestId, 1);
        assertEq(l1UpdateAfter.pendingCancelResolutions[0].cancelJustified, false);
        address status = rolldown.processedL2Requests(merkleRoot);
        assertTrue(status == rolldown.CLOSED());
    }

    function testUnsuccessfulWithdrawalRequest() public {
        address recipient = 0x0000000000000000000000000000000000000006;
        uint256 amount = 123456;

        Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            recipient: recipient,
            tokenAddress: address(token),
            amount: amount,
            ferryTip: 0
        });

        vm.startPrank(ALICE);
        // merkleRoot of tree with single element is just that single element
        bytes32 merkleRoot = rolldown.hashWithdrawal(withdrawal);
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkleRoot, range);
        vm.stopPrank();

        bytes32[] memory proofs = new bytes32[](0);
        assertEq(token.balanceOf(recipient), 0);

        assertEq(rolldown.lastProcessedUpdate_origin_l2(), 1);

        vm.startPrank(ALICE);
        vm.expectRevert("ERC20: transfer amount exceeds balance");
        rolldown.close_withdrawal(withdrawal, merkleRoot, proofs);
        vm.stopPrank();
        assertEq(token.balanceOf(recipient), 0);

        assertEq(rolldown.lastProcessedUpdate_origin_l2(), 1);
        address status = rolldown.processedL2Requests(merkleRoot);
        assertTrue(status != rolldown.CLOSED());
    }

    function testAcceptOnlyConsecutiveUpdatesWithoutGaps() public {
        vm.startPrank(ALICE);
        rolldown.update_l1_from_l2(
            0x0000000000000000000000000000000000000000000000000000000000000000,
            IRolldownPrimitives.Range({start: 1, end: 1})
        );
        rolldown.update_l1_from_l2(
            0x0000000000000000000000000000000000000000000000000000000000000000,
            IRolldownPrimitives.Range({start: 2, end: 2})
        );
        rolldown.update_l1_from_l2(
            0x0000000000000000000000000000000000000000000000000000000000000000,
            IRolldownPrimitives.Range({start: 2, end: 10})
        );
        rolldown.update_l1_from_l2(
            0x0000000000000000000000000000000000000000000000000000000000000000,
            IRolldownPrimitives.Range({start: 9, end: 11})
        );
        rolldown.update_l1_from_l2(
            0x0000000000000000000000000000000000000000000000000000000000000000,
            IRolldownPrimitives.Range({start: 1, end: 12})
        );

        rolldown.update_l1_from_l2(
            0x0000000000000000000000000000000000000000000000000000000000000001,
            IRolldownPrimitives.Range({start: 2, end: 13})
        );
        rolldown.update_l1_from_l2(
            0x0000000000000000000000000000000000000000000000000000000000000002,
            IRolldownPrimitives.Range({start: 12, end: 14})
        );

        vm.stopPrank();
        // validate storage & getters after updates
        uint256 lastId = 14;
        assertEq(rolldown.lastProcessedUpdate_origin_l2(), lastId);
        bytes32 expectedRoot = 0x0000000000000000000000000000000000000000000000000000000000000001;
        (uint256 start, uint256 end) = rolldown.merkleRootRange(expectedRoot);
        assertEq(start, 2);
        assertEq(end, 13);

        //New update win over old updates
        bytes32 batch = rolldown.find_l2_batch(3);
        (start, end) = rolldown.merkleRootRange(batch);
        assertEq(start, 2);
        assertEq(end, 13);

        batch = rolldown.find_l2_batch(12);
        (start, end) = rolldown.merkleRootRange(batch);
        assertEq(start, 12);
        assertEq(end, 14);

        uint256 nonExistingId = 66;
        vm.expectRevert(abi.encodeWithSelector(IRolldownPrimitives.InvalidRequestId.selector, nonExistingId));
        rolldown.find_l2_batch(nonExistingId);
    }

    function testRejectUpdateWithoutNewRequests() public {
        vm.startPrank(ALICE);
        rolldown.update_l1_from_l2(
            0x0000000000000000000000000000000000000000000000000000000000000000,
            IRolldownPrimitives.Range({start: 1, end: 10})
        );
        vm.stopPrank();

        vm.startPrank(ALICE);
        vm.expectRevert(abi.encodeWithSelector(IRolldownPrimitives.UpdateAlreadyApplied.selector, 9, 10));
        rolldown.update_l1_from_l2(
            0x0000000000000000000000000000000000000000000000000000000000000001,
            IRolldownPrimitives.Range({start: 9, end: 9})
        );
        vm.stopPrank();

        vm.startPrank(ALICE);
        vm.expectRevert(abi.encodeWithSelector(IRolldownPrimitives.UpdateAlreadyApplied.selector, 9, 10));
        rolldown.update_l1_from_l2(
            0x0000000000000000000000000000000000000000000000000000000000000002,
            IRolldownPrimitives.Range({start: 1, end: 9})
        );
        vm.stopPrank();

        vm.startPrank(ALICE);
        vm.expectRevert(abi.encodeWithSelector(IRolldownPrimitives.UpdateAlreadyApplied.selector, 10, 10));
        rolldown.update_l1_from_l2(
            0x0000000000000000000000000000000000000000000000000000000000000003,
            IRolldownPrimitives.Range({start: 1, end: 10})
        );
        vm.stopPrank();

        vm.startPrank(ALICE);
        vm.expectRevert(abi.encodeWithSelector(IRolldownPrimitives.UpdateAlreadyApplied.selector, 10, 10));
        rolldown.update_l1_from_l2(
            0x0000000000000000000000000000000000000000000000000000000000000004,
            IRolldownPrimitives.Range({start: 10, end: 10})
        );
        vm.stopPrank();

        vm.startPrank(ALICE);
        vm.expectRevert(abi.encodeWithSelector(IRolldownPrimitives.UpdateAlreadyApplied.selector, 1, 10));
        rolldown.update_l1_from_l2(
            0x0000000000000000000000000000000000000000000000000000000000000005,
            IRolldownPrimitives.Range({start: 1, end: 1})
        );
        vm.stopPrank();
    }

    function testRejectUpdateWithGaps() public {
        vm.startPrank(ALICE);
        rolldown.update_l1_from_l2(
            0x0000000000000000000000000000000000000000000000000000000000000000,
            IRolldownPrimitives.Range({start: 1, end: 10})
        );
        vm.stopPrank();

        vm.startPrank(ALICE);
        vm.expectRevert(abi.encodeWithSelector(IRolldownPrimitives.PreviousUpdateMissed.selector, 12, 10));
        rolldown.update_l1_from_l2(
            0x0000000000000000000000000000000000000000000000000000000000000000,
            IRolldownPrimitives.Range({start: 12, end: 12})
        );
        vm.stopPrank();
    }

    function testVerifyBalancedMerkleRoot() public pure {
        //                                   ROOT
        //                      /                             \
        //                     /                               \
        //            /             \                    /              \
        //           /               \                  /                \
        //      /      \          /     \           /      \          /     \
        //     /        \        /       \         /        \        /       \
        // 0x00..00 0x11..11 0x22..22 0x33..33 0x44..44 0x55..55 0x66..66 0x77.77

        bytes32 rootHash = 0x36e4d0b08be66e3c82af50073c3f2833ac26cb61026c0c16d2d58230d8682998;
        bytes32[] memory proof = new bytes32[](3);

        proof[0] = 0x1111111111111111111111111111111111111111111111111111111111111111;
        proof[1] = 0xf3357627f4934d47fe409005b05c900777a6d97ec3788304e2d9c7b4d322cd4d;
        proof[2] = 0x9aaa5fc7595410234204542d86b85ce74d07a1f98d62d1bbf23f1c2378cc3089;
        assertEq(
            rootHash,
            LMerkleTree.calculateRoot(0x0000000000000000000000000000000000000000000000000000000000000000, 0, proof, 8)
        );

        proof[0] = 0x0000000000000000000000000000000000000000000000000000000000000000;
        proof[1] = 0xf3357627f4934d47fe409005b05c900777a6d97ec3788304e2d9c7b4d322cd4d;
        proof[2] = 0x9aaa5fc7595410234204542d86b85ce74d07a1f98d62d1bbf23f1c2378cc3089;
        assertEq(
            rootHash,
            LMerkleTree.calculateRoot(0x1111111111111111111111111111111111111111111111111111111111111111, 1, proof, 8)
        );

        proof[0] = 0x3333333333333333333333333333333333333333333333333333333333333333;
        proof[1] = 0x8e4b8e18156a1c7271055ce5b7ef53bb370294ebd631a3b95418a92da46e681f;
        proof[2] = 0x9aaa5fc7595410234204542d86b85ce74d07a1f98d62d1bbf23f1c2378cc3089;
        assertEq(
            rootHash,
            LMerkleTree.calculateRoot(0x2222222222222222222222222222222222222222222222222222222222222222, 2, proof, 8)
        );

        proof[0] = 0x2222222222222222222222222222222222222222222222222222222222222222;
        proof[1] = 0x8e4b8e18156a1c7271055ce5b7ef53bb370294ebd631a3b95418a92da46e681f;
        proof[2] = 0x9aaa5fc7595410234204542d86b85ce74d07a1f98d62d1bbf23f1c2378cc3089;
        assertEq(
            rootHash,
            LMerkleTree.calculateRoot(0x3333333333333333333333333333333333333333333333333333333333333333, 3, proof, 8)
        );

        proof[0] = 0x5555555555555555555555555555555555555555555555555555555555555555;
        proof[1] = 0x37df8a86dbd0a06a5a6720079d9a4ce5a5a5c93198607ca71402d78b7db2869e;
        proof[2] = 0xd287edfff411d3b45e9c7bf7186d7e9d44fa2a0fe36d85154165da0a1d7ce5bd;
        assertEq(
            rootHash,
            LMerkleTree.calculateRoot(0x4444444444444444444444444444444444444444444444444444444444444444, 4, proof, 8)
        );

        proof[0] = 0x4444444444444444444444444444444444444444444444444444444444444444;
        proof[1] = 0x37df8a86dbd0a06a5a6720079d9a4ce5a5a5c93198607ca71402d78b7db2869e;
        proof[2] = 0xd287edfff411d3b45e9c7bf7186d7e9d44fa2a0fe36d85154165da0a1d7ce5bd;
        assertEq(
            rootHash,
            LMerkleTree.calculateRoot(0x5555555555555555555555555555555555555555555555555555555555555555, 5, proof, 8)
        );

        proof[0] = 0x7777777777777777777777777777777777777777777777777777777777777777;
        proof[1] = 0x60c25b70d66af589f985b3cf4732585b8f7ecea5df88cb12368650edfe7e6f50;
        proof[2] = 0xd287edfff411d3b45e9c7bf7186d7e9d44fa2a0fe36d85154165da0a1d7ce5bd;
        assertEq(
            rootHash,
            LMerkleTree.calculateRoot(0x6666666666666666666666666666666666666666666666666666666666666666, 6, proof, 8)
        );

        proof[0] = 0x6666666666666666666666666666666666666666666666666666666666666666;
        proof[1] = 0x60c25b70d66af589f985b3cf4732585b8f7ecea5df88cb12368650edfe7e6f50;
        proof[2] = 0xd287edfff411d3b45e9c7bf7186d7e9d44fa2a0fe36d85154165da0a1d7ce5bd;
        assertEq(
            rootHash,
            LMerkleTree.calculateRoot(0x7777777777777777777777777777777777777777777777777777777777777777, 7, proof, 8)
        );
    }

    function testVerifyUnBalancedMerkleRoot1() public pure {
        //                                   ROOT
        //                      /                             \
        //                     /                               \
        //            /             \                    /              \
        //           /               \                  /                \
        //      /      \          /     \           /      \          /     \
        //     /        \        /       \         /        \        /       \
        // 0x00..00 0x11..11 0x22..22 0x33..33 0x44..44 0x55..55 0x66..66
        //
        bytes32 rootHash = 0x018f1011ee4add9a8c0e73b4909158862437ec4aadcc6ca697b357f49425e9ad;
        bytes32[] memory proof = new bytes32[](3);
        proof[0] = 0x60c25b70d66af589f985b3cf4732585b8f7ecea5df88cb12368650edfe7e6f50;
        proof[1] = 0xd287edfff411d3b45e9c7bf7186d7e9d44fa2a0fe36d85154165da0a1d7ce5bd;

        assertEq(
            rootHash,
            LMerkleTree.calculateRoot(0x6666666666666666666666666666666666666666666666666666666666666666, 6, proof, 7)
        );
    }

    function testMerkleProofs() public view {
        string memory config_data = vm.readFile("./test/merkle-verificaction-testdata.json");
        uint256 test_cases_amount = stdJson.readUint(config_data, ".cases_count");

        for (uint256 i = 0; i < test_cases_amount; ++i) {
            console.log("TEST case: ", i);
            uint256 leavePos =
                stdJson.readUint(config_data, string.concat(".cases.[", Strings.toString(i), "].leave_pos"));
            bytes32 leaveHash =
                stdJson.readBytes32(config_data, string.concat(".cases.[", Strings.toString(i), "].leave_hash"));
            bytes32 expectedRoot =
                stdJson.readBytes32(config_data, string.concat(".cases.[", Strings.toString(i), "].expected_root"));
            bytes32[] memory proof =
                stdJson.readBytes32Array(config_data, string.concat(".cases.[", Strings.toString(i), "].proof"));
            bytes32[] memory leaves =
                stdJson.readBytes32Array(config_data, string.concat(".cases.[", Strings.toString(i), "].leaves"));

            assertEq(LMerkleTree.calculateRoot(leaveHash, uint32(leavePos), proof, uint32(leaves.length)), expectedRoot);
        }
    }

    function toBytes32Array(bytes32[2] memory input) internal pure returns (bytes32[] memory) {
        bytes32[] memory result = new bytes32[](2);
        result[0] = input[0];
        result[1] = input[1];
        return result;
    }

    function testRefundFailedERC20DepositWithoutFerryFee() public {
        uint256 amount = 123456;
        token.mint(address(rolldown));
        token.mint(ALICE);

        vm.startPrank(ALICE);
        token.approve(address(rolldown), amount);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.DepositAcceptedIntoQueue(1, ALICE, address(token), amount, 0);
        rolldown.deposit_erc20(address(token), amount);
        vm.stopPrank();

        FailedDepositResolution memory failedDeposit = IRolldownPrimitives.FailedDepositResolution({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            ferry: address(0)
        });

        vm.startPrank(ALICE);
        bytes32 merkleRoot = rolldown.hashFailedDepositResolution(failedDeposit);
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkleRoot, range);
        vm.stopPrank();

        bytes32[] memory proofs = new bytes32[](0);

        uint256 aliceBefore = token.balanceOf(ALICE);

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.ERC20TokensWithdrawn(ALICE, address(token), amount);
        emit IRolldownPrimitives.FailedDepositResolutionClosed(1, 1, merkleRoot);
        rolldown.close_deposit_refund(failedDeposit, merkleRoot, proofs);
        vm.stopPrank();

        uint256 aliceAfter = token.balanceOf(ALICE);
        assertEq(aliceAfter - aliceBefore, amount);
    }

    function testRefundFailedERC20DepositWithFerryFee() public {
        uint256 amount = 123456;
        uint256 fee = 10;
        token.mint(address(rolldown));
        token.mint(ALICE);

        vm.startPrank(ALICE);
        token.approve(address(rolldown), amount);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.DepositAcceptedIntoQueue(1, ALICE, address(token), amount, fee);
        rolldown.deposit_erc20(address(token), amount, fee);
        vm.stopPrank();

        FailedDepositResolution memory failedDeposit = IRolldownPrimitives.FailedDepositResolution({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            ferry: address(0)
        });

        vm.startPrank(ALICE);
        bytes32 merkleRoot = rolldown.hashFailedDepositResolution(failedDeposit);
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkleRoot, range);
        vm.stopPrank();

        bytes32[] memory proofs = new bytes32[](0);

        uint256 aliceBefore = token.balanceOf(ALICE);

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.ERC20TokensWithdrawn(ALICE, address(token), amount);
        emit IRolldownPrimitives.FailedDepositResolutionClosed(1, 1, merkleRoot);
        rolldown.close_deposit_refund(failedDeposit, merkleRoot, proofs);
        vm.stopPrank();

        uint256 aliceAfter = token.balanceOf(ALICE);
        assertEq(aliceAfter - aliceBefore, amount);
    }

    function testRefundFailedERC20FerriedDepositWithoutFerryFee() public {
        address ferry = 0x0000000000000000000000000000000000000006;
        uint256 amount = 123456;
        token.mint(address(rolldown));
        token.mint(ALICE);

        vm.startPrank(ALICE);
        token.approve(address(rolldown), amount);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.DepositAcceptedIntoQueue(1, ALICE, address(token), amount, 0);
        rolldown.deposit_erc20(address(token), amount);
        vm.stopPrank();

        FailedDepositResolution memory failedDeposit = IRolldownPrimitives.FailedDepositResolution({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            ferry: ferry
        });

        vm.startPrank(ALICE);
        bytes32 merkleRoot = rolldown.hashFailedDepositResolution(failedDeposit);
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkleRoot, range);
        vm.stopPrank();

        bytes32[] memory proofs = new bytes32[](0);
        assertEq(token.balanceOf(ferry), 0);

        uint256 aliceBefore = token.balanceOf(ALICE);

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.FailedDepositResolutionClosed(1, 1, merkleRoot);
        rolldown.close_deposit_refund(failedDeposit, merkleRoot, proofs);
        vm.stopPrank();

        assertEq(token.balanceOf(ALICE), aliceBefore);
        assertEq(token.balanceOf(ferry), amount);
    }

    function testRefundFailedERC20FerriedDepositWithFerryFee() public {
        address ferry = 0x0000000000000000000000000000000000000006;
        uint256 amount = 123456;
        uint256 fee = 10;
        token.mint(address(rolldown));
        token.mint(ALICE);

        vm.startPrank(ALICE);
        token.approve(address(rolldown), amount);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.DepositAcceptedIntoQueue(1, ALICE, address(token), amount, fee);
        rolldown.deposit_erc20(address(token), amount, fee);
        vm.stopPrank();

        FailedDepositResolution memory failedDeposit = IRolldownPrimitives.FailedDepositResolution({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            ferry: ferry
        });

        vm.startPrank(ALICE);
        bytes32 merkleRoot = rolldown.hashFailedDepositResolution(failedDeposit);
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkleRoot, range);
        vm.stopPrank();

        bytes32[] memory proofs = new bytes32[](0);

        uint256 aliceBefore = token.balanceOf(ALICE);

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.ERC20TokensWithdrawn(ferry, address(token), amount);
        emit IRolldownPrimitives.FailedDepositResolutionClosed(1, 1, merkleRoot);
        rolldown.close_deposit_refund(failedDeposit, merkleRoot, proofs);
        vm.stopPrank();

        assertEq(token.balanceOf(ALICE), aliceBefore);
        assertEq(token.balanceOf(ferry), amount);
    }

    function testRefundFailedNativeDepositWithoutFerryFee() public {
        uint256 amount = 123456;

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.DepositAcceptedIntoQueue(1, ALICE, NATIVE_TOKEN_ADDRESS, amount, 0);
        rolldown.deposit_native{value: amount}();
        vm.stopPrank();

        FailedDepositResolution memory failedDeposit = IRolldownPrimitives.FailedDepositResolution({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            ferry: address(0)
        });

        vm.startPrank(ALICE);
        bytes32 merkleRoot = rolldown.hashFailedDepositResolution(failedDeposit);
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkleRoot, range);
        vm.stopPrank();

        bytes32[] memory proofs = new bytes32[](0);
        uint256 aliceBefore = ALICE.balance;

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.NativeTokensWithdrawn(ALICE, amount);
        emit IRolldownPrimitives.FailedDepositResolutionClosed(1, 1, merkleRoot);
        rolldown.close_deposit_refund(failedDeposit, merkleRoot, proofs);
        vm.stopPrank();

        assertEq(ALICE.balance - aliceBefore, amount);
    }

    function testRefundFailedNativeDepositWithFerryFee() public {
        uint256 amount = 123456;
        uint256 fee = 10;

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.DepositAcceptedIntoQueue(1, ALICE, NATIVE_TOKEN_ADDRESS, amount, fee);
        rolldown.deposit_native{value: amount}(fee);
        vm.stopPrank();

        FailedDepositResolution memory failedDeposit = IRolldownPrimitives.FailedDepositResolution({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            ferry: address(0)
        });

        vm.startPrank(ALICE);
        bytes32 merkleRoot = rolldown.hashFailedDepositResolution(failedDeposit);
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkleRoot, range);
        vm.stopPrank();

        bytes32[] memory proofs = new bytes32[](0);

        uint256 aliceBefore = ALICE.balance;

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.NativeTokensWithdrawn(ALICE, amount);
        emit IRolldownPrimitives.FailedDepositResolutionClosed(1, 1, merkleRoot);
        rolldown.close_deposit_refund(failedDeposit, merkleRoot, proofs);
        vm.stopPrank();

        assertEq(ALICE.balance - aliceBefore, amount);
    }

    function testRefundFailedNativeFerriedDepositWithoutFerryFee() public {
        address ferry = 0x0000000000000000000000000000000000000006;
        uint256 amount = 123456;

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.DepositAcceptedIntoQueue(1, ALICE, NATIVE_TOKEN_ADDRESS, amount, 0);
        rolldown.deposit_native{value: amount}();
        vm.stopPrank();

        FailedDepositResolution memory failedDeposit = IRolldownPrimitives.FailedDepositResolution({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            ferry: ferry
        });

        vm.startPrank(ALICE);
        bytes32 merkleRoot = rolldown.hashFailedDepositResolution(failedDeposit);
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkleRoot, range);
        vm.stopPrank();

        bytes32[] memory proofs = new bytes32[](0);
        assertEq(ferry.balance, 0);

        uint256 aliceBefore = ALICE.balance;

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.NativeTokensWithdrawn(ferry, amount);
        emit IRolldownPrimitives.FailedDepositResolutionClosed(1, 1, merkleRoot);
        rolldown.close_deposit_refund(failedDeposit, merkleRoot, proofs);
        vm.stopPrank();

        assertEq(ALICE.balance, aliceBefore);
        assertEq(ferry.balance, amount);
    }

    function testRefundFailedNativeFerriedDepositWithFerryFee() public {
        address ferry = 0x0000000000000000000000000000000000000006;
        uint256 amount = 123456;
        uint256 fee = 10;

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.DepositAcceptedIntoQueue(1, ALICE, NATIVE_TOKEN_ADDRESS, amount, fee);
        rolldown.deposit_native{value: amount}(fee);
        vm.stopPrank();

        FailedDepositResolution memory failedDeposit = IRolldownPrimitives.FailedDepositResolution({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            originRequestId: 1,
            ferry: ferry
        });

        vm.startPrank(ALICE);
        bytes32 merkleRoot = rolldown.hashFailedDepositResolution(failedDeposit);
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkleRoot, range);
        vm.stopPrank();

        bytes32[] memory proofs = new bytes32[](0);

        uint256 aliceBefore = ALICE.balance;

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.NativeTokensWithdrawn(ferry, amount);
        emit IRolldownPrimitives.FailedDepositResolutionClosed(1, 1, merkleRoot);
        rolldown.close_deposit_refund(failedDeposit, merkleRoot, proofs);
        vm.stopPrank();

        assertEq(ALICE.balance, aliceBefore);
        assertEq(ferry.balance, amount);
    }

    function testFerryWithdrawalErc20WithTip() public {
        address recipient = 0x0000000000000000000000000000000000000006;
        uint256 amount = 123456;
        uint256 ferryTip = 10;
        token.mint(address(rolldown));
        token.mint(address(ALICE));

        Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            recipient: recipient,
            tokenAddress: address(token),
            amount: amount,
            ferryTip: ferryTip
        });
        bytes32 withdrawalHash = rolldown.hashWithdrawal(withdrawal);

        uint256 ferryBefore = token.balanceOf(ALICE);

        vm.startPrank(ALICE);
        token.approve(address(rolldown), amount - ferryTip);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.WithdrawalFerried(
            withdrawal.requestId.id, amount - ferryTip, recipient, ALICE, withdrawalHash
        );
        rolldown.ferry_withdrawal(withdrawal);
        vm.stopPrank();

        assertEq(token.balanceOf(recipient), amount - ferryTip);
        assertEq(token.balanceOf(ALICE), ferryBefore - amount + ferryTip);

        vm.startPrank(ALICE);
        // merkleRoot of tree with single element is just that single element
        bytes32 merkleRoot = withdrawalHash;
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkleRoot, range);
        vm.stopPrank();

        bytes32[] memory proofs = new bytes32[](0);

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.ERC20TokensWithdrawn(ALICE, address(token), amount);
        emit IRolldownPrimitives.WithdrawalClosed(1, withdrawalHash);
        rolldown.close_withdrawal(withdrawal, merkleRoot, proofs);
        vm.stopPrank();

        assertEq(token.balanceOf(ALICE), ferryBefore + ferryTip);
    }

    function testFerryWithdrawalErc20WithoutTip() public {
        address recipient = 0x0000000000000000000000000000000000000006;
        uint256 amount = 123456;
        token.mint(address(rolldown));
        token.mint(address(ALICE));

        Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            recipient: recipient,
            tokenAddress: address(token),
            amount: amount,
            ferryTip: 0
        });
        bytes32 withdrawalHash = rolldown.hashWithdrawal(withdrawal);

        uint256 ferryBefore = token.balanceOf(ALICE);

        vm.startPrank(ALICE);
        token.approve(address(rolldown), amount);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.WithdrawalFerried(withdrawal.requestId.id, amount, recipient, ALICE, withdrawalHash);
        rolldown.ferry_withdrawal(withdrawal);
        vm.stopPrank();

        assertEq(token.balanceOf(recipient), amount);
        assertEq(token.balanceOf(ALICE), ferryBefore - amount);

        vm.startPrank(ALICE);
        // merkleRoot of tree with single element is just that single element
        bytes32 merkleRoot = withdrawalHash;
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkleRoot, range);
        vm.stopPrank();

        bytes32[] memory proofs = new bytes32[](0);

        // uint256 aliceBefore = token.balanceOf(ALICE);
        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.ERC20TokensWithdrawn(ALICE, address(token), amount);
        emit IRolldownPrimitives.WithdrawalClosed(1, withdrawalHash);
        rolldown.close_withdrawal(withdrawal, merkleRoot, proofs);
        vm.stopPrank();

        assertEq(token.balanceOf(ALICE), ferryBefore);
    }

    function testFerryWithdrawalNativeWithTip() public {
        address recipient = 0x0000000000000000000000000000000000000006;
        uint256 amount = 123456;
        uint256 ferryTip = 10;
        deal(address(rolldown), 999999 ether);
        deal(ALICE, 123466 ether);
        uint256 aliceBefore = ALICE.balance;

        Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            recipient: recipient,
            tokenAddress: NATIVE_TOKEN_ADDRESS,
            amount: amount,
            ferryTip: ferryTip
        });
        bytes32 withdrawalHash = rolldown.hashWithdrawal(withdrawal);

        uint256 ferryBefore = ALICE.balance;

        vm.startPrank(ALICE);
        // token.approve(address(rolldown), amount - ferryTip);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.WithdrawalFerried(
            withdrawal.requestId.id, amount - ferryTip, recipient, ALICE, withdrawalHash
        );
        rolldown.ferry_withdrawal{value: amount - ferryTip}(withdrawal);
        vm.stopPrank();

        assertEq(recipient.balance, amount - ferryTip);
        assertEq(ALICE.balance, ferryBefore - amount + ferryTip);

        vm.startPrank(ALICE);
        // merkleRoot of tree with single element is just that single element
        bytes32 merkleRoot = withdrawalHash;
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkleRoot, range);
        vm.stopPrank();

        bytes32[] memory proofs = new bytes32[](0);

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.NativeTokensWithdrawn(ALICE, amount);
        emit IRolldownPrimitives.FerriedWithdrawalClosed(1, withdrawalHash);
        rolldown.close_withdrawal(withdrawal, merkleRoot, proofs);
        vm.stopPrank();

        assertEq(ALICE.balance, aliceBefore + ferryTip);
    }

    function testFerryWithdrawalNativeWithoutTip() public {
        address recipient = 0x0000000000000000000000000000000000000006;
        uint256 amount = 123456;
        deal(address(rolldown), 999999 ether);
        deal(ALICE, 123466 ether);
        uint256 aliceBefore = ALICE.balance;

        Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            recipient: recipient,
            tokenAddress: NATIVE_TOKEN_ADDRESS,
            amount: amount,
            ferryTip: 0
        });
        bytes32 withdrawalHash = rolldown.hashWithdrawal(withdrawal);

        uint256 ferryBefore = ALICE.balance;

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.WithdrawalFerried(withdrawal.requestId.id, amount, recipient, ALICE, withdrawalHash);
        rolldown.ferry_withdrawal{value: amount}(withdrawal);
        vm.stopPrank();

        assertEq(recipient.balance, amount);
        assertEq(ALICE.balance, ferryBefore - amount);

        vm.startPrank(ALICE);
        // merkleRoot of tree with single element is just that single element
        bytes32 merkleRoot = withdrawalHash;
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkleRoot, range);
        vm.stopPrank();

        bytes32[] memory proofs = new bytes32[](0);

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.NativeTokensWithdrawn(ALICE, amount);
        emit IRolldownPrimitives.FerriedWithdrawalClosed(1, withdrawalHash);
        rolldown.close_withdrawal(withdrawal, merkleRoot, proofs);
        vm.stopPrank();

        assertEq(ALICE.balance, aliceBefore);
    }

    /// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    /// NOTE: Below hash values should not be ever chaned, there are comaptible test implemented in mangata to node
    /// to ensure abi compatibility between L1 & L2
    /// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    function testL1UpdateHashCompatibilityWithMangataNode() public pure {
        Rolldown.L1Update memory l1Update;
        l1Update.chain = ChainId.Ethereum;
        l1Update.pendingDeposits = new Rolldown.Deposit[](1);
        l1Update.pendingCancelResolutions = new Rolldown.CancelResolution[](1);

        l1Update.pendingDeposits[0] = IRolldownPrimitives.Deposit({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L1}),
            depositRecipient: 0x1111111111111111111111111111111111111111,
            tokenAddress: 0x2222222222222222222222222222222222222222,
            amount: 123456,
            timeStamp: 987,
            ferryTip: 321987
        });

        l1Update.pendingCancelResolutions[0] = IRolldownPrimitives.CancelResolution({
            requestId: IRolldownPrimitives.RequestId({id: 123, origin: IRolldownPrimitives.Origin.L1}),
            l2RequestId: 123456,
            cancelJustified: true,
            timeStamp: 987
        });

        uint256[] memory l2UpdatesToRemove = new uint256[](1);
        l2UpdatesToRemove[0] = 13;

        assertEq(keccak256(abi.encode(l1Update)), 0x663fa3ddfe64659f67b2728637936fa8d21f18ef96c07dec110cdd8f45be6fee);
    }

    function testChainWithMangataNode() public pure {
        assertEq(
            keccak256(abi.encode(ChainId.Ethereum)), 0x290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e563
        );

        assertEq(
            keccak256(abi.encode(ChainId.Arbitrum)), 0xb10e2d527612073b26eecdfd717e6a320cf44b4afac2b0732d9fcbe2b7fa0cf6
        );
    }

    function testWithdrawalHash() public pure {
        Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 123, origin: IRolldownPrimitives.Origin.L2}),
            recipient: address(0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF),
            tokenAddress: 0x1f1f1F1f1F1f1f1F1F1F1f1F1f1F1f1F1F1F1F1F,
            amount: 123456,
            ferryTip: 465789
        });

        assertEq(keccak256(abi.encode(withdrawal)), 0xa931da68c445f23b06a72768d07a3513f85c0118ff80f6e284117a221869ae8b);
    }

    function testDepositResolutionHashMatches() public pure {
        FailedDepositResolution memory failedDeposit = FailedDepositResolution({
            requestId: IRolldownPrimitives.RequestId({id: 123, origin: IRolldownPrimitives.Origin.L1}),
            originRequestId: 1234,
            ferry: 0xb5b5B5b5B5b5B5B5B5B5B5b5B5B5B5b5B5B5b5b5
        });

        assertEq(
            keccak256(abi.encode(failedDeposit)), 0xd3def31efb42dd99500c389f59115f0eef5e008db0ee0a81562ef3acbe02eece
        );
    }

    function testCloseFerryableWithdrawalThatWasNotFerriedMoveFerryTipToWhoeverClosesIt() public {
        address recipient = 0x0000000000000000000000000000000000000006;
        uint256 amount = 123456;
        token.mint(address(rolldown));

        Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
            recipient: recipient,
            tokenAddress: address(token),
            amount: amount,
            ferryTip: 23456
        });

        vm.startPrank(ALICE);
        // merkleRoot of tree with single element is just that single element
        bytes32 merkleRoot = rolldown.hashWithdrawal(withdrawal);
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkleRoot, range);
        vm.stopPrank();

        bytes32[] memory proofs = new bytes32[](0);
        uint256 aliceBefore = token.balanceOf(ALICE);
        vm.startPrank(ALICE);
        rolldown.close_withdrawal(withdrawal, merkleRoot, proofs);
        vm.stopPrank();

        assertEq(token.balanceOf(recipient), withdrawal.amount - withdrawal.ferryTip);
        assertEq(token.balanceOf(ALICE) - aliceBefore, withdrawal.ferryTip);
    }
}

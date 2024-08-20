pragma solidity ^0.8.9;
import {Rolldown} from "../src/Rolldown.sol";
import {stdStorage, StdStorage, Test} from "forge-std/Test.sol";
import "forge-std/console.sol";
import "forge-std/StdJson.sol";
import {Utilities, MyERC20} from "./utils/Utilities.sol";
import {IRolldownPrimitives} from "../src/IRolldownPrimitives.sol";
import "@eigenlayer/contracts/permissions/PauserRegistry.sol";
import "@openzeppelin/contracts/utils/Strings.sol";

contract RolldownTest is Test, IRolldownPrimitives {
    using stdStorage for StdStorage;
    Rolldown public rolldown;
    Utilities internal utils;
    address payable[] internal users;
    address payable ALICE;
    address payable BOB;
    address payable CHARLIE;
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

        users = utils.createUsers(3);
        ALICE = users[0];
        BOB = users[1];
        CHARLIE = users[2];
        rolldown = new Rolldown();
        rolldown.initialize(avsPauserReg, avsOwner, ChainId.Ethereum, users[0]);
        ETH_TOKEN_ADDRESS = payable(0x0000000000000000000000000000000000000001);
        token = new MyERC20();
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
        rolldown.deposit_native{value: amount}();
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

    function deposit_native_emits_event() public {
        address payable alice = users[0];
        uint256 amount = 1000;
        address payable tokenAddress = payable(ETH_TOKEN_ADDRESS);

        // Act
        vm.startPrank(alice);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.DepositAcceptedIntoQueue(1, alice, tokenAddress, amount);
        rolldown.deposit_native{value: amount}();
        vm.stopPrank();
    }

    function deposit_erc20_emits_event() public {
        address payable alice = users[0];
        uint256 amount = 1000;
        address payable tokenAddress = payable(ETH_TOKEN_ADDRESS);

        // Act
        vm.startPrank(alice);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.DepositAcceptedIntoQueue(1, alice, tokenAddress, amount);
        rolldown.deposit_erc20(tokenAddress, amount);
        vm.stopPrank();
    }

    function testExecuteWithdrawErc20() public {
        address recipient = 0x0000000000000000000000000000000000000006;
        uint256 amount = 123456;
        token.mint(address(rolldown));

        Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
          requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
          withdrawalRecipient: recipient,
          tokenAddress: address(token),
          amount: amount
        });

        vm.startPrank(ALICE);
        // merkle_root of tree with single element is just that single element
        bytes32 merkle_root = keccak256(abi.encode(withdrawal));
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkle_root, range);
        vm.stopPrank();

        bytes32[] memory proofs = new bytes32[](0);
        assertEq(token.balanceOf(recipient), 0);

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.WithdrawalResolutionAcceptedIntoQueue(1, true);
        emit IRolldownPrimitives.FundsWithdrawn(recipient, address(token), amount);
        rolldown.close_withdrawal(withdrawal, merkle_root, proofs);
        vm.stopPrank();
        assertEq(token.balanceOf(recipient), amount);

        Rolldown.L1Update memory l1Update = rolldown.getUpdateForL2();
        assertEq(l1Update.pendingWithdrawalResolutions.length, 1);
    }

    function testExecuteWithdrawErc20WithWrongHash() public {
        address recipient = 0x0000000000000000000000000000000000000006;
        uint256 amount = 123456;
        token.mint(address(rolldown));

        Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
          requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
          withdrawalRecipient: recipient,
          tokenAddress: address(token),
          amount: amount
        });

        vm.startPrank(ALICE);
        // merkle_root of tree with single element is just that single element
        bytes32 merkle_root = keccak256(abi.encode(withdrawal));
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkle_root, range);
        vm.stopPrank();

        bytes32[] memory proofs = new bytes32[](0);
        withdrawal.amount += 1;

        vm.startPrank(ALICE);
        vm.expectRevert("Invalid proof");
        rolldown.close_withdrawal(withdrawal, merkle_root, proofs);
        vm.stopPrank();
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


     function testAcceptUpdateWithMultipleWithdrawals() public {
       uint256 amount = 12345; 
      token.mint(address(rolldown));

       Withdrawal memory withdrawalBob1 = IRolldownPrimitives.Withdrawal({
         requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
         withdrawalRecipient: BOB,
         tokenAddress: address(token),
         amount: amount
       });

       Withdrawal memory withdrawalBob2 = IRolldownPrimitives.Withdrawal({
         requestId: IRolldownPrimitives.RequestId({id: 2, origin: IRolldownPrimitives.Origin.L2}),
         withdrawalRecipient: BOB,
         tokenAddress: address(token),
         amount: amount
       });

       Withdrawal memory withdrawalCharlie = IRolldownPrimitives.Withdrawal({
         requestId: IRolldownPrimitives.RequestId({id: 3, origin: IRolldownPrimitives.Origin.L2}),
         withdrawalRecipient: CHARLIE,
         tokenAddress: address(token),
         amount: amount
       });

       Cancel memory cancel = IRolldownPrimitives.Cancel({
         requestId: IRolldownPrimitives.RequestId({id: 4, origin: IRolldownPrimitives.Origin.L1}),
         range: Range({start: 1, end: 1}),
         hash: 0x0000000000000000000000000000000000000000000000000000000000000000
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
      bytes32 node_0 = keccak256(abi.encode(withdrawalBob1));
      bytes32 node_1 = keccak256(abi.encode(withdrawalBob2));
      bytes32 node_2 = keccak256(abi.encode(withdrawalCharlie));
      bytes32 node_3 = keccak256(abi.encode(cancel));

      bytes32 node_01 = keccak256(abi.encodePacked(node_0, node_1));
      bytes32 node_23 = keccak256(abi.encodePacked(node_2, node_3));

      bytes32 merkle_root = keccak256(abi.encodePacked(node_01, node_23));

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

      // bytes32[] memory proof_cancel = new bytes32[](2);
      // proof_cancel[0] = node_2;
      // proof_cancel[1] = node_01;

      vm.startPrank(ALICE);
      rolldown.update_l1_from_l2(merkle_root, IRolldownPrimitives.Range({start: 1, end: 4}));
      vm.stopPrank();

      vm.startPrank(CHARLIE);
      vm.expectRevert("Invalid proof");
      rolldown.close_withdrawal(withdrawalBob1, merkle_root, proof_withdrawalCharlie);
      vm.stopPrank();


      vm.startPrank(ALICE);
      vm.expectEmit(true, true, true, true);
      emit IRolldownPrimitives.FundsWithdrawn(CHARLIE, address(token), 12345);
      rolldown.close_withdrawal(withdrawalCharlie, merkle_root, proof_withdrawalCharlie);
      vm.stopPrank();

      vm.startPrank(CHARLIE);
      vm.expectEmit(true, true, true, true);
      emit IRolldownPrimitives.FundsWithdrawn(BOB, address(token), 12345);
      rolldown.close_withdrawal(withdrawalBob1, merkle_root, proof_withdrawalBob1);
      vm.stopPrank();

     }

    function testCloseSameWithdrawalTwiceFails() public {
        address recipient = 0x0000000000000000000000000000000000000006;
        uint256 amount = 123456;
        token.mint(address(rolldown));

        Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
          requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
          withdrawalRecipient: recipient,
          tokenAddress: address(token),
          amount: amount
        });

        vm.startPrank(ALICE);
        // merkle_root of tree with single element is just that single element
        bytes32 merkle_root = keccak256(abi.encode(withdrawal));
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkle_root, range);
        vm.stopPrank();

        bytes32[] memory proofs = new bytes32[](0);
        rolldown.close_withdrawal(withdrawal, merkle_root, proofs);

        vm.startPrank(ALICE);
        vm.expectRevert("Already processed");
        rolldown.close_withdrawal(withdrawal, merkle_root, proofs);
        vm.stopPrank();
    }

    function testL1UpdateHashCompatibilityWithMangataNode() public {
        Rolldown.L1Update memory l1Update;
        l1Update.chain = ChainId.Ethereum;
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
            0x3c1e43a559da200b6b94ab0efb9f273b653242cb014efe2310807ff26d1db2d1
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

    function testCancelWithNonMatchingHashResultsWithUnjustifiedStatus()
        public
    {
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
        // merkle_root of tree with single element is just that single element
        bytes32 merkle_root = keccak256(abi.encode(cancel));
        bytes32[] memory proofs = new bytes32[](0);
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkle_root, range);
        vm.stopPrank();

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.DisputeResolutionAcceptedIntoQueue(1, true);
        rolldown.close_cancel(cancel, merkle_root, proofs);
        vm.stopPrank();
    }

    function testCancelResolutionWithMatchingHashResultsWithJustifiedStatus()
        public
    {
        uint256 amount = 123456;
        token.mint(ALICE);

        vm.startPrank(ALICE);
        token.approve(address(rolldown), amount);
        rolldown.deposit_erc20(address(token), amount);
        vm.stopPrank();

        L1Update memory l1Update = rolldown.getPendingRequests(1,1);

        Cancel memory cancel = IRolldownPrimitives.Cancel({
          requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
          range: IRolldownPrimitives.Range({start: 1, end: 1}),
          hash: keccak256(abi.encode(l1Update))
        });


        vm.startPrank(ALICE);
        // merkle_root of tree with single element is just that single element
        bytes32 merkle_root = keccak256(abi.encode(cancel));
        bytes32[] memory proofs = new bytes32[](0);
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkle_root, range);
        vm.stopPrank();

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.DisputeResolutionAcceptedIntoQueue(1, false);
        rolldown.close_cancel(cancel, merkle_root, proofs);
        vm.stopPrank();
    }

    function testUnsuccessfulWithdrawalRequest() public {
        address recipient = 0x0000000000000000000000000000000000000006;
        uint256 amount = 123456;

        Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
          requestId: IRolldownPrimitives.RequestId({id: 1, origin: IRolldownPrimitives.Origin.L2}),
          withdrawalRecipient: recipient,
          tokenAddress: address(token),
          amount: amount
        });

        vm.startPrank(ALICE);
        // merkle_root of tree with single element is just that single element
        bytes32 merkle_root = keccak256(abi.encode(withdrawal));
        Range memory range = IRolldownPrimitives.Range({start: 1, end: 1});
        rolldown.update_l1_from_l2(merkle_root, range);
        vm.stopPrank();

        bytes32[] memory proofs = new bytes32[](0);
        assertEq(token.balanceOf(recipient), 0);

        vm.startPrank(ALICE);
        vm.expectEmit(true, true, true, true);
        emit IRolldownPrimitives.WithdrawalResolutionAcceptedIntoQueue(1, false);
        rolldown.close_withdrawal(withdrawal, merkle_root, proofs);
        vm.stopPrank();
        assertEq(token.balanceOf(recipient), 0);

        Rolldown.L1Update memory l1Update = rolldown.getUpdateForL2();
        assertEq(l1Update.pendingWithdrawalResolutions.length, 1);
    }

    function testAcceptOnlyConsecutiveUpdatesWithoutGaps() public {
        vm.startPrank(ALICE);
        rolldown.update_l1_from_l2(0x0000000000000000000000000000000000000000000000000000000000000000, IRolldownPrimitives.Range({start: 1, end: 1}));
        rolldown.update_l1_from_l2(0x0000000000000000000000000000000000000000000000000000000000000000, IRolldownPrimitives.Range({start: 2, end: 2}));
        rolldown.update_l1_from_l2(0x0000000000000000000000000000000000000000000000000000000000000000, IRolldownPrimitives.Range({start: 2, end: 10}));
        rolldown.update_l1_from_l2(0x0000000000000000000000000000000000000000000000000000000000000000, IRolldownPrimitives.Range({start: 9, end: 11}));
        rolldown.update_l1_from_l2(0x0000000000000000000000000000000000000000000000000000000000000000, IRolldownPrimitives.Range({start: 1, end: 12}));
        vm.stopPrank();
    }

    function testRejectUpdateWithoutNewRequests() public {
        vm.startPrank(ALICE);
        rolldown.update_l1_from_l2(0x0000000000000000000000000000000000000000000000000000000000000000, IRolldownPrimitives.Range({start: 1, end: 10}));
        vm.stopPrank();

        vm.startPrank(ALICE);
        vm.expectRevert("Update brings no new data");
        rolldown.update_l1_from_l2(0x0000000000000000000000000000000000000000000000000000000000000000, IRolldownPrimitives.Range({start: 9, end: 9}));
        vm.stopPrank();

        vm.startPrank(ALICE);
        vm.expectRevert("Update brings no new data");
        rolldown.update_l1_from_l2(0x0000000000000000000000000000000000000000000000000000000000000000, IRolldownPrimitives.Range({start: 1, end: 9}));
        vm.stopPrank();

        vm.startPrank(ALICE);
        vm.expectRevert("Update brings no new data");
        rolldown.update_l1_from_l2(0x0000000000000000000000000000000000000000000000000000000000000000, IRolldownPrimitives.Range({start: 1, end: 10}));
        vm.stopPrank();

    }

    function testRejectUpdateWithGaps() public {
        vm.startPrank(ALICE);
        rolldown.update_l1_from_l2(0x0000000000000000000000000000000000000000000000000000000000000000, IRolldownPrimitives.Range({start: 1, end: 10}));
        vm.stopPrank();

        vm.startPrank(ALICE);
        vm.expectRevert("Previous update missing");
        rolldown.update_l1_from_l2(0x0000000000000000000000000000000000000000000000000000000000000000, IRolldownPrimitives.Range({start: 12, end: 12}));
        vm.stopPrank();

    }

    function testVerifyBalancedMerkleRoot() public {
      //                                   ROOT
      //                      /                             \
      //                     /                               \
      //            /             \                    /              \
      //           /               \                  /                \
      //      /      \          /     \           /      \          /     \
      //     /        \        /       \         /        \        /       \
      // 0x00..00 0x11..11 0x22..22 0x33..33 0x44..44 0x55..55 0x66..66 0x77.77

        bytes32 root_hash = 0x36e4d0b08be66e3c82af50073c3f2833ac26cb61026c0c16d2d58230d8682998;
        bytes32[] memory proof = new bytes32[](3);

        proof[0] = 0x1111111111111111111111111111111111111111111111111111111111111111;
        proof[1] = 0xf3357627f4934d47fe409005b05c900777a6d97ec3788304e2d9c7b4d322cd4d;
        proof[2] = 0x9aaa5fc7595410234204542d86b85ce74d07a1f98d62d1bbf23f1c2378cc3089;
        assertEq(root_hash, rolldown.calculate_root(0x0000000000000000000000000000000000000000000000000000000000000000, 0, proof, 8));

        proof[0] = 0x0000000000000000000000000000000000000000000000000000000000000000;
        proof[1] = 0xf3357627f4934d47fe409005b05c900777a6d97ec3788304e2d9c7b4d322cd4d;
        proof[2] = 0x9aaa5fc7595410234204542d86b85ce74d07a1f98d62d1bbf23f1c2378cc3089;
        assertEq(root_hash, rolldown.calculate_root(0x1111111111111111111111111111111111111111111111111111111111111111, 1, proof, 8));

        proof[0] = 0x3333333333333333333333333333333333333333333333333333333333333333;
        proof[1] = 0x8e4b8e18156a1c7271055ce5b7ef53bb370294ebd631a3b95418a92da46e681f;
        proof[2] = 0x9aaa5fc7595410234204542d86b85ce74d07a1f98d62d1bbf23f1c2378cc3089;
        assertEq(root_hash, rolldown.calculate_root(0x2222222222222222222222222222222222222222222222222222222222222222, 2, proof, 8));

        proof[0] = 0x2222222222222222222222222222222222222222222222222222222222222222;
        proof[1] = 0x8e4b8e18156a1c7271055ce5b7ef53bb370294ebd631a3b95418a92da46e681f;
        proof[2] = 0x9aaa5fc7595410234204542d86b85ce74d07a1f98d62d1bbf23f1c2378cc3089;
        assertEq(root_hash, rolldown.calculate_root(0x3333333333333333333333333333333333333333333333333333333333333333, 3, proof, 8));

        proof[0] = 0x5555555555555555555555555555555555555555555555555555555555555555;
        proof[1] = 0x37df8a86dbd0a06a5a6720079d9a4ce5a5a5c93198607ca71402d78b7db2869e;
        proof[2] = 0xd287edfff411d3b45e9c7bf7186d7e9d44fa2a0fe36d85154165da0a1d7ce5bd;
        assertEq(root_hash, rolldown.calculate_root(0x4444444444444444444444444444444444444444444444444444444444444444, 4, proof, 8));

        proof[0] = 0x4444444444444444444444444444444444444444444444444444444444444444;
        proof[1] = 0x37df8a86dbd0a06a5a6720079d9a4ce5a5a5c93198607ca71402d78b7db2869e;
        proof[2] = 0xd287edfff411d3b45e9c7bf7186d7e9d44fa2a0fe36d85154165da0a1d7ce5bd;
        assertEq(root_hash, rolldown.calculate_root(0x5555555555555555555555555555555555555555555555555555555555555555, 5, proof, 8));

        proof[0] = 0x7777777777777777777777777777777777777777777777777777777777777777;
        proof[1] = 0x60c25b70d66af589f985b3cf4732585b8f7ecea5df88cb12368650edfe7e6f50;
        proof[2] = 0xd287edfff411d3b45e9c7bf7186d7e9d44fa2a0fe36d85154165da0a1d7ce5bd;
        assertEq(root_hash, rolldown.calculate_root(0x6666666666666666666666666666666666666666666666666666666666666666, 6, proof, 8));

        proof[0] = 0x6666666666666666666666666666666666666666666666666666666666666666;
        proof[1] = 0x60c25b70d66af589f985b3cf4732585b8f7ecea5df88cb12368650edfe7e6f50;
        proof[2] = 0xd287edfff411d3b45e9c7bf7186d7e9d44fa2a0fe36d85154165da0a1d7ce5bd;
        assertEq(root_hash, rolldown.calculate_root(0x7777777777777777777777777777777777777777777777777777777777777777, 7, proof, 8));
    }

    function testVerifyUnBalancedMerkleRoot1() public {
      //                                   ROOT
      //                      /                             \
      //                     /                               \
      //            /             \                    /              \
      //           /               \                  /                \
      //      /      \          /     \           /      \          /     \
      //     /        \        /       \         /        \        /       \
      // 0x00..00 0x11..11 0x22..22 0x33..33 0x44..44 0x55..55 0x66..66
      //
        bytes32 root_hash = 0x018f1011ee4add9a8c0e73b4909158862437ec4aadcc6ca697b357f49425e9ad;
        bytes32[] memory proof = new bytes32[](3);

        proof[0] = 0x60c25b70d66af589f985b3cf4732585b8f7ecea5df88cb12368650edfe7e6f50;
        proof[1] = 0xd287edfff411d3b45e9c7bf7186d7e9d44fa2a0fe36d85154165da0a1d7ce5bd;
        assertEq(root_hash, rolldown.calculate_root(0x6666666666666666666666666666666666666666666666666666666666666666, 6, proof, 7));
    }

    function testMerkleProofs() public {
      string memory config_data = vm.readFile("./test/merkle-verificaction-testdata.json");
      uint256 test_cases_amount = stdJson.readUint(config_data, ".cases_count");

      for (uint256 i = 0; i < test_cases_amount ; ++i) {
        console.log("TEST case: ", i);
        uint256 leavePos = stdJson.readUint(config_data, string.concat(".cases.[", Strings.toString(i), "].leave_pos"));
        bytes32 leaveHash = stdJson.readBytes32(config_data, string.concat(".cases.[", Strings.toString(i),"].leave_hash"));
        bytes32 expectedRoot = stdJson.readBytes32(config_data, string.concat(".cases.[", Strings.toString(i),"].expected_root"));
        bytes32[] memory proof = stdJson.readBytes32Array(config_data, string.concat(".cases.[", Strings.toString(i), "].proof"));
        bytes32[] memory leaves = stdJson.readBytes32Array(config_data, string.concat(".cases.[", Strings.toString(i), "].leaves"));

        assertEq(
          rolldown.calculate_root(leaveHash, uint32(leavePos), proof, uint32(leaves.length)),
          expectedRoot
        );
      }

    }
}

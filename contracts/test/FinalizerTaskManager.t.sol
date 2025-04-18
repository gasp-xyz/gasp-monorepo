// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {BLSSignatureChecker} from "@eigenlayer-middleware/src/BLSSignatureChecker.sol";
import {IBLSSignatureChecker} from "@eigenlayer-middleware/src/interfaces/IBLSSignatureChecker.sol";
import {IPauserRegistry} from "@eigenlayer/contracts/permissions/Pausable.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {Test} from "forge-std/Test.sol";
import {FinalizerTaskManager} from "./../src/FinalizerTaskManager.sol";
import {IFinalizerTaskManager} from "./../src/interfaces/IFinalizerTaskManager.sol";
import {IRolldown} from "./../src/interfaces/IRolldown.sol";
import {IRolldownPrimitives} from "./../src/interfaces/IRolldownPrimitives.sol";

contract FinalizerTaskManagerTest is Test {
    FinalizerTaskManager internal taskManager;

    address internal owner;
    address internal aggregator;
    address internal generator;
    address internal blsSignatureChecker;
    address internal operatorStateRetrieverExtended;
    address internal pauserRegistry;
    address internal rolldown;

    uint32 internal constant TASK_RESPONSE_WINDOW_BLOCK = 100;

    function setUp() public {
        owner = makeAddr("owner");
        aggregator = makeAddr("aggregator");
        generator = makeAddr("generator");
        blsSignatureChecker = makeAddr("blsSignatureChecker");
        operatorStateRetrieverExtended = makeAddr("operatorStateRetrieverExtended");
        pauserRegistry = makeAddr("pauserRegistry");
        rolldown = makeAddr("rolldown");

        FinalizerTaskManager impl = new FinalizerTaskManager();
        ProxyAdmin proxyAdmin = new ProxyAdmin();
        TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(
            address(impl),
            address(proxyAdmin),
            abi.encodeCall(
                FinalizerTaskManager.initialize,
                (
                    IPauserRegistry(pauserRegistry),
                    owner,
                    aggregator,
                    generator,
                    true, // allowNonRootInit
                    blsSignatureChecker,
                    TASK_RESPONSE_WINDOW_BLOCK,
                    operatorStateRetrieverExtended,
                    IRolldown(rolldown)
                )
            )
        );
        taskManager = FinalizerTaskManager(payable(address(proxy)));
    }

    function test_SetAggregator() public {
        address newAggregator = makeAddr("newAggregator");

        vm.prank(owner);
        taskManager.setAggregator(newAggregator);

        assertEq(taskManager.aggregator(), newAggregator);
    }

    function test_RevertIf_NotOwner_SetAggregator() public {
        address newAggregator = makeAddr("newAggregator");

        vm.prank(aggregator);
        vm.expectRevert("Ownable: caller is not the owner");
        taskManager.setAggregator(newAggregator);
    }

    function test_CreateNewOpTask() public {
        uint32 quorumThresholdPercentage = 100;
        bytes memory quorumNumbers = abi.encodePacked(uint8(1));

        vm.expectCall(
            address(taskManager),
            abi.encodeWithSelector(
                FinalizerTaskManager.createNewOpTask.selector, quorumThresholdPercentage, quorumNumbers
            )
        );

        vm.prank(generator);
        taskManager.createNewOpTask(quorumThresholdPercentage, quorumNumbers);

        assertEq(taskManager.latestOpTaskNum(), 1);
        assertEq(taskManager.isTaskPending(), true);
    }

    function test_RespondToOpTask() public {
        // First create a task
        uint32 quorumThresholdPercentage = 100;
        bytes memory quorumNumbers = abi.encodePacked(uint8(1));

        vm.prank(generator);
        taskManager.createNewOpTask(quorumThresholdPercentage, quorumNumbers);

        // Setup task response data
        FinalizerTaskManager.OpTask memory task = IFinalizerTaskManager.OpTask({
            taskNum: 0,
            quorumNumbers: quorumNumbers,
            quorumThresholdPercentage: quorumThresholdPercentage,
            taskCreatedBlock: uint32(block.number),
            lastCompletedOpTaskNum: 0,
            lastCompletedOpTaskCreatedBlock: uint32(block.number),
            lastCompletedOpTaskQuorumNumbers: quorumNumbers,
            lastCompletedOpTaskQuorumThresholdPercentage: quorumThresholdPercentage
        });

        FinalizerTaskManager.OpTaskResponse memory taskResponse = IFinalizerTaskManager.OpTaskResponse({
            referenceTaskIndex: 0,
            referenceTaskHash: bytes32(0),
            operatorsStateInfoHash: bytes32(0)
        });

        BLSSignatureChecker.NonSignerStakesAndSignature memory nonSignerStakesAndSignature;

        vm.mockCall(
            address(blsSignatureChecker),
            abi.encodeWithSelector(BLSSignatureChecker.checkSignatures.selector),
            abi.encode(
                IBLSSignatureChecker.QuorumStakeTotals({
                    totalStakeForQuorum: new uint96[](1),
                    signedStakeForQuorum: new uint96[](1)
                }),
                bytes32(0)
            )
        );

        vm.prank(aggregator);
        taskManager.respondToOpTask(task, taskResponse, nonSignerStakesAndSignature);

        assertEq(taskManager.isTaskPending(), false);
    }

    function test_CancelPendingTasks() public {
        // Create task first
        uint32 quorumThresholdPercentage = 100;
        bytes memory quorumNumbers = abi.encodePacked(uint8(1));

        vm.prank(generator);
        taskManager.createNewOpTask(quorumThresholdPercentage, quorumNumbers);

        vm.prank(generator);
        taskManager.cancelPendingTasks();

        assertEq(taskManager.isTaskPending(), false);
    }

    function test_CreateNewRdTask() public {
        // First complete an op task to initialize op state
        uint32 quorumThresholdPercentage = 100;
        bytes memory quorumNumbers = abi.encodePacked(uint8(1));

        vm.prank(generator);
        taskManager.createNewOpTask(quorumThresholdPercentage, quorumNumbers);

        FinalizerTaskManager.OpTask memory task = IFinalizerTaskManager.OpTask({
            taskNum: 0,
            quorumNumbers: quorumNumbers,
            quorumThresholdPercentage: quorumThresholdPercentage,
            taskCreatedBlock: uint32(block.number),
            lastCompletedOpTaskNum: 0,
            lastCompletedOpTaskCreatedBlock: uint32(block.number),
            lastCompletedOpTaskQuorumNumbers: quorumNumbers,
            lastCompletedOpTaskQuorumThresholdPercentage: quorumThresholdPercentage
        });

        FinalizerTaskManager.OpTaskResponse memory taskResponse = IFinalizerTaskManager.OpTaskResponse({
            referenceTaskIndex: 0,
            referenceTaskHash: bytes32(0),
            operatorsStateInfoHash: bytes32(0)
        });

        vm.prank(owner);
        taskManager.forceRespondToOpTask(task, taskResponse);

        // Now create RD task
        vm.prank(generator);
        taskManager.createNewRdTask(IRolldownPrimitives.ChainId.Ethereum, 1);

        assertEq(taskManager.latestRdTaskNum(), 1);
        assertEq(taskManager.isTaskPending(), true);
    }

    function test_RespondToRdTask() public {
        // Create and complete op task first
        test_CreateNewRdTask();

        // Setup RD task response
        FinalizerTaskManager.RdTask memory task = IFinalizerTaskManager.RdTask({
            taskNum: 0,
            chainId: IRolldownPrimitives.ChainId.Ethereum,
            batchId: 1,
            taskCreatedBlock: uint32(block.number),
            lastCompletedOpTaskNum: 0,
            lastCompletedOpTaskCreatedBlock: uint32(block.number),
            lastCompletedOpTaskQuorumNumbers: abi.encodePacked(uint8(1)),
            lastCompletedOpTaskQuorumThresholdPercentage: 100
        });

        FinalizerTaskManager.RdTaskResponse memory taskResponse = IFinalizerTaskManager.RdTaskResponse({
            referenceTaskIndex: 0,
            referenceTaskHash: bytes32(0),
            chainId: IRolldownPrimitives.ChainId.Ethereum,
            batchId: 1,
            rdUpdate: bytes32(0),
            rangeStart: 0,
            rangeEnd: 1,
            updater: rolldown
        });

        BLSSignatureChecker.NonSignerStakesAndSignature memory nonSignerStakesAndSignature;

        vm.mockCall(
            address(blsSignatureChecker),
            abi.encodeWithSelector(BLSSignatureChecker.checkSignatures.selector),
            abi.encode(
                IBLSSignatureChecker.QuorumStakeTotals({
                    totalStakeForQuorum: new uint96[](1),
                    signedStakeForQuorum: new uint96[](1)
                }),
                bytes32(0)
            )
        );

        vm.mockCall(address(rolldown), abi.encodeWithSelector(IRolldown.update_l1_from_l2.selector), abi.encode());

        vm.prank(aggregator);
        taskManager.respondToRdTask(task, taskResponse, nonSignerStakesAndSignature);

        assertEq(taskManager.isTaskPending(), false);
    }

    function test_ForceCancelPendingTasks() public {
        // Create task first
        uint32 quorumThresholdPercentage = 100;
        bytes memory quorumNumbers = abi.encodePacked(uint8(1));

        vm.prank(generator);
        taskManager.createNewOpTask(quorumThresholdPercentage, quorumNumbers);

        vm.prank(owner);
        taskManager.forceCancelPendingTasks();

        assertEq(taskManager.isTaskPending(), false);
    }

    function test_CheckSignatures() public {
        bytes32 msgHash = keccak256("test");
        bytes memory quorumNumbers = abi.encodePacked(uint8(1));
        uint32 referenceBlockNumber = uint32(block.number);
        BLSSignatureChecker.NonSignerStakesAndSignature memory nonSignerStakesAndSignature;

        vm.mockCall(
            address(blsSignatureChecker),
            abi.encodeWithSelector(BLSSignatureChecker.checkSignatures.selector),
            abi.encode(
                IBLSSignatureChecker.QuorumStakeTotals({
                    totalStakeForQuorum: new uint96[](1),
                    signedStakeForQuorum: new uint96[](1)
                }),
                bytes32(0)
            )
        );

        (BLSSignatureChecker.QuorumStakeTotals memory stakeTotals, bytes32 hashOfNonSigners) =
            taskManager.checkSignatures(msgHash, quorumNumbers, referenceBlockNumber, nonSignerStakesAndSignature);

        assertEq(stakeTotals.totalStakeForQuorum.length, 1);
        assertEq(stakeTotals.signedStakeForQuorum.length, 1);
        assertEq(hashOfNonSigners, bytes32(0));
    }

    function test_ForceCreateNewOpTask() public {
        uint32 quorumThresholdPercentage = 100;
        bytes memory quorumNumbers = abi.encodePacked(uint8(1));

        vm.prank(owner);
        taskManager.forceCreateNewOpTask(quorumThresholdPercentage, quorumNumbers);

        assertEq(taskManager.latestOpTaskNum(), 1);
        assertEq(taskManager.isTaskPending(), true);
    }

    function test_RevertIf_NotOwner_ForceCreateNewOpTask() public {
        uint32 quorumThresholdPercentage = 100;
        bytes memory quorumNumbers = abi.encodePacked(uint8(1));

        vm.prank(aggregator);
        vm.expectRevert("Ownable: caller is not the owner");
        taskManager.forceCreateNewOpTask(quorumThresholdPercentage, quorumNumbers);
    }

    function test_ForceRespondToOpTask() public {
        // First create a task
        uint32 quorumThresholdPercentage = 100;
        bytes memory quorumNumbers = abi.encodePacked(uint8(1));

        vm.prank(generator);
        taskManager.createNewOpTask(quorumThresholdPercentage, quorumNumbers);

        // Setup task response data
        FinalizerTaskManager.OpTask memory task = IFinalizerTaskManager.OpTask({
            taskNum: 0,
            quorumNumbers: quorumNumbers,
            quorumThresholdPercentage: quorumThresholdPercentage,
            taskCreatedBlock: uint32(block.number),
            lastCompletedOpTaskNum: 0,
            lastCompletedOpTaskCreatedBlock: uint32(block.number),
            lastCompletedOpTaskQuorumNumbers: quorumNumbers,
            lastCompletedOpTaskQuorumThresholdPercentage: quorumThresholdPercentage
        });

        FinalizerTaskManager.OpTaskResponse memory taskResponse = IFinalizerTaskManager.OpTaskResponse({
            referenceTaskIndex: 0,
            referenceTaskHash: bytes32(0),
            operatorsStateInfoHash: bytes32(0)
        });

        vm.prank(owner);
        taskManager.forceRespondToOpTask(task, taskResponse);

        assertEq(taskManager.isTaskPending(), false);
    }

    function test_RevertIf_NotOwner_ForceRespondToOpTask() public {
        // First create a task
        uint32 quorumThresholdPercentage = 100;
        bytes memory quorumNumbers = abi.encodePacked(uint8(1));

        vm.prank(generator);
        taskManager.createNewOpTask(quorumThresholdPercentage, quorumNumbers);

        // Setup task response data
        FinalizerTaskManager.OpTask memory task = IFinalizerTaskManager.OpTask({
            taskNum: 0,
            quorumNumbers: quorumNumbers,
            quorumThresholdPercentage: quorumThresholdPercentage,
            taskCreatedBlock: uint32(block.number),
            lastCompletedOpTaskNum: 0,
            lastCompletedOpTaskCreatedBlock: uint32(block.number),
            lastCompletedOpTaskQuorumNumbers: quorumNumbers,
            lastCompletedOpTaskQuorumThresholdPercentage: quorumThresholdPercentage
        });

        FinalizerTaskManager.OpTaskResponse memory taskResponse = IFinalizerTaskManager.OpTaskResponse({
            referenceTaskIndex: 0,
            referenceTaskHash: bytes32(0),
            operatorsStateInfoHash: bytes32(0)
        });

        vm.prank(aggregator);
        vm.expectRevert("Ownable: caller is not the owner");
        taskManager.forceRespondToOpTask(task, taskResponse);
    }
}

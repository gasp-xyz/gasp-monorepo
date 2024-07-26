// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

// import "../src/FinalizerServiceManager.sol" as msm;
// import {FinalizerTaskManager} from "../src/FinalizerTaskManager.sol";
// import {BLSMockAVSDeployer} from "@eigenlayer-middleware/test/utils/BLSMockAVSDeployer.sol";
// import {BLSSignatureChecker} from "@eigenlayer-middleware/src/BLSSignatureChecker.sol";
// import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
// import {BitmapUtils} from "@eigenlayer-middleware/src/libraries/BitmapUtils.sol";
// import { FinalizerTaskManager } from "../src/FinalizerTaskManager.sol";
// import { IFinalizerTaskManager } from "../src/IFinalizerTaskManager.sol";
// import { IGaspMultiRollupServicePrimitives } from "../src/IGaspMultiRollupServicePrimitives.sol";

// import { IBLSSignatureChecker  } from "@eigenlayer-middleware/src/interfaces/IBLSSignatureChecker.sol";

// contract FinalizerTaskManagerTest is BLSMockAVSDeployer {
//     msm.FinalizerServiceManager sm;
//     msm.FinalizerServiceManager smImplementation;
//     FinalizerTaskManager tm;
//     FinalizerTaskManager tmImplementation;

//     event NewTaskCreated(uint32 indexed taskIndex, FinalizerTaskManager.Task task);

//     uint32 public constant TASK_RESPONSE_WINDOW_BLOCK = 30;
//     address aggregator =
//         address(uint160(uint256(keccak256(abi.encodePacked("aggregator")))));
//     address generator =
//         address(uint160(uint256(keccak256(abi.encodePacked("generator")))));

//     function setUp() public {
//         _setUpBLSMockAVSDeployer();

//         tmImplementation = new FinalizerTaskManager(
//             msm.IRegistryCoordinator(
//                 address(registryCoordinator)
//             ),
//             TASK_RESPONSE_WINDOW_BLOCK
//         );

//         // Third, upgrade the proxy contracts to use the correct implementation contracts and initialize them.
//         tm = FinalizerTaskManager(
//             address(
//                 new TransparentUpgradeableProxy(
//                     address(tmImplementation),
//                     address(proxyAdmin),
//                     abi.encodeWithSelector(
//                         tm.initialize.selector,
//                         pauserRegistry,
//                         registryCoordinatorOwner,
//                         aggregator,
//                         generator
//                     )
//                 )
//             )
//         );
//     }

//     function testCreateNewTask() public {
//         bytes memory quorumNumbers = new bytes(0);
//         cheats.prank(generator, generator);

//         FinalizerTaskManager.Task memory newTask;
//         newTask.blockNumber = 2;
//         newTask.taskCreatedBlock = 1;
//         newTask.quorumThresholdPercentage = 100;
//         newTask.quorumNumbers = quorumNumbers;
//         vm.expectEmit(true, true, false, true);
//         emit NewTaskCreated( 0, newTask);
//         tm.createNewTask(2, 100, quorumNumbers);
//         assertEq(tm.latestTaskNum(), 1);
        
//     }
//     function testCreateNewTaskOnlyGenerator() public {
//         bytes memory quorumNumbers = new bytes(0);
//         cheats.prank(aggregator, aggregator);
//         vm.expectRevert("Task generator must be the caller");
//         tm.createNewTask(2, 100, quorumNumbers);
//     }
//     function testCreateNewTaskTwiceSameBlock() public {
//         bytes memory quorumNumbers = new bytes(0);
//         cheats.prank(generator, generator);
//         tm.createNewTask(2, 100, quorumNumbers);
//         assertEq(tm.latestTaskNum(), 1);
        
//         cheats.prank(generator, generator);
//         tm.createNewTask(2, 100, quorumNumbers);
//         assertEq(tm.latestTaskNum(), 2);

//         FinalizerTaskManager.Task memory newTask;
//         newTask.blockNumber = 2;
//         newTask.taskCreatedBlock = 1;
//         newTask.quorumThresholdPercentage = 100;
//         newTask.quorumNumbers = quorumNumbers;

//         //Validate that the two last tasks correspond with the latest tasks in the storage.
//         assertEq(keccak256(abi.encode(newTask)), tm.allTaskHashes(tm.latestTaskNum() -2 ));
//         assertEq(keccak256(abi.encode(newTask)), tm.allTaskHashes(tm.latestTaskNum() -1 ));

//     }
//     function testCreateAndRespondOnlyAggregator() public {
        
//         bytes memory quorumNumbers = BitmapUtils.bitmapToBytesArray(0);

//         FinalizerTaskManager.Task memory newTask;
//         newTask.blockNumber = 2;
//         newTask.taskCreatedBlock = 1;
//         newTask.quorumThresholdPercentage = 100;
//         newTask.quorumNumbers = quorumNumbers;

//         cheats.prank(generator, generator);
//         vm.expectEmit(true, true, false, true);
//         emit NewTaskCreated( 0, newTask);
//         tm.createNewTask(2, 100, quorumNumbers);

//         IFinalizerTaskManager.TaskResponse memory taskResponse;
//         taskResponse.referenceTaskIndex = tm.latestTaskNum() -1;
//         taskResponse.blockHash = keccak256(abi.encode(newTask));
//         taskResponse.storageProofHash = keccak256(abi.encodePacked("storageProofHash"));
//         taskResponse.pendingStateHash = keccak256(abi.encodePacked("pendingStateHash"));

//         assertEq(keccak256(abi.encode(newTask)), tm.allTaskHashes(tm.latestTaskNum() -1 ));
        
//         IBLSSignatureChecker.NonSignerStakesAndSignature memory nonSignerStakesAndSignature;
//         IGaspMultiRollupServicePrimitives.NonSignerStakesAndSignatureForOldState memory NonSignerStakesAndSignatureForOldState;

//         IFinalizerTaskManager.Task memory newTaskResponse;
//         newTaskResponse.blockNumber = 2;
//         newTaskResponse.taskCreatedBlock = 1;
//         newTaskResponse.quorumThresholdPercentage = 100;
//         newTaskResponse.quorumNumbers = quorumNumbers;

//         assertEq(keccak256(abi.encode(newTaskResponse)), tm.allTaskHashes(tm.latestTaskNum() -1 ));
        
//         cheats.prank(generator, generator);
//         vm.expectRevert("Aggregator must be the caller");
//         tm.respondToTask(newTaskResponse, taskResponse, nonSignerStakesAndSignature, NonSignerStakesAndSignatureForOldState);
        
//     }
//     function testCreateAndRespondTaskValidation() public {
        
//         bytes memory quorumNumbers = BitmapUtils.bitmapToBytesArray(0);

//         FinalizerTaskManager.Task memory newTask;
//         newTask.blockNumber = 2;
//         newTask.taskCreatedBlock = 1;
//         newTask.quorumThresholdPercentage = 100;
//         newTask.quorumNumbers = quorumNumbers;

//         cheats.prank(generator, generator);
//         vm.expectEmit(true, true, false, true);
//         emit NewTaskCreated( 0, newTask);
//         tm.createNewTask(2, 100, quorumNumbers);

//         IFinalizerTaskManager.TaskResponse memory taskResponse;
//         taskResponse.referenceTaskIndex = tm.latestTaskNum() -1;
//         taskResponse.blockHash = keccak256(abi.encode(newTask));
//         taskResponse.storageProofHash = keccak256(abi.encodePacked("storageProofHash"));
//         taskResponse.pendingStateHash = keccak256(abi.encodePacked("pendingStateHash"));

//         assertEq(keccak256(abi.encode(newTask)), tm.allTaskHashes(tm.latestTaskNum() -1 ));
        
//         IBLSSignatureChecker.NonSignerStakesAndSignature memory nonSignerStakesAndSignature;
//         IGaspMultiRollupServicePrimitives.NonSignerStakesAndSignatureForOldState memory NonSignerStakesAndSignatureForOldState;

//         IFinalizerTaskManager.Task memory newTaskResponse;
//         newTaskResponse.blockNumber = 2;
//         newTaskResponse.taskCreatedBlock = 6666;
//         newTaskResponse.quorumThresholdPercentage = 101;
//         newTaskResponse.quorumNumbers = quorumNumbers;
        
//         cheats.prank(aggregator, aggregator);
//         vm.expectRevert("supplied task does not match the one recorded in the contract");
//         tm.respondToTask(newTaskResponse, taskResponse, nonSignerStakesAndSignature, NonSignerStakesAndSignatureForOldState);
        
//     }
//     function getwindowBock() public {
        
//         bytes memory quorumNumbers = BitmapUtils.bitmapToBytesArray(0);

//         FinalizerTaskManager.Task memory newTask;
//         newTask.blockNumber = 2;
//         newTask.taskCreatedBlock = 1;
//         newTask.quorumThresholdPercentage = 100;
//         newTask.quorumNumbers = quorumNumbers;

//         cheats.prank(generator, generator);
//         vm.expectEmit(true, true, false, true);
//         emit NewTaskCreated( 0, newTask);
//         tm.createNewTask(2, 100, quorumNumbers);

//         IFinalizerTaskManager.TaskResponse memory taskResponse;
//         taskResponse.referenceTaskIndex = tm.latestTaskNum() -1;
//         taskResponse.blockHash = keccak256(abi.encode(newTask));
//         taskResponse.storageProofHash = keccak256(abi.encodePacked("storageProofHash"));
//         taskResponse.pendingStateHash = keccak256(abi.encodePacked("pendingStateHash"));

//         assertEq(keccak256(abi.encode(newTask)), tm.allTaskHashes(tm.latestTaskNum() -1 ));
        
//         IBLSSignatureChecker.NonSignerStakesAndSignature memory nonSignerStakesAndSignature;
//         IGaspMultiRollupServicePrimitives.NonSignerStakesAndSignatureForOldState memory NonSignerStakesAndSignatureForOldState;

//         IFinalizerTaskManager.Task memory newTaskResponse;
//         newTaskResponse.blockNumber = 2;
//         newTaskResponse.taskCreatedBlock = 1;
//         newTaskResponse.quorumThresholdPercentage = 100;
//         newTaskResponse.quorumNumbers = quorumNumbers;

//         assertEq(keccak256(abi.encode(newTaskResponse)), tm.allTaskHashes(tm.latestTaskNum() -1 ));
        
//         cheats.prank(aggregator, aggregator);
//         vm.expectRevert("BLSSignatureChecker.checkSignatures: empty quorum input");
//         tm.respondToTask(newTaskResponse, taskResponse, nonSignerStakesAndSignature, NonSignerStakesAndSignatureForOldState);
        
//     }
//     function testGetTimeWindow() public {
//         uint32 timeWindow = tm.getTaskResponseWindowBlock();
//         assertEq(timeWindow, TASK_RESPONSE_WINDOW_BLOCK);
//     } 
//     function testCreateAndRespondTaskTimeWindow() public {
        
//         bytes memory quorumNumbers = BitmapUtils.bitmapToBytesArray(0);

//         FinalizerTaskManager.Task memory newTask;
//         newTask.blockNumber = 2;
//         newTask.taskCreatedBlock = 1;
//         newTask.quorumThresholdPercentage = 100;
//         newTask.quorumNumbers = quorumNumbers;

//         cheats.prank(generator, generator);
//         vm.expectEmit(true, true, false, true);
//         emit NewTaskCreated( 0, newTask);
//         tm.createNewTask(2, 100, quorumNumbers);
        
//         vm.roll( block.number + tm.getTaskResponseWindowBlock() + 1);
//         IFinalizerTaskManager.TaskResponse memory taskResponse;
//         taskResponse.referenceTaskIndex = tm.latestTaskNum() -1;
//         taskResponse.blockHash = keccak256(abi.encode(newTask));
//         taskResponse.storageProofHash = keccak256(abi.encodePacked("storageProofHash"));
//         taskResponse.pendingStateHash = keccak256(abi.encodePacked("pendingStateHash"));

//         assertEq(keccak256(abi.encode(newTask)), tm.allTaskHashes(tm.latestTaskNum() -1 ));
        
//         IBLSSignatureChecker.NonSignerStakesAndSignature memory nonSignerStakesAndSignature;
//         IGaspMultiRollupServicePrimitives.NonSignerStakesAndSignatureForOldState memory NonSignerStakesAndSignatureForOldState;

//         IFinalizerTaskManager.Task memory newTaskResponse;
//         newTaskResponse.blockNumber = 2;
//         newTaskResponse.taskCreatedBlock = 1;
//         newTaskResponse.quorumThresholdPercentage = 100;
//         newTaskResponse.quorumNumbers = quorumNumbers;
        
//         cheats.prank(aggregator, aggregator);
//         vm.expectRevert("Aggregator has responded to the task too late");
//         tm.respondToTask(newTaskResponse, taskResponse, nonSignerStakesAndSignature, NonSignerStakesAndSignatureForOldState);
        
//     }
//     //TODO: Create test for
//     // respondToTask -> happy path
//     // respondToTask -> alreadyResponded
//     // respondToTask -> Threshold not reached ( validate storage value )
//     // respondToTask -> Threshold reached ( validate storage value )
//     // respondToTask -> getLatestStateHash
// }

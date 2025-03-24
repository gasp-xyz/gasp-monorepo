// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import { Test } from "forge-std/Test.sol";
import { FinalizerTaskManager } from "../src/FinalizerTaskManager.sol";
import { IFinalizerTaskManager } from "../src/interfaces/IFinalizerTaskManager.sol";
import { IPauserRegistry } from "@eigenlayer/contracts/permissions/Pausable.sol";
import { BLSSignatureChecker } from "@eigenlayer-middleware/src/BLSSignatureChecker.sol";
import { IBLSSignatureChecker } from "@eigenlayer-middleware/src/interfaces/IBLSSignatureChecker.sol";
import { IRolldown } from "../src/interfaces/IRolldown.sol";
import { IRolldownPrimitives } from "../src/interfaces/IRolldownPrimitives.sol";
import { ProxyAdmin } from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import { TransparentUpgradeableProxy } from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import { BLSMockAVSDeployer } from "@eigenlayer-middleware/test/utils/BLSMockAVSDeployer.sol";
import { BitmapUtils } from "@eigenlayer-middleware/src/libraries/BitmapUtils.sol";
import { BN254 } from "@eigenlayer-middleware/src/libraries/BN254.sol";

contract FinalizerTaskManagerIntegrationTest is Test, BLSMockAVSDeployer {
  using BN254 for BN254.G1Point;

  BLSSignatureChecker blsSignatureChecker;
  FinalizerTaskManager internal taskManager;
  address internal owner;
  address internal aggregator;
  address internal generator;
  address internal rolldown;
  uint32 constant TASK_RESPONSE_WINDOW_BLOCK = 1000;

  function setUp() public {
    _setUpBLSMockAVSDeployer();

    owner = makeAddr("owner");
    aggregator = makeAddr("aggregator");
    generator = makeAddr("generator");
    rolldown = makeAddr("rolldown");

    blsSignatureChecker = new BLSSignatureChecker(registryCoordinator);
    FinalizerTaskManager impl = new FinalizerTaskManager();
    ProxyAdmin proxyAdmin = new ProxyAdmin();
    TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(
      address(impl),
      address(proxyAdmin),
      abi.encodeCall(
        FinalizerTaskManager.initialize,
        (
          IPauserRegistry(address(registryCoordinator)),
          owner,
          aggregator,
          generator,
          true, // allowNonRootInit
          address(blsSignatureChecker),
          TASK_RESPONSE_WINDOW_BLOCK,
          address(operatorStateRetriever),
          IRolldown(rolldown)
        )
      )
    );
    taskManager = FinalizerTaskManager(payable(address(proxy)));
  }

  //   function test_RespondToOpTaskWithRealBLS() public {
  //     // First create a task
  //     uint32 quorumThresholdPercentage = 100;
  //     bytes memory quorumNumbers = BitmapUtils.bitmapToBytesArray(1); // Single quorum

  //     // Get real BLS signature data from the mock deployer
  //     (
  //       uint32 referenceBlockNumber,
  //       BLSSignatureChecker.NonSignerStakesAndSignature memory nonSignerStakesAndSignature
  //     ) = _registerSignatoriesAndGetNonSignerStakeAndSignatureRandom(111, 1, 1);

  //     vm.prank(generator);
  //     taskManager.createNewOpTask(quorumThresholdPercentage, quorumNumbers);

  //     // Setup task response data
  //     FinalizerTaskManager.OpTask memory task = IFinalizerTaskManager.OpTask({
  //       taskNum: 0,
  //       quorumNumbers: quorumNumbers,
  //       quorumThresholdPercentage: quorumThresholdPercentage,
  //       taskCreatedBlock: uint32(block.number),
  //       lastCompletedOpTaskNum: 0,
  //       lastCompletedOpTaskCreatedBlock: uint32(block.number),
  //       lastCompletedOpTaskQuorumNumbers: quorumNumbers,
  //       lastCompletedOpTaskQuorumThresholdPercentage: quorumThresholdPercentage
  //     });

  //     FinalizerTaskManager.OpTaskResponse memory taskResponse = IFinalizerTaskManager.OpTaskResponse({
  //       referenceTaskIndex: 0,
  //       referenceTaskHash: bytes32(0),
  //       operatorsStateInfoHash: bytes32(0)
  //     });

  //     vm.prank(aggregator);
  //     taskManager.respondToOpTask(task, taskResponse, nonSignerStakesAndSignature);

  //     assertEq(taskManager.isTaskPending(), false);
  //   }

  function test_RespondToRdTaskWithRealBLS() public {
    // First complete an op task to initialize op state
    uint32 quorumThresholdPercentage = 100;
    bytes memory quorumNumbers = BitmapUtils.bitmapToBytesArray(1);

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
    msgHash = keccak256(abi.encode(taskResponse));
    _setAggregatePublicKeysAndSignature();

    (
      uint32 referenceBlockNumber,
      BLSSignatureChecker.NonSignerStakesAndSignature memory nonSignerStakesAndSignature
    ) = _registerSignatoriesAndGetNonSignerStakeAndSignatureRandom(111, 0, 1);

    // init OP state
    vm.prank(generator);
    taskManager.createNewOpTask(quorumThresholdPercentage, quorumNumbers);

    FinalizerTaskManager.OpTask memory opTask = IFinalizerTaskManager.OpTask({
      taskNum: 0,
      quorumNumbers: quorumNumbers,
      quorumThresholdPercentage: quorumThresholdPercentage,
      taskCreatedBlock: uint32(block.number),
      lastCompletedOpTaskNum: 0,
      lastCompletedOpTaskCreatedBlock: uint32(block.number),
      lastCompletedOpTaskQuorumNumbers: quorumNumbers,
      lastCompletedOpTaskQuorumThresholdPercentage: quorumThresholdPercentage
    });

    FinalizerTaskManager.OpTaskResponse memory opTaskResponse = IFinalizerTaskManager
      .OpTaskResponse({
        referenceTaskIndex: 0,
        referenceTaskHash: bytes32(0),
        operatorsStateInfoHash: bytes32(0)
      });

    vm.prank(owner);
    taskManager.forceRespondToOpTask(opTask, opTaskResponse);


    // Now create RD task
    vm.prank(generator);
    taskManager.createNewRdTask(IRolldownPrimitives.ChainId.Ethereum, 1);

    // Setup RD task response
    FinalizerTaskManager.RdTask memory task = IFinalizerTaskManager.RdTask({
      taskNum: 0,
      chainId: IRolldownPrimitives.ChainId.Ethereum,
      batchId: 1,
      taskCreatedBlock: uint32(block.number),
      lastCompletedOpTaskNum: 0,
      lastCompletedOpTaskCreatedBlock: opTask.taskCreatedBlock,
      lastCompletedOpTaskQuorumNumbers: quorumNumbers,
      lastCompletedOpTaskQuorumThresholdPercentage: quorumThresholdPercentage
    });

    vm.mockCall(
      address(rolldown),
      abi.encodeWithSelector(IRolldown.update_l1_from_l2.selector),
      abi.encode()
    );

    vm.roll(block.number + 10);
    vm.prank(aggregator);
    taskManager.respondToRdTask(task, taskResponse, nonSignerStakesAndSignature);

    assertEq(taskManager.isTaskPending(), false);
  }
}

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@openzeppelin-upgrades/contracts/proxy/utils/Initializable.sol";
import "@openzeppelin-upgrades/contracts/access/OwnableUpgradeable.sol";

import "@eigenlayer/contracts/permissions/Pausable.sol";

import "@eigenlayer-middleware/src/libraries/BN254.sol";
import "@eigenlayer-middleware/src/interfaces/IServiceManager.sol";

import {BLSApkRegistry} from "@eigenlayer-middleware/src/BLSApkRegistry.sol";
import {RegistryCoordinator} from "@eigenlayer-middleware/src/RegistryCoordinator.sol";
import {BLSSignatureChecker, IRegistryCoordinator} from "@eigenlayer-middleware/src/BLSSignatureChecker.sol";
import {OperatorStateRetriever} from "@eigenlayer-middleware/src/OperatorStateRetriever.sol";

import "./IFinalizerTaskManager.sol";

contract FinalizerTaskManager is
    Initializable,
    OwnableUpgradeable,
    Pausable,
    BLSSignatureChecker,
    OperatorStateRetriever,
    IFinalizerTaskManager
{
    using BN254 for BN254.G1Point;

    /* CONSTANT */
    // The number of blocks from the task initialization within which the aggregator has to respond to
    uint32 internal immutable _TASK_RESPONSE_WINDOW_BLOCK;
    uint256 internal constant _THRESHOLD_DENOMINATOR = 100;

    /* STORAGE */
    // The latest task index
    uint32 public latestTaskNum;

    // mapping of task indices to all tasks hashes
    // when a task is created, task hash is stored here,
    // and responses need to pass the actual task,
    // which is hashed onchain and checked against this mapping
    mapping(uint32 => bytes32) public allTaskHashes;

    // mapping of task indices to hash of abi.encode(taskResponse, taskResponseMetadata)
    mapping(uint32 => bytes32) public allTaskResponses;

    // mapping of task indices to hash of abi.encode(taskResponse, taskResponseMetadata)
    mapping(uint32 => TaskStatus) public indexToTaskStatus;

    uint32 lastCompletedTaskCreatedBlock;
    // uint32 lastCompletedTaskNum;
    bytes lastCompletedTaskQuorumNumbers;
    uint32 lastCompletedTaskQuorumThresholdPercentage;

    address public aggregator;
    address public generator;

    bytes32 latestPendingStateHash;

    // DATA STRUCTURES
    enum TaskStatus
    {
        // default is NOT_INITIALIZED
        NOT_INITIALIZED,
        INITIALIZED_BUT_NOT_COMPLETED,
        CANCELLED,
        COMPLETED
    }

    /* MODIFIERS */
    modifier onlyAggregator() {
        require(msg.sender == aggregator, "Aggregator must be the caller");
        _;
    }

    // onlyTaskGenerator is used to restrict createNewTask from only being called by a permissioned entity
    modifier onlyTaskGenerator() {
        require(msg.sender == generator, "Task generator must be the caller");
        _;
    }

    constructor(IRegistryCoordinator _registryCoordinator, uint32 _taskResponseWindowBlock)
        BLSSignatureChecker(_registryCoordinator)
    {
        _TASK_RESPONSE_WINDOW_BLOCK = _taskResponseWindowBlock;
    }

    function initialize(IPauserRegistry _pauserRegistry, address initialOwner, address _aggregator, address _generator)
        public
        initializer
    {
        _initializePauser(_pauserRegistry, UNPAUSE_ALL);
        _transferOwnership(initialOwner);
        aggregator = _aggregator;
        generator = _generator;
        // lastCompletedTaskQuorumNumbers.push(0) ;
        // lastCompletedTaskQuorumThresholdPercentage = 66;
    }

    /* FUNCTIONS */
    // NOTE: this function creates new task, assigns it a taskId
    function createNewTask(uint256 blockNumber, uint32 quorumThresholdPercentage, bytes calldata quorumNumbers)
        external
        onlyTaskGenerator
    {
        require(
            lastCompletedTaskCreatedBlock != block.number && block.number != 0,
            "Can't create a task in the same block as a completed task"
        );
        // create a new task struct
        Task memory newTask;
        newTask.blockNumber = blockNumber;
        newTask.taskCreatedBlock = uint32(block.number);
        newTask.quorumThresholdPercentage = quorumThresholdPercentage;
        newTask.quorumNumbers = quorumNumbers;
        newTask.lastCompletedTaskCreatedBlock = lastCompletedTaskCreatedBlock;
        newTask.lastCompletedTaskQuorumNumbers = lastCompletedTaskQuorumNumbers;
        newTask.lastCompletedTaskQuorumThresholdPercentage = lastCompletedTaskQuorumThresholdPercentage;

        // Ensure new previous task was either cancelled or completed
        // Here for now we auto cancel previous task if not completed
        if (latestTaskNum > 0) {
            uint32 lastTaskNum = latestTaskNum - 1;
            if (indexToTaskStatus[lastTaskNum] == TaskStatus.INITIALIZED_BUT_NOT_COMPLETED){
                indexToTaskStatus[lastTaskNum] = TaskStatus.CANCELLED;
            }
        }

        // store hash of task onchain, emit event, and increase taskNum
        allTaskHashes[latestTaskNum] = keccak256(abi.encode(newTask));
        indexToTaskStatus[latestTaskNum] = TaskStatus.INITIALIZED_BUT_NOT_COMPLETED;
        emit NewTaskCreated(latestTaskNum, newTask);
        latestTaskNum = latestTaskNum + 1;
    }

    // NOTE: this function responds to existing tasks.
    function respondToTask(
        Task calldata task,
        TaskResponse calldata taskResponse,
        NonSignerStakesAndSignature memory nonSignerStakesAndSignature
    ) external onlyAggregator {
        uint32 taskCreatedBlock = task.taskCreatedBlock;
        bytes calldata quorumNumbers = task.quorumNumbers;
        uint32 quorumThresholdPercentage = task.quorumThresholdPercentage;

        // check that the task is valid, hasn't been responsed yet, and is being responsed in time
        require(
            keccak256(abi.encode(task)) == allTaskHashes[taskResponse.referenceTaskIndex],
            "supplied task does not match the one recorded in the contract"
        );
        // some logical checks
        require(
            indexToTaskStatus[taskResponse.referenceTaskIndex] == TaskStatus.INITIALIZED_BUT_NOT_COMPLETED,
            "Aggregator has already responded to the task"
        );
        require(
            allTaskResponses[taskResponse.referenceTaskIndex] == bytes32(0),
            "Aggregator has already responded to the task"
        );
        require(
            uint32(block.number) <= taskCreatedBlock + _TASK_RESPONSE_WINDOW_BLOCK,
            "Aggregator has responded to the task too late"
        );

        // Maybe also redundantly check here that taskResponse.referenceTaskIndex == lastestTaskNum - 1 ( safe since createNewTask increments latestTaskNum and the only task that should be INITIALIZED_BUT_NOT_COMPLETED is the last created task)

        /* CHECKING SIGNATURES & WHETHER THRESHOLD IS MET OR NOT */
        // calculate message which operators signed
        bytes32 message = keccak256(abi.encode(taskResponse));

        // check the BLS signature
        (QuorumStakeTotals memory quorumStakeTotals, bytes32 hashOfNonSigners) =
            checkSignatures(message, quorumNumbers, taskCreatedBlock, nonSignerStakesAndSignature);

        TaskResponseMetadata memory taskResponseMetadata = TaskResponseMetadata(
            uint32(block.number),
            hashOfNonSigners,
            quorumStakeTotals.totalStakeForQuorum,
            quorumStakeTotals.signedStakeForQuorum
        );
        // updating the storage with task responsea
        allTaskResponses[taskResponse.referenceTaskIndex] = keccak256(abi.encode(taskResponse, taskResponseMetadata));

        // emitting event
        emit TaskResponded(taskResponse, taskResponseMetadata);

        // check that signatories own at least a threshold percentage of each quourm
        for (uint256 i = 0; i < quorumNumbers.length; i++) {
            // we don't check that the quorumThresholdPercentages are not >100 because a greater value would trivially fail the check, implying
            // signed stake > total stake
            if (
                quorumStakeTotals.signedStakeForQuorum[i] * _THRESHOLD_DENOMINATOR
                    < quorumStakeTotals.totalStakeForQuorum[i] * uint8(quorumThresholdPercentage)
            ) {
                // "Signatories do not own at least threshold percentage of a quorum"
                return;
            }
        }

        latestPendingStateHash = taskResponse.pendingStateHash;
        indexToTaskStatus[taskResponse.referenceTaskIndex] == TaskStatus.COMPLETED;
        lastCompletedTaskCreatedBlock = task.taskCreatedBlock;
        lastCompletedTaskQuorumNumbers = task.quorumNumbers;
        lastCompletedTaskQuorumThresholdPercentage = task.quorumThresholdPercentage;
        // emitting completed event
        emit TaskCompleted(taskResponse.referenceTaskIndex, taskResponse.blockHash);
    }

    function taskNumber() external view returns (uint32) {
        return latestTaskNum;
    }

    function getTaskResponseWindowBlock() external view returns (uint32) {
        return _TASK_RESPONSE_WINDOW_BLOCK;
    }

    function getLatestPendingStateHash() external view returns (bytes32) {
        return latestPendingStateHash;
    }
}

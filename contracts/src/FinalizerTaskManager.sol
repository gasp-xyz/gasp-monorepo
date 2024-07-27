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
import "./IGaspMultiRollupServicePrimitives.sol";

import "./IFinalizerTaskManager.sol";

contract FinalizerTaskManager is
    Initializable,
    OwnableUpgradeable,
    Pausable,
    OperatorStateRetriever,
    IFinalizerTaskManager
{
    using BN254 for BN254.G1Point;

    BLSSignatureChecker public blsSignatureChecker;

    /* CONSTANT */
    // The number of blocks from the task initialization within which the aggregator has to respond to
    uint32 public TASK_RESPONSE_WINDOW_BLOCK;
    uint256 public constant THRESHOLD_DENOMINATOR = 100;

    /* STORAGE */
    // The latest task index
    uint32 public latestOpTaskNum;
    uint32 public latestRdTaskNum;

    // mapping of task indices to all tasks hashes
    // when a task is created, task hash is stored here,
    // and responses need to pass the actual task,
    // which is hashed onchain and checked against this mapping
    mapping(TaskType => mapping(uint32 => bytes32)) public allTaskHashes;

    // mapping of task indices to hash of abi.encode(taskResponse, taskResponseMetadata)
    mapping(TaskType => mapping(uint32 => bytes32)) public allTaskResponses;

    mapping(TaskType => mapping(uint32 => TaskStatus)) public idToTaskStatus;

    uint32 public lastCompletedOpTaskNum;
    uint32 public lastCompletedOpTaskCreatedBlock;
    // uint32 lastCompletedTaskNum;
    bytes public lastCompletedOpTaskQuorumNumbers;
    uint32 public lastCompletedOpTaskQuorumThresholdPercentage;

    address public aggregator;
    address public generator;

    bytes32 public latestPendingStateHash;

    bool public allowNonRootInit;

    // DATA STRUCTURES
    enum TaskStatus
    {
        // default is NOT_INITIALIZED
        NOT_INITIALIZED,
        INITIALIZED,
        CANCELLED,
        RESPONDED,
        COMPLETED
    }
    enum TaskType
    {
        // default is OpTask
        OP_TASK,
        RD_TASK
    }

    /* MODIFIERS */
    modifier onlyAggregator() {
        require(msg.sender == aggregator, "Auth0");
        _;
    }

    // onlyTaskGenerator is used to restrict createNewTask from only being called by a permissioned entity
    modifier onlyTaskGenerator() {
        require(msg.sender == generator, "Auth1");
        _;
    }

    function initialize(IPauserRegistry _pauserRegistry, address initialOwner, address _aggregator, address _generator, bool _allowNonRootInit, address _blsSignatureCheckerAddress, uint32 _taskResponseWindowBlock)
        public
        initializer
    {
        _initializePauser(_pauserRegistry, UNPAUSE_ALL);
        _transferOwnership(initialOwner);
        aggregator = _aggregator;
        generator = _generator;
        allowNonRootInit = _allowNonRootInit;
        blsSignatureChecker = BLSSignatureChecker(_blsSignatureCheckerAddress);
        TASK_RESPONSE_WINDOW_BLOCK = _taskResponseWindowBlock;
    }

    function update_bls_signature_checker_address(address _blsSignatureCheckerAddress) external onlyOwner{
        blsSignatureChecker = BLSSignatureChecker(_blsSignatureCheckerAddress);
        emit BLSSignatureCheckerAddressUpdated(_blsSignatureCheckerAddress);
    }

    // TODO!!!
    // DEDUP ALL THIS!
    
    /* FUNCTIONS */
    // NOTE: this function creates new task, assigns it a taskId
    function createNewOpTask(uint32 quorumThresholdPercentage, bytes calldata quorumNumbers)
        external
        onlyTaskGenerator
    {
        require(
            lastCompletedOpTaskCreatedBlock != block.number && block.number != 0,
            "Can't create a task in the same block as a completed task"
        );
        // create a new task struct
        OperatorUpdateTask memory newTask;
        newTask.taskNum = latestOpTaskNum;
        newTask.taskCreatedBlock = uint32(block.number);
        newTask.quorumThresholdPercentage = quorumThresholdPercentage;
        newTask.quorumNumbers = quorumNumbers;
        newTask.lastCompletedOpTaskCreatedBlock = lastCompletedOpTaskCreatedBlock;
        newTask.lastCompletedOpTaskQuorumNumbers = lastCompletedOpTaskQuorumNumbers;
        newTask.lastCompletedOpTaskQuorumThresholdPercentage = lastCompletedOpTaskQuorumThresholdPercentage;

        // Ensure new previous task was either cancelled or completed
        // Here for now we auto cancel previous task if not completed
        if (latestOpTaskNum > 0) {
            uint32 lastTaskNum = latestOpTaskNum - 1;
            if (idToTaskStatus[TaskType.OP_TASK][lastTaskNum] == TaskStatus.INITIALIZED){
                idToTaskStatus[TaskType.OP_TASK][lastTaskNum] = TaskStatus.CANCELLED;
            }
        }

        // store hash of task onchain, emit event, and increase taskNum
        allTaskHashes[TaskType.OP_TASK][latestOpTaskNum] = keccak256(abi.encode(newTask));
        idToTaskStatus[TaskType.OP_TASK][latestOpTaskNum] = TaskStatus.INITIALIZED;
        emit NewOpTaskCreated(latestOpTaskNum, newTask);
        latestOpTaskNum = latestOpTaskNum + 1;
    }

    // NOTE: this function responds to existing tasks.
    function respondToOpTask(
        OperatorUpdateTask calldata task,
        OperatorUpdateTaskResponse calldata taskResponse,
        BLSSignatureChecker.NonSignerStakesAndSignature memory nonSignerStakesAndSignature
    ) external {
        uint32 taskReferenceBlock = task.lastCompletedOpTaskCreatedBlock;

        if (taskReferenceBlock == 0) {
            if (allowNonRootInit) {
                require(msg.sender == aggregator, "Auth0");
            } else {
                require(msg.sender == owner(), "Auth2");
            }
        } else {
            require(msg.sender == aggregator, "Auth0");
        }


        uint32 taskCreatedBlock = task.taskCreatedBlock;
        bytes calldata quorumNumbers = task.lastCompletedOpTaskQuorumNumbers;
        uint32 quorumThresholdPercentage = task.lastCompletedOpTaskQuorumThresholdPercentage;

        // check that the task is valid, hasn't been responsed yet, and is being responsed in time
        require(
            keccak256(abi.encode(task)) == allTaskHashes[TaskType.OP_TASK][taskResponse.referenceTaskIndex],
            "supplied task does not match the one recorded in the contract"
        );
        // some logical checks
        require(
            idToTaskStatus[TaskType.OP_TASK][taskResponse.referenceTaskIndex] == TaskStatus.INITIALIZED,
            "Aggregator has already responded to the task"
        );
        require(
            allTaskResponses[TaskType.OP_TASK][taskResponse.referenceTaskIndex] == bytes32(0),
            "Aggregator has already responded to the task"
        );
        require(
            uint32(block.number) <= taskCreatedBlock + TASK_RESPONSE_WINDOW_BLOCK,
            "Aggregator has responded to the task too late"
        );

        // Maybe also redundantly check here that taskResponse.referenceTaskIndex == lastestTaskNum - 1 ( safe since createNewTask increments latestTaskNum and the only task that should be INITIALIZED is the last created task)

        /* CHECKING SIGNATURES & WHETHER THRESHOLD IS MET OR NOT */
        // calculate message which operators signed
        bytes32 message = keccak256(abi.encode(taskResponse));

        BLSSignatureChecker.QuorumStakeTotals memory quorumStakeTotals; bytes32 hashOfNonSigners;

        if (taskReferenceBlock != 0) {
            // check the BLS signature
            (quorumStakeTotals, hashOfNonSigners) =
                blsSignatureChecker.checkSignatures(message, quorumNumbers, taskReferenceBlock, nonSignerStakesAndSignature);
        }

        TaskResponseMetadata memory taskResponseMetadata = TaskResponseMetadata(
            uint32(block.number),
            hashOfNonSigners,
            quorumStakeTotals.totalStakeForQuorum,
            quorumStakeTotals.signedStakeForQuorum
        );
        // updating the storage with task responsea
        allTaskResponses[TaskType.OP_TASK][taskResponse.referenceTaskIndex] = keccak256(abi.encode(taskResponse, taskResponseMetadata));
        idToTaskStatus[TaskType.OP_TASK][taskResponse.referenceTaskIndex] == TaskStatus.RESPONDED;

        // emitting event
        emit OpTaskResponded(task.taskNum, taskResponse, taskResponseMetadata);


        if (taskReferenceBlock != 0) {
            // check that signatories own at least a threshold percentage of each quourm
            for (uint256 i = 0; i < quorumNumbers.length; i++) {
                // we don't check that the quorumThresholdPercentages are not >100 because a greater value would trivially fail the check, implying
                // signed stake > total stake
                if (
                    quorumStakeTotals.signedStakeForQuorum[i] * THRESHOLD_DENOMINATOR
                        < quorumStakeTotals.totalStakeForQuorum[i] * uint8(quorumThresholdPercentage)
                ) {
                    // "Signatories do not own at least threshold percentage of a quorum"
                    return;
                }
            }
        }

        idToTaskStatus[TaskType.OP_TASK][taskResponse.referenceTaskIndex] == TaskStatus.COMPLETED;
        lastCompletedOpTaskCreatedBlock = task.taskCreatedBlock;
        lastCompletedOpTaskQuorumNumbers = task.quorumNumbers;
        lastCompletedOpTaskQuorumThresholdPercentage = task.quorumThresholdPercentage;
        lastCompletedOpTaskNum = task.taskNum;
        // emitting completed event
        emit OpTaskCompleted(taskResponse.referenceTaskIndex, taskResponse);
    }


    /* FUNCTIONS */
    // NOTE: this function creates new task, assigns it a taskId
    function createNewRdTask(uint256 blockNumber)
        external
        onlyTaskGenerator
    {
        require(
            lastCompletedOpTaskCreatedBlock != 0 && block.number != 0,
            "Op State uninit"
        );
        // create a new task struct
        RolldownUpdateTask memory newTask;
        newTask.taskNum = latestRdTaskNum;
        newTask.blockNumber = blockNumber;
        newTask.taskCreatedBlock = uint32(block.number);
        newTask.lastCompletedOpTaskQuorumThresholdPercentage = lastCompletedOpTaskQuorumThresholdPercentage;
        newTask.lastCompletedOpTaskQuorumNumbers = lastCompletedOpTaskQuorumNumbers;
        newTask.lastCompletedOpTaskCreatedBlock = lastCompletedOpTaskCreatedBlock;

        // Ensure new previous task was either cancelled or completed
        // Here for now we auto cancel previous task if not completed
        if (latestRdTaskNum > 0) {
            uint32 lastTaskNum = latestRdTaskNum - 1;
            if (idToTaskStatus[TaskType.RD_TASK][lastTaskNum] == TaskStatus.INITIALIZED){
                idToTaskStatus[TaskType.RD_TASK][lastTaskNum] = TaskStatus.CANCELLED;
            }
        }

        // store hash of task onchain, emit event, and increase taskNum
        allTaskHashes[TaskType.RD_TASK][latestRdTaskNum] = keccak256(abi.encode(newTask));
        idToTaskStatus[TaskType.RD_TASK][latestRdTaskNum] = TaskStatus.INITIALIZED;
        emit NewRdTaskCreated(latestRdTaskNum, newTask);
        latestRdTaskNum = latestRdTaskNum + 1;
    }

    // NOTE: this function responds to existing tasks.
    function respondToRolldownUpdateTask(
        RolldownUpdateTask calldata task,
        RolldownUpdateTaskResponse calldata taskResponse,
        BLSSignatureChecker.NonSignerStakesAndSignature memory nonSignerStakesAndSignature
    ) external onlyAggregator {
        uint32 taskReferenceBlock = task.lastCompletedOpTaskCreatedBlock;
        uint32 taskCreatedBlock = task.taskCreatedBlock;
        bytes calldata quorumNumbers = task.lastCompletedOpTaskQuorumNumbers;
        uint32 quorumThresholdPercentage = task.lastCompletedOpTaskQuorumThresholdPercentage;

        // check that the task is valid, hasn't been responsed yet, and is being responsed in time
        require(
            keccak256(abi.encode(task)) == allTaskHashes[TaskType.RD_TASK][taskResponse.referenceTaskIndex],
            "supplied task does not match the one recorded in the contract"
        );
        // some logical checks
        require(
            idToTaskStatus[TaskType.RD_TASK][taskResponse.referenceTaskIndex] == TaskStatus.INITIALIZED,
            "Aggregator has already responded to the task"
        );
        require(
            allTaskResponses[TaskType.RD_TASK][taskResponse.referenceTaskIndex] == bytes32(0),
            "Aggregator has already responded to the task"
        );
        require(
            uint32(block.number) <= taskCreatedBlock + TASK_RESPONSE_WINDOW_BLOCK,
            "Aggregator has responded to the task too late"
        );

        // Maybe also redundantly check here that taskResponse.referenceTaskIndex == lastestTaskNum - 1 ( safe since createNewTask increments latestTaskNum and the only task that should be INITIALIZED is the last created task)

        /* CHECKING SIGNATURES & WHETHER THRESHOLD IS MET OR NOT */
        // calculate message which operators signed
        bytes32 message = keccak256(abi.encode(taskResponse));

        // check the BLS signature
        (BLSSignatureChecker.QuorumStakeTotals memory quorumStakeTotals, bytes32 hashOfNonSigners) =
            blsSignatureChecker.checkSignatures(message, quorumNumbers, taskReferenceBlock, nonSignerStakesAndSignature);

        TaskResponseMetadata memory taskResponseMetadata = TaskResponseMetadata(
            uint32(block.number),
            hashOfNonSigners,
            quorumStakeTotals.totalStakeForQuorum,
            quorumStakeTotals.signedStakeForQuorum
        );
        // updating the storage with task responses
        allTaskResponses[TaskType.RD_TASK][taskResponse.referenceTaskIndex] = keccak256(abi.encode(taskResponse, taskResponseMetadata));
        idToTaskStatus[TaskType.RD_TASK][taskResponse.referenceTaskIndex] == TaskStatus.RESPONDED;

        // emitting event
        emit RdTaskResponded(task.taskNum, taskResponse, taskResponseMetadata);

        // check that signatories own at least a threshold percentage of each quourm
        for (uint256 i = 0; i < quorumNumbers.length; i++) {
            // we don't check that the quorumThresholdPercentages are not >100 because a greater value would trivially fail the check, implying
            // signed stake > total stake
            if (
                quorumStakeTotals.signedStakeForQuorum[i] * THRESHOLD_DENOMINATOR
                    < quorumStakeTotals.totalStakeForQuorum[i] * uint8(quorumThresholdPercentage)
            ) {
                // "Signatories do not own at least threshold percentage of a quorum"
                return;
            }
        }

        latestPendingStateHash = taskResponse.pendingStateHash;
        idToTaskStatus[TaskType.RD_TASK][taskResponse.referenceTaskIndex] == TaskStatus.COMPLETED;

        // emitting completed event
        emit RdTaskCompleted(taskResponse.referenceTaskIndex, taskResponse.blockHash, taskResponse);
    }
    
    function dummyForOperatorStateInfoType(IGaspMultiRollupServicePrimitives.OperatorStateInfo calldata _operatorStateInfo) external view {
    }
}

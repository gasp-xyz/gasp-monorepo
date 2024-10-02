// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@eigenlayer-middleware/src/libraries/BN254.sol";
import "./IGaspMultiRollupServicePrimitives.sol";
import {IRolldown} from "./IRolldown.sol";

interface IFinalizerTaskManager {
    // EVENTS

    event PauseTrackingOpState();
    event ResumeTrackingOpState(bool resetTrackedQuorums);

    event BLSSignatureCheckerAddressUpdated(address blsSignatureCheckerAddress);

    event OpTaskCancelled(uint32 indexed taskIndex);
    event NewOpTaskCreated(uint32 indexed taskIndex, OpTask task);
    event NewOpTaskForceCreated();

    // When we have some response from OPs
    // note we want to keep track of responded tasks that did not meet the completion criteria 
    event OpTaskResponded(
        uint32 indexed taskIndex,
        OpTaskResponse taskResponse,
        TaskResponseMetadata taskResponseMetadata
    );

    // When aggregated stake for OP's responses exceeds the required threshold
    event OpTaskCompleted(uint32 indexed taskIndex,
        OpTaskResponse taskResponse);

    event OpTaskForceCompleted(uint32 indexed taskIndex,
        OpTaskResponse taskResponse);

    event RdTaskCancelled(uint32 indexed taskIndex);
    event NewRdTaskCreated(uint32 indexed taskIndex, RdTask task);

    // When we have some response from OPs
    // note we want to keep track of responded tasks that did not meet the completion criteria 
    event RdTaskResponded(
        uint32 indexed taskIndex,
        RdTaskResponse taskResponse,
        TaskResponseMetadata taskResponseMetadata
    );

    // When aggregated stake for OP's responses exceeds the required threshold
    event RdTaskCompleted(uint32 indexed taskIndex,
        RdTaskResponse taskResponse);

    event RolldownTargetUpdated(address rolldownAddress);

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
    
    struct OpTask {
        // the task number
        uint32 taskNum;
        // used for expiration checks
        uint32 taskCreatedBlock;
        // The last completed task used as reference block for operator state on other L1s
        uint32 lastCompletedOpTaskCreatedBlock;
        // task submitter decides on the criteria for a task to be completed
        // note that this does not mean the task was "correctly" answered
        // task is completed when each quorumNumbers specified here
        // are signed by at least quorumThresholdPercentage of the operators
        // note that we set the quorumThresholdPercentage to be the same for all quorumNumbers, but this could be changed
        bytes quorumNumbers;
        // percentage of quorum's total stake needed to consider task completed
        uint32 quorumThresholdPercentage;
        // We require these to validate the old state correctly
        bytes lastCompletedOpTaskQuorumNumbers;
        uint32 lastCompletedOpTaskQuorumThresholdPercentage;
    }

    // Task response is hashed and signed by operators.
    // these signatures are aggregated and sent to the contract as response.
    struct OpTaskResponse {
        // Can be obtained by the operator from the event NewTaskCreated.
        uint32 referenceTaskIndex;
        bytes32 referenceTaskHash;

        bytes32 operatorsStateInfoHash;
    }

    struct RdTask {
        // the task number
        uint32 taskNum;
        IRolldown.ChainId chainId;
        uint32 batchId;
        // used for expiration checks
        uint32 taskCreatedBlock;
        // The last completed task used as reference block for operator state on other L1s
        uint32 lastCompletedOpTaskCreatedBlock;
        // We require these to validate the old state correctly
        bytes lastCompletedOpTaskQuorumNumbers;
        uint32 lastCompletedOpTaskQuorumThresholdPercentage;
    }

    // Task response is hashed and signed by operators.
    // these signatures are aggregated and sent to the contract as response.
    struct RdTaskResponse {
        // Can be obtained by the operator from the event NewTaskCreated.
        uint32 referenceTaskIndex;
        bytes32 referenceTaskHash;

        IRolldown.ChainId chainId;
        uint32 batchId;
        bytes32 rdUpdate;
        uint256 rangeStart;
        uint256 rangeEnd;
        address updater;
    }


    // Extra information related to taskResponse, which is filled inside the contract.
    // It thus cannot be signed by operators, so we keep it in a separate struct than TaskResponse
    // This metadata serves informative purposes for now - emitted in TaskResponded event
    struct TaskResponseMetadata {
        uint32 taskResponsedBlock;
        bytes32 hashOfNonSigners;
        // current total stake for quorums
        uint96[] quroumStakeTotals;
        // quorum stake signed by the majority of OPs
        uint96[] quroumStakeSigned;
    }

    // FUNCTIONS

}

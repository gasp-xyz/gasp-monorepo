// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@eigenlayer-middleware/src/libraries/BN254.sol";
import "./IGaspMultiRollupServicePrimitives.sol";

interface IFinalizerTaskManager {
    // EVENTS
    event NewTaskCreated(uint32 indexed taskIndex, Task task);

    // When we have some response from OPs
    // note we want to keep track of responded tasks that did not meet the completion criteria 
    event TaskResponded(
        TaskResponse taskResponse,
        TaskResponseMetadata taskResponseMetadata
    );

    // When aggregated stake for OP's responses exceeds the required threshold
    event TaskCompleted(uint32 indexed taskIndex, bytes32 indexed blockHash);

    // STRUCTS
    struct Task {
        // L2 block number which operators are required to execute and provide proofs for
        uint256 blockNumber;
        // used for expiration checks
        uint32 taskCreatedBlock;
        // The last completed task used as reference block for operator state on other L1s
        uint32 lastCompletedTaskCreatedBlock;
        // task submitter decides on the criteria for a task to be completed
        // note that this does not mean the task was "correctly" answered
        // task is completed when each quorumNumbers specified here
        // are signed by at least quorumThresholdPercentage of the operators
        // note that we set the quorumThresholdPercentage to be the same for all quorumNumbers, but this could be changed
        bytes quorumNumbers;
        // percentage of quorum's total stake needed to consider task completed
        uint32 quorumThresholdPercentage;
        // We require these to validate the old state correctly
        bytes lastCompletedTaskQuorumNumbers;
        uint32 lastCompletedTaskQuorumThresholdPercentage;
    }

    // Task response is hashed and signed by operators.
    // these signatures are aggregated and sent to the contract as response.
    struct TaskResponse {
        // Can be obtained by the operator from the event NewTaskCreated.
        uint32 referenceTaskIndex;
        Task referenceTask;

        OperatorStateInfo operatorsStateInfo;
        // This is the response that the operator has to provide for a finalized block.
        bytes32 blockHash;
        // This is the response that the operator has to provide for a an executed block.
        bytes32 storageProofHash;
        // This is the response that the operator has to provide for a state hash at given block.
        bytes32 pendingStateHash;
    }

    struct OperatorStateInfo {
        bool operatorsStateChanged;
        bool operatorsStateProvided;
        // uint8 quorumCountUpdate;
        
        uint8[] quorumsRemoved;
        IGaspMultiRollupServicePrimitives.QuorumsAdded[] quorumsAdded;
        IGaspMultiRollupServicePrimitives.QuorumsStakeUpdate[] quorumsStakeUpdate;
        IGaspMultiRollupServicePrimitives.QuorumsApkUpdate[] quorumsApkUpdate;

        bytes32[] OperatorsRemoved;
        IGaspMultiRollupServicePrimitives.OperatorsAdded[] operatorsAdded; // Sorted!
        IGaspMultiRollupServicePrimitives.OperatorsStakeUpdate[] operatorsStakeUpdate;
        IGaspMultiRollupServicePrimitives.OperatorsQuorumCountUpdate[] operatorsQuorumCountUpdate;


        // IGaspMultiRollupServicePrimitives.QuorumsApkUpdate quorumApkUpdate;
        // IGaspMultiRollupServicePrimitives.QuorumsStakeUpdate quorumsStakeUpdate;
        // IGaspMultiRollupServicePrimitives.OperatorStakeUpdate[] OperatorStakeUpdate;
        // IGaspMultiRollupServicePrimitives.OperatorKeyUpdate[] operatorKeyUpdate;
        // IGaspMultiRollupServicePrimitives.QuorumOperatorsUpdate[] quorumOperatorsUpdate;
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
    // NOTE: this function creates new task.
    function createNewTask(
        uint256 blockNumber,
        uint32 quorumThresholdPercentage,
        bytes calldata quorumNumbers
    ) external;

    /// @notice Returns the current 'taskNumber' for the middleware
    function taskNumber() external view returns (uint32);

    /// @notice Returns the TASK_RESPONSE_WINDOW_BLOCK
    function getTaskResponseWindowBlock() external view returns (uint32);
}

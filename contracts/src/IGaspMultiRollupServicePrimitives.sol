// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@eigenlayer-middleware/src/libraries/BN254.sol";

interface IGaspMultiRollupServicePrimitives {

    event EigenUpdateProcessed(
        uint32 taskNumber,
        uint32 taskCreatedBlock
    );

    // struct OperatorStakeUpdate{
    //     address operator;
    //     QuorumsStakeUpdate quorumsStakeUpdate;
    // }

    // struct OperatorKeyUpdate {
    //     address operator;
    //     BN254.G1Point g1Key;
    // }

    // struct QuorumsStakeUpdate{
    //     uint8[] quorumsToUpdate;
    //     uint96[] qourumsUpdatedStake;
    // }
    // struct QuorumsApkUpdate{
    //     uint8[] quorumsToUpdate;
    //     BN254.G1Point[] qourumsUpdatedApk;
    // }

    // struct QuorumOperatorsUpdate{
    //     uint8 quorumToUpdate;
    //     // Each entry here must have a corresponding one in 
    //     address[] operatorAdded;
    //     address[] operatorRemoved;
    // }


    struct NonSignerStakesAndSignatureForOldState {
        // uint32[] nonSignerPubkeysIndicesforOperatorIdsRemovedForOldState;
        // BN254.G1Point[] nonSignerPubkeysAddedForOldState;
        BN254.G1Point[] nonSignerG1PubkeysForOldState;
        BN254.G2Point apkG2ForOldState;
        BN254.G1Point sigmaForOldState;
    }


    struct OperatorStateInfo {
        bool operatorsStateChanged;
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

    struct QuorumsAdded{
        uint8 quorumNumber;
        uint96 quorumStake;
        BN254.G1Point quorumApk;
    }

    struct QuorumsStakeUpdate{
        uint8 quorumNumber;
        uint96 quorumStake;
    }
    struct QuorumsApkUpdate{
        uint8 quorumNumber;
        BN254.G1Point quorumApk;
    }

    struct OperatorsAdded {
        bytes32 operatorId;
        uint8[] quorumForStakes;
        uint96[] quorumStakes;
        // Maybe remove and use quorumForStakes.len()?
        uint8 quorumCount;
    }

    struct OperatorsStakeUpdate{
        bytes32 operatorId;
        uint8[] quorumForStakes;
        uint96[] quorumStakes;
    }

    struct OperatorsQuorumCountUpdate{
        bytes32 operatorId;
        uint8 quorumCount;
    }

}

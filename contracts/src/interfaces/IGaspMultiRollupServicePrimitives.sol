// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {BN254} from "@eigenlayer-middleware/src/libraries/BN254.sol";

interface IGaspMultiRollupServicePrimitives {
    event EigenOpUpdateProcessed(uint32 taskNumber, uint32 taskCreatedBlock);
    event EigenRdUpdateProcessed(uint32 taskNumber, uint32 taskCreatedBlock);
    event EigenReinitProcessed(uint32 taskNumber, uint32 taskCreatedBlock);

    event RolldownTargetUpdated(address rolldownAddress);

    struct OperatorStateInfo {
        bool operatorsStateChanged;
        uint8[] quorumsRemoved;
        IGaspMultiRollupServicePrimitives.QuorumsAdded[] quorumsAdded;
        IGaspMultiRollupServicePrimitives.QuorumsStakeUpdate[] quorumsStakeUpdate;
        IGaspMultiRollupServicePrimitives.QuorumsApkUpdate[] quorumsApkUpdate;
        bytes32[] operatorsRemoved;
        IGaspMultiRollupServicePrimitives.OperatorsAdded[] operatorsAdded; // Sorted!
        IGaspMultiRollupServicePrimitives.OperatorsStakeUpdate[] operatorsStakeUpdate;
        IGaspMultiRollupServicePrimitives.OperatorsQuorumCountUpdate[] operatorsQuorumCountUpdate;
    }

    struct QuorumsAdded {
        uint8 quorumNumber;
        uint96 quorumStake;
        BN254.G1Point quorumApk;
    }

    struct QuorumsStakeUpdate {
        uint8 quorumNumber;
        uint96 quorumStake;
    }

    struct QuorumsApkUpdate {
        uint8 quorumNumber;
        BN254.G1Point quorumApk;
    }

    struct OperatorsAdded {
        bytes32 operatorId;
        uint8[] quorumForStakes;
        uint96[] quorumStakes;
        uint8 quorumCount;
    }

    struct OperatorsStakeUpdate {
        bytes32 operatorId;
        uint8[] quorumForStakes;
        uint96[] quorumStakes;
    }

    struct OperatorsQuorumCountUpdate {
        bytes32 operatorId;
        uint8 quorumCount;
    }
}

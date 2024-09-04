// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/utils/Address.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "forge-std/console.sol";

import {BN254} from "@eigenlayer-middleware/src/libraries/BN254.sol";
import "@openzeppelin-upgrades/contracts/proxy/utils/Initializable.sol";
import "@openzeppelin-upgrades/contracts/access/OwnableUpgradeable.sol";
import "@eigenlayer/contracts/permissions/Pausable.sol";
import "./GaspMultiRollupServiceStorage.sol";
import "@eigenlayer-middleware/src/interfaces/IBLSSignatureChecker.sol";
import "./IFinalizerTaskManager.sol";
import {IRolldown} from "./IRolldown.sol";
 
contract GaspMultiRollupService is
    Initializable,
    OwnableUpgradeable,
    Pausable,
    GaspMultiRollupServiceStorage
{
    using BN254 for BN254.G1Point;

    uint256 internal constant _THRESHOLD_DENOMINATOR = 100;
    uint256 internal constant PAIRING_EQUALITY_CHECK_GAS = 120000;

    function initialize(IPauserRegistry _pauserRegistry, address initialOwner, address _updater, bool _allowNonRootInit, IRolldown _rolldown)
        public
        initializer
    {
        _initializePauser(_pauserRegistry, UNPAUSE_ALL);
        _transferOwnership(initialOwner);
        updater = _updater;
        allowNonRootInit = _allowNonRootInit;
        rolldown = _rolldown;
    }

    /* MODIFIERS */
    modifier onlyUpdater() {
        require(msg.sender == updater, "Updater must be the caller");
        _;
    }

    function set_updater(address _updater) public onlyOwner {
        updater = _updater;
    }

    function setRolldown(IRolldown _rolldown) external whenNotPaused onlyOwner {
      rolldown = _rolldown;
      emit RolldownTargetUpdated(address(_rolldown));
    }

    function process_eigen_reinit(IFinalizerTaskManager.OpTask calldata task, OperatorStateInfo calldata operatorStateInfo, bytes32[] calldata merkleRoots, IRolldown.Range[] calldata ranges) public onlyOwner{

        require(merkleRoots.length == ranges.length, "rdUpdate info length mismatch");

        for (uint256 i = 0; i < operatorStateInfo.quorumsRemoved.length; i++) {
            delete quorumToStakes[operatorStateInfo.quorumsRemoved[i]];
            delete qourumApk[operatorStateInfo.quorumsRemoved[i]];
        }

        for (uint256 i = 0; i < operatorStateInfo.quorumsAdded.length; i++) {
            quorumToStakes[operatorStateInfo.quorumsAdded[i].quorumNumber] = operatorStateInfo.quorumsAdded[i].quorumStake;
            qourumApk[operatorStateInfo.quorumsAdded[i].quorumNumber] = operatorStateInfo.quorumsAdded[i].quorumApk;
        }

        for (uint256 i = 0; i < operatorStateInfo.quorumsStakeUpdate.length; i++) {
            quorumToStakes[operatorStateInfo.quorumsStakeUpdate[i].quorumNumber] = operatorStateInfo.quorumsStakeUpdate[i].quorumStake;
        }

        for (uint256 i = 0; i < operatorStateInfo.quorumsApkUpdate.length; i++) {
            qourumApk[operatorStateInfo.quorumsApkUpdate[i].quorumNumber] = operatorStateInfo.quorumsApkUpdate[i].quorumApk;
        }

        for (uint256 i = 0; i < operatorStateInfo.operatorsRemoved.length; i++) {
            for (uint256 j = 0; j < quorumNumbers.length; i++) {
               delete operatorAndQuorumToStakes[operatorStateInfo.operatorsRemoved[i]][uint8(quorumNumbers[j])];
            }
            delete operatorIdQuorumCount[operatorStateInfo.operatorsRemoved[i]];
        }

        for (uint256 i = 0; i < operatorStateInfo.operatorsAdded.length; i++) {
            operatorIdQuorumCount[operatorStateInfo.operatorsAdded[i].operatorId] = operatorStateInfo.operatorsAdded[i].quorumCount;

            for (uint256 j = 0; j < operatorStateInfo.operatorsAdded[i].quorumForStakes.length; j++) {
                operatorAndQuorumToStakes[operatorStateInfo.operatorsAdded[i].operatorId][operatorStateInfo.operatorsAdded[i].quorumForStakes[j]] = operatorStateInfo.operatorsAdded[i].quorumStakes[j];
            }
        }

        for (uint256 i = 0; i < operatorStateInfo.operatorsStakeUpdate.length; i++) {

            for (uint256 j = 0; j < operatorStateInfo.operatorsStakeUpdate[i].quorumForStakes.length; j++) {
                operatorAndQuorumToStakes[operatorStateInfo.operatorsStakeUpdate[i].operatorId][operatorStateInfo.operatorsStakeUpdate[i].quorumForStakes[j]] = operatorStateInfo.operatorsStakeUpdate[i].quorumStakes[j];
            }
        }

        for (uint256 i = 0; i < operatorStateInfo.operatorsQuorumCountUpdate.length; i++) {
            operatorIdQuorumCount[operatorStateInfo.operatorsQuorumCountUpdate[i].operatorId] = operatorStateInfo.operatorsQuorumCountUpdate[i].quorumCount;
        }

        latestCompletedOpTaskNumber = task.taskNum;
        latestCompletedOpTaskCreatedBlock = task.taskCreatedBlock;
        lastOpUpdateBlockTimestamp = block.timestamp;

        quorumNumbers = task.quorumNumbers;
        quorumThresholdPercentage = task.quorumThresholdPercentage;

        for (uint256 i = 0; i < merkleRoots.length; i++) {
            rolldown.update_l1_from_l2(merkleRoots[i], ranges[i]);
        }

        emit EigenReinitProcessed(task.taskNum, task.taskCreatedBlock);
        
    }

    function process_eigen_op_update(IFinalizerTaskManager.OpTask calldata task, IFinalizerTaskManager.OpTaskResponse calldata taskResponse, IBLSSignatureChecker.NonSignerStakesAndSignature calldata nonSignerStakesAndSignature, OperatorStateInfo calldata operatorStateInfo) public onlyUpdater  {


        bool isInit = latestCompletedOpTaskCreatedBlock == 0;

        if (isInit) {
            if (allowNonRootInit) {
                require(msg.sender == updater, "Auth0");
            } else {
                require(msg.sender == owner(), "Auth2");
            }
        } else {
            require(msg.sender == updater, "Auth0");
        }

        require(taskResponse.referenceTaskHash == keccak256(abi.encode(task)), "referenceTaskHash hash mismatch");
        require(taskResponse.operatorsStateInfoHash == keccak256(abi.encode(operatorStateInfo)), "operatorStateInfo hash mismatch");

        if (!isInit) {
        require(latestCompletedOpTaskCreatedBlock == task.lastCompletedOpTaskCreatedBlock, "reference block mismatch");
        require(latestCompletedOpTaskCreatedBlock + 14400 > task.taskCreatedBlock, "stale state 0");
        require(lastOpUpdateBlockTimestamp + 3 days > block.timestamp, "stale state 1");

        
        // if the this is the first task then don't check sigs
        IBLSSignatureChecker.QuorumStakeTotals memory quorumStakeTotals = checkSignatures(keccak256(abi.encode(taskResponse)), nonSignerStakesAndSignature);

        uint32 QuorumThresholdPercentage = quorumThresholdPercentage;
        // check that signatories own at least a threshold percentage of each quourm
        for (uint256 i = 0; i < quorumNumbers.length; i++) {
            // we don't check that the quorumThresholdPercentages are not >100 because a greater value would trivially fail the check, implying
            // signed stake > total stake
            if (
                quorumStakeTotals.signedStakeForQuorum[i] * _THRESHOLD_DENOMINATOR
                    < quorumStakeTotals.totalStakeForQuorum[i] * uint8(QuorumThresholdPercentage)
            ) {
                // "Signatories do not own at least threshold percentage of a quorum"
                revert("Failed to meet quorum");
            }
        }
        }

        for (uint256 i = 0; i < operatorStateInfo.quorumsRemoved.length; i++) {
            delete quorumToStakes[operatorStateInfo.quorumsRemoved[i]];
            delete qourumApk[operatorStateInfo.quorumsRemoved[i]];
        }

        for (uint256 i = 0; i < operatorStateInfo.quorumsAdded.length; i++) {
            quorumToStakes[operatorStateInfo.quorumsAdded[i].quorumNumber] = operatorStateInfo.quorumsAdded[i].quorumStake;
            qourumApk[operatorStateInfo.quorumsAdded[i].quorumNumber] = operatorStateInfo.quorumsAdded[i].quorumApk;
        }

        for (uint256 i = 0; i < operatorStateInfo.quorumsStakeUpdate.length; i++) {
            quorumToStakes[operatorStateInfo.quorumsStakeUpdate[i].quorumNumber] = operatorStateInfo.quorumsStakeUpdate[i].quorumStake;
        }

        for (uint256 i = 0; i < operatorStateInfo.quorumsApkUpdate.length; i++) {
            qourumApk[operatorStateInfo.quorumsApkUpdate[i].quorumNumber] = operatorStateInfo.quorumsApkUpdate[i].quorumApk;
        }

        for (uint256 i = 0; i < operatorStateInfo.operatorsRemoved.length; i++) {
            for (uint256 j = 0; j < quorumNumbers.length; i++) {
               delete operatorAndQuorumToStakes[operatorStateInfo.operatorsRemoved[i]][uint8(quorumNumbers[j])];
            }
            delete operatorIdQuorumCount[operatorStateInfo.operatorsRemoved[i]];
        }

        for (uint256 i = 0; i < operatorStateInfo.operatorsAdded.length; i++) {
            operatorIdQuorumCount[operatorStateInfo.operatorsAdded[i].operatorId] = operatorStateInfo.operatorsAdded[i].quorumCount;

            for (uint256 j = 0; j < operatorStateInfo.operatorsAdded[i].quorumForStakes.length; j++) {
                operatorAndQuorumToStakes[operatorStateInfo.operatorsAdded[i].operatorId][operatorStateInfo.operatorsAdded[i].quorumForStakes[j]] = operatorStateInfo.operatorsAdded[i].quorumStakes[j];
            }
        }

        for (uint256 i = 0; i < operatorStateInfo.operatorsStakeUpdate.length; i++) {

            for (uint256 j = 0; j < operatorStateInfo.operatorsStakeUpdate[i].quorumForStakes.length; j++) {
                operatorAndQuorumToStakes[operatorStateInfo.operatorsStakeUpdate[i].operatorId][operatorStateInfo.operatorsStakeUpdate[i].quorumForStakes[j]] = operatorStateInfo.operatorsStakeUpdate[i].quorumStakes[j];
            }
        }

        for (uint256 i = 0; i < operatorStateInfo.operatorsQuorumCountUpdate.length; i++) {
            operatorIdQuorumCount[operatorStateInfo.operatorsQuorumCountUpdate[i].operatorId] = operatorStateInfo.operatorsQuorumCountUpdate[i].quorumCount;
        }

        latestCompletedOpTaskNumber = task.taskNum;
        latestCompletedOpTaskCreatedBlock = task.taskCreatedBlock;
        lastOpUpdateBlockTimestamp = block.timestamp;

        quorumNumbers = task.quorumNumbers;
        quorumThresholdPercentage = task.quorumThresholdPercentage;

        emit EigenOpUpdateProcessed(task.taskNum, task.taskCreatedBlock);
        
    }

    function process_eigen_rd_update(IFinalizerTaskManager.RdTask calldata task, IFinalizerTaskManager.RdTaskResponse calldata taskResponse, IBLSSignatureChecker.NonSignerStakesAndSignature calldata nonSignerStakesAndSignature) public onlyUpdater {

        require(latestCompletedRdTaskNumber == 0 || latestCompletedRdTaskNumber < task.taskNum, "Stale RdTask");
        require(latestCompletedOpTaskCreatedBlock != 0, "Op state uninit");
        require(latestCompletedOpTaskCreatedBlock == task.lastCompletedOpTaskCreatedBlock, "reference block hash mismatch");
        require(taskResponse.referenceTaskHash == keccak256(abi.encode(task)), "referenceTaskHash hash mismatch");

        require(latestCompletedOpTaskCreatedBlock + 14400 > task.taskCreatedBlock, "stale state 0");
        require(lastOpUpdateBlockTimestamp + 3 days > block.timestamp, "stale state 1");

        
        // if the this is the first task then don't check sigs
        IBLSSignatureChecker.QuorumStakeTotals memory quorumStakeTotals = checkSignatures(keccak256(abi.encode(taskResponse)), nonSignerStakesAndSignature);

        uint32 QuorumThresholdPercentage = quorumThresholdPercentage;
        // check that signatories own at least a threshold percentage of each quourm
        for (uint256 i = 0; i < quorumNumbers.length; i++) {
            // we don't check that the quorumThresholdPercentages are not >100 because a greater value would trivially fail the check, implying
            // signed stake > total stake
            if (
                quorumStakeTotals.signedStakeForQuorum[i] * _THRESHOLD_DENOMINATOR
                    < quorumStakeTotals.totalStakeForQuorum[i] * uint8(QuorumThresholdPercentage)
            ) {
                // "Signatories do not own at least threshold percentage of a quorum"
                revert("Failed to meet quorum");
            }
        }

        IRolldown.Range memory range;
        range.start = taskResponse.rangeStart;
        range.end = taskResponse.rangeEnd;
        rolldown.update_l1_from_l2(taskResponse.rdUpdate, range);

        emit EigenRdUpdateProcessed(task.taskNum, task.taskCreatedBlock);
        
    }

    function checkSignatures(
        bytes32 msgHash,
        IBLSSignatureChecker.NonSignerStakesAndSignature memory params
    )
        public 
        view
        returns (
            IBLSSignatureChecker.QuorumStakeTotals memory
        )
    {
        // This method needs to calculate the aggregate pubkey for all signing operators across
        // all signing quorums. To do that, we can query the aggregate pubkey for each quorum
        // and subtract out the pubkey for each nonsigning operator registered to that quorum.
        //
        // In practice, we do this in reverse - calculating an aggregate pubkey for all nonsigners,
        // negating that pubkey, then adding the aggregate pubkey for each quorum.
        BN254.G1Point memory apk = BN254.G1Point(0, 0);

        uint256 quorumNumbersLength = quorumNumbers.length;

        // For each quorum, we're also going to query the total stake for all registered operators
        // at the referenceBlockNumber, and derive the stake held by signers by subtracting out
        // stakes held by nonsigners.
        IBLSSignatureChecker.QuorumStakeTotals memory stakeTotals;
        stakeTotals.totalStakeForQuorum = new uint96[](quorumNumbersLength);
        stakeTotals.signedStakeForQuorum = new uint96[](quorumNumbersLength);

        bytes32[] memory nonSignersPubkeyHashes = new bytes32[](params.nonSignerPubkeys.length);

        uint8 quorumNumber;

        {

            for (uint256 j = 0; j < params.nonSignerPubkeys.length; j++) {
                // The nonsigner's pubkey hash doubles as their operatorId
                // The check below validates that these operatorIds are sorted (and therefore
                // free of duplicates)
                nonSignersPubkeyHashes[j] = params.nonSignerPubkeys[j].hashG1Point();
                if (j != 0) {
                    require(
                        uint256(nonSignersPubkeyHashes[j]) > uint256(nonSignersPubkeyHashes[j - 1]),
                        "BLSSignatureChecker.checkSignatures: nonSignerPubkeys not sorted"
                    );
                }

                apk = apk.plus(
                    params.nonSignerPubkeys[j]
                        .scalar_mul_tiny(
                            operatorIdQuorumCount[nonSignersPubkeyHashes[j]]
                        )
                );
            }
        }

        // Negate the sum of the nonsigner aggregate pubkeys - from here, we'll add the
        // total aggregate pubkey from each quorum. Because the nonsigners' pubkeys are
        // in these quorums, this initial negation ensures they're cancelled out
        apk = apk.negate();

        /**
         * For each quorum (at referenceBlockNumber):
         * - add the apk for all registered operators
         * - query the total stake for each quorum
         * - subtract the stake for each nonsigner to calculate the stake belonging to signers
         */
        {
            
            for (uint256 i = 0; i < quorumNumbersLength; i++) {

                quorumNumber = uint8(quorumNumbers[i]);
                
                apk = apk.plus(qourumApk[quorumNumber]);

                // Get the total and starting signed stake for the quorum at referenceBlockNumber
                stakeTotals.totalStakeForQuorum[i] = 
                    quorumToStakes[quorumNumber];
                stakeTotals.signedStakeForQuorum[i] = stakeTotals.totalStakeForQuorum[i];
                
                // loop through all nonSigners, checking that they are a part of the quorum via their quorumBitmap
                // if so, load their stake at referenceBlockNumber and subtract it from running stake signed
                for (uint256 j = 0; j < params.nonSignerPubkeys.length; j++) {
                        stakeTotals.signedStakeForQuorum[i] -=
                            operatorAndQuorumToStakes[nonSignersPubkeyHashes[j]][quorumNumber];
                }
            }
        }
        {
            // verify the signature
            (bool pairingSuccessful, bool signatureIsValid) = trySignatureAndApkVerification(
                msgHash, 
                apk, 
                params.apkG2, 
                params.sigma
            );
            require(pairingSuccessful, "BLSSignatureChecker.checkSignatures: pairing precompile call failed");
            require(signatureIsValid, "BLSSignatureChecker.checkSignatures: signature is invalid");
        }

        // return the total stakes that signed for each quorum, and a hash of the information required to prove the exact signers and stake
        return (stakeTotals);
    }

    /**
     * trySignatureAndApkVerification verifies a BLS aggregate signature and the veracity of a calculated G1 Public key
     * @param msgHash is the hash being signed
     * @param apk is the claimed G1 public key
     * @param apkG2 is provided G2 public key
     * @param sigma is the G1 point signature
     * @return pairingSuccessful is true if the pairing precompile call was successful
     * @return siganatureIsValid is true if the signature is valid
     */
    function trySignatureAndApkVerification(
        bytes32 msgHash,
        BN254.G1Point memory apk,
        BN254.G2Point memory apkG2,
        BN254.G1Point memory sigma
    ) public view returns(bool pairingSuccessful, bool siganatureIsValid) {
        // gamma = keccak256(abi.encodePacked(msgHash, apk, apkG2, sigma))
        uint256 gamma = uint256(keccak256(abi.encodePacked(msgHash, apk.X, apk.Y, apkG2.X[0], apkG2.X[1], apkG2.Y[0], apkG2.Y[1], sigma.X, sigma.Y))) % BN254.FR_MODULUS;
        // verify the signature
        (pairingSuccessful, siganatureIsValid) = BN254.safePairing(
                sigma.plus(apk.scalar_mul(gamma)),
                BN254.negGeneratorG2(),
                BN254.hashToG1(msgHash).plus(BN254.generatorG1().scalar_mul(gamma)),
                apkG2,
                PAIRING_EQUALITY_CHECK_GAS
            );
    }
}
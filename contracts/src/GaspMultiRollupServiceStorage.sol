// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.9;

import "./IGaspMultiRollupService.sol";
import "@eigenlayer-middleware/src/libraries/BN254.sol";

abstract contract GaspMultiRollupServiceStorage is IGaspMultiRollupService {

    address public updater;

    bool public stalled;
    bool public allowNonRootInit;
    uint256 public lastOpUpdateBlockTimestamp;

    bytes32 public latestPendingStateHash;

    uint32 public latestCompletedOpTaskNumber;
    uint32 public latestCompletedOpTaskCreatedBlock;

    bytes public quorumNumbers;
    uint32 public quorumThresholdPercentage;
    
    mapping(uint8 => uint96) public quorumToStakes;
    mapping(bytes32 => mapping(uint8 => uint96)) public operatorAndQuorumToStakes;

    mapping(uint8 => BN254.G1Point) public qourumApk;
    mapping(bytes32 => uint8) public operatorIdQuorumCount;

    // mapping(bytes32 => address) public OperatorIdToOperator;

    /**
     * @dev This empty reserved space is put in place to allow future versions to add new
     * variables without shifting down storage in the inheritance chain.
     * See https://docs.openzeppelin.com/contracts/4.x/upgradeable#storage_gaps
     */
    uint256[50] private __gap;
}

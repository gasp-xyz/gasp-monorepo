// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.9;

import "./IGaspMultiRollupService.sol";
import "./IRolldown.sol";
import "@eigenlayer-middleware/src/libraries/BN254.sol";

abstract contract GaspMultiRollupServiceStorage is IGaspMultiRollupService {
/*
    // Counter for mapping key
    uint256 public counter;
    // Counter for last processed request to ensure not reading and processing already processed
    uint256 public lastProcessedUpdate_origin_l1;
    // Counter for last processed updates comming from l2 to ensure not reading and processing already processed
    uint256 public lastProcessedUpdate_origin_l2;

    mapping(uint256 => WithdrawalResolution) public withdrawalResolutions;
    mapping(uint256 => CancelResolution) public cancelResolutions;
    mapping(uint256 => Deposit) internal deposits;
    mapping(uint256 => L2UpdatesToRemove) internal l2UpdatesToRemove;
    mapping(address => uint) public pendingEthWithdrawals;   

*/

    address public updater;
    IRolldown public rolldown;

    bool public stalled;

    bytes32 public latestPendingStateHash;

    uint32 public latestCompletedTaskNumber;
    uint32 public latestCompletedTaskCreatedBlock;

    bytes public quorumNumbers;
    uint32 public quorumThresholdPercentage;
    
    mapping(uint8 => uint96) public quorumToStakes;
    mapping(bytes32 => mapping(uint8 => uint96)) public operatorAndQuorumToStakes;

    mapping(uint8 => BN254.G1Point) public qourumApk;
    // mapping(uint8 => mapping(bytes32 => bool)) public QuorumOperators;
    // mapping(bytes32 => BN254.G1Point) public OperatorIdPubKey;
    mapping(bytes32 => uint8) public operatorIdQuorumCount;

    // mapping(bytes32 => address) public OperatorIdToOperator;

    /**
     * @dev This empty reserved space is put in place to allow future versions to add new
     * variables without shifting down storage in the inheritance chain.
     * See https://docs.openzeppelin.com/contracts/4.x/upgradeable#storage_gaps
     */
    uint256[50] private __gap;
}

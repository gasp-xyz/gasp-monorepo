// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {BN254} from "@eigenlayer-middleware/src/libraries/BN254.sol";
import {IGaspMultiRollupService} from "./interfaces/IGaspMultiRollupService.sol";
import {IRolldown} from "./interfaces/IRolldown.sol";

abstract contract GaspMultiRollupServiceStorage is IGaspMultiRollupService {
    IRolldown public rolldown;
    uint32 public chainRdBatchNonce;
    IRolldown.ChainId public chainId;
    address public updater;
    bool public stalled;
    bool public allowNonRootInit;
    uint256 public lastOpUpdateBlockTimestamp;
    uint32 public latestCompletedRdTaskNumber;
    uint32 public latestCompletedOpTaskNumber;
    uint32 public latestCompletedOpTaskCreatedBlock;
    bytes public quorumNumbers;
    uint32 public quorumThresholdPercentage;
    mapping(uint8 => uint96) public quorumToStakes;
    mapping(bytes32 => mapping(uint8 => uint96)) public operatorAndQuorumToStakes;
    mapping(uint8 => BN254.G1Point) public qourumApk;
    mapping(bytes32 => uint8) public operatorIdQuorumCount;
    // Placing it here to be able to upgrade the testnetv3
    uint32 public latestCompletedRdTaskCreatedBlock;

    /**
     * @dev This empty reserved space is put in place to allow future versions to add new
     * variables without shifting down storage in the inheritance chain.
     * See https://docs.openzeppelin.com/contracts/4.x/upgradeable#storage_gaps
     */
    uint256[49] private __gap;
}

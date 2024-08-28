// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.9;

import {IRegistryCoordinator} from "@eigenlayer-middleware/src/interfaces/IRegistryCoordinator.sol";
import {IBLSApkRegistry} from "@eigenlayer-middleware/src/interfaces/IBLSApkRegistry.sol";
import {IStakeRegistry} from "@eigenlayer-middleware/src/interfaces/IStakeRegistry.sol";
import {IIndexRegistry} from "@eigenlayer-middleware/src/interfaces/IIndexRegistry.sol";
import {BitmapUtils} from "@eigenlayer-middleware/src/libraries/BitmapUtils.sol";
import {OperatorStateRetriever} from "@eigenlayer-middleware/src/OperatorStateRetriever.sol";

/**
 * @title OperatorStateRetriever with view functions that allow to retrieve the state of an AVSs registry system.
 * @author Layr Labs Inc.
 */
contract OperatorStateRetrieverExtended is OperatorStateRetriever {
    using BitmapUtils for *;

    function operatorStakeForQuorum(IRegistryCoordinator registryCoordinator, uint8 quorumNumber, address operator) public virtual view returns (uint96, bool) {
        IStakeRegistry stakeRegistry = registryCoordinator.stakeRegistry();
        uint96 weight = stakeRegistry.weightOfOperatorForQuorum(quorumNumber, operator);
        // Return the weight, and `true` if the operator meets the quorum's minimum stake
        bool hasMinimumStake = weight >= stakeRegistry.minimumStakeForQuorum(quorumNumber);
        return (weight, hasMinimumStake);
    }

    function operatorStakeForQuorumClipped(IRegistryCoordinator registryCoordinator, uint8 quorumNumber, address operator) public virtual view returns (uint96) {
        (uint96 weight, bool hasMinimumStake) = operatorStakeForQuorum(registryCoordinator, quorumNumber, operator);
        if (!hasMinimumStake){
            weight = 0;
        }
        return weight;
    }

    // function operatorsStakesForQuorum(IRegistryCoordinator registryCoordinator, bytes calldata quorumNumbers, address[] calldata operators) public virtual view returns (uint96[][] memory){

    //     uint96[][] memory stakes = new uint96[][](quorumNumbers.length);

    //     for (uint256 i = 0; i < quorumNumbers.length; i++) {
    //         uint8 quorumNumber = uint8(quorumNumbers[i]);
    //         stakes[i] = new uint96[](operators.length);
    //         for (uint256 j = 0; j < operators.length; j++) {
    //             stakes[i][j] = operatorStakeForQuorumClipped(registryCoordinator, quorumNumber, operators[j]);
    //         }
    //     }
    // }

    function getOperatorsStakesForQuorum(IRegistryCoordinator registryCoordinator, bytes calldata quorumNumbers, address[] calldata operatorsAddr) public virtual view returns (Operator[][] memory){
        IBLSApkRegistry blsApkRegistry = registryCoordinator.blsApkRegistry();
        IStakeRegistry stakeRegistry = registryCoordinator.stakeRegistry();

        Operator[][] memory operators = new Operator[][](quorumNumbers.length);
        for (uint256 i = 0; i < quorumNumbers.length; i++) {
            uint8 quorumNumber = uint8(quorumNumbers[i]);
            bytes memory quorumBytes = new bytes(1);
            quorumBytes[0] = quorumNumbers[i];
            operators[i] = new Operator[](operatorsAddr.length);
            for (uint256 j = 0; j < operatorsAddr.length; j++) {
                bytes32 operatorId = blsApkRegistry.getOperatorId(operatorsAddr[j]);
                uint96 weight = operatorStakeForQuorumClipped(registryCoordinator, quorumNumber, operatorsAddr[j]);
                if (operatorId == 0){
                    weight = 0;
                }
                if (!BitmapUtils.orderedBytesArrayToBitmap(quorumBytes).isSubsetOf(registryCoordinator.getCurrentQuorumBitmap(operatorId))){
                    weight = 0;
                }
                operators[i][j] = Operator({
                    operator: address(operatorsAddr[j]),
                    operatorId: operatorId,
                    stake: weight
                });
            }
        }
            
        return operators;
    }

    /// @notice Returns the current quorum bitmap for the given `operatorId` or 0 if the operator is not registered for any quorum
    function getOperatorIdQuorums(IRegistryCoordinator registryCoordinator, bytes32 operatorId) external view returns (bytes memory) {
        uint192 currentBitmap = registryCoordinator.getCurrentQuorumBitmap(operatorId);
        bytes memory quorums = BitmapUtils.bitmapToBytesArray(currentBitmap);
        return quorums;
    }

    /// @notice Returns the current quorum bitmap for the given `operatorId` or 0 if the operator is not registered for any quorum
    function getOperatorsFromIds(IRegistryCoordinator registryCoordinator, bytes32[] calldata operatorIds) external view returns (address[] memory) {
        address[] memory operators = new address[](operatorIds.length);
        for (uint256 i = 0; i < operatorIds.length; i++) {
            operators[i] = registryCoordinator.getOperatorFromId(operatorIds[i]);
        }
        return operators;
    }
}

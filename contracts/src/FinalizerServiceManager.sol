// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@eigenlayer/contracts/libraries/BytesLib.sol";
import "./IFinalizerTaskManager.sol";
import "@eigenlayer-middleware/src/ServiceManagerBase.sol";

/**
 * @title Primary entrypoint for procuring services from Finalizer.
 * @author Layr Labs, Inc.
 */
contract FinalizerServiceManager is ServiceManagerBase {
    using BytesLib for bytes;

    IFinalizerTaskManager public immutable taskManager;
    /// @notice The minimum threshold for reccurent registration in blocks
    uint64 public immutable recurrentRegistrationBlocksLimit;

    /// @notice when applied to a function, ensures that the function is only callable by the `taskManager`.
    modifier onlyTaskManager() {
        require(msg.sender == address(taskManager), "onlyTaskManager: not from finalizer task manager");
        _;
    }

    constructor(
        IAVSDirectory _avsDirectory,
        IRegistryCoordinator _registryCoordinator,
        IStakeRegistry _stakeRegistry,
        IFinalizerTaskManager _taskManager,
        uint64 _recurrentRegistrationBlocksLimit
    ) ServiceManagerBase(_avsDirectory, _registryCoordinator, _stakeRegistry) {
        taskManager = _taskManager;
        recurrentRegistrationBlocksLimit = _recurrentRegistrationBlocksLimit;
    }

    function initialize(address initialOwner) public virtual initializer {
        __ServiceManagerBase_init(initialOwner);
    }

    function registerOperatorToAVS(
        address operator,
        ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature
    ) public override onlyRegistryCoordinator {
        bytes32 operatorId = _registryCoordinator.getOperatorId(operator);
        // for each quorum check last stake update block number, if the operator was deregistered, there will be an entry,
        // operator can register again only after some 'recurrentRegistrationBlocksLimit' amount of blocks passes
        for (uint8 i = 0; i < _registryCoordinator.quorumCount(); i++) {
            IStakeRegistry.StakeUpdate memory latestStakeUpdate = _stakeRegistry.getLatestStakeUpdate(operatorId, i);
            // when stake is zero & last update non zero, the OP was deregistered
            if (latestStakeUpdate.stake == 0 && latestStakeUpdate.updateBlockNumber != 0) {
                require(
                    block.number - latestStakeUpdate.updateBlockNumber > recurrentRegistrationBlocksLimit,
                    "ServiceManager.registerOperatorToAVS: minimum blocks elapsed limit for recurrent registration not met"
                );
            }
        }

        _avsDirectory.registerOperatorToAVS(operator, operatorSignature);
    }
}

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

    IFinalizerTaskManager
        public immutable taskManager;

    /// @notice when applied to a function, ensures that the function is only callable by the `taskManager`.
    modifier onlyTaskManager() {
        require(
            msg.sender == address(taskManager),
            "onlyTaskManager: not from finalizer task manager"
        );
        _;
    }

    constructor(
        IAVSDirectory _avsDirectory,
        IRegistryCoordinator _registryCoordinator,
        IStakeRegistry _stakeRegistry,
        IFinalizerTaskManager _taskManager
    ) ServiceManagerBase(_avsDirectory, _registryCoordinator, _stakeRegistry) {
        taskManager = _taskManager;
    }

    function initialize(address initialOwner) public virtual initializer {
        __ServiceManagerBase_init(initialOwner);
    }
}

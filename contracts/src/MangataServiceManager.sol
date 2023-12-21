// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@eigenlayer/contracts/libraries/BytesLib.sol";
import "./IMangataTaskManager.sol";
import "@eigenlayer-middleware/src/ServiceManagerBase.sol";

/**
 * @title Primary entrypoint for procuring services from Mangata.
 * @author Layr Labs, Inc.
 */
contract MangataServiceManager is ServiceManagerBase {
    using BytesLib for bytes;

    IMangataTaskManager
        public immutable taskManager;

    /// @notice when applied to a function, ensures that the function is only callable by the `taskManager`.
    modifier onlyTaskManager() {
        require(
            msg.sender == address(taskManager),
            "onlyTaskManager: not from mangata task manager"
        );
        _;
    }

    constructor(
        IBLSRegistryCoordinatorWithIndices _registryCoordinator,
        ISlasher _slasher,
        IMangataTaskManager _taskManager
    ) ServiceManagerBase(_registryCoordinator, _slasher) {
        taskManager = _taskManager;
    }

    /// @notice Called in the event of challenge resolution, in order to forward a call to the Slasher, which 'freezes' the `operator`.
    /// @dev The Slasher contract is under active development and its interface expected to change.
    ///      We recommend writing slashing logic without integrating with the Slasher at this point in time.
    function freezeOperator(
        address operatorAddr
    ) external override onlyTaskManager {
        // slasher.freezeOperator(operatorAddr);
    }
}

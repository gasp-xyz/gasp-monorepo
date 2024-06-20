// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@eigenlayer/contracts/libraries/BytesLib.sol";
import "./IFinalizerTaskManager.sol";
import "./IFinalizerServiceManager.sol";
import "@eigenlayer-middleware/src/ServiceManagerBase.sol";

/**
 * @title Primary entrypoint for procuring services from Finalizer.
 * @author Layr Labs, Inc.
 */
contract FinalizerServiceManager is ServiceManagerBase, IFinalizerServiceManager {
    using BytesLib for bytes;

    IFinalizerTaskManager public immutable taskManager;
    /// @notice The minimum threshold for reccurent registration in blocks
    uint64 public immutable recurrentRegistrationBlocksLimit;

    /// @notice the address of the entity allowed to eject operators from the AVS
    address public ejector;

    /// @notice when applied to a function, ensures that the function is only callable by the `taskManager`.
    modifier onlyTaskManager() {
        require(msg.sender == address(taskManager), "onlyTaskManager: not from finalizer task manager");
        _;
    }

    modifier onlyEjector() {
        require(msg.sender == ejector, "RegistryCoordinator.onlyEjector: caller is not the ejector");
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

    function initialize(address initialOwner, address _ejector) public virtual initializer {
        __ServiceManagerBase_init(initialOwner);
        ejector = _ejector;
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

    function ejectOperators(address[] calldata operators, bytes[] calldata quorumNumbers) external onlyEjector() {
        require(
            operators.length == quorumNumbers.length, "RegistryCoordinator.ejectOperators: args length does not match"
        );
        for (uint256 i = 0; i < operators.length; i++) {
            _registryCoordinator.ejectOperator({operator: operators[i], quorumNumbers: quorumNumbers[i]});
        }
    }

    /**
     * @notice Sets the ejector, which can force-deregister operators from quorums
     * @param _ejector the new ejector
     * @dev only callable by the owner
     */
    function setEjector(address _ejector) external onlyOwner {
        _setEjector(_ejector);
    }

    function _setEjector(address newEjector) internal {
        emit EjectorUpdated(ejector, newEjector);
        ejector = newEjector;
    }
}

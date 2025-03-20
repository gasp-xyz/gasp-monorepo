// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

/**
 * @title Primary entrypoint for procuring services from Finalizer.
 */
interface IFinalizerServiceManager {
    event EjectorUpdated(address prevEjector, address newEjector);

    function ejectOperators(address[] calldata operators, bytes[] calldata quorumNumbers) external;
}

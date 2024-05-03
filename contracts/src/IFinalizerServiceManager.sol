// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

/**
 * @title Primary entrypoint for procuring services from Finalizer.
 */
interface IFinalizerServiceManager {
    event EjectorUpdated(address prevEjector, address newEjector);

    function ejectOperators(address[] calldata operators, bytes[] calldata quorumNumbers) external;
}

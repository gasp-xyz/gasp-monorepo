// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {console} from "forge-std/console.sol";
import {stdStorage, StdStorage, Test} from "forge-std/Test.sol";
import {IFinalizerTaskManager} from "./../src/interfaces/IFinalizerTaskManager.sol";
import {IGaspMultiRollupServicePrimitives} from "./../src/interfaces/IGaspMultiRollupServicePrimitives.sol";

contract GaspMultiRollupServiceTest is Test, IGaspMultiRollupServicePrimitives {
    using stdStorage for StdStorage;

    // function createNewTask(
    //     uint256 blockNumber,
    //     uint32 quorumThresholdPercentage,
    //     bytes calldata quorumNumbers
    // ) external{return;}

    // /// @notice Returns the current 'taskNumber' for the middleware
    // function taskNumber() external view returns (uint32){return 0;}

    // /// @notice Returns the TASK_RESPONSE_WINDOW_BLOCK
    // function getTaskResponseWindowBlock() external view returns (uint32){
    //     return 0;
    // }

    function testGasp() public pure {
        bytes memory qbytes = new bytes(1);
        qbytes[0] = 0x00;

        bytes memory lqbytes = new bytes(0);

        IFinalizerTaskManager.OpTask memory task = IFinalizerTaskManager.OpTask({
            taskNum: 1,
            taskCreatedBlock: 30,
            lastCompletedOpTaskNum: 0,
            lastCompletedOpTaskCreatedBlock: 0,
            quorumNumbers: qbytes,
            quorumThresholdPercentage: 66,
            lastCompletedOpTaskQuorumNumbers: lqbytes,
            lastCompletedOpTaskQuorumThresholdPercentage: 0
        });

        bytes memory encodedTask = abi.encode(task);
        console.logBytes(encodedTask);
        bytes32 hashTask = keccak256(abi.encode(task));
        console.logBytes32(hashTask);
    }
}

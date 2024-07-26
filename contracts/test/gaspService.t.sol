pragma solidity ^0.8.9;
// import {GaspMultiRollupService} from "../src/GaspMultiRollupService.sol";
// import {stdStorage, StdStorage, Test} from "forge-std/Test.sol";
// import "forge-std/console.sol";
// import {Utilities, MyERC20} from "./utils/Utilities.sol";
// import {IGaspMultiRollupServicePrimitives} from "../src/IGaspMultiRollupServicePrimitives.sol";
// import {IFinalizerTaskManager} from "../src/IFinalizerTaskManager.sol";

// contract GaspMultiRollupServiceTest is Test, IGaspMultiRollupServicePrimitives {
//     using stdStorage for StdStorage;

//     // function createNewTask(
//     //     uint256 blockNumber,
//     //     uint32 quorumThresholdPercentage,
//     //     bytes calldata quorumNumbers
//     // ) external{return;}

//     // /// @notice Returns the current 'taskNumber' for the middleware
//     // function taskNumber() external view returns (uint32){return 0;}

//     // /// @notice Returns the TASK_RESPONSE_WINDOW_BLOCK
//     // function getTaskResponseWindowBlock() external view returns (uint32){
//     //     return 0;
//     // }

//     function testGasp() public {

//         bytes memory qbytes = new bytes(1);
//         qbytes[0] = 0x00;

//         bytes memory lqbytes = new bytes(0);

//         IFinalizerTaskManager.Task memory task =
//             IFinalizerTaskManager.Task ({
//         taskNum :1,
//         blockNumber:20,
//         taskCreatedBlock:30,
//         lastCompletedTaskCreatedBlock:0 ,
//         quorumNumbers: qbytes,
//         quorumThresholdPercentage: 66,
//         lastCompletedTaskQuorumNumbers: lqbytes,
//         lastCompletedTaskQuorumThresholdPercentage:0
//     });

//         bytes memory encodedTask =  abi.encode(task);
//         console.logBytes(encodedTask);
//         bytes32 hashTask =  keccak256(abi.encode(task));
//         console.logBytes32(hashTask);
//     }

// }
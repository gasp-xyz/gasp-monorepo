// SPDX-License-Identifier: BUSL-1.1
//cd contracts ;  pkill anvil ; cdlear ; cd ../ pkille start-anclearnd-cd ../  ;makecontracts ;   forge test --cdtch-contract Ejforge testRulesTest -vvv --fork-url=http://localhost:8545
pragma solidity =0.8.12;

import { FinalizerServiceManager } from "../../src/FinalizerServiceManager.sol";
import {BitmapUtils} from "@eigenlayer-middleware/src/libraries/BitmapUtils.sol";
import { TestUtils } from "./TestUtils.sol";
import "forge-std/StdJson.sol";
import "forge-std/Test.sol";

contract IntegrationEjectorAdminRuleTest is Test {

   address deployer = address(0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266);

   function testEjectorCanBeResetByDeployer() public {

         address tu = address(deployer);

         FinalizerServiceManager fsm  = FinalizerServiceManager(stdJson.readAddress(new TestUtils().getRollDownConfigFileEth(), ".addresses.serviceManager"));
         address ejectorAddr = fsm.ejector();
         emit log_address(ejectorAddr);

         vm.startBroadcast(tu);
         fsm.setEjector(tu);
         vm.stopBroadcast();
         
         address ejectorAddr2 = fsm.ejector();
         emit log_address(ejectorAddr2);
         assert(ejectorAddr != ejectorAddr2);

   }
   function testEjectorCanNotBeResetByaNonDeployer() public {

         address tu = address(101);

         FinalizerServiceManager fsm  = FinalizerServiceManager(stdJson.readAddress(new TestUtils().getRollDownConfigFileEth(), ".addresses.serviceManager"));
         address ejectorAddr = fsm.ejector();
         emit log_address(ejectorAddr);

         vm.startBroadcast(tu);
         vm.expectRevert("Ownable: caller is not the owner");
         fsm.setEjector(tu);
         vm.stopBroadcast();
         
         address ejectorAddr2 = fsm.ejector();
         emit log_address(ejectorAddr2);
         assert(ejectorAddr == ejectorAddr2);

   }
   function testEjectorCanEject() public {

         FinalizerServiceManager fsm  = FinalizerServiceManager(stdJson.readAddress(new TestUtils().getRollDownConfigFileEth(), ".addresses.serviceManager"));
         address ejectorAddr = fsm.ejector();

         address[] memory operators = new address[](2);
            operators[0] = address(0x1);
            operators[1] = address(0x2);
   
         bytes[] memory quorumNumbers = new bytes[](2) ;
         quorumNumbers[0] = BitmapUtils.bitmapToBytesArray(0);
         quorumNumbers[1] = BitmapUtils.bitmapToBytesArray(1);
         console.log("quorumNumbers"); 
         
         vm.startBroadcast(ejectorAddr);
         vm.expectRevert("RegistryCoordinator._deregisterOperator: operator is not registered");
         fsm.ejectOperators(operators, quorumNumbers);
         vm.stopBroadcast();

   }
   //https://mangatafinance.atlassian.net/browse/MGX-1315
   function testOwnerCanEject() public {

         FinalizerServiceManager fsm  = FinalizerServiceManager(stdJson.readAddress(new TestUtils().getRollDownConfigFileEth(), ".addresses.serviceManager"));
         address ejectorAddr = fsm.ejector();

         address[] memory operators = new address[](2);
            operators[0] = address(0x1);
            operators[1] = address(0x2);
   
         bytes[] memory quorumNumbers = new bytes[](2) ;
         quorumNumbers[0] = BitmapUtils.bitmapToBytesArray(0);
         quorumNumbers[1] = BitmapUtils.bitmapToBytesArray(1);
         console.log("quorumNumbers"); 
         
         vm.startBroadcast(deployer);
         vm.expectRevert("RegistryCoordinator._deregisterOperator: operator is not registered");
         fsm.ejectOperators(operators, quorumNumbers);
         vm.stopBroadcast();

   }
}
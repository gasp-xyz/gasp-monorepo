// SPDX-License-Identifier: BUSL-1.1
//cd contracts ;  pkill anvil ; cdlear ; cd ../ pkille start-anclearnd-cd ../  ;makecontracts ;   forge test --cdtch-contract Ejforge testRulesTest -vvv --fork-url=http://localhost:8545
pragma solidity =0.8.12;

import { FinalizerServiceManager } from "../../src/FinalizerServiceManager.sol";
import { TestUtils } from "./TestUtils.sol";
import "forge-std/StdJson.sol";
import "forge-std/Test.sol";

contract IntegrationEjectorAdminRuleTest is Test {

   address deployer = address(0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266);

   function testEjectorCanBeResetByOwner() public {

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
}
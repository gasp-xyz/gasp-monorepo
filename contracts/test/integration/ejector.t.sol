// SPDX-License-Identifier: BUSL-1.1
//cd contracts ;  pkill anvil ; cdlear ; cd ../ pkille start-anclearnd-cd ../  ;makecontracts ;   forge test --cdtch-contract Ejforge testRulesTest -vvv --fork-url=http://localhost:8545
pragma solidity =0.8.12;

import { FinalizerServiceManager } from "../../src/FinalizerServiceManager.sol";

import "forge-std/Test.sol";

contract IntegrationEjectorAdminRuleTest is Test {

   address deployer = address(0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266);

   function testEjectorCanBeResetByOwner() public {

        FinalizerServiceManager fsm ;
        fsm = FinalizerServiceManager(address(0xa82fF9aFd8f496c3d6ac40E2a0F282E47488CFc9));
        address ejectorAddr = fsm.ejector();
        emit log_address(ejectorAddr);
        address tu = address(deployer);
        deal(tu, 100 ether);
        vm.startBroadcast(tu);
        fsm.setEjector(tu);
        vm.stopBroadcast();
        address ejectorAddr2 = fsm.ejector();
        emit log_address(ejectorAddr2);


   }
}
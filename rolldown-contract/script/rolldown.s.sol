// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "../src/rolldown.sol";
import {RollDown} from "../src/rolldown.sol";
import {TestToken} from "../src/token.sol";

contract MyScript is Script {

    function run() external {
        // anvil account
        uint256 anvilPriv = 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80;
        address anvilAddr = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266;

        vm.startBroadcast(anvilPriv);
        RollDown rd = new RollDown();
        TestToken token = new TestToken();

        // mint tokens to test account
        token.mint(anvilAddr, 1000000);

        // approve rolldown contract to use token
        token.approve(address(rd), 1000000);

  
        rd.deposit(address(token), 2000);
        rd.deposit(address(token), 2000);


        vm.stopBroadcast();
    }
}

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "../src/rolldown.sol";
import {RollDown} from "../src/rolldown.sol";
import {TestToken} from "../src/token.sol";

contract MyScript is Script {

    function run() external {
        // anvil account

        vm.startBroadcast();
        RollDown rd = new RollDown();
        TestToken token = new TestToken();

        // mint tokens to test account
        token.mint(msg.sender, 1000000);

        // approve rolldown contract to use token
        token.approve(address(rd), 1000000);
        rd.deposit_erc20(address(token), 2000);
        rd.deposit_erc20(address(token), 2000);

        _record_addresses(address(token), address(rd));

        vm.stopBroadcast();
    }


    function _record_addresses(
        address tokenContractAddress,
        address rolldownContractAddress
    ) internal {
        string memory parent_object = "parent object";
        vm.serializeAddress(parent_object, "tokenAddress", tokenContractAddress);
        string memory output = vm.serializeAddress(parent_object, "rolldownContractAddress", rolldownContractAddress);
        console.log(output);
        vm.writeJson(output, "./out/addresses.json");
    }
}

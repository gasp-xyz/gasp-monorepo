// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {Script} from "forge-std/Script.sol";
import {Test} from "forge-std/Test.sol";

contract TestUtils is Script, Test {
    function getAvsConfigFile() public view returns (string memory) {
        string memory configData = vm.readFile("script/output/31337/avs_deployment_output.json");
        return configData;
    }

    function getRollDownConfigFileEth() public view returns (string memory) {
        string memory configData = vm.readFile("script/output/31337/rolldown_output.json");
        return configData;
    }

    function getRollDownConfigFileArb() public view returns (string memory) {
        string memory configData = vm.readFile("script/output/31337/arbitrum_rolldown_output.json");
        return configData;
    }

    function getErc20ConfigFile() public view returns (string memory) {
        string memory configData = vm.readFile("script/output/31337/strategy_output.json");
        return configData;
    }
}

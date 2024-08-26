pragma solidity =0.8.12;

import "forge-std/Test.sol";
import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "forge-std/console.sol";

contract TestUtils is Script, Test {

    function getAvsConfigFile()  view public returns (string memory){
        string memory configData = vm.readFile("script/output/31337/avs_deployment_output.json");
        return configData;
    }
    function getRollDownConfigFileEth()  view public returns (string memory){
        string memory configData = vm.readFile("script/output/31337/ethereum_rolldown_output.json");
        return configData;
    }
    function getRollDownConfigFileArb()  view public returns (string memory){
        string memory configData = vm.readFile("script/output/31337/arbitrum_rolldown_output.json");
        return configData;
    }
    function getErc20ConfigFile() view public returns (string memory){
        string memory configData = vm.readFile("script/output/31337/strategy_output.json");
        return configData;
    }
}
// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

// Link Github PR, Jira Ticket and Notion Tracker
// Jira GASP-2120
// S1_U1MX - 1st Script - Contract Upgrade 1 / No Migration
// Contract to upgrade: FinalizerTaskManager

import {ProxyAdmin, TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {console} from "forge-std/console.sol";
import {Script} from "forge-std/Script.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {Test} from "forge-std/Test.sol";
import {FinalizerTaskManager} from "../../../../src/FinalizerTaskManager.sol";
import {Utils} from "../../../utils/Utils.sol";

// To upgrade and verify FinalizerTaskManager
// forge script script/S1_U1MX_FinalizerTaskManager.s.sol:Deployer --rpc-url $RPC_URL  --private-key $PRIVATE_KEY --broadcast -vvvv --verify --etherscan-api-key $ETHERSCAN_API_KEY --resume
contract Deployer is Script, Test, Utils {
    string internal constant _OUTPUT_PATH =
        "./script/upgrades-and-migrations/holesky/S1_U1MX_FinalizerTaskManager/upgrade_output.json";

    ProxyAdmin public avsProxyAdmin;
    FinalizerTaskManager public taskManager;
    FinalizerTaskManager public taskManagerImplementation;

    function run() external {
        require(!vm.exists(_OUTPUT_PATH), "Output already exists!");

        _loadData();
        vm.startBroadcast();

        taskManagerImplementation = new FinalizerTaskManager();
        avsProxyAdmin.upgrade(
            TransparentUpgradeableProxy(payable(address(taskManager))), address(taskManagerImplementation)
        );

        vm.stopBroadcast();
        _writeOutput();
    }

    function verify() external {
        _loadData();

        string memory upgradeOutputPath = string.concat(vm.projectRoot(), "./upgrade_output.json");
        string memory upgradeData = vm.readFile(upgradeOutputPath);
        taskManagerImplementation =
            FinalizerTaskManager(stdJson.readAddress(upgradeData, ".addresses.taskManagerImplementation"));

        require(
            avsProxyAdmin.getProxyImplementation(TransparentUpgradeableProxy(payable(address(taskManager))))
                == address(taskManagerImplementation),
            "taskManager: implementation set incorrectly"
        );
    }

    function _loadData() internal {
        string memory configPath = string.concat(vm.projectRoot(), "/script/config/17000/deploy.config.json");
        string memory configData = vm.readFile(configPath);
        uint256 configChainId = stdJson.readUint(configData, ".chainInfo.chainId");
        require(configChainId == block.chainid, "You are on the wrong chain for this config");

        string memory avsDeploymentOutputPath =
            string.concat(vm.projectRoot(), "/script/output/17000/avs_deployment_output.json");
        string memory avsDeploymentData = vm.readFile(avsDeploymentOutputPath);
        avsProxyAdmin = ProxyAdmin(stdJson.readAddress(avsDeploymentData, ".addresses.avsProxyAdmin"));
        taskManager = FinalizerTaskManager(stdJson.readAddress(avsDeploymentData, ".addresses.taskManager"));

        emit log_named_uint("You are deploying on chain id", block.chainid);
    }

    function _writeOutput() internal {
        string memory parentObject = "parent object";

        string memory deployedAddresses = "addresses";
        vm.serializeAddress(deployedAddresses, "taskManager", address(taskManager));
        string memory deployedAddressesOutput =
            vm.serializeAddress(deployedAddresses, "taskManagerImplementation", address(taskManagerImplementation));

        string memory chainInfo = "chainInfo";
        vm.serializeUint(chainInfo, "lastUpgradeImplDeployBlock", block.number);

        string memory chainInfoOutput = vm.serializeUint(chainInfo, "chainId", block.chainid);
        vm.serializeString(parentObject, chainInfo, chainInfoOutput);

        string memory finalJson = vm.serializeString(parentObject, deployedAddresses, deployedAddressesOutput);
        vm.writeJson(finalJson, _OUTPUT_PATH);
        console.logString(finalJson);
    }
}

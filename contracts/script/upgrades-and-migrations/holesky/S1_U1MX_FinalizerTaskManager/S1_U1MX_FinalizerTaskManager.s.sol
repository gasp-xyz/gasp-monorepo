// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

// Link Github PR, Jira Ticket and Notion Tracker
// Jira GASP-2120
// S1_U1MX - 1st Script - Contract Upgrade 1 / No Migration
// Contract to upgrade: FinalizerTaskManager

import {RegistryCoordinator} from "@eigenlayer-middleware/src/RegistryCoordinator.sol";
import {StakeRegistry} from "@eigenlayer-middleware/src/StakeRegistry.sol";
import {AVSDirectory} from "@eigenlayer/contracts/core/AVSDirectory.sol";
import {RewardsCoordinator} from "@eigenlayer/contracts/core/RewardsCoordinator.sol";
import {ProxyAdmin, TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {console} from "forge-std/console.sol";
import {Script} from "forge-std/Script.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {Test} from "forge-std/Test.sol";
import {FinalizerTaskManager} from "../../../../src/FinalizerTaskManager.sol";
import {IFinalizerTaskManager} from "../../../../src/interfaces/IFinalizerTaskManager.sol";
import {Utils} from "../../../utils/Utils.sol";

// To deploy and verify contract
// forge script script/Alpha_init_deploy.s.sol:Deployer --rpc-url $RPC_URL  --private-key $PRIVATE_KEY --broadcast -vvvv --verify --etherscan-api-key $ETHERSCAN_API_KEY --resume

// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Deploys finalizer contracts
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
contract Deployer is Script, Utils, Test {
    string internal constant _OUTPUT_PATH =
        "./script/upgrades-and-migrations/holesky/S1_U1MX_FinalizerTaskManager/upgrade_output.json";

    ProxyAdmin public avsProxyAdmin;
    // Upgradeable contract proxies
    FinalizerTaskManager public taskManager;
    // Upgradeable contract implementations
    FinalizerTaskManager public taskManagerImplementation;

    function run() external {
        require(!vm.exists(_OUTPUT_PATH), "Output already exists!");

        _loadData();
        vm.startBroadcast();

        taskManagerImplementation = new FinalizerTaskManager();

        vm.stopBroadcast();
        _writeOutput();
    }

    function verify() external {
        _loadData();

        string memory path = string.concat(vm.projectRoot(), "./upgrade_output.json");
        string memory upgradeData = vm.readFile(path);
        taskManagerImplementation =
            FinalizerTaskManager(stdJson.readAddress(upgradeData, ".addresses.serviceManagerImplementation"));

        require(
            avsProxyAdmin.getProxyImplementation(TransparentUpgradeableProxy(payable(address(taskManager))))
                == address(taskManagerImplementation),
            "taskManager: implementation set incorrectly"
        );
    }

    function getOutputPath() public pure returns (string memory) {
        return _OUTPUT_PATH;
    }

    function _loadData() internal {
        // Eigenlayer contracts
        string memory path = string.concat(vm.projectRoot(), "/script/config/holesky-stub.json");
        string memory configData = vm.readFile(path);

        path = string.concat(vm.projectRoot(), "/script/output/17000/avs_deployment_output.json");
        string memory avsDeploymentData = vm.readFile(path);

        uint256 configChainId = stdJson.readUint(configData, ".chainInfo.chainId");
        require(configChainId == block.chainid, "You are on the wrong chain for this config");
        taskManager = FinalizerTaskManager(stdJson.readAddress(avsDeploymentData, ".addresses.taskManager"));

        emit log_named_uint("You are deploying on ChainID", block.chainid);
    }

    function _writeOutput() internal {
        string memory parentObject = "parent object";

        string memory deployedAddresses = "addresses";
        vm.serializeAddress(deployedAddresses, "serviceManager", address(taskManager));
        string memory deployedAddressesOutput = vm.serializeAddress(
            deployedAddresses, "taskManagerImplementation", address(taskManagerImplementation)
        );

        string memory chainInfo = "chainInfo";
        vm.serializeUint(chainInfo, "lastUpgradeImplDeployBlock", block.number);
        string memory chainInfoOutput = vm.serializeUint(chainInfo, "chainId", block.chainid);

        vm.serializeString(parentObject, chainInfo, chainInfoOutput);
        string memory finalJson = vm.serializeString(parentObject, deployedAddresses, deployedAddressesOutput);
        console.logString(finalJson);
        _writeOutputLocal(finalJson);
    }

    function _writeOutputLocal(string memory outputJson) internal {
        vm.writeJson(outputJson, getOutputPath());
    }
}

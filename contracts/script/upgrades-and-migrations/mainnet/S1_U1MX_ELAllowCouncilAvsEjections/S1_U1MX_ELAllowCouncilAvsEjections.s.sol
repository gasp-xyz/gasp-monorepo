// Link Github PR, Jira Ticket and Notion Tracker
// Jira GASP-1315

// S1_U1MX - 1st Upgrade or Migration - Contract Upgrade 1, No Migration
// Contracts upgraded: FinalizerServiceManager

// This upgrade allows the Ethereum L1 Council to be able to eject Eigenlayer Avs Operators

// The following script merely create a new implementation contract
// After this the council will require to call the ProxyAdmin to upgrade the proxy
// No initialization required, since initialization alters proxy storage, and proxy storage is of course already initialized
// After switching the implementation run the Verify to ensure that the proxy has infact changed as desired
// Update the /script/output/1/avs_deployment_output.json with the output file in this folder

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";

import "@eigenlayer/contracts/permissions/PauserRegistry.sol";
import "@eigenlayer/contracts/core/AVSDirectory.sol";
import "@eigenlayer/contracts/core/DelegationManager.sol";
import "@eigenlayer/contracts/core/RewardsCoordinator.sol";
import "@eigenlayer/test/mocks/EmptyContract.sol";
import "@openzeppelin/contracts/utils/Strings.sol";

import "@eigenlayer-middleware/src/interfaces/IStakeRegistry.sol";
import "@eigenlayer-middleware/src/RegistryCoordinator.sol";
import "@eigenlayer-middleware/src/BLSApkRegistry.sol";
import "@eigenlayer-middleware/src/IndexRegistry.sol";
import "@eigenlayer-middleware/src/StakeRegistry.sol";
import {BLSSignatureChecker} from "@eigenlayer-middleware/src/BLSSignatureChecker.sol";

import {FinalizerServiceManager, IServiceManager} from "../../../../src/FinalizerServiceManager.sol";
import {FinalizerTaskManager} from "../../../../src/FinalizerTaskManager.sol";
import {IFinalizerTaskManager} from "../../../../src/interfaces/IFinalizerTaskManager.sol";
import {IRolldownPrimitives} from "../../../../src/interfaces/IRolldownPrimitives.sol";
import {OperatorStateRetrieverExtended} from "../../../../src/OperatorStateRetrieverExtended.sol";
import {Rolldown} from "../../../../src/Rolldown.sol";

import {Utils} from "../../../utils/Utils.sol";

import "forge-std/Test.sol";
import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "forge-std/console.sol";

// # To deploy and verify our contract
// forge script script/Alpha_init_deploy.s.sol:Deployer --rpc-url $RPC_URL  --private-key $PRIVATE_KEY --broadcast -vvvv --verify --etherscan-api-key $ETHERSCAN_API_KEY --resume

// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Deploys finalizer contracts
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
contract Deployer is Script, Utils, Test {
    string constant _OUTPUT_PATH = "./script/upgrades-and-migrations/mainnet/S1_U1MX_ELAllowCouncilAvsEjections/upgrade_output.json";

    ProxyAdmin public avsProxyAdmin;
    address public avsOwner;

    //upgradeable contracts
    FinalizerServiceManager public serviceManager;
    FinalizerTaskManager public taskManager;
    RegistryCoordinator public registryCoordinator;
    StakeRegistry public stakeRegistry;

    //upgradeable contract implementations
    FinalizerServiceManager public serviceManagerImplementation;
    RegistryCoordinator public registryCoordinatorImplementation;

    // EigenLayer Contracts
    AVSDirectory public avsDirectory;
    RewardsCoordinator public rewardsCoordinator;

    address public ejector;
    uint64 public recurrentRegistrationLimit;

    function run() external {

        require(!vm.exists(_OUTPUT_PATH), "Output already exists!");

        _loadData();

        // START BROADCAST
        vm.startBroadcast();

        serviceManagerImplementation = new FinalizerServiceManager(
            avsDirectory,
            rewardsCoordinator,
            registryCoordinator,
            stakeRegistry,
            IFinalizerTaskManager(address(taskManager)),
            recurrentRegistrationLimit
        );

        // end deployment
        vm.stopBroadcast();

        //write output
        _writeOutput();
    }

    function _loadData() internal {
        // Eigenlayer contracts
        string memory path = string.concat(
            vm.projectRoot(),
            "/lib/eigenlayer-middleware/lib/eigenlayer-contracts/script/configs/mainnet/Mainnet_current_deployment.config.json"
        );
        string memory eigenlayerDeployedContracts = vm.readFile(path);
        avsDirectory = AVSDirectory(stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.avsDirectory"));
        rewardsCoordinator =
            RewardsCoordinator(stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.rewardsCoordinator"));

        // READ JSON CONFIG DATA
        path = string.concat(
            vm.projectRoot(),
            "/script/config/eth_main/deploy_init.config.json"
        );
        string memory configData = vm.readFile(path);

        path = string.concat(
            vm.projectRoot(),
            "/script/output/1/avs_deployment_output.json"
        );
        string memory avsDeploymentData = vm.readFile(path);

        // check that the chainID matches the one in the config
        uint256 configChainId = stdJson.readUint(configData, ".chainInfo.chainId");
        emit log_named_uint("You are deploying on ChainID", block.chainid);
        require(configChainId == block.chainid, "You are on the wrong chain for this config");

        avsOwner = stdJson.readAddress(configData, ".permissions.owner");
        ejector = stdJson.readAddress(configData, ".permissions.ejector");
        recurrentRegistrationLimit =
            uint64(stdJson.readUint(configData, ".registryParams.recurrentRegistrationLimit"));

        serviceManager = FinalizerServiceManager(stdJson.readAddress(avsDeploymentData, ".addresses.serviceManager"));
        registryCoordinator = RegistryCoordinator(stdJson.readAddress(avsDeploymentData, ".addresses.registryCoordinator"));
        registryCoordinatorImplementation = RegistryCoordinator(stdJson.readAddress(avsDeploymentData, ".addresses.registryCoordinatorImplementation"));
        stakeRegistry = StakeRegistry(stdJson.readAddress(avsDeploymentData, ".addresses.stakeRegistry"));
        taskManager = FinalizerTaskManager(stdJson.readAddress(avsDeploymentData, ".addresses.taskManager"));

    }

    function verify() external {
        _loadData();

        string memory path = string.concat(
            vm.projectRoot(),
            "./upgrade_output.json"
        );
        string memory upgradeData = vm.readFile(path);
        serviceManagerImplementation = FinalizerServiceManager(stdJson.readAddress(upgradeData, ".addresses.serviceManagerImplementation"));

        // sanity checks
        _verifyContractPointers(
            serviceManager, registryCoordinator
        );

        _verifyContractPointers(
            serviceManagerImplementation,
            registryCoordinatorImplementation
        );

        _verifyImplementations();
        _verifyInitalizations(
        );
    }

    function _verifyContractPointers(
        FinalizerServiceManager _serviceManager,
        RegistryCoordinator _registryCoordinator
    ) internal view {
        require(_serviceManager.taskManager() == taskManager, "serviceManager.taskManager() != taskManager");

        require(
            address(_registryCoordinator.serviceManager()) == address(serviceManager),
            "registryCoordinator.serviceManager() != serviceManager"
        );

    }

    function _verifyImplementations() internal view {
        require(
            avsProxyAdmin.getProxyImplementation(TransparentUpgradeableProxy(payable(address(serviceManager))))
                == address(serviceManagerImplementation),
            "serviceManager: implementation set incorrectly"
        );
    }

    function _verifyInitalizations(
    ) internal view {
        require(serviceManager.owner() == avsOwner, "serviceManager.owner() != avsOwner");
        require(serviceManager.ejector() == ejector, "serviceManager.ejector() != ejector");
        require(
            registryCoordinator.ejector() == address(serviceManager), "registryCoordinator.ejector() != serviceManager"
        );
    }


    function _writeOutput() internal {
        string memory parent_object = "parent object";

        string memory deployed_addresses = "addresses";
        vm.serializeAddress(deployed_addresses, "serviceManager", address(serviceManager));
        string memory deployed_addresses_output =
        vm.serializeAddress(deployed_addresses, "serviceManagerImplementation", address(serviceManagerImplementation));

        string memory chain_info = "chainInfo";
        vm.serializeUint(chain_info, "lastUpgradeImplDeployBlock", block.number);
        string memory chain_info_output = vm.serializeUint(chain_info, "chainId", block.chainid);

        vm.serializeString(parent_object, chain_info, chain_info_output);
        string memory finalJson = vm.serializeString(parent_object, deployed_addresses, deployed_addresses_output);
        console.logString(finalJson);
        _writeOutputLocal(finalJson);
    }

    function _writeOutputLocal(
        string memory outputJson
    ) internal {
        vm.writeJson(outputJson, getOutputPath());
    }

    function getOutputPath() public view returns (string memory) {
        return _OUTPUT_PATH;
    }
}

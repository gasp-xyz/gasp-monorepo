// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {EmptyContract} from "@eigenlayer/test/mocks/EmptyContract.sol";
import {console} from "forge-std/console.sol";
import {Script} from "forge-std/Script.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {AnvilSetup} from "../script/0_AnvilSetup.s.sol";
import {FinalizerAVSDeployer, FinalizerTaskManager} from "../script/1_FinalizerAVSDeployer.s.sol";
import {
    GaspMultiRollupService, GaspMultiRollupServiceDeployer
} from "../script/GaspMultiRollupServiceDeployer.s.sol";
import {M2Deployer} from "../script/M2_Deploy_From_Scratch.s.sol";
import {IRolldownPrimitives, Rolldown, RolldownDeployer} from "../script/RolldownDeployer.s.sol";
import {IRolldown} from "../src/interfaces/IRolldown.sol";
import {Utils} from "./utils/Utils.sol";

contract MultiStage is Script, Utils {
    function run() external {
        bytes32 env = keccak256(abi.encodePacked(vm.envString("ENV_SELECTOR")));

        if (env == keccak256(abi.encodePacked("ethereum-stub"))) {
            _incrementAccountNonce();

            M2Deployer eigenDeployer = new M2Deployer();
            AnvilSetup anvilDeployer = new AnvilSetup();
            FinalizerAVSDeployer finalizerAVSDeployer = new FinalizerAVSDeployer();
            RolldownDeployer rolldownDeployer = new RolldownDeployer();

            if (!rolldownDeployer.isProxyDeployed()) {
                console.log("################################################################################");
                console.log("Deploying eigen layer infra");
                console.log("################################################################################");
                eigenDeployer.run("M2_deploy_from_scratch.anvil.config.json");

                console.log("################################################################################");
                console.log("Initializing eigen layer infra");
                console.log("################################################################################");
                anvilDeployer.run();

                console.log("################################################################################");
                console.log("Deploying finalizer contracts");
                console.log("################################################################################");
                finalizerAVSDeployer.run();

                console.log("################################################################################");
                console.log("Deploying rolldown contracts");
                console.log("################################################################################");
                rolldownDeployer.run(IRolldownPrimitives.ChainId.Ethereum);
            } else {
                console.log("################################################################################");
                console.log("Deploying rolldown contracts");
                console.log("################################################################################");
                rolldownDeployer.run(IRolldownPrimitives.ChainId.Ethereum);
            }

            Rolldown rolldown;
            FinalizerTaskManager taskManager;
            address avsOwner;

            string memory eigenlayerDeployedContracts = readOutput("avs_deployment_output");
            taskManager =
                FinalizerTaskManager(stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.taskManager"));

            string memory rolldownDeployedContracts = readOutput("rolldown_output");
            rolldown = Rolldown(payable(stdJson.readAddress(rolldownDeployedContracts, ".addresses.rolldown")));

            string memory _CONFIG_PATH = "deploy.config";
            string memory configData = readConfig(_CONFIG_PATH);
            avsOwner = stdJson.readAddress(configData, ".permissions.owner");

            vm.startBroadcast(avsOwner);

            taskManager.setRolldown(IRolldown(address(rolldown)));
            rolldown.setUpdater(address(taskManager));

            vm.stopBroadcast();
        } else if (env == keccak256(abi.encodePacked("arbitrum-stub"))) {
            _incrementAccountNonce();
            _deployRolldownAndGMRS(IRolldownPrimitives.ChainId.Arbitrum);
        } else if (env == keccak256(abi.encodePacked("base-stub"))) {
            _incrementAccountNonce();
            _deployRolldownAndGMRS(IRolldownPrimitives.ChainId.Base);
        } else if (env == keccak256(abi.encodePacked("arbitrum-sepolia"))) {
            if (outputExists("arbitrum_rolldown_output") || outputExists("arbitrum_gmrs_output")) {
                revert("Already deployed");
            }
            _deployRolldownAndGMRS(IRolldownPrimitives.ChainId.Arbitrum);
        } else if (env == keccak256(abi.encodePacked("base-sepolia"))) {
            if (outputExists("base_rolldown_output") || outputExists("base_gmrs_output")) {
                revert("Already deployed");
            }
            _deployRolldownAndGMRS(IRolldownPrimitives.ChainId.Base);
        } else if (env == keccak256(abi.encodePacked("ethereum-holesky"))) {
            console.log("################################################################################");
            console.log("Deploying finalizer contracts");
            console.log("################################################################################");
            FinalizerAVSDeployer finalizerAVSDeployer = new FinalizerAVSDeployer();
            if (outputExists(finalizerAVSDeployer.getOutputPath())) {
                revert("Already deployed");
            }

            finalizerAVSDeployer.run();

            console.log("################################################################################");
            console.log("Deploying rolldown contracts");
            console.log("################################################################################");
            RolldownDeployer rolldownDeployer = new RolldownDeployer();
            rolldownDeployer.run(IRolldownPrimitives.ChainId.Ethereum);

            Rolldown rolldown;
            FinalizerTaskManager taskManager;
            address avsOwner;

            string memory eigenlayerDeployedContracts = readOutput("avs_deployment_output");
            taskManager =
                FinalizerTaskManager(stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.taskManager"));

            string memory rolldownDeployedContracts = readOutput("rolldown_output");
            rolldown = Rolldown(payable(stdJson.readAddress(rolldownDeployedContracts, ".addresses.rolldown")));

            string memory configData = readConfig("deploy.config");
            avsOwner = stdJson.readAddress(configData, ".permissions.owner");

            vm.startBroadcast(avsOwner);

            taskManager.setRolldown(IRolldown(address(rolldown)));
            rolldown.setUpdater(address(taskManager));

            vm.stopBroadcast();
        } else if (env == keccak256(abi.encodePacked("upgrade-rolldown-ethereum-holesky"))) {
            console.log("################################################################################");
            console.log("Upgrading rolldown contracts");
            console.log("################################################################################");
            RolldownDeployer rolldownDeployer = new RolldownDeployer();
            // We only want upgrade and not deploy from scratch
            require(rolldownDeployer.isProxyDeployed(), "Proxy not deployed");
            rolldownDeployer.run(IRolldownPrimitives.ChainId.Ethereum);
        } else if (env == keccak256(abi.encodePacked("upgrade-rolldown-arbitrum-sepolia"))) {
            console.log("################################################################################");
            console.log("Upgrading rolldown contracts");
            console.log("################################################################################");
            RolldownDeployer rolldownDeployer = new RolldownDeployer();
            // We only want upgrade and not deploy from scratch
            require(rolldownDeployer.isProxyDeployed(), "Proxy not deployed");
            rolldownDeployer.run(IRolldownPrimitives.ChainId.Arbitrum);
        } else if (env == keccak256(abi.encodePacked("upgrade-rolldown-base-sepolia"))) {
            console.log("################################################################################");
            console.log("Upgrading rolldown contracts");
            console.log("################################################################################");
            RolldownDeployer rolldownDeployer = new RolldownDeployer();
            // We only want upgrade and not deploy from scratch
            require(rolldownDeployer.isProxyDeployed(), "Proxy not deployed");
            rolldownDeployer.run(IRolldownPrimitives.ChainId.Base);
        } else {
            revert("Unsupported environment");
        }
    }

    function _incrementAccountNonce() private {
        vm.startBroadcast();
        new EmptyContract();
        vm.stopBroadcast();
    }

    function _deployRolldownAndGMRS(IRolldownPrimitives.ChainId chain) private {
        Rolldown rolldown;
        GaspMultiRollupService gmrs;
        address avsOwner;

        console.log("################################################################################");
        console.log("Deploying rolldown contracts");
        console.log("################################################################################");
        RolldownDeployer rolldownDeployer = new RolldownDeployer();
        rolldownDeployer.run(chain);

        string memory configData = readConfig("deploy.config");
        avsOwner = stdJson.readAddress(configData, ".permissions.owner");

        console.log("################################################################################");
        console.log("Deploying gaspMultiRollupService contracts");
        console.log("################################################################################");
        GaspMultiRollupServiceDeployer gaspMultiRollupServiceDeployer = new GaspMultiRollupServiceDeployer();
        gaspMultiRollupServiceDeployer.run(chain);

        string memory gmrsDeployedContracts = readOutput("gmrs_output");
        gmrs = GaspMultiRollupService(stdJson.readAddress(gmrsDeployedContracts, ".addresses.gmrs"));

        string memory rolldownDeployedContracts = readOutput("rolldown_output");
        rolldown = Rolldown(payable(stdJson.readAddress(rolldownDeployedContracts, ".addresses.rolldown")));

        vm.startBroadcast(avsOwner);

        gmrs.setRolldown(IRolldown(address(rolldown)));
        rolldown.setUpdater(address(gmrs));

        vm.stopBroadcast();
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {EmptyContract} from "@eigenlayer/test/mocks/EmptyContract.sol";
import {console} from "forge-std/console.sol";
import {Script} from "forge-std/Script.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {AnvilSetup} from "../script/0_AnvilSetup.s.sol";
import {FinalizerAVSDeployer, FinalizerTaskManager} from "../script/1_FinalizerAVSDeployer.s.sol";
import {GaspMultiRollupService, GaspMultiRollupServiceDeployer} from "../script/GaspMultiRollupServiceDeployer.s.sol";
import {M2Deployer} from "../script/M2_Deploy_From_Scratch.s.sol";
import {IRolldownPrimitives, Rolldown, RolldownDeployer} from "../script/RolldownDeployer.s.sol";
import {IRolldown} from "../src/interfaces/IRolldown.sol";
import {Utils} from "./utils/Utils.sol";

contract MultiStage is Script, Utils {
    bytes32 public envHash = _stringToHash(vm.envString("ENV_SELECTOR"));

    function run() public {
        if (envHash == _stringToHash("ethereum-stub")) {
            _incrementAccountNonce();

            M2Deployer eigenDeployer = new M2Deployer();
            AnvilSetup anvilDeployer = new AnvilSetup();
            FinalizerAVSDeployer finalizerAVSDeployer = new FinalizerAVSDeployer();
            RolldownDeployer rolldownDeployer = new RolldownDeployer();

            if (!rolldownDeployer.isProxyDeployed()) {
                _printMessage("Deploying eigen layer infra");
                eigenDeployer.run("M2_deploy_from_scratch.anvil.config.json");

                _printMessage("Initializing eigen layer infra");
                anvilDeployer.run();

                _printMessage("Deploying finalizer contracts");
                finalizerAVSDeployer.run();
            }

            _printMessage("Deploying rolldown contracts");
            rolldownDeployer.run(IRolldownPrimitives.ChainId.Ethereum);

            string memory eigenlayerDeployedContracts = readOutput("avs_deployment_output");
            FinalizerTaskManager taskManager =
                FinalizerTaskManager(stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.taskManager"));

            string memory rolldownDeployedContracts = readOutput("rolldown_output");
            Rolldown rolldown = Rolldown(payable(stdJson.readAddress(rolldownDeployedContracts, ".addresses.rolldown")));

            string memory configData = readConfig("deploy.config");
            address avsOwner = stdJson.readAddress(configData, ".permissions.owner");

            vm.startBroadcast(avsOwner);

            taskManager.setRolldown(IRolldown(address(rolldown)));
            rolldown.setUpdater(address(taskManager));

            vm.stopBroadcast();
            return;
        }

        if (envHash == _stringToHash("arbitrum-stub")) {
            _deployRolldownAndGMRS(IRolldownPrimitives.ChainId.Arbitrum);
            return;
        }

        if (envHash == _stringToHash("base-stub")) {
            _deployRolldownAndGMRS(IRolldownPrimitives.ChainId.Base);
            return;
        }

        if (envHash == _stringToHash("monad-stub")) {
            _deployRolldownAndGMRS(IRolldownPrimitives.ChainId.Monad);
            return;
        }

        if (envHash == _stringToHash("megaeth-stub")) {
            _deployRolldownAndGMRS(IRolldownPrimitives.ChainId.Megaeth);
            return;
        }

        if (envHash == _stringToHash("ethereum-holesky")) {
            _printMessage("Deploying finalizer contracts");
            FinalizerAVSDeployer finalizerAVSDeployer = new FinalizerAVSDeployer();
            require(
                !outputExists(finalizerAVSDeployer.getOutputPath()), "Contracts already deployed on Ethereum Holesky"
            );

            finalizerAVSDeployer.run();

            _printMessage("Deploying rolldown contracts");
            RolldownDeployer rolldownDeployer = new RolldownDeployer();
            rolldownDeployer.run(IRolldownPrimitives.ChainId.Ethereum);

            string memory eigenlayerDeployedContracts = readOutput("avs_deployment_output");
            FinalizerTaskManager taskManager =
                FinalizerTaskManager(stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.taskManager"));

            string memory rolldownDeployedContracts = readOutput("rolldown_output");
            Rolldown rolldown = Rolldown(payable(stdJson.readAddress(rolldownDeployedContracts, ".addresses.rolldown")));

            string memory configData = readConfig("deploy.config");
            address avsOwner = stdJson.readAddress(configData, ".permissions.owner");

            vm.startBroadcast(avsOwner);

            taskManager.setRolldown(IRolldown(address(rolldown)));
            rolldown.setUpdater(address(taskManager));

            vm.stopBroadcast();
            return;
        }

        if (envHash == _stringToHash("arbitrum-sepolia")) {
            require(
                !outputExists("arbitrum_rolldown_output") && !outputExists("arbitrum_gmrs_output"),
                "Contracts already deployed on Arbitrum Sepolia"
            );

            _deployRolldownAndGMRS(IRolldownPrimitives.ChainId.Arbitrum);
            return;
        }

        if (envHash == _stringToHash("base-sepolia")) {
            require(
                !outputExists("base_rolldown_output") && !outputExists("base_gmrs_output"),
                "Contracts already deployed on Base Sepolia"
            );

            _deployRolldownAndGMRS(IRolldownPrimitives.ChainId.Base);
            return;
        }

        if (envHash == _stringToHash("monad-testnet")) {
            require(
                !outputExists("monad_rolldown_output") && !outputExists("monad_gmrs_output"),
                "Contracts already deployed on Monad Testnet"
            );

            _deployRolldownAndGMRS(IRolldownPrimitives.ChainId.Monad);
            return;
        }

        if (envHash == _stringToHash("megaeth-testnet")) {
            require(
                !outputExists("megaeth_rolldown_output") && !outputExists("megaeth_gmrs_output"),
                "Contracts already deployed on Megaeth Testnet"
            );

            _deployRolldownAndGMRS(IRolldownPrimitives.ChainId.Megaeth);
            return;
        }

        if (envHash == _stringToHash("upgrade-rolldown-ethereum-holesky")) {
            _printMessage("Upgrading rolldown contracts");
            RolldownDeployer rolldownDeployer = new RolldownDeployer();
            require(rolldownDeployer.isProxyDeployed(), "Proxy not deployed on Ethereum Holesky");

            rolldownDeployer.run(IRolldownPrimitives.ChainId.Ethereum);
            return;
        }

        if (envHash == _stringToHash("upgrade-rolldown-arbitrum-sepolia")) {
            _printMessage("Upgrading rolldown contracts");
            RolldownDeployer rolldownDeployer = new RolldownDeployer();
            require(rolldownDeployer.isProxyDeployed(), "Proxy not deployed on Arbitrum Sepolia");

            rolldownDeployer.run(IRolldownPrimitives.ChainId.Arbitrum);
            return;
        }

        if (envHash == _stringToHash("upgrade-rolldown-base-sepolia")) {
            _printMessage("Upgrading rolldown contracts");
            RolldownDeployer rolldownDeployer = new RolldownDeployer();
            require(rolldownDeployer.isProxyDeployed(), "Proxy not deployed on Base Sepolia");

            rolldownDeployer.run(IRolldownPrimitives.ChainId.Base);
            return;
        }

        revert(string.concat("Unsupported environment: ", vm.envString("ENV_SELECTOR")));
    }

    function _incrementAccountNonce() private {
        vm.startBroadcast();
        new EmptyContract();
        vm.stopBroadcast();
    }

    function _deployRolldownAndGMRS(IRolldownPrimitives.ChainId chain) private {
        _incrementAccountNonce();

        _printMessage("Deploying rolldown contracts");
        RolldownDeployer rolldownDeployer = new RolldownDeployer();
        rolldownDeployer.run(chain);

        string memory configData = readConfig("deploy.config");
        address avsOwner = stdJson.readAddress(configData, ".permissions.owner");

        _printMessage("Deploying gasp multi rollup service contracts");
        GaspMultiRollupServiceDeployer gaspMultiRollupServiceDeployer = new GaspMultiRollupServiceDeployer();
        gaspMultiRollupServiceDeployer.run(chain);

        string memory gmrsDeployedContracts = readOutput("gmrs_output");
        GaspMultiRollupService gmrs =
            GaspMultiRollupService(stdJson.readAddress(gmrsDeployedContracts, ".addresses.gmrs"));

        string memory rolldownDeployedContracts = readOutput("rolldown_output");
        Rolldown rolldown = Rolldown(payable(stdJson.readAddress(rolldownDeployedContracts, ".addresses.rolldown")));

        vm.startBroadcast(avsOwner);

        gmrs.setRolldown(IRolldown(address(rolldown)));
        rolldown.setUpdater(address(gmrs));

        vm.stopBroadcast();
    }

    function _printMessage(string memory message) private pure {
        console.log("################################################################################");
        console.log(message);
        console.log("################################################################################");
    }

    function _stringToHash(string memory s) private pure returns (bytes32) {
        return keccak256(abi.encodePacked(s));
    }
}

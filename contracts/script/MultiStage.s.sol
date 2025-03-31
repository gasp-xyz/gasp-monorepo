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

            rolldownDeployer.run(IRolldownPrimitives.ChainId.Ethereum, "ethereum-stub");

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
            _runContractDeployers(IRolldownPrimitives.ChainId.Arbitrum, "arbitrum-stub");
            return;
        }

        if (envHash == _stringToHash("base-stub")) {
            _runContractDeployers(IRolldownPrimitives.ChainId.Base, "base-stub");
            return;
        }

        if (envHash == _stringToHash("monad-stub")) {
            _runContractDeployers(IRolldownPrimitives.ChainId.Monad, "monad-stub");
            return;
        }

        if (envHash == _stringToHash("megaeth-stub")) {
            _runContractDeployers(IRolldownPrimitives.ChainId.MegaEth, "megaeth-stub");
            return;
        }

        if (envHash == _stringToHash("ethereum-holesky")) {
            _printMessage("Deploying finalizer contracts");
            FinalizerAVSDeployer finalizerAVSDeployer = new FinalizerAVSDeployer();
            require(
                !outputExists(finalizerAVSDeployer.getOutputPath()), "Contracts already deployed on Ethereum Holesky"
            );
            finalizerAVSDeployer.run();

            RolldownDeployer rolldownDeployer = new RolldownDeployer();
            rolldownDeployer.run(IRolldownPrimitives.ChainId.Ethereum, "ethereum-holesky");

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
            _runContractDeployers(IRolldownPrimitives.ChainId.Arbitrum, "arbitrum-sepolia");
            return;
        }

        if (envHash == _stringToHash("base-sepolia")) {
            _runContractDeployers(IRolldownPrimitives.ChainId.Base, "base-sepolia");
            return;
        }

        if (envHash == _stringToHash("monad-testnet")) {
            _runContractDeployers(IRolldownPrimitives.ChainId.Monad, "monad-testnet");
            return;
        }

        if (envHash == _stringToHash("megaeth-testnet")) {
            _runContractDeployers(IRolldownPrimitives.ChainId.MegaEth, "megaeth-testnet");
            return;
        }

        revert(string.concat("Unsupported environment: ", vm.envString("ENV_SELECTOR")));
    }

    function _incrementAccountNonce() private {
        vm.startBroadcast();
        new EmptyContract();
        vm.stopBroadcast();
    }

    function _runContractDeployers(IRolldownPrimitives.ChainId chainId, string memory chainName) private {
        _incrementAccountNonce();

        RolldownDeployer rolldownDeployer = new RolldownDeployer();
        rolldownDeployer.run(chainId, chainName);

        string memory rolldownDeployedContracts = readOutput("rolldown_output");
        address rolldownAddress = stdJson.readAddress(rolldownDeployedContracts, ".addresses.rolldown");
        Rolldown rolldown = Rolldown(payable(rolldownAddress));

        GaspMultiRollupServiceDeployer gaspMultiRollupServiceDeployer = new GaspMultiRollupServiceDeployer();
        gaspMultiRollupServiceDeployer.run(chainId, chainName);

        string memory gmrsDeployedContracts = readOutput("gmrs_output");
        address gmrsAddress = stdJson.readAddress(gmrsDeployedContracts, ".addresses.gmrs");
        GaspMultiRollupService gmrs = GaspMultiRollupService(gmrsAddress);

        string memory configData = readConfig("deploy.config");
        address avsOwner = stdJson.readAddress(configData, ".permissions.owner");

        vm.startBroadcast(avsOwner);

        gmrs.setRolldown(IRolldown(rolldownAddress));
        rolldown.setUpdater(gmrsAddress);

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

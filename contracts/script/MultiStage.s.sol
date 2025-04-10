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
                _printMessage(string.concat("Deploying eigen layer infra to ethereum-stub"));
                eigenDeployer.run("M2_deploy_from_scratch.anvil.config.json");

                _printMessage(string.concat("Initializing eigen layer infra in ethereum-stub"));
                anvilDeployer.run();

                _printMessage(string.concat("Deploying finalizer contracts to ethereum-stub"));
                finalizerAVSDeployer.run();
            }

            rolldownDeployer.run(IRolldownPrimitives.ChainId.Ethereum, "ethereum-stub");

            _initializeRolldownAndTaskManager();
            return;
        }

        if (envHash == _stringToHash("arbitrum-stub")) {
            _runRolldownAndGMRSDeployers(IRolldownPrimitives.ChainId.Arbitrum, "arbitrum-stub");
            return;
        }

        if (envHash == _stringToHash("base-stub")) {
            _runRolldownAndGMRSDeployers(IRolldownPrimitives.ChainId.Base, "base-stub");
            return;
        }

        if (envHash == _stringToHash("monad-stub")) {
            _runRolldownAndGMRSDeployers(IRolldownPrimitives.ChainId.Monad, "monad-stub");
            return;
        }

        if (envHash == _stringToHash("megaeth-stub")) {
            _runRolldownAndGMRSDeployers(IRolldownPrimitives.ChainId.MegaEth, "megaeth-stub");
            return;
        }

        if (envHash == _stringToHash("berachain-stub")) {
            _runRolldownAndGMRSDeployers(IRolldownPrimitives.ChainId.Berachain, "berachain-stub");
            return;
        }

        if (envHash == _stringToHash("ethereum-holesky")) {
            _printMessage(string.concat("Deploying finalizer contracts to ethereum-holesky"));
            FinalizerAVSDeployer finalizerAVSDeployer = new FinalizerAVSDeployer();
            finalizerAVSDeployer.run();

            RolldownDeployer rolldownDeployer = new RolldownDeployer();
            rolldownDeployer.run(IRolldownPrimitives.ChainId.Ethereum, "ethereum-holesky");

            _initializeRolldownAndTaskManager();
            return;
        }

        if (envHash == _stringToHash("arbitrum-sepolia")) {
            _runRolldownAndGMRSDeployers(IRolldownPrimitives.ChainId.Arbitrum, "arbitrum-sepolia");
            return;
        }

        if (envHash == _stringToHash("base-sepolia")) {
            _runRolldownAndGMRSDeployers(IRolldownPrimitives.ChainId.Base, "base-sepolia");
            return;
        }

        if (envHash == _stringToHash("monad-testnet")) {
            _runRolldownAndGMRSDeployers(IRolldownPrimitives.ChainId.Monad, "monad-testnet");
            return;
        }

        if (envHash == _stringToHash("megaeth-testnet")) {
            _runRolldownAndGMRSDeployers(IRolldownPrimitives.ChainId.MegaEth, "megaeth-testnet");
            return;
        }

        revert(string.concat("Unsupported environment: ", vm.envString("ENV_SELECTOR")));
    }

    function _incrementAccountNonce() private {
        vm.startBroadcast();
        new EmptyContract();
        vm.stopBroadcast();
    }

    function _runRolldownAndGMRSDeployers(IRolldownPrimitives.ChainId chainId, string memory chainName) private {
        _incrementAccountNonce();

        RolldownDeployer rolldownDeployer = new RolldownDeployer();
        rolldownDeployer.run(chainId, chainName);

        string memory rolldownDeployedContracts = readOutput(rolldownDeployer.outputPath());
        address rolldownAddress = stdJson.readAddress(rolldownDeployedContracts, ".addresses.rolldown");
        Rolldown rolldown = Rolldown(payable(rolldownAddress));

        GaspMultiRollupServiceDeployer gaspMultiRollupServiceDeployer = new GaspMultiRollupServiceDeployer();
        gaspMultiRollupServiceDeployer.run(chainId, chainName);

        string memory gmrsDeployedContracts = readOutput(gaspMultiRollupServiceDeployer.outputPath());
        address gmrsAddress = stdJson.readAddress(gmrsDeployedContracts, ".addresses.gmrs");
        GaspMultiRollupService gmrs = GaspMultiRollupService(gmrsAddress);

        vm.startBroadcast(_getAVSOwner());

        gmrs.setRolldown(IRolldown(rolldownAddress));
        rolldown.setUpdater(gmrsAddress);

        vm.stopBroadcast();
    }

    function _initializeRolldownAndTaskManager() private {
        string memory eigenlayerDeployedContracts = readOutput("avs_deployment_output");
        FinalizerTaskManager taskManager =
            FinalizerTaskManager(stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.taskManager"));

        string memory rolldownDeployedContracts = readOutput("rolldown_output");
        Rolldown rolldown = Rolldown(payable(stdJson.readAddress(rolldownDeployedContracts, ".addresses.rolldown")));

        vm.startBroadcast(_getAVSOwner());

        taskManager.setRolldown(IRolldown(address(rolldown)));
        rolldown.setUpdater(address(taskManager));

        vm.stopBroadcast();
    }

    function _getAVSOwner() private view returns (address) {
        string memory configData = readConfig("deploy.config");
        return stdJson.readAddress(configData, ".permissions.owner");
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

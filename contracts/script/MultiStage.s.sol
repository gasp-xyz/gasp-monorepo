pragma solidity ^0.8.12;
import "../script/0_AnvilSetup.s.sol";
import "../script/1_FinalizerAvsDeployer.s.sol";
import "../script/M2_Deploy_From_Scratch.s.sol";
import "../script/RolldownDeployer.s.sol";
import "../script/GaspMultiRollupServiceDeployer.s.sol";
import "../src/IRolldown.sol";
import {IRolldownPrimitives} from "../src/Rolldown.sol";
import {Utils} from "./utils/Utils.sol";

import "forge-std/StdJson.sol";

contract MultiStage is Script, Utils, Test {

    function deployRolldown() private {
    }

    function run() external {
      string memory variant = vm.envString("ENV_SELECTOR");
      if (keccak256(abi.encodePacked(variant)) == keccak256(abi.encodePacked("ethereum-stub"))){
        Deployer_M2 eigenDeployer = new Deployer_M2();
        AnvilSetup anvilDeployer = new AnvilSetup();
        Deployer finalizerDeployer = new Deployer();
        RolldownDeployer rolldownDeployer = new RolldownDeployer();

        if (!rolldownDeployer.isProxyDeployed(IRolldownPrimitives.ChainId.Ethereum)) {
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
          finalizerDeployer.run();

          console.log("################################################################################");
          console.log("Deploying rolldown contracts");
          console.log("################################################################################");
          rolldownDeployer.run(IRolldownPrimitives.ChainId.Ethereum);
        } else {
          //TODO: 
          //redeploy finalizer contracts as well

          console.log("################################################################################");
          console.log("Deploying rolldown contracts");
          console.log("################################################################################");
          rolldownDeployer.run(IRolldownPrimitives.ChainId.Ethereum);
        }


        Rolldown rolldown;
        FinalizerTaskManager taskManager;
        address avsOwner;

        string memory _EIGEN_OUTPUT_PATH = "avs_deployment_output";
        string memory eigenlayerDeployedContracts = readOutput(_EIGEN_OUTPUT_PATH);
        taskManager = FinalizerTaskManager(stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.taskManager"));

        string memory _ROLLDOWN_OUTPUT_PATH = "rolldown_output";
        string memory rolldownDeployedContracts = readOutput(evmPrefixedPath(IRolldownPrimitives.ChainId.Ethereum, _ROLLDOWN_OUTPUT_PATH));
        rolldown = Rolldown(stdJson.readAddress(rolldownDeployedContracts, ".addresses.rolldown"));

        string memory _CONFIG_PATH = "deploy.config";
        string memory configData = readConfig(_CONFIG_PATH);
        avsOwner = stdJson.readAddress(configData, ".permissions.owner");

        vm.startBroadcast(avsOwner);

        taskManager.setRolldown(IRolldown(address(rolldown)));
        rolldown.setUpdater(address(taskManager));

        vm.stopBroadcast();


      }else if (keccak256(abi.encodePacked(variant)) == keccak256(abi.encodePacked("arbitrum-stub"))){

        console.log("################################################################################");
        console.log("Deploying rolldown contracts");
        console.log("################################################################################");
        RolldownDeployer rolldownDeployer = new RolldownDeployer();
        rolldownDeployer.run(IRolldownPrimitives.ChainId.Arbitrum);

        console.log("################################################################################");
        console.log("Deploying gaspMultiRollupService contracts");
        console.log("################################################################################");
        GaspMultiRollupServiceDeployer gaspMultiRollupServiceDeployer = new GaspMultiRollupServiceDeployer();
        gaspMultiRollupServiceDeployer.run(IRolldownPrimitives.ChainId.Arbitrum, true);


        Rolldown rolldown;
        GaspMultiRollupService gmrs;
        address avsOwner;

        string memory _GMRS_OUTPUT_PATH = "gmrs_output";
        string memory gmrsDeployedContracts = readOutput(evmPrefixedPath(IRolldownPrimitives.ChainId.Arbitrum, _GMRS_OUTPUT_PATH));
        gmrs = GaspMultiRollupService(stdJson.readAddress(gmrsDeployedContracts, ".addresses.gmrs"));

        string memory _ROLLDOWN_OUTPUT_PATH = "rolldown_output";
        string memory rolldownDeployedContracts = readOutput(evmPrefixedPath(IRolldownPrimitives.ChainId.Arbitrum, _ROLLDOWN_OUTPUT_PATH));
        rolldown = Rolldown(stdJson.readAddress(rolldownDeployedContracts, ".addresses.rolldown"));

        string memory _CONFIG_PATH = "deploy.config";
        string memory configData = readConfig(_CONFIG_PATH);
        avsOwner = stdJson.readAddress(configData, ".permissions.owner");

        vm.startBroadcast(avsOwner);

        gmrs.setRolldown(IRolldown(address(rolldown)));
        rolldown.setUpdater(address(gmrs));

        vm.stopBroadcast();

      }else if (keccak256(abi.encodePacked(variant)) == keccak256(abi.encodePacked("arbitrum-sepolia"))){

        console.log("################################################################################");
        console.log("Deploying rolldown contracts");
        console.log("################################################################################");
        RolldownDeployer rolldownDeployer = new RolldownDeployer();
        rolldownDeployer.run(IRolldownPrimitives.ChainId.Arbitrum);

      }else if (keccak256(abi.encodePacked(variant)) == keccak256(abi.encodePacked("ethereum-holesky"))){

        console.log("################################################################################");
        console.log("Deploying rolldown contracts");
        console.log("################################################################################");
        RolldownDeployer rolldownDeployer = new RolldownDeployer();
        rolldownDeployer.run(IRolldownPrimitives.ChainId.Ethereum);

      }else{
 
        //TODO: ethereum-prod
        //TODO: arbitrum-prod
        //TODO: ...
        revert("Unsupported variant");
      }
    }
}

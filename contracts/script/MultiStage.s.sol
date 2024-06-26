pragma solidity ^0.8.12;
import "../script/0_AnvilSetup.s.sol";
import "../script/1_FinalizerAvsDeployer.s.sol";
import "../script/M2_Deploy_From_Scratch.s.sol";
import "../script/RolldownDeployer.s.sol";
import {IRolldownPrimitives} from "../src/Rolldown.sol";


contract MultiStage is Script, Utils, Test {
    string constant _CONFIG_PATH = "deploy.config";

    function deployRolldown() private {
    }

    function run() external {
      string memory variant = vm.envString("ENV_SELECTOR");
      if (keccak256(abi.encodePacked(variant)) == keccak256(abi.encodePacked("ethereum-stub"))){
        Deployer_M2 eigenDeployer = new Deployer_M2();
        AnvilSetup anvilDeployer = new AnvilSetup();
        Deployer finalizerDeployer = new Deployer();
        RolldownDeployer rolldownDeployer = new RolldownDeployer();

        if (rolldownDeployer.isProxyDeployed(IRolldownPrimitives.ChainId.Ethereum)) {
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

      }else if (keccak256(abi.encodePacked(variant)) == keccak256(abi.encodePacked("arbitrum-stub"))){

        console.log("################################################################################");
        console.log("Deploying rolldown contracts");
        console.log("################################################################################");
        RolldownDeployer rolldownDeployer = new RolldownDeployer();
        rolldownDeployer.run(IRolldownPrimitives.ChainId.Arbitrum);

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

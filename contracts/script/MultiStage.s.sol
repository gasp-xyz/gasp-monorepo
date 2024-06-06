pragma solidity ^0.8.12;
import "../script/0_AnvilSetup.s.sol";
import "../script/1_FinalizerAvsDeployer.s.sol";
import "../script/M2_Deploy_From_Scratch.s.sol";


contract MultiStage is Script, Utils, Test {

    /// wrapper scripts that does whole deplyment in single step
    function run() external {
      Deployer_M2 eigenDeployer = new Deployer_M2();
      AnvilSetup anvilDeployer = new AnvilSetup();
      Deployer finalizerDeployer = new Deployer();

      console.log("Deploying from scratch");
      eigenDeployer.run("M2_deploy_from_scratch.anvil.config.json");
      console.log("Anvil deployment");
      anvilDeployer.run();
      console.log("Finalizer deployment");
      finalizerDeployer.run();
    }
}

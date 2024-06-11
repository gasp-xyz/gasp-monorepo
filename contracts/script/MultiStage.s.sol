pragma solidity ^0.8.12;
import "../script/0_AnvilSetup.s.sol";
import "../script/1_FinalizerAvsDeployer.s.sol";
import "../script/M2_Deploy_From_Scratch.s.sol";
import "../script/RolldownDeployer.s.sol";


contract MultiStage is Script, Utils, Test {
    string constant _CONFIG_PATH = "deploy.config";

    /// wrapper scripts that does whole deplyment in single step
    function run() external {
      string memory variant = vm.envString("ENV_SELECTOR");
      if (keccak256(abi.encodePacked(variant)) == keccak256(abi.encodePacked("ethereum-stub"))){
        Deployer_M2 eigenDeployer = new Deployer_M2();
        AnvilSetup anvilDeployer = new AnvilSetup();
        Deployer finalizerDeployer = new Deployer();
        RolldownDeployer rolldownDeployer = new RolldownDeployer();

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
        rolldownDeployer.run(finalizerDeployer.avsProxyAdmin(), finalizerDeployer.avsPauserReg());

        vm.startBroadcast();
        finalizerDeployer.avsProxyAdmin().transferOwnership(finalizerDeployer.avsUpgrader());
        vm.stopBroadcast();
              

      }else if (keccak256(abi.encodePacked(variant)) == keccak256(abi.encodePacked("arbitrum-stub"))){
        console.log("################################################################################");
        console.log("Deploying ProxyAdmin");
        console.log("################################################################################");

        string memory configData = readConfig(_CONFIG_PATH);
        address avsOwner = stdJson.readAddress(configData, ".permissions.owner");
        address avsUpgrader = stdJson.readAddress(configData, ".permissions.upgrader");
        vm.startBroadcast();
        ProxyAdmin avsProxyAdmin = new ProxyAdmin();
        address[] memory pausers = new address[](1);
        pausers[0] = avsOwner;
        PauserRegistry avsPauserReg = new PauserRegistry(pausers, avsOwner);
        vm.stopBroadcast();

        console.log("################################################################################");
        console.log("Deploying rolldown contracts");
        console.log("################################################################################");
        RolldownDeployer rolldownDeployer = new RolldownDeployer();
        rolldownDeployer.run(avsProxyAdmin, avsPauserReg);

        vm.startBroadcast();
        avsProxyAdmin.transferOwnership(avsUpgrader);
        vm.stopBroadcast();
      }else{
        //TODO: ethereum-prod
        //TODO: arbitrum-prod
        //TODO: ...
        console.log("Unsupported variant", variant);
      }

    }
}

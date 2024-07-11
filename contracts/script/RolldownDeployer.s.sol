// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";

import "@eigenlayer/contracts/permissions/PauserRegistry.sol";
import "@eigenlayer/test/mocks/EmptyContract.sol";


import {Rolldown} from "../src/Rolldown.sol";

import {Utils} from "./utils/Utils.sol";

import {Gasp} from "../src/GaspToken.sol";

import "forge-std/Test.sol";
import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "forge-std/console.sol";
import {Rolldown} from "../src/Rolldown.sol";
import {IRolldownPrimitives} from "../src/Rolldown.sol";

contract RolldownDeployer is Script, Utils, Test {
    string constant _EIGEN_DEPLOYMENT_PATH = "eigenlayer_deployment_output";
    string constant _CONFIG_PATH = "deploy.config";
    string constant _OUTPUT_PATH = "rolldown_output";

    string public deployConfigPath;

    ProxyAdmin public rolldownProxyAdmin;
    PauserRegistry public rolldownPauserReg;

    Gasp public erc20Mock;
    Rolldown public rolldown;
    Rolldown public rolldownImplementation;
    address public owner;
    address public upgrader;
    address public updaterAccount;

    function evmPrefixedPath(IRolldownPrimitives.ChainId chain) public view returns (string memory) {
      string memory evm;

      if (chain == IRolldownPrimitives.ChainId.Ethereum) {
        evm = "ethereum_";
      } else if (chain == IRolldownPrimitives.ChainId.Arbitrum) {
        evm = "arbitrum_"; 
      } else {
        revert("Unsupported chain");
      }

      return string.concat(evm, _OUTPUT_PATH);
    }

    function upgrade(IRolldownPrimitives.ChainId chain) public {
      string memory configData = readInput(evmPrefixedPath(chain));
      upgrader = stdJson.readAddress(configData, ".permissions.rolldownUpgrader");
      address proxyAdmin = stdJson.readAddress(configData, ".addresses.rolldownProxyAdmin");
      address rolldownAddress = stdJson.readAddress(configData, ".addresses.rolldown");
      rolldownProxyAdmin = ProxyAdmin(proxyAdmin);

      rolldown = Rolldown(rolldownAddress);
      vm.startBroadcast();
      rolldownImplementation = new Rolldown();

      // upgrade rolldown proxy to implementation and initialize
      rolldownProxyAdmin.upgrade(
        TransparentUpgradeableProxy(payable(address(rolldown))),
        address(rolldownImplementation)
      );
      vm.stopBroadcast();

      _verifyImplementations();

      _writeOutput(chain);

    }

    function isProxyDeployed(IRolldownPrimitives.ChainId chain) public returns (bool){
      if (!inputExists(evmPrefixedPath(chain))){
        return false;
      }
      string memory configData = readInput(evmPrefixedPath(chain));
      address proxyAdmin = stdJson.readAddress(configData, ".addresses.rolldownProxyAdmin");
      return proxyAdmin.code.length > 0;
    }

    function initialDeployment(IRolldownPrimitives.ChainId chain) public {
      string memory configData = readConfig(_CONFIG_PATH);
      owner = stdJson.readAddress(configData, ".permissions.owner");
      upgrader = stdJson.readAddress(configData, ".permissions.upgrader");
      updaterAccount = stdJson.readAddress(configData, ".permissions.rolldownUpdater");


      vm.startBroadcast();
      rolldownProxyAdmin = new ProxyAdmin();
      address[] memory pausers = new address[](1);
      pausers[0] = owner;
      rolldownPauserReg = new PauserRegistry(pausers, owner);
      erc20Mock = new Gasp();

        /**
         * First, deploy upgradeable proxy contracts that **will point** to the implementations. Since the implementation contracts are
         * not yet deployed, we give these proxies an empty contract as the initial implementation, to act as if they have no code.
         */
        EmptyContract emptyContract = new EmptyContract();
        rolldown = Rolldown(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(rolldownProxyAdmin),
                    ""
                )
            )
        );
        rolldownImplementation = new Rolldown();

        // upgrade rolldown proxy to implementation and initialize
        rolldownProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(rolldown))),
            address(rolldownImplementation),
            abi.encodeWithSelector(rolldown.initialize.selector, rolldownPauserReg, owner, chain, updaterAccount)
        );


        // end deployment
        vm.stopBroadcast();

        _verifyImplementations();
        _verifyInitalizations();

        _writeOutput(chain);
    }

    function run(IRolldownPrimitives.ChainId chain) external {
      if (isProxyDeployed(chain)){
        console.log("Upgrading proxy");
        upgrade(chain);
      }else{
        console.log("Initial deployment");
        initialDeployment(chain);
      }
    }

    function _writeOutput(IRolldownPrimitives.ChainId chain) internal {
        string memory parent_object = "parent object";

        string memory deployed_addresses = "addresses";
        vm.serializeAddress(deployed_addresses, "rolldownProxyAdmin", address(rolldownProxyAdmin));
        vm.serializeAddress(deployed_addresses, "rolldownPauseReg", address(rolldownPauserReg));
        vm.serializeAddress(deployed_addresses, "rolldown", address(rolldown));
        vm.serializeAddress(deployed_addresses, "rolldownImplementation", address(rolldownImplementation));
        string memory deployed_addresses_output = vm.serializeAddress(deployed_addresses, "gaspErc20Mock", address(erc20Mock));

        string memory chain_info = "chainInfo";
        vm.serializeUint(chain_info, "deploymentBlock", block.number);
        string memory chain_info_output = vm.serializeUint(chain_info, "chainId", block.chainid);

        string memory permissions = "permissions";
        vm.serializeAddress(permissions, "rolldownOwner", owner);
        vm.serializeAddress(permissions, "rolldownUpgrader", upgrader);
        string memory permissions_output = vm.serializeAddress(permissions, "rolldownUpdater", updaterAccount);

        vm.serializeString(parent_object, chain_info, chain_info_output);
        vm.serializeString(parent_object, deployed_addresses, deployed_addresses_output);
        string memory finalJson = vm.serializeString(parent_object, permissions, permissions_output);
        console.logString(finalJson);
        writeOutput(finalJson, evmPrefixedPath(chain));
    }


    function _verifyImplementations() internal view {
        require(
            rolldownProxyAdmin.getProxyImplementation(TransparentUpgradeableProxy(payable(address(rolldown))))
                == address(rolldownImplementation),
            "rolldown: implementation set incorrectly"
        );
    }
    
    function _verifyInitalizations(
    ) internal view {
        require(rolldown.owner() == owner, "rolldown.owner() != owner");
        require(rolldown.lastProcessedUpdate_origin_l1() == 0, "rolldown.lastProcessedUpdate_origin_l1 != 0");
        require(rolldown.counter() == 1, "rolldown.counter != 1");
        require(rolldown.lastProcessedUpdate_origin_l2() == 0, "rolldown.lastProcessedUpdate_origin_l2 != 0");
        require(
            rolldown.pauserRegistry() == rolldownPauserReg,
            "rolldown: pauser registry not set correctly"
        );
        require(rolldown.paused() == 0, "rolldown: init paused status set incorrectly");
    }
}

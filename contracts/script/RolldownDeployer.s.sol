// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";

import "@eigenlayer/contracts/permissions/PauserRegistry.sol";
import "@eigenlayer/test/mocks/EmptyContract.sol";


import {Rolldown} from "../src/Rolldown.sol";

import {Utils} from "./utils/Utils.sol";

import "../src/ERC20Mock.sol";

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

    ProxyAdmin public avsProxyAdmin;
    PauserRegistry public avsPauserReg;

    ERC20Mock public erc20Mock;
    Rolldown public rolldown;
    Rolldown public rolldownImplementation;
    address public avsOwner;
    address public avsUpgrader;
    address public updaterAccount;

    function run(ProxyAdmin proxy, PauserRegistry pauser) external {
      avsPauserReg = pauser;
      avsProxyAdmin = proxy;
      string memory configData = readConfig(_CONFIG_PATH);
      avsOwner = stdJson.readAddress(configData, ".permissions.owner");
      avsUpgrader = stdJson.readAddress(configData, ".permissions.upgrader");
      updaterAccount = stdJson.readAddress(configData, ".permissions.rolldownUpdater");

      avsProxyAdmin = proxy;

        vm.startBroadcast();
        erc20Mock = new ERC20Mock();

        /**
         * First, deploy upgradeable proxy contracts that **will point** to the implementations. Since the implementation contracts are
         * not yet deployed, we give these proxies an empty contract as the initial implementation, to act as if they have no code.
         */
        EmptyContract emptyContract = new EmptyContract();
        rolldown = Rolldown(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(avsProxyAdmin),
                    ""
                )
            )
        );
        rolldownImplementation = new Rolldown();

        string memory evmId = vm.envString("EVM_ID");
        IRolldownPrimitives.ChainId chain = IRolldownPrimitives.ChainId.Ethereum;
        if (keccak256(abi.encodePacked(evmId)) == keccak256(abi.encodePacked("Arbitrum"))){
          chain = IRolldownPrimitives.ChainId.Arbitrum;
        }

        // upgrade rolldown proxy to implementation and initialize
        avsProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(rolldown))),
            address(rolldownImplementation),
            abi.encodeWithSelector(rolldown.initialize.selector, avsPauserReg, avsOwner, chain, updaterAccount)
        );


        // end deployment
        vm.stopBroadcast();

        _verifyImplementations();
        _verifyInitalizations();

        _writeOutput();
    }

    function _writeOutput() internal {
        string memory parent_object = "parent object";

        string memory deployed_addresses = "addresses";
        vm.serializeAddress(deployed_addresses, "avsProxyAdmin", address(avsProxyAdmin));
        vm.serializeAddress(deployed_addresses, "avsPauseReg", address(avsPauserReg));
        vm.serializeAddress(deployed_addresses, "rolldown", address(rolldown));
        vm.serializeAddress(deployed_addresses, "rolldownImplementation", address(rolldownImplementation));
        string memory deployed_addresses_output = vm.serializeAddress(deployed_addresses, "gaspMock", address(erc20Mock));

        string memory chain_info = "chainInfo";
        vm.serializeUint(chain_info, "deploymentBlock", block.number);
        string memory chain_info_output = vm.serializeUint(chain_info, "chainId", block.chainid);

        string memory permissions = "permissions";
        vm.serializeAddress(permissions, "avsOwner", avsOwner);
        vm.serializeAddress(permissions, "avsUpgrader", avsUpgrader);
        string memory permissions_output = vm.serializeAddress(permissions, "updaterAccount", updaterAccount);

        vm.serializeString(parent_object, chain_info, chain_info_output);
        vm.serializeString(parent_object, deployed_addresses, deployed_addresses_output);
        string memory finalJson = vm.serializeString(parent_object, permissions, permissions_output);
        console.logString(finalJson);
        writeOutput(finalJson, _OUTPUT_PATH);
    }


    function _verifyImplementations() internal view {
        require(
            avsProxyAdmin.getProxyImplementation(TransparentUpgradeableProxy(payable(address(rolldown))))
                == address(rolldownImplementation),
            "rolldown: implementation set incorrectly"
        );
    }
    
    function _verifyInitalizations(
    ) internal view {
        require(rolldown.owner() == avsOwner, "rolldown.owner() != avsOwner");
        require(rolldown.lastProcessedUpdate_origin_l1() == 0, "rolldown.lastProcessedUpdate_origin_l1 != 0");
        require(rolldown.counter() == 1, "rolldown.counter != 1");
        require(rolldown.lastProcessedUpdate_origin_l2() == 0, "rolldown.lastProcessedUpdate_origin_l2 != 0");
        require(
            rolldown.pauserRegistry() == avsPauserReg,
            "rolldown: pauser registry not set correctly"
        );
        require(rolldown.paused() == 0, "rolldown: init paused status set incorrectly");
    }
}

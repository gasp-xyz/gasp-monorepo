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

// 0xPutYourGeneratedPubKeysHere0000000000000
// # To deploy and verify our contract
// forge script script/1_FinalizerAvsDeployer.s.sol:Deployer --rpc-url $RPC_URL  --private-key $PRIVATE_KEY --broadcast -vvvv
contract Deployer is Script, Utils, Test {

    string constant _OUTPUT_PATH = "x_alt_l1_deployment_output";

    string public deployConfigPath;

    ProxyAdmin public avsProxyAdmin;

    PauserRegistry public avsPauserReg;
    address public avsOwner;
    address public avsUpgrader;
    address public user;

    ERC20Mock public erc20Mock;
    Rolldown public rolldown;
    Rolldown public rolldownImplementation;

    function run(string memory configFile) external {

        // Leaving the below here for now in case required 

        // // READ JSON CONFIG DATA
        // deployConfigPath = string(bytes(string.concat("script/", configFile)));
        // string memory config_data = vm.readFile(deployConfigPath);


        // avsOwner = stdJson.readAddress(config_data, ".avsOwner");
        // avsUpgrader = stdJson.readAddress(config_data, ".avsUpgrader");

        // START BROADCAST
        vm.startBroadcast();

        // Last two chars changed from Anvil Setup operatorAddress
        uint256 avsOwnerPrivateKey = vm.parseUint("0x113d0ef74250eab659fd828e62a33ca72fcb22948897b2ed66b1fa695a8b9300");
        avsOwner = vm.addr(avsOwnerPrivateKey);
        uint256 avsUpgraderPrivateKey = vm.parseUint("0x113d0ef74250eab659fd828e62a33ca72fcb22948897b2ed66b1fa695a8b9301");
        avsUpgrader = vm.addr(avsUpgraderPrivateKey);
        uint256 userPrivateKey = vm.parseUint("0x113d0ef74250eab659fd828e62a33ca72fcb22948897b2ed66b1fa695a8b9302");
        user = vm.addr(userPrivateKey);

        erc20Mock = new ERC20Mock();

        erc20Mock.mint(user, 100);
        user.call{value: 100 ether}("");

        // deploy proxy admin for ability to upgrade proxy contracts
        avsProxyAdmin = new ProxyAdmin();

        address unpauseMultisig = avsOwner;
        {
            address[] memory pausers = new address[](2);
            pausers[0] = avsOwner;
            pausers[1] = unpauseMultisig;
            avsPauserReg = new PauserRegistry(pausers, unpauseMultisig);
        }

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

        // upgrade rolldown proxy to implementation and initialize
        avsProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(rolldown))),
            address(rolldownImplementation),
            abi.encodeWithSelector(rolldown.initialize.selector, avsPauserReg, avsOwner)
        );

        // transfer ownership of proxy admin to upgrader
        avsProxyAdmin.transferOwnership(avsUpgrader);

        // end deployment
        vm.stopBroadcast();

        _writeOutput();
    }

    function _writeOutput() internal {
        string memory parent_object = "parent object";

        string memory deployed_addresses = "addresses";
        vm.serializeAddress(deployed_addresses, "avsProxyAdmin", address(avsProxyAdmin));
        vm.serializeAddress(deployed_addresses, "avsPauseReg", address(avsPauserReg));
        vm.serializeAddress(deployed_addresses, "rolldown", address(rolldown));
        vm.serializeAddress(deployed_addresses, "rolldownImplementation", address(rolldownImplementation));
        string memory deployed_addresses_output = vm.serializeAddress(deployed_addresses, "erc20Mock", address(erc20Mock));

        string memory chain_info = "chainInfo";
        vm.serializeUint(chain_info, "deploymentBlock", block.number);
        string memory chain_info_output = vm.serializeUint(chain_info, "chainId", block.chainid);

        string memory permissions = "permissions";
        vm.serializeAddress(permissions, "avsOwner", avsOwner);
        vm.serializeAddress(permissions, "avsUpgrader", avsUpgrader);
        string memory permissions_output = vm.serializeAddress(permissions, "user", user);

        vm.serializeString(parent_object, chain_info, chain_info_output);
        vm.serializeString(parent_object, deployed_addresses, deployed_addresses_output);
        string memory finalJson = vm.serializeString(parent_object, permissions, permissions_output);
        console.logString(finalJson);
        writeOutput(finalJson, _OUTPUT_PATH);
    }
}

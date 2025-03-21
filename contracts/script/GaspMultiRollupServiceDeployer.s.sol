// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import { PauserRegistry } from "@eigenlayer/contracts/permissions/PauserRegistry.sol";
import { EmptyContract } from "@eigenlayer/test/mocks/EmptyContract.sol";
import { ProxyAdmin, TransparentUpgradeableProxy } from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import { console } from "forge-std/console.sol";
import { Script } from "forge-std/Script.sol";
import { stdJson } from "forge-std/StdJson.sol";
import { GaspMultiRollupService } from "./../src/GaspMultiRollupService.sol";
import { IRolldownPrimitives } from "./../src/interfaces/IRolldownPrimitives.sol";
import { Utils } from "./utils/Utils.sol";

contract GaspMultiRollupServiceDeployer is Script, Utils {
    string internal constant _EIGEN_DEPLOYMENT_PATH = "eigenlayer_deployment_output";
    string internal constant _CONFIG_PATH = "deploy.config";
    string internal constant _OUTPUT_PATH = "gmrs_output";

    string public deployConfigPath;

    ProxyAdmin public gmrsProxyAdmin;
    PauserRegistry public gmrsPauserReg;
    address public dummy_rolldown_address;

    GaspMultiRollupService public gmrs;
    GaspMultiRollupService public gmrsImplementation;
    address public owner;
    address public upgrader;
    address public updaterAccount;
    bool public allowNonRootInit;

    function upgrade() public {
        string memory configData = readInput(_OUTPUT_PATH);
        upgrader = stdJson.readAddress(configData, ".permissions.gmrsUpgrader");
        address proxyAdmin = stdJson.readAddress(configData, ".addresses.gmrsProxyAdmin");
        address gmrsAddress = stdJson.readAddress(configData, ".addresses.gmrs");
        gmrsProxyAdmin = ProxyAdmin(proxyAdmin);

        gmrs = GaspMultiRollupService(gmrsAddress);
        vm.startBroadcast();
        gmrsImplementation = new GaspMultiRollupService();

        // upgrade gmrs proxy to implementation and initialize
        gmrsProxyAdmin.upgrade(TransparentUpgradeableProxy(payable(address(gmrs))), address(gmrsImplementation));
        vm.stopBroadcast();

        _verifyImplementations();

        _writeOutput();
    }

    function isProxyDeployed() public returns (bool) {
        if (!inputExists(_OUTPUT_PATH)) {
            return false;
        }
        string memory configData = readInput(_OUTPUT_PATH);
        address proxyAdmin = stdJson.readAddress(configData, ".addresses.gmrsProxyAdmin");
        return proxyAdmin.code.length > 0;
    }

    function initialDeployment(IRolldownPrimitives.ChainId chain) public {
        string memory configData = readConfig(_CONFIG_PATH);
        owner = stdJson.readAddress(configData, ".permissions.owner");
        upgrader = stdJson.readAddress(configData, ".permissions.upgrader");
        updaterAccount = stdJson.readAddress(configData, ".permissions.gmrsUpdater");
        allowNonRootInit = stdJson.readBool(configData, ".allow_non_root_gmrs_init");

        vm.startBroadcast();
        gmrsProxyAdmin = new ProxyAdmin();
        address[] memory pausers = new address[](1);
        pausers[0] = owner;
        gmrsPauserReg = new PauserRegistry(pausers, owner);

        /**
         * First, deploy upgradeable proxy contracts that **will point** to the implementations. Since the implementation contracts are
         * not yet deployed, we give these proxies an empty contract as the initial implementation, to act as if they have no code.
         */
        EmptyContract emptyContract = new EmptyContract();
        gmrs = GaspMultiRollupService(
            address(new TransparentUpgradeableProxy(address(emptyContract), address(gmrsProxyAdmin), ""))
        );
        gmrsImplementation = new GaspMultiRollupService();

        // upgrade gmrs proxy to implementation and initialize
        gmrsProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(gmrs))),
            address(gmrsImplementation),
            abi.encodeWithSelector(
                gmrs.initialize.selector,
                gmrsPauserReg,
                owner,
                updaterAccount,
                allowNonRootInit,
                dummy_rolldown_address,
                chain
            )
        );

        // end deployment
        vm.stopBroadcast();

        _verifyImplementations();
        _verifyInitalizations();

        _writeOutput();
    }

    function run(IRolldownPrimitives.ChainId chain) external {
        if (isProxyDeployed()) {
            console.log("Upgrading proxy");
            upgrade();
        } else {
            console.log("Initial deployment");
            initialDeployment(chain);
        }
    }

    function _writeOutput() internal {
        string memory parent_object = "parent object";

        string memory deployed_addresses = "addresses";
        vm.serializeAddress(deployed_addresses, "gmrsProxyAdmin", address(gmrsProxyAdmin));
        vm.serializeAddress(deployed_addresses, "gmrsPauseReg", address(gmrsPauserReg));
        vm.serializeAddress(deployed_addresses, "gmrs", address(gmrs));
        string memory deployed_addresses_output =
            vm.serializeAddress(deployed_addresses, "gmrsImplementation", address(gmrsImplementation));

        string memory chain_info = "chainInfo";
        vm.serializeUint(chain_info, "deploymentBlock", block.number);
        string memory chain_info_output = vm.serializeUint(chain_info, "chainId", block.chainid);

        string memory permissions = "permissions";
        vm.serializeAddress(permissions, "gmrsOwner", owner);
        vm.serializeAddress(permissions, "gmrsUpgrader", upgrader);
        string memory permissions_output = vm.serializeAddress(permissions, "gmrsUpdater", updaterAccount);

        vm.serializeString(parent_object, chain_info, chain_info_output);
        vm.serializeString(parent_object, deployed_addresses, deployed_addresses_output);
        string memory finalJson = vm.serializeString(parent_object, permissions, permissions_output);
        console.logString(finalJson);
        writeOutput(finalJson, _OUTPUT_PATH);
    }

    function _verifyImplementations() internal view {
        require(
            gmrsProxyAdmin.getProxyImplementation(TransparentUpgradeableProxy(payable(address(gmrs))))
                == address(gmrsImplementation),
            "gmrs: implementation set incorrectly"
        );
    }

    function _verifyInitalizations() internal view {
        require(gmrs.owner() == owner, "gmrs.owner() != owner");
        require(gmrs.pauserRegistry() == gmrsPauserReg, "gmrs: pauser registry not set correctly");
        require(gmrs.paused() == 0, "gmrs: init paused status set incorrectly");
    }
}

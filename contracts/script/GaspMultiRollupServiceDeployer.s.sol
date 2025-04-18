// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {PauserRegistry} from "@eigenlayer/contracts/permissions/PauserRegistry.sol";
import {EmptyContract} from "@eigenlayer/test/mocks/EmptyContract.sol";
import {ProxyAdmin, TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {console} from "forge-std/console.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {GaspMultiRollupService} from "../src/GaspMultiRollupService.sol";
import {IRolldownPrimitives} from "../src/interfaces/IRolldownPrimitives.sol";
import {BaseDeployer} from "./BaseDeployer.s.sol";

contract GaspMultiRollupServiceDeployer is BaseDeployer("gmrs") {
    ProxyAdmin public gmrsProxyAdmin;
    PauserRegistry public gmrsPauserReg;
    address public dummyRolldownAddress;
    GaspMultiRollupService public gmrs;
    GaspMultiRollupService public gmrsImplementation;
    address public owner;
    address public upgrader;
    address public gmrsUpdater;
    bool public allowNonRootInit;

    function deploy() public override {
        string memory configData = readConfig(_CONFIG_PATH);
        owner = stdJson.readAddress(configData, ".permissions.owner");
        upgrader = stdJson.readAddress(configData, ".permissions.upgrader");
        gmrsUpdater = stdJson.readAddress(configData, ".permissions.gmrsUpdater");
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

        gmrsProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(gmrs))),
            address(gmrsImplementation),
            abi.encodeWithSelector(
                gmrs.initialize.selector,
                gmrsPauserReg,
                owner,
                gmrsUpdater,
                allowNonRootInit,
                dummyRolldownAddress
            )
        );

        vm.stopBroadcast();

        _verifyImplementation(gmrsProxyAdmin, address(gmrs), address(gmrsImplementation));
        _verifyInitalization();
        _writeOutput();
    }

    function upgrade() public override {
        string memory configData = readInput(outputPath);
        upgrader = stdJson.readAddress(configData, ".permissions.gmrsUpgrader");
        gmrsProxyAdmin = ProxyAdmin(stdJson.readAddress(configData, ".addresses.gmrsProxyAdmin"));
        gmrs = GaspMultiRollupService(stdJson.readAddress(configData, ".addresses.gmrs"));

        vm.startBroadcast();

        gmrsImplementation = new GaspMultiRollupService();
        gmrsProxyAdmin.upgrade(TransparentUpgradeableProxy(payable(address(gmrs))), address(gmrsImplementation));

        vm.stopBroadcast();

        _verifyImplementation(gmrsProxyAdmin, address(gmrs), address(gmrsImplementation));
        _writeOutput();
    }

    function _writeOutput() internal {
        string memory parentObject = "parent object";

        string memory deployedAddresses = "addresses";
        vm.serializeAddress(deployedAddresses, "gmrsProxyAdmin", address(gmrsProxyAdmin));
        vm.serializeAddress(deployedAddresses, "gmrsPauseReg", address(gmrsPauserReg));
        vm.serializeAddress(deployedAddresses, "gmrs", address(gmrs));
        string memory deployedAddressesOutput =
            vm.serializeAddress(deployedAddresses, "gmrsImplementation", address(gmrsImplementation));

        string memory chainInfo = "chainInfo";
        vm.serializeUint(chainInfo, "deploymentBlock", block.number);
        string memory chainInfoOutput = vm.serializeUint(chainInfo, "chainId", block.chainid);

        string memory permissions = "permissions";
        vm.serializeAddress(permissions, "gmrsOwner", owner);
        vm.serializeAddress(permissions, "gmrsUpgrader", upgrader);
        string memory permissionsOutput = vm.serializeAddress(permissions, "gmrsUpdater", gmrsUpdater);

        vm.serializeString(parentObject, chainInfo, chainInfoOutput);
        vm.serializeString(parentObject, deployedAddresses, deployedAddressesOutput);

        string memory finalJson = vm.serializeString(parentObject, permissions, permissionsOutput);
        console.logString(finalJson);
        writeOutput(finalJson, outputPath);
    }

    function _verifyInitalization() internal view {
        require(gmrs.owner() == owner, "gmrs.owner() != owner");
        require(gmrs.pauserRegistry() == gmrsPauserReg, "gmrs: pauser registry not set correctly");
        require(gmrs.paused() == 0, "gmrs: init paused status set incorrectly");
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {EmptyContract} from "@eigenlayer/test/mocks/EmptyContract.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {console} from "forge-std/console.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {IRolldownPrimitives} from "../src/interfaces/IRolldownPrimitives.sol";
import {Rolldown} from "../src/Rolldown.sol";
import {GaspTestToken} from "../test/mocks/GaspTestToken.sol";
import {BaseDeployer} from "./BaseDeployer.s.sol";

contract RolldownDeployer is BaseDeployer("rolldown") {
    bytes32 public constant DEFAULT_ADMIN_ROLE = 0x00;
    bytes32 public constant UPDATER_ROLE = keccak256("UPDATER_ROLE");

    ProxyAdmin public rolldownProxyAdmin;
    GaspTestToken public erc20Mock;
    Rolldown public rolldown;
    Rolldown public rolldownImplementation;
    address public owner;
    address public upgrader;
    address public rolldownUpdater;

    function deploy(IRolldownPrimitives.ChainId chainId) public override {
        string memory configData = readConfig(_CONFIG_PATH);
        owner = stdJson.readAddress(configData, ".permissions.owner");
        upgrader = stdJson.readAddress(configData, ".permissions.upgrader");
        rolldownUpdater = stdJson.readAddress(configData, ".permissions.rolldownUpdater");

        vm.startBroadcast();

        rolldownProxyAdmin = new ProxyAdmin();
        erc20Mock = new GaspTestToken();

        EmptyContract emptyContract = new EmptyContract();
        rolldown = Rolldown(
            payable(address(new TransparentUpgradeableProxy(address(emptyContract), address(rolldownProxyAdmin), "")))
        );
        rolldownImplementation = new Rolldown();

        rolldownProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(rolldown))),
            address(rolldownImplementation),
            abi.encodeCall(rolldown.initialize, (owner, chainId, rolldownUpdater))
        );

        vm.stopBroadcast();

        _verifyImplementation(rolldownProxyAdmin, address(rolldown), address(rolldownImplementation));
        _verifyInitalization();
        _writeOutput();
    }

    function upgrade() public override {
        string memory configData = readInput(outputPath);
        upgrader = stdJson.readAddress(configData, ".permissions.rolldownUpgrader");
        rolldownProxyAdmin = ProxyAdmin(stdJson.readAddress(configData, ".addresses.rolldownProxyAdmin"));
        rolldown = Rolldown(payable(stdJson.readAddress(configData, ".addresses.rolldown")));

        vm.startBroadcast();

        rolldownImplementation = new Rolldown();
        rolldownProxyAdmin.upgrade(
            TransparentUpgradeableProxy(payable(address(rolldown))), address(rolldownImplementation)
        );

        vm.stopBroadcast();

        _verifyImplementation(rolldownProxyAdmin, address(rolldown), address(rolldownImplementation));
        _writeOutput();
    }

    function _writeOutput() internal {
        string memory parentObject = "parent object";

        string memory deployedAddresses = "addresses";
        vm.serializeAddress(deployedAddresses, "rolldownProxyAdmin", address(rolldownProxyAdmin));
        vm.serializeAddress(deployedAddresses, "rolldown", address(rolldown));
        vm.serializeAddress(deployedAddresses, "rolldownImplementation", address(rolldownImplementation));
        string memory deployedAddressesOutput =
            vm.serializeAddress(deployedAddresses, "gaspErc20Mock", address(erc20Mock));

        string memory chainInfo = "chainInfo";
        vm.serializeUint(chainInfo, "deploymentBlock", block.number);
        string memory chainInfoOutput = vm.serializeUint(chainInfo, "chainId", block.chainid);

        string memory permissions = "permissions";
        vm.serializeAddress(permissions, "rolldownOwner", owner);
        vm.serializeAddress(permissions, "rolldownUpgrader", upgrader);
        string memory permissionsOutput = vm.serializeAddress(permissions, "rolldownUpdater", rolldownUpdater);

        vm.serializeString(parentObject, chainInfo, chainInfoOutput);
        vm.serializeString(parentObject, deployedAddresses, deployedAddressesOutput);

        string memory finalJson = vm.serializeString(parentObject, permissions, permissionsOutput);
        console.logString(finalJson);
        writeOutput(finalJson, outputPath);
    }

    function _verifyInitalization() internal view {
        require(rolldown.hasRole(DEFAULT_ADMIN_ROLE, owner), "rolldown.hasRole(DEFAULT_ADMIN_ROLE) != owner");
        require(rolldown.hasRole(UPDATER_ROLE, rolldownUpdater), "rolldown.hasRole(UPDATER_ROLE) != updater");
        require(!rolldown.paused(), "rolldown: paused status set incorrectly");
        require(rolldown.counter() == 1, "rolldown.counter != 1");
        require(rolldown.lastProcessedUpdate_origin_l1() == 0, "rolldown.lastProcessedUpdate_origin_l1 != 0");
        require(rolldown.lastProcessedUpdate_origin_l2() == 0, "rolldown.lastProcessedUpdate_origin_l2 != 0");
    }
}

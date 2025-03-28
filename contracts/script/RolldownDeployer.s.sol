// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {EmptyContract} from "@eigenlayer/test/mocks/EmptyContract.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {console} from "forge-std/console.sol";
import {Script} from "forge-std/Script.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {IRolldownPrimitives} from "../src/interfaces/IRolldownPrimitives.sol";
import {Rolldown} from "../src/Rolldown.sol";
import {GaspTestToken} from "../test/mocks/GaspTestToken.sol";
import {Utils} from "./utils/Utils.sol";

contract RolldownDeployer is Script, Utils {
    bytes32 public constant DEFAULT_ADMIN_ROLE = 0x00;
    bytes32 public constant UPDATER_ROLE = keccak256("UPDATER_ROLE");
    string private constant _EIGEN_DEPLOYMENT_PATH = "eigenlayer_deployment_output";
    string private constant _CONFIG_PATH = "deploy.config";
    string private constant _OUTPUT_PATH = "rolldown_output";

    string public deployConfigPath;
    ProxyAdmin public rolldownProxyAdmin;
    GaspTestToken public erc20Mock;
    Rolldown public rolldown;
    Rolldown public rolldownImplementation;
    address public owner;
    address public upgrader;
    address public updaterAccount;

    function run(IRolldownPrimitives.ChainId chain) external {
        if (isProxyDeployed()) {
            console.log("Upgrading proxy");
            upgrade();
        } else {
            console.log("Initial deployment");
            initialDeployment(chain);
        }
    }

    function isProxyDeployed() public returns (bool) {
        if (!inputExists(_OUTPUT_PATH)) {
            return false;
        }

        string memory configData = readInput(_OUTPUT_PATH);
        address proxyAdmin = stdJson.readAddress(configData, ".addresses.rolldownProxyAdmin");

        return proxyAdmin.code.length > 0;
    }

    function upgrade() public {
        string memory configData = readInput(_OUTPUT_PATH);
        upgrader = stdJson.readAddress(configData, ".permissions.rolldownUpgrader");
        address proxyAdmin = stdJson.readAddress(configData, ".addresses.rolldownProxyAdmin");
        address payable rolldownAddress = payable(stdJson.readAddress(configData, ".addresses.rolldown"));

        rolldownProxyAdmin = ProxyAdmin(proxyAdmin);
        rolldown = Rolldown(rolldownAddress);

        vm.startBroadcast();

        rolldownImplementation = new Rolldown();
        rolldownProxyAdmin.upgrade(
            TransparentUpgradeableProxy(payable(address(rolldown))), address(rolldownImplementation)
        );

        vm.stopBroadcast();

        _verifyImplementations();
        _writeOutput();
    }

    function initialDeployment(IRolldownPrimitives.ChainId chain) public {
        string memory configData = readConfig(_CONFIG_PATH);
        owner = stdJson.readAddress(configData, ".permissions.owner");
        upgrader = stdJson.readAddress(configData, ".permissions.upgrader");
        updaterAccount = stdJson.readAddress(configData, ".permissions.rolldownUpdater");

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
            abi.encodeCall(rolldown.initialize, (owner, chain, updaterAccount))
        );

        vm.stopBroadcast();

        _verifyImplementations();
        _verifyInitalizations();
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
        string memory permissionsOutput = vm.serializeAddress(permissions, "rolldownUpdater", updaterAccount);

        vm.serializeString(parentObject, chainInfo, chainInfoOutput);
        vm.serializeString(parentObject, deployedAddresses, deployedAddressesOutput);

        string memory finalJson = vm.serializeString(parentObject, permissions, permissionsOutput);
        console.logString(finalJson);
        writeOutput(finalJson, _OUTPUT_PATH);
    }

    function _verifyImplementations() internal view {
        require(
            rolldownProxyAdmin.getProxyImplementation(TransparentUpgradeableProxy(payable(address(rolldown))))
                == address(rolldownImplementation),
            "rolldown: implementation set incorrectly"
        );
    }

    function _verifyInitalizations() internal view {
        require(rolldown.hasRole(DEFAULT_ADMIN_ROLE, owner), "rolldown.hasRole(DEFAULT_ADMIN_ROLE) != owner");
        require(rolldown.hasRole(UPDATER_ROLE, updaterAccount), "rolldown.hasRole(UPDATER_ROLE) != updater");
        require(!rolldown.paused(), "rolldown: paused status set incorrectly");
        require(rolldown.counter() == 1, "rolldown.counter != 1");
        require(rolldown.lastProcessedUpdate_origin_l1() == 0, "rolldown.lastProcessedUpdate_origin_l1 != 0");
        require(rolldown.lastProcessedUpdate_origin_l2() == 0, "rolldown.lastProcessedUpdate_origin_l2 != 0");
    }
}

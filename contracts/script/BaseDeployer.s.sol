// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {ProxyAdmin, TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {console} from "forge-std/console.sol";
import {Script} from "forge-std/Script.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {IRolldownPrimitives} from "../src/interfaces/IRolldownPrimitives.sol";
import {Utils} from "./utils/Utils.sol";

abstract contract BaseDeployer is Script, Utils {
    string internal constant _EIGEN_DEPLOYMENT_PATH = "eigenlayer_deployment_output";
    string internal constant _CONFIG_PATH = "deploy.config";

    string public deployerKind;
    string public outputPath;

    constructor(string memory deployerKind_) {
        deployerKind = deployerKind_;
        outputPath = string.concat(deployerKind_, "_output");
    }

    function run(string memory chainName) external {
        if (isProxyDeployed()) {
            _printMessage(string.concat("Upgrading ", deployerKind, " contracts on ", chainName));
            upgrade();
            return;
        }

        _printMessage(string.concat("Deploying ", deployerKind, " contracts to ", chainName));
        deploy();
    }

    function isProxyDeployed() public returns (bool) {
        if (!inputExists(outputPath)) {
            return false;
        }

        string memory configData = readInput(outputPath);
        address proxyAdmin = stdJson.readAddress(configData, string.concat(".addresses.", deployerKind, "ProxyAdmin"));
        return proxyAdmin.code.length > 0;
    }

    function deploy() public virtual;

    function upgrade() public virtual;

    function _verifyImplementation(ProxyAdmin proxyAdmin, address proxy, address implementation)
        internal
        view
        virtual
    {
        require(
            proxyAdmin.getProxyImplementation(TransparentUpgradeableProxy(payable(proxy))) == implementation,
            string.concat(deployerKind, ": implementation set incorrectly")
        );
    }

    function _printMessage(string memory message) private pure {
        console.log("################################################################################");
        console.log(message);
        console.log("################################################################################");
    }
}

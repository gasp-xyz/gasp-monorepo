// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {DelegationManager, IDelegationManager} from "@eigenlayer/contracts/core/DelegationManager.sol";
import {StrategyManager} from "@eigenlayer/contracts/core/StrategyManager.sol";
import {PauserRegistry} from "@eigenlayer/contracts/permissions/PauserRegistry.sol";
import {IStrategy, StrategyBaseTVLLimits} from "@eigenlayer/contracts/strategies/StrategyBaseTVLLimits.sol";
import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {ProxyAdmin, TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {Script} from "forge-std/Script.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {Test} from "forge-std/Test.sol";
import {ERC20Mock} from "../test/mocks/ERC20Mock.sol";
import {Utils} from "./utils/Utils.sol";

// # To deploy strategy and setup deposits on Anvil
// forge script script/0_AnvilSetup.s.sol:AnvilSetup --rpc-url http://localhost:8545 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --broadcast -vvvv
//
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Configures strategy(token) for the eigen layer. This is only needed in the
// environment where we are in charge of base eigen layer setup/contracts.
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
contract AnvilSetup is Script, Test, Utils {
    uint256 public constant ANVIL_CHAIN_ID = 31337;
    uint256 public constant RETH_CHAIN_ID = 1337;
    string internal constant _EIGEN_DEPLOYMENT_PATH = "eigenlayer_deployment_output";
    string internal constant _CONFIG_PATH = "deploy.config";
    string internal constant _OUTPUT_PATH = "strategy_output";

    // ERC20 and Strategy: we need to deploy this erc20, create a strategy for it, and whitelist this strategy in the strategymanager
    ERC20Mock public erc20Mock;
    StrategyBaseTVLLimits public erc20MockStrategy;

    function run() external {
        // Eigenlayer contracts
        string memory eigenlayerDeployedContracts = readInput(_EIGEN_DEPLOYMENT_PATH);
        StrategyManager strategyManager =
            StrategyManager(stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.strategyManager"));
        DelegationManager delegation =
            DelegationManager(stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.delegationManager"));

        ProxyAdmin eigenLayerProxyAdmin =
            ProxyAdmin(stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.eigenLayerProxyAdmin"));
        StrategyBaseTVLLimits baseStrategyImplementation = StrategyBaseTVLLimits(
            stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.baseStrategyImplementation")
        );
        PauserRegistry eigenLayerPauserReg =
            PauserRegistry(stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.eigenLayerPauserReg"));

        string memory configData = readConfig(_CONFIG_PATH);

        // check that the chainID matches the one in the config
        uint256 currentChainId = block.chainid;
        uint256 configChainId = stdJson.readUint(configData, ".chainInfo.chainId");
        emit log_named_uint("You are deploying on ChainID", currentChainId);
        require(
            configChainId == RETH_CHAIN_ID || configChainId == ANVIL_CHAIN_ID,
            "You are on the wrong chain for this config, only Anvil 31337 allowed"
        );
        require(configChainId == currentChainId, "You are on the wrong chain for this config");

        // tests/keys/test.ecdsa.key.json
        uint256 operatorPrivateKey = vm.parseUint("0x113d0ef74250eab659fd828e62a33ca72fcb22948897b2ed66b1fa695a8b9313");
        address operatorAddress = vm.addr(operatorPrivateKey);

        vm.startBroadcast();

        erc20Mock = new ERC20Mock();

        // make sure you update config/{chain_id}/deploy.config.json with correct strategy address from the output of this script
        erc20MockStrategy = StrategyBaseTVLLimits(
            address(
                new TransparentUpgradeableProxy(
                    address(baseStrategyImplementation),
                    address(eigenLayerProxyAdmin),
                    abi.encodeWithSelector(
                        StrategyBaseTVLLimits.initialize.selector,
                        1 ether, // maxPerDeposit
                        100 ether, // maxDeposits
                        IERC20(erc20Mock),
                        eigenLayerPauserReg
                    )
                )
            )
        );
        IStrategy[] memory strats = new IStrategy[](1);
        bool[] memory thirdPartyTransfersForbidden = new bool[](1);
        thirdPartyTransfersForbidden[0] = false;
        strats[0] = erc20MockStrategy;
        strategyManager.addStrategiesToDepositWhitelist(strats, thirdPartyTransfersForbidden);

        erc20Mock.mint(operatorAddress, 100);
        (bool success,) = operatorAddress.call{value: 100 ether}("");
        if (!success) {
            revert("Failed to transfer to operator");
        }

        vm.stopBroadcast();

        // setup deposit for tests/keys/test.ecdsa.key.json
        vm.startBroadcast(operatorPrivateKey);
        erc20Mock.approve(address(strategyManager), 100);
        strategyManager.depositIntoStrategy(erc20MockStrategy, erc20Mock, 100);
        IDelegationManager.OperatorDetails memory op =
            IDelegationManager.OperatorDetails(operatorAddress, address(0), 0);
        delegation.registerAsOperator(op, "");
        vm.stopBroadcast();

        _writeOutput();
    }

    function _writeOutput() internal {
        string memory parentObject = "parent object";

        string memory deployedAddresses = "addresses";
        vm.serializeAddress(deployedAddresses, "erc20Mock", address(erc20Mock));
        string memory deployed_addresses_output =
            vm.serializeAddress(deployedAddresses, "erc20MockStrategy", address(erc20MockStrategy));

        string memory chainInfo = "chainInfo";
        vm.serializeUint(chainInfo, "deploymentBlock", block.number);
        string memory chain_info_output = vm.serializeUint(chainInfo, "chainId", block.chainid);

        vm.serializeString(parentObject, chainInfo, chain_info_output);
        string memory finalJson = vm.serializeString(parentObject, deployedAddresses, deployed_addresses_output);
        writeOutput(finalJson, _OUTPUT_PATH);
    }
}

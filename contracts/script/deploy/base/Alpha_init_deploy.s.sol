// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {PauserRegistry} from "@eigenlayer/contracts/permissions/PauserRegistry.sol";
import {EmptyContract} from "@eigenlayer/test/mocks/EmptyContract.sol";
import {ProxyAdmin, TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {console} from "forge-std/console.sol";
import {Script} from "forge-std/Script.sol";
import {Test} from "forge-std/Test.sol";
import {GaspMultiRollupService} from "../../../src/GaspMultiRollupService.sol";
import {IRolldownPrimitives} from "../../../src/interfaces/IRolldownPrimitives.sol";
import {Rolldown} from "../../../src/Rolldown.sol";
import {Utils} from "../../utils/Utils.sol";

// # To deploy and verify our contract
// forge script script/Alpha_init_deploy.s.sol:Deployer --rpc-url $RPC_URL  --private-key $PRIVATE_KEY --broadcast -vvvv

// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Deploys finalizer contracts
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
contract Deployer is Script, Utils, Test {
    string internal constant _OUTPUT_PATH = "base_deployment_output";

    ProxyAdmin public avsProxyAdmin;
    PauserRegistry public avsPauserReg;
    address public avsOwner;
    address public avsUpdater;

    //upgradeable contracts
    GaspMultiRollupService public gmrs;
    Rolldown public rolldown;

    //upgradeable contract implementations
    GaspMultiRollupService public gmrsImplementation;
    Rolldown public rolldownImplementation;

    function run() external {
        avsOwner = vm.parseAddress("0xFc8151F19e338C509D1F5B8c981249324014b6B7");
        avsUpdater = vm.parseAddress("0x74AFD71912532AFeB74B752dA70fF939D9Be7BB8");

        require(block.chainid == 8453, "You are on the wrong chain");

        // START BROADCAST
        vm.startBroadcast();

        // deploy proxy admin for ability to upgrade proxy contracts
        avsProxyAdmin = new ProxyAdmin();

        // deploy pauser registry
        address unpauseMultisig = avsOwner;
        {
            address[] memory pausers = new address[](1);
            pausers[0] = avsOwner;
            avsPauserReg = new PauserRegistry(pausers, unpauseMultisig);
        }

        /**
         * First, deploy upgradeable proxy contracts that **will point** to the implementations. Since the implementation contracts are
         * not yet deployed, we give these proxies an empty contract as the initial implementation, to act as if they have no code.
         */
        EmptyContract emptyContract = new EmptyContract();

        rolldown = Rolldown(
            payable(address(new TransparentUpgradeableProxy(address(emptyContract), address(avsProxyAdmin), "")))
        );
        gmrs = GaspMultiRollupService(
            address(new TransparentUpgradeableProxy(address(emptyContract), address(avsProxyAdmin), ""))
        );

        rolldownImplementation = new Rolldown();
        avsProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(rolldown))),
            address(rolldownImplementation),
            abi.encodeCall(rolldown.initialize, (avsOwner, address(gmrs)))
        );

        gmrsImplementation = new GaspMultiRollupService();
        avsProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(gmrs))),
            address(gmrsImplementation),
            abi.encodeWithSelector(
                gmrs.initialize.selector, avsPauserReg, avsOwner, avsUpdater, false, address(rolldown)
            )
        );
        // transfer ownership of proxy admin to upgrader
        avsProxyAdmin.transferOwnership(avsOwner);

        // end deployment
        vm.stopBroadcast();

        // sanity checks
        _verifyContractPointers();

        _verifyImplementations();
        _verifyInitalizations();

        _checkPauserInitializations();

        //write output
        _writeOutput();
    }

    function _verifyContractPointers() internal view {
        require(rolldown.updaterAccount() == address(gmrs), "rolldown.updaterAccount() != gmrs");
        require(gmrs.rolldown() == rolldown, "gmrs.rolldown() != rolldown");
    }

    function _verifyImplementations() internal view {
        require(
            avsProxyAdmin.getProxyImplementation(TransparentUpgradeableProxy(payable(address(rolldown))))
                == address(rolldownImplementation),
            "rolldown: implementation set incorrectly"
        );
        require(
            avsProxyAdmin.getProxyImplementation(TransparentUpgradeableProxy(payable(address(gmrs))))
                == address(gmrsImplementation),
            "grmrs: implementation set incorrectly"
        );
    }

    function _verifyInitalizations() internal view {
        require(rolldown.hasRole(0x00, avsOwner), "rolldown default role admin != avsOwner");
        require(gmrs.owner() == avsOwner, "gmrs.owner() != avsOwner");
        require(gmrs.updater() == avsUpdater, "gmrs.updater() != avsUpdater");
    }

    function _checkPauserInitializations() internal view {
        require(gmrs.pauserRegistry() == avsPauserReg, "gmrs: pauser registry not set correctly");

        require(avsPauserReg.isPauser(avsOwner), "pauserRegistry: avsOwner is not pauser");
        require(avsPauserReg.unpauser() == avsOwner, "pauserRegistry: unpauser not set correctly");

        require(rolldown.paused() == false, "rolldown: init paused status set incorrectly");
        require(gmrs.paused() == 0, "gmrs: init paused status set incorrectly");
    }

    function _writeOutput() internal {
        string memory parent_object = "parent object";

        string memory deployed_addresses = "addresses";
        vm.serializeAddress(deployed_addresses, "avsProxyAdmin", address(avsProxyAdmin));
        vm.serializeAddress(deployed_addresses, "avsPauseReg", address(avsPauserReg));
        vm.serializeAddress(deployed_addresses, "gmrs", address(gmrs));
        vm.serializeAddress(deployed_addresses, "gmrsImplementation", address(gmrsImplementation));
        vm.serializeAddress(deployed_addresses, "rolldown", address(rolldown));
        string memory deployed_addresses_output =
            vm.serializeAddress(deployed_addresses, "rolldownImplementation", address(rolldownImplementation));

        string memory chain_info = "chainInfo";
        vm.serializeUint(chain_info, "deploymentBlock", block.number);
        string memory chain_info_output = vm.serializeUint(chain_info, "chainId", block.chainid);

        string memory permissions = "permissions";
        vm.serializeAddress(permissions, "avsOwner", avsOwner);
        string memory permissions_output = vm.serializeAddress(permissions, "avsUpdater", avsUpdater);

        vm.serializeString(parent_object, chain_info, chain_info_output);
        vm.serializeString(parent_object, deployed_addresses, deployed_addresses_output);
        string memory finalJson = vm.serializeString(parent_object, permissions, permissions_output);
        console.logString(finalJson);
        writeOutput(finalJson, _OUTPUT_PATH);
    }

    function getOutputPath() external pure returns (string memory) {
        return _OUTPUT_PATH;
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {IStakeRegistry} from "@eigenlayer-middleware/src/BLSSignatureChecker.sol";
import {IRegistryCoordinator} from "@eigenlayer-middleware/src/RegistryCoordinator.sol";
import {IAVSDirectory} from "@eigenlayer/contracts/core/AVSDirectory.sol";
import {IRewardsCoordinator} from "@eigenlayer/contracts/interfaces/IRewardsCoordinator.sol";
import {ISignatureUtils} from "@eigenlayer/contracts/interfaces/ISignatureUtils.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {Test} from "forge-std/Test.sol";
import {FinalizerServiceManager} from "./../src/FinalizerServiceManager.sol";
import {IFinalizerTaskManager} from "./../src/interfaces/IFinalizerTaskManager.sol";

contract FinalizerServiceManagerTest is Test {
    FinalizerServiceManager internal serviceManager;
    IAVSDirectory internal avsDirectory;
    IRewardsCoordinator internal rewardsCoordinator;
    IRegistryCoordinator internal registryCoordinator;
    IStakeRegistry internal stakeRegistry;
    IFinalizerTaskManager internal taskManager;
    address internal owner;
    address internal ejector;
    address internal operator;
    ISignatureUtils.SignatureWithSaltAndExpiry internal operatorSignature;

    uint64 internal constant LIMIT = 100;

    function setUp() public {
        owner = makeAddr("owner");
        ejector = makeAddr("ejector");
        operator = makeAddr("operator");

        avsDirectory = IAVSDirectory(makeAddr("avsDirectory"));
        rewardsCoordinator = IRewardsCoordinator(makeAddr("rewardsCoordinator"));
        registryCoordinator = IRegistryCoordinator(makeAddr("registryCoordinator"));
        stakeRegistry = IStakeRegistry(makeAddr("stakeRegistry"));
        taskManager = IFinalizerTaskManager(makeAddr("taskManager"));

        FinalizerServiceManager impl = new FinalizerServiceManager(
            avsDirectory, rewardsCoordinator, registryCoordinator, stakeRegistry, taskManager, LIMIT
        );
        ProxyAdmin proxyAdmin = new ProxyAdmin();
        TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(
            address(impl),
            address(proxyAdmin),
            abi.encodeCall(FinalizerServiceManager.initialize, (owner, makeAddr("rewardsInitiator"), ejector))
        );
        serviceManager = FinalizerServiceManager(payable(address(proxy)));
    }

    function test_RegisterOperatorToAVS() public {
        vm.roll(200);
        bytes32 operatorId = keccak256(abi.encodePacked(operator));
        vm.mockCall(
            address(registryCoordinator),
            abi.encodeWithSelector(IRegistryCoordinator.getOperatorId.selector, operator),
            abi.encode(operatorId)
        );
        vm.mockCall(
            address(registryCoordinator),
            abi.encodeWithSelector(IRegistryCoordinator.quorumCount.selector),
            abi.encode(uint8(1))
        );
        vm.mockCall(
            address(avsDirectory),
            abi.encodeWithSelector(IAVSDirectory.registerOperatorToAVS.selector, operator, operatorSignature),
            abi.encode()
        );

        IStakeRegistry.StakeUpdate memory stakeUpdate =
            IStakeRegistry.StakeUpdate({updateBlockNumber: uint32(block.number), nextUpdateBlockNumber: 0, stake: 0});
        vm.mockCall(
            address(stakeRegistry),
            abi.encodeWithSelector(IStakeRegistry.getLatestStakeUpdate.selector, operatorId, uint8(0)),
            abi.encode(stakeUpdate)
        );

        vm.expectRevert(
            "ServiceManager.registerOperatorToAVS: minimum blocks elapsed limit for recurrent registration not met"
        );
        vm.prank(address(registryCoordinator));
        serviceManager.registerOperatorToAVS(operator, operatorSignature);

        stakeUpdate = IStakeRegistry.StakeUpdate({
            updateBlockNumber: uint32(block.number - LIMIT - 1),
            nextUpdateBlockNumber: 0,
            stake: 0
        });
        vm.mockCall(
            address(stakeRegistry),
            abi.encodeWithSelector(IStakeRegistry.getLatestStakeUpdate.selector, operatorId, uint8(0)),
            abi.encode(stakeUpdate)
        );

        vm.expectCall(address(avsDirectory), abi.encodeWithSelector(IAVSDirectory.registerOperatorToAVS.selector));
        vm.prank(address(registryCoordinator));
        serviceManager.registerOperatorToAVS(operator, operatorSignature);
    }

    function test_EjectOperators() public {
        address[] memory operators = new address[](2);
        operators[0] = operator;
        operators[1] = makeAddr("operator2");
        bytes[] memory quorumNumbers = new bytes[](2);
        quorumNumbers[0] = abi.encodePacked(uint8(1));
        quorumNumbers[1] = abi.encodePacked(uint8(1));

        vm.mockCall(
            address(registryCoordinator),
            abi.encodeWithSelector(IRegistryCoordinator.ejectOperator.selector),
            abi.encode()
        );
        vm.expectCall(
            address(registryCoordinator), abi.encodeWithSelector(IRegistryCoordinator.ejectOperator.selector), 2
        );

        vm.prank(ejector);
        serviceManager.ejectOperators(operators, quorumNumbers);
    }

    function test_SetEjector() public {
        address newEjector = makeAddr("newEjector");

        vm.prank(owner);
        serviceManager.setEjector(newEjector);

        assertEq(serviceManager.ejector(), newEjector);
    }

    function test_RevertIf_NotOwner_SetEjector() public {
        address newEjector = makeAddr("newEjector");

        vm.prank(operator);
        vm.expectRevert("Ownable: caller is not the owner");
        serviceManager.setEjector(newEjector);
    }
}

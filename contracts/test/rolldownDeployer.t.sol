// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {EmptyContract} from "@eigenlayer/test/mocks/EmptyContract.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {stdStorage, StdStorage, Test} from "forge-std/Test.sol";
import {RolldownDeployer} from "./../script/RolldownDeployer.s.sol";
import {IRolldownPrimitives} from "./../src/IRolldownPrimitives.sol";
import {Rolldown} from "./../src/Rolldown.sol";
import {RollDownUpg} from "./utils/RollDownUpg.sol";
import {Utilities, MyERC20} from "./utils/Utilities.sol";

contract RolldownDeployerTest is Test {
    using stdStorage for StdStorage;

    bytes32 public constant DEFAULT_ADMIN_ROLE = 0x00;
    bytes32 public constant UPDATER_ROLE = keccak256("UPDATER_ROLE");

    RolldownDeployer public rolldownDeployer;
    Utilities public utils;
    address payable[] public users;
    address public owner;
    address public upgrader;
    address public updaterAccount;
    MyERC20 public token;
    address payable public NATIVE_TOKEN_ADDRESS;
    ProxyAdmin public rolldownProxyAdmin;
    Rolldown public rolldown;
    Rolldown public rolldownImplementation;

    function setUp() public {
        utils = new Utilities();
        users = utils.createUsers(3);
        owner = users[0];
        upgrader = users[1];
        updaterAccount = users[2];

        deal(owner, 100 ether);

        vm.startBroadcast(owner);

        rolldownProxyAdmin = new ProxyAdmin();

        EmptyContract emptyContract = new EmptyContract();
        rolldown =
            Rolldown(payable(address(new TransparentUpgradeableProxy(address(emptyContract), address(rolldownProxyAdmin), ""))));

        vm.stopBroadcast();
    }

    function testRolldownFromZeroToInitializedByUpgrade() public {
        vm.startBroadcast(owner);

        rolldownImplementation = new Rolldown();
        rolldownProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(rolldown))),
            address(rolldownImplementation),
            abi.encodeWithSelector(
                rolldown.initialize.selector, owner, IRolldownPrimitives.ChainId.Ethereum, updaterAccount
            )
        );

        vm.stopBroadcast();

        Rolldown rolldown2 = Rolldown(payable(address(rolldown)));
        assertTrue(rolldown2.hasRole(DEFAULT_ADMIN_ROLE, owner));
        assertTrue(rolldown2.hasRole(UPDATER_ROLE, updaterAccount));
    }

    function testRolldownFromInitializeReinitialize() public {
        vm.startBroadcast(owner);

        rolldownImplementation = new Rolldown();
        rolldownProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(rolldown))),
            address(rolldownImplementation),
            abi.encodeWithSelector(
                rolldown.initialize.selector, owner, IRolldownPrimitives.ChainId.Ethereum, updaterAccount
            )
        );

        rolldownImplementation = new Rolldown();
        vm.expectRevert("Initializable: contract is already initialized");
        rolldownProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(rolldown))),
            address(rolldownImplementation),
            abi.encodeWithSelector(
                rolldown.initialize.selector, owner, IRolldownPrimitives.ChainId.Ethereum, updaterAccount
            )
        );

        vm.stopBroadcast();

        Rolldown rolldown2 = Rolldown(payable(address(rolldown)));
        assertTrue(rolldown2.hasRole(DEFAULT_ADMIN_ROLE, owner));
        assertTrue(rolldown2.hasRole(UPDATER_ROLE, updaterAccount));
    }

    function testRolldownFromInitializedtoUpdated() public {
        address payable[] memory users2 = utils.createUsers(1);
        address notOwner = users2[0];
        deal(notOwner, 100 ether);

        vm.startBroadcast(owner);

        rolldownImplementation = new Rolldown();
        rolldownProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(rolldown))),
            address(rolldownImplementation),
            abi.encodeWithSelector(
                rolldown.initialize.selector, owner, IRolldownPrimitives.ChainId.Ethereum, updaterAccount
            )
        );

        vm.stopBroadcast();

        RollDownUpg rolldown2 = RollDownUpg(payable(address(rolldown)));
        vm.expectRevert();
        bool res = rolldown2.imUpgraded();
        Rolldown rd2 = new RollDownUpg();

        vm.startBroadcast(owner);

        rolldownImplementation = rd2;
        rolldownProxyAdmin.upgrade(
            TransparentUpgradeableProxy(payable(address(rolldown))), address(rolldownImplementation)
        );

        vm.stopBroadcast();

        rolldown2 = RollDownUpg(payable(address(rolldown)));
        res = rolldown2.imUpgraded();
        assertTrue(res);
    }

    function testRolldownFromInitializedtoUpdatedNotOwner() public {
        address payable[] memory users2 = utils.createUsers(1);
        address notOwner = users2[0];
        deal(notOwner, 100 ether);

        vm.startBroadcast(owner);

        rolldownImplementation = new Rolldown();
        rolldownProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(rolldown))),
            address(rolldownImplementation),
            abi.encodeWithSelector(
                rolldown.initialize.selector, owner, IRolldownPrimitives.ChainId.Ethereum, updaterAccount
            )
        );

        vm.stopBroadcast();

        RollDownUpg rolldown2 = RollDownUpg(payable(address(rolldown)));
        vm.expectRevert();
        rolldown2.imUpgraded();

        Rolldown rd2 = new RollDownUpg();
        vm.startBroadcast(notOwner);

        rolldownImplementation = rd2;
        vm.expectRevert("Ownable: caller is not the owner");
        rolldownProxyAdmin.upgrade(
            TransparentUpgradeableProxy(payable(address(rolldown))), address(rolldownImplementation)
        );

        vm.stopBroadcast();
    }
}

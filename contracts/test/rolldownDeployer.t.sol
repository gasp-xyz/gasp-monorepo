pragma solidity ^0.8.9;
import {RolldownDeployer} from "../script/RolldownDeployer.s.sol";
import {stdStorage, StdStorage, Test} from "forge-std/Test.sol";
import "forge-std/console.sol";
import "forge-std/StdJson.sol";
import {Utilities, MyERC20} from "./utils/Utilities.sol";
import {IRolldownPrimitives} from "../src/IRolldownPrimitives.sol";
import {Rolldown} from "../src/Rolldown.sol";
import "@eigenlayer/contracts/permissions/PauserRegistry.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import "@eigenlayer/test/mocks/EmptyContract.sol";

import {Gasp} from "../src/GaspToken.sol";
import {RollDownUpg} from "./utils/RollDownUpg.sol" ;

contract RolldownDeployerTest is Test {
    RolldownDeployer rolldownDeployer;
    using stdStorage for StdStorage;
    Utilities internal utils;
    address payable[] internal users;
    address owner;
    address upgrader;
    address updaterAccount;

    MyERC20 internal token;
    address payable internal ETH_TOKEN_ADDRESS;

    ProxyAdmin public rolldownProxyAdmin;
    PauserRegistry public rolldownPauserReg;
    Rolldown public rolldown;
    Rolldown public rolldownImplementation;

    function setUp() public {
        address payable[] memory admins;

        utils = new Utilities();
        users = utils.createUsers(3);
        owner = users[0];
        upgrader = users[1];
        updaterAccount = users[2];

    
        deal(owner, 100 ether);
        vm.startBroadcast(owner);

        rolldownProxyAdmin = new ProxyAdmin();
        address[] memory pausers = new address[](1);
        pausers[0] = owner;
        rolldownPauserReg = new PauserRegistry(pausers, owner);
      
        EmptyContract emptyContract = new EmptyContract();
        rolldown = Rolldown(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(rolldownProxyAdmin),
                    ""
                )
            )
        );
        // end deployment
        vm.stopBroadcast();
    }

    function testRolldownFromZeroToInitializedByUpgrade() public {
        
        vm.startBroadcast(owner);
            rolldownImplementation = new Rolldown();
            rolldownProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(rolldown))),
            address(rolldownImplementation),
            abi.encodeWithSelector(rolldown.initialize.selector, rolldownPauserReg, owner, IRolldownPrimitives.ChainId.Ethereum, updaterAccount)
        );
        vm.stopBroadcast();

        Rolldown rolldown2 = Rolldown(address(rolldown));
        address ownerFromCt = rolldown2.owner();
        assertEq(owner, ownerFromCt);

    }
    function testRolldownFromInitializeReInitialize() public {
        
        vm.startBroadcast(owner);
            rolldownImplementation = new Rolldown();
            rolldownProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(rolldown))),
            address(rolldownImplementation),
            abi.encodeWithSelector(rolldown.initialize.selector, rolldownPauserReg, owner, IRolldownPrimitives.ChainId.Ethereum, updaterAccount)
        );
        vm.stopBroadcast();

        vm.startBroadcast(owner);
            rolldownImplementation = new Rolldown();
            vm.expectRevert("Initializable: contract is already initialized");
            rolldownProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(rolldown))),
            address(rolldownImplementation),
            abi.encodeWithSelector(rolldown.initialize.selector, rolldownPauserReg, owner, IRolldownPrimitives.ChainId.Ethereum, updaterAccount)
        );
        vm.stopBroadcast();

        Rolldown rolldown2 = Rolldown(address(rolldown));
        address ownerFromCt = rolldown2.owner();
        assertEq(owner, ownerFromCt);

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
            abi.encodeWithSelector(rolldown.initialize.selector, rolldownPauserReg, owner, IRolldownPrimitives.ChainId.Ethereum, updaterAccount)
        );
        vm.stopBroadcast();
        
        RollDownUpg rolldown2 = RollDownUpg(address(rolldown));
        vm.expectRevert();
        bool res = rolldown2.imUpgraded();

        
        Rolldown rd2 = new RollDownUpg();
        vm.startBroadcast(owner);
            rolldownImplementation = rd2;
            rolldownProxyAdmin.upgrade(
            TransparentUpgradeableProxy(payable(address(rolldown))),
            address(rolldownImplementation)
        );
        vm.stopBroadcast();
        
        rolldown2 = RollDownUpg(address(rolldown));
        res = rolldown2.imUpgraded();
        assertEq(res, true);

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
            abi.encodeWithSelector(rolldown.initialize.selector, rolldownPauserReg, owner, IRolldownPrimitives.ChainId.Ethereum, updaterAccount)
        );
        vm.stopBroadcast();
        
        RollDownUpg rolldown2 = RollDownUpg(address(rolldown));
        vm.expectRevert();
        bool res = rolldown2.imUpgraded();

        
        Rolldown rd2 = new RollDownUpg();
        vm.startBroadcast(notOwner);
            rolldownImplementation = rd2;
            vm.expectRevert("Ownable: caller is not the owner");
            rolldownProxyAdmin.upgrade(
            TransparentUpgradeableProxy(payable(address(rolldown))),
            address(rolldownImplementation)
        );
        vm.stopBroadcast();


    }
}




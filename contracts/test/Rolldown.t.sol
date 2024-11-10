// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {PauserRegistry} from "@eigenlayer/contracts/permissions/PauserRegistry.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {Test} from "forge-std/Test.sol";
import {IRolldown} from "../src/IRolldown.sol";
import {IRolldownPrimitives} from "../src/IRolldownPrimitives.sol";
import {Rolldown} from "../src/Rolldown.sol";
import {Empty, MyERC20} from "./utils/Utilities.sol";

contract RolldownTest is Test, IRolldownPrimitives {
    struct Users {
        address admin;
        address executor;
        address grantee;
        address recipient;
        address updater;
        address feePayeeA;
        address feePayeeB;
        address feePayeeC;
    }

    Users public users;
    PauserRegistry public pauserRegistry;
    MyERC20 public token;
    IRolldown public rolldown;
    bytes32 public defaultAdminRole = 0x00;
    bytes32 public updaterRole;
    address public nativeTokenAddress;

    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
    event Transfer(address indexed from, address indexed to, uint256 value);

    function setUp() public {
        users = Users({
            admin: makeAddr("admin"),
            executor: makeAddr("executor"),
            grantee: makeAddr("grantee"),
            updater: makeAddr("updater"),
            recipient: makeAddr("recipient"),
            feePayeeA: makeAddr("feePayeeA"),
            feePayeeB: makeAddr("feePayeeB"),
            feePayeeC: makeAddr("feePayeeC")
        });

        deal(payable(users.admin), 100 ether);
        deal(payable(users.executor), 100 ether);
        deal(payable(users.grantee), 100 ether);
        deal(payable(users.updater), 100 ether);
        deal(payable(users.recipient), 100 ether);
        deal(payable(users.feePayeeA), 100 ether);
        deal(payable(users.feePayeeB), 100 ether);
        deal(payable(users.feePayeeC), 100 ether);

        address[] memory admins = new address[](1);
        admins[0] = users.admin;
        pauserRegistry = new PauserRegistry(admins, users.admin);

        token = new MyERC20();

        Rolldown implementation = new Rolldown();
        ProxyAdmin proxyAdmin = new ProxyAdmin();
        TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(
            address(implementation),
            address(proxyAdmin),
            abi.encodeWithSelector(
                Rolldown.initialize.selector,
                pauserRegistry,
                users.admin,
                IRolldownPrimitives.ChainId.Ethereum,
                users.updater
            )
        );
        rolldown = IRolldown(address(proxy));

        updaterRole = rolldown.UPDATER_ROLE();
        nativeTokenAddress = rolldown.NATIVE_TOKEN_ADDRESS();
    }
}

contract Deploy is RolldownTest {
    function test_HasDefaultAdminRole() external view {
        bool hasRole = rolldown.hasRole(defaultAdminRole, users.admin);
        assertTrue(hasRole);
    }

    function test_HasUpdaterRole() external view {
        bool hasRole = rolldown.hasRole(updaterRole, users.updater);
        assertTrue(hasRole);
    }

    function test_HasRightPausedStatus() external view {
        uint256 paused = rolldown.paused();
        assertEq(paused, 0);
    }

    function test_UpdaterAccountSet() external view {
        address updaterAccount = rolldown.updaterAccount();
        assertEq(updaterAccount, users.updater);
    }

    function test_CounterSet() external view {
        uint256 counter = rolldown.counter();
        assertEq(counter, 1);
    }

    function test_LastProcessedUpdateOriginL1Unset() external view {
        uint256 lastProcessedUpdate_origin_l1 = rolldown.lastProcessedUpdate_origin_l1();
        assertEq(lastProcessedUpdate_origin_l1, 0);
    }

    function test_LastProcessedUpdateOriginL2Unset() external view {
        uint256 lastProcessedUpdate_origin_l2 = rolldown.lastProcessedUpdate_origin_l2();
        assertEq(lastProcessedUpdate_origin_l2, 0);
    }

    function test_ChainIdSet() external view {
        IRolldownPrimitives.ChainId chainId = rolldown.chain();
        assertEq(uint256(chainId), uint256(IRolldownPrimitives.ChainId.Ethereum));
    }

    function test_RevertIf_InvalidInitialization() external {
        vm.expectRevert("Initializable: contract is already initialized");
        rolldown.initialize(pauserRegistry, users.admin, IRolldownPrimitives.ChainId.Ethereum, users.updater);
    }

    function test_RevertIf_ZeroAdmin() external {
        Rolldown implementation = new Rolldown();
        ProxyAdmin proxyAdmin = new ProxyAdmin();

        vm.expectRevert(abi.encodeWithSelector(ZeroAdmin.selector));
        new TransparentUpgradeableProxy(
            address(implementation),
            address(proxyAdmin),
            abi.encodeWithSelector(
                Rolldown.initialize.selector,
                pauserRegistry,
                address(0),
                IRolldownPrimitives.ChainId.Ethereum,
                users.updater
            )
        );
    }

    function test_RevertIf_ZeroUpdater() external {
        Rolldown implementation = new Rolldown();
        ProxyAdmin proxyAdmin = new ProxyAdmin();

        vm.expectRevert(abi.encodeWithSelector(ZeroUpdater.selector));
        new TransparentUpgradeableProxy(
            address(implementation),
            address(proxyAdmin),
            abi.encodeWithSelector(
                Rolldown.initialize.selector,
                pauserRegistry,
                users.admin,
                IRolldownPrimitives.ChainId.Ethereum,
                address(0)
            )
        );
    }
}

contract GrantRole is RolldownTest {
    function test_EmitRoleGranted() external {
        vm.prank(users.admin);
        vm.expectEmit();
        emit RoleGranted(defaultAdminRole, users.updater, users.admin);
        rolldown.grantRole(defaultAdminRole, users.updater);
    }

    function test_HasDefaultAdminRole() external {
        vm.prank(users.admin);
        rolldown.grantRole(defaultAdminRole, users.grantee);

        bool hasRole = rolldown.hasRole(defaultAdminRole, users.grantee);
        assertTrue(hasRole);
    }

    function test_RevertIf_UnauthorizedAccount() external {
        vm.prank(users.grantee);
        vm.expectRevert(
            "AccessControl: account 0x6e03f5de2c52648bedc16f2ad3d5cfe890017a90 is missing role 0x0000000000000000000000000000000000000000000000000000000000000000"
        );
        rolldown.grantRole(defaultAdminRole, users.grantee);
    }
}

contract RevokeRole is RolldownTest {
    function test_EmitRoleRevoked() external {
        vm.startPrank(users.admin);

        rolldown.grantRole(defaultAdminRole, users.grantee);

        vm.expectEmit();
        emit RoleRevoked(defaultAdminRole, users.grantee, users.admin);
        rolldown.revokeRole(defaultAdminRole, users.grantee);

        vm.stopPrank();
    }

    function test_HasNotDefaultAdminRole() external {
        vm.startPrank(users.admin);

        rolldown.grantRole(defaultAdminRole, users.grantee);
        rolldown.revokeRole(defaultAdminRole, users.grantee);

        vm.stopPrank();

        bool hasRole = rolldown.hasRole(defaultAdminRole, users.grantee);
        assertFalse(hasRole);
    }

    function test_RevertIf_UnathorizedAccount() external {
        vm.prank(users.admin);
        rolldown.grantRole(defaultAdminRole, users.grantee);

        vm.prank(users.executor);
        vm.expectRevert(
            "AccessControl: account 0x4aff9da56bee9417ae4be3933b24f77b3e29ca30 is missing role 0x0000000000000000000000000000000000000000000000000000000000000000"
        );
        rolldown.revokeRole(defaultAdminRole, users.grantee);
    }
}

contract RenounceRole is RolldownTest {
    function test_EmitRoleRevoked() external {
        vm.prank(users.admin);
        rolldown.grantRole(defaultAdminRole, users.grantee);

        vm.prank(users.grantee);
        vm.expectEmit();
        emit RoleRevoked(defaultAdminRole, users.grantee, users.grantee);
        rolldown.renounceRole(defaultAdminRole, users.grantee);
    }

    function test_HasNotDefaultAdminRole() external {
        vm.prank(users.admin);
        rolldown.grantRole(defaultAdminRole, users.grantee);

        vm.prank(users.grantee);
        rolldown.renounceRole(defaultAdminRole, users.grantee);

        bool hasRole = rolldown.hasRole(defaultAdminRole, users.grantee);
        assertFalse(hasRole);
    }

    function test_RevertIf_BadConfirmation() external {
        vm.startPrank(users.admin);

        rolldown.grantRole(defaultAdminRole, users.grantee);

        vm.expectRevert("AccessControl: can only renounce roles for self");
        rolldown.renounceRole(defaultAdminRole, users.grantee);

        vm.stopPrank();
    }
}

contract SetUpdater is RolldownTest {
    address newUpdater = makeAddr("newUpdater");

    function test_EmitNewUpdaterSet() external {
        vm.prank(users.admin);
        vm.expectEmit();
        emit NewUpdaterSet(newUpdater);
        rolldown.setUpdater(newUpdater);
    }

    function test_SetNewUpdaterAddress() external {
        vm.prank(users.admin);
        rolldown.setUpdater(newUpdater);

        bool initialUpdater = rolldown.hasRole(updaterRole, users.updater);
        assertFalse(initialUpdater);

        bool currentUpdater = rolldown.hasRole(updaterRole, newUpdater);
        assertTrue(currentUpdater);
        assertEq(rolldown.updaterAccount(), newUpdater);
    }

    function test_RevertIf_UnauthorizedAccount() external {
        vm.prank(users.executor);
        vm.expectRevert(
            "AccessControl: account 0x4aff9da56bee9417ae4be3933b24f77b3e29ca30 is missing role 0x0000000000000000000000000000000000000000000000000000000000000000"
        );
        rolldown.setUpdater(newUpdater);
    }

    function test_RevertIf_ZeroUpdater() external {
        vm.prank(users.admin);
        vm.expectRevert(abi.encodeWithSelector(ZeroUpdater.selector));
        rolldown.setUpdater(address(0));
    }
}

contract DepositNative is RolldownTest {
    uint256 public amount = 1 ether;
    uint256 public ferryTip = 0;

    function test_EmitDepositAcceptedIntoQueue() external {
        uint256 requestId = rolldown.counter();

        vm.prank(users.executor);
        vm.expectEmit();
        emit DepositAcceptedIntoQueue(requestId, users.executor, nativeTokenAddress, amount, ferryTip);
        rolldown.depositNative{value: amount}();
    }

    function test_ChangeBalances() external {
        uint256 initialExecutorBalance = users.executor.balance;
        uint256 initialRolldownBalance = address(rolldown).balance;

        vm.prank(users.executor);
        rolldown.depositNative{value: amount}();

        uint256 currentExecutorBalance = users.executor.balance;
        assertEq(initialExecutorBalance - currentExecutorBalance, amount);

        uint256 currentRolldownBalance = address(rolldown).balance;
        assertEq(currentRolldownBalance - initialRolldownBalance, amount);
    }

    function test_GetUpdateForL2() external {
        uint256 requestId = rolldown.counter();

        vm.prank(users.executor);
        rolldown.depositNative{value: amount}();

        Rolldown.L1Update memory l1Update = rolldown.getUpdateForL2();
        assertEq(l1Update.pendingDeposits.length, 1);

        Rolldown.Deposit memory pendingDeposit = l1Update.pendingDeposits[0];
        assertEq(pendingDeposit.requestId.id, requestId);
        assertEq(pendingDeposit.depositRecipient, users.executor);
        assertEq(pendingDeposit.tokenAddress, nativeTokenAddress);
        assertEq(pendingDeposit.amount, amount);
        assertEq(pendingDeposit.ferryTip, ferryTip);
    }

    function test_RevertIf_Paused() external {
        vm.prank(users.admin);
        rolldown.pause(1);

        vm.prank(users.executor);
        vm.expectRevert("Pausable: contract is paused");
        rolldown.depositNative{value: amount}();
    }

    function test_RevertIf_ZeroAmount() external {
        vm.prank(users.executor);
        vm.expectRevert(abi.encodeWithSelector(ZeroAmount.selector));
        rolldown.depositNative{value: 0}();
    }
}

contract DepositNativeWithFerryTip is RolldownTest {
    uint256 public amount = 1 ether;
    uint256 public ferryTip = amount / 100;

    function test_EmitDepositAcceptedIntoQueue() external {
        uint256 requestId = rolldown.counter();

        vm.prank(users.executor);
        vm.expectEmit();
        emit DepositAcceptedIntoQueue(requestId, users.executor, nativeTokenAddress, amount, ferryTip);
        rolldown.depositNative{value: amount}(ferryTip);
    }

    function test_ChangeBalances() external {
        uint256 initialExecutorBalance = users.executor.balance;
        uint256 initialRolldownBalance = address(rolldown).balance;

        vm.prank(users.executor);
        rolldown.depositNative{value: amount}(ferryTip);

        uint256 currentExecutorBalance = users.executor.balance;
        assertEq(initialExecutorBalance - currentExecutorBalance, amount);

        uint256 currentRolldownBalance = address(rolldown).balance;
        assertEq(currentRolldownBalance - initialRolldownBalance, amount);
    }

    function test_GetUpdateForL2() external {
        uint256 requestId = rolldown.counter();

        vm.prank(users.executor);
        rolldown.depositNative{value: amount}(ferryTip);

        Rolldown.L1Update memory l1Update = rolldown.getUpdateForL2();
        assertEq(l1Update.pendingDeposits.length, 1);

        Rolldown.Deposit memory pendingDeposit = l1Update.pendingDeposits[0];
        assertEq(pendingDeposit.requestId.id, requestId);
        assertEq(pendingDeposit.depositRecipient, users.executor);
        assertEq(pendingDeposit.tokenAddress, nativeTokenAddress);
        assertEq(pendingDeposit.amount, amount);
        assertEq(pendingDeposit.ferryTip, ferryTip);
    }

    function test_RevertIf_Paused() external {
        vm.prank(users.admin);
        rolldown.pause(1);

        vm.prank(users.executor);
        vm.expectRevert("Pausable: contract is paused");
        rolldown.depositNative{value: amount}(ferryTip);
    }

    function test_RevertIf_ZeroAmount() external {
        vm.prank(users.executor);
        vm.expectRevert(abi.encodeWithSelector(ZeroAmount.selector));
        rolldown.depositNative{value: 0}(ferryTip);
    }

    function test_RevertIf_FerryTipExceedsAmount() external {
        ferryTip = amount + 1;

        vm.prank(users.executor);
        vm.expectRevert(abi.encodeWithSelector(FerryTipExceedsAmount.selector, ferryTip, amount));
        rolldown.depositNative{value: amount}(ferryTip);
    }
}

contract DepositERC20 is RolldownTest {
    uint256 public amount = 1 ether;
    uint256 public ferryTip = 0;

    function test_EmitDepositAcceptedIntoQueue() external {
        vm.startPrank(users.executor);

        uint256 requestId = rolldown.counter();
        token.mint(users.executor);
        token.approve(address(rolldown), amount);

        vm.expectEmit();
        emit DepositAcceptedIntoQueue(requestId, users.executor, address(token), amount, ferryTip);
        rolldown.depositERC20(address(token), amount);

        vm.stopPrank();
    }

    function test_ChangeBalances() external {
        token.mint(users.executor);
        uint256 initialExecutorBalance = token.balanceOf(users.executor);
        uint256 initialRolldownBalance = token.balanceOf(address(rolldown));

        vm.startPrank(users.executor);

        token.approve(address(rolldown), amount);
        rolldown.depositERC20(address(token), amount);

        vm.stopPrank();

        uint256 currentExecutorBalance = token.balanceOf(users.executor);
        assertEq(initialExecutorBalance - currentExecutorBalance, amount);

        uint256 currentRolldownBalance = token.balanceOf(address(rolldown));
        assertEq(currentRolldownBalance - initialRolldownBalance, amount);
    }

    function test_GetUpdateForL2() external {
        token.mint(users.executor);
        uint256 requestId = rolldown.counter();

        vm.startPrank(users.executor);

        token.approve(address(rolldown), amount);
        rolldown.depositERC20(address(token), amount);

        vm.stopPrank();

        Rolldown.L1Update memory l1Update = rolldown.getUpdateForL2();
        assertEq(l1Update.pendingDeposits.length, 1);

        Rolldown.Deposit memory pendingDeposit = l1Update.pendingDeposits[0];
        assertEq(pendingDeposit.requestId.id, requestId);
        assertEq(pendingDeposit.depositRecipient, users.executor);
        assertEq(pendingDeposit.tokenAddress, address(token));
        assertEq(pendingDeposit.amount, amount);
        assertEq(pendingDeposit.ferryTip, ferryTip);
    }

    function test_RevertIf_Paused() external {
        vm.prank(users.admin);
        rolldown.pause(1);

        vm.prank(users.executor);
        vm.expectRevert("Pausable: contract is paused");
        rolldown.depositERC20(address(token), amount);
    }

    function test_RevertIf_ZeroAmount() external {
        vm.prank(users.executor);
        vm.expectRevert(abi.encodeWithSelector(ZeroAmount.selector));
        rolldown.depositERC20(address(token), 0);
    }

    function test_RevertIf_ZeroToken() external {
        vm.prank(users.executor);
        vm.expectRevert(abi.encodeWithSelector(ZeroToken.selector));
        rolldown.depositERC20(address(0), amount);
    }
}

contract DepositERC20WithFerryTip is RolldownTest {
    uint256 public amount = 1 ether;
    uint256 public ferryTip = amount / 100;

    function test_EmitDepositAcceptedIntoQueue() external {
        vm.startPrank(users.executor);

        uint256 requestId = rolldown.counter();
        token.mint(users.executor);
        token.approve(address(rolldown), amount);

        vm.expectEmit();
        emit DepositAcceptedIntoQueue(requestId, users.executor, address(token), amount, ferryTip);
        rolldown.depositERC20(address(token), amount, ferryTip);

        vm.stopPrank();
    }

    function test_ChangeBalances() external {
        token.mint(users.executor);
        uint256 initialExecutorBalance = token.balanceOf(users.executor);
        uint256 initialRolldownBalance = token.balanceOf(address(rolldown));

        vm.startPrank(users.executor);

        token.approve(address(rolldown), amount);
        rolldown.depositERC20(address(token), amount, ferryTip);

        vm.stopPrank();

        uint256 currentExecutorBalance = token.balanceOf(users.executor);
        assertEq(initialExecutorBalance - currentExecutorBalance, amount);

        uint256 currentRolldownBalance = token.balanceOf(address(rolldown));
        assertEq(currentRolldownBalance - initialRolldownBalance, amount);
    }

    function test_GetUpdateForL2() external {
        token.mint(users.executor);
        uint256 requestId = rolldown.counter();

        vm.startPrank(users.executor);

        token.approve(address(rolldown), amount);
        rolldown.depositERC20(address(token), amount, ferryTip);

        vm.stopPrank();

        Rolldown.L1Update memory l1Update = rolldown.getUpdateForL2();
        assertEq(l1Update.pendingDeposits.length, 1);

        Rolldown.Deposit memory pendingDeposit = l1Update.pendingDeposits[0];
        assertEq(pendingDeposit.requestId.id, requestId);
        assertEq(pendingDeposit.depositRecipient, users.executor);
        assertEq(pendingDeposit.tokenAddress, address(token));
        assertEq(pendingDeposit.amount, amount);
        assertEq(pendingDeposit.ferryTip, ferryTip);
    }

    function test_RevertIf_Paused() external {
        vm.prank(users.admin);
        rolldown.pause(1);

        vm.prank(users.executor);
        vm.expectRevert("Pausable: contract is paused");
        rolldown.depositERC20(address(token), amount, ferryTip);
    }

    function test_RevertIf_ZeroAmount() external {
        vm.prank(users.executor);
        vm.expectRevert(abi.encodeWithSelector(ZeroAmount.selector));
        rolldown.depositERC20(address(token), 0, ferryTip);
    }

    function test_RevertIf_ZeroToken() external {
        vm.prank(users.executor);
        vm.expectRevert(abi.encodeWithSelector(ZeroToken.selector));
        rolldown.depositERC20(address(0), amount, ferryTip);
    }

    function test_RevertIf_FerryTipExceedsAmount() external {
        ferryTip = amount + 1;

        vm.prank(users.executor);
        vm.expectRevert(abi.encodeWithSelector(FerryTipExceedsAmount.selector, ferryTip, amount));
        rolldown.depositERC20(address(token), amount, ferryTip);
    }
}

contract FerryWithdrawal is RolldownTest {
    uint256 public amount = 1 ether;
    uint256 public ferryTip = amount / 100;
    uint256 public ferriedAmount = amount - ferryTip;

    function test_EmitWithdrawalFerriedNative() external {
        IRolldownPrimitives.Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: _createRequestId(),
            recipient: users.recipient,
            tokenAddress: nativeTokenAddress,
            amount: amount,
            ferryTip: ferryTip
        });

        bytes32 withdrawalHash = rolldown.hashWithdrawal(withdrawal);

        vm.prank(users.executor);
        vm.expectEmit();
        emit WithdrawalFerried(withdrawal.requestId.id, ferriedAmount, users.recipient, users.executor, withdrawalHash);
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
    }

    function test_EmitWithdrawalFerriedERC20() external {
        token.mint(users.executor);
        token.mint(address(rolldown));

        IRolldownPrimitives.Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: _createRequestId(),
            recipient: users.recipient,
            tokenAddress: address(token),
            amount: amount,
            ferryTip: ferryTip
        });

        bytes32 withdrawalHash = rolldown.hashWithdrawal(withdrawal);

        vm.startPrank(users.executor);

        token.approve(address(rolldown), ferriedAmount);

        vm.expectEmit();
        emit WithdrawalFerried(withdrawal.requestId.id, ferriedAmount, users.recipient, users.executor, withdrawalHash);
        rolldown.ferryWithdrawal(withdrawal);

        vm.stopPrank();
    }

    function test_ChangeBalancesNative() external {
        IRolldownPrimitives.Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: _createRequestId(),
            recipient: users.recipient,
            tokenAddress: nativeTokenAddress,
            amount: amount,
            ferryTip: ferryTip
        });

        uint256 intitalExecutorBalance = users.executor.balance;

        vm.prank(users.executor);
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);

        uint256 currentRecipientBalance = users.recipient.balance;
        assertEq(currentRecipientBalance, intitalExecutorBalance + ferriedAmount);

        uint256 currentExecutorBalance = users.executor.balance;
        assertEq(currentExecutorBalance, intitalExecutorBalance - amount + ferryTip);
    }

    function test_ChangeBalancesERC20() external {
        token.mint(users.executor);
        token.mint(address(rolldown));

        IRolldownPrimitives.Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: _createRequestId(),
            recipient: users.recipient,
            tokenAddress: address(token),
            amount: amount,
            ferryTip: ferryTip
        });

        uint256 intitalExecutorBalance = token.balanceOf(users.executor);

        vm.startPrank(users.executor);

        token.approve(address(rolldown), ferriedAmount);
        rolldown.ferryWithdrawal(withdrawal);

        vm.stopPrank();

        uint256 currentRecipientBalance = token.balanceOf(users.recipient);
        assertEq(currentRecipientBalance, ferriedAmount);

        uint256 currentExecutorBalance = token.balanceOf(users.executor);
        assertEq(currentExecutorBalance, intitalExecutorBalance - amount + ferryTip);
    }

    function test_RevertIf_Paused() external {
        vm.prank(users.admin);
        rolldown.pause(1);

        IRolldownPrimitives.Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: _createRequestId(),
            recipient: users.recipient,
            tokenAddress: address(token),
            amount: amount,
            ferryTip: ferryTip
        });

        vm.prank(users.executor);
        vm.expectRevert("Pausable: contract is paused");
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
    }

    function test_RevertIf_ZeroAmount() external {
        IRolldownPrimitives.Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: _createRequestId(),
            recipient: users.recipient,
            tokenAddress: nativeTokenAddress,
            amount: 0,
            ferryTip: ferryTip
        });

        vm.prank(users.executor);
        vm.expectRevert(abi.encodeWithSelector(ZeroAmount.selector));
        rolldown.ferryWithdrawal{value: 0}(withdrawal);
    }

    function test_RevertIf_FerryTipExceedsAmount() external {
        ferryTip = amount + 1;

        IRolldownPrimitives.Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: _createRequestId(),
            recipient: users.recipient,
            tokenAddress: nativeTokenAddress,
            amount: amount,
            ferryTip: ferryTip
        });

        vm.prank(users.executor);
        vm.expectRevert(abi.encodeWithSelector(FerryTipExceedsAmount.selector, ferryTip, amount));
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
    }

    function test_RevertIf_ZeroRecipient() external {
        IRolldownPrimitives.Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: _createRequestId(),
            recipient: address(0),
            tokenAddress: nativeTokenAddress,
            amount: amount,
            ferryTip: ferryTip
        });

        vm.prank(users.executor);
        vm.expectRevert(IRolldownPrimitives.ZeroRecipient.selector);
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
    }

    function test_RevertIf_InvalidFerriedAmount() external {
        ferriedAmount -= 1;

        IRolldownPrimitives.Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: _createRequestId(),
            recipient: users.recipient,
            tokenAddress: nativeTokenAddress,
            amount: amount,
            ferryTip: ferryTip
        });

        vm.prank(users.executor);
        vm.expectRevert(abi.encodeWithSelector(InvalidFerriedAmount.selector, ferriedAmount, amount - ferryTip));
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
    }

    function test_RevertIf_InvalidFerriedAmount2() external {
        IRolldownPrimitives.Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: _createRequestId(),
            recipient: users.recipient,
            tokenAddress: nativeTokenAddress,
            amount: amount,
            ferryTip: ferryTip
        });

        vm.prank(users.executor);
        vm.expectRevert(abi.encodeWithSelector(InvalidFerriedAmount.selector, 0, ferriedAmount));
        rolldown.ferryWithdrawal{value: 0}(withdrawal);
    }

    function test_RevertIf_WithdrawalAlreadyFerried() external {
        IRolldownPrimitives.Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: _createRequestId(),
            recipient: users.recipient,
            tokenAddress: nativeTokenAddress,
            amount: amount,
            ferryTip: ferryTip
        });

        bytes32 withdrawalHash = rolldown.hashWithdrawal(withdrawal);

        vm.startPrank(users.executor);

        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);

        vm.expectRevert(abi.encodeWithSelector(WithdrawalAlreadyFerried.selector, withdrawalHash));
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);

        vm.stopPrank();
    }

    function test_RevertIf_NonContractCall() external {
        IRolldownPrimitives.Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: _createRequestId(),
            recipient: users.recipient,
            tokenAddress: address(0),
            amount: amount,
            ferryTip: ferryTip
        });

        vm.prank(users.executor);
        vm.expectRevert("Address: call to non-contract");
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
    }

    function test_RevertIf_TransferOperationFailed() external {
        Empty empty = new Empty();

        IRolldownPrimitives.Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: _createRequestId(),
            recipient: users.recipient,
            tokenAddress: address(empty),
            amount: amount,
            ferryTip: ferryTip
        });

        vm.prank(users.executor);
        vm.expectRevert("SafeERC20: low-level call failed");
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
    }

    function _createRequestId() private view returns (IRolldownPrimitives.RequestId memory) {
        uint256 requestId = rolldown.counter();
        return IRolldownPrimitives.RequestId({id: requestId, origin: IRolldownPrimitives.Origin.L2});
    }
}

contract CloseWithdrawal is RolldownTest {
    uint256 public ferryTip = 10;
    uint256 public amount = 10;

    function _createWithdrawal(uint256 id, address recipient, uint256 amount, address tokenAddress, uint256 ferryTip)
        internal
        pure
        returns (IRolldownPrimitives.Withdrawal memory)
    {
        return IRolldownPrimitives.Withdrawal({
            requestId: IRolldownPrimitives.RequestId({id: id, origin: IRolldownPrimitives.Origin.L2}),
            recipient: recipient,
            tokenAddress: tokenAddress,
            amount: amount,
            ferryTip: ferryTip
        });
    }

    function test_EmitNativeTokensWithdrawn() external {
        vm.startPrank(users.updater);
        token.approve(address(rolldown), amount);
        rolldown.depositNative{value: amount}();
        vm.stopPrank();

        IRolldownPrimitives.Withdrawal memory withdrawalPayeeA =
            _createWithdrawal(1, users.feePayeeA, amount, nativeTokenAddress, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalPayeeB =
            _createWithdrawal(2, users.feePayeeB, amount, nativeTokenAddress, 0);

        bytes32 nodeA = rolldown.hashWithdrawal(withdrawalPayeeA);
        bytes32 nodeB = rolldown.hashWithdrawal(withdrawalPayeeB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(nodeA, nodeB));
        bytes32[] memory proof_withdrawalPayeeB = new bytes32[](1);
        proof_withdrawalPayeeB[0] = nodeA;

        vm.prank(users.updater);
        rolldown.update_l1_from_l2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 2}));

        vm.startPrank(users.feePayeeB);
        vm.expectEmit();
        emit NativeTokensWithdrawn(users.feePayeeB, amount);
        vm.expectEmit();
        emit WithdrawalClosed(2, nodeB);

        rolldown.closeWithdrawal(withdrawalPayeeB, merkleRoot, proof_withdrawalPayeeB);
        vm.stopPrank();
    }

    function test_EmitERC20TokensWithdrawn() external {
        token.mint(users.updater);
        token.mint(address(rolldown));

        vm.startPrank(users.updater);
        token.approve(address(rolldown), amount);
        rolldown.depositERC20(address(token), amount);
        vm.stopPrank();

        IRolldownPrimitives.Withdrawal memory withdrawalPayeeA =
            _createWithdrawal(1, users.feePayeeA, amount, address(token), 0);
        IRolldownPrimitives.Withdrawal memory withdrawalPayeeB =
            _createWithdrawal(2, users.feePayeeB, amount, address(token), 0);

        bytes32 nodeA = rolldown.hashWithdrawal(withdrawalPayeeA);
        bytes32 nodeB = rolldown.hashWithdrawal(withdrawalPayeeB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(nodeA, nodeB));
        bytes32[] memory proof_withdrawalPayeeB = new bytes32[](1);
        proof_withdrawalPayeeB[0] = nodeA;

        vm.prank(users.updater);
        rolldown.update_l1_from_l2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 2}));

        vm.startPrank(users.feePayeeB);
        vm.expectEmit();
        emit ERC20TokensWithdrawn(users.feePayeeB, address(token), amount);
        vm.expectEmit();
        emit Transfer(address(rolldown), users.feePayeeB, amount);
        vm.expectEmit();
        emit WithdrawalClosed(2, nodeB);

        rolldown.closeWithdrawal(withdrawalPayeeB, merkleRoot, proof_withdrawalPayeeB);
        vm.stopPrank();
    }

    function test_EmitNativeTokensWithdrawnFerryTip() external {
        vm.startPrank(users.updater);
        token.approve(address(rolldown), amount);
        rolldown.depositNative{value: amount}();
        vm.stopPrank();

        IRolldownPrimitives.Withdrawal memory withdrawalPayeeA =
            _createWithdrawal(1, users.feePayeeA, amount, nativeTokenAddress, ferryTip);
        IRolldownPrimitives.Withdrawal memory withdrawalPayeeB =
            _createWithdrawal(2, users.feePayeeB, amount, nativeTokenAddress, ferryTip);

        bytes32 nodeA = rolldown.hashWithdrawal(withdrawalPayeeA);
        bytes32 nodeB = rolldown.hashWithdrawal(withdrawalPayeeB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(nodeA, nodeB));
        bytes32[] memory proof_withdrawalPayeeB = new bytes32[](1);
        proof_withdrawalPayeeB[0] = nodeA;

        vm.prank(users.updater);
        rolldown.update_l1_from_l2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 2}));

        vm.startPrank(users.feePayeeB);
        token.approve(address(rolldown), amount - ferryTip);
        rolldown.ferryWithdrawal(withdrawalPayeeB);

        vm.expectEmit();
        emit NativeTokensWithdrawn(users.feePayeeB, amount);
        vm.expectEmit();
        emit FerriedWithdrawalClosed(2, nodeB);

        rolldown.closeWithdrawal(withdrawalPayeeB, merkleRoot, proof_withdrawalPayeeB);
        vm.stopPrank();
    }

    function test_EmitERC20TokensWithdrawnFerryTip() external {
        token.mint(users.updater);
        token.mint(address(rolldown));

        vm.startPrank(users.updater);
        token.approve(address(rolldown), amount);
        rolldown.depositERC20(address(token), amount);
        vm.stopPrank();

        IRolldownPrimitives.Withdrawal memory withdrawalPayeeA =
            _createWithdrawal(1, users.feePayeeA, amount, address(token), ferryTip);
        IRolldownPrimitives.Withdrawal memory withdrawalPayeeB =
            _createWithdrawal(2, users.feePayeeB, amount, address(token), ferryTip);

        bytes32 nodeA = rolldown.hashWithdrawal(withdrawalPayeeA);
        bytes32 nodeB = rolldown.hashWithdrawal(withdrawalPayeeB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(nodeA, nodeB));
        bytes32[] memory proof_withdrawalPayeeB = new bytes32[](1);
        proof_withdrawalPayeeB[0] = nodeA;

        vm.prank(users.updater);
        rolldown.update_l1_from_l2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 2}));

        vm.startPrank(users.feePayeeB);
        token.approve(address(rolldown), amount - ferryTip);
        rolldown.ferryWithdrawal(withdrawalPayeeB);

        vm.expectEmit();
        emit ERC20TokensWithdrawn(users.feePayeeB, address(token), amount);
        vm.expectEmit();
        emit Transfer(address(rolldown), users.feePayeeB, amount);
        vm.expectEmit();
        emit FerriedWithdrawalClosed(2, nodeB);

        rolldown.closeWithdrawal(withdrawalPayeeB, merkleRoot, proof_withdrawalPayeeB);
        vm.stopPrank();
    }

    function test_ExecuteCloseWithdrawalNative() external {
        vm.startPrank(users.updater);
        token.approve(address(rolldown), amount * 4);
        rolldown.depositNative{value: amount * 4}();
        vm.stopPrank();

        uint256 initialFeePayeeABalance = users.feePayeeA.balance;
        uint256 initialFeePayeeBBalance = users.feePayeeB.balance;
        uint256 initialFeePayeeCBalance = users.feePayeeC.balance;

        IRolldownPrimitives.Withdrawal memory withdrawalPayeeA =
            _createWithdrawal(1, users.feePayeeA, amount, nativeTokenAddress, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalPayeeA2 =
            _createWithdrawal(2, users.feePayeeA, amount, nativeTokenAddress, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalPayeeB =
            _createWithdrawal(3, users.feePayeeB, amount, nativeTokenAddress, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalPayeeC =
            _createWithdrawal(4, users.feePayeeC, amount, nativeTokenAddress, 0);

        bytes32[] memory nodes = new bytes32[](6);
        nodes[0] = rolldown.hashWithdrawal(withdrawalPayeeA);
        nodes[1] = rolldown.hashWithdrawal(withdrawalPayeeA2);
        nodes[2] = rolldown.hashWithdrawal(withdrawalPayeeB);
        nodes[3] = rolldown.hashWithdrawal(withdrawalPayeeC);
        nodes[4] = keccak256(abi.encodePacked(nodes[0], nodes[1]));
        nodes[5] = keccak256(abi.encodePacked(nodes[2], nodes[3]));

        bytes32 merkleRoot = keccak256(abi.encodePacked(nodes[4], nodes[5]));

        bytes32[][] memory proofs = new bytes32[][](4);
        proofs[0] = new bytes32[](2);
        proofs[1] = new bytes32[](2);
        proofs[2] = new bytes32[](2);
        proofs[3] = new bytes32[](2);

        proofs[0][0] = nodes[1];
        proofs[0][1] = nodes[5];
        proofs[1][0] = nodes[0];
        proofs[1][1] = nodes[5];
        proofs[2][0] = nodes[3];
        proofs[2][1] = nodes[4];
        proofs[3][0] = nodes[2];
        proofs[3][1] = nodes[4];

        vm.prank(users.updater);
        rolldown.update_l1_from_l2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 4}));

        vm.startPrank(users.feePayeeA);
        rolldown.closeWithdrawal(withdrawalPayeeA, merkleRoot, proofs[0]);
        rolldown.closeWithdrawal(withdrawalPayeeA2, merkleRoot, proofs[1]);
        vm.stopPrank();

        vm.prank(users.feePayeeB);
        rolldown.closeWithdrawal(withdrawalPayeeB, merkleRoot, proofs[2]);

        vm.prank(users.feePayeeC);
        rolldown.closeWithdrawal(withdrawalPayeeC, merkleRoot, proofs[3]);

        uint256 currentFeePayeeABalance = users.feePayeeA.balance;
        uint256 currentFeePayeeBBalance = users.feePayeeB.balance;
        uint256 currentFeePayeeCBalance = users.feePayeeC.balance;

        assertEq(currentFeePayeeABalance, initialFeePayeeABalance + amount * 2);
        assertEq(currentFeePayeeBBalance, initialFeePayeeBBalance + amount);
        assertEq(currentFeePayeeCBalance, initialFeePayeeCBalance + amount);
        assertTrue(rolldown.processedL2Requests(nodes[0]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(nodes[1]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(nodes[2]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(nodes[3]) == rolldown.CLOSED());
        assertEq(rolldown.lastProcessedUpdate_origin_l2(), 4);
    }

    function test_ExecuteCloseWithdrawalERC20() external {
        token.mint(users.updater);
        token.mint(address(rolldown));

        vm.startPrank(users.updater);
        token.approve(address(rolldown), amount);
        rolldown.depositERC20(address(token), amount);
        vm.stopPrank();

        uint256 initialFeePayeeABalance = token.balanceOf(users.feePayeeA);
        uint256 initialFeePayeeBBalance = token.balanceOf(users.feePayeeB);
        uint256 initialFeePayeeCBalance = token.balanceOf(users.feePayeeC);

        IRolldownPrimitives.Withdrawal memory withdrawalPayeeA =
            _createWithdrawal(1, users.feePayeeA, amount, address(token), 0);
        IRolldownPrimitives.Withdrawal memory withdrawalPayeeA2 =
            _createWithdrawal(2, users.feePayeeA, amount, address(token), 0);
        IRolldownPrimitives.Withdrawal memory withdrawalPayeeB =
            _createWithdrawal(3, users.feePayeeB, amount, address(token), 0);
        IRolldownPrimitives.Withdrawal memory withdrawalPayeeC =
            _createWithdrawal(4, users.feePayeeC, amount, address(token), 0);

        bytes32[] memory nodes = new bytes32[](6);
        nodes[0] = rolldown.hashWithdrawal(withdrawalPayeeA);
        nodes[1] = rolldown.hashWithdrawal(withdrawalPayeeA2);
        nodes[2] = rolldown.hashWithdrawal(withdrawalPayeeB);
        nodes[3] = rolldown.hashWithdrawal(withdrawalPayeeC);
        nodes[4] = keccak256(abi.encodePacked(nodes[0], nodes[1]));
        nodes[5] = keccak256(abi.encodePacked(nodes[2], nodes[3]));

        bytes32 merkleRoot = keccak256(abi.encodePacked(nodes[4], nodes[5]));

        bytes32[][] memory proofs = new bytes32[][](4);
        proofs[0] = new bytes32[](2);
        proofs[1] = new bytes32[](2);
        proofs[2] = new bytes32[](2);
        proofs[3] = new bytes32[](2);

        proofs[0][0] = nodes[1];
        proofs[0][1] = nodes[5];
        proofs[1][0] = nodes[0];
        proofs[1][1] = nodes[5];
        proofs[2][0] = nodes[3];
        proofs[2][1] = nodes[4];
        proofs[3][0] = nodes[2];
        proofs[3][1] = nodes[4];

        vm.prank(users.updater);
        rolldown.update_l1_from_l2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 4}));

        vm.startPrank(users.feePayeeA);
        rolldown.closeWithdrawal(withdrawalPayeeA, merkleRoot, proofs[0]);
        rolldown.closeWithdrawal(withdrawalPayeeA2, merkleRoot, proofs[1]);
        vm.stopPrank();

        vm.prank(users.feePayeeB);
        rolldown.closeWithdrawal(withdrawalPayeeB, merkleRoot, proofs[2]);

        vm.prank(users.feePayeeC);
        rolldown.closeWithdrawal(withdrawalPayeeC, merkleRoot, proofs[3]);

        uint256 currentFeePayeeABalance = token.balanceOf(users.feePayeeA);
        uint256 currentFeePayeeBBalance = token.balanceOf(users.feePayeeB);
        uint256 currentFeePayeeCBalance = token.balanceOf(users.feePayeeC);

        assertEq(currentFeePayeeABalance, initialFeePayeeABalance + amount * 2);
        assertEq(currentFeePayeeBBalance, initialFeePayeeBBalance + amount);
        assertEq(currentFeePayeeCBalance, initialFeePayeeCBalance + amount);
        assertTrue(rolldown.processedL2Requests(nodes[0]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(nodes[1]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(nodes[2]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(nodes[3]) == rolldown.CLOSED());
        assertEq(rolldown.lastProcessedUpdate_origin_l2(), 4);
    }

    function test_ExecuteCloseWithdrawalNativeFerryTip() external {
        vm.startPrank(users.updater);
        token.approve(address(rolldown), amount * 4);
        rolldown.depositNative{value: amount * 4}();
        vm.stopPrank();

        uint256 initialFeePayeeABalance = users.feePayeeA.balance;
        uint256 initialFeePayeeBBalance = users.feePayeeB.balance;
        uint256 initialFeePayeeCBalance = users.feePayeeC.balance;

        IRolldownPrimitives.Withdrawal memory withdrawalPayeeA =
            _createWithdrawal(1, users.feePayeeA, amount, nativeTokenAddress, ferryTip);
        IRolldownPrimitives.Withdrawal memory withdrawalPayeeA2 =
            _createWithdrawal(2, users.feePayeeA, amount, nativeTokenAddress, ferryTip);
        IRolldownPrimitives.Withdrawal memory withdrawalPayeeB =
            _createWithdrawal(3, users.feePayeeB, amount, nativeTokenAddress, ferryTip);
        IRolldownPrimitives.Withdrawal memory withdrawalPayeeC =
            _createWithdrawal(4, users.feePayeeC, amount, nativeTokenAddress, ferryTip);

        bytes32[] memory nodes = new bytes32[](6);
        nodes[0] = rolldown.hashWithdrawal(withdrawalPayeeA);
        nodes[1] = rolldown.hashWithdrawal(withdrawalPayeeA2);
        nodes[2] = rolldown.hashWithdrawal(withdrawalPayeeB);
        nodes[3] = rolldown.hashWithdrawal(withdrawalPayeeC);
        nodes[4] = keccak256(abi.encodePacked(nodes[0], nodes[1]));
        nodes[5] = keccak256(abi.encodePacked(nodes[2], nodes[3]));

        bytes32 merkleRoot = keccak256(abi.encodePacked(nodes[4], nodes[5]));

        bytes32[][] memory proofs = new bytes32[][](4);
        proofs[0] = new bytes32[](2);
        proofs[1] = new bytes32[](2);
        proofs[2] = new bytes32[](2);
        proofs[3] = new bytes32[](2);

        proofs[0][0] = nodes[1];
        proofs[0][1] = nodes[5];
        proofs[1][0] = nodes[0];
        proofs[1][1] = nodes[5];
        proofs[2][0] = nodes[3];
        proofs[2][1] = nodes[4];
        proofs[3][0] = nodes[2];
        proofs[3][1] = nodes[4];

        vm.prank(users.updater);
        rolldown.update_l1_from_l2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 4}));

        vm.startPrank(users.feePayeeA);
        token.approve(address(rolldown), amount - ferryTip);
        rolldown.ferryWithdrawal(withdrawalPayeeA);
        rolldown.closeWithdrawal(withdrawalPayeeA, merkleRoot, proofs[0]);
        token.approve(address(rolldown), amount - ferryTip);
        rolldown.ferryWithdrawal(withdrawalPayeeA2);
        rolldown.closeWithdrawal(withdrawalPayeeA2, merkleRoot, proofs[1]);
        vm.stopPrank();

        vm.startPrank(users.feePayeeB);
        token.approve(address(rolldown), amount - ferryTip);
        rolldown.ferryWithdrawal(withdrawalPayeeB);
        rolldown.closeWithdrawal(withdrawalPayeeB, merkleRoot, proofs[2]);
        vm.stopPrank();

        vm.startPrank(users.feePayeeC);
        token.approve(address(rolldown), amount - ferryTip);
        rolldown.ferryWithdrawal(withdrawalPayeeC);
        rolldown.closeWithdrawal(withdrawalPayeeC, merkleRoot, proofs[3]);
        vm.stopPrank();

        uint256 currentFeePayeeABalance = users.feePayeeA.balance;
        uint256 currentFeePayeeBBalance = users.feePayeeB.balance;
        uint256 currentFeePayeeCBalance = users.feePayeeC.balance;

        assertEq(currentFeePayeeABalance, initialFeePayeeABalance + amount * 2);
        assertEq(currentFeePayeeBBalance, initialFeePayeeBBalance + amount);
        assertEq(currentFeePayeeCBalance, initialFeePayeeCBalance + amount);
        assertTrue(rolldown.processedL2Requests(nodes[0]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(nodes[1]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(nodes[2]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(nodes[3]) == rolldown.CLOSED());
        assertEq(rolldown.lastProcessedUpdate_origin_l2(), 4);
    }

    function test_ExecuteCloseWithdrawalERC20FerryTip() external {
        token.mint(users.updater);
        token.mint(address(rolldown));

        vm.startPrank(users.updater);
        token.approve(address(rolldown), amount);
        rolldown.depositERC20(address(token), amount);
        vm.stopPrank();

        uint256 initialFeePayeeABalance = token.balanceOf(users.feePayeeA);
        uint256 initialFeePayeeBBalance = token.balanceOf(users.feePayeeB);
        uint256 initialFeePayeeCBalance = token.balanceOf(users.feePayeeC);

        IRolldownPrimitives.Withdrawal memory withdrawalPayeeA =
            _createWithdrawal(1, users.feePayeeA, amount, address(token), ferryTip);
        IRolldownPrimitives.Withdrawal memory withdrawalPayeeA2 =
            _createWithdrawal(2, users.feePayeeA, amount, address(token), ferryTip);
        IRolldownPrimitives.Withdrawal memory withdrawalPayeeB =
            _createWithdrawal(3, users.feePayeeB, amount, address(token), ferryTip);
        IRolldownPrimitives.Withdrawal memory withdrawalPayeeC =
            _createWithdrawal(4, users.feePayeeC, amount, address(token), ferryTip);

        bytes32[] memory nodes = new bytes32[](6);
        nodes[0] = rolldown.hashWithdrawal(withdrawalPayeeA);
        nodes[1] = rolldown.hashWithdrawal(withdrawalPayeeA2);
        nodes[2] = rolldown.hashWithdrawal(withdrawalPayeeB);
        nodes[3] = rolldown.hashWithdrawal(withdrawalPayeeC);
        nodes[4] = keccak256(abi.encodePacked(nodes[0], nodes[1]));
        nodes[5] = keccak256(abi.encodePacked(nodes[2], nodes[3]));

        bytes32 merkleRoot = keccak256(abi.encodePacked(nodes[4], nodes[5]));

        bytes32[][] memory proofs = new bytes32[][](4);
        proofs[0] = new bytes32[](2);
        proofs[1] = new bytes32[](2);
        proofs[2] = new bytes32[](2);
        proofs[3] = new bytes32[](2);

        proofs[0][0] = nodes[1];
        proofs[0][1] = nodes[5];
        proofs[1][0] = nodes[0];
        proofs[1][1] = nodes[5];
        proofs[2][0] = nodes[3];
        proofs[2][1] = nodes[4];
        proofs[3][0] = nodes[2];
        proofs[3][1] = nodes[4];

        vm.prank(users.updater);
        rolldown.update_l1_from_l2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 4}));

        vm.startPrank(users.feePayeeA);
        token.approve(address(rolldown), amount - ferryTip);
        rolldown.ferryWithdrawal(withdrawalPayeeA);
        rolldown.closeWithdrawal(withdrawalPayeeA, merkleRoot, proofs[0]);
        rolldown.ferryWithdrawal(withdrawalPayeeA2);
        rolldown.closeWithdrawal(withdrawalPayeeA2, merkleRoot, proofs[1]);
        vm.stopPrank();

        vm.startPrank(users.feePayeeB);
        token.approve(address(rolldown), amount - ferryTip);
        rolldown.ferryWithdrawal(withdrawalPayeeB);
        rolldown.closeWithdrawal(withdrawalPayeeB, merkleRoot, proofs[2]);
        vm.stopPrank();

        vm.startPrank(users.feePayeeC);
        token.approve(address(rolldown), amount - ferryTip);
        rolldown.ferryWithdrawal(withdrawalPayeeC);
        rolldown.closeWithdrawal(withdrawalPayeeC, merkleRoot, proofs[3]);
        vm.stopPrank();

        uint256 currentFeePayeeABalance = token.balanceOf(users.feePayeeA);
        uint256 currentFeePayeeBBalance = token.balanceOf(users.feePayeeB);
        uint256 currentFeePayeeCBalance = token.balanceOf(users.feePayeeC);

        assertEq(currentFeePayeeABalance, initialFeePayeeABalance + amount * 2);
        assertEq(currentFeePayeeBBalance, initialFeePayeeBBalance + amount);
        assertEq(currentFeePayeeCBalance, initialFeePayeeCBalance + amount);
        assertTrue(rolldown.processedL2Requests(nodes[0]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(nodes[1]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(nodes[2]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(nodes[3]) == rolldown.CLOSED());
        assertEq(rolldown.lastProcessedUpdate_origin_l2(), 4);
    }

    function test_RevertIf_Paused() external {
        vm.prank(users.admin);
        rolldown.pause(1);

        IRolldownPrimitives.Withdrawal memory withdrawalPayeeA =
            _createWithdrawal(1, users.feePayeeA, amount, nativeTokenAddress, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalPayeeB =
            _createWithdrawal(2, users.feePayeeB, amount, nativeTokenAddress, 0);

        bytes32 nodeA = rolldown.hashWithdrawal(withdrawalPayeeA);
        bytes32 nodeB = rolldown.hashWithdrawal(withdrawalPayeeB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(nodeA, nodeB));
        bytes32[] memory proof_withdrawalPayeeB = new bytes32[](1);
        proof_withdrawalPayeeB[0] = nodeA;

        vm.startPrank(users.feePayeeB);
        vm.expectRevert("Pausable: contract is paused");
        rolldown.closeWithdrawal(withdrawalPayeeB, merkleRoot, proof_withdrawalPayeeB);
        vm.stopPrank();
    }

    function test_RevertIf_L2RequestAlreadyProcessed() external {
        vm.startPrank(users.updater);
        token.approve(address(rolldown), amount);
        rolldown.depositNative{value: amount}();
        vm.stopPrank();

        IRolldownPrimitives.Withdrawal memory withdrawalPayeeA =
            _createWithdrawal(1, users.feePayeeA, amount, nativeTokenAddress, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalPayeeB =
            _createWithdrawal(2, users.feePayeeB, amount, nativeTokenAddress, 0);

        bytes32 nodeA = rolldown.hashWithdrawal(withdrawalPayeeA);
        bytes32 nodeB = rolldown.hashWithdrawal(withdrawalPayeeB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(nodeA, nodeB));
        bytes32[] memory proof_withdrawalPayeeB = new bytes32[](1);
        proof_withdrawalPayeeB[0] = nodeA;

        vm.prank(users.updater);
        rolldown.update_l1_from_l2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 2}));

        vm.startPrank(users.feePayeeB);
        rolldown.closeWithdrawal(withdrawalPayeeB, merkleRoot, proof_withdrawalPayeeB);
        vm.expectRevert(abi.encodeWithSelector(L2RequestAlreadyProcessed.selector, nodeB));
        rolldown.closeWithdrawal(withdrawalPayeeB, merkleRoot, proof_withdrawalPayeeB);
        vm.stopPrank();
    }

    function test_RevertIf_UnexpectedMerkleRoot() external {
        vm.startPrank(users.updater);
        token.approve(address(rolldown), amount);
        rolldown.depositNative{value: amount}();
        vm.stopPrank();

        IRolldownPrimitives.Withdrawal memory withdrawalPayeeA =
            _createWithdrawal(1, users.feePayeeA, amount, nativeTokenAddress, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalPayeeB =
            _createWithdrawal(2, users.feePayeeB, amount, nativeTokenAddress, 0);

        bytes32 nodeA = rolldown.hashWithdrawal(withdrawalPayeeA);
        bytes32 nodeB = rolldown.hashWithdrawal(withdrawalPayeeB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(nodeA, nodeB));
        bytes32[] memory proof_withdrawalPayeeB = new bytes32[](1);
        proof_withdrawalPayeeB[0] = nodeA;

        vm.prank(users.updater);
        vm.expectRevert(abi.encodeWithSelector(UnexpectedMerkleRoot.selector));
        rolldown.closeWithdrawal(withdrawalPayeeB, merkleRoot, proof_withdrawalPayeeB);
        vm.stopPrank();
    }

    function test_RevertIf_RequestOutOfRange() external {
        vm.startPrank(users.updater);
        token.approve(address(rolldown), amount);
        rolldown.depositNative{value: amount}();
        vm.stopPrank();

        IRolldownPrimitives.Withdrawal memory withdrawalPayeeA =
            _createWithdrawal(2, users.feePayeeA, amount, nativeTokenAddress, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalPayeeB =
            _createWithdrawal(3, users.feePayeeB, amount, nativeTokenAddress, 0);

        bytes32 nodeA = rolldown.hashWithdrawal(withdrawalPayeeA);
        bytes32 nodeB = rolldown.hashWithdrawal(withdrawalPayeeB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(nodeA, nodeB));
        bytes32[] memory proof_withdrawalPayeeB = new bytes32[](1);
        proof_withdrawalPayeeB[0] = nodeA;

        vm.prank(users.updater);
        rolldown.update_l1_from_l2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 2}));

        vm.startPrank(users.feePayeeB);
        vm.expectRevert(abi.encodeWithSelector(RequestOutOfRange.selector, 3, 1, 2));
        rolldown.closeWithdrawal(withdrawalPayeeB, merkleRoot, proof_withdrawalPayeeB);
        vm.stopPrank();
    }

    function test_RevertIf_RequestRangeTooLarge() external {
        vm.startPrank(users.updater);
        token.approve(address(rolldown), amount);
        rolldown.depositNative{value: amount}();
        vm.stopPrank();

        IRolldownPrimitives.Withdrawal memory withdrawalPayeeA =
            _createWithdrawal(1, users.feePayeeA, amount, nativeTokenAddress, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalPayeeB =
            _createWithdrawal(2, users.feePayeeB, amount, nativeTokenAddress, 0);

        bytes32 nodeA = rolldown.hashWithdrawal(withdrawalPayeeA);
        bytes32 nodeB = rolldown.hashWithdrawal(withdrawalPayeeB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(nodeA, nodeB));
        bytes32[] memory proof_withdrawalPayeeB = new bytes32[](1);
        proof_withdrawalPayeeB[0] = nodeA;

        vm.prank(users.updater);
        rolldown.update_l1_from_l2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: type(uint64).max}));

        vm.startPrank(users.feePayeeB);
        vm.expectRevert(abi.encodeWithSelector(RequestRangeTooLarge.selector, type(uint64).max));
        rolldown.closeWithdrawal(withdrawalPayeeB, merkleRoot, proof_withdrawalPayeeB);
        vm.stopPrank();
    }

    function test_RevertIf_InvalidRequestProof() external {
        vm.startPrank(users.updater);
        token.approve(address(rolldown), amount);
        rolldown.depositNative{value: amount}();
        vm.stopPrank();

        IRolldownPrimitives.Withdrawal memory withdrawalPayeeA =
            _createWithdrawal(1, users.feePayeeA, amount, nativeTokenAddress, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalPayeeB =
            _createWithdrawal(2, users.feePayeeB, amount, nativeTokenAddress, 0);

        bytes32 nodeA = rolldown.hashWithdrawal(withdrawalPayeeA);
        bytes32 nodeB = rolldown.hashWithdrawal(withdrawalPayeeB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(nodeA, nodeB));
        bytes32[] memory proof_withdrawalPayeeB = new bytes32[](1);
        proof_withdrawalPayeeB[0] = nodeB;

        vm.prank(users.updater);
        rolldown.update_l1_from_l2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 2}));

        vm.startPrank(users.feePayeeB);
        vm.expectRevert(abi.encodeWithSelector(InvalidRequestProof.selector, merkleRoot));
        rolldown.closeWithdrawal(withdrawalPayeeB, merkleRoot, proof_withdrawalPayeeB);
        vm.stopPrank();
    }

    function test_RevertIf_ZeroTransferAmountNative() external {
        vm.startPrank(users.updater);
        token.approve(address(rolldown), amount);
        rolldown.depositNative{value: amount}();
        vm.stopPrank();

        IRolldownPrimitives.Withdrawal memory withdrawalPayeeA =
            _createWithdrawal(1, users.feePayeeA, 0, nativeTokenAddress, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalPayeeB =
            _createWithdrawal(2, users.feePayeeB, 0, nativeTokenAddress, 0);

        bytes32 nodeA = rolldown.hashWithdrawal(withdrawalPayeeA);
        bytes32 nodeB = rolldown.hashWithdrawal(withdrawalPayeeB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(nodeA, nodeB));
        bytes32[] memory proof_withdrawalPayeeB = new bytes32[](1);
        proof_withdrawalPayeeB[0] = nodeA;

        vm.prank(users.updater);
        rolldown.update_l1_from_l2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 2}));

        vm.startPrank(users.feePayeeB);
        vm.expectRevert(abi.encodeWithSelector(ZeroTransferAmount.selector));
        rolldown.closeWithdrawal(withdrawalPayeeB, merkleRoot, proof_withdrawalPayeeB);
        vm.stopPrank();
    }

    function test_RevertIf_ZeroTransferAmountERC20() external {
        token.mint(users.updater);
        token.mint(address(rolldown));

        vm.startPrank(users.updater);
        token.approve(address(rolldown), amount);
        rolldown.depositERC20(address(token), amount);
        vm.stopPrank();

        IRolldownPrimitives.Withdrawal memory withdrawalPayeeA =
            _createWithdrawal(1, users.feePayeeA, 0, address(token), 0);
        IRolldownPrimitives.Withdrawal memory withdrawalPayeeB =
            _createWithdrawal(2, users.feePayeeB, 0, address(token), 0);

        bytes32 nodeA = rolldown.hashWithdrawal(withdrawalPayeeA);
        bytes32 nodeB = rolldown.hashWithdrawal(withdrawalPayeeB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(nodeA, nodeB));
        bytes32[] memory proof_withdrawalPayeeB = new bytes32[](1);
        proof_withdrawalPayeeB[0] = nodeA;

        vm.prank(users.updater);
        rolldown.update_l1_from_l2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 2}));

        vm.startPrank(users.feePayeeB);
        vm.expectRevert(abi.encodeWithSelector(ZeroTransferAmount.selector));
        rolldown.closeWithdrawal(withdrawalPayeeB, merkleRoot, proof_withdrawalPayeeB);
        vm.stopPrank();
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {EmptyContract} from "@eigenlayer/test/mocks/EmptyContract.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {Test} from "forge-std/Test.sol";
import {IRolldown} from "../src/IRolldown.sol";
import {IRolldownPrimitives} from "../src/IRolldownPrimitives.sol";
import {Rolldown} from "../src/Rolldown.sol";
import {MyERC20} from "./utils/Utilities.sol";

contract RolldownTest is Test, IRolldownPrimitives {
    struct Users {
        address admin;
        address depositor;
        address grantee;
        address recipient;
        address updater;
        address withdrawerA;
        address withdrawerB;
        address withdrawerC;
    }

    Users public users;
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
            depositor: makeAddr("depositor"),
            grantee: makeAddr("grantee"),
            updater: makeAddr("updater"),
            recipient: makeAddr("recipient"),
            withdrawerA: makeAddr("withdrawerA"),
            withdrawerB: makeAddr("withdrawerB"),
            withdrawerC: makeAddr("withdrawerC")
        });

        deal(payable(users.admin), 100 ether);
        deal(payable(users.depositor), 100 ether);
        deal(payable(users.grantee), 100 ether);
        deal(payable(users.updater), 100 ether);
        deal(payable(users.recipient), 100 ether);
        deal(payable(users.withdrawerA), 100 ether);
        deal(payable(users.withdrawerB), 100 ether);
        deal(payable(users.withdrawerC), 100 ether);

        token = new MyERC20();

        Rolldown implementation = new Rolldown();
        ProxyAdmin proxyAdmin = new ProxyAdmin();
        TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(
            address(implementation),
            address(proxyAdmin),
            abi.encodeWithSelector(
                Rolldown.initialize.selector, users.admin, IRolldownPrimitives.ChainId.Ethereum, users.updater
            )
        );
        rolldown = Rolldown(address(proxy));

        updaterRole = rolldown.UPDATER_ROLE();
        nativeTokenAddress = rolldown.NATIVE_TOKEN_ADDRESS();
    }

    function _createWithdrawal(uint256 id, address recipient, address tokenAddress, uint256 amount, uint256 ferryTip)
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
        rolldown.initialize(users.admin, IRolldownPrimitives.ChainId.Ethereum, users.updater);
    }

    function test_RevertIf_ZeroAdmin() external {
        Rolldown implementation = new Rolldown();
        ProxyAdmin proxyAdmin = new ProxyAdmin();

        vm.expectRevert(abi.encodeWithSelector(ZeroAdmin.selector));
        new TransparentUpgradeableProxy(
            address(implementation),
            address(proxyAdmin),
            abi.encodeWithSelector(
                Rolldown.initialize.selector, address(0), IRolldownPrimitives.ChainId.Ethereum, users.updater
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
                Rolldown.initialize.selector, users.admin, IRolldownPrimitives.ChainId.Ethereum, address(0)
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

        vm.prank(users.depositor);
        vm.expectRevert(
            "AccessControl: account 0x1ffc33f5e217b1cf95e713db49fcd86c8195666c is missing role 0x0000000000000000000000000000000000000000000000000000000000000000"
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
        vm.prank(users.depositor);
        vm.expectRevert(
            "AccessControl: account 0x1ffc33f5e217b1cf95e713db49fcd86c8195666c is missing role 0x0000000000000000000000000000000000000000000000000000000000000000"
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

        vm.prank(users.depositor);
        vm.expectEmit();
        emit DepositAcceptedIntoQueue(requestId, users.depositor, nativeTokenAddress, amount, ferryTip);
        rolldown.depositNative{value: amount}();
    }

    function test_ChangeBalances() external {
        uint256 initialDepositorBalance = users.depositor.balance;
        uint256 initialRolldownBalance = address(rolldown).balance;

        vm.prank(users.depositor);
        rolldown.depositNative{value: amount}();

        uint256 currentDepositorBalance = users.depositor.balance;
        assertEq(initialDepositorBalance - currentDepositorBalance, amount);

        uint256 currentRolldownBalance = address(rolldown).balance;
        assertEq(currentRolldownBalance - initialRolldownBalance, amount);
    }

    function test_GetUpdateForL2() external {
        uint256 requestId = rolldown.counter();

        vm.prank(users.depositor);
        rolldown.depositNative{value: amount}();

        Rolldown.L1Update memory l1Update = rolldown.getUpdateForL2();
        assertEq(l1Update.pendingDeposits.length, 1);

        Rolldown.Deposit memory pendingDeposit = l1Update.pendingDeposits[0];
        assertEq(pendingDeposit.requestId.id, requestId);
        assertEq(pendingDeposit.depositRecipient, users.depositor);
        assertEq(pendingDeposit.tokenAddress, nativeTokenAddress);
        assertEq(pendingDeposit.amount, amount);
        assertEq(pendingDeposit.ferryTip, ferryTip);
    }

    function test_RevertIf_Paused() external {
        vm.prank(users.admin);
        rolldown.pause();

        vm.prank(users.depositor);
        vm.expectRevert("Pausable: paused");
        rolldown.depositNative{value: amount}();
    }

    function test_RevertIf_ZeroAmount() external {
        vm.prank(users.depositor);
        vm.expectRevert(abi.encodeWithSelector(ZeroAmount.selector));
        rolldown.depositNative{value: 0}();
    }
}

contract DepositNativeWithFerryTip is RolldownTest {
    uint256 public amount = 1 ether;
    uint256 public ferryTip = amount / 100;

    function test_EmitDepositAcceptedIntoQueue() external {
        uint256 requestId = rolldown.counter();

        vm.prank(users.depositor);
        vm.expectEmit();
        emit DepositAcceptedIntoQueue(requestId, users.depositor, nativeTokenAddress, amount, ferryTip);
        rolldown.depositNative{value: amount}(ferryTip);
    }

    function test_ChangeBalances() external {
        uint256 initialDepositorBalance = users.depositor.balance;
        uint256 initialRolldownBalance = address(rolldown).balance;

        vm.prank(users.depositor);
        rolldown.depositNative{value: amount}(ferryTip);

        uint256 currentDepositorBalance = users.depositor.balance;
        assertEq(initialDepositorBalance - currentDepositorBalance, amount);

        uint256 currentRolldownBalance = address(rolldown).balance;
        assertEq(currentRolldownBalance - initialRolldownBalance, amount);
    }

    function test_GetUpdateForL2() external {
        uint256 requestId = rolldown.counter();

        vm.prank(users.depositor);
        rolldown.depositNative{value: amount}(ferryTip);

        Rolldown.L1Update memory l1Update = rolldown.getUpdateForL2();
        assertEq(l1Update.pendingDeposits.length, 1);

        Rolldown.Deposit memory pendingDeposit = l1Update.pendingDeposits[0];
        assertEq(pendingDeposit.requestId.id, requestId);
        assertEq(pendingDeposit.depositRecipient, users.depositor);
        assertEq(pendingDeposit.tokenAddress, nativeTokenAddress);
        assertEq(pendingDeposit.amount, amount);
        assertEq(pendingDeposit.ferryTip, ferryTip);
    }

    function test_RevertIf_Paused() external {
        vm.prank(users.admin);
        rolldown.pause();

        vm.prank(users.depositor);
        vm.expectRevert("Pausable: paused");
        rolldown.depositNative{value: amount}(ferryTip);
    }

    function test_RevertIf_ZeroAmount() external {
        vm.prank(users.depositor);
        vm.expectRevert(abi.encodeWithSelector(ZeroAmount.selector));
        rolldown.depositNative{value: 0}(ferryTip);
    }

    function test_RevertIf_FerryTipExceedsAmount() external {
        ferryTip = amount + 1;

        vm.prank(users.depositor);
        vm.expectRevert(abi.encodeWithSelector(FerryTipExceedsAmount.selector, ferryTip, amount));
        rolldown.depositNative{value: amount}(ferryTip);
    }
}

contract DepositERC20 is RolldownTest {
    uint256 public amount = 1 ether;
    uint256 public ferryTip = 0;

    function test_EmitDepositAcceptedIntoQueue() external {
        vm.startPrank(users.depositor);

        uint256 requestId = rolldown.counter();
        token.mint(users.depositor);
        token.approve(address(rolldown), amount);

        vm.expectEmit();
        emit DepositAcceptedIntoQueue(requestId, users.depositor, address(token), amount, ferryTip);
        rolldown.depositERC20(address(token), amount);

        vm.stopPrank();
    }

    function test_ChangeBalances() external {
        token.mint(users.depositor);
        uint256 initialDepositorBalance = token.balanceOf(users.depositor);
        uint256 initialRolldownBalance = token.balanceOf(address(rolldown));

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount);
        rolldown.depositERC20(address(token), amount);

        vm.stopPrank();

        uint256 currentDepositorBalance = token.balanceOf(users.depositor);
        assertEq(initialDepositorBalance - currentDepositorBalance, amount);

        uint256 currentRolldownBalance = token.balanceOf(address(rolldown));
        assertEq(currentRolldownBalance - initialRolldownBalance, amount);
    }

    function test_GetUpdateForL2() external {
        token.mint(users.depositor);
        uint256 requestId = rolldown.counter();

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount);
        rolldown.depositERC20(address(token), amount);

        vm.stopPrank();

        Rolldown.L1Update memory l1Update = rolldown.getUpdateForL2();
        assertEq(l1Update.pendingDeposits.length, 1);

        Rolldown.Deposit memory pendingDeposit = l1Update.pendingDeposits[0];
        assertEq(pendingDeposit.requestId.id, requestId);
        assertEq(pendingDeposit.depositRecipient, users.depositor);
        assertEq(pendingDeposit.tokenAddress, address(token));
        assertEq(pendingDeposit.amount, amount);
        assertEq(pendingDeposit.ferryTip, ferryTip);
    }

    function test_RevertIf_Paused() external {
        vm.prank(users.admin);
        rolldown.pause();

        vm.prank(users.depositor);
        vm.expectRevert("Pausable: paused");
        rolldown.depositERC20(address(token), amount);
    }

    function test_RevertIf_ZeroAmount() external {
        vm.prank(users.depositor);
        vm.expectRevert(abi.encodeWithSelector(ZeroAmount.selector));
        rolldown.depositERC20(address(token), 0);
    }

    function test_RevertIf_ZeroToken() external {
        vm.prank(users.depositor);
        vm.expectRevert(abi.encodeWithSelector(ZeroToken.selector));
        rolldown.depositERC20(address(0), amount);
    }
}

contract DepositERC20WithFerryTip is RolldownTest {
    uint256 public amount = 1 ether;
    uint256 public ferryTip = amount / 100;

    function test_EmitDepositAcceptedIntoQueue() external {
        vm.startPrank(users.depositor);

        uint256 requestId = rolldown.counter();
        token.mint(users.depositor);
        token.approve(address(rolldown), amount);

        vm.expectEmit();
        emit DepositAcceptedIntoQueue(requestId, users.depositor, address(token), amount, ferryTip);
        rolldown.depositERC20(address(token), amount, ferryTip);

        vm.stopPrank();
    }

    function test_ChangeBalances() external {
        token.mint(users.depositor);
        uint256 initialDepositorBalance = token.balanceOf(users.depositor);
        uint256 initialRolldownBalance = token.balanceOf(address(rolldown));

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount);
        rolldown.depositERC20(address(token), amount, ferryTip);

        vm.stopPrank();

        uint256 currentDepositorBalance = token.balanceOf(users.depositor);
        assertEq(initialDepositorBalance - currentDepositorBalance, amount);

        uint256 currentRolldownBalance = token.balanceOf(address(rolldown));
        assertEq(currentRolldownBalance - initialRolldownBalance, amount);
    }

    function test_GetUpdateForL2() external {
        token.mint(users.depositor);
        uint256 requestId = rolldown.counter();

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount);
        rolldown.depositERC20(address(token), amount, ferryTip);

        vm.stopPrank();

        Rolldown.L1Update memory l1Update = rolldown.getUpdateForL2();
        assertEq(l1Update.pendingDeposits.length, 1);

        Rolldown.Deposit memory pendingDeposit = l1Update.pendingDeposits[0];
        assertEq(pendingDeposit.requestId.id, requestId);
        assertEq(pendingDeposit.depositRecipient, users.depositor);
        assertEq(pendingDeposit.tokenAddress, address(token));
        assertEq(pendingDeposit.amount, amount);
        assertEq(pendingDeposit.ferryTip, ferryTip);
    }

    function test_RevertIf_Paused() external {
        vm.prank(users.admin);
        rolldown.pause();

        vm.prank(users.depositor);
        vm.expectRevert("Pausable: paused");
        rolldown.depositERC20(address(token), amount, ferryTip);
    }

    function test_RevertIf_ZeroAmount() external {
        vm.prank(users.depositor);
        vm.expectRevert(abi.encodeWithSelector(ZeroAmount.selector));
        rolldown.depositERC20(address(token), 0, ferryTip);
    }

    function test_RevertIf_ZeroToken() external {
        vm.prank(users.depositor);
        vm.expectRevert(abi.encodeWithSelector(ZeroToken.selector));
        rolldown.depositERC20(address(0), amount, ferryTip);
    }

    function test_RevertIf_FerryTipExceedsAmount() external {
        ferryTip = amount + 1;

        vm.prank(users.depositor);
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

        vm.prank(users.depositor);
        vm.expectEmit();
        emit WithdrawalFerried(withdrawal.requestId.id, ferriedAmount, users.recipient, users.depositor, withdrawalHash);
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
    }

    function test_EmitWithdrawalFerriedERC20() external {
        token.mint(users.depositor);
        token.mint(address(rolldown));

        IRolldownPrimitives.Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: _createRequestId(),
            recipient: users.recipient,
            tokenAddress: address(token),
            amount: amount,
            ferryTip: ferryTip
        });

        bytes32 withdrawalHash = rolldown.hashWithdrawal(withdrawal);

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), ferriedAmount);

        vm.expectEmit();
        emit WithdrawalFerried(withdrawal.requestId.id, ferriedAmount, users.recipient, users.depositor, withdrawalHash);
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

        uint256 intitalExecutorBalance = users.depositor.balance;

        vm.prank(users.depositor);
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);

        uint256 currentRecipientBalance = users.recipient.balance;
        assertEq(currentRecipientBalance, intitalExecutorBalance + ferriedAmount);

        uint256 currentDepositorBalance = users.depositor.balance;
        assertEq(currentDepositorBalance, intitalExecutorBalance - amount + ferryTip);
    }

    function test_ChangeBalancesERC20() external {
        token.mint(users.depositor);
        token.mint(address(rolldown));

        IRolldownPrimitives.Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: _createRequestId(),
            recipient: users.recipient,
            tokenAddress: address(token),
            amount: amount,
            ferryTip: ferryTip
        });

        uint256 intitalExecutorBalance = token.balanceOf(users.depositor);

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), ferriedAmount);
        rolldown.ferryWithdrawal(withdrawal);

        vm.stopPrank();

        uint256 currentRecipientBalance = token.balanceOf(users.recipient);
        assertEq(currentRecipientBalance, ferriedAmount);

        uint256 currentDepositorBalance = token.balanceOf(users.depositor);
        assertEq(currentDepositorBalance, intitalExecutorBalance - amount + ferryTip);
    }

    function test_RevertIf_Paused() external {
        vm.prank(users.admin);
        rolldown.pause();

        IRolldownPrimitives.Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: _createRequestId(),
            recipient: users.recipient,
            tokenAddress: address(token),
            amount: amount,
            ferryTip: ferryTip
        });

        vm.prank(users.depositor);
        vm.expectRevert("Pausable: paused");
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

        vm.prank(users.depositor);
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

        vm.prank(users.depositor);
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

        vm.prank(users.depositor);
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

        vm.prank(users.depositor);
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

        vm.prank(users.depositor);
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

        vm.startPrank(users.depositor);

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

        vm.prank(users.depositor);
        vm.expectRevert("Address: call to non-contract");
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
    }

    function test_RevertIf_TransferOperationFailed() external {
        IRolldownPrimitives.Withdrawal memory withdrawal = IRolldownPrimitives.Withdrawal({
            requestId: _createRequestId(),
            recipient: users.recipient,
            tokenAddress: address(new EmptyContract()),
            amount: amount,
            ferryTip: ferryTip
        });

        vm.prank(users.depositor);
        vm.expectRevert("SafeERC20: low-level call failed");
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
    }

    function _createRequestId() private view returns (IRolldownPrimitives.RequestId memory) {
        uint256 requestId = rolldown.counter();
        return IRolldownPrimitives.RequestId({id: requestId, origin: IRolldownPrimitives.Origin.L2});
    }
}

contract CloseWithdrawal is RolldownTest {
    uint256 public amount = 10;
    uint256 public ferryTip = 10;

    function test_EmitNativeTokensWithdrawn() external {
        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount);
        rolldown.depositNative{value: amount}();

        vm.stopPrank();

        IRolldownPrimitives.Withdrawal memory withdrawalA =
            _createWithdrawal(1, users.withdrawerA, nativeTokenAddress, amount, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalB =
            _createWithdrawal(2, users.withdrawerB, nativeTokenAddress, amount, 0);

        bytes32 nodeA = rolldown.hashWithdrawal(withdrawalA);
        bytes32 nodeB = rolldown.hashWithdrawal(withdrawalB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(nodeA, nodeB));
        bytes32[] memory merkleProof = new bytes32[](1);
        merkleProof[0] = nodeA;

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 2}));

        vm.prank(users.withdrawerB);
        vm.expectEmit();
        emit NativeTokensWithdrawn(users.withdrawerB, amount);
        emit WithdrawalClosed(2, nodeB);
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, merkleProof);
    }

    function test_EmitERC20TokensWithdrawn() external {
        token.mint(users.depositor);

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount);
        rolldown.depositERC20(address(token), amount);

        vm.stopPrank();

        IRolldownPrimitives.Withdrawal memory withdrawalA =
            _createWithdrawal(1, users.withdrawerA, address(token), amount, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalB =
            _createWithdrawal(2, users.withdrawerB, address(token), amount, 0);

        bytes32 nodeA = rolldown.hashWithdrawal(withdrawalA);
        bytes32 nodeB = rolldown.hashWithdrawal(withdrawalB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(nodeA, nodeB));
        bytes32[] memory merkleProof = new bytes32[](1);
        merkleProof[0] = nodeA;

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 2}));

        vm.prank(users.withdrawerB);
        vm.expectEmit();
        emit ERC20TokensWithdrawn(users.withdrawerB, address(token), amount);
        emit Transfer(address(rolldown), users.withdrawerB, amount);
        emit WithdrawalClosed(2, nodeB);
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, merkleProof);
    }

    function test_EmitNativeTokensWithdrawnFerryTip() external {
        vm.prank(users.depositor);
        rolldown.depositNative{value: amount}();

        IRolldownPrimitives.Withdrawal memory withdrawalA =
            _createWithdrawal(1, users.withdrawerA, nativeTokenAddress, amount, ferryTip);
        IRolldownPrimitives.Withdrawal memory withdrawalB =
            _createWithdrawal(2, users.withdrawerB, nativeTokenAddress, amount, ferryTip);

        bytes32 nodeA = rolldown.hashWithdrawal(withdrawalA);
        bytes32 nodeB = rolldown.hashWithdrawal(withdrawalB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(nodeA, nodeB));
        bytes32[] memory merkleProof = new bytes32[](1);
        merkleProof[0] = nodeA;

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 2}));

        vm.startPrank(users.withdrawerB);

        rolldown.ferryWithdrawal{value: amount - ferryTip}(withdrawalB);

        vm.expectEmit();
        emit NativeTokensWithdrawn(users.withdrawerB, amount);
        emit FerriedWithdrawalClosed(2, nodeB);
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, merkleProof);

        vm.stopPrank();
    }

    function test_EmitERC20TokensWithdrawnFerryTip() external {
        token.mint(users.depositor);

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount);
        rolldown.depositERC20(address(token), amount);

        vm.stopPrank();

        IRolldownPrimitives.Withdrawal memory withdrawalA =
            _createWithdrawal(1, users.withdrawerA, address(token), amount, ferryTip);
        IRolldownPrimitives.Withdrawal memory withdrawalB =
            _createWithdrawal(2, users.withdrawerB, address(token), amount, ferryTip);

        bytes32 nodeA = rolldown.hashWithdrawal(withdrawalA);
        bytes32 nodeB = rolldown.hashWithdrawal(withdrawalB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(nodeA, nodeB));
        bytes32[] memory merkleProof = new bytes32[](1);
        merkleProof[0] = nodeA;

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 2}));

        vm.startPrank(users.withdrawerB);

        token.approve(address(rolldown), amount - ferryTip);
        rolldown.ferryWithdrawal(withdrawalB);

        vm.expectEmit();
        emit ERC20TokensWithdrawn(users.withdrawerB, address(token), amount);
        emit Transfer(address(rolldown), users.withdrawerB, amount);
        emit FerriedWithdrawalClosed(2, nodeB);
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, merkleProof);

        vm.stopPrank();
    }

    function test_ExecuteCloseWithdrawalNative() external {
        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount * 4);
        rolldown.depositNative{value: amount * 4}();

        vm.stopPrank();

        uint256 initialPayeeABalance = users.withdrawerA.balance;
        uint256 initialPayeeBBalance = users.withdrawerB.balance;
        uint256 initialPayeeCBalance = users.withdrawerC.balance;

        IRolldownPrimitives.Withdrawal memory withdrawalA =
            _createWithdrawal(1, users.withdrawerA, nativeTokenAddress, amount, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalA2 =
            _createWithdrawal(2, users.withdrawerA, nativeTokenAddress, amount, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalB =
            _createWithdrawal(3, users.withdrawerB, nativeTokenAddress, amount, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalC =
            _createWithdrawal(4, users.withdrawerC, nativeTokenAddress, amount, 0);

        bytes32[] memory nodes = new bytes32[](6);
        nodes[0] = rolldown.hashWithdrawal(withdrawalA);
        nodes[1] = rolldown.hashWithdrawal(withdrawalA2);
        nodes[2] = rolldown.hashWithdrawal(withdrawalB);
        nodes[3] = rolldown.hashWithdrawal(withdrawalC);
        nodes[4] = keccak256(abi.encodePacked(nodes[0], nodes[1]));
        nodes[5] = keccak256(abi.encodePacked(nodes[2], nodes[3]));

        bytes32 merkleRoot = keccak256(abi.encodePacked(nodes[4], nodes[5]));

        bytes32[][] memory proofs = new bytes32[][](4);
        proofs[0] = new bytes32[](2);
        proofs[0][0] = nodes[1];
        proofs[0][1] = nodes[5];
        proofs[1] = new bytes32[](2);
        proofs[1][0] = nodes[0];
        proofs[1][1] = nodes[5];
        proofs[2] = new bytes32[](2);
        proofs[2][0] = nodes[3];
        proofs[2][1] = nodes[4];
        proofs[3] = new bytes32[](2);
        proofs[3][0] = nodes[2];
        proofs[3][1] = nodes[4];

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 4}));

        vm.startPrank(users.withdrawerA);

        rolldown.closeWithdrawal(withdrawalA, merkleRoot, proofs[0]);
        rolldown.closeWithdrawal(withdrawalA2, merkleRoot, proofs[1]);

        vm.stopPrank();

        vm.prank(users.withdrawerB);
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, proofs[2]);

        vm.prank(users.withdrawerC);
        rolldown.closeWithdrawal(withdrawalC, merkleRoot, proofs[3]);

        uint256 currentPayeeABalance = users.withdrawerA.balance;
        assertEq(currentPayeeABalance, initialPayeeABalance + amount * 2);

        uint256 currentPayeeBBalance = users.withdrawerB.balance;
        assertEq(currentPayeeBBalance, initialPayeeBBalance + amount);

        uint256 currentPayeeCBalance = users.withdrawerC.balance;
        assertEq(currentPayeeCBalance, initialPayeeCBalance + amount);

        assertTrue(rolldown.processedL2Requests(nodes[0]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(nodes[1]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(nodes[2]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(nodes[3]) == rolldown.CLOSED());
        assertEq(rolldown.lastProcessedUpdate_origin_l2(), 4);
    }

    function test_ExecuteCloseWithdrawalERC20() external {
        token.mint(users.depositor);
        token.mint(address(rolldown));

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount);
        rolldown.depositERC20(address(token), amount);

        vm.stopPrank();

        uint256 initialPayeeABalance = token.balanceOf(users.withdrawerA);
        uint256 initialPayeeBBalance = token.balanceOf(users.withdrawerB);
        uint256 initialPayeeCBalance = token.balanceOf(users.withdrawerC);

        IRolldownPrimitives.Withdrawal memory withdrawalA =
            _createWithdrawal(1, users.withdrawerA, address(token), amount, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalA2 =
            _createWithdrawal(2, users.withdrawerA, address(token), amount, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalB =
            _createWithdrawal(3, users.withdrawerB, address(token), amount, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalC =
            _createWithdrawal(4, users.withdrawerC, address(token), amount, 0);

        bytes32[] memory nodes = new bytes32[](6);
        nodes[0] = rolldown.hashWithdrawal(withdrawalA);
        nodes[1] = rolldown.hashWithdrawal(withdrawalA2);
        nodes[2] = rolldown.hashWithdrawal(withdrawalB);
        nodes[3] = rolldown.hashWithdrawal(withdrawalC);
        nodes[4] = keccak256(abi.encodePacked(nodes[0], nodes[1]));
        nodes[5] = keccak256(abi.encodePacked(nodes[2], nodes[3]));

        bytes32 merkleRoot = keccak256(abi.encodePacked(nodes[4], nodes[5]));

        bytes32[][] memory proofs = new bytes32[][](4);
        proofs[0] = new bytes32[](2);
        proofs[0][0] = nodes[1];
        proofs[0][1] = nodes[5];
        proofs[1] = new bytes32[](2);
        proofs[1][0] = nodes[0];
        proofs[1][1] = nodes[5];
        proofs[2] = new bytes32[](2);
        proofs[2][0] = nodes[3];
        proofs[2][1] = nodes[4];
        proofs[3] = new bytes32[](2);
        proofs[3][0] = nodes[2];
        proofs[3][1] = nodes[4];

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 4}));

        vm.startPrank(users.withdrawerA);

        rolldown.closeWithdrawal(withdrawalA, merkleRoot, proofs[0]);
        rolldown.closeWithdrawal(withdrawalA2, merkleRoot, proofs[1]);

        vm.stopPrank();

        vm.prank(users.withdrawerB);
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, proofs[2]);

        vm.prank(users.withdrawerC);
        rolldown.closeWithdrawal(withdrawalC, merkleRoot, proofs[3]);

        uint256 currentPayeeABalance = token.balanceOf(users.withdrawerA);
        assertEq(currentPayeeABalance, initialPayeeABalance + amount * 2);

        uint256 currentPayeeBBalance = token.balanceOf(users.withdrawerB);
        assertEq(currentPayeeBBalance, initialPayeeBBalance + amount);

        uint256 currentPayeeCBalance = token.balanceOf(users.withdrawerC);
        assertEq(currentPayeeCBalance, initialPayeeCBalance + amount);

        assertTrue(rolldown.processedL2Requests(nodes[0]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(nodes[1]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(nodes[2]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(nodes[3]) == rolldown.CLOSED());
        assertEq(rolldown.lastProcessedUpdate_origin_l2(), 4);
    }

    function test_ExecuteCloseWithdrawalNativeFerryTip() external {
        vm.prank(users.depositor);
        rolldown.depositNative{value: amount * 4}();

        uint256 initialPayeeABalance = users.withdrawerA.balance;
        uint256 initialPayeeBBalance = users.withdrawerB.balance;
        uint256 initialPayeeCBalance = users.withdrawerC.balance;

        IRolldownPrimitives.Withdrawal memory withdrawalA =
            _createWithdrawal(1, users.withdrawerA, nativeTokenAddress, amount, ferryTip);
        IRolldownPrimitives.Withdrawal memory withdrawalA2 =
            _createWithdrawal(2, users.withdrawerA, nativeTokenAddress, amount, ferryTip);
        IRolldownPrimitives.Withdrawal memory withdrawalB =
            _createWithdrawal(3, users.withdrawerB, nativeTokenAddress, amount, ferryTip);
        IRolldownPrimitives.Withdrawal memory withdrawalC =
            _createWithdrawal(4, users.withdrawerC, nativeTokenAddress, amount, ferryTip);

        bytes32[] memory nodes = new bytes32[](6);
        nodes[0] = rolldown.hashWithdrawal(withdrawalA);
        nodes[1] = rolldown.hashWithdrawal(withdrawalA2);
        nodes[2] = rolldown.hashWithdrawal(withdrawalB);
        nodes[3] = rolldown.hashWithdrawal(withdrawalC);
        nodes[4] = keccak256(abi.encodePacked(nodes[0], nodes[1]));
        nodes[5] = keccak256(abi.encodePacked(nodes[2], nodes[3]));

        bytes32 merkleRoot = keccak256(abi.encodePacked(nodes[4], nodes[5]));

        bytes32[][] memory proofs = new bytes32[][](4);
        proofs[0] = new bytes32[](2);
        proofs[0][0] = nodes[1];
        proofs[0][1] = nodes[5];
        proofs[1] = new bytes32[](2);
        proofs[1][0] = nodes[0];
        proofs[1][1] = nodes[5];
        proofs[2] = new bytes32[](2);
        proofs[2][0] = nodes[3];
        proofs[2][1] = nodes[4];
        proofs[3] = new bytes32[](2);
        proofs[3][0] = nodes[2];
        proofs[3][1] = nodes[4];

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 4}));

        vm.startPrank(users.withdrawerA);

        rolldown.ferryWithdrawal(withdrawalA);
        rolldown.closeWithdrawal(withdrawalA, merkleRoot, proofs[0]);
        rolldown.ferryWithdrawal(withdrawalA2);
        rolldown.closeWithdrawal(withdrawalA2, merkleRoot, proofs[1]);

        vm.stopPrank();

        vm.startPrank(users.withdrawerB);

        token.approve(address(rolldown), amount - ferryTip);
        rolldown.ferryWithdrawal(withdrawalB);
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, proofs[2]);

        vm.stopPrank();

        vm.startPrank(users.withdrawerC);

        token.approve(address(rolldown), amount - ferryTip);
        rolldown.ferryWithdrawal(withdrawalC);
        rolldown.closeWithdrawal(withdrawalC, merkleRoot, proofs[3]);

        vm.stopPrank();

        uint256 currentPayeeABalance = users.withdrawerA.balance;
        assertEq(currentPayeeABalance, initialPayeeABalance + amount * 2);

        uint256 currentPayeeBBalance = users.withdrawerB.balance;
        assertEq(currentPayeeBBalance, initialPayeeBBalance + amount);

        uint256 currentPayeeCBalance = users.withdrawerC.balance;
        assertEq(currentPayeeCBalance, initialPayeeCBalance + amount);

        assertTrue(rolldown.processedL2Requests(nodes[0]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(nodes[1]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(nodes[2]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(nodes[3]) == rolldown.CLOSED());
        assertEq(rolldown.lastProcessedUpdate_origin_l2(), 4);
    }

    function test_ExecuteCloseWithdrawalERC20FerryTip() external {
        token.mint(users.depositor);
        token.mint(address(rolldown));

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount);
        rolldown.depositERC20(address(token), amount);

        vm.stopPrank();

        uint256 initialPayeeABalance = token.balanceOf(users.withdrawerA);
        uint256 initialPayeeBBalance = token.balanceOf(users.withdrawerB);
        uint256 initialPayeeCBalance = token.balanceOf(users.withdrawerC);

        IRolldownPrimitives.Withdrawal memory withdrawalA =
            _createWithdrawal(1, users.withdrawerA, address(token), amount, ferryTip);
        IRolldownPrimitives.Withdrawal memory withdrawalA2 =
            _createWithdrawal(2, users.withdrawerA, address(token), amount, ferryTip);
        IRolldownPrimitives.Withdrawal memory withdrawalB =
            _createWithdrawal(3, users.withdrawerB, address(token), amount, ferryTip);
        IRolldownPrimitives.Withdrawal memory withdrawalC =
            _createWithdrawal(4, users.withdrawerC, address(token), amount, ferryTip);

        bytes32[] memory nodes = new bytes32[](6);
        nodes[0] = rolldown.hashWithdrawal(withdrawalA);
        nodes[1] = rolldown.hashWithdrawal(withdrawalA2);
        nodes[2] = rolldown.hashWithdrawal(withdrawalB);
        nodes[3] = rolldown.hashWithdrawal(withdrawalC);
        nodes[4] = keccak256(abi.encodePacked(nodes[0], nodes[1]));
        nodes[5] = keccak256(abi.encodePacked(nodes[2], nodes[3]));

        bytes32 merkleRoot = keccak256(abi.encodePacked(nodes[4], nodes[5]));

        bytes32[][] memory proofs = new bytes32[][](4);
        proofs[0] = new bytes32[](2);
        proofs[0][0] = nodes[1];
        proofs[0][1] = nodes[5];
        proofs[1] = new bytes32[](2);
        proofs[1][0] = nodes[0];
        proofs[1][1] = nodes[5];
        proofs[2] = new bytes32[](2);
        proofs[2][0] = nodes[3];
        proofs[2][1] = nodes[4];
        proofs[3] = new bytes32[](2);
        proofs[3][0] = nodes[2];
        proofs[3][1] = nodes[4];

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 4}));

        vm.startPrank(users.withdrawerA);

        token.approve(address(rolldown), amount - ferryTip);
        rolldown.ferryWithdrawal(withdrawalA);
        rolldown.closeWithdrawal(withdrawalA, merkleRoot, proofs[0]);
        rolldown.ferryWithdrawal(withdrawalA2);
        rolldown.closeWithdrawal(withdrawalA2, merkleRoot, proofs[1]);

        vm.stopPrank();

        vm.startPrank(users.withdrawerB);

        token.approve(address(rolldown), amount - ferryTip);
        rolldown.ferryWithdrawal(withdrawalB);
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, proofs[2]);

        vm.stopPrank();

        vm.startPrank(users.withdrawerC);

        token.approve(address(rolldown), amount - ferryTip);
        rolldown.ferryWithdrawal(withdrawalC);
        rolldown.closeWithdrawal(withdrawalC, merkleRoot, proofs[3]);

        vm.stopPrank();

        uint256 currentPayeeABalance = token.balanceOf(users.withdrawerA);
        assertEq(currentPayeeABalance, initialPayeeABalance + amount * 2);

        uint256 currentPayeeBBalance = token.balanceOf(users.withdrawerB);
        assertEq(currentPayeeBBalance, initialPayeeBBalance + amount);

        uint256 currentPayeeCBalance = token.balanceOf(users.withdrawerC);
        assertEq(currentPayeeCBalance, initialPayeeCBalance + amount);

        assertTrue(rolldown.processedL2Requests(nodes[0]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(nodes[1]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(nodes[2]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(nodes[3]) == rolldown.CLOSED());
        assertEq(rolldown.lastProcessedUpdate_origin_l2(), 4);
    }

    function test_RevertIf_Paused() external {
        vm.prank(users.admin);
        rolldown.pause();

        IRolldownPrimitives.Withdrawal memory withdrawalA =
            _createWithdrawal(1, users.withdrawerA, nativeTokenAddress, amount, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalB =
            _createWithdrawal(2, users.withdrawerB, nativeTokenAddress, amount, 0);

        bytes32 nodeA = rolldown.hashWithdrawal(withdrawalA);
        bytes32 nodeB = rolldown.hashWithdrawal(withdrawalB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(nodeA, nodeB));
        bytes32[] memory merkleProof = new bytes32[](1);
        merkleProof[0] = nodeA;

        vm.prank(users.withdrawerB);
        vm.expectRevert("Pausable: paused");
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, merkleProof);
    }

    function test_RevertIf_L2RequestAlreadyProcessed() external {
        vm.prank(users.depositor);
        rolldown.depositNative{value: amount}();

        IRolldownPrimitives.Withdrawal memory withdrawalA =
            _createWithdrawal(1, users.withdrawerA, nativeTokenAddress, amount, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalB =
            _createWithdrawal(2, users.withdrawerB, nativeTokenAddress, amount, 0);

        bytes32 nodeA = rolldown.hashWithdrawal(withdrawalA);
        bytes32 nodeB = rolldown.hashWithdrawal(withdrawalB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(nodeA, nodeB));
        bytes32[] memory merkleProof = new bytes32[](1);
        merkleProof[0] = nodeA;

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 2}));

        vm.startPrank(users.withdrawerB);

        rolldown.closeWithdrawal(withdrawalB, merkleRoot, merkleProof);

        vm.expectRevert(abi.encodeWithSelector(L2RequestAlreadyProcessed.selector, nodeB));
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, merkleProof);

        vm.stopPrank();
    }

    function test_RevertIf_UnexpectedMerkleRoot() external {
        vm.prank(users.depositor);
        rolldown.depositNative{value: amount}();

        IRolldownPrimitives.Withdrawal memory withdrawalA =
            _createWithdrawal(1, users.withdrawerA, nativeTokenAddress, amount, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalB =
            _createWithdrawal(2, users.withdrawerB, nativeTokenAddress, amount, 0);

        bytes32 nodeA = rolldown.hashWithdrawal(withdrawalA);
        bytes32 nodeB = rolldown.hashWithdrawal(withdrawalB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(nodeA, nodeB));
        bytes32[] memory merkleProof = new bytes32[](1);
        merkleProof[0] = nodeA;

        vm.prank(users.updater);
        vm.expectRevert(abi.encodeWithSelector(UnexpectedMerkleRoot.selector));
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, merkleProof);
    }

    function test_RevertIf_RequestOutOfRange() external {
        vm.prank(users.depositor);
        rolldown.depositNative{value: amount}();

        IRolldownPrimitives.Withdrawal memory withdrawalA =
            _createWithdrawal(2, users.withdrawerA, nativeTokenAddress, amount, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalB =
            _createWithdrawal(3, users.withdrawerB, nativeTokenAddress, amount, 0);

        bytes32 nodeA = rolldown.hashWithdrawal(withdrawalA);
        bytes32 nodeB = rolldown.hashWithdrawal(withdrawalB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(nodeA, nodeB));
        bytes32[] memory merkleProof = new bytes32[](1);
        merkleProof[0] = nodeA;

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 2}));

        vm.prank(users.withdrawerB);
        vm.expectRevert(abi.encodeWithSelector(RequestOutOfRange.selector, 3, 1, 2));
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, merkleProof);
    }

    function test_RevertIf_RequestRangeTooLarge() external {
        vm.prank(users.depositor);
        rolldown.depositNative{value: amount}();

        IRolldownPrimitives.Withdrawal memory withdrawalA =
            _createWithdrawal(1, users.withdrawerA, nativeTokenAddress, amount, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalB =
            _createWithdrawal(2, users.withdrawerB, nativeTokenAddress, amount, 0);

        bytes32 nodeA = rolldown.hashWithdrawal(withdrawalA);
        bytes32 nodeB = rolldown.hashWithdrawal(withdrawalB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(nodeA, nodeB));
        bytes32[] memory merkleProof = new bytes32[](1);
        merkleProof[0] = nodeA;

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: type(uint64).max}));

        vm.prank(users.withdrawerB);
        vm.expectRevert(abi.encodeWithSelector(RequestRangeTooLarge.selector, type(uint64).max));
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, merkleProof);
    }

    function test_RevertIf_InvalidRequestProof() external {
        vm.prank(users.depositor);
        rolldown.depositNative{value: amount}();

        IRolldownPrimitives.Withdrawal memory withdrawalA =
            _createWithdrawal(1, users.withdrawerA, nativeTokenAddress, amount, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalB =
            _createWithdrawal(2, users.withdrawerB, nativeTokenAddress, amount, 0);

        bytes32 nodeA = rolldown.hashWithdrawal(withdrawalA);
        bytes32 nodeB = rolldown.hashWithdrawal(withdrawalB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(nodeA, nodeB));
        bytes32[] memory merkleProof = new bytes32[](1);
        merkleProof[0] = nodeB;

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 2}));

        vm.prank(users.withdrawerB);
        vm.expectRevert(abi.encodeWithSelector(InvalidRequestProof.selector, merkleRoot));
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, merkleProof);
    }

    function test_RevertIf_ZeroTransferAmountNative() external {
        vm.prank(users.depositor);
        rolldown.depositNative{value: amount}();

        IRolldownPrimitives.Withdrawal memory withdrawalA =
            _createWithdrawal(1, users.withdrawerA, nativeTokenAddress, 0, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalB =
            _createWithdrawal(2, users.withdrawerB, nativeTokenAddress, 0, 0);

        bytes32 nodeA = rolldown.hashWithdrawal(withdrawalA);
        bytes32 nodeB = rolldown.hashWithdrawal(withdrawalB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(nodeA, nodeB));
        bytes32[] memory merkleProof = new bytes32[](1);
        merkleProof[0] = nodeA;

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 2}));

        vm.prank(users.withdrawerB);
        vm.expectRevert(abi.encodeWithSelector(ZeroTransferAmount.selector));
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, merkleProof);
    }

    function test_RevertIf_ZeroTransferAmountERC20() external {
        token.mint(users.depositor);

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount);
        rolldown.depositERC20(address(token), amount);

        vm.stopPrank();

        IRolldownPrimitives.Withdrawal memory withdrawalA =
            _createWithdrawal(1, users.withdrawerA, address(token), 0, 0);
        IRolldownPrimitives.Withdrawal memory withdrawalB =
            _createWithdrawal(2, users.withdrawerB, address(token), 0, 0);

        bytes32 nodeA = rolldown.hashWithdrawal(withdrawalA);
        bytes32 nodeB = rolldown.hashWithdrawal(withdrawalB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(nodeA, nodeB));
        bytes32[] memory merkleProof = new bytes32[](1);
        merkleProof[0] = nodeA;

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, IRolldownPrimitives.Range({start: 1, end: 2}));

        vm.prank(users.withdrawerB);
        vm.expectRevert(abi.encodeWithSelector(ZeroTransferAmount.selector));
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, merkleProof);
    }
}

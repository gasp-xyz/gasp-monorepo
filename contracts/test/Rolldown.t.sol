// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {EmptyContract} from "@eigenlayer/test/mocks/EmptyContract.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {Test} from "forge-std/Test.sol";
import {IRolldown} from "../src/IRolldown.sol";
import {IRolldownPrimitives} from "../src/IRolldownPrimitives.sol";
import {Rolldown} from "../src/Rolldown.sol";
import {MyERC20} from "./utils/Utilities.sol";

contract RolldownTest is Test, IRolldownPrimitives {
    // solhint-disable-next-line gas-struct-packing
    struct Users {
        address admin;
        address grantee;
        address updater;
        address depositor;
        address withdrawerA;
        address withdrawerB;
        address withdrawerC;
        address recipient;
    }

    Users public users;
    MyERC20 public token;
    Rolldown public rolldown;
    bytes32 public defaultAdminRole = 0x00;
    bytes32 public updaterRole;
    address public nativeTokenAddress;

    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
    event Transfer(address indexed from, address indexed to, uint256 value);

    function setUp() external {
        users = Users({
            admin: makeAddr("admin"),
            grantee: makeAddr("grantee"),
            updater: makeAddr("updater"),
            depositor: makeAddr("depositor"),
            withdrawerA: makeAddr("withdrawerA"),
            withdrawerB: makeAddr("withdrawerB"),
            withdrawerC: makeAddr("withdrawerC"),
            recipient: makeAddr("recipient")
        });

        deal(payable(users.admin), 100 ether);
        deal(payable(users.grantee), 100 ether);
        deal(payable(users.updater), 100 ether);
        deal(payable(users.depositor), 100 ether);
        deal(payable(users.withdrawerA), 100 ether);
        deal(payable(users.withdrawerB), 100 ether);
        deal(payable(users.withdrawerC), 100 ether);
        deal(payable(users.recipient), 100 ether);

        token = new MyERC20();

        Rolldown implementation = new Rolldown();
        ProxyAdmin proxyAdmin = new ProxyAdmin();
        TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(
            address(implementation),
            address(proxyAdmin),
            abi.encodeCall(Rolldown.initialize, (users.admin, ChainId.Ethereum, users.updater))
        );
        rolldown = Rolldown(address(proxy));

        updaterRole = rolldown.UPDATER_ROLE();
        nativeTokenAddress = rolldown.NATIVE_TOKEN_ADDRESS();
    }

    function createRequestId(Origin origin) internal view returns (RequestId memory) {
        return RequestId({origin: origin, id: rolldown.counter()});
    }

    function accessControlMessage(address account) internal pure returns (bytes memory) {
        return bytes(
            string.concat(
                "AccessControl: account ",
                Strings.toHexString(uint160(account), 20),
                " is missing role 0x0000000000000000000000000000000000000000000000000000000000000000"
            )
        );
    }
}

contract Deploy is RolldownTest {
    function test_HasDefaultAdminRole() external view {
        assertTrue(rolldown.hasRole(defaultAdminRole, users.admin));
    }

    function test_HasUpdaterRole() external view {
        assertTrue(rolldown.hasRole(updaterRole, users.updater));
    }

    function test_ReturnCounter() external view {
        assertEq(rolldown.counter(), 1);
    }

    function test_ReturnLastProcessedUpdateOriginL1() external view {
        assertEq(rolldown.lastProcessedUpdate_origin_l1(), 0);
    }

    function test_ReturnLastProcessedUpdateOriginL2() external view {
        assertEq(rolldown.lastProcessedUpdate_origin_l2(), 0);
    }

    function test_ReturnChainId() external view {
        assertEq(uint256(rolldown.chain()), uint256(ChainId.Ethereum));
    }

    function test_ReturnUpdaterAccount() external view {
        assertEq(rolldown.updaterAccount(), users.updater);
    }

    function test_RevertIf_InvalidInitialization() external {
        vm.expectRevert("Initializable: contract is already initialized");
        rolldown.initialize(users.admin, ChainId.Ethereum, users.updater);
    }

    function test_RevertIf_ZeroAdmin() external {
        Rolldown implementation = new Rolldown();
        ProxyAdmin proxyAdmin = new ProxyAdmin();

        vm.expectRevert(ZeroAdmin.selector);
        new TransparentUpgradeableProxy(
            address(implementation),
            address(proxyAdmin),
            abi.encodeCall(Rolldown.initialize, (address(0), ChainId.Ethereum, users.updater))
        );
    }

    function test_RevertIf_ZeroUpdater() external {
        Rolldown implementation = new Rolldown();
        ProxyAdmin proxyAdmin = new ProxyAdmin();

        vm.expectRevert(ZeroUpdater.selector);
        new TransparentUpgradeableProxy(
            address(implementation),
            address(proxyAdmin),
            abi.encodeCall(Rolldown.initialize, (users.admin, ChainId.Ethereum, address(0)))
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
        assertTrue(rolldown.hasRole(defaultAdminRole, users.grantee));
    }

    function test_RevertIf_UnauthorizedAccount() external {
        vm.prank(users.grantee);
        vm.expectRevert(accessControlMessage(users.grantee));
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
        assertFalse(rolldown.hasRole(defaultAdminRole, users.grantee));

        vm.stopPrank();
    }

    function test_RevertIf_UnauthorizedAccount() external {
        vm.prank(users.admin);
        rolldown.grantRole(defaultAdminRole, users.grantee);

        vm.prank(users.depositor);
        vm.expectRevert(accessControlMessage(users.depositor));
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
        assertFalse(rolldown.hasRole(defaultAdminRole, users.grantee));
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
    address public newUpdater = makeAddr("newUpdater");

    function test_EmitNewUpdaterSet() external {
        vm.prank(users.admin);
        vm.expectEmit();
        emit NewUpdaterSet(newUpdater);
        rolldown.setUpdater(newUpdater);
    }

    function test_ReturnNewUpdater() external {
        vm.prank(users.admin);
        rolldown.setUpdater(newUpdater);
        assertEq(rolldown.updaterAccount(), newUpdater);
    }

    function test_HasUpdaterRole() external {
        vm.prank(users.admin);
        rolldown.setUpdater(newUpdater);
        assertTrue(rolldown.hasRole(updaterRole, newUpdater));
    }

    function test_RevertIf_UnauthorizedAccount() external {
        vm.prank(users.depositor);
        vm.expectRevert(accessControlMessage(users.depositor));
        rolldown.setUpdater(newUpdater);
    }

    function test_RevertIf_ZeroUpdater() external {
        vm.prank(users.admin);
        vm.expectRevert(ZeroUpdater.selector);
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

    function test_EmitDepositAcceptedIntoQueue_BackwardCompatibility() external {
        uint256 requestId = rolldown.counter();

        vm.prank(users.depositor);
        vm.expectEmit();
        emit DepositAcceptedIntoQueue(requestId, users.depositor, nativeTokenAddress, amount, ferryTip);
        rolldown.deposit_native{value: amount}();
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
        assertEq(pendingDeposit.timeStamp, block.timestamp);
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
        vm.expectRevert(ZeroAmount.selector);
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

    function test_EmitDepositAcceptedIntoQueue_BackwardCompatibility() external {
        uint256 requestId = rolldown.counter();

        vm.prank(users.depositor);
        vm.expectEmit();
        emit DepositAcceptedIntoQueue(requestId, users.depositor, nativeTokenAddress, amount, ferryTip);
        rolldown.deposit_native{value: amount}(ferryTip);
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
        assertEq(pendingDeposit.timeStamp, block.timestamp);
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
        vm.expectRevert(ZeroAmount.selector);
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
        token.mint(users.depositor);

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount);
        uint256 requestId = rolldown.counter();

        vm.expectEmit();
        emit DepositAcceptedIntoQueue(requestId, users.depositor, address(token), amount, ferryTip);
        rolldown.depositERC20(address(token), amount);

        vm.stopPrank();
    }

    function test_EmitDepositAcceptedIntoQueue_BackwardCompatibility() external {
        token.mint(users.depositor);

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount);
        uint256 requestId = rolldown.counter();

        vm.expectEmit();
        emit DepositAcceptedIntoQueue(requestId, users.depositor, address(token), amount, ferryTip);
        rolldown.deposit_erc20(address(token), amount);

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

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount);
        uint256 requestId = rolldown.counter();
        rolldown.depositERC20(address(token), amount);

        vm.stopPrank();

        Rolldown.L1Update memory l1Update = rolldown.getUpdateForL2();
        assertEq(l1Update.pendingDeposits.length, 1);

        Rolldown.Deposit memory pendingDeposit = l1Update.pendingDeposits[0];
        assertEq(pendingDeposit.requestId.id, requestId);
        assertEq(pendingDeposit.depositRecipient, users.depositor);
        assertEq(pendingDeposit.tokenAddress, address(token));
        assertEq(pendingDeposit.amount, amount);
        assertEq(pendingDeposit.timeStamp, block.timestamp);
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
        vm.expectRevert(ZeroAmount.selector);
        rolldown.depositERC20(address(token), 0);
    }

    function test_RevertIf_ZeroToken() external {
        vm.prank(users.depositor);
        vm.expectRevert(ZeroToken.selector);
        rolldown.depositERC20(address(0), amount);
    }

    function test_RevertIf_InsufficientAllowance() external {
        token.mint(users.depositor);

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount - 1);

        vm.expectRevert("ERC20: insufficient allowance");
        rolldown.depositERC20(address(token), amount);

        vm.stopPrank();
    }
}

contract DepositERC20WithFerryTip is RolldownTest {
    uint256 public amount = 1 ether;
    uint256 public ferryTip = amount / 100;

    function test_EmitDepositAcceptedIntoQueue() external {
        token.mint(users.depositor);

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount);
        uint256 requestId = rolldown.counter();

        vm.expectEmit();
        emit DepositAcceptedIntoQueue(requestId, users.depositor, address(token), amount, ferryTip);
        rolldown.depositERC20(address(token), amount, ferryTip);

        vm.stopPrank();
    }

    function test_EmitDepositAcceptedIntoQueue_BackwardCompatibility() external {
        token.mint(users.depositor);

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount);
        uint256 requestId = rolldown.counter();

        vm.expectEmit();
        emit DepositAcceptedIntoQueue(requestId, users.depositor, address(token), amount, ferryTip);
        rolldown.deposit_erc20(address(token), amount, ferryTip);

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

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount);
        uint256 requestId = rolldown.counter();
        rolldown.depositERC20(address(token), amount, ferryTip);

        vm.stopPrank();

        Rolldown.L1Update memory l1Update = rolldown.getUpdateForL2();
        assertEq(l1Update.pendingDeposits.length, 1);

        Rolldown.Deposit memory pendingDeposit = l1Update.pendingDeposits[0];
        assertEq(pendingDeposit.requestId.id, requestId);
        assertEq(pendingDeposit.depositRecipient, users.depositor);
        assertEq(pendingDeposit.tokenAddress, address(token));
        assertEq(pendingDeposit.amount, amount);
        assertEq(pendingDeposit.timeStamp, block.timestamp);
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
        vm.expectRevert(ZeroAmount.selector);
        rolldown.depositERC20(address(token), 0, ferryTip);
    }

    function test_RevertIf_ZeroToken() external {
        vm.prank(users.depositor);
        vm.expectRevert(ZeroToken.selector);
        rolldown.depositERC20(address(0), amount, ferryTip);
    }

    function test_RevertIf_FerryTipExceedsAmount() external {
        ferryTip = amount + 1;

        vm.prank(users.depositor);
        vm.expectRevert(abi.encodeWithSelector(FerryTipExceedsAmount.selector, ferryTip, amount));
        rolldown.depositERC20(address(token), amount, ferryTip);
    }

    function test_RevertIf_InsufficientAllowance() external {
        token.mint(users.depositor);

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount - 1);

        vm.expectRevert("ERC20: insufficient allowance");
        rolldown.depositERC20(address(token), amount, ferryTip);

        vm.stopPrank();
    }
}

contract FerryWithdrawal is RolldownTest {
    uint256 public amount = 1 ether;
    uint256 public ferryTip = amount / 100;
    uint256 public ferriedAmount = amount - ferryTip;

    function test_EmitWithdrawalFerried_Native_WithoutFerryTip() external {
        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, 0);
        bytes32 withdrawalHash = rolldown.hashWithdrawal(withdrawal);

        vm.prank(users.withdrawerA);
        vm.expectEmit();
        emit WithdrawalFerried(withdrawal.requestId.id, amount, users.recipient, users.withdrawerA, withdrawalHash);
        rolldown.ferryWithdrawal{value: amount}(withdrawal);
    }

    function test_EmitWithdrawalFerried_Native_WithoutFerryTip_BackwardCompatibility() external {
        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, 0);
        bytes32 withdrawalHash = rolldown.hashWithdrawal(withdrawal);

        vm.prank(users.withdrawerA);
        vm.expectEmit();
        emit WithdrawalFerried(withdrawal.requestId.id, amount, users.recipient, users.withdrawerA, withdrawalHash);
        rolldown.ferry_withdrawal{value: amount}(withdrawal);
    }

    function test_EmitWithdrawalFerried_Native_WithFerryTip() external {
        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, ferryTip);
        bytes32 withdrawalHash = rolldown.hashWithdrawal(withdrawal);

        vm.prank(users.withdrawerA);
        vm.expectEmit();
        emit WithdrawalFerried(
            withdrawal.requestId.id, ferriedAmount, users.recipient, users.withdrawerA, withdrawalHash
        );
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
    }

    function test_EmitWithdrawalFerried_ERC20_WithoutFerryTip() external {
        token.mint(users.withdrawerA);

        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, address(token), amount, 0);
        bytes32 withdrawalHash = rolldown.hashWithdrawal(withdrawal);

        vm.startPrank(users.withdrawerA);

        token.approve(address(rolldown), amount);

        vm.expectEmit();
        emit WithdrawalFerried(withdrawal.requestId.id, amount, users.recipient, users.withdrawerA, withdrawalHash);
        rolldown.ferryWithdrawal(withdrawal);

        vm.stopPrank();
    }

    function test_EmitWithdrawalFerried_ERC20_WithoutFerryTip_BackwardCompatibility() external {
        token.mint(users.withdrawerA);

        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, address(token), amount, 0);
        bytes32 withdrawalHash = rolldown.hashWithdrawal(withdrawal);

        vm.startPrank(users.withdrawerA);

        token.approve(address(rolldown), amount);

        vm.expectEmit();
        emit WithdrawalFerried(withdrawal.requestId.id, amount, users.recipient, users.withdrawerA, withdrawalHash);
        rolldown.ferry_withdrawal(withdrawal);

        vm.stopPrank();
    }

    function test_EmitWithdrawalFerried_ERC20_WithFerryTip() external {
        token.mint(users.withdrawerA);

        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, address(token), amount, ferryTip);
        bytes32 withdrawalHash = rolldown.hashWithdrawal(withdrawal);

        vm.startPrank(users.withdrawerA);

        token.approve(address(rolldown), ferriedAmount);

        vm.expectEmit();
        emit WithdrawalFerried(
            withdrawal.requestId.id, ferriedAmount, users.recipient, users.withdrawerA, withdrawalHash
        );
        rolldown.ferryWithdrawal(withdrawal);

        vm.stopPrank();
    }

    function test_ChangeBalances_Native_WithoutFerryTip() external {
        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, 0);

        uint256 initialWithdrawerBalance = users.withdrawerA.balance;
        uint256 initialRecipientBalance = users.recipient.balance;

        vm.prank(users.withdrawerA);
        rolldown.ferryWithdrawal{value: amount}(withdrawal);

        uint256 currentWithdrawerBalance = users.withdrawerA.balance;
        assertEq(currentWithdrawerBalance, initialWithdrawerBalance - amount);

        uint256 currentRecipientBalance = users.recipient.balance;
        assertEq(currentRecipientBalance, initialRecipientBalance + amount);
    }

    function test_ChangeBalances_Native_WithFerryTip() external {
        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, ferryTip);

        uint256 initialWithdrawerBalance = users.withdrawerA.balance;
        uint256 initialRecipientBalance = users.recipient.balance;

        vm.prank(users.withdrawerA);
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);

        uint256 currentWithdrawerBalance = users.withdrawerA.balance;
        assertEq(currentWithdrawerBalance, initialWithdrawerBalance - amount + ferryTip);

        uint256 currentRecipientBalance = users.recipient.balance;
        assertEq(currentRecipientBalance, initialRecipientBalance + ferriedAmount);
    }

    function test_ChangeBalances_ERC20_WithoutFerryTip() external {
        token.mint(users.withdrawerA);

        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, address(token), amount, 0);

        uint256 initialWithdrawerBalance = token.balanceOf(users.withdrawerA);
        uint256 initialRecipientBalance = token.balanceOf(users.recipient);

        vm.startPrank(users.withdrawerA);

        token.approve(address(rolldown), amount);
        rolldown.ferryWithdrawal(withdrawal);

        vm.stopPrank();

        uint256 currentWithdrawerBalance = token.balanceOf(users.withdrawerA);
        assertEq(currentWithdrawerBalance, initialWithdrawerBalance - amount);

        uint256 currentRecipientBalance = token.balanceOf(users.recipient);
        assertEq(currentRecipientBalance, initialRecipientBalance + amount);
    }

    function test_ChangeBalances_ERC20_WithFerryTip() external {
        token.mint(users.withdrawerA);

        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, address(token), amount, ferryTip);

        uint256 initialWithdrawerBalance = token.balanceOf(users.withdrawerA);
        uint256 initialRecipientBalance = token.balanceOf(users.recipient);

        vm.startPrank(users.withdrawerA);

        token.approve(address(rolldown), ferriedAmount);
        rolldown.ferryWithdrawal(withdrawal);

        vm.stopPrank();

        uint256 currentWithdrawerBalance = token.balanceOf(users.withdrawerA);
        assertEq(currentWithdrawerBalance, initialWithdrawerBalance - amount + ferryTip);

        uint256 currentRecipientBalance = token.balanceOf(users.recipient);
        assertEq(currentRecipientBalance, initialRecipientBalance + ferriedAmount);
    }

    function test_GetProcessedL2Requests_Native() external {
        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, ferryTip);
        bytes32 withdrawalHash = rolldown.hashWithdrawal(withdrawal);

        vm.prank(users.withdrawerA);
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
        assertEq(rolldown.processedL2Requests(withdrawalHash), users.withdrawerA);
    }

    function test_GetProcessedL2Requests_ERC20() external {
        token.mint(users.withdrawerA);

        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, address(token), amount, ferryTip);
        bytes32 withdrawalHash = rolldown.hashWithdrawal(withdrawal);

        vm.startPrank(users.withdrawerA);

        token.approve(address(rolldown), ferriedAmount);
        rolldown.ferryWithdrawal(withdrawal);
        assertEq(rolldown.processedL2Requests(withdrawalHash), users.withdrawerA);

        vm.stopPrank();
    }

    function test_RevertIf_Paused() external {
        vm.prank(users.admin);
        rolldown.pause();

        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, address(token), amount, ferryTip);

        vm.prank(users.withdrawerA);
        vm.expectRevert("Pausable: paused");
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
    }

    function test_RevertIf_ZeroAmount_Native() external {
        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, 0, ferryTip);

        vm.prank(users.withdrawerA);
        vm.expectRevert(ZeroAmount.selector);
        rolldown.ferryWithdrawal{value: 0}(withdrawal);
    }

    function test_RevertIf_ZeroFerriedAmount_ERC20() external {
        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, address(token), 0, ferryTip);

        vm.prank(users.withdrawerA);
        vm.expectRevert(ZeroAmount.selector);
        rolldown.ferryWithdrawal(withdrawal);
    }

    function test_RevertIf_FerryTipExceedsAmount_Native() external {
        ferryTip = amount + 1;

        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, ferryTip);

        vm.prank(users.withdrawerA);
        vm.expectRevert(abi.encodeWithSelector(FerryTipExceedsAmount.selector, ferryTip, amount));
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
    }

    function test_RevertIf_FerryTipExceedsAmount_ERC20() external {
        ferryTip = amount + 1;

        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, address(token), amount, ferryTip);

        vm.prank(users.withdrawerA);
        vm.expectRevert(abi.encodeWithSelector(FerryTipExceedsAmount.selector, ferryTip, amount));
        rolldown.ferryWithdrawal(withdrawal);
    }

    function test_RevertIf_ZeroRecipient() external {
        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), address(0), nativeTokenAddress, amount, ferryTip);

        vm.prank(users.withdrawerA);
        vm.expectRevert(ZeroRecipient.selector);
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
    }

    function test_RevertIf_InsufficientFerriedAmount_Native() external {
        ferriedAmount -= 1;

        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, ferryTip);

        vm.prank(users.withdrawerA);
        vm.expectRevert(abi.encodeWithSelector(InvalidFerriedAmount.selector, ferriedAmount, amount - ferryTip));
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
    }

    function test_RevertIf_ZeroFerriedAmount_Native() external {
        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, ferryTip);

        vm.prank(users.withdrawerA);
        vm.expectRevert(abi.encodeWithSelector(InvalidFerriedAmount.selector, 0, ferriedAmount));
        rolldown.ferryWithdrawal{value: 0}(withdrawal);
    }

    function test_RevertIf_WithdrawalAlreadyFerried() external {
        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, ferryTip);
        bytes32 withdrawalHash = rolldown.hashWithdrawal(withdrawal);

        vm.startPrank(users.withdrawerA);

        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);

        vm.expectRevert(abi.encodeWithSelector(WithdrawalAlreadyFerried.selector, withdrawalHash));
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);

        vm.stopPrank();
    }

    function test_RevertIf_CallToNonContract() external {
        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, address(0), amount, ferryTip);

        vm.prank(users.withdrawerA);
        vm.expectRevert("Address: call to non-contract");
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
    }

    function test_RevertIf_LowLevelCallFailed() external {
        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, address(new EmptyContract()), amount, ferryTip);

        vm.prank(users.withdrawerA);
        vm.expectRevert("SafeERC20: low-level call failed");
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
    }

    function test_RevertIf_InsufficientAllowance() external {
        token.mint(users.withdrawerA);

        ferriedAmount -= 1;
        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, address(token), amount, ferryTip);

        vm.startPrank(users.withdrawerA);

        token.approve(address(rolldown), ferriedAmount);

        vm.expectRevert("ERC20: insufficient allowance");
        rolldown.ferryWithdrawal(withdrawal);

        vm.stopPrank();
    }
}

contract UpdateL1FromL2 is RolldownTest {
    uint256 public amount = 1 ether;
    uint256 public ferryTip = amount / 100;
    uint256 public ferriedAmount = amount - ferryTip;

    function test_EmitL2UpdateAccepted() external {
        token.mint(users.withdrawerB);

        Withdrawal memory withdrawalA =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, 0);
        bytes32 withdrawalHashA = rolldown.hashWithdrawal(withdrawalA);
        Withdrawal memory withdrawalB =
            Withdrawal(createRequestId(Origin.L2), users.recipient, address(token), amount, 0);
        bytes32 withdrawalHashB = rolldown.hashWithdrawal(withdrawalB);

        vm.prank(users.withdrawerA);
        rolldown.ferryWithdrawal{value: amount}(withdrawalA);

        vm.startPrank(users.withdrawerB);

        token.approve(address(rolldown), amount);
        rolldown.ferryWithdrawal(withdrawalB);

        vm.stopPrank();

        bytes32 merkleRoot = keccak256(abi.encodePacked(withdrawalHashA, withdrawalHashB));
        Range memory range = Range(1, 2);

        vm.prank(users.updater);
        vm.expectEmit();
        emit L2UpdateAccepted(merkleRoot, range);
        rolldown.updateL1FromL2(merkleRoot, range);
    }

    function test_ReturnRoots() external {
        token.mint(users.withdrawerB);

        Withdrawal memory withdrawalA =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, 0);
        bytes32 withdrawalHashA = rolldown.hashWithdrawal(withdrawalA);
        Withdrawal memory withdrawalB =
            Withdrawal(createRequestId(Origin.L2), users.recipient, address(token), amount, 0);
        bytes32 withdrawalHashB = rolldown.hashWithdrawal(withdrawalB);

        vm.prank(users.withdrawerA);
        rolldown.ferryWithdrawal{value: amount}(withdrawalA);

        vm.startPrank(users.withdrawerB);

        token.approve(address(rolldown), amount);
        rolldown.ferryWithdrawal(withdrawalB);

        vm.stopPrank();

        bytes32 merkleRoot = keccak256(abi.encodePacked(withdrawalHashA, withdrawalHashB));
        Range memory range = Range(1, 2);

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, range);
        assertEq(rolldown.roots(0), merkleRoot);
    }

    function test_ReturnMerkeRootRange() external {
        token.mint(users.withdrawerB);

        Withdrawal memory withdrawalA =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, 0);
        bytes32 withdrawalHashA = rolldown.hashWithdrawal(withdrawalA);
        Withdrawal memory withdrawalB =
            Withdrawal(createRequestId(Origin.L2), users.recipient, address(token), amount, 0);
        bytes32 withdrawalHashB = rolldown.hashWithdrawal(withdrawalB);

        vm.prank(users.withdrawerA);
        rolldown.ferryWithdrawal{value: amount}(withdrawalA);

        vm.startPrank(users.withdrawerB);

        token.approve(address(rolldown), amount);
        rolldown.ferryWithdrawal(withdrawalB);

        vm.stopPrank();

        bytes32 merkleRoot = keccak256(abi.encodePacked(withdrawalHashA, withdrawalHashB));
        Range memory range = Range(1, 2);

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, range);

        (uint256 startRange, uint256 endRange) = rolldown.merkleRootRange(merkleRoot);
        assertEq(startRange, range.start);
        assertEq(endRange, range.end);
    }

    function test_ReturnLastProcessedUpdateOriginL2() external {
        token.mint(users.withdrawerB);

        Withdrawal memory withdrawalA =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, 0);
        bytes32 withdrawalHashA = rolldown.hashWithdrawal(withdrawalA);
        Withdrawal memory withdrawalB =
            Withdrawal(createRequestId(Origin.L2), users.recipient, address(token), amount, 0);
        bytes32 withdrawalHashB = rolldown.hashWithdrawal(withdrawalB);

        vm.prank(users.withdrawerA);
        rolldown.ferryWithdrawal{value: amount}(withdrawalA);

        vm.startPrank(users.withdrawerB);

        token.approve(address(rolldown), amount);
        rolldown.ferryWithdrawal(withdrawalB);

        vm.stopPrank();

        bytes32 merkleRoot = keccak256(abi.encodePacked(withdrawalHashA, withdrawalHashB));
        Range memory range = Range(1, 2);

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, range);
        assertEq(rolldown.lastProcessedUpdate_origin_l2(), range.end);
    }

    function test_RevertIf_Paused() external {
        vm.prank(users.admin);
        rolldown.pause();

        Range memory range = Range(1, 1);

        vm.prank(users.updater);
        vm.expectRevert("Pausable: paused");
        rolldown.updateL1FromL2(bytes32(""), range);
    }

    function test_RevertIf_ZeroUpdateRange() external {
        Range memory range = Range(0, 1);

        vm.prank(users.updater);
        vm.expectRevert(ZeroUpdateRange.selector);
        rolldown.updateL1FromL2(bytes32(""), range);
    }

    function test_RevertIf_InvalidRangeStart() external {
        Range memory range = Range(1, 0);

        vm.prank(users.updater);
        vm.expectRevert(abi.encodeWithSelector(InvalidUpdateRange.selector, range.start, range.end));
        rolldown.updateL1FromL2(bytes32(""), range);
    }

    function test_RevertIf_PreviousUpdateMissed() external {
        token.mint(users.withdrawerB);

        Withdrawal memory withdrawalA =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, 0);
        bytes32 withdrawalHashA = rolldown.hashWithdrawal(withdrawalA);
        Withdrawal memory withdrawalB =
            Withdrawal(createRequestId(Origin.L2), users.recipient, address(token), amount, 0);
        bytes32 withdrawalHashB = rolldown.hashWithdrawal(withdrawalB);

        vm.prank(users.withdrawerA);
        rolldown.ferryWithdrawal{value: amount}(withdrawalA);

        vm.startPrank(users.withdrawerB);

        token.approve(address(rolldown), amount);
        rolldown.ferryWithdrawal(withdrawalB);

        vm.stopPrank();

        bytes32 merkleRoot = keccak256(abi.encodePacked(withdrawalHashA, withdrawalHashB));
        Range memory range = Range(2, 3);
        uint256 lastProcessedUpdate = rolldown.lastProcessedUpdate_origin_l2();

        vm.prank(users.updater);
        vm.expectRevert(abi.encodeWithSelector(PreviousUpdateMissed.selector, range.start, lastProcessedUpdate));
        rolldown.updateL1FromL2(merkleRoot, range);
    }

    function test_RevertIf_UpdateAlreadyApplied() external {
        token.mint(users.withdrawerB);

        Withdrawal memory withdrawalA =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, 0);
        bytes32 withdrawalHashA = rolldown.hashWithdrawal(withdrawalA);
        Withdrawal memory withdrawalB =
            Withdrawal(createRequestId(Origin.L2), users.recipient, address(token), amount, 0);
        bytes32 withdrawalHashB = rolldown.hashWithdrawal(withdrawalB);

        vm.prank(users.withdrawerA);
        rolldown.ferryWithdrawal{value: amount}(withdrawalA);

        vm.startPrank(users.withdrawerB);

        token.approve(address(rolldown), amount);
        rolldown.ferryWithdrawal(withdrawalB);

        vm.stopPrank();

        bytes32 merkleRoot = keccak256(abi.encodePacked(withdrawalHashA, withdrawalHashB));
        Range memory range = Range(1, 2);

        vm.startPrank(users.updater);

        rolldown.updateL1FromL2(merkleRoot, range);
        uint256 lastProcessedUpdate = rolldown.lastProcessedUpdate_origin_l2();

        vm.expectRevert(abi.encodeWithSelector(UpdateAlreadyApplied.selector, range.end, lastProcessedUpdate));
        rolldown.updateL1FromL2(merkleRoot, range);

        vm.stopPrank();
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

        Withdrawal memory withdrawalA =
            Withdrawal(RequestId(Origin.L2, 1), users.withdrawerA, nativeTokenAddress, amount, 0);
        Withdrawal memory withdrawalB =
            Withdrawal(RequestId(Origin.L2, 2), users.withdrawerB, nativeTokenAddress, amount, 0);

        bytes32 hashA = rolldown.hashWithdrawal(withdrawalA);
        bytes32 hashB = rolldown.hashWithdrawal(withdrawalB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(hashA, hashB));
        bytes32[] memory proof = new bytes32[](1);
        proof[0] = hashA;

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, Range(1, 2));

        vm.prank(users.withdrawerB);
        vm.expectEmit();
        emit NativeTokensWithdrawn(users.withdrawerB, amount);
        emit WithdrawalClosed(2, hashB);
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, proof);
    }

    function test_EmitERC20TokensWithdrawn() external {
        token.mint(users.depositor);

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount);
        rolldown.depositERC20(address(token), amount);

        vm.stopPrank();

        Withdrawal memory withdrawalA =
            Withdrawal(RequestId(Origin.L2, 1), users.withdrawerA, address(token), amount, 0);
        Withdrawal memory withdrawalB =
            Withdrawal(RequestId(Origin.L2, 2), users.withdrawerB, address(token), amount, 0);

        bytes32 hashA = rolldown.hashWithdrawal(withdrawalA);
        bytes32 hashB = rolldown.hashWithdrawal(withdrawalB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(hashA, hashB));
        bytes32[] memory proof = new bytes32[](1);
        proof[0] = hashA;

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, Range(1, 2));

        vm.prank(users.withdrawerB);
        vm.expectEmit();
        emit ERC20TokensWithdrawn(users.withdrawerB, address(token), amount);
        emit Transfer(address(rolldown), users.withdrawerB, amount);
        emit WithdrawalClosed(2, hashB);
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, proof);
    }

    function test_EmitNativeTokensWithdrawnFerryTip() external {
        vm.prank(users.depositor);
        rolldown.depositNative{value: amount}();

        Withdrawal memory withdrawalA =
            Withdrawal(RequestId(Origin.L2, 1), users.withdrawerA, nativeTokenAddress, amount, ferryTip);
        Withdrawal memory withdrawalB =
            Withdrawal(RequestId(Origin.L2, 2), users.withdrawerB, nativeTokenAddress, amount, ferryTip);

        bytes32 hashA = rolldown.hashWithdrawal(withdrawalA);
        bytes32 hashB = rolldown.hashWithdrawal(withdrawalB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(hashA, hashB));
        bytes32[] memory proof = new bytes32[](1);
        proof[0] = hashA;

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, Range(1, 2));

        vm.startPrank(users.withdrawerB);

        rolldown.ferryWithdrawal{value: amount - ferryTip}(withdrawalB);

        vm.expectEmit();
        emit NativeTokensWithdrawn(users.withdrawerB, amount);
        emit FerriedWithdrawalClosed(2, hashB);
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, proof);

        vm.stopPrank();
    }

    function test_EmitERC20TokensWithdrawnFerryTip() external {
        token.mint(users.depositor);

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount);
        rolldown.depositERC20(address(token), amount);

        vm.stopPrank();

        Withdrawal memory withdrawalA =
            Withdrawal(RequestId(Origin.L2, 1), users.withdrawerA, address(token), amount, ferryTip);
        Withdrawal memory withdrawalB =
            Withdrawal(RequestId(Origin.L2, 2), users.withdrawerB, address(token), amount, ferryTip);

        bytes32 hashA = rolldown.hashWithdrawal(withdrawalA);
        bytes32 hashB = rolldown.hashWithdrawal(withdrawalB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(hashA, hashB));
        bytes32[] memory proof = new bytes32[](1);
        proof[0] = hashA;

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, Range(1, 2));

        vm.startPrank(users.withdrawerB);

        token.approve(address(rolldown), amount - ferryTip);
        rolldown.ferryWithdrawal(withdrawalB);

        vm.expectEmit();
        emit ERC20TokensWithdrawn(users.withdrawerB, address(token), amount);
        emit Transfer(address(rolldown), users.withdrawerB, amount);
        emit FerriedWithdrawalClosed(2, hashB);
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, proof);

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

        Withdrawal memory withdrawalA =
            Withdrawal(RequestId(Origin.L2, 1), users.withdrawerA, nativeTokenAddress, amount, 0);
        Withdrawal memory withdrawalA2 =
            Withdrawal(RequestId(Origin.L2, 2), users.withdrawerA, nativeTokenAddress, amount, 0);
        Withdrawal memory withdrawalB =
            Withdrawal(RequestId(Origin.L2, 3), users.withdrawerB, nativeTokenAddress, amount, 0);
        Withdrawal memory withdrawalC =
            Withdrawal(RequestId(Origin.L2, 4), users.withdrawerC, nativeTokenAddress, amount, 0);

        bytes32[] memory hashes = new bytes32[](6);
        hashes[0] = rolldown.hashWithdrawal(withdrawalA);
        hashes[1] = rolldown.hashWithdrawal(withdrawalA2);
        hashes[2] = rolldown.hashWithdrawal(withdrawalB);
        hashes[3] = rolldown.hashWithdrawal(withdrawalC);
        hashes[4] = keccak256(abi.encodePacked(hashes[0], hashes[1]));
        hashes[5] = keccak256(abi.encodePacked(hashes[2], hashes[3]));

        bytes32 merkleRoot = keccak256(abi.encodePacked(hashes[4], hashes[5]));

        bytes32[][] memory proofs = new bytes32[][](4);
        proofs[0] = new bytes32[](2);
        proofs[0][0] = hashes[1];
        proofs[0][1] = hashes[5];
        proofs[1] = new bytes32[](2);
        proofs[1][0] = hashes[0];
        proofs[1][1] = hashes[5];
        proofs[2] = new bytes32[](2);
        proofs[2][0] = hashes[3];
        proofs[2][1] = hashes[4];
        proofs[3] = new bytes32[](2);
        proofs[3][0] = hashes[2];
        proofs[3][1] = hashes[4];

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, Range(1, 4));

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

        assertTrue(rolldown.processedL2Requests(hashes[0]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(hashes[1]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(hashes[2]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(hashes[3]) == rolldown.CLOSED());
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

        Withdrawal memory withdrawalA =
            Withdrawal(RequestId(Origin.L2, 1), users.withdrawerA, address(token), amount, 0);
        Withdrawal memory withdrawalA2 =
            Withdrawal(RequestId(Origin.L2, 2), users.withdrawerA, address(token), amount, 0);
        Withdrawal memory withdrawalB =
            Withdrawal(RequestId(Origin.L2, 3), users.withdrawerB, address(token), amount, 0);
        Withdrawal memory withdrawalC =
            Withdrawal(RequestId(Origin.L2, 4), users.withdrawerC, address(token), amount, 0);

        bytes32[] memory hashes = new bytes32[](6);
        hashes[0] = rolldown.hashWithdrawal(withdrawalA);
        hashes[1] = rolldown.hashWithdrawal(withdrawalA2);
        hashes[2] = rolldown.hashWithdrawal(withdrawalB);
        hashes[3] = rolldown.hashWithdrawal(withdrawalC);
        hashes[4] = keccak256(abi.encodePacked(hashes[0], hashes[1]));
        hashes[5] = keccak256(abi.encodePacked(hashes[2], hashes[3]));

        bytes32 merkleRoot = keccak256(abi.encodePacked(hashes[4], hashes[5]));

        bytes32[][] memory proofs = new bytes32[][](4);
        proofs[0] = new bytes32[](2);
        proofs[0][0] = hashes[1];
        proofs[0][1] = hashes[5];
        proofs[1] = new bytes32[](2);
        proofs[1][0] = hashes[0];
        proofs[1][1] = hashes[5];
        proofs[2] = new bytes32[](2);
        proofs[2][0] = hashes[3];
        proofs[2][1] = hashes[4];
        proofs[3] = new bytes32[](2);
        proofs[3][0] = hashes[2];
        proofs[3][1] = hashes[4];

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, Range(1, 4));

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

        assertTrue(rolldown.processedL2Requests(hashes[0]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(hashes[1]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(hashes[2]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(hashes[3]) == rolldown.CLOSED());
        assertEq(rolldown.lastProcessedUpdate_origin_l2(), 4);
    }

    function test_ExecuteCloseWithdrawalNativeFerryTip() external {
        vm.prank(users.depositor);
        rolldown.depositNative{value: amount * 4}();

        uint256 initialPayeeABalance = users.withdrawerA.balance;
        uint256 initialPayeeBBalance = users.withdrawerB.balance;
        uint256 initialPayeeCBalance = users.withdrawerC.balance;

        Withdrawal memory withdrawalA =
            Withdrawal(RequestId(Origin.L2, 1), users.withdrawerA, nativeTokenAddress, amount, ferryTip);
        Withdrawal memory withdrawalA2 =
            Withdrawal(RequestId(Origin.L2, 2), users.withdrawerA, nativeTokenAddress, amount, ferryTip);
        Withdrawal memory withdrawalB =
            Withdrawal(RequestId(Origin.L2, 3), users.withdrawerB, nativeTokenAddress, amount, ferryTip);
        Withdrawal memory withdrawalC =
            Withdrawal(RequestId(Origin.L2, 4), users.withdrawerC, nativeTokenAddress, amount, ferryTip);

        bytes32[] memory hashes = new bytes32[](6);
        hashes[0] = rolldown.hashWithdrawal(withdrawalA);
        hashes[1] = rolldown.hashWithdrawal(withdrawalA2);
        hashes[2] = rolldown.hashWithdrawal(withdrawalB);
        hashes[3] = rolldown.hashWithdrawal(withdrawalC);
        hashes[4] = keccak256(abi.encodePacked(hashes[0], hashes[1]));
        hashes[5] = keccak256(abi.encodePacked(hashes[2], hashes[3]));

        bytes32 merkleRoot = keccak256(abi.encodePacked(hashes[4], hashes[5]));

        bytes32[][] memory proofs = new bytes32[][](4);
        proofs[0] = new bytes32[](2);
        proofs[0][0] = hashes[1];
        proofs[0][1] = hashes[5];
        proofs[1] = new bytes32[](2);
        proofs[1][0] = hashes[0];
        proofs[1][1] = hashes[5];
        proofs[2] = new bytes32[](2);
        proofs[2][0] = hashes[3];
        proofs[2][1] = hashes[4];
        proofs[3] = new bytes32[](2);
        proofs[3][0] = hashes[2];
        proofs[3][1] = hashes[4];

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, Range(1, 4));

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

        assertTrue(rolldown.processedL2Requests(hashes[0]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(hashes[1]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(hashes[2]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(hashes[3]) == rolldown.CLOSED());
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

        Withdrawal memory withdrawalA =
            Withdrawal(RequestId(Origin.L2, 1), users.withdrawerA, address(token), amount, ferryTip);
        Withdrawal memory withdrawalA2 =
            Withdrawal(RequestId(Origin.L2, 2), users.withdrawerA, address(token), amount, ferryTip);
        Withdrawal memory withdrawalB =
            Withdrawal(RequestId(Origin.L2, 3), users.withdrawerB, address(token), amount, ferryTip);
        Withdrawal memory withdrawalC =
            Withdrawal(RequestId(Origin.L2, 4), users.withdrawerC, address(token), amount, ferryTip);

        bytes32[] memory hashes = new bytes32[](6);
        hashes[0] = rolldown.hashWithdrawal(withdrawalA);
        hashes[1] = rolldown.hashWithdrawal(withdrawalA2);
        hashes[2] = rolldown.hashWithdrawal(withdrawalB);
        hashes[3] = rolldown.hashWithdrawal(withdrawalC);
        hashes[4] = keccak256(abi.encodePacked(hashes[0], hashes[1]));
        hashes[5] = keccak256(abi.encodePacked(hashes[2], hashes[3]));

        bytes32 merkleRoot = keccak256(abi.encodePacked(hashes[4], hashes[5]));

        bytes32[][] memory proofs = new bytes32[][](4);
        proofs[0] = new bytes32[](2);
        proofs[0][0] = hashes[1];
        proofs[0][1] = hashes[5];
        proofs[1] = new bytes32[](2);
        proofs[1][0] = hashes[0];
        proofs[1][1] = hashes[5];
        proofs[2] = new bytes32[](2);
        proofs[2][0] = hashes[3];
        proofs[2][1] = hashes[4];
        proofs[3] = new bytes32[](2);
        proofs[3][0] = hashes[2];
        proofs[3][1] = hashes[4];

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, Range(1, 4));

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

        assertTrue(rolldown.processedL2Requests(hashes[0]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(hashes[1]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(hashes[2]) == rolldown.CLOSED());
        assertTrue(rolldown.processedL2Requests(hashes[3]) == rolldown.CLOSED());
        assertEq(rolldown.lastProcessedUpdate_origin_l2(), 4);
    }

    function test_RevertIf_Paused() external {
        vm.prank(users.admin);
        rolldown.pause();

        Withdrawal memory withdrawalA =
            Withdrawal(RequestId(Origin.L2, 1), users.withdrawerA, nativeTokenAddress, amount, 0);
        Withdrawal memory withdrawalB =
            Withdrawal(RequestId(Origin.L2, 2), users.withdrawerB, nativeTokenAddress, amount, 0);

        bytes32 hashA = rolldown.hashWithdrawal(withdrawalA);
        bytes32 hashB = rolldown.hashWithdrawal(withdrawalB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(hashA, hashB));
        bytes32[] memory proof = new bytes32[](1);
        proof[0] = hashA;

        vm.prank(users.withdrawerB);
        vm.expectRevert("Pausable: paused");
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, proof);
    }

    function test_RevertIf_L2RequestAlreadyProcessed() external {
        vm.prank(users.depositor);
        rolldown.depositNative{value: amount}();

        Withdrawal memory withdrawalA =
            Withdrawal(RequestId(Origin.L2, 1), users.withdrawerA, nativeTokenAddress, amount, 0);
        Withdrawal memory withdrawalB =
            Withdrawal(RequestId(Origin.L2, 2), users.withdrawerB, nativeTokenAddress, amount, 0);

        bytes32 hashA = rolldown.hashWithdrawal(withdrawalA);
        bytes32 hashB = rolldown.hashWithdrawal(withdrawalB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(hashA, hashB));
        bytes32[] memory proof = new bytes32[](1);
        proof[0] = hashA;

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, Range(1, 2));

        vm.startPrank(users.withdrawerB);

        rolldown.closeWithdrawal(withdrawalB, merkleRoot, proof);

        vm.expectRevert(abi.encodeWithSelector(L2RequestAlreadyProcessed.selector, hashB));
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, proof);

        vm.stopPrank();
    }

    function test_RevertIf_UnexpectedMerkleRoot() external {
        vm.prank(users.depositor);
        rolldown.depositNative{value: amount}();

        Withdrawal memory withdrawalA =
            Withdrawal(RequestId(Origin.L2, 1), users.withdrawerA, nativeTokenAddress, amount, 0);
        Withdrawal memory withdrawalB =
            Withdrawal(RequestId(Origin.L2, 2), users.withdrawerB, nativeTokenAddress, amount, 0);

        bytes32 hashA = rolldown.hashWithdrawal(withdrawalA);
        bytes32 hashB = rolldown.hashWithdrawal(withdrawalB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(hashA, hashB));
        bytes32[] memory proof = new bytes32[](1);
        proof[0] = hashA;

        vm.prank(users.updater);
        vm.expectRevert(UnexpectedMerkleRoot.selector);
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, proof);
    }

    function test_RevertIf_RequestOutOfRange() external {
        vm.prank(users.depositor);
        rolldown.depositNative{value: amount}();

        Withdrawal memory withdrawalA =
            Withdrawal(RequestId(Origin.L2, 2), users.withdrawerA, nativeTokenAddress, amount, 0);
        Withdrawal memory withdrawalB =
            Withdrawal(RequestId(Origin.L2, 3), users.withdrawerB, nativeTokenAddress, amount, 0);

        bytes32 hashA = rolldown.hashWithdrawal(withdrawalA);
        bytes32 hashB = rolldown.hashWithdrawal(withdrawalB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(hashA, hashB));
        bytes32[] memory proof = new bytes32[](1);
        proof[0] = hashA;

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, Range(1, 2));

        vm.prank(users.withdrawerB);
        vm.expectRevert(abi.encodeWithSelector(RequestOutOfRange.selector, 3, 1, 2));
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, proof);
    }

    function test_RevertIf_RequestRangeTooLarge() external {
        vm.prank(users.depositor);
        rolldown.depositNative{value: amount}();

        Withdrawal memory withdrawalA =
            Withdrawal(RequestId(Origin.L2, 1), users.withdrawerA, nativeTokenAddress, amount, 0);
        Withdrawal memory withdrawalB =
            Withdrawal(RequestId(Origin.L2, 2), users.withdrawerB, nativeTokenAddress, amount, 0);

        bytes32 hashA = rolldown.hashWithdrawal(withdrawalA);
        bytes32 hashB = rolldown.hashWithdrawal(withdrawalB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(hashA, hashB));
        bytes32[] memory proof = new bytes32[](1);
        proof[0] = hashA;

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, Range(1, type(uint64).max));

        vm.prank(users.withdrawerB);
        vm.expectRevert(abi.encodeWithSelector(RequestRangeTooLarge.selector, type(uint64).max));
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, proof);
    }

    function test_RevertIf_InvalidRequestProof() external {
        vm.prank(users.depositor);
        rolldown.depositNative{value: amount}();

        Withdrawal memory withdrawalA =
            Withdrawal(RequestId(Origin.L2, 1), users.withdrawerA, nativeTokenAddress, amount, 0);
        Withdrawal memory withdrawalB =
            Withdrawal(RequestId(Origin.L2, 2), users.withdrawerB, nativeTokenAddress, amount, 0);

        bytes32 hashA = rolldown.hashWithdrawal(withdrawalA);
        bytes32 hashB = rolldown.hashWithdrawal(withdrawalB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(hashA, hashB));
        bytes32[] memory proof = new bytes32[](1);
        proof[0] = hashB;

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, Range(1, 2));

        vm.prank(users.withdrawerB);
        vm.expectRevert(abi.encodeWithSelector(InvalidRequestProof.selector, merkleRoot));
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, proof);
    }

    function test_RevertIf_ZeroTransferAmountNative() external {
        vm.prank(users.depositor);
        rolldown.depositNative{value: amount}();

        Withdrawal memory withdrawalA = Withdrawal(RequestId(Origin.L2, 1), users.withdrawerA, nativeTokenAddress, 0, 0);
        Withdrawal memory withdrawalB = Withdrawal(RequestId(Origin.L2, 2), users.withdrawerB, nativeTokenAddress, 0, 0);

        bytes32 hashA = rolldown.hashWithdrawal(withdrawalA);
        bytes32 hashB = rolldown.hashWithdrawal(withdrawalB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(hashA, hashB));
        bytes32[] memory proof = new bytes32[](1);
        proof[0] = hashA;

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, Range(1, 2));

        vm.prank(users.withdrawerB);
        vm.expectRevert(ZeroTransferAmount.selector);
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, proof);
    }

    function test_RevertIf_ZeroTransferAmountERC20() external {
        token.mint(users.depositor);

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount);
        rolldown.depositERC20(address(token), amount);

        vm.stopPrank();

        Withdrawal memory withdrawalA = Withdrawal(RequestId(Origin.L2, 1), users.withdrawerA, address(token), 0, 0);
        Withdrawal memory withdrawalB = Withdrawal(RequestId(Origin.L2, 2), users.withdrawerB, address(token), 0, 0);

        bytes32 hashA = rolldown.hashWithdrawal(withdrawalA);
        bytes32 hashB = rolldown.hashWithdrawal(withdrawalB);
        bytes32 merkleRoot = keccak256(abi.encodePacked(hashA, hashB));
        bytes32[] memory proof = new bytes32[](1);
        proof[0] = hashA;

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, Range(1, 2));

        vm.prank(users.withdrawerB);
        vm.expectRevert(ZeroTransferAmount.selector);
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, proof);
    }
}

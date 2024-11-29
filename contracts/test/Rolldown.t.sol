// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {EmptyContract} from "@eigenlayer/test/mocks/EmptyContract.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {Test} from "forge-std/Test.sol";
import {console} from "forge-std/console.sol";
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
        address recipient;
    }

    Users public users;
    MyERC20 public token;
    Rolldown public rolldown;
    bytes32 public defaultAdminRole = 0x00;
    bytes32 public updaterRole;
    address public nativeTokenAddress;
    address public closed;

    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
    event Transfer(address indexed from, address indexed to, uint256 value);

    function setUp() public virtual {
        users = Users({
            admin: makeAddr("admin"),
            grantee: makeAddr("grantee"),
            updater: makeAddr("updater"),
            depositor: makeAddr("depositor"),
            withdrawerA: makeAddr("withdrawerA"),
            withdrawerB: makeAddr("withdrawerB"),
            recipient: makeAddr("recipient")
        });

        deal(payable(users.admin), 100 ether);
        deal(payable(users.grantee), 100 ether);
        deal(payable(users.updater), 100 ether);
        deal(payable(users.depositor), 100 ether);
        deal(payable(users.withdrawerA), 100 ether);
        deal(payable(users.withdrawerB), 100 ether);
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
        closed = rolldown.CLOSED();
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

    function test_GetCounter() external view {
        assertEq(rolldown.counter(), 1);
    }

    function test_GetLastProcessedUpdateOriginL1() external view {
        assertEq(rolldown.lastProcessedUpdate_origin_l1(), 0);
    }

    function test_GetLastProcessedUpdateOriginL2() external view {
        assertEq(rolldown.lastProcessedUpdate_origin_l2(), 0);
    }

    function test_GetChainId() external view {
        assertEq(uint256(rolldown.chain()), uint256(ChainId.Ethereum));
    }

    function test_GetUpdaterAccount() external view {
        assertEq(rolldown.updaterAccount(), users.updater);
    }

    function test_GetRootsLength() external view {
        assertEq(rolldown.getMerkleRootsLength(), 0);
    }

    function test_GetPendingRequests() external view {
        L1Update memory l1Update = rolldown.getPendingRequests(0, 0);
        assertEq(uint256(l1Update.chain), uint256(ChainId.Ethereum));
        assertEq(l1Update.pendingDeposits.length, 0);
        assertEq(l1Update.pendingCancelResolutions.length, 0);
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

    function test_RevertIf_InvalidRequestRange() external {
        vm.expectRevert(abi.encodeWithSelector(InvalidRequestRange.selector, 0, 1));
        rolldown.getPendingRequests(0, 1);
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

    function test_GetNewUpdater() external {
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
    bytes32 public merkleRoot;
    Range public range = Range(1, 2);

    function setUp() public override {
        super.setUp();

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

        merkleRoot = keccak256(abi.encodePacked(withdrawalHashA, withdrawalHashB));
    }

    function test_EmitL2UpdateAccepted() external {
        vm.prank(users.updater);
        vm.expectEmit();
        emit L2UpdateAccepted(merkleRoot, range);
        rolldown.updateL1FromL2(merkleRoot, range);
    }

    function test_EmitL2UpdateAccepted_BackwardCompatibility() external {
        vm.prank(users.updater);
        vm.expectEmit();
        emit L2UpdateAccepted(merkleRoot, range);
        rolldown.update_l1_from_l2(merkleRoot, range);
    }

    function test_GetRoots() external {
        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, range);
        assertEq(rolldown.roots(0), merkleRoot);
    }

    function test_GetRootsLength() external {
        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, range);
        assertEq(rolldown.getMerkleRootsLength(), 1);
    }

    function test_GetMerkeRootRange() external {
        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, range);

        (uint256 startRange, uint256 endRange) = rolldown.merkleRootRange(merkleRoot);
        assertEq(startRange, range.start);
        assertEq(endRange, range.end);
    }

    function test_GetLastProcessedUpdateOriginL2() external {
        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, range);
        assertEq(rolldown.lastProcessedUpdate_origin_l2(), range.end);
    }

    function test_RevertIf_Paused() external {
        vm.prank(users.admin);
        rolldown.pause();

        range = Range(1, 1);

        vm.prank(users.updater);
        vm.expectRevert("Pausable: paused");
        rolldown.updateL1FromL2(bytes32(""), range);
    }

    function test_RevertIf_ZeroUpdateRange() external {
        range = Range(0, 1);

        vm.prank(users.updater);
        vm.expectRevert(ZeroUpdateRange.selector);
        rolldown.updateL1FromL2(bytes32(""), range);
    }

    function test_RevertIf_InvalidRangeStart() external {
        range = Range(1, 0);

        vm.prank(users.updater);
        vm.expectRevert(abi.encodeWithSelector(InvalidUpdateRange.selector, range.start, range.end));
        rolldown.updateL1FromL2(bytes32(""), range);
    }

    function test_RevertIf_PreviousUpdateMissed() external {
        range = Range(2, 3);
        uint256 lastProcessedUpdate = rolldown.lastProcessedUpdate_origin_l2();

        vm.prank(users.updater);
        vm.expectRevert(abi.encodeWithSelector(PreviousUpdateMissed.selector, range.start, lastProcessedUpdate));
        rolldown.updateL1FromL2(merkleRoot, range);
    }

    function test_RevertIf_UpdateAlreadyApplied() external {
        vm.startPrank(users.updater);

        rolldown.updateL1FromL2(merkleRoot, range);
        uint256 lastProcessedUpdate = rolldown.lastProcessedUpdate_origin_l2();

        vm.expectRevert(abi.encodeWithSelector(UpdateAlreadyApplied.selector, range.end, lastProcessedUpdate));
        rolldown.updateL1FromL2(merkleRoot, range);

        vm.stopPrank();
    }
}

contract CloseWithdrawal is RolldownTest {
    uint256 public amount = 1 ether;
    bytes32 public merkleRoot;
    Withdrawal public withdrawalA;
    Withdrawal public withdrawalB;
    bytes32 public withdrawalHashA;
    bytes32 public withdrawalHashB;
    Range public range = Range(1, 2);

    function setUp() public override {
        super.setUp();

        deal(address(rolldown), amount);
        token.mint(users.withdrawerB);

        withdrawalA = Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, 0);
        withdrawalHashA = rolldown.hashWithdrawal(withdrawalA);
        withdrawalB = Withdrawal(createRequestId(Origin.L2), users.recipient, address(token), amount, 0);
        withdrawalHashB = rolldown.hashWithdrawal(withdrawalB);

        vm.prank(users.withdrawerA);
        rolldown.ferryWithdrawal{value: amount}(withdrawalA);

        vm.startPrank(users.withdrawerB);

        token.approve(address(rolldown), amount);
        rolldown.ferryWithdrawal(withdrawalB);

        vm.stopPrank();

        merkleRoot = keccak256(abi.encodePacked(withdrawalHashA, withdrawalHashB));

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, range);
    }

    function test_EmitNativeTokensWithdrawnAndWithdrawalClosed() external {
        bytes32[] memory merkleProof = new bytes32[](1);
        merkleProof[0] = withdrawalHashB;

        vm.prank(users.withdrawerA);
        vm.expectEmit();
        emit NativeTokensWithdrawn(users.withdrawerA, amount);
        emit WithdrawalClosed(2, withdrawalHashA);
        rolldown.closeWithdrawal(withdrawalA, merkleRoot, merkleProof);
    }

    function test_EmitNativeTokensWithdrawnAndWithdrawalClosed_BackwardCompatibility() external {
        bytes32[] memory merkleProof = new bytes32[](1);
        merkleProof[0] = withdrawalHashB;

        vm.prank(users.withdrawerA);
        vm.expectEmit();
        emit NativeTokensWithdrawn(users.withdrawerA, amount);
        emit WithdrawalClosed(2, withdrawalHashA);
        rolldown.close_withdrawal(withdrawalA, merkleRoot, merkleProof);
    }

    function test_GetProcessedL2Request() external {
        bytes32[] memory merkleProof = new bytes32[](1);
        merkleProof[0] = withdrawalHashB;

        vm.prank(users.withdrawerA);
        rolldown.closeWithdrawal(withdrawalA, merkleRoot, merkleProof);

        assertEq(rolldown.processedL2Requests(withdrawalHashA), closed);
        assertNotEq(rolldown.processedL2Requests(withdrawalHashB), closed);
    }

    function test_RevertIf_Paused() external {
        vm.prank(users.admin);
        rolldown.pause();

        bytes32[] memory merkleProof = new bytes32[](1);
        merkleProof[0] = withdrawalHashB;

        vm.prank(users.withdrawerA);
        vm.expectRevert("Pausable: paused");
        rolldown.closeWithdrawal(withdrawalB, merkleRoot, merkleProof);
    }

    function test_RevertIf_L2RequestAlreadyProcessed() external {
        bytes32[] memory merkleProof = new bytes32[](1);
        merkleProof[0] = withdrawalHashB;

        vm.startPrank(users.withdrawerA);

        rolldown.closeWithdrawal(withdrawalA, merkleRoot, merkleProof);

        vm.expectRevert(abi.encodeWithSelector(L2RequestAlreadyProcessed.selector, withdrawalHashA));
        rolldown.closeWithdrawal(withdrawalA, merkleRoot, merkleProof);

        vm.stopPrank();
    }

    function test_RevertIf_UnexpectedMerkleRoot() external {
        merkleRoot = keccak256(abi.encodePacked(withdrawalHashB, withdrawalHashA));
        bytes32[] memory merkleProof = new bytes32[](1);
        merkleProof[0] = withdrawalHashB;

        vm.prank(users.withdrawerA);
        vm.expectRevert(UnexpectedMerkleRoot.selector);
        rolldown.closeWithdrawal(withdrawalA, merkleRoot, merkleProof);
    }

    function test_RevertIf_RequestOutOfRange() external {
        token.mint(users.withdrawerB);

        Withdrawal memory withdrawalC = Withdrawal(RequestId(Origin.L2, 3), users.recipient, nativeTokenAddress, amount, 0);
        bytes32 withdrawalHashC = rolldown.hashWithdrawal(withdrawalC);
        Withdrawal memory withdrawalD = Withdrawal(RequestId(Origin.L2, 4), users.recipient, address(token), amount, 0);

        vm.prank(users.withdrawerA);
        rolldown.ferryWithdrawal{value: amount}(withdrawalC);

        vm.startPrank(users.withdrawerB);

        token.approve(address(rolldown), amount);
        rolldown.ferryWithdrawal(withdrawalD);

        vm.stopPrank();

        merkleRoot = keccak256(abi.encodePacked(withdrawalHashC));
        bytes32[] memory merkleProof = new bytes32[](1);
        merkleProof[0] = withdrawalHashC;

        range = Range(3, 3);
        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, range);

        vm.prank(users.withdrawerB);
        vm.expectRevert(abi.encodeWithSelector(RequestOutOfRange.selector, 4, range.start, range.end));
        rolldown.closeWithdrawal(withdrawalD, merkleRoot, merkleProof);
    }

    function test_RevertIf_RequestRangeTooLarge() external {
        token.mint(users.withdrawerB);

        Withdrawal memory withdrawalC = Withdrawal(RequestId(Origin.L2, 3), users.recipient, nativeTokenAddress, amount, 0);
        bytes32 withdrawalHashC = rolldown.hashWithdrawal(withdrawalC);
        Withdrawal memory withdrawalD = Withdrawal(RequestId(Origin.L2, 4), users.recipient, address(token), amount, 0);

        vm.prank(users.withdrawerA);
        rolldown.ferryWithdrawal{value: amount}(withdrawalC);

        vm.startPrank(users.withdrawerB);

        token.approve(address(rolldown), amount);
        rolldown.ferryWithdrawal(withdrawalD);

        vm.stopPrank();

        merkleRoot = keccak256(abi.encodePacked(withdrawalHashC));
        bytes32[] memory merkleProof = new bytes32[](1);
        merkleProof[0] = withdrawalHashC;

        range = Range(3, type(uint64).max);
        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, range);

        vm.prank(users.withdrawerB);
        vm.expectRevert(abi.encodeWithSelector(RequestRangeTooLarge.selector, type(uint64).max - 2));
        rolldown.closeWithdrawal(withdrawalD, merkleRoot, merkleProof);
    }

    function test_RevertIf_InvalidRequestProof() external {
        bytes32[] memory merkleProof = new bytes32[](1);
        merkleProof[0] = withdrawalHashA;

        vm.prank(users.withdrawerA);
        vm.expectRevert(abi.encodeWithSelector(InvalidRequestProof.selector, merkleRoot));
        rolldown.closeWithdrawal(withdrawalA, merkleRoot, merkleProof);
    }
}

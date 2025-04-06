// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {EmptyContract} from "@eigenlayer/test/mocks/EmptyContract.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {Test} from "forge-std/Test.sol";
import {Merkle} from "murky/Merkle.sol";
import {IRolldownPrimitives} from "../src/interfaces/IRolldownPrimitives.sol";
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

    Users internal users;
    MyERC20 internal token;
    Rolldown internal rolldown;
    bytes32 internal defaultAdminRole = 0x00;
    bytes32 internal updaterRole;
    address internal nativeTokenAddress;
    address internal closed;

    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
    event Paused(address account);
    event Unpaused(address account);
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

        deal(payable(users.depositor), 100 ether);
        deal(payable(users.withdrawerA), 100 ether);

        token = new MyERC20();

        Rolldown implementation = new Rolldown();
        ProxyAdmin proxyAdmin = new ProxyAdmin();
        TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(
            address(implementation),
            address(proxyAdmin),
            abi.encodeCall(Rolldown.initialize, (users.admin, ChainId.Ethereum, users.updater))
        );
        rolldown = Rolldown(payable(address(proxy)));

        vm.label(address(this), "RolldownTest");
        vm.label(address(token), "ERC20Token");
        vm.label(address(proxy), "Rolldown");

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
    function test_HasDefaultAdminRole() public view {
        assertTrue(rolldown.hasRole(defaultAdminRole, users.admin));
    }

    function test_HasUpdaterRole() public view {
        assertTrue(rolldown.hasRole(updaterRole, users.updater));
    }

    function test_IsPaused() public view {
        assertFalse(rolldown.paused());
    }

    function test_GetCounter() public view {
        assertEq(rolldown.counter(), 1);
    }

    function test_GetLastProcessedUpdateOriginL1() public view {
        assertEq(rolldown.lastProcessedUpdate_origin_l1(), 0);
    }

    function test_GetLastProcessedUpdateOriginL2() public view {
        assertEq(rolldown.lastProcessedUpdate_origin_l2(), 0);
    }

    function test_GetChainId() public view {
        assertEq(uint256(rolldown.chain()), uint256(ChainId.Ethereum));
    }

    function test_GetUpdaterAccount() public view {
        assertEq(rolldown.updaterAccount(), users.updater);
    }

    function test_GetRootsLength() public view {
        assertEq(rolldown.getMerkleRootsLength(), 0);
    }

    function test_GetPendingRequests() public view {
        L1Update memory l1Update = rolldown.getPendingRequests(0, 0);
        assertEq(uint256(l1Update.chain), uint256(ChainId.Ethereum));
        assertEq(l1Update.pendingDeposits.length, 0);
        assertEq(l1Update.pendingCancelResolutions.length, 0);
    }

    function test_RevertIf_AlreadyInitialized() public {
        vm.expectRevert("Initializable: contract is already initialized");
        rolldown.initialize(users.admin, ChainId.Ethereum, users.updater);
    }

    function test_RevertIf_ZeroAdmin() public {
        Rolldown implementation = new Rolldown();
        ProxyAdmin proxyAdmin = new ProxyAdmin();

        vm.expectRevert(ZeroAdmin.selector);
        new TransparentUpgradeableProxy(
            address(implementation),
            address(proxyAdmin),
            abi.encodeCall(Rolldown.initialize, (address(0), ChainId.Ethereum, users.updater))
        );
    }

    function test_RevertIf_ZeroUpdater() public {
        Rolldown implementation = new Rolldown();
        ProxyAdmin proxyAdmin = new ProxyAdmin();

        vm.expectRevert(ZeroUpdater.selector);
        new TransparentUpgradeableProxy(
            address(implementation),
            address(proxyAdmin),
            abi.encodeCall(Rolldown.initialize, (users.admin, ChainId.Ethereum, address(0)))
        );
    }

    function test_RevertIf_InvalidRequestRange() public {
        vm.expectRevert(abi.encodeWithSelector(InvalidRequestRange.selector, 0, 1));
        rolldown.getPendingRequests(0, 1);
    }
}

contract GrantRole is RolldownTest {
    function test_EmitRoleGranted() public {
        vm.prank(users.admin);
        vm.expectEmit();
        emit RoleGranted(defaultAdminRole, users.updater, users.admin);
        rolldown.grantRole(defaultAdminRole, users.updater);
    }

    function test_HasDefaultAdminRole() public {
        vm.prank(users.admin);
        rolldown.grantRole(defaultAdminRole, users.grantee);
        assertTrue(rolldown.hasRole(defaultAdminRole, users.grantee));
    }

    function test_RevertIf_UnauthorizedAccount() public {
        vm.prank(users.grantee);
        vm.expectRevert(accessControlMessage(users.grantee));
        rolldown.grantRole(defaultAdminRole, users.grantee);
    }
}

contract RevokeRole is RolldownTest {
    function test_EmitRoleRevoked() public {
        vm.startPrank(users.admin);

        rolldown.grantRole(defaultAdminRole, users.grantee);

        vm.expectEmit();
        emit RoleRevoked(defaultAdminRole, users.grantee, users.admin);
        rolldown.revokeRole(defaultAdminRole, users.grantee);

        vm.stopPrank();
    }

    function test_HasNotDefaultAdminRole() public {
        vm.startPrank(users.admin);

        rolldown.grantRole(defaultAdminRole, users.grantee);
        rolldown.revokeRole(defaultAdminRole, users.grantee);
        assertFalse(rolldown.hasRole(defaultAdminRole, users.grantee));

        vm.stopPrank();
    }

    function test_RevertIf_UnauthorizedAccount() public {
        vm.prank(users.admin);
        rolldown.grantRole(defaultAdminRole, users.grantee);

        vm.prank(users.depositor);
        vm.expectRevert(accessControlMessage(users.depositor));
        rolldown.revokeRole(defaultAdminRole, users.grantee);
    }
}

contract RenounceRole is RolldownTest {
    function test_EmitRoleRevoked() public {
        vm.prank(users.admin);
        rolldown.grantRole(defaultAdminRole, users.grantee);

        vm.prank(users.grantee);
        vm.expectEmit();
        emit RoleRevoked(defaultAdminRole, users.grantee, users.grantee);
        rolldown.renounceRole(defaultAdminRole, users.grantee);
    }

    function test_HasNotDefaultAdminRole() public {
        vm.prank(users.admin);
        rolldown.grantRole(defaultAdminRole, users.grantee);

        vm.prank(users.grantee);
        rolldown.renounceRole(defaultAdminRole, users.grantee);
        assertFalse(rolldown.hasRole(defaultAdminRole, users.grantee));
    }

    function test_RevertIf_BadConfirmation() public {
        vm.startPrank(users.admin);

        rolldown.grantRole(defaultAdminRole, users.grantee);

        vm.expectRevert("AccessControl: can only renounce roles for self");
        rolldown.renounceRole(defaultAdminRole, users.grantee);

        vm.stopPrank();
    }
}

contract Pause is RolldownTest {
    function test_EmitPaused() public {
        vm.prank(users.admin);
        vm.expectEmit();
        emit Paused(users.admin);
        rolldown.pause();
    }

    function test_GetPaused() public {
        vm.prank(users.admin);
        rolldown.pause();

        assertTrue(rolldown.paused());
    }

    function test_RevertIf_Paused() public {
        vm.startPrank(users.admin);
        rolldown.pause();

        vm.expectRevert("Pausable: paused");
        rolldown.pause();

        vm.stopPrank();
    }
}

contract Unpause is RolldownTest {
    function test_EmitUnpaused() public {
        vm.startPrank(users.admin);

        rolldown.pause();

        vm.expectEmit();
        emit Unpaused(users.admin);
        rolldown.unpause();

        vm.stopPrank();
    }

    function test_GetPaused() public {
        vm.startPrank(users.admin);

        rolldown.pause();

        vm.expectEmit();
        emit Unpaused(users.admin);
        rolldown.unpause();

        vm.stopPrank();

        assertFalse(rolldown.paused());
    }

    function test_RevertIf_NotPaused() public {
        vm.prank(users.admin);
        vm.expectRevert("Pausable: not paused");
        rolldown.unpause();
    }
}

contract SetUpdater is RolldownTest {
    address public newUpdater = makeAddr("newUpdater");

    function test_EmitNewUpdaterSet() public {
        vm.prank(users.admin);
        vm.expectEmit();
        emit NewUpdaterSet(newUpdater);
        rolldown.setUpdater(newUpdater);
    }

    function test_GetNewUpdater() public {
        vm.prank(users.admin);
        rolldown.setUpdater(newUpdater);
        assertEq(rolldown.updaterAccount(), newUpdater);
    }

    function test_HasUpdaterRole() public {
        vm.prank(users.admin);
        rolldown.setUpdater(newUpdater);
        assertTrue(rolldown.hasRole(updaterRole, newUpdater));
    }

    function test_RevertIf_UnauthorizedAccount() public {
        vm.prank(users.depositor);
        vm.expectRevert(accessControlMessage(users.depositor));
        rolldown.setUpdater(newUpdater);
    }

    function test_RevertIf_ZeroUpdater() public {
        vm.prank(users.admin);
        vm.expectRevert(ZeroUpdater.selector);
        rolldown.setUpdater(address(0));
    }
}

contract DepositNative is RolldownTest {
    uint256 public amount = 1 ether;
    uint256 public ferryTip = 0;

    function test_EmitDepositAcceptedIntoQueue() public {
        uint256 requestId = rolldown.counter();

        vm.prank(users.depositor);
        vm.expectEmit();
        emit DepositAcceptedIntoQueue(requestId, users.depositor, nativeTokenAddress, amount, ferryTip);
        rolldown.depositNative{value: amount}();
    }

    function test_EmitDepositAcceptedIntoQueue_BackwardCompatibility() public {
        uint256 requestId = rolldown.counter();

        vm.prank(users.depositor);
        vm.expectEmit();
        emit DepositAcceptedIntoQueue(requestId, users.depositor, nativeTokenAddress, amount, ferryTip);
        rolldown.deposit_native{value: amount}();
    }

    function test_ChangeBalances() public {
        uint256 initialDepositorBalance = users.depositor.balance;
        uint256 initialRolldownBalance = address(rolldown).balance;

        vm.prank(users.depositor);
        rolldown.depositNative{value: amount}();

        uint256 currentDepositorBalance = users.depositor.balance;
        assertEq(initialDepositorBalance - currentDepositorBalance, amount);

        uint256 currentRolldownBalance = address(rolldown).balance;
        assertEq(currentRolldownBalance - initialRolldownBalance, amount);
    }

    function test_GetUpdateForL2() public {
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

    function test_RevertIf_Paused() public {
        vm.prank(users.admin);
        rolldown.pause();

        vm.prank(users.depositor);
        vm.expectRevert("Pausable: paused");
        rolldown.depositNative{value: amount}();
    }

    function test_RevertIf_ZeroAmount() public {
        vm.prank(users.depositor);
        vm.expectRevert(ZeroAmount.selector);
        rolldown.depositNative{value: 0}();
    }
}

contract DepositNativeWithFerryTip is RolldownTest {
    uint256 public amount = 1 ether;
    uint256 public ferryTip = amount / 100;

    function test_EmitDepositAcceptedIntoQueue() public {
        uint256 requestId = rolldown.counter();

        vm.prank(users.depositor);
        vm.expectEmit();
        emit DepositAcceptedIntoQueue(requestId, users.depositor, nativeTokenAddress, amount, ferryTip);
        rolldown.depositNative{value: amount}(ferryTip);
    }

    function test_EmitDepositAcceptedIntoQueue_BackwardCompatibility() public {
        uint256 requestId = rolldown.counter();

        vm.prank(users.depositor);
        vm.expectEmit();
        emit DepositAcceptedIntoQueue(requestId, users.depositor, nativeTokenAddress, amount, ferryTip);
        rolldown.deposit_native{value: amount}(ferryTip);
    }

    function test_ChangeBalances() public {
        uint256 initialDepositorBalance = users.depositor.balance;
        uint256 initialRolldownBalance = address(rolldown).balance;

        vm.prank(users.depositor);
        rolldown.depositNative{value: amount}(ferryTip);

        uint256 currentDepositorBalance = users.depositor.balance;
        assertEq(initialDepositorBalance - currentDepositorBalance, amount);

        uint256 currentRolldownBalance = address(rolldown).balance;
        assertEq(currentRolldownBalance - initialRolldownBalance, amount);
    }

    function test_GetUpdateForL2() public {
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

    function test_RevertIf_Paused() public {
        vm.prank(users.admin);
        rolldown.pause();

        vm.prank(users.depositor);
        vm.expectRevert("Pausable: paused");
        rolldown.depositNative{value: amount}(ferryTip);
    }

    function test_RevertIf_ZeroAmount() public {
        vm.prank(users.depositor);
        vm.expectRevert(ZeroAmount.selector);
        rolldown.depositNative{value: 0}(ferryTip);
    }

    function test_RevertIf_FerryTipExceedsAmount() public {
        ferryTip = amount + 1;

        vm.prank(users.depositor);
        vm.expectRevert(abi.encodeWithSelector(FerryTipExceedsAmount.selector, ferryTip, amount));
        rolldown.depositNative{value: amount}(ferryTip);
    }
}

contract DepositERC20 is RolldownTest {
    uint256 public amount = 1 ether;
    uint256 public ferryTip = 0;

    function test_EmitDepositAcceptedIntoQueue() public {
        token.mint(users.depositor);

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount);
        uint256 requestId = rolldown.counter();

        vm.expectEmit();
        emit DepositAcceptedIntoQueue(requestId, users.depositor, address(token), amount, ferryTip);
        rolldown.depositERC20(address(token), amount);

        vm.stopPrank();
    }

    function test_EmitDepositAcceptedIntoQueue_BackwardCompatibility() public {
        token.mint(users.depositor);

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount);
        uint256 requestId = rolldown.counter();

        vm.expectEmit();
        emit DepositAcceptedIntoQueue(requestId, users.depositor, address(token), amount, ferryTip);
        rolldown.deposit_erc20(address(token), amount);

        vm.stopPrank();
    }

    function test_EmitDepositAcceptedIntoQueue_BackwardCompatibility2() public {
        token.mint(users.depositor);

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount);
        uint256 requestId = rolldown.counter();

        vm.expectEmit();
        emit DepositAcceptedIntoQueue(requestId, users.depositor, address(token), amount, ferryTip);
        rolldown.deposit(address(token), amount);

        vm.stopPrank();
    }

    function test_ChangeBalances() public {
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

    function test_GetUpdateForL2() public {
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

    function test_RevertIf_Paused() public {
        vm.prank(users.admin);
        rolldown.pause();

        vm.prank(users.depositor);
        vm.expectRevert("Pausable: paused");
        rolldown.depositERC20(address(token), amount);
    }

    function test_RevertIf_ZeroAmount() public {
        vm.prank(users.depositor);
        vm.expectRevert(ZeroAmount.selector);
        rolldown.depositERC20(address(token), 0);
    }

    function test_RevertIf_ZeroToken() public {
        vm.prank(users.depositor);
        vm.expectRevert(ZeroToken.selector);
        rolldown.depositERC20(address(0), amount);
    }

    function test_RevertIf_InsufficientAllowance() public {
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

    function test_EmitDepositAcceptedIntoQueue() public {
        token.mint(users.depositor);

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount);
        uint256 requestId = rolldown.counter();

        vm.expectEmit();
        emit DepositAcceptedIntoQueue(requestId, users.depositor, address(token), amount, ferryTip);
        rolldown.depositERC20(address(token), amount, ferryTip);

        vm.stopPrank();
    }

    function test_EmitDepositAcceptedIntoQueue_BackwardCompatibility() public {
        token.mint(users.depositor);

        vm.startPrank(users.depositor);

        token.approve(address(rolldown), amount);
        uint256 requestId = rolldown.counter();

        vm.expectEmit();
        emit DepositAcceptedIntoQueue(requestId, users.depositor, address(token), amount, ferryTip);
        rolldown.deposit_erc20(address(token), amount, ferryTip);

        vm.stopPrank();
    }

    function test_ChangeBalances() public {
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

    function test_GetUpdateForL2() public {
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

    function test_RevertIf_Paused() public {
        vm.prank(users.admin);
        rolldown.pause();

        vm.prank(users.depositor);
        vm.expectRevert("Pausable: paused");
        rolldown.depositERC20(address(token), amount, ferryTip);
    }

    function test_RevertIf_ZeroAmount() public {
        vm.prank(users.depositor);
        vm.expectRevert(ZeroAmount.selector);
        rolldown.depositERC20(address(token), 0, ferryTip);
    }

    function test_RevertIf_ZeroToken() public {
        vm.prank(users.depositor);
        vm.expectRevert(ZeroToken.selector);
        rolldown.depositERC20(address(0), amount, ferryTip);
    }

    function test_RevertIf_FerryTipExceedsAmount() public {
        ferryTip = amount + 1;

        vm.prank(users.depositor);
        vm.expectRevert(abi.encodeWithSelector(FerryTipExceedsAmount.selector, ferryTip, amount));
        rolldown.depositERC20(address(token), amount, ferryTip);
    }

    function test_RevertIf_InsufficientAllowance() public {
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

    function test_EmitWithdrawalFerried_Native_WithoutFerryTip() public {
        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, 0);
        bytes32 withdrawalHash = rolldown.hashWithdrawal(withdrawal);

        vm.prank(users.withdrawerA);
        vm.expectEmit();
        emit WithdrawalFerried(withdrawal.requestId.id, amount, users.recipient, users.withdrawerA, withdrawalHash);
        rolldown.ferryWithdrawal{value: amount}(withdrawal);
    }

    function test_EmitWithdrawalFerried_Native_WithoutFerryTip_BackwardCompatibility() public {
        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, 0);
        bytes32 withdrawalHash = rolldown.hashWithdrawal(withdrawal);

        vm.prank(users.withdrawerA);
        vm.expectEmit();
        emit WithdrawalFerried(withdrawal.requestId.id, amount, users.recipient, users.withdrawerA, withdrawalHash);
        rolldown.ferry_withdrawal{value: amount}(withdrawal);
    }

    function test_EmitWithdrawalFerried_Native_WithFerryTip() public {
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

    function test_EmitWithdrawalFerried_ERC20_WithoutFerryTip() public {
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

    function test_EmitWithdrawalFerried_ERC20_WithoutFerryTip_BackwardCompatibility() public {
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

    function test_EmitWithdrawalFerried_ERC20_WithFerryTip() public {
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

    function test_ChangeBalances_Native_WithoutFerryTip() public {
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

    function test_ChangeBalances_Native_WithFerryTip() public {
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

    function test_ChangeBalances_ERC20_WithoutFerryTip() public {
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

    function test_ChangeBalances_ERC20_WithFerryTip() public {
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

    function test_GetProcessedL2Requests_Native() public {
        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, ferryTip);
        bytes32 withdrawalHash = rolldown.hashWithdrawal(withdrawal);

        vm.prank(users.withdrawerA);
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
        assertEq(rolldown.processedL2Requests(withdrawalHash), users.withdrawerA);
    }

    function test_GetProcessedL2Requests_ERC20() public {
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

    function test_RevertIf_Paused() public {
        vm.prank(users.admin);
        rolldown.pause();

        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, address(token), amount, ferryTip);

        vm.prank(users.withdrawerA);
        vm.expectRevert("Pausable: paused");
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
    }

    function test_RevertIf_ZeroAmount_Native() public {
        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, 0, ferryTip);

        vm.prank(users.withdrawerA);
        vm.expectRevert(ZeroAmount.selector);
        rolldown.ferryWithdrawal{value: 0}(withdrawal);
    }

    function test_RevertIf_ZeroFerriedAmount_ERC20() public {
        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, address(token), 0, ferryTip);

        vm.prank(users.withdrawerA);
        vm.expectRevert(ZeroAmount.selector);
        rolldown.ferryWithdrawal(withdrawal);
    }

    function test_RevertIf_FerryTipExceedsAmount_Native() public {
        ferryTip = amount + 1;

        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, ferryTip);

        vm.prank(users.withdrawerA);
        vm.expectRevert(abi.encodeWithSelector(FerryTipExceedsAmount.selector, ferryTip, amount));
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
    }

    function test_RevertIf_FerryTipExceedsAmount_ERC20() public {
        ferryTip = amount + 1;

        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, address(token), amount, ferryTip);

        vm.prank(users.withdrawerA);
        vm.expectRevert(abi.encodeWithSelector(FerryTipExceedsAmount.selector, ferryTip, amount));
        rolldown.ferryWithdrawal(withdrawal);
    }

    function test_RevertIf_ZeroRecipient() public {
        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), address(0), nativeTokenAddress, amount, ferryTip);

        vm.prank(users.withdrawerA);
        vm.expectRevert(ZeroRecipient.selector);
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
    }

    function test_RevertIf_InsufficientFerriedAmount_Native() public {
        ferriedAmount -= 1;

        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, ferryTip);

        vm.prank(users.withdrawerA);
        vm.expectRevert(abi.encodeWithSelector(InvalidFerriedAmount.selector, ferriedAmount, amount - ferryTip));
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
    }

    function test_RevertIf_ZeroFerriedAmount_Native() public {
        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, ferryTip);

        vm.prank(users.withdrawerA);
        vm.expectRevert(abi.encodeWithSelector(InvalidFerriedAmount.selector, 0, ferriedAmount));
        rolldown.ferryWithdrawal{value: 0}(withdrawal);
    }

    function test_RevertIf_WithdrawalAlreadyFerried() public {
        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, ferryTip);
        bytes32 withdrawalHash = rolldown.hashWithdrawal(withdrawal);

        vm.startPrank(users.withdrawerA);

        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);

        vm.expectRevert(abi.encodeWithSelector(WithdrawalAlreadyFerried.selector, withdrawalHash));
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);

        vm.stopPrank();
    }

    function test_RevertIf_CallToNonContract() public {
        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, address(0), amount, ferryTip);

        vm.prank(users.withdrawerA);
        vm.expectRevert("Address: call to non-contract");
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
    }

    function test_RevertIf_LowLevelCallFailed() public {
        Withdrawal memory withdrawal =
            Withdrawal(createRequestId(Origin.L2), users.recipient, address(new EmptyContract()), amount, ferryTip);

        vm.prank(users.withdrawerA);
        vm.expectRevert("SafeERC20: low-level call failed");
        rolldown.ferryWithdrawal{value: ferriedAmount}(withdrawal);
    }

    function test_RevertIf_InsufficientAllowance() public {
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

    function test_EmitL2UpdateAccepted() public {
        vm.prank(users.updater);
        vm.expectEmit();
        emit L2UpdateAccepted(merkleRoot, range);
        rolldown.updateL1FromL2(merkleRoot, range);
    }

    function test_EmitL2UpdateAccepted_BackwardCompatibility() public {
        vm.prank(users.updater);
        vm.expectEmit();
        emit L2UpdateAccepted(merkleRoot, range);
        rolldown.update_l1_from_l2(merkleRoot, range);
    }

    function test_GetRoots() public {
        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, range);
        assertEq(rolldown.roots(0), merkleRoot);
    }

    function test_GetRootsLength() public {
        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, range);
        assertEq(rolldown.getMerkleRootsLength(), 1);
    }

    function test_GetMerkeRootRange() public {
        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, range);

        (uint256 startRange, uint256 endRange) = rolldown.merkleRootRange(merkleRoot);
        assertEq(startRange, range.start);
        assertEq(endRange, range.end);
    }

    function test_GetLastProcessedUpdateOriginL2() public {
        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, range);
        assertEq(rolldown.lastProcessedUpdate_origin_l2(), range.end);
    }

    function test_FindL2Batch() public {
        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, range);

        uint256 requestId = rolldown.lastProcessedUpdate_origin_l2();

        bytes32 l2Batch = rolldown.findL2Batch(requestId);
        assertEq(l2Batch, merkleRoot);
    }

    function test_RevertIf_BatchNotFound() public {
        vm.prank(users.updater);

        uint256 requestId = rolldown.lastProcessedUpdate_origin_l2();

        vm.expectRevert(BatchNotFound.selector);
        rolldown.findL2Batch(requestId);
    }

    function test_RevertIf_InvalidRequestId() public {
        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, range);

        uint256 requestId = rolldown.lastProcessedUpdate_origin_l2() + 1;
        vm.expectRevert(abi.encodeWithSelector(InvalidRequestId.selector, requestId));
        rolldown.findL2Batch(requestId);
    }

    function test_RevertIf_Paused() public {
        vm.prank(users.admin);
        rolldown.pause();

        range = Range(1, 1);

        vm.prank(users.updater);
        vm.expectRevert("Pausable: paused");
        rolldown.updateL1FromL2(bytes32(""), range);
    }

    function test_RevertIf_ZeroUpdateRange() public {
        range = Range(0, 1);

        vm.prank(users.updater);
        vm.expectRevert(ZeroUpdateRange.selector);
        rolldown.updateL1FromL2(bytes32(""), range);
    }

    function test_RevertIf_InvalidRangeStart() public {
        range = Range(1, 0);

        vm.prank(users.updater);
        vm.expectRevert(abi.encodeWithSelector(InvalidUpdateRange.selector, range.start, range.end));
        rolldown.updateL1FromL2(bytes32(""), range);
    }

    function test_RevertIf_PreviousUpdateMissed() public {
        range = Range(2, 3);
        uint256 lastProcessedUpdate = rolldown.lastProcessedUpdate_origin_l2();

        vm.prank(users.updater);
        vm.expectRevert(abi.encodeWithSelector(PreviousUpdateMissed.selector, range.start, lastProcessedUpdate));
        rolldown.updateL1FromL2(merkleRoot, range);
    }

    function test_RevertIf_UpdateAlreadyApplied() public {
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
    Withdrawal[] public withdrawals;
    bytes32[] public withdrawalHashes;
    Merkle public merkle = new Merkle();
    Range public range = Range(1, 6);

    function setUp() public override {
        super.setUp();

        deal(address(rolldown), amount);
        token.mint(users.withdrawerB);

        for (uint256 i = 0; i < 4; ++i) {
            withdrawals.push(Withdrawal(createRequestId(Origin.L2), users.recipient, nativeTokenAddress, amount, 0));
            withdrawalHashes.push(rolldown.hashWithdrawal(withdrawals[0]));

            withdrawals.push(Withdrawal(createRequestId(Origin.L2), users.recipient, address(token), amount, 0));
            withdrawalHashes.push(rolldown.hashWithdrawal(withdrawals[1]));
        }

        vm.prank(users.withdrawerA);
        rolldown.ferryWithdrawal{value: amount}(withdrawals[0]);

        vm.startPrank(users.withdrawerB);

        token.approve(address(rolldown), amount);
        rolldown.ferryWithdrawal(withdrawals[1]);

        vm.stopPrank();

        bytes32 merkleRoot = merkle.getRoot(withdrawalHashes);

        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, range);
    }

    function test_EmitNativeTokensWithdrawnAndWithdrawalClosed() public {
        bytes32 merkleRoot = merkle.getRoot(withdrawalHashes);
        bytes32[] memory merkleProof = merkle.getProof(withdrawalHashes, 0);

        vm.prank(users.withdrawerA);
        vm.expectEmit();
        emit NativeTokensWithdrawn(users.withdrawerA, amount);
        emit WithdrawalClosed(2, withdrawalHashes[0]);
        rolldown.closeWithdrawal(withdrawals[0], merkleRoot, merkleProof);
    }

    function test_EmitNativeTokensWithdrawnAndWithdrawalClosed_BackwardCompatibility() public {
        bytes32 merkleRoot = merkle.getRoot(withdrawalHashes);
        bytes32[] memory merkleProof = merkle.getProof(withdrawalHashes, 0);

        vm.prank(users.withdrawerA);
        vm.expectEmit();
        emit NativeTokensWithdrawn(users.withdrawerA, amount);
        emit WithdrawalClosed(2, withdrawalHashes[0]);
        rolldown.close_withdrawal(withdrawals[0], merkleRoot, merkleProof);
    }

    function test_GetProcessedL2Request() public {
        bytes32 merkleRoot = merkle.getRoot(withdrawalHashes);
        bytes32[] memory merkleProof = merkle.getProof(withdrawalHashes, 0);

        vm.prank(users.withdrawerA);
        rolldown.closeWithdrawal(withdrawals[0], merkleRoot, merkleProof);

        assertEq(rolldown.processedL2Requests(withdrawalHashes[0]), closed);
        assertNotEq(rolldown.processedL2Requests(withdrawalHashes[1]), closed);
    }

    function test_RevertIf_Paused() public {
        vm.prank(users.admin);
        rolldown.pause();

        bytes32 merkleRoot = merkle.getRoot(withdrawalHashes);
        bytes32[] memory merkleProof = merkle.getProof(withdrawalHashes, 0);

        vm.prank(users.withdrawerA);
        vm.expectRevert("Pausable: paused");
        rolldown.closeWithdrawal(withdrawals[1], merkleRoot, merkleProof);
    }

    function test_RevertIf_L2RequestAlreadyProcessed() public {
        bytes32 merkleRoot = merkle.getRoot(withdrawalHashes);
        bytes32[] memory merkleProof = merkle.getProof(withdrawalHashes, 0);

        vm.startPrank(users.withdrawerA);
        rolldown.closeWithdrawal(withdrawals[0], merkleRoot, merkleProof);

        vm.expectRevert(abi.encodeWithSelector(L2RequestAlreadyProcessed.selector, withdrawalHashes[0]));
        rolldown.closeWithdrawal(withdrawals[0], merkleRoot, merkleProof);
        vm.stopPrank();
    }

    function test_RevertIf_UnexpectedMerkleRoot() public {
        bytes32[] memory withdrawalHashes = new bytes32[](2);
        withdrawalHashes[0] = withdrawalHashes[1];
        withdrawalHashes[1] = withdrawalHashes[0];

        bytes32 merkleRoot = merkle.getRoot(withdrawalHashes);
        bytes32[] memory merkleProof = merkle.getProof(withdrawalHashes, 0);

        vm.prank(users.withdrawerA);
        vm.expectRevert(UnexpectedMerkleRoot.selector);
        rolldown.closeWithdrawal(withdrawals[0], merkleRoot, merkleProof);
    }

    function test_RevertIf_RequestOutOfRange() public {
        token.mint(users.withdrawerB);

        Withdrawal memory withdrawalC =
            Withdrawal(RequestId(Origin.L2, 3), users.recipient, nativeTokenAddress, amount, 0);
        bytes32 withdrawalHashC = rolldown.hashWithdrawal(withdrawalC);
        Withdrawal memory withdrawalD = Withdrawal(RequestId(Origin.L2, 4), users.recipient, address(token), amount, 0);

        vm.prank(users.withdrawerA);
        rolldown.ferryWithdrawal{value: amount}(withdrawalC);

        vm.startPrank(users.withdrawerB);

        token.approve(address(rolldown), amount);
        rolldown.ferryWithdrawal(withdrawalD);

        vm.stopPrank();

        bytes32[] memory withdrawalHashes = new bytes32[](2);
        withdrawalHashes[0] = withdrawalHashC;
        withdrawalHashes[1] = withdrawalHashC;

        bytes32 merkleRoot = merkle.getRoot(withdrawalHashes);
        bytes32[] memory merkleProof = merkle.getProof(withdrawalHashes, 0);

        range = Range(7, 7);
        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, range);

        vm.prank(users.withdrawerB);
        vm.expectRevert(abi.encodeWithSelector(RequestOutOfRange.selector, 4, range.start, range.end));
        rolldown.closeWithdrawal(withdrawalD, merkleRoot, merkleProof);
    }

    function test_RevertIf_RequestRangeTooLarge() public {
        token.mint(users.withdrawerB);

        Withdrawal memory withdrawalC =
            Withdrawal(RequestId(Origin.L2, 3), users.recipient, nativeTokenAddress, amount, 0);
        bytes32 withdrawalHashC = rolldown.hashWithdrawal(withdrawalC);
        Withdrawal memory withdrawalD = Withdrawal(RequestId(Origin.L2, 4), users.recipient, address(token), amount, 0);

        vm.prank(users.withdrawerA);
        rolldown.ferryWithdrawal{value: amount}(withdrawalC);

        vm.startPrank(users.withdrawerB);
        token.approve(address(rolldown), amount);
        rolldown.ferryWithdrawal(withdrawalD);
        vm.stopPrank();

        bytes32[] memory withdrawalHashes = new bytes32[](2);
        withdrawalHashes[0] = withdrawalHashC;
        withdrawalHashes[1] = withdrawalHashC;

        bytes32 merkleRoot = merkle.getRoot(withdrawalHashes);
        bytes32[] memory merkleProof = merkle.getProof(withdrawalHashes, 0);

        range = Range(3, type(uint64).max);
        vm.prank(users.updater);
        rolldown.updateL1FromL2(merkleRoot, range);

        vm.prank(users.withdrawerB);
        vm.expectRevert(abi.encodeWithSelector(RequestRangeTooLarge.selector, type(uint64).max - 2));
        rolldown.closeWithdrawal(withdrawalD, merkleRoot, merkleProof);
    }

    function test_RevertIf_InvalidRequestProof() public {
        bytes32 merkleRoot = merkle.getRoot(withdrawalHashes);
        bytes32[] memory merkleProof = merkle.getProof(withdrawalHashes, 1);

        vm.prank(users.withdrawerA);
        vm.expectRevert(abi.encodeWithSelector(InvalidRequestProof.selector, merkleRoot));
        rolldown.closeWithdrawal(withdrawals[0], merkleRoot, merkleProof);
    }
}

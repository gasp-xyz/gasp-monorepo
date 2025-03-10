// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {ERC20, IERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {Test} from "forge-std/Test.sol";
import {IGaspToken} from "../src/interfaces/IGaspToken.sol";
import {GaspToken} from "../src/GaspToken.sol";
import {Rolldown} from "../src/Rolldown.sol";

contract USDTTokenMock is ERC20 {
    constructor(uint256 initialSupply) ERC20("USDT", "USDT") {
        _mint(msg.sender, initialSupply);
    }
}

contract UniswapPoolMock {
    IERC20 internal token0;
    IERC20 internal token1;

    constructor(address token0_, address token1_) {
        token0 = IERC20(token0_);
        token1 = IERC20(token1_);
    }

    function addLiquidity(uint256 amount) public {
        IERC20(token0).transferFrom(msg.sender, address(this), amount);
        IERC20(token1).transferFrom(msg.sender, address(this), amount);
    }

    function removeLiquidity(uint256 amount) public {
        IERC20(token0).transfer(msg.sender, amount);
        IERC20(token1).transfer(msg.sender, amount);
    }

    function swapToken0ToToken1(uint256 amount) public {
        IERC20(token0).transferFrom(msg.sender, address(this), amount);
        IERC20(token1).transfer(msg.sender, amount);
    }

    function swapToken1ToToken0(uint256 amount) public {
        IERC20(token1).transferFrom(msg.sender, address(this), amount);
        IERC20(token0).transfer(msg.sender, amount);
    }
}

contract RolldownMock {
    using SafeERC20 for IERC20;

    IERC20 internal token;

    constructor(address token_) {
        token = IERC20(token_);
    }

    function deposit(uint256 amount) public {
        IERC20(token).safeTransferFrom(msg.sender, address(this), amount);
    }

    function withdraw(uint256 amount) public {
        IERC20(token).transfer(msg.sender, amount);
    }
}

contract GaspTokenTest is Test {
    struct Accounts {
        address deployer;
        address l1Council;
        address swapper;
        address depositor;
        address sender;
        address recipient;
    }

    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event AllowTransfersSet(bool allowTransfers_);
    event AddedToSenderWhitelist(address indexed account);
    event AddedToRecipientWhitelist(address indexed account);
    event RemovedFromSenderWhitelist(address indexed account);
    event RemovedFromRecipientWhitelist(address indexed account);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    event Transfer(address indexed from, address indexed to, uint256 value);

    error ZeroL1Council();
    error ZeroRolldown();
    error ZeroWhitelistedAccount();
    error TransfersAlreadyAllowed();
    error AccountAlreadyWhitelisted(address account);
    error AccountNotWhitelisted(address account);
    error OperationForbidden(bytes32 selector);

    uint8 internal constant DECIMALS = 18;
    uint256 internal constant TOTAL_SUPPLY = 1_000_000_000 * 10 ** DECIMALS;
    string internal constant NAME = "GASP";
    string internal constant SYMBOL = "GASP";

    Accounts internal accounts;
    USDTTokenMock internal usdtToken;
    UniswapPoolMock internal uniswapPool;
    RolldownMock internal rolldown;
    GaspToken internal gaspToken;
    uint256 internal amount = 1 * 10 ** DECIMALS;

    function setUp() public virtual {
        accounts = Accounts({
            deployer: makeAddr("deployer"),
            l1Council: makeAddr("l1Council"),
            swapper: makeAddr("swapper"),
            depositor: makeAddr("depositor"),
            sender: makeAddr("sender"),
            recipient: makeAddr("recipient")
        });

        vm.startPrank(accounts.deployer);

        usdtToken = new USDTTokenMock(TOTAL_SUPPLY);
        gaspToken = new GaspToken(accounts.l1Council);
        uniswapPool = new UniswapPoolMock(address(usdtToken), address(gaspToken));
        rolldown = new RolldownMock(address(gaspToken));

        vm.label(address(this), "GaspTokenTest");
        vm.label(address(usdtToken), "USDTToken");
        vm.label(address(gaspToken), "GaspToken");
        vm.label(address(uniswapPool), "UniswapPool");
        vm.label(address(rolldown), "Rolldown");

        usdtToken.transfer(accounts.l1Council, TOTAL_SUPPLY / 2);

        vm.stopPrank();
        vm.startPrank(accounts.l1Council);

        gaspToken.addToSenderWhitelist(address(uniswapPool));
        gaspToken.addToSenderWhitelist(address(rolldown));
        gaspToken.addToRecipientWhitelist(address(rolldown));

        vm.stopPrank();
    }
}

contract Deploy is GaspTokenTest {
    function test_GetOwner() public view {
        assertEq(gaspToken.owner(), accounts.l1Council);
    }

    function test_GetName() public view {
        assertEq(gaspToken.name(), NAME);
    }

    function test_GetSymbol() public view {
        assertEq(gaspToken.symbol(), SYMBOL);
    }

    function test_GetDecimals() public view {
        assertEq(gaspToken.decimals(), DECIMALS);
    }

    function test_GetTotalSupply() public view {
        assertEq(gaspToken.totalSupply(), TOTAL_SUPPLY);
    }

    function test_GetAllowTransfers() public view {
        assertFalse(gaspToken.allowTransfers());
    }

    function test_GetSenderWhitelist() public view {
        assertTrue(gaspToken.senderWhitelist(accounts.l1Council));
    }

    function test_GetRecipientWhitelist() public view {
        assertTrue(gaspToken.recipientWhitelist(accounts.l1Council));
    }

    function test_GetL1CouncilBalance() public view {
        assertEq(gaspToken.balanceOf(accounts.l1Council), TOTAL_SUPPLY);
    }

    function test_RevertIf_ZeroL1Council() public {
        vm.prank(accounts.deployer);
        vm.expectRevert(ZeroL1Council.selector);
        gaspToken = new GaspToken(address(0));
    }
}

contract TransferOwnership is GaspTokenTest {
    function test_EmitOwnershipTransferred() public {
        vm.prank(accounts.l1Council);
        vm.expectEmit();
        emit OwnershipTransferred(accounts.l1Council, accounts.deployer);
        gaspToken.transferOwnership(accounts.deployer);
    }

    function test_GetOwner() public {
        vm.prank(accounts.l1Council);
        gaspToken.transferOwnership(accounts.deployer);
        assertEq(gaspToken.owner(), accounts.deployer);
    }

    function test_RevertIf_NotOwner() public {
        vm.prank(accounts.deployer);
        vm.expectRevert("Ownable: caller is not the owner");
        gaspToken.transferOwnership(accounts.deployer);
    }
}

contract SetAllowTransfers is GaspTokenTest {
    function test_EmitAllowTransfersSet_IfSetAllowed() public {
        bool allowTransfers = true;

        vm.prank(accounts.l1Council);
        vm.expectEmit();
        emit AllowTransfersSet(allowTransfers);
        gaspToken.setAllowTransfers(allowTransfers);
    }

    function test_EmitAllowTransfersSet_IfSetNotAllowed() public {
        bool allowTransfers = false;

        vm.prank(accounts.l1Council);
        vm.expectEmit();
        emit AllowTransfersSet(allowTransfers);
        gaspToken.setAllowTransfers(allowTransfers);
    }

    function test_GetAllowTransfers() public {
        vm.prank(accounts.l1Council);
        gaspToken.setAllowTransfers(true);
        assertTrue(gaspToken.allowTransfers());
    }

    function test_RevertIf_TransfersAlreadyAllowed() public {
        vm.startPrank(accounts.l1Council);

        gaspToken.setAllowTransfers(true);

        vm.expectRevert(TransfersAlreadyAllowed.selector);
        gaspToken.setAllowTransfers(false);

        vm.stopPrank();
    }

    function test_RevertIf_NotOwner() public {
        vm.prank(accounts.sender);
        vm.expectRevert("Ownable: caller is not the owner");
        gaspToken.setAllowTransfers(false);
    }
}

contract AddToSenderWhitelist is GaspTokenTest {
    function test_EmitAddedToSenderWhitelist() public {
        vm.prank(accounts.l1Council);
        vm.expectEmit();
        emit AddedToSenderWhitelist(accounts.sender);
        gaspToken.addToSenderWhitelist(accounts.sender);
    }

    function test_GetSenderWhitelist() public {
        vm.prank(accounts.l1Council);
        gaspToken.addToSenderWhitelist(accounts.sender);
        assertTrue(gaspToken.senderWhitelist(accounts.sender));
    }

    function test_RevertIf_NotOwner() public {
        vm.prank(accounts.sender);
        vm.expectRevert("Ownable: caller is not the owner");
        gaspToken.addToSenderWhitelist(accounts.sender);
    }

    function test_RevertIf_ZeroWhitelistedAccount() public {
        vm.prank(accounts.l1Council);
        vm.expectRevert(ZeroWhitelistedAccount.selector);
        gaspToken.addToSenderWhitelist(address(0));
    }

    function test_RevertIf_AccountAlreadyWhitelisted() public {
        vm.startPrank(accounts.l1Council);

        gaspToken.addToSenderWhitelist(accounts.sender);

        vm.expectRevert(abi.encodeWithSelector(AccountAlreadyWhitelisted.selector, accounts.sender));
        gaspToken.addToSenderWhitelist(accounts.sender);

        vm.stopPrank();
    }
}

contract AddToRecipientWhitelist is GaspTokenTest {
    function test_EmitAddedToRecipientWhitelist() public {
        vm.prank(accounts.l1Council);
        vm.expectEmit();
        emit AddedToRecipientWhitelist(accounts.sender);
        gaspToken.addToRecipientWhitelist(accounts.sender);
    }

    function test_GetRecipientWhitelist() public {
        vm.prank(accounts.l1Council);
        gaspToken.addToRecipientWhitelist(accounts.sender);
        assertTrue(gaspToken.recipientWhitelist(accounts.sender));
    }

    function test_RevertIf_NotOwner() public {
        vm.prank(accounts.sender);
        vm.expectRevert("Ownable: caller is not the owner");
        gaspToken.addToRecipientWhitelist(accounts.sender);
    }

    function test_RevertIf_ZeroWhitelistedAccount() public {
        vm.prank(accounts.l1Council);
        vm.expectRevert(ZeroWhitelistedAccount.selector);
        gaspToken.addToRecipientWhitelist(address(0));
    }

    function test_RevertIf_AccountAlreadyWhitelisted() public {
        vm.startPrank(accounts.l1Council);

        gaspToken.addToRecipientWhitelist(accounts.sender);

        vm.expectRevert(abi.encodeWithSelector(AccountAlreadyWhitelisted.selector, accounts.sender));
        gaspToken.addToRecipientWhitelist(accounts.sender);

        vm.stopPrank();
    }
}

contract RemoveFromSenderWhitelist is GaspTokenTest {
    function test_EmitRemovedFromSenderWhitelist() public {
        vm.prank(accounts.l1Council);
        vm.expectEmit();
        emit RemovedFromSenderWhitelist(accounts.l1Council);
        gaspToken.removeFromSenderWhitelist(accounts.l1Council);
    }

    function test_GetSenderWhitelist() public {
        vm.startPrank(accounts.l1Council);

        gaspToken.addToSenderWhitelist(accounts.sender);
        gaspToken.removeFromSenderWhitelist(accounts.sender);

        vm.stopPrank();

        assertFalse(gaspToken.senderWhitelist(accounts.sender));
    }

    function test_RevertIf_NotOwner() public {
        vm.prank(accounts.sender);
        vm.expectRevert("Ownable: caller is not the owner");
        gaspToken.removeFromSenderWhitelist(accounts.sender);
    }

    function test_RevertIf_ZeroWhitelistedAccount() public {
        vm.prank(accounts.l1Council);
        vm.expectRevert(ZeroWhitelistedAccount.selector);
        gaspToken.removeFromSenderWhitelist(address(0));
    }

    function test_RevertIf_AccountNotWhitelisted() public {
        vm.prank(accounts.l1Council);
        vm.expectRevert(abi.encodeWithSelector(AccountNotWhitelisted.selector, accounts.sender));
        gaspToken.removeFromSenderWhitelist(accounts.sender);
    }
}

contract RemoveFromRecipientWhitelist is GaspTokenTest {
    function test_EmitRemovedFromRecipientWhitelist() public {
        vm.prank(accounts.l1Council);
        vm.expectEmit();
        emit RemovedFromRecipientWhitelist(accounts.l1Council);
        gaspToken.removeFromRecipientWhitelist(accounts.l1Council);
    }

    function test_GetRecipientWhitelist() public {
        vm.startPrank(accounts.l1Council);

        gaspToken.addToRecipientWhitelist(accounts.sender);
        gaspToken.removeFromRecipientWhitelist(accounts.sender);

        vm.stopPrank();

        assertFalse(gaspToken.recipientWhitelist(accounts.sender));
    }

    function test_RevertIf_NotOwner() public {
        vm.prank(accounts.sender);
        vm.expectRevert("Ownable: caller is not the owner");
        gaspToken.removeFromRecipientWhitelist(accounts.sender);
    }

    function test_RevertIf_ZeroWhitelistedAccount() public {
        vm.prank(accounts.l1Council);
        vm.expectRevert(ZeroWhitelistedAccount.selector);
        gaspToken.removeFromRecipientWhitelist(address(0));
    }

    function test_RevertIf_AccountNotWhitelisted() public {
        vm.prank(accounts.l1Council);
        vm.expectRevert(abi.encodeWithSelector(AccountNotWhitelisted.selector, accounts.sender));
        gaspToken.removeFromRecipientWhitelist(accounts.sender);
    }
}

contract TransferToken is GaspTokenTest {
    function test_EmitTransfer_IfSenderWhitelisted() public {
        vm.startPrank(accounts.l1Council);

        gaspToken.transfer(accounts.sender, amount);
        gaspToken.addToSenderWhitelist(accounts.sender);

        vm.stopPrank();

        vm.prank(accounts.sender);
        vm.expectEmit();
        emit Transfer(accounts.sender, accounts.recipient, amount);
        gaspToken.transfer(accounts.recipient, amount);
    }

    function test_EmitTransfer_IfRecipientWhitelisted() public {
        vm.startPrank(accounts.l1Council);

        gaspToken.transfer(accounts.sender, amount);
        gaspToken.addToRecipientWhitelist(accounts.recipient);

        vm.stopPrank();

        vm.prank(accounts.sender);
        vm.expectEmit();
        emit Transfer(accounts.sender, accounts.recipient, amount);
        gaspToken.transfer(accounts.recipient, amount);
    }

    function test_EmitTransfer_IfTransfersAllowed() public {
        vm.startPrank(accounts.l1Council);

        gaspToken.setAllowTransfers(true);
        gaspToken.transfer(accounts.sender, amount);

        vm.stopPrank();

        vm.prank(accounts.sender);
        vm.expectEmit();
        emit Transfer(accounts.sender, accounts.recipient, amount);
        gaspToken.transfer(accounts.recipient, amount);
    }

    function test_ChangeBalances() public {
        uint256 initialL1CouncilBalance = gaspToken.balanceOf(accounts.l1Council);
        uint256 initialRecipientBalance = gaspToken.balanceOf(accounts.recipient);

        vm.prank(accounts.l1Council);
        gaspToken.transfer(accounts.recipient, amount);

        assertEq(gaspToken.balanceOf(accounts.l1Council), initialL1CouncilBalance - amount);
        assertEq(gaspToken.balanceOf(accounts.recipient), initialRecipientBalance + amount);
    }

    function test_RevertIf_OperationForbiddenWhenSenderAndRecipientNotWhitelisted() public {
        vm.prank(accounts.sender);
        vm.expectRevert(abi.encodeWithSelector(OperationForbidden.selector, IERC20.transfer.selector));
        gaspToken.transfer(accounts.recipient, amount);
    }

    function test_RevertIf_TransferFromZeroAddress() public {
        vm.prank(accounts.l1Council);
        gaspToken.setAllowTransfers(true);

        vm.prank(address(0));
        vm.expectRevert("ERC20: transfer from the zero address");
        gaspToken.transfer(accounts.recipient, amount);
    }

    function test_RevertIf_TransferToZeroAddress() public {
        vm.startPrank(accounts.l1Council);

        gaspToken.setAllowTransfers(true);

        vm.expectRevert("ERC20: transfer to the zero address");
        gaspToken.transfer(address(0), amount);

        vm.stopPrank();
    }

    function test_RevertIf_InsufficientBalance() public {
        vm.startPrank(accounts.l1Council);

        gaspToken.transfer(accounts.sender, amount);
        gaspToken.addToSenderWhitelist(accounts.sender);

        vm.stopPrank();

        vm.prank(accounts.sender);
        vm.expectRevert("ERC20: transfer amount exceeds balance");
        gaspToken.transfer(accounts.recipient, amount + 1);
    }
}

contract TransferTokenFrom is GaspTokenTest {
    function test_EmitTransfer_IfOwnerWhitelisted() public {
        vm.startPrank(accounts.l1Council);

        gaspToken.transfer(accounts.swapper, amount);
        gaspToken.addToSenderWhitelist(accounts.swapper);

        vm.stopPrank();

        vm.prank(accounts.swapper);
        gaspToken.approve(accounts.sender, amount);

        vm.prank(accounts.sender);
        vm.expectEmit();
        emit Transfer(accounts.swapper, accounts.recipient, amount);
        gaspToken.transferFrom(accounts.swapper, accounts.recipient, amount);
    }

    function test_EmitTransfer_IfRecipientWhitelisted() public {
        vm.startPrank(accounts.l1Council);

        gaspToken.transfer(accounts.swapper, amount);
        gaspToken.addToRecipientWhitelist(accounts.recipient);

        vm.stopPrank();

        vm.prank(accounts.swapper);
        gaspToken.approve(accounts.sender, amount);

        vm.prank(accounts.sender);
        vm.expectEmit();
        emit Transfer(accounts.swapper, accounts.recipient, amount);
        gaspToken.transferFrom(accounts.swapper, accounts.recipient, amount);
    }

    function test_EmitTransfer_IfTransfersAllowed() public {
        vm.startPrank(accounts.l1Council);

        gaspToken.transfer(accounts.swapper, amount);
        gaspToken.setAllowTransfers(true);

        vm.stopPrank();

        vm.prank(accounts.swapper);
        gaspToken.approve(accounts.sender, amount);

        vm.prank(accounts.sender);
        vm.expectEmit();
        emit Transfer(accounts.swapper, accounts.recipient, amount);
        gaspToken.transferFrom(accounts.swapper, accounts.recipient, amount);
    }

    function test_ChangeBalances() public {
        vm.startPrank(accounts.l1Council);

        gaspToken.addToSenderWhitelist(accounts.sender);
        gaspToken.approve(accounts.sender, amount);

        vm.stopPrank();

        uint256 initialL1CouncilBalance = gaspToken.balanceOf(accounts.l1Council);
        uint256 initialSenderBalance = gaspToken.balanceOf(accounts.sender);
        uint256 initialRecipientBalance = gaspToken.balanceOf(accounts.recipient);

        vm.prank(accounts.sender);
        gaspToken.transferFrom(accounts.l1Council, accounts.recipient, amount);

        assertEq(gaspToken.balanceOf(accounts.l1Council), initialL1CouncilBalance - amount);
        assertEq(gaspToken.balanceOf(accounts.sender), initialSenderBalance);
        assertEq(gaspToken.balanceOf(accounts.recipient), initialRecipientBalance + amount);
    }

    function test_GetAllowance() public {
        vm.startPrank(accounts.l1Council);

        gaspToken.addToSenderWhitelist(accounts.sender);
        gaspToken.approve(accounts.sender, amount);

        vm.stopPrank();

        assertEq(gaspToken.allowance(accounts.l1Council, accounts.sender), amount);

        vm.prank(accounts.sender);
        gaspToken.transferFrom(accounts.l1Council, accounts.recipient, amount);

        assertEq(gaspToken.allowance(accounts.l1Council, accounts.sender), 0);
    }

    function test_RevertIf_OperationForbiddenWhenOwnerOrRecipientNotWhitelisted() public {
        vm.prank(accounts.l1Council);
        gaspToken.transfer(accounts.swapper, amount);

        vm.prank(accounts.swapper);
        gaspToken.approve(accounts.sender, amount);

        vm.prank(accounts.sender);
        vm.expectRevert(abi.encodeWithSelector(OperationForbidden.selector, IERC20.transferFrom.selector));
        gaspToken.transferFrom(accounts.swapper, accounts.recipient, amount);
    }

    function test_RevertIf_TransferToZeroAddress() public {
        vm.prank(accounts.l1Council);
        gaspToken.approve(accounts.sender, amount);

        vm.prank(accounts.sender);
        vm.expectRevert("ERC20: transfer to the zero address");
        gaspToken.transferFrom(accounts.l1Council, address(0), amount);
    }
}

contract ApproveToken is GaspTokenTest {
    function test_EmitApproval() public {
        vm.prank(accounts.sender);
        vm.expectEmit();
        emit Approval(accounts.sender, accounts.recipient, amount);
        gaspToken.approve(accounts.recipient, amount);
    }

    function test_GetAllowance() public {
        vm.prank(accounts.l1Council);
        gaspToken.approve(accounts.sender, amount);
        assertEq(gaspToken.allowance(accounts.l1Council, accounts.sender), amount);
    }

    function test_RevertIf_ApproveFromZeroAddress() public {
        vm.prank(address(0));
        vm.expectRevert("ERC20: approve from the zero address");
        gaspToken.approve(accounts.sender, amount);
    }

    function test_RevertIf_ApproveToZeroAddress() public {
        vm.prank(accounts.sender);
        vm.expectRevert("ERC20: approve to the zero address");
        gaspToken.approve(address(0), amount);
    }
}

contract IncreaseAllowance is GaspTokenTest {
    function test_EmitApproval() public {
        vm.prank(accounts.sender);
        vm.expectEmit();
        emit Approval(accounts.sender, accounts.recipient, amount);
        gaspToken.increaseAllowance(accounts.recipient, amount);
    }

    function test_GetAllowance() public {
        vm.prank(accounts.l1Council);
        gaspToken.increaseAllowance(accounts.sender, amount);
        assertEq(gaspToken.allowance(accounts.l1Council, accounts.sender), amount);
    }

    function test_RevertIf_ApproveFromZeroAddress() public {
        vm.prank(address(0));
        vm.expectRevert("ERC20: approve from the zero address");
        gaspToken.increaseAllowance(accounts.sender, amount);
    }

    function test_RevertIf_ApproveToZeroAddress() public {
        vm.prank(accounts.sender);
        vm.expectRevert("ERC20: approve to the zero address");
        gaspToken.increaseAllowance(address(0), amount);
    }
}

contract DecreaseAllowance is GaspTokenTest {
    function test_EmitApproval() public {
        vm.startPrank(accounts.sender);

        gaspToken.increaseAllowance(accounts.recipient, amount);

        vm.expectEmit();
        emit Approval(accounts.sender, accounts.recipient, 0);
        gaspToken.decreaseAllowance(accounts.recipient, amount);

        vm.stopPrank();
    }

    function test_GetAllowance() public {
        vm.startPrank(accounts.l1Council);

        gaspToken.increaseAllowance(accounts.sender, amount);
        gaspToken.decreaseAllowance(accounts.sender, amount);

        assertEq(gaspToken.allowance(accounts.l1Council, accounts.sender), 0);

        vm.stopPrank();
    }

    function test_RevertIf_DecreasedAllowanceBelowZero() public {
        vm.prank(accounts.sender);
        vm.expectRevert("ERC20: decreased allowance below zero");
        gaspToken.decreaseAllowance(accounts.sender, 1);
    }
}

contract UniswapPoolSwapUSDTToGASPTest is GaspTokenTest {
    function setUp() public override {
        super.setUp();

        vm.startPrank(accounts.l1Council);

        usdtToken.approve(address(uniswapPool), TOTAL_SUPPLY / 2);
        gaspToken.approve(address(uniswapPool), TOTAL_SUPPLY / 2);
        uniswapPool.addLiquidity(TOTAL_SUPPLY / 2);

        vm.stopPrank();

        vm.prank(accounts.deployer);
        usdtToken.transfer(accounts.swapper, amount);
    }

    function test_EmitTransfer_IfTransfersNotAllowed() public {
        vm.startPrank(accounts.swapper);

        usdtToken.approve(address(uniswapPool), amount);

        vm.expectEmit(true, true, true, true, address(usdtToken));
        emit Transfer(accounts.swapper, address(uniswapPool), amount);
        vm.expectEmit(true, true, true, true, address(gaspToken));
        emit Transfer(address(uniswapPool), accounts.swapper, amount);
        uniswapPool.swapToken0ToToken1(amount);

        vm.stopPrank();
    }

    function test_EmitTransfer_IfTransfersAllowed() public {
        vm.prank(accounts.l1Council);
        gaspToken.setAllowTransfers(true);

        vm.startPrank(accounts.swapper);

        usdtToken.approve(address(uniswapPool), amount);

        vm.expectEmit(true, true, true, true, address(usdtToken));
        emit Transfer(accounts.swapper, address(uniswapPool), amount);
        vm.expectEmit(true, true, true, true, address(gaspToken));
        emit Transfer(address(uniswapPool), accounts.swapper, amount);
        uniswapPool.swapToken0ToToken1(amount);

        vm.stopPrank();
    }

    function test_RevertIf_OperationForbiddenWhenWithUniswapPoolRemovedFromSenderWhitelist() public {
        vm.prank(accounts.l1Council);
        gaspToken.removeFromSenderWhitelist(address(uniswapPool));

        vm.startPrank(accounts.swapper);

        usdtToken.approve(address(uniswapPool), amount);

        vm.expectRevert(abi.encodeWithSelector(OperationForbidden.selector, IERC20.transfer.selector));
        uniswapPool.swapToken0ToToken1(amount);

        vm.stopPrank();
    }
}

contract UniswapPoolSwapGASPToUSDTTest is GaspTokenTest {
    function setUp() public override {
        super.setUp();

        vm.startPrank(accounts.l1Council);

        usdtToken.approve(address(uniswapPool), TOTAL_SUPPLY / 2);
        gaspToken.approve(address(uniswapPool), TOTAL_SUPPLY / 2);
        uniswapPool.addLiquidity(TOTAL_SUPPLY / 2);

        vm.stopPrank();

        vm.prank(accounts.deployer);
        usdtToken.transfer(accounts.swapper, amount);
    }

    function test_EmitTransfer_IfTransfersAllowed() public {
        vm.prank(accounts.l1Council);
        gaspToken.setAllowTransfers(true);

        vm.startPrank(accounts.swapper);

        usdtToken.approve(address(uniswapPool), amount);
        uniswapPool.swapToken0ToToken1(amount);

        gaspToken.approve(address(uniswapPool), amount);

        vm.expectEmit(true, true, true, true, address(gaspToken));
        emit Transfer(accounts.swapper, address(uniswapPool), amount);
        vm.expectEmit(true, true, true, true, address(usdtToken));
        emit Transfer(address(uniswapPool), accounts.swapper, amount);
        uniswapPool.swapToken1ToToken0(amount);

        vm.stopPrank();
    }

    function test_EmitTransfer_IfUniswapPoolAddedToRecipientWhitelist() public {
        vm.prank(accounts.l1Council);
        gaspToken.addToRecipientWhitelist(address(uniswapPool));

        vm.startPrank(accounts.swapper);

        usdtToken.approve(address(uniswapPool), amount);
        uniswapPool.swapToken0ToToken1(amount);

        gaspToken.approve(address(uniswapPool), amount);

        vm.expectEmit(true, true, true, true, address(gaspToken));
        emit Transfer(accounts.swapper, address(uniswapPool), amount);
        vm.expectEmit(true, true, true, true, address(usdtToken));
        emit Transfer(address(uniswapPool), accounts.swapper, amount);
        uniswapPool.swapToken1ToToken0(amount);

        vm.stopPrank();
    }

    function test_RevertIf_OperationForbiddenWhenTransfersNotAllowed() public {
        vm.startPrank(accounts.swapper);

        usdtToken.approve(address(uniswapPool), amount);
        uniswapPool.swapToken0ToToken1(amount);

        gaspToken.approve(address(uniswapPool), amount);
        vm.expectRevert(abi.encodeWithSelector(OperationForbidden.selector, IERC20.transferFrom.selector));
        uniswapPool.swapToken1ToToken0(amount);

        vm.stopPrank();
    }
}

contract RolldownDepositTest is GaspTokenTest {
    function setUp() public override {
        super.setUp();

        vm.prank(accounts.l1Council);
        gaspToken.transfer(accounts.depositor, amount);
    }

    function test_EmitTransfer_IfTransfersNotAllowed() public {
        vm.startPrank(accounts.depositor);

        gaspToken.approve(address(rolldown), amount);

        vm.expectEmit();
        emit Transfer(accounts.depositor, address(rolldown), amount);
        rolldown.deposit(amount);

        vm.stopPrank();
    }

    function test_EmitTransfer_IfTransfersAllowed() public {
        vm.prank(accounts.l1Council);
        gaspToken.setAllowTransfers(true);

        vm.startPrank(accounts.depositor);

        gaspToken.approve(address(rolldown), amount);

        vm.expectEmit();
        emit Transfer(accounts.depositor, address(rolldown), amount);
        rolldown.deposit(amount);

        vm.stopPrank();
    }

    function test_RevertIf_OperationForbiddenWhenRolldownRemovedFromRecipientWhitelist() public {
        vm.prank(accounts.l1Council);
        gaspToken.removeFromRecipientWhitelist(address(rolldown));

        vm.startPrank(accounts.depositor);

        gaspToken.approve(address(rolldown), amount);

        vm.expectRevert(abi.encodeWithSelector(OperationForbidden.selector, IERC20.transferFrom.selector));
        rolldown.deposit(amount);

        vm.stopPrank();
    }
}

contract RolldownWithdrawTest is GaspTokenTest {
    function setUp() public override {
        super.setUp();

        vm.prank(accounts.l1Council);
        gaspToken.transfer(accounts.depositor, amount);
    }

    function test_EmitTransfer_IfTransfersNotAllowed() public {
        vm.startPrank(accounts.depositor);

        gaspToken.approve(address(rolldown), amount);
        rolldown.deposit(amount);

        vm.expectEmit();
        emit Transfer(address(rolldown), accounts.depositor, amount);
        rolldown.withdraw(amount);

        vm.stopPrank();
    }

    function test_RevertIf_OperationForbiddenWhenRolldownRemovedFromSenderWhitelist() public {
        vm.startPrank(accounts.depositor);

        gaspToken.approve(address(rolldown), amount);
        rolldown.deposit(amount);

        vm.stopPrank();

        vm.prank(accounts.l1Council);
        gaspToken.removeFromSenderWhitelist(address(rolldown));

        vm.prank(accounts.depositor);
        vm.expectRevert(abi.encodeWithSelector(OperationForbidden.selector, IERC20.transfer.selector));
        rolldown.withdraw(amount);
    }
}

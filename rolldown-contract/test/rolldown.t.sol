pragma solidity ^0.8.0;
import {RollDown} from "../src/rolldown.sol";
import {stdStorage, StdStorage, Test} from "forge-std/Test.sol";


  contract RollDownTest is Test{
    RollDown public rollDown;

    function setUp() public {
      rollDown = new RollDown();
    }

    function beforeEach() public {
    }

    function testExecuteDeposit() public {
        // Arrange
        uint256 amount = 1000;
        address tokenAddress = 0xa7c534E6dF83C118A858Cb6fb4C1e8B92b03464b;

        // Act
        rollDown.deposit(tokenAddress, amount);
        RollDown.L1Update memory l1Update = rollDown.getUpdateForL2();

        // Assert
        assertEq(l1Update.order.length, 1);
        assert(l1Update.order[0] == RollDown.PendingRequestType.DEPOSIT);
        assertEq(l1Update.pendingDeposits.length, 1);
        //assertEq(l1Update.pendingDeposits[0].depositRecipient, msg.sender);
        assertEq(l1Update.pendingDeposits[0].tokenAddress, tokenAddress);
        assertEq(l1Update.pendingDeposits[0].amount, amount);
    }

    function testExecuteWithdrawal() public {
        // Arrange
        uint256 amount = 1000;
        address tokenAddress = 0xa7c534E6dF83C118A858Cb6fb4C1e8B92b03464b;

        // Act
        rollDown.withdraw(tokenAddress, amount);
        RollDown.L1Update memory l1Update = rollDown.getUpdateForL2();

        // Assert
        assertEq(l1Update.order.length, 1);
        assert(l1Update.order[0] == RollDown.PendingRequestType.WITHDRAWAL);
        assertEq(l1Update.pendingWithdraws.length, 1);
        //assertEq(l1Update.pendingDeposits[0].depositRecipient, msg.sender);
        assertEq(l1Update.pendingWithdraws[0].tokenAddress, tokenAddress);
        assertEq(l1Update.pendingWithdraws[0].amount, amount);
    }

    function testAcceptL2Update() public {
        // Arrange
        uint256 amount = 1000;
        address tokenAddress = 0xa7c534E6dF83C118A858Cb6fb4C1e8B92b03464b;
        rollDown.withdraw(tokenAddress, amount);

        RollDown.L1Update memory l1Update = rollDown.getUpdateForL2();
        assertEq(l1Update.order.length, 1);
        assert(l1Update.order[0] == RollDown.PendingRequestType.WITHDRAWAL);
        assertEq(l1Update.offset, 1);
        assertEq(l1Update.lastProccessedRequestOnL1, 0);
        assertEq(l1Update.lastAcceptedRequestOnL1, 1);

        RollDown.L2Update memory l2Update;
        l2Update.cancles = new RollDown.Cancel[](0);
        l2Update.results = new RollDown.RequestResult[](1);
        l2Update.results[0] = RollDown.RequestResult({
            requestId: 1,
            updateType: RollDown.UpdateType.WITHDRAWAL,
            status: false
        });

        // Act
        rollDown.update_l1_from_l2(l2Update);

        l1Update = rollDown.getUpdateForL2();
        assertEq(l1Update.order.length, 1);
        assert(l1Update.order[0] == RollDown.PendingRequestType.L2_UPDATES_TO_REMOVE);
        assertEq(l1Update.offset, 2);
        assertEq(l1Update.lastProccessedRequestOnL1, 1);
        assertEq(l1Update.lastAcceptedRequestOnL1, 2);

        RollDown.L2Update memory l2Update2;
        l2Update2.cancles = new RollDown.Cancel[](0);
        l2Update2.results = new RollDown.RequestResult[](1);
        l2Update2.results[0] = RollDown.RequestResult({
            requestId: 2,
            updateType: RollDown.UpdateType.INDEX_UPDATE,
            status: true
        });
        rollDown.update_l1_from_l2(l2Update2);

        l1Update = rollDown.getUpdateForL2();
        assertEq(l1Update.order.length, 1);
        assertEq(l1Update.offset, 3);
        assertEq(l1Update.lastProccessedRequestOnL1, 2);
        assertEq(l1Update.lastAcceptedRequestOnL1, 3);


        RollDown.L2Update memory l2Update3;
        l2Update3.cancles = new RollDown.Cancel[](0);
        l2Update3.results = new RollDown.RequestResult[](1);
        l2Update3.results[0] = RollDown.RequestResult({
            requestId: 3,
            updateType: RollDown.UpdateType.INDEX_UPDATE,
            status: true
        });
        rollDown.update_l1_from_l2(l2Update3);

        // Assert
        l1Update = rollDown.getUpdateForL2();
        assertEq(l1Update.order.length, 1);
        assertEq(l1Update.offset, 4);
        assertEq(l1Update.lastProccessedRequestOnL1, 3);
        assertEq(l1Update.lastAcceptedRequestOnL1, 4);
    }
}

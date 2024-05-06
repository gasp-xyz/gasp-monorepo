pragma solidity ^0.8.0;

// Import ERC-20 interface
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/utils/Address.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "forge-std/console.sol";

contract RollDown {
    //tbr
    address owner;

    // Counter for mapping key
    uint256 public counter;
    // Counter for last processed request to ensure not reading and processing already processed
    uint256 public lastProcessedUpdate_origin_l1;
    // Counter for last processed updates comming from l2 to ensure not reading and processing already processed
    uint256 public lastProcessedUpdate_origin_l2;

    address public constant ETH_TOKEN_ADDRESS =
        0x5748395867463837537395739375937493733457;

    event DepositAcceptedIntoQueue(
        uint256 requestId,
        address depositRecipient,
        address tokenAddress,
        uint256 amount
    );
    event DisputeResolutionAcceptedIntoQueue(
        uint256 requestId,
        bool cancelJustified
    );
    event WithdrawalResolutionAcceptedIntoQueue(
        uint256 requestId,
        bool success
    );
    event L2UpdatesToRemovedAcceptedIntoQueue(
        uint256 requestId,
        uint256[] l2UpdatesToRemove
    );
    event FundsWithdrawn(
        address withdrawRecipient,
        address tokenAddress,
        uint256 amount
    );
    event FundsReturned(
        address depositRecipient,
        address tokenAddress,
        uint256 amount
    );
    event cancelAndCalculatedHash(bytes32 cancelHash, bytes32 calculatedHash);
    event EthWithdrawPending(address sender, uint amount);
    event PendingEthWithdrawn(address sender, uint amount);

    enum Origin {
        L1,
        L2
    }

    struct RequestId {
        Origin origin;
        uint256 id;
    }

    struct Deposit {
        RequestId requestId;
        address depositRecipient;
        address tokenAddress;
        uint256 amount;
        uint256 timeStamp;
    }

    struct L2UpdatesToRemove {
        RequestId requestId;
        uint256[] l2UpdatesToRemove;
        uint256 timeStamp;
    }

    struct CancelResolution {
        RequestId requestId;
        uint256 l2RequestId;
        bool cancelJustified;
        uint256 timeStamp;
    }

    struct WithdrawalResolution {
        RequestId requestId;
        uint256 l2RequestId;
        bool status;
        uint256 timeStamp;
    }

    struct L1Update {
        Deposit[] pendingDeposits;
        CancelResolution[] pendingCancelResolutions;
        WithdrawalResolution[] pendingWithdrawalResolutions;
        L2UpdatesToRemove[] pendingL2UpdatesToRemove;
    }

    mapping(uint256 => WithdrawalResolution) public withdrawalResolutions;
    mapping(uint256 => CancelResolution) public cancelResolutions;
    mapping(uint256 => Deposit) private deposits;
    mapping(uint256 => L2UpdatesToRemove) private l2UpdatesToRemove;
    mapping(address => uint) public pendingEthWithdrawals;    

    //TODO: should be renamed to RequestType
    enum UpdateType {
        DEPOSIT,
        WITHDRAWAL,
        WITHDRAWAL_RESOLUTION,
        INDEX_UPDATE,
        CANCEL,
        CANCEL_RESOLUTION
    }

    struct RequestResult {
        RequestId requestId;
        uint256 originRequestId;
        UpdateType updateType;
        bool status;
    }

    struct L2Update {
        Cancel[] cancels;
        Withdrawal[] withdrawals;
        RequestResult[] results;
    }

    struct Range {
        uint256 start;
        uint256 end;
    }

    struct Cancel {
        RequestId requestId;
        Range range;
        bytes32 hash;
    }

    struct Withdrawal {
        RequestId requestId;
        address withdrawalRecipient;
        address tokenAddress;
        uint256 amount;
    }

    constructor() {
        lastProcessedUpdate_origin_l1 = 0;
        counter = 1;
        lastProcessedUpdate_origin_l2 = 0;
        owner = msg.sender;
    }

    function withdraw_pending_eth(uint256 amount) external {
        require(amount > 0, "Amount must be greater than zero");
        address payable sender = payable(msg.sender);
        require(pendingEthWithdrawals[sender] >= amount, "not enough pending withdraw amount");
        require(payable(address(this)).balance >= amount, "not enough eth funds");

        // important to set this here before .sendValue
        // to prevent reentrancy
        pendingEthWithdrawals[sender] -= amount;

        emit PendingEthWithdrawn(sender, amount);

        // send value uses call (gas unbounded)
        // and reverts upon failure
        Address.sendValue(sender, amount);
    }

    function deposit_eth() external payable {
        require(msg.value > 0, "msg value must be greater that 0");
        address depositRecipient = msg.sender;
        uint amount = msg.value;

        bytes32 blockHash = blockhash(block.number);
        Deposit memory depositRequest = Deposit({
            requestId: RequestId({origin: Origin.L1, id: counter++}),
            depositRecipient: depositRecipient,
            tokenAddress: ETH_TOKEN_ADDRESS,
            amount: amount,
            blockHash: blockHash
        });
        // Add the new request to the mapping
        deposits[depositRequest.requestId.id] = depositRequest;
        emit DepositAcceptedIntoQueue(
            depositRequest.requestId.id,
            depositRecipient,
            ETH_TOKEN_ADDRESS,
            amount
        );
    }

    function deposit(address tokenAddress, uint256 amount) public {
        deposit_erc20(tokenAddress, amount);
    }

    function deposit_erc20(address tokenAddress, uint256 amount) public {
        require(tokenAddress != address(0), "Invalid token address");
        require(amount > 0, "Amount must be greater than zero");
        address depositRecipient = msg.sender;

        IERC20 token = IERC20(tokenAddress);
        require(
            token.transferFrom(msg.sender, address(this), amount),
            "Token transfer failed"
        );

        uint256 timeStamp = block.timestamp;
        Deposit memory depositRequest = Deposit({
            requestId: RequestId({origin: Origin.L1, id: counter++}),
            depositRecipient: depositRecipient,
            tokenAddress: tokenAddress,
            amount: amount,
            timeStamp: timeStamp
        });
        // Add the new request to the mapping
        deposits[depositRequest.requestId.id] = depositRequest;
        emit DepositAcceptedIntoQueue(
            depositRequest.requestId.id,
            depositRecipient,
            tokenAddress,
            amount
        );
    }

    function getUpdateForL2() public view returns (L1Update memory) {
        return
            getPendingRequests(lastProcessedUpdate_origin_l1 + 1, counter - 1);
    }

    function min(uint256 a, uint256 b) private pure returns (uint256) {
        return a < b ? a : b;
    }

    function max(uint256 a, uint256 b) private pure returns (uint256) {
        return a > b ? a : b;
    }

    function getRequestsRange(
        L2Update calldata update
    ) private returns (uint256, uint256) {
        uint256 firstId;
        unchecked {
            firstId = uint256(0) - 1;
        }
        uint256 lastId = 0;

        if (update.cancels.length > 0) {
            firstId = min(update.cancels[0].requestId.id, firstId);
            lastId = max(
                update.cancels[update.cancels.length - 1].requestId.id,
                lastId
            );
        }

        if (update.withdrawals.length > 0) {
            firstId = min(update.withdrawals[0].requestId.id, firstId);
            lastId = max(
                update.withdrawals[update.withdrawals.length - 1].requestId.id,
                lastId
            );
        }

        if (update.results.length > 0) {
            firstId = min(update.results[0].requestId.id, firstId);
            lastId = max(
                update.results[update.results.length - 1].requestId.id,
                lastId
            );
        }

        return (firstId, lastId);
    }

    function getOrderOfRequestsOriginatingOnL2(
        uint256 firstId,
        L2Update calldata update
    ) private returns (UpdateType[] memory) {
        if (
            update.results.length == 0 &&
            update.cancels.length == 0 &&
            update.withdrawals.length == 0
        ) {
            return new UpdateType[](0);
        }

        uint256 withdrawalId = 0;
        uint256 cancelId = 0;
        uint256 resultId = 0;
        uint256 orderId = 0;
        uint256 updatesAmount = update.cancels.length +
            update.withdrawals.length +
            update.results.length;
        UpdateType[] memory order = new UpdateType[](updatesAmount);

        for (uint256 i = firstId; i < firstId + updatesAmount; i++) {
            if (
                withdrawalId < update.withdrawals.length &&
                update.withdrawals[withdrawalId].requestId.id == i
            ) {
                order[orderId] = UpdateType.WITHDRAWAL;
                withdrawalId++;
                orderId++;
            } else if (
                cancelId < update.cancels.length &&
                update.cancels[cancelId].requestId.id == i
            ) {
                order[orderId] = UpdateType.CANCEL;
                cancelId++;
                orderId++;
            } else if (
                resultId < update.results.length &&
                update.results[resultId].requestId.id == i
            ) {
                order[orderId] = UpdateType.INDEX_UPDATE;
                resultId++;
                orderId++;
            } else {
                console.log("requests not in order");
                revert("invalide L2Update");
            }
        }
        return order;
    }

    function processRequestsOriginatingOnL2(
        UpdateType[] memory order,
        L2Update calldata inputArray
    ) private {
        uint256 cancelId = 0;
        uint256 withdrawalId = 0;
        uint256 resultsId = 0;

        for (uint256 i = 0; i < order.length; i++) {
            if (order[i] == UpdateType.WITHDRAWAL) {
                Withdrawal calldata withdrawal = inputArray.withdrawals[
                    withdrawalId++
                ];
                if (withdrawal.requestId.id <= lastProcessedUpdate_origin_l2) {
                    continue;
                }
                process_l2_update_withdrawal(withdrawal);
                lastProcessedUpdate_origin_l2++;
            } else if (order[i] == UpdateType.CANCEL) {
                Cancel calldata cancel = inputArray.cancels[cancelId++];
                if (cancel.requestId.id <= lastProcessedUpdate_origin_l2) {
                    continue;
                }
                process_l2_update_cancels(cancel);
                lastProcessedUpdate_origin_l2++;
            } else if (order[i] == UpdateType.INDEX_UPDATE) {
                RequestResult calldata result = inputArray.results[resultsId++];
                if (result.requestId.id <= lastProcessedUpdate_origin_l2) {
                    continue;
                }
                lastProcessedUpdate_origin_l2++;
            } else {
                revert("unknown update type");
            }
        }
    }

    function update_l1_from_l2(L2Update calldata inputArray) external {
        //1st iteration, security comes from ensuring dedicated acc
        //Ensure sender is dedic acc
        // TODO: uncomment & fix UT
        // require(msg.sender == owner, "Not the owner");
        require(
            inputArray.results.length >= 1 ||
                inputArray.cancels.length >= 1 ||
                inputArray.withdrawals.length >= 1,
            "Array must have at least 1 update"
        );
        (uint256 firstId, uint256 lastId) = getRequestsRange(inputArray);
        require(firstId != 0, "Invalid L2Update");
        require(
            firstId <= lastProcessedUpdate_origin_l2 + 1,
            "Invalid L2Update"
        );
        require(lastId > lastProcessedUpdate_origin_l2, "Invalid L2Update");

        UpdateType[] memory order = getOrderOfRequestsOriginatingOnL2(
            firstId,
            inputArray
        );

        uint256[]
            memory l2UpdatesToBeRemoved = process_l2_update_requests_results(
                order,
                inputArray.results
            );

        processRequestsOriginatingOnL2(order, inputArray);

        // Create a new array with the correct size
        if (l2UpdatesToBeRemoved.length > 0) {
            uint256 rid = counter++;
            l2UpdatesToRemove[rid] = L2UpdatesToRemove({
                requestId: RequestId({origin: Origin.L1, id: rid}),
                l2UpdatesToRemove: l2UpdatesToBeRemoved,
                timeStamp: block.timestamp
            });
            lastProcessedUpdate_origin_l1 += l2UpdatesToBeRemoved.length;
            emit L2UpdatesToRemovedAcceptedIntoQueue(rid, l2UpdatesToBeRemoved);
        }
    }

    function process_l2_update_requests_results(
        UpdateType[] memory order,
        RequestResult[] calldata results
    ) private returns (uint256[] memory) {
        if (results.length == 0) {
            return new uint256[](0);
        }
        uint256 updatesToBeRemovedCounter = 0;
        uint256[] memory l2UpdatesToBeRemovedTemp = new uint256[](
            results.length
        );
        uint256 updatesCnt = 0;

        for (uint256 idx = 0; idx < order.length; idx++) {
            if (order[idx] == UpdateType.INDEX_UPDATE) {
                RequestResult memory element = results[updatesCnt++];
                if (element.requestId.id <= lastProcessedUpdate_origin_l2) {
                    continue;
                }
                if (
                    element.updateType == UpdateType.DEPOSIT ||
                    element.updateType == UpdateType.INDEX_UPDATE ||
                    element.updateType == UpdateType.CANCEL_RESOLUTION ||
                    element.updateType == UpdateType.WITHDRAWAL_RESOLUTION
                ) {
                    l2UpdatesToBeRemovedTemp[updatesToBeRemovedCounter++] = (
                        element.originRequestId
                    );
                    if (element.updateType == UpdateType.DEPOSIT){
                      process_l2_update_deposit(element);
                    }
                } else {
                    revert("unknown request type");
                }
            }
        }

        uint256[] memory l2UpdatesToBeRemoved = new uint256[](
            updatesToBeRemovedCounter
        );

        for (uint256 i = 0; i < updatesToBeRemovedCounter; i++) {
            l2UpdatesToBeRemoved[i] = l2UpdatesToBeRemovedTemp[i];
        }

        return l2UpdatesToBeRemoved;
    }

    function process_l2_update_cancels(Cancel calldata cancel) private {
        L1Update memory pending = getPendingRequests(
            cancel.range.start,
            cancel.range.end
        );
        bytes32 correct_hash = keccak256(abi.encode(pending));
        uint256 timeStamp = block.timestamp;

        CancelResolution memory resolution = CancelResolution({
            requestId: RequestId({origin: Origin.L1, id: counter++}),
            l2RequestId: cancel.requestId.id,
            cancelJustified: correct_hash == cancel.hash,
            timeStamp: timeStamp
        });

        cancelResolutions[resolution.requestId.id] = resolution;
        emit DisputeResolutionAcceptedIntoQueue(
            resolution.l2RequestId,
            resolution.cancelJustified
        );
    }

    function process_l2_update_withdrawal(
        Withdrawal calldata withdrawal
    ) private {
        if (withdrawal.tokenAddress == ETH_TOKEN_ADDRESS){
            process_eth_withdrawal(withdrawal);
        }
        else {
            process_erc20_withdrawal(withdrawal);
        }
    }

    function process_eth_withdrawal(
        Withdrawal calldata withdrawal
    ) private {
        bool status = payable(address(this)).balance >= withdrawal.amount;
        bytes32 blockHash = blockhash(block.number);

        WithdrawalResolution memory resolution = WithdrawalResolution({
            requestId: RequestId({origin: Origin.L1, id: counter++}),
            l2RequestId: withdrawal.requestId.id,
            status: status,
            blockHash: blockHash
        });

        withdrawalResolutions[resolution.requestId.id] = resolution;
        emit WithdrawalResolutionAcceptedIntoQueue(
            resolution.requestId.id,
            status
        );

        if (status) {
            pendingEthWithdrawals[withdrawal.withdrawalRecipient] += withdrawal.amount;
            emit EthWithdrawPending(
                withdrawal.withdrawalRecipient,
                pendingEthWithdrawals[withdrawal.withdrawalRecipient]
            );
        }
    }

    function process_erc20_withdrawal(
        Withdrawal calldata withdrawal
    ) private {
        IERC20 token = IERC20(withdrawal.tokenAddress);
        bool status = token.balanceOf(address(this)) >= withdrawal.amount;
        uint256 timeStamp = block.timestamp;

        WithdrawalResolution memory resolution = WithdrawalResolution({
            requestId: RequestId({origin: Origin.L1, id: counter++}),
            l2RequestId: withdrawal.requestId.id,
            status: status,
            timeStamp: timeStamp
        });

        withdrawalResolutions[resolution.requestId.id] = resolution;
        emit WithdrawalResolutionAcceptedIntoQueue(
            resolution.requestId.id,
            status
        );

        if (status) {
            token.transfer(withdrawal.withdrawalRecipient, withdrawal.amount);
            emit FundsWithdrawn(
                withdrawal.withdrawalRecipient,
                withdrawal.tokenAddress,
                withdrawal.amount
            );
        }
    }

    function process_l2_update_deposit(
        RequestResult memory depositResult
    ) private {
        if (!depositResult.status) {
            uint256 requestId = depositResult.requestId.id;
            Deposit memory theDeposit = deposits[requestId];
            IERC20 token = IERC20(theDeposit.tokenAddress);
            token.transfer(theDeposit.depositRecipient, theDeposit.amount);

            emit FundsReturned(
                theDeposit.depositRecipient,
                theDeposit.tokenAddress,
                theDeposit.amount
            );
        }
    }

    function getPendingRequests(
        uint256 start,
        uint256 end
    ) public view returns (L1Update memory) {
        L1Update memory result;

        uint256 depositsCounter = 0;
        uint256 withdrawalsCounter = 0;
        uint256 cancelsCounter = 0;
        uint256 updatesToBeRemovedCounter = 0;

        for (uint256 requestId = start; requestId <= end; requestId++) {
            if (deposits[requestId].requestId.id != 0) {
                depositsCounter++;
            } else if (l2UpdatesToRemove[requestId].requestId.id != 0) {
                updatesToBeRemovedCounter++;
            } else if (withdrawalResolutions[requestId].requestId.id != 0) {
                withdrawalsCounter++;
            } else if (cancelResolutions[requestId].requestId.id != 0) {
                cancelsCounter++;
            }
        }

        result.pendingDeposits = new Deposit[](depositsCounter);
        result.pendingCancelResolutions = new CancelResolution[](cancelsCounter);
        result.pendingWithdrawalResolutions = new WithdrawalResolution[](
            withdrawalsCounter
        );
        result.pendingL2UpdatesToRemove = new L2UpdatesToRemove[](
            updatesToBeRemovedCounter
        );

        withdrawalsCounter = 0;
        depositsCounter = 0;
        cancelsCounter = 0;
        updatesToBeRemovedCounter = 0;

        for (uint256 requestId = start; requestId <= end; requestId++) {
            if (deposits[requestId].requestId.id > 0) {
                result.pendingDeposits[depositsCounter++] = deposits[requestId];
            } else if (withdrawalResolutions[requestId].requestId.id > 0) {
                result.pendingWithdrawalResolutions[
                    withdrawalsCounter++
                ] = withdrawalResolutions[requestId];
            } else if (l2UpdatesToRemove[requestId].requestId.id > 0) {
                result.pendingL2UpdatesToRemove[
                    updatesToBeRemovedCounter++
                ] = l2UpdatesToRemove[requestId];
            } else if (cancelResolutions[requestId].l2RequestId > 0) {
                result.pendingCancelResolutions[
                    cancelsCounter++
                ] = cancelResolutions[requestId];
            } else {
                break;
            }
        }

        return result;
    }
}

// TODO: counter l2 check
// TODO: align types again

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// Import ERC-20 interface
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/utils/Strings.sol";

contract RollDown {
    //tbr
    address owner;

    // Counter for mapping key
    uint256 public counter;
    // Counter for last processed request to ensure not reading and processing already processed
    uint256 public lastProcessedUpdate_origin_l1;
    // Counter for last processed updates comming from l2 to ensure not reading and processing already processed
    uint256 public lastProcessedUpdate_origin_l2;

    event DepositAcceptedIntoQueue(
        uint256 requestId,
        address depositRecipient,
        address tokenAddress,
        uint256 amount
    );
    event DisputeResolutionAcceptedIntoQueue(
        uint256 requestId,
        uint256 originalRequestId,
        bool cancelJustified
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
    event cancelAndCalculatedHash(bytes32 cancelHash, bytes32 calculatedHash);

    // Pending requests storage and structs
    struct Deposit {
        address depositRecipient;
        address tokenAddress;
        uint256 amount;
        bytes32 blockHash;
    }

    /// PENING REQUESTS TYPES (L1)

    struct L2UpdatesToRemove {
        uint256[] l2UpdatesToRemove;
        bytes32 blockHash;
    }

    struct CancelResolution {
        uint256 l2RequestId;
        bool cancelJustified;
        bytes32 blockHash;
    }

    enum PendingRequestType {
        DEPOSIT,
        WITHDRAWAL,
        CANCEL_RESOLUTION,
        L2_UPDATES_TO_REMOVE
    }

    struct L1Update {
        uint256 lastProccessedRequestOnL1;
        uint256 lastAcceptedRequestOnL1;
        uint256 offset;
        PendingRequestType[] order;
        Deposit[] pendingDeposits;
        CancelResolution[] pendingCancelResultions;
        L2UpdatesToRemove[] pendingL2UpdatesToRemove;
    }

    mapping(uint256 => CancelResolution) public cancelResolutions;
    mapping(uint256 => Deposit) private deposits;
    mapping(uint256 => L2UpdatesToRemove) private l2UpdatesToRemove;
    /// PENING REQUESTS TYPES (L1)

    enum UpdateType {
        DEPOSIT,
        WITHDRAWAL,
        INDEX_UPDATE,
        CANCEL_RESOLUTION
    }

    struct L2Update {
        Withdraw[] withdraws;
        Cancel[] cancels;
        RequestResult[] results;
    }

    struct Cancel {
        uint256 requestId;
        bytes updater;
        bytes canceler;
        uint256 lastProccessedRequestOnL1;
        uint256 lastAcceptedRequestOnL1;
        bytes32 hash;
    }

    struct Withdraw {
        uint256 requestId;
        address withdrawRecipient;
        address tokenAddress;
        uint256 amount;
    }

    struct RequestResult {
        uint256 requestId;
        UpdateType updateType;
        bool status;
    }

    constructor() {
        lastProcessedUpdate_origin_l1 = 0;
        counter = 1;
        lastProcessedUpdate_origin_l2 = 170141183460469231731687303715884105727;
        owner = msg.sender;
    }

    function deposit(address tokenAddress, uint256 amount) public {
        require(tokenAddress != address(0), "Invalid token address");
        require(amount > 0, "Amount must be greater than zero");
        address depositRecipient = msg.sender;

        IERC20 token = IERC20(tokenAddress);
        // TODO: uncomment
        // require transfer tokens from the sender to the contract
        require(
            token.transferFrom(msg.sender, address(this), amount),
            "Token transfer failed"
        );

        bytes32 blockHash = blockhash(block.number);

        Deposit memory newDeposit = Deposit({
            depositRecipient: depositRecipient,
            tokenAddress: tokenAddress,
            amount: amount,
            blockHash: blockHash
        });
        // Add the new request to the mapping
        deposits[counter++] = newDeposit;
        emit DepositAcceptedIntoQueue(
            counter - 1,
            depositRecipient,
            tokenAddress,
            amount
        );
    }

    function getUpdateForL2() public view returns (L1Update memory) {
        return getPendingRequests(lastProcessedUpdate_origin_l1 + 1, counter);
    }

    function update_l1_from_l2(L2Update calldata inputArray) external {
        //1st iteration, security comes from ensuring dedicated acc
        //Ensure sender is dedic acc

        require(msg.sender == owner, "Not the owner");
        require(
            inputArray.results.length >= 1 ||
                inputArray.cancels.length >= 1 ||
                inputArray.withdraws.length >= 1,
            "Array must have at least 1 update"
        );

        uint256 updatesToBeRemovedCounter = 0;
        uint256[] memory l2UpdatesToBeRemovedTemp = new uint256[](
            inputArray.results.length +
                inputArray.withdraws.length +
                inputArray.cancels.length
        );

        if (inputArray.results.length >= 1) {
            uint256 orderCounter = inputArray.results[0].requestId;
            // check if no request is missing
            require(
                orderCounter <= lastProcessedUpdate_origin_l1 + 1,
                "Requests missing between updates"
            );
            // check order of requests
            for (uint256 idx = 1; idx < inputArray.results.length; idx++) {
                if (inputArray.results[idx].requestId != orderCounter + 1) {
                    revert("Requests are not in order");
                }
                orderCounter = inputArray.results[idx].requestId;
            }

            for (uint256 idx = 0; idx < inputArray.results.length; idx++) {
                RequestResult calldata element = inputArray.results[idx];

                if (element.requestId <= lastProcessedUpdate_origin_l1) {
                    continue;
                }

                if (element.updateType == UpdateType.DEPOSIT) {
                    l2UpdatesToBeRemovedTemp[updatesToBeRemovedCounter++] = (
                        element.requestId
                    );
                } else if (element.updateType == UpdateType.INDEX_UPDATE) {
                    l2UpdatesToBeRemovedTemp[updatesToBeRemovedCounter++] = (
                        element.requestId
                    );
                } else {
                    revert("unknown request type");
                }
            }
            lastProcessedUpdate_origin_l1 = orderCounter;
        }

        if (
            inputArray.cancels.length >= 1 || inputArray.withdraws.length >= 1
        ) {
            uint256 first_requestId = 0;
            if (inputArray.cancels.length >= 1) {
                first_requestId = inputArray.cancels[0].requestId;
                if (inputArray.withdraws.length >= 1) {
                    if (inputArray.withdraws[0].requestId < first_requestId) {
                        first_requestId = inputArray.withdraws[0].requestId;
                    }
                }
            } else {
                first_requestId = inputArray.withdraws[0].requestId;
            }

            // check if no request is missing between updates
            require(
                first_requestId <= lastProcessedUpdate_origin_l2 + 1,
                "Requests missing between updates"
            );

            uint256 cancelIdx = 0;
            uint256 withdrawIdx = 0;

            for (
                uint256 requestId = first_requestId;
                requestId <
                (inputArray.cancels.length +
                    inputArray.withdraws.length +
                    first_requestId);
                requestId++
            ) {
                if (
                    inputArray.cancels.length >= 1 &&
                    inputArray.cancels[cancelIdx].requestId == requestId
                ) {
                    process_pending_update_cancel(
                        inputArray.cancels[cancelIdx].requestId,
                        inputArray.cancels[cancelIdx].lastProccessedRequestOnL1,
                        inputArray.cancels[cancelIdx].lastAcceptedRequestOnL1,
                        uint256(inputArray.cancels[cancelIdx].hash)
                    );
                    cancelIdx++;
                } else if (
                    inputArray.withdraws.length >= 1 &&
                    inputArray.withdraws[withdrawIdx].requestId == requestId
                ) {
                    process_pending_update_withdraw(
                        inputArray.withdraws[withdrawIdx].withdrawRecipient,
                        inputArray.withdraws[withdrawIdx].tokenAddress,
                        inputArray.withdraws[withdrawIdx].amount
                    );
                    l2UpdatesToBeRemovedTemp[updatesToBeRemovedCounter++] = (
                        inputArray.withdraws[withdrawIdx].requestId
                    );
                    withdrawIdx++;
                } else {
                    revert("Requests are not in order");
                }
            }
            lastProcessedUpdate_origin_l2 =
                inputArray.cancels.length +
                inputArray.withdraws.length +
                first_requestId -
                1;
        }

        if (updatesToBeRemovedCounter > 0) {
            // Create a new array with the correct size
            uint256[] memory l2UpdatesToBeRemoved = new uint256[](
                updatesToBeRemovedCounter
            );

            // Copy values from temp array to final array
            for (uint256 i = 0; i < updatesToBeRemovedCounter; i++) {
                l2UpdatesToBeRemoved[i] = l2UpdatesToBeRemovedTemp[i];
            }

            l2UpdatesToRemove[counter++]
                .l2UpdatesToRemove = l2UpdatesToBeRemoved; // .push(l2UpdatesToBeRemoved[i]);

            bytes32 blockHash = blockhash(block.number);

            l2UpdatesToRemove[counter++].blockHash = blockHash;
            emit L2UpdatesToRemovedAcceptedIntoQueue(
                counter - 1,
                l2UpdatesToBeRemoved
            );
        }
    }

    // Process functions

    function process_pending_update_cancel(
        uint256 requestId,
        uint256 lastProccessedRequestOnL1,
        uint256 lastAcceptedRequestOnL1,
        uint256 cancelHashAsUint
    ) private {
        counter++;
        //create json string from requests on l1
        //string memory jsonString = getPendingRequestsJson(
        //    lastProccessedRequestOnL1,
        //    lastAcceptedRequestOnL1
        //);
        string memory jsonString = "hello";
        //create hash from requests on l1
        bytes32 jsonHash = keccak256(abi.encodePacked(jsonString));
        //compare with hash provided in cancel
        bool cancelJustified = (jsonHash != bytes32(cancelHashAsUint));

        bytes32 blockHash = blockhash(block.number);

        CancelResolution memory newCancelResolution = CancelResolution({
            l2RequestId: requestId,
            cancelJustified: cancelJustified,
            blockHash: blockHash
        });
        // Add the new request to the mapping
        cancelResolutions[counter] = newCancelResolution;

        //tbr
        emit cancelAndCalculatedHash(bytes32(cancelHashAsUint), jsonHash);

        emit DisputeResolutionAcceptedIntoQueue(
            counter,
            requestId,
            cancelJustified
        );
    }

    function process_pending_update_withdraw(
        address withdrawRecipient,
        address tokenAddress,
        uint256 amount
    ) private {
        IERC20 token = IERC20(tokenAddress);
        require(
            token.balanceOf(address(this)) >= amount,
            "Insufficient balance in the contract"
        );

        // Transfer tokens from the contract to the recipient
        token.transfer(withdrawRecipient, amount);

        emit FundsWithdrawn(withdrawRecipient, tokenAddress, amount);
    }

    function getPendingRequestsRemove(
        uint256 id
    ) private view returns (uint256[] memory) {
        return l2UpdatesToRemove[id].l2UpdatesToRemove;
    }

    function getPendingRequests(
        uint256 start,
        uint256 end
    ) private view returns (L1Update memory) {
        L1Update memory result;

        uint256 depositsCounter = 0;
        uint256 cancelsCounter = 0;
        uint256 updatesToBeRemovedCounter = 0;

        for (uint256 requestId = start; requestId <= end; requestId++) {
            if (deposits[requestId].depositRecipient != address(0)) {
                depositsCounter++;
            } else if (
                l2UpdatesToRemove[requestId].l2UpdatesToRemove.length > 0
            ) {
                updatesToBeRemovedCounter++;
            } else if (cancelResolutions[requestId].l2RequestId > 0) {
                cancelsCounter++;
            }
        }

        uint256 requestCounter = depositsCounter +
            cancelsCounter +
            updatesToBeRemovedCounter;
        result.order = new PendingRequestType[](requestCounter);

        result.offset = start;
        result.pendingDeposits = new Deposit[](depositsCounter);
        result.pendingCancelResultions = new CancelResolution[](cancelsCounter);
        result.pendingL2UpdatesToRemove = new L2UpdatesToRemove[](
            updatesToBeRemovedCounter
        );
        result.lastProccessedRequestOnL1 = lastProcessedUpdate_origin_l1;
        result.lastAcceptedRequestOnL1 = counter - 1;

        depositsCounter = 0;
        cancelsCounter = 0;
        updatesToBeRemovedCounter = 0;
        requestCounter = 0;

        for (uint256 requestId = start; requestId <= end; requestId++) {
            if (deposits[requestId].depositRecipient != address(0)) {
                result.order[requestCounter++] = PendingRequestType.DEPOSIT;
                result.pendingDeposits[depositsCounter++] = deposits[requestId];
            } else if (
                l2UpdatesToRemove[requestId].l2UpdatesToRemove.length > 0
            ) {
                result.order[requestCounter++] = PendingRequestType
                    .L2_UPDATES_TO_REMOVE;
                result.pendingL2UpdatesToRemove[
                    updatesToBeRemovedCounter++
                ] = l2UpdatesToRemove[requestId];
            } else if (cancelResolutions[requestId].l2RequestId > 0) {
                result.order[requestCounter++] = PendingRequestType
                    .CANCEL_RESOLUTION;
                result.pendingCancelResultions[
                    cancelsCounter++
                ] = cancelResolutions[requestId];
            } else {
                break;
            }
        }

        return result;
    }
}

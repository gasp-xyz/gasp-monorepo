// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// Import ERC-20 interface
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
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
    event L2UpdatesToRemovedAcceptedIntoQueue(uint256 requestId, uint256[] l2UpdatesToRemove);
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
    }

    /// PENING REQUESTS TYPES (L1)

    struct L2UpdatesToRemove {
        uint256[] l2UpdatesToRemove;
    }

    struct CancelResolution {
        uint256 l2RequestId;
        bool cancelJustified;
    }

    enum PendingRequestType{ DEPOSIT, WITHDRAWAL, CANCEL_RESOLUTION, L2_UPDATES_TO_REMOVE}

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

    enum UpdateType{ DEPOSIT, WITHDRAWAL, INDEX_UPDATE, CANCEL_RESOLUTION}

    struct RequestResult {
      uint256 requestId;
      UpdateType updateType;
      bool status;
    }

    struct L2Update {
      Cancel[] cancles;
      Withdraw[] withdraws;
      RequestResult[] results;
    }

    struct Cancel {
        uint256 l2RequestId;
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


    constructor() {
        lastProcessedUpdate_origin_l1 = 0;
        counter = 1;
        lastProcessedUpdate_origin_l2 = 0;
        owner = msg.sender;
    }

    function deposit(address tokenAddress, uint256 amount) public {
        require(tokenAddress != address(0), "Invalid token address");
        require(amount > 0, "Amount must be greater than zero");
        address depositRecipient = msg.sender;

        IERC20 token = IERC20(tokenAddress);
        require(
            token.transferFrom(msg.sender, tokenAddress, amount),
            "Token transfer failed"
        );

        Deposit memory newDeposit = Deposit({
            depositRecipient: depositRecipient,
            tokenAddress: tokenAddress,
            amount: amount
        });
        // Add the new request to the mapping
        deposits[counter++] = newDeposit;
        emit DepositAcceptedIntoQueue(counter-1, depositRecipient, tokenAddress, amount);
    }


    function getUpdateForL2() public view returns (L1Update memory) {
        return getPendingRequests(lastProcessedUpdate_origin_l1 + 1, counter - 1);
    }

    function getOrderOfRequestsOriginatingOnL2(L2Update calldata update) private returns (UpdateType[] memory){
      if (update.cancles.length == 0 && update.withdraws.length == 0 ){
        return new UpdateType[](0);
      }

      uint256 withdrawalId = 0;
      uint256 cancelId = 0;
      uint256 orderId = 0;
      uint256 firstId;
      unchecked {
        firstId =  uint256(0) - 1;
      }
      uint256 updatesAmount = update.cancles.length + update.withdraws.length;
      UpdateType[] memory order = new UpdateType[](updatesAmount);

      if (update.cancles.length > 0){
        firstId = update.cancles[0].l2RequestId <  firstId ? update.cancles[0].l2RequestId : firstId;
      }

      if (update.withdraws.length > 0){
        firstId = update.withdraws[0].requestId <  firstId ? update.withdraws[0].requestId : firstId;
      }

      for( uint256 i = firstId; i < firstId + updatesAmount; i++) {
        if (withdrawalId < update.withdraws.length && update.withdraws[withdrawalId].requestId == i){
          order[orderId] = UpdateType.WITHDRAWAL;
          withdrawalId++;
          orderId++;
        }else if (cancelId < update.cancles.length && update.cancles[cancelId].l2RequestId == i){
          order[orderId] = UpdateType.CANCEL_RESOLUTION;
          cancelId++;
          orderId++;
        }else{
          revert("invalide L2Update");
        }
      }
      return order;
    }
     
    function processRequestsOriginatingOnL2(L2Update calldata inputArray) private {
        UpdateType[] memory order = getOrderOfRequestsOriginatingOnL2(inputArray);
        uint256 cancelId = 0;
        uint256 withdrawalId = 0; 
        for (uint256 i = 0; i < order.length; i++){
          if (order[i] == UpdateType.WITHDRAWAL) {
            process_l2_update_withdrawal(inputArray.withdraws[withdrawalId++]);
          }else if (order[i] == UpdateType.CANCEL_RESOLUTION) {
            process_l2_update_cancels(inputArray.cancles[cancelId++]);
          }else{
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
                inputArray.cancles.length >= 1 ||
                inputArray.withdraws.length >= 1,
            "Array must have at least 1 update"
        );

        uint256[]
            memory l2UpdatesToBeRemoved = process_l2_update_requests_results(
                inputArray.results
            );

        processRequestsOriginatingOnL2(inputArray);

        // Create a new array with the correct size
        if (l2UpdatesToBeRemoved.length > 0) {
            l2UpdatesToRemove[counter++]
                .l2UpdatesToRemove = l2UpdatesToBeRemoved; // .push(l2UpdatesToBeRemoved[i]);
            lastProcessedUpdate_origin_l1 += l2UpdatesToBeRemoved.length;
            emit L2UpdatesToRemovedAcceptedIntoQueue(
                counter - 1,
                l2UpdatesToBeRemoved
            );
        }
    }


    function process_l2_update_requests_results(
        RequestResult[] calldata results
    ) private returns (uint256[] memory) {
        uint256 updatesToBeRemovedCounter = 0;
        if (results.length == 0) {
            return new uint256[](0);
        }
        uint256 oderCounter = results[0].requestId;
        uint256[] memory l2UpdatesToBeRemovedTemp = new uint256[](
            results.length
        );

        for (uint256 idx = 1; idx < results.length; idx++) {
            if (results[idx].requestId != oderCounter + 1) {
              revert("Requests are not in order");
            }
            oderCounter = results[idx].requestId;
        }

        for (uint256 idx = 0; idx < results.length; idx++) {
            RequestResult calldata element = results[idx];

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

        uint256[] memory l2UpdatesToBeRemoved = new uint256[](
            updatesToBeRemovedCounter
        );

        for (uint256 i = 0; i < updatesToBeRemovedCounter; i++) {
            l2UpdatesToBeRemoved[i] = l2UpdatesToBeRemovedTemp[i];
        }

        return l2UpdatesToBeRemoved;
    }

    function process_l2_update_cancels(
        Cancel calldata cancel
    ) private {

            L1Update memory pending = getPendingRequests(
                cancel.lastProccessedRequestOnL1 + 1,
                cancel.lastAcceptedRequestOnL1
            );
            bytes32 correct_hash = keccak256(abi.encode(pending));

            CancelResolution memory resolution = CancelResolution({
                l2RequestId: cancel.l2RequestId,
                cancelJustified: correct_hash == cancel.hash
            });

            cancelResolutions[counter++] = resolution;
            emit DisputeResolutionAcceptedIntoQueue(
                resolution.l2RequestId,
                resolution.cancelJustified
            );
    }

    function process_l2_update_withdrawal(Withdraw calldata withdraw) private {
        IERC20 token = IERC20(withdraw.tokenAddress);
        require(
            token.balanceOf(address(this)) >= withdraw.amount,
            "Insufficient balance in the contract"
        );

        // Transfer tokens from the contract to the recipient
        token.transfer(withdraw.withdrawRecipient, withdraw.amount);

        emit FundsWithdrawn(withdraw.withdrawRecipient, withdraw.tokenAddress, withdraw.amount);
    }

    function getPendingRequestsRemove(uint256 id) private view returns (uint256[] memory){
      return l2UpdatesToRemove[id].l2UpdatesToRemove;
    }

    function getPendingRequests(
        uint256 start,
        uint256 end
    ) private view returns ( L1Update memory) {
        L1Update memory result;

        uint256 depositsCounter = 0;
        uint256 withdrawsCounter = 0;
        uint256 cancelsCounter = 0;
        uint256 updatesToBeRemovedCounter = 0;

        for (uint256 requestId = start; requestId <= end; requestId++) {
            if (deposits[requestId].depositRecipient != address(0)) {
              depositsCounter++;
            } else if (l2UpdatesToRemove[requestId].l2UpdatesToRemove.length > 0) {
              updatesToBeRemovedCounter++;
            } else if (cancelResolutions[requestId].l2RequestId > 0) {
              cancelsCounter++;
            }
        }

        uint256 requestCounter = depositsCounter + withdrawsCounter + cancelsCounter + updatesToBeRemovedCounter;
        result.order = new PendingRequestType[](requestCounter);

        result.offset = start;
        result.pendingDeposits = new Deposit[](depositsCounter);
        result.pendingCancelResultions = new CancelResolution[](cancelsCounter);
        result.pendingL2UpdatesToRemove = new L2UpdatesToRemove[](updatesToBeRemovedCounter);
        result.lastProccessedRequestOnL1 = lastProcessedUpdate_origin_l1;
        result.lastAcceptedRequestOnL1 = counter - 1;

        withdrawsCounter = 0;
        depositsCounter = 0;
        cancelsCounter = 0;
        updatesToBeRemovedCounter = 0;
        requestCounter = 0;

        for (uint256 requestId = start; requestId <= end; requestId++) {
            if (deposits[requestId].depositRecipient != address(0)) {
              result.order[requestCounter++] = PendingRequestType.DEPOSIT;
              result.pendingDeposits[depositsCounter++] = deposits[requestId];
            } else if ( l2UpdatesToRemove[requestId].l2UpdatesToRemove.length > 0) {
              result.order[requestCounter++] = PendingRequestType.L2_UPDATES_TO_REMOVE;
              result.pendingL2UpdatesToRemove[updatesToBeRemovedCounter++] = l2UpdatesToRemove[requestId];
            } else if (cancelResolutions[requestId].l2RequestId > 0) {
              result.order[requestCounter++] = PendingRequestType.CANCEL_RESOLUTION;
              result.pendingCancelResultions[cancelsCounter++] = cancelResolutions[requestId];
            }else{
              break;
            }
        }

        return result;
    }
}

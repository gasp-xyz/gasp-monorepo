// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

interface IRolldownPrimitives {

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

}

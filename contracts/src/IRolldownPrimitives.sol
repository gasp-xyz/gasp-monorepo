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

    // NOTE: PR DESC
    // event WithdrawalResolutionAcceptedIntoQueue(
    //     uint256 requestId,
    //     bool success
    // );
 
    // NOTE: PR DESC
    // event L2UpdatesToRemovedAcceptedIntoQueue(
    //     uint256 requestId,
    //     uint256[] l2UpdatesToRemove
    // );
    // NOTE: PR DESC
    // event FundsWithdrawn(
    //     address withdrawRecipient,
    //     address tokenAddress,
    //     uint256 amount
    // );
    // NOTE: PR DESC
    // event FundsReturned(
    //     address depositRecipient,
    //     address tokenAddress,
    //     uint256 amount
    // );

    // NOTE: PR DESC
    // event cancelAndCalculatedHash(bytes32 cancelHash, bytes32 calculatedHash);

    // NOTE: PR DESC
    // event EthWithdrawPending(address sender, uint amount);
    
    event NativeTokensWithdrawn(
      address sender,
      uint256 amount
    );

    event ERC20TokensWithdrawn(
      address sender, 
      address token_address, 
      uint256 amount
    );

    event WithdrawalClosed(
      uint256 requestId,
      bytes32 withdrawalHash
    );

    event NewUpdaterSet(address updater);

    enum Origin {
        L1,
        L2
    }

    struct RequestId {
        Origin origin;
        uint256 id;
    }

    struct Range {
        uint256 start;
        uint256 end;
    }

    struct Deposit {
        RequestId requestId;
        address depositRecipient;
        address tokenAddress;
        uint256 amount;
        uint256 timeStamp;
    }

    // NOTE: PR DESC
    // struct L2UpdatesToRemove {
    //     RequestId requestId;
    //     uint256[] l2UpdatesToRemove;
    //     uint256 timeStamp;
    // }

    struct CancelResolution {
        RequestId requestId;
        uint256 l2RequestId;
        bool cancelJustified;
        uint256 timeStamp;
    }

    // NOTE: PR DESC
    // struct WithdrawalResolution {
    //     RequestId requestId;
    //     uint256 l2RequestId;
    //     bool status;
    //     uint256 timeStamp;
    // }

		enum ChainId{ Ethereum, Arbitrum }

    struct L1Update {
        ChainId chain;
        Deposit[] pendingDeposits;
        CancelResolution[] pendingCancelResolutions;
        // NOTE: PR DESC
        // WithdrawalResolution[] pendingWithdrawalResolutions;
        // L2UpdatesToRemove[] pendingL2UpdatesToRemove;
    } 

    // NOTE: PR DESC
    ////TODO: should be renamed to RequestType
    //enum UpdateType {
    //    DEPOSIT,
    //    WITHDRAWAL,
    //    WITHDRAWAL_RESOLUTION,
    //    INDEX_UPDATE,
    //    CANCEL,
    //    CANCEL_RESOLUTION
    //}

    struct RequestResult {
        RequestId requestId;
        uint256 originRequestId;
        // NOTE: PR DESC
        // UpdateType updateType;
        bool status;
    }

    // NOTE: PR DESC
    // struct L2Update {
    //     Cancel[] cancels;
    //     Withdrawal[] withdrawals;
    //     RequestResult[] results;
    // }

    struct Cancel {
        RequestId requestId;
        Range range;
        bytes32 hash;
    }

    struct Withdrawal {
        RequestId requestId;
        address recipient;
        address tokenAddress;
        uint256 amount;
    }

}

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

    event L2UpdateAccepted(
      bytes32 root,
      Range range
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

    struct CancelResolution {
        RequestId requestId;
        uint256 l2RequestId;
        bool cancelJustified;
        uint256 timeStamp;
    }

		enum ChainId{ Ethereum, Arbitrum }

    struct L1Update {
        ChainId chain;
        Deposit[] pendingDeposits;
        CancelResolution[] pendingCancelResolutions;
    } 

    struct RequestResult {
        RequestId requestId;
        uint256 originRequestId;
        bool status;
    }

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

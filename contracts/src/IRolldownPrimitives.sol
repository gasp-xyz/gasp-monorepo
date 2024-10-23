// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

interface IRolldownPrimitives {
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
        uint256 ferryTip;
    }

    struct FailedDepositResolution {
        RequestId requestId;
        uint256 originRequestId;
        address ferry;
    }

    struct CancelResolution {
        RequestId requestId;
        uint256 l2RequestId;
        bool cancelJustified;
        uint256 timeStamp;
    }

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
        uint256 ferryTip;
    }

    enum Origin {
        L1,
        L2
    }

    enum ChainId {
        Ethereum,
        Arbitrum
    }

    enum L2RequestType {
        Withdrawal,
        Cancel,
        FailedDepositResolution
    }

    event DepositAcceptedIntoQueue(
        uint256 indexed requestId,
        address indexed depositRecipient,
        address indexed tokenAddress,
        uint256 amount,
        uint256 ferryTip
    );

    event DisputeResolutionAcceptedIntoQueue(
        uint256 indexed requestId, bool cancelJustified, bytes32 cancelResolutionHash
    );

    event NativeTokensWithdrawn(address indexed sender, uint256 amount);

    event ERC20TokensWithdrawn(address indexed sender, address indexed tokenAddress, uint256 amount);

    event WithdrawalClosed(uint256 indexed requestId, bytes32 withdrawalHash);

    event FerriedWithdrawalClosed(uint256 indexed requestId, bytes32 withdrawalHash);

    event WithdrawalFerried(
        uint256 indexedrequestId,
        uint256 amount,
        address indexed recipient,
        address indexed ferry,
        bytes32 withdrawalHash
    );

    event FailedDepositResolutionClosed(
        uint256 indexedrequestId, uint256 originDepositId, bytes32 failedDespotiResolutionHash
    );

    event L2UpdateAccepted(bytes32 root, Range range);

    event NewUpdaterSet(address indexed updater);
}

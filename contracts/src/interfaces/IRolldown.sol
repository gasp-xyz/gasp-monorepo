// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.13;

interface IRolldown {
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

    enum ChainId {
        Ethereum,
        Arbitrum
    }

    enum Origin {
        L1,
        L2
    }

    event DepositAcceptedIntoQueue(
        uint256 indexed requestId,
        address indexed depositRecipient,
        address indexed tokenAddress,
        uint256 amount,
        uint256 ferryTip
    );

    event DisputeResolutionAcceptedIntoQueue(uint256 indexed requestId, bool cancelJustified);

    event NativeTokensWithdrawn(address indexed sender, uint256 amount);

    event ERC20TokensWithdrawn(address indexed sender, address indexed tokenAddress, uint256 amount);

    event WithdrawalClosed(uint256 indexed requestId, bytes32 withdrawalHash);

    event FerriedWithdrawalClosed(uint256 indexed requestId, bytes32 withdrawalHash);

    event WithdrawalFerried(
        uint256 indexed requestId,
        uint256 amount,
        address indexed recipient,
        address indexed ferry,
        bytes32 withdrawalHash
    );

    event FailedDepositResolutionClosed(
        uint256 indexed requestId, uint256 indexed originDepositId, bytes32 failedDespotiResolutionHash
    );

    event L2UpdateAccepted(bytes32 root, Range range);

    event UpdaterSet(address indexed updater);

    function setUpdater(address updater) external;

    function depositNative() external payable;

    function depositNative(uint256 ferryTip) external payable;

    function deposit(address tokenAddress, uint256 amount) external;

    function deposit(address tokenAddress, uint256 amount, uint256 ferryTip) external;

    function depositERC20(address tokenAddress, uint256 amount) external;

    function depositERC20(address tokenAddress, uint256 amount, uint256 ferryTip) external;

    function ferryWithdrawal(Withdrawal calldata withdrawal) external payable;

    function closeWithdrawal(Withdrawal calldata withdrawal, bytes32 merkle_root, bytes32[] calldata proof) external;

    function closeCancel(Cancel calldata cancel, bytes32 merkle_root, bytes32[] calldata proof) external;

    function closeDepositRefund(
        FailedDepositResolution calldata failedDeposit,
        bytes32 merkleRoot,
        bytes32[] calldata proof
    ) external;

    function updateL1FromL2(bytes32 merkle_root, Range calldata range) external;

    function findL2Batch(uint256 requestId) external view returns (Range memory);

    function getUpdateForL2() external view returns (L1Update memory);

    function getMerkleRootsLength() external view returns (uint256);

    function getPendingRequests(uint256 start, uint256 end) external view returns (L1Update memory);

    function calculateRoot(bytes32 leaveHash, uint256 leaveIdx, bytes32[] calldata proof, uint256 leaveCount)
        external
        pure
        returns (bytes32);

    function calculateRootImpl(
        uint256 level,
        uint256 pos,
        bytes32 hash,
        bytes32[] calldata proofs,
        uint256 proofIdx,
        uint256 maxIdx
    ) external pure returns (bytes32);
}

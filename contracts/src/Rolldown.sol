// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.13;

import {IPauserRegistry, Pausable} from "@eigenlayer/contracts/permissions/Pausable.sol";
import {OwnableUpgradeable} from "@openzeppelin-upgrades/contracts/access/OwnableUpgradeable.sol";
import {Initializable} from "@openzeppelin-upgrades/contracts/proxy/utils/Initializable.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {IRolldown} from "./interfaces/IRolldown.sol";

contract Rolldown is Initializable, OwnableUpgradeable, ReentrancyGuard, Pausable, IRolldown {
    using SafeERC20 for IERC20;

    address public constant NATIVE_TOKEN_ADDRESS = 0x0000000000000000000000000000000000000001;
    address public constant CLOSED = 0x1111111111111111111111111111111111111111;

    // Counter for mapping key
    uint256 public counter;
    // Counter for last processed request to ensure not reading and processing already processed
    uint256 public lastProcessedUpdate_origin_l1;
    // Counter for last processed updates comming from l2 to ensure not reading and processing already processed
    uint256 public lastProcessedUpdate_origin_l2;
    // Chain identificator
    ChainId public chain;
    // Updater addcount address
    address public updaterAccount;

    // NOTE: PR DESC
    // mapping(uint256 => WithdrawalResolution) public withdrawalResolutions;
    mapping(uint256 => CancelResolution) public cancelResolutions;
    mapping(uint256 => Deposit) public deposits;
    // NOTE: PR DESC
    // mapping(uint256 => L2UpdatesToRemove) internal l2UpdatesToRemove;
    // NOTE: PR DESC
    // mapping(address => uint) public pendingEthWithdrawals;

    mapping(bytes32 => Range) public merkleRootRange;
    mapping(bytes32 => address) public processedL2Requests;
    // stores all merkle roots in order, seems like binary search on this array
    // is the most efficient way to find merkle root that contains particular tx id
    bytes32[] public roots;

    function initialize(IPauserRegistry _pauserRegistry, address initialOwner, ChainId chainId, address updater)
        external
        initializer
    {
        _initializePauser(_pauserRegistry, UNPAUSE_ALL);
        _transferOwnership(initialOwner);
        lastProcessedUpdate_origin_l1 = 0;
        counter = 1;
        lastProcessedUpdate_origin_l2 = 0;
        chain = chainId;
        updaterAccount = updater;
    }

    function setUpdater(address updater) external override onlyOwner whenNotPaused {
        updaterAccount = updater;
        emit NewUpdaterSet(updaterAccount);
    }

    function depositNative() external payable override nonReentrant whenNotPaused {
        _depositNativeWithTip(0);
    }

    function depositNative(uint256 ferryTip) external payable override nonReentrant whenNotPaused {
        _depositNativeWithTip(ferryTip);
    }

    function deposit(address tokenAddress, uint256 amount) external override nonReentrant whenNotPaused {
        _depositERC20WithTip(tokenAddress, amount, 0);
    }

    function deposit(address tokenAddress, uint256 amount, uint256 ferryTip)
        external
        override
        nonReentrant
        whenNotPaused
    {
        _depositERC20WithTip(tokenAddress, amount, ferryTip);
    }

    function depositERC20(address tokenAddress, uint256 amount) external override nonReentrant whenNotPaused {
        _depositERC20WithTip(tokenAddress, amount, 0);
    }

    function depositERC20(address tokenAddress, uint256 amount, uint256 ferryTip)
        external
        override
        nonReentrant
        whenNotPaused
    {
        _depositERC20WithTip(tokenAddress, amount, ferryTip);
    }

    function ferryWithdrawal(Withdrawal calldata withdrawal) external payable override nonReentrant whenNotPaused {
        require(withdrawal.ferryTip <= withdrawal.amount, "Tip exceeds deposited amount");
        uint256 ferriedAmount = withdrawal.amount - withdrawal.ferryTip;
        bytes32 withdrawalHash = keccak256(abi.encode(withdrawal));

        require(processedL2Requests[withdrawalHash] == address(0), "Already ferried");
        processedL2Requests[withdrawalHash] = msg.sender;

        if (withdrawal.tokenAddress == NATIVE_TOKEN_ADDRESS) {
            // console.log(msg.value);
            // console.log(ferriedAmount);
            require(msg.value > 0, "Native token not sent");
            require(msg.value == ferriedAmount, "Sent amount must exactly match amount without ferryTip");
            payable(withdrawal.recipient).transfer(ferriedAmount);
            emit WithdrawalFerried(
                withdrawal.requestId.id, ferriedAmount, withdrawal.recipient, msg.sender, withdrawalHash
            );
        } else {
            IERC20 token = IERC20(withdrawal.tokenAddress);
            require(token.balanceOf(address(msg.sender)) >= ferriedAmount, "Not enough funds");
            token.transferFrom(msg.sender, withdrawal.recipient, ferriedAmount);
            emit WithdrawalFerried(
                withdrawal.requestId.id, ferriedAmount, withdrawal.recipient, msg.sender, withdrawalHash
            );
        }
    }

    function closeWithdrawal(Withdrawal calldata withdrawal, bytes32 merkleRoot, bytes32[] calldata proof)
        external
        override
        nonReentrant
        whenNotPaused
    {
        _verifyRequestProof(withdrawal.requestId.id, keccak256(abi.encode(withdrawal)), merkleRoot, proof);
        bytes32 withdrawalHash = keccak256(abi.encode(withdrawal));

        address ferryAddress = processedL2Requests[withdrawalHash];
        bool isFerried = ferryAddress != address(0);
        processedL2Requests[withdrawalHash] = CLOSED;

        if (!isFerried) {
            if (withdrawal.tokenAddress == NATIVE_TOKEN_ADDRESS) {
                _sendNativeAndEmitEvent(withdrawal.recipient, withdrawal.amount - withdrawal.ferryTip);
                if (withdrawal.ferryTip > 0) {
                    _sendNativeAndEmitEvent(msg.sender, withdrawal.ferryTip);
                }
            } else {
                _sendERC20AndEmitEvent(
                    withdrawal.recipient, withdrawal.tokenAddress, withdrawal.amount - withdrawal.ferryTip
                );
                if (withdrawal.ferryTip > 0) {
                    _sendERC20AndEmitEvent(msg.sender, withdrawal.tokenAddress, withdrawal.ferryTip);
                }
            }

            emit WithdrawalClosed(withdrawal.requestId.id, keccak256(abi.encode(withdrawal)));
        } else {
            if (withdrawal.tokenAddress == NATIVE_TOKEN_ADDRESS) {
                _sendNativeAndEmitEvent(ferryAddress, withdrawal.amount);
            } else {
                _sendERC20AndEmitEvent(ferryAddress, withdrawal.tokenAddress, withdrawal.amount);
            }

            emit FerriedWithdrawalClosed(withdrawal.requestId.id, keccak256(abi.encode(withdrawal)));
        }
    }

    function closeCancel(Cancel calldata cancel, bytes32 merkleRoot, bytes32[] calldata proof)
        external
        override
        nonReentrant
        whenNotPaused
    {
        bytes32 hash = keccak256(abi.encode(cancel));
        _verifyRequestProof(cancel.requestId.id, hash, merkleRoot, proof);
        _processL2UpdateCancels(cancel);
        processedL2Requests[hash] = CLOSED;
    }

    function closeDepositRefund(
        FailedDepositResolution calldata failedDeposit,
        bytes32 merkleRoot,
        bytes32[] calldata proof
    ) external override nonReentrant whenNotPaused {
        bytes32 hash = keccak256(abi.encode(failedDeposit));
        _verifyRequestProof(failedDeposit.requestId.id, keccak256(abi.encode(failedDeposit)), merkleRoot, proof);
        _processL2UpdateFailedDeposit(failedDeposit);
        processedL2Requests[hash] = CLOSED;
    }

    // TODO:
    // - verify that merkleRoot is correct (passing TaskResponse along with the merkle root?)
    // - verify that range is correct and belongs to particular merkleRoot
    function updateL1FromL2(bytes32 merkleRoot, Range calldata range /*,TaskResponse calldata response ??? */ )
        external
        override
        whenNotPaused
    {
        require(msg.sender == updaterAccount, "Not the owner");
        require(range.end > lastProcessedUpdate_origin_l2, "Update brings no new data");
        require(range.start > 0, "range id must be greater than 0");
        require(range.start - 1 <= lastProcessedUpdate_origin_l2, "Previous update missing");
        require(range.end >= range.start, "Invalid range");
        roots.push(merkleRoot);
        merkleRootRange[merkleRoot] = range;
        lastProcessedUpdate_origin_l2 = range.end;
        emit L2UpdateAccepted(merkleRoot, range);
    }

    function findL2Batch(uint256 requestId) external view override returns (Range memory) {
        require(requestId <= lastProcessedUpdate_origin_l2, "Invalid request id");
        if (roots.length == 0) {
            return Range({start: 0, end: 0});
        }

        uint256 rootCount = roots.length;
        for (uint256 i = rootCount - 1; i >= 0;) {
            if (requestId >= merkleRootRange[roots[i]].start && requestId <= merkleRootRange[roots[i]].end) {
                return merkleRootRange[roots[i]];
            }

            unchecked {
                --i;
            }
        }

        return Range({start: 0, end: 0});
    }

    function getUpdateForL2() external view override returns (L1Update memory) {
        return getPendingRequests(lastProcessedUpdate_origin_l1 + 1, counter - 1);
    }

    function getMerkleRootsLength() external view override returns (uint256) {
        return roots.length;
    }

    function getPendingRequests(uint256 start, uint256 end) public view override returns (L1Update memory) {
        L1Update memory result;

        result.chain = chain;
        uint256 depositCounter = 0;
        uint256 cancelCounter = 0;

        if (start == 0 && end == 0) {
            return result;
        }

        for (uint256 i = start; i <= end;) {
            if (deposits[i].requestId.id != 0) {
                ++depositCounter;
            } else if (cancelResolutions[i].requestId.id != 0) {
                ++cancelCounter;
            } else {
                revert("Invalid range");
            }

            unchecked {
                ++i;
            }
        }

        result.pendingDeposits = new Deposit[](depositCounter);
        result.pendingCancelResolutions = new CancelResolution[](cancelCounter);

        depositCounter = 0;
        cancelCounter = 0;

        for (uint256 i = start; i <= end;) {
            if (deposits[i].requestId.id > 0) {
                result.pendingDeposits[depositCounter++] = deposits[i];
            } else if (cancelResolutions[i].l2RequestId > 0) {
                result.pendingCancelResolutions[cancelCounter++] = cancelResolutions[i];
            } else {
                break;
            }

            unchecked {
                ++i;
            }
        }

        return result;
    }

    // TODO: move to separate modoule/contract
    function calculateRoot(bytes32 leaveHash, uint32 leaveIdx, bytes32[] calldata proof, uint32 leaveCount)
        public
        pure
        override
        returns (bytes32)
    {
        uint32 levels = 0;
        uint32 tmp = leaveCount;
        while (tmp > 0) {
            tmp = tmp / 2;
            levels += 1;
        }
        return calculateRootImpl(levels, leaveIdx, leaveHash, proof, 0, leaveCount - 1);
    }

    function calculateRootImpl(
        uint32 level,
        uint32 pos,
        bytes32 hash,
        bytes32[] calldata proofs,
        uint32 proofIdx,
        uint32 maxIdx
    ) public pure override returns (bytes32) {
        if (pos % 2 == 0) {
            if (pos == maxIdx) {
                // promoted node
            } else {
                hash = keccak256(abi.encodePacked(hash, proofs[proofIdx++]));
            }
        } else {
            hash = keccak256(abi.encodePacked(proofs[proofIdx++], hash));
        }

        if (level == 1) {
            return hash;
        } else {
            return calculateRootImpl(level - 1, pos / 2, hash, proofs, proofIdx, maxIdx / 2);
        }
    }

    function _depositNativeWithTip(uint256 ferryTip) private {
        require(ferryTip <= msg.value, "Tip exceeds deposited amount");
        require(msg.value > 0, "msg value must be greater that 0");
        address depositRecipient = msg.sender;
        uint256 amount = msg.value;

        uint256 timeStamp = block.timestamp;
        Deposit memory depositRequest = Deposit({
            requestId: RequestId({origin: Origin.L1, id: counter++}),
            depositRecipient: depositRecipient,
            tokenAddress: NATIVE_TOKEN_ADDRESS,
            amount: amount,
            timeStamp: timeStamp,
            ferryTip: ferryTip
        });

        // Add the new request to the mapping
        deposits[depositRequest.requestId.id] = depositRequest;
        emit DepositAcceptedIntoQueue(
            depositRequest.requestId.id, depositRecipient, NATIVE_TOKEN_ADDRESS, amount, ferryTip
        );
    }

    function _depositERC20WithTip(address tokenAddress, uint256 amount, uint256 ferryTip) private {
        require(ferryTip <= amount, "Tip exceeds deposited amount");
        require(tokenAddress != address(0), "Invalid token address");
        require(amount > 0, "Amount must be greater than zero");
        address depositRecipient = msg.sender;

        IERC20 token = IERC20(tokenAddress);
        token.safeTransferFrom(msg.sender, address(this), amount);

        uint256 timeStamp = block.timestamp;
        Deposit memory depositRequest = Deposit({
            requestId: RequestId({origin: Origin.L1, id: counter++}),
            depositRecipient: depositRecipient,
            tokenAddress: tokenAddress,
            amount: amount,
            timeStamp: timeStamp,
            ferryTip: ferryTip
        });
        // Add the new request to the mapping
        deposits[depositRequest.requestId.id] = depositRequest;
        emit DepositAcceptedIntoQueue(depositRequest.requestId.id, depositRecipient, tokenAddress, amount, ferryTip);
    }

    function _processL2UpdateFailedDeposit(FailedDepositResolution calldata failedDeposit) private {
        Deposit storage originDeposit = deposits[failedDeposit.originRequestId];
        address recipient = originDeposit.depositRecipient;

        if (failedDeposit.ferry != address(0)) {
            recipient = failedDeposit.ferry;
        }

        if (originDeposit.tokenAddress == NATIVE_TOKEN_ADDRESS) {
            _sendNativeAndEmitEvent(recipient, originDeposit.amount);
        } else {
            _sendERC20AndEmitEvent(recipient, originDeposit.tokenAddress, originDeposit.amount);
        }

        emit FailedDepositResolutionClosed(
            failedDeposit.requestId.id, failedDeposit.originRequestId, keccak256(abi.encode(failedDeposit))
        );
    }

    function _processL2UpdateCancels(Cancel calldata cancel) private {
        bool cancelJustified = false;

        if (cancel.range.end > counter - 1) {
            cancelJustified = true;
        } else {
            L1Update memory pending = getPendingRequests(cancel.range.start, cancel.range.end);
            bytes32 correctHash = keccak256(abi.encode(pending));
            cancelJustified = correctHash != cancel.hash;
        }
        uint256 timeStamp = block.timestamp;

        CancelResolution memory resolution = CancelResolution({
            requestId: RequestId({origin: Origin.L1, id: counter++}),
            l2RequestId: cancel.requestId.id,
            cancelJustified: cancelJustified,
            timeStamp: timeStamp
        });

        cancelResolutions[resolution.requestId.id] = resolution;
        emit DisputeResolutionAcceptedIntoQueue(resolution.l2RequestId, resolution.cancelJustified);
    }

    function _sendNativeAndEmitEvent(address recipient, uint256 amount) private {
        require(payable(address(this)).balance >= amount, "Not enough funds in contract");
        require(amount > 0, "Amount must be greater than zero");
        emit NativeTokensWithdrawn(recipient, amount);
        Address.sendValue(payable(recipient), amount);
    }

    function _sendERC20AndEmitEvent(address recipient, address tokenAddress, uint256 amount) private {
        IERC20 token = IERC20(tokenAddress);
        require(token.balanceOf(address(this)) >= amount, "Not enough funds in contract");
        require(amount > 0, "Amount must be greater than zero");

        token.safeTransfer(recipient, amount);
        emit ERC20TokensWithdrawn(recipient, tokenAddress, amount);
    }

    function _verifyRequestProof(uint256 requestId, bytes32 requestHash, bytes32 merkleRoot, bytes32[] calldata proof)
        private
        view
    {
        Range memory r = merkleRootRange[merkleRoot];
        require(r.start != 0 && r.end != 0, "Unknown merkle root");

        require(processedL2Requests[requestHash] != CLOSED, "Already processed");

        uint32 leaveCount = uint32(r.end - r.start + 1);
        uint32 pos = uint32(requestId - r.start);
        require(calculateRoot(requestHash, pos, proof, leaveCount) == merkleRoot, "Invalid proof");
    }

    function _min(uint256 a, uint256 b) private pure returns (uint256) {
        return a < b ? a : b;
    }

    function _max(uint256 a, uint256 b) private pure returns (uint256) {
        return a > b ? a : b;
    }
}

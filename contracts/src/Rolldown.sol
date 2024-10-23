// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {IPauserRegistry, Pausable} from "@eigenlayer/contracts/permissions/Pausable.sol";
import {OwnableUpgradeable} from "@openzeppelin-upgrades/contracts/access/OwnableUpgradeable.sol";
import {Initializable} from "@openzeppelin-upgrades/contracts/proxy/utils/Initializable.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {IRolldown} from "./IRolldown.sol";
import {IRolldownPrimitives} from "./IRolldownPrimitives.sol";
import {LMerkleTree} from "./LMerkleTree.sol";

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
    // Chain identificator
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

    function initialize(IPauserRegistry _pauserRegistry, address owner, ChainId chainId, address updater)
        external
        initializer
    {
        require(owner != address(0), "Zero owner address");
        _transferOwnership(owner);

        require(updater != address(0), "Zero updater address");
        updaterAccount = updater;

        _initializePauser(_pauserRegistry, UNPAUSE_ALL);

        counter = 1;
        chain = chainId;
    }

    function setUpdater(address updater) external override onlyOwner whenNotPaused {
        require(updater != address(0), "Zero updater address");

        updaterAccount = updater;
        emit NewUpdaterSet(updaterAccount);
    }

    function deposit_native() external payable override nonReentrant whenNotPaused {
        _depositNativeWithTip(0);
    }

    function depositNative() external payable override nonReentrant whenNotPaused {
        _depositNativeWithTip(0);
    }

    function deposit_native(uint256 ferryTip) external payable override nonReentrant whenNotPaused {
        _depositNativeWithTip(ferryTip);
    }

    function depositNative(uint256 ferryTip) external payable override nonReentrant whenNotPaused {
        _depositNativeWithTip(ferryTip);
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

    function deposit_erc20(address tokenAddress, uint256 amount) external override nonReentrant whenNotPaused {
        _depositERC20WithTip(tokenAddress, amount, 0);
    }

    function depositERC20(address tokenAddress, uint256 amount) external override nonReentrant whenNotPaused {
        _depositERC20WithTip(tokenAddress, amount, 0);
    }

    function deposit_erc20(address tokenAddress, uint256 amount, uint256 ferryTip)
        external
        override
        nonReentrant
        whenNotPaused
    {
        _depositERC20WithTip(tokenAddress, amount, ferryTip);
    }

    function depositERC20(address tokenAddress, uint256 amount, uint256 ferryTip)
        external
        override
        nonReentrant
        whenNotPaused
    {
        _depositERC20WithTip(tokenAddress, amount, ferryTip);
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

    function getUpdateForL2() external view override returns (L1Update memory) {
        return getPendingRequests(lastProcessedUpdate_origin_l1 + 1, counter - 1);
    }

    function ferry_withdrawal(Withdrawal calldata withdrawal) external payable override nonReentrant whenNotPaused {
        _ferryWithdrawal(withdrawal);
    }

    function ferryWithdrawal(Withdrawal calldata withdrawal) external payable override nonReentrant whenNotPaused {
        _ferryWithdrawal(withdrawal);
    }

    function _ferryWithdrawal(Withdrawal calldata withdrawal) private {
        require(withdrawal.ferryTip <= withdrawal.amount, "Tip exceeds deposited amount");
        uint256 ferriedAmount = withdrawal.amount - withdrawal.ferryTip;
        bytes32 withdrawalHash = hashWithdrawal(withdrawal);

        require(processedL2Requests[withdrawalHash] == address(0), "Already ferried");
        processedL2Requests[withdrawalHash] = msg.sender;

        if (withdrawal.tokenAddress == NATIVE_TOKEN_ADDRESS) {
            require(msg.value > 0, "Native token not sent");
            require(msg.value == ferriedAmount, "Sent amount must exactly match amount without ferryTip");
            payable(withdrawal.recipient).transfer(ferriedAmount);
            emit WithdrawalFerried(
                withdrawal.requestId.id, ferriedAmount, withdrawal.recipient, msg.sender, withdrawalHash
            );
        } else {
            IERC20 token = IERC20(withdrawal.tokenAddress);
            require(token.balanceOf(address(msg.sender)) >= ferriedAmount, "Not enough funds");
            token.safeTransferFrom(msg.sender, withdrawal.recipient, ferriedAmount);
            emit WithdrawalFerried(
                withdrawal.requestId.id, ferriedAmount, withdrawal.recipient, msg.sender, withdrawalHash
            );
        }
    }

    function close_withdrawal(Withdrawal calldata withdrawal, bytes32 merkleRoot, bytes32[] calldata proof)
        external
        override
        nonReentrant
        whenNotPaused
    {
        _closeWithdrawal(withdrawal, merkleRoot, proof);
    }

    function closeWithdrawal(Withdrawal calldata withdrawal, bytes32 merkleRoot, bytes32[] calldata proof)
        external
        override
        nonReentrant
        whenNotPaused
    {
        _closeWithdrawal(withdrawal, merkleRoot, proof);
    }

    function _closeWithdrawal(Withdrawal calldata withdrawal, bytes32 merkleRoot, bytes32[] calldata proof) private {
        bytes32 withdrawalHash = hashWithdrawal(withdrawal);
        _verifyRequestProof(withdrawal.requestId.id, withdrawalHash, merkleRoot, proof);

        address ferryAddress = processedL2Requests[withdrawalHash];
        bool isFerried = ferryAddress != address(0);
        processedL2Requests[withdrawalHash] = CLOSED;

        if (!isFerried) {
            if (withdrawal.tokenAddress == NATIVE_TOKEN_ADDRESS) {
                _sendNative(withdrawal.recipient, withdrawal.amount - withdrawal.ferryTip);
                if (withdrawal.ferryTip > 0) {
                    _sendNative(msg.sender, withdrawal.ferryTip);
                }
            } else {
                _sendERC20(withdrawal.recipient, withdrawal.tokenAddress, withdrawal.amount - withdrawal.ferryTip);
                if (withdrawal.ferryTip > 0) {
                    _sendERC20(msg.sender, withdrawal.tokenAddress, withdrawal.ferryTip);
                }
            }

            emit WithdrawalClosed(withdrawal.requestId.id, withdrawalHash);
        } else {
            if (withdrawal.tokenAddress == NATIVE_TOKEN_ADDRESS) {
                _sendNative(ferryAddress, withdrawal.amount);
            } else {
                _sendERC20(ferryAddress, withdrawal.tokenAddress, withdrawal.amount);
            }

            emit FerriedWithdrawalClosed(withdrawal.requestId.id, withdrawalHash);
        }
    }

    function find_l2_batch(uint256 requestId) external view override returns (bytes32) {
        return _findL2Batch(requestId);
    }

    function findL2Batch(uint256 requestId) external view override returns (bytes32) {
        return _findL2Batch(requestId);
    }

    function _findL2Batch(uint256 requestId) private view returns (bytes32) {
        require(requestId <= lastProcessedUpdate_origin_l2, "Invalid request id");
        if (roots.length == 0) {
            revert("there are no roots yet on the contract");
        }

        for (uint256 i = roots.length - 1; i >= 0; i--) {
            if (requestId >= merkleRootRange[roots[i]].start && requestId <= merkleRootRange[roots[i]].end) {
                return roots[i];
            }
        }

        revert("couldnt find the batch containing the request");
    }

    function _verifyRequestProof(uint256 requestId, bytes32 requestHash, bytes32 merkleRoot, bytes32[] calldata proof)
        private
        view
    {
        Range memory r = merkleRootRange[merkleRoot];
        require(r.start != 0 && r.end != 0, "Unknown merkle root");

        require(processedL2Requests[requestHash] != CLOSED, "Already processed");

        if (r.end < r.start) {
            revert("Invalid request range, end < start");
        }

        if (requestId < r.start || requestId > r.end) {
            revert("Request id outside of range");
        }

        if (r.end - r.start + 1 > type(uint32).max) {
            revert("Range too big");
        }

        uint32 leaveCount = uint32(r.end - r.start + 1);
        uint32 pos = uint32(requestId - r.start);
        require(LMerkleTree.calculateRoot(requestHash, pos, proof, leaveCount) == merkleRoot, "Invalid proof");
    }

    function hashWithdrawal(Withdrawal calldata withdrawal) public pure override returns (bytes32) {
        return keccak256(bytes.concat(abi.encode(L2RequestType.Withdrawal), abi.encode(withdrawal)));
    }

    function hashCancel(Cancel calldata cancel) public pure override returns (bytes32) {
        return keccak256(bytes.concat(abi.encode(L2RequestType.Cancel), abi.encode(cancel)));
    }

    function hashFailedDepositResolution(FailedDepositResolution calldata failedDeposit)
        public
        pure
        override
        returns (bytes32)
    {
        return keccak256(bytes.concat(abi.encode(L2RequestType.FailedDepositResolution), abi.encode(failedDeposit)));
    }

    function close_cancel(Cancel calldata cancel, bytes32 merkleRoot, bytes32[] calldata proof)
        external
        override
        whenNotPaused
        nonReentrant
    {
        _closeCancel(cancel, merkleRoot, proof);
    }

    function closeCancel(Cancel calldata cancel, bytes32 merkleRoot, bytes32[] calldata proof)
        external
        override
        whenNotPaused
        nonReentrant
    {
        _closeCancel(cancel, merkleRoot, proof);
    }

    function _closeCancel(Cancel calldata cancel, bytes32 merkleRoot, bytes32[] calldata proof) private {
        bytes32 hash = hashCancel(cancel);
        _verifyRequestProof(cancel.requestId.id, hash, merkleRoot, proof);
        _processL2UpdateCancels(cancel, hash);
        processedL2Requests[hash] = CLOSED;
    }

    function close_deposit_refund(
        FailedDepositResolution calldata failedDeposit,
        bytes32 merkleRoot,
        bytes32[] calldata proof
    ) external override nonReentrant whenNotPaused {
        _closeDepositRefund(failedDeposit, merkleRoot, proof);
    }

    function closeDepositRefund(
        FailedDepositResolution calldata failedDeposit,
        bytes32 merkleRoot,
        bytes32[] calldata proof
    ) external override nonReentrant whenNotPaused {
        _closeDepositRefund(failedDeposit, merkleRoot, proof);
    }

    function _closeDepositRefund(
        FailedDepositResolution calldata failedDeposit,
        bytes32 merkleRoot,
        bytes32[] calldata proof
    ) private {
        bytes32 hash = hashFailedDepositResolution(failedDeposit);
        _verifyRequestProof(failedDeposit.requestId.id, hash, merkleRoot, proof);
        _processL2UpdateFailedDeposit(failedDeposit, hash);
        processedL2Requests[hash] = CLOSED;
    }

    function _processL2UpdateFailedDeposit(FailedDepositResolution calldata failedDeposit, bytes32 failedDepositHash)
        private
    {
        Deposit storage originDeposit = deposits[failedDeposit.originRequestId];
        address recipient = originDeposit.depositRecipient;

        if (failedDeposit.ferry != address(0)) {
            recipient = failedDeposit.ferry;
        }

        if (originDeposit.tokenAddress == NATIVE_TOKEN_ADDRESS) {
            _sendNative(recipient, originDeposit.amount);
        } else {
            _sendERC20(recipient, originDeposit.tokenAddress, originDeposit.amount);
        }

        emit FailedDepositResolutionClosed(failedDeposit.requestId.id, failedDeposit.originRequestId, failedDepositHash);
    }

    function update_l1_from_l2(bytes32 merkleRoot, Range calldata range) external override whenNotPaused {
        _updateL1FromL2(merkleRoot, range);
    }

    function updateL1FromL2(bytes32 merkleRoot, Range calldata range) external override whenNotPaused {
        _updateL1FromL2(merkleRoot, range);
    }

    // TODO:
    // - verify that merkleRoot is correct (passing TaskResponse along with the merkle root?)
    // - verify that range is correct and belongs to particular merkleRoot
    function _updateL1FromL2(bytes32 merkleRoot, Range calldata range /*,TaskResponse calldata response ??? */ )
        private
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

    function getMerkleRootsLength() external view override returns (uint256) {
        return roots.length;
    }

    function _processL2UpdateCancels(Cancel calldata cancel, bytes32 cancelHash) private {
        bool cancelJustified = false;

        if (cancel.range.end > counter - 1) {
            cancelJustified = true;
        } else {
            L1Update memory pending = getPendingRequests(cancel.range.start, cancel.range.end);
            bytes32 correct_hash = keccak256(abi.encode(pending));
            cancelJustified = correct_hash != cancel.hash;
        }
        uint256 timeStamp = block.timestamp;

        CancelResolution memory resolution = CancelResolution({
            requestId: RequestId({origin: Origin.L1, id: counter++}),
            l2RequestId: cancel.requestId.id,
            cancelJustified: cancelJustified,
            timeStamp: timeStamp
        });

        cancelResolutions[resolution.requestId.id] = resolution;
        emit DisputeResolutionAcceptedIntoQueue(resolution.l2RequestId, resolution.cancelJustified, cancelHash);
    }

    function _sendNative(address recipient, uint256 amount) private {
        require(payable(address(this)).balance >= amount, "Not enough funds in contract");
        require(amount > 0, "Amount must be greater than zero");
        emit NativeTokensWithdrawn(recipient, amount);
        Address.sendValue(payable(recipient), amount);
    }

    function _sendERC20(address recipient, address tokenAddress, uint256 amount) private {
        IERC20 token = IERC20(tokenAddress);
        require(token.balanceOf(address(this)) >= amount, "Not enough funds in contract");
        require(amount > 0, "Amount must be greater than zero");

        token.safeTransfer(recipient, amount);
        emit ERC20TokensWithdrawn(recipient, tokenAddress, amount);
    }

    function getPendingRequests(uint256 start, uint256 end) public view override returns (L1Update memory) {
        L1Update memory result;

        result.chain = chain;
        uint256 depositsCounter = 0;
        uint256 cancelsCounter = 0;

        if (start == 0 && end == 0) {
            return result;
        }

        for (uint256 requestId = start; requestId <= end; requestId++) {
            if (deposits[requestId].requestId.id != 0) {
                depositsCounter++;
            } else if (cancelResolutions[requestId].requestId.id != 0) {
                cancelsCounter++;
            } else {
                revert("Invalid range");
            }
        }

        result.pendingDeposits = new Deposit[](depositsCounter);
        result.pendingCancelResolutions = new CancelResolution[](cancelsCounter);

        depositsCounter = 0;
        cancelsCounter = 0;

        for (uint256 requestId = start; requestId <= end; requestId++) {
            if (deposits[requestId].requestId.id > 0) {
                result.pendingDeposits[depositsCounter++] = deposits[requestId];
            } else if (cancelResolutions[requestId].l2RequestId > 0) {
                result.pendingCancelResolutions[cancelsCounter++] = cancelResolutions[requestId];
            } else {
                break;
            }
        }

        return result;
    }
}

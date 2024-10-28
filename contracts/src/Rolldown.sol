// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {Initializable} from "@openzeppelin-upgrades/contracts/proxy/utils/Initializable.sol";
import {AccessControlUpgradeable} from "@openzeppelin-upgrades/contracts/access/AccessControlUpgradeable.sol";
import {ContextUpgradeable} from "@openzeppelin-upgrades/contracts/utils/ContextUpgradeable.sol";
import {ReentrancyGuardUpgradeable} from "@openzeppelin-upgrades/contracts/security/ReentrancyGuardUpgradeable.sol";
import {IPauserRegistry, Pausable} from "@eigenlayer/contracts/permissions/Pausable.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {IRolldown} from "./IRolldown.sol";
import {IRolldownPrimitives} from "./IRolldownPrimitives.sol";
import {LMerkleTree} from "./LMerkleTree.sol";

contract Rolldown is
    Initializable,
    ContextUpgradeable,
    AccessControlUpgradeable,
    ReentrancyGuardUpgradeable,
    Pausable,
    IRolldown
{
    using Address for address payable;
    using SafeERC20 for IERC20;

    bytes32 public constant UPDATER_ROLE = keccak256("UPDATER_ROLE");
    address public constant NATIVE_TOKEN_ADDRESS = 0x0000000000000000000000000000000000000001;
    address public constant CLOSED = 0x1111111111111111111111111111111111111111;

    // Counter for mapping key
    uint256 public override counter;
    // Counter for last processed request to ensure not reading and processing already processed
    uint256 public override lastProcessedUpdate_origin_l1;
    // Counter for last processed updates comming from l2 to ensure not reading and processing already processed
    uint256 public override lastProcessedUpdate_origin_l2;
    // Chain identificator
    ChainId public override chain;
    // Updater account address
    address public override updaterAccount;

    // NOTE: PR DESC
    // mapping(uint256 => WithdrawalResolution) public withdrawalResolutions;
    mapping(uint256 => CancelResolution) public cancelResolutions;

    mapping(uint256 => Deposit) public deposits;
    // NOTE: PR DESC
    // mapping(uint256 => L2UpdatesToRemove) internal l2UpdatesToRemove;
    // NOTE: PR DESC
    // mapping(address => uint) public pendingEthWithdrawals;

    mapping(bytes32 => Range) public merkleRootRange;

    mapping(bytes32 => address) public override processedL2Requests;
    // stores all merkle roots in order, seems like binary search on this array
    // is the most efficient way to find merkle root that contains particular tx id
    bytes32[] public roots;

    function initialize(IPauserRegistry pauserRegistry_, address admin, ChainId chainId, address updater)
        external
        initializer
    {
        ContextUpgradeable.__Context_init();
        AccessControlUpgradeable.__AccessControl_init();
        ReentrancyGuardUpgradeable.__ReentrancyGuard_init();

        require(admin != address(0), "Zero admin address");
        _grantRole(DEFAULT_ADMIN_ROLE, admin);

        require(updater != address(0), "Zero updater address");
        _grantRole(UPDATER_ROLE, updater);
        updaterAccount = updater;

        _initializePauser(pauserRegistry_, UNPAUSE_ALL);

        counter = 1;
        lastProcessedUpdate_origin_l1 = 0;
        lastProcessedUpdate_origin_l2 = 0;
        chain = chainId;
    }

    function setUpdater(address updater) external override onlyRole(DEFAULT_ADMIN_ROLE) {
        require(updater != address(0), "Zero updater address");

        updaterAccount = updater;
        emit NewUpdaterSet(updaterAccount);
    }

    function deposit_native() external payable override whenNotPaused {
        _depositNativeWithTip(0);
    }

    function depositNative() external payable override whenNotPaused {
        _depositNativeWithTip(0);
    }

    function deposit_native(uint256 ferryTip) external payable override whenNotPaused {
        _depositNativeWithTip(ferryTip);
    }

    function depositNative(uint256 ferryTip) external payable override whenNotPaused {
        _depositNativeWithTip(ferryTip);
    }

    function _depositNativeWithTip(uint256 ferryTip) private {
        uint256 amount = msg.value;
        require(amount > 0, "Value must be greater than zero");
        require(ferryTip <= amount, "Tip exceeds deposited amount");

        address depositRecipient = _msgSender();

        Deposit memory depositRequest = Deposit({
            requestId: RequestId({origin: Origin.L1, id: counter++}),
            depositRecipient: depositRecipient,
            tokenAddress: NATIVE_TOKEN_ADDRESS,
            amount: amount,
            timeStamp: block.timestamp,
            ferryTip: ferryTip
        });
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
        require(tokenAddress != address(0), "Invalid token address");
        require(amount > 0, "Amount must be greater than zero");
        require(ferryTip <= amount, "Tip exceeds deposited amount");

        address depositRecipient = _msgSender();

        Deposit memory depositRequest = Deposit({
            requestId: RequestId({origin: Origin.L1, id: counter++}),
            depositRecipient: depositRecipient,
            tokenAddress: tokenAddress,
            amount: amount,
            timeStamp: block.timestamp,
            ferryTip: ferryTip
        });
        deposits[depositRequest.requestId.id] = depositRequest;

        emit DepositAcceptedIntoQueue(depositRequest.requestId.id, depositRecipient, tokenAddress, amount, ferryTip);

        IERC20(tokenAddress).safeTransferFrom(depositRecipient, address(this), amount);
    }

    function ferry_withdrawal(Withdrawal calldata withdrawal) external payable override nonReentrant whenNotPaused {
        _ferryWithdrawal(withdrawal);
    }

    function ferryWithdrawal(Withdrawal calldata withdrawal) external payable override nonReentrant whenNotPaused {
        _ferryWithdrawal(withdrawal);
    }

    function _ferryWithdrawal(Withdrawal calldata withdrawal) private {
        require(withdrawal.ferryTip <= withdrawal.amount, "Tip exceeds deposited amount");

        bytes32 withdrawalHash = hashWithdrawal(withdrawal);
        require(processedL2Requests[withdrawalHash] == address(0), "Already ferried");

        processedL2Requests[withdrawalHash] = _msgSender();
        uint256 ferriedAmount = withdrawal.amount - withdrawal.ferryTip;

        if (withdrawal.tokenAddress == NATIVE_TOKEN_ADDRESS) {
            require(msg.value > 0, "Native token not sent");
            require(msg.value == ferriedAmount, "Sent amount must exactly match amount without ferryTip");

            emit WithdrawalFerried(
                withdrawal.requestId.id, ferriedAmount, withdrawal.recipient, _msgSender(), withdrawalHash
            );

            payable(withdrawal.recipient).sendValue(ferriedAmount);
            return;
        }

        emit WithdrawalFerried(
            withdrawal.requestId.id, ferriedAmount, withdrawal.recipient, _msgSender(), withdrawalHash
        );

        IERC20(withdrawal.tokenAddress).safeTransferFrom(_msgSender(), withdrawal.recipient, ferriedAmount);
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
        processedL2Requests[withdrawalHash] = CLOSED;

        if (ferryAddress != address(0)) {
            withdrawal.tokenAddress == NATIVE_TOKEN_ADDRESS
                ? _sendNative(ferryAddress, withdrawal.amount)
                : _sendERC20(ferryAddress, withdrawal.tokenAddress, withdrawal.amount);

            emit FerriedWithdrawalClosed(withdrawal.requestId.id, withdrawalHash);
            return;
        }

        uint256 finalWithdrawalAmount = withdrawal.amount - withdrawal.ferryTip;
        if (withdrawal.tokenAddress == NATIVE_TOKEN_ADDRESS) {
            _sendNative(withdrawal.recipient, finalWithdrawalAmount);

            if (withdrawal.ferryTip > 0) {
                _sendNative(_msgSender(), withdrawal.ferryTip);
            }
        } else {
            _sendERC20(withdrawal.recipient, withdrawal.tokenAddress, finalWithdrawalAmount);

            if (withdrawal.ferryTip > 0) {
                _sendERC20(_msgSender(), withdrawal.tokenAddress, withdrawal.ferryTip);
            }
        }

        emit WithdrawalClosed(withdrawal.requestId.id, withdrawalHash);
    }

    function find_l2_batch(uint256 requestId) external view override returns (bytes32) {
        return _findL2Batch(requestId);
    }

    function findL2Batch(uint256 requestId) external view override returns (bytes32) {
        return _findL2Batch(requestId);
    }

    function _findL2Batch(uint256 requestId) private view returns (bytes32) {
        require(requestId <= lastProcessedUpdate_origin_l2, "Invalid request id");

        uint256 rootCount = roots.length;
        require(rootCount > 0, "No roots found yet");

        for (uint256 i = rootCount; i > 0;) {
            bytes32 root = roots[i - 1];
            Range memory range = merkleRootRange[root];

            if (requestId >= range.start && requestId <= range.end) {
                return root;
            }

            unchecked {
                --i;
            }
        }

        revert("Couldn't find batch with request");
    }

    function _verifyRequestProof(uint256 requestId, bytes32 requestHash, bytes32 merkleRoot, bytes32[] calldata proof)
        private
        view
    {
        require(processedL2Requests[requestHash] != CLOSED, "Already processed");

        Range memory range = merkleRootRange[merkleRoot];
        require(range.start > 0 && range.end > 0, "Unknown merkle root");
        require(range.end >= range.start, "Invalid request range");
        require(requestId >= range.start && requestId <= range.end, "Request id out of range");

        uint256 leaveCount = range.end - range.start + 1;
        require(leaveCount <= type(uint32).max, "Range too big");

        uint256 pos = requestId - range.start;
        bytes32 root = LMerkleTree.calculateRoot(requestHash, pos, proof, leaveCount);
        require(root == merkleRoot, "Invalid proof");
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

    // slither-disable-next-line reentrancy-eth
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
        address depositRecipient = originDeposit.depositRecipient;

        if (failedDeposit.ferry != address(0)) {
            depositRecipient = failedDeposit.ferry;
        }

        originDeposit.tokenAddress == NATIVE_TOKEN_ADDRESS
            ? _sendNative(depositRecipient, originDeposit.amount)
            : _sendERC20(depositRecipient, originDeposit.tokenAddress, originDeposit.amount);

        emit FailedDepositResolutionClosed(failedDeposit.requestId.id, failedDeposit.originRequestId, failedDepositHash);
    }

    function update_l1_from_l2(bytes32 merkleRoot, Range calldata range) external override whenNotPaused onlyRole(UPDATER_ROLE) {
        _updateL1FromL2(merkleRoot, range);
    }

    function updateL1FromL2(bytes32 merkleRoot, Range calldata range) external override whenNotPaused onlyRole(UPDATER_ROLE) {
        _updateL1FromL2(merkleRoot, range);
    }

    // TODO:
    // - verify that merkleRoot is correct (passing TaskResponse along with the merkle root?)
    // - verify that range is correct and belongs to particular merkleRoot
    function _updateL1FromL2(bytes32 merkleRoot, Range calldata range /*,TaskResponse calldata response ??? */ )
        private
    {
        require(range.start > 0, "Range id must be greater than zero");
        require(range.start - 1 <= lastProcessedUpdate_origin_l2, "Previous update missing");
        require(range.end >= range.start, "Invalid range");
        require(range.end > lastProcessedUpdate_origin_l2, "Update brings no new data");

        roots.push(merkleRoot);
        merkleRootRange[merkleRoot] = range;
        lastProcessedUpdate_origin_l2 = range.end;

        emit L2UpdateAccepted(merkleRoot, range);
    }

    function _processL2UpdateCancels(Cancel calldata cancel, bytes32 cancelHash) private {
        bool cancelJustified;

        if (cancel.range.end > counter - 1) {
            cancelJustified = true;
        } else {
            L1Update memory pending = getPendingRequests(cancel.range.start, cancel.range.end);
            cancelJustified = cancel.hash != keccak256(abi.encode(pending));
        }

        CancelResolution memory resolution = CancelResolution({
            requestId: RequestId({origin: Origin.L1, id: counter++}),
            l2RequestId: cancel.requestId.id,
            cancelJustified: cancelJustified,
            timeStamp: block.timestamp
        });

        cancelResolutions[resolution.requestId.id] = resolution;

        emit DisputeResolutionAcceptedIntoQueue(resolution.l2RequestId, resolution.cancelJustified, cancelHash);
    }

    function _sendNative(address recipient, uint256 amount) private {
        require(amount > 0, "Amount must be greater than zero");

        emit NativeTokensWithdrawn(recipient, amount);

        Address.sendValue(payable(recipient), amount);
    }

    function _sendERC20(address recipient, address tokenAddress, uint256 amount) private {
        require(amount > 0, "Amount must be greater than zero");

        emit ERC20TokensWithdrawn(recipient, tokenAddress, amount);

        IERC20(tokenAddress).safeTransfer(recipient, amount);
    }

    function getUpdateForL2() external view override returns (L1Update memory) {
        return getPendingRequests(lastProcessedUpdate_origin_l1 + 1, counter - 1);
    }

    function getMerkleRootsLength() external view override returns (uint256) {
        return roots.length;
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

    function getPendingRequests(uint256 start, uint256 end) public view override returns (L1Update memory) {
        L1Update memory result = L1Update({
            chain: chain,
            pendingDeposits: new Deposit[](0),
            pendingCancelResolutions: new CancelResolution[](0)
        });

        if (start == 0 && end == 0) {
            return result;
        }

        uint256 depositCounter = 0;
        uint256 cancelCounter = 0;

        for (uint256 id = start; id <= end;) {
            if (deposits[id].requestId.id > 0) {
                ++depositCounter;
            } else if (cancelResolutions[id].requestId.id > 0) {
                ++cancelCounter;
            } else {
                revert("Invalid range");
            }

            unchecked {
                ++id;
            }
        }

        result.pendingDeposits = new Deposit[](depositCounter);
        result.pendingCancelResolutions = new CancelResolution[](cancelCounter);

        depositCounter = 0;
        cancelCounter = 0;

        for (uint256 id = start; id <= end;) {
            if (deposits[id].requestId.id > 0) {
                result.pendingDeposits[depositCounter++] = deposits[id];
            } else if (cancelResolutions[id].l2RequestId > 0) {
                result.pendingCancelResolutions[cancelCounter++] = cancelResolutions[id];
            } else {
                break;
            }

            unchecked {
                ++id;
            }
        }

        return result;
    }
}

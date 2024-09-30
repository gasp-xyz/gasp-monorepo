// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/Address.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "forge-std/console.sol";

import "@openzeppelin-upgrades/contracts/proxy/utils/Initializable.sol";
import "@openzeppelin-upgrades/contracts/access/OwnableUpgradeable.sol";
import "@eigenlayer/contracts/permissions/Pausable.sol";
import "./RolldownStorage.sol";

contract Rolldown is
    Initializable,
    OwnableUpgradeable,
    Pausable,
    RolldownStorage,
    ReentrancyGuard
{
    using SafeERC20 for IERC20;

    address public constant NATIVE_TOKEN_ADDRESS =
        0x0000000000000000000000000000000000000001;

    address public constant CLOSED = 0x1111111111111111111111111111111111111111;

    // TODO: move to separate modoule/contract
    function calculate_root(bytes32 leave_hash, uint32 leave_idx, bytes32[] calldata proof, uint32 leaves_count) pure public returns (bytes32) {
      uint32 levels = 0;
      uint32 tmp = leaves_count;
      while (tmp > 0) {
        tmp = tmp / 2;
        levels += 1;
      }
      return calculate_root_impl(levels, leave_idx, leave_hash, proof, 0, leaves_count - 1);
    }

    function calculate_root_impl(uint32 level, uint32 pos, bytes32 hash, bytes32[] calldata proofs, uint32 proof_idx, uint32 max_index) pure public returns (bytes32) {
      if (pos % 2 == 0) {
        if (pos == max_index) {
          // promoted node
        }else{
          hash = keccak256(abi.encodePacked(hash, proofs[proof_idx++]));
        }
      } else {
        hash = keccak256(abi.encodePacked(proofs[proof_idx++], hash));
      }

      if (level == 1) {
        return hash;
      }else{
        return calculate_root_impl(level-1, pos/2, hash, proofs, proof_idx, max_index/2);
      }
    }

    function initialize(IPauserRegistry _pauserRegistry, address initialOwner, ChainId chainId, address updater)
        public
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

    function setUpdater(address updater) external whenNotPaused {
      require(msg.sender == updaterAccount, "Only active updater can move rights to a new a account");
      updaterAccount = updater;
      emit NewUpdaterSet(updaterAccount);
    }

    function deposit_native() external payable nonReentrant whenNotPaused {
      deposit_native_with_tip(0);
    }

    function deposit_native(uint256 ferryTip) external payable nonReentrant whenNotPaused {
      deposit_native_with_tip(ferryTip);
    }

    function deposit_native_with_tip(uint256 ferryTip) private {
        require(ferryTip <= msg.value, "Tip exceeds deposited amount");
        require(msg.value > 0, "msg value must be greater that 0");
        address depositRecipient = msg.sender;
        uint amount = msg.value;

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
            depositRequest.requestId.id,
            depositRecipient,
            NATIVE_TOKEN_ADDRESS,
            amount,
            ferryTip
        );
    }

    function deposit(address tokenAddress, uint256 amount) public whenNotPaused nonReentrant  {
        deposit_erc20_with_tip(tokenAddress, amount, 0);
    }

    function deposit(address tokenAddress, uint256 amount, uint256 ferryTip) public whenNotPaused nonReentrant  {
        deposit_erc20_with_tip(tokenAddress, amount, ferryTip);
    }

    function deposit_erc20(address tokenAddress, uint256 amount) public whenNotPaused nonReentrant {
        deposit_erc20_with_tip(tokenAddress, amount, 0);
    }

    function deposit_erc20(address tokenAddress, uint256 amount, uint256 ferryTip) public whenNotPaused nonReentrant {
        deposit_erc20_with_tip(tokenAddress, amount, ferryTip);
    }

    function deposit_erc20_with_tip(address tokenAddress, uint256 amount, uint256 ferryTip) private {
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
        emit DepositAcceptedIntoQueue(
            depositRequest.requestId.id,
            depositRecipient,
            tokenAddress,
            amount,
            ferryTip
        );
    }

    function getUpdateForL2() public view returns (L1Update memory) {
        return
            getPendingRequests(lastProcessedUpdate_origin_l1 + 1, counter - 1);
    }

    function min(uint256 a, uint256 b) private pure returns (uint256) {
        return a < b ? a : b;
    }

    function max(uint256 a, uint256 b) private pure returns (uint256) {
        return a > b ? a : b;
    }

    function ferry_withdrawal(Withdrawal calldata withdrawal) payable public whenNotPaused nonReentrant {
      require(withdrawal.ferryTip <= withdrawal.amount, "Tip exceeds deposited amount");
      uint256 ferriedAmount = withdrawal.amount - withdrawal.ferryTip;
      bytes32 withdrawalHash = keccak256(abi.encode(withdrawal));

      require(processedL2Requests[withdrawalHash] == address(0), "Already ferried");
      processedL2Requests[withdrawalHash] = msg.sender;

      if (withdrawal.tokenAddress == NATIVE_TOKEN_ADDRESS){
        // console.log(msg.value);
        // console.log(ferriedAmount);
        require(msg.value > 0, "Native token not sent");
        require(msg.value == ferriedAmount, "Sent amount should exactly match withdrawal.amount - withdrawal.ferryTip");
        payable(withdrawal.recipient).transfer(ferriedAmount);
        emit WithdrawalFerried(
          withdrawal.requestId.id,
          ferriedAmount,
          withdrawal.recipient,
          msg.sender,
          withdrawalHash
        );
      } else {
        IERC20 token = IERC20(withdrawal.tokenAddress);
        require(token.balanceOf(address(msg.sender)) >= ferriedAmount, "Not enough funds");
        token.transferFrom(msg.sender, withdrawal.recipient, ferriedAmount);
        emit WithdrawalFerried(
          withdrawal.requestId.id,
          ferriedAmount,
          withdrawal.recipient,
          msg.sender,
          withdrawalHash
        );
      }
    }

    function close_withdrawal(Withdrawal calldata withdrawal, bytes32 merkle_root, bytes32[] calldata proof) public whenNotPaused nonReentrant {
        verify_request_proof(withdrawal.requestId.id, keccak256(abi.encode(withdrawal)), merkle_root, proof);
        bytes32 withdrawalHash = keccak256(abi.encode(withdrawal));

        address ferryAddress = processedL2Requests[withdrawalHash];
        bool isFerried = ferryAddress != address(0);
        processedL2Requests[withdrawalHash] = CLOSED;

        if ( !isFerried ){

          if (withdrawal.tokenAddress == NATIVE_TOKEN_ADDRESS){
            send_native_and_emit_event(withdrawal.recipient, withdrawal.amount);
          }
          else {
            send_erc20_and_emit_event(withdrawal.recipient, withdrawal.tokenAddress, withdrawal.amount);
          }

          emit WithdrawalClosed(
            withdrawal.requestId.id,
            keccak256(abi.encode(withdrawal))
          );

        }else{

          if (withdrawal.tokenAddress == NATIVE_TOKEN_ADDRESS){
            send_native_and_emit_event(ferryAddress, withdrawal.amount);
          }
          else {
            send_erc20_and_emit_event(ferryAddress, withdrawal.tokenAddress, withdrawal.amount);
          }

          emit FerriedWithdrawalClosed(
            withdrawal.requestId.id,
            keccak256(abi.encode(withdrawal))
          );

        }


    }

    function find_l2_batch(uint256 requestId) view public returns (Range memory) {
        require(requestId <= lastProcessedUpdate_origin_l2, "Invalid request id");
        if (roots.length == 0) {
            return Range({start: 0, end: 0});
        }

        for (uint256 i = roots.length - 1; i >= 0; i--) {
          if ( requestId >= merkleRootRange[roots[i]].start && requestId <= merkleRootRange[roots[i]].end) {
            return merkleRootRange[roots[i]];
          }
        }

        return Range({start: 0, end: 0});

    }

    function verify_request_proof(uint256 requestId, bytes32 request_hash, bytes32 merkle_root, bytes32[] calldata proof) private view {
        Range memory r = merkleRootRange[merkle_root];
        require(r.start != 0 && r.end != 0, "Unknown merkle root"); 

        require(processedL2Requests[request_hash] != CLOSED, "Already processed");

        uint32 leaves_count = uint32(r.end - r.start + 1);
        uint32 pos = uint32(requestId - r.start);
        require(
          calculate_root(request_hash, pos, proof, leaves_count) == merkle_root,
          "Invalid proof"
        );
    }

    function close_cancel(Cancel calldata cancel, bytes32 merkle_root, bytes32[] calldata proof) public whenNotPaused nonReentrant {
        bytes32 hash = keccak256(abi.encode(cancel));
        verify_request_proof(cancel.requestId.id, hash, merkle_root, proof);
        process_l2_update_cancels(cancel);
        processedL2Requests[hash] = CLOSED;
    }

    function close_deposit_refund(FailedDepositResolution calldata failedDeposit, bytes32 merkle_root, bytes32[] calldata proof) public whenNotPaused nonReentrant {
        bytes32 hash = keccak256(abi.encode(failedDeposit));
        verify_request_proof(failedDeposit.requestId.id, keccak256(abi.encode(failedDeposit)), merkle_root, proof);
        process_l2_update_failed_deposit(failedDeposit);
        processedL2Requests[hash] = CLOSED;
    }

    function process_l2_update_failed_deposit(FailedDepositResolution calldata failedDeposit) private {
        Deposit storage originDeposit = deposits[failedDeposit.originRequestId];
        address recipient = originDeposit.depositRecipient;

        if (failedDeposit.ferry != address(0)) {
          recipient = failedDeposit.ferry;
        }

        if (originDeposit.tokenAddress == NATIVE_TOKEN_ADDRESS) {
              send_native_and_emit_event(recipient, originDeposit.amount);
        } else {
              send_erc20_and_emit_event(recipient, originDeposit.tokenAddress, originDeposit.amount);
        }

        emit FailedDepositResolutionClosed(
          failedDeposit.requestId.id,
          failedDeposit.originRequestId,
          keccak256(abi.encode(failedDeposit))
        );
    }


    // TODO:
    // - verify that merkle_root is correct (passing TaskResponse along with the merkle root?)
    // - verify that range is correct and belongs to particular merkle_root
    function update_l1_from_l2(bytes32 merkle_root, Range calldata range /*,TaskResponse calldata response ??? */) external whenNotPaused {
        require(msg.sender == updaterAccount, "Not the owner");
        require(range.end > lastProcessedUpdate_origin_l2, "Update brings no new data");
        require(range.start > 0 , "range id must be greater than 0");
        require(range.start - 1 <= lastProcessedUpdate_origin_l2, "Previous update missing");
        require(range.end >= range.start, "Invalid range");
        roots.push(merkle_root);
        merkleRootRange[merkle_root] = range;
        lastProcessedUpdate_origin_l2 = range.end;
        emit L2UpdateAccepted(merkle_root, range);
    }

    function process_l2_update_cancels(Cancel calldata cancel) private {
        L1Update memory pending = getPendingRequests(
            cancel.range.start,
            cancel.range.end
        );
        bytes32 correct_hash = keccak256(abi.encode(pending));
        uint256 timeStamp = block.timestamp;

        CancelResolution memory resolution = CancelResolution({
            requestId: RequestId({origin: Origin.L1, id: counter++}),
            l2RequestId: cancel.requestId.id,
            cancelJustified: correct_hash != cancel.hash,
            timeStamp: timeStamp
        });

        cancelResolutions[resolution.requestId.id] = resolution;
        emit DisputeResolutionAcceptedIntoQueue(
            resolution.l2RequestId,
            resolution.cancelJustified
        );
    }

    function send_native_and_emit_event(
        address recipient,
        uint256 amount
    ) private {
        require(payable(address(this)).balance >= amount, "Not enough funds in contract");
        require(amount > 0, "Amount must be greater than zero");
        emit NativeTokensWithdrawn(recipient, amount);
        Address.sendValue(payable(recipient), amount);
    }


    function send_erc20_and_emit_event(
        address recipient,
        address tokenAddress,
        uint256 amount
    ) private {
        IERC20 token = IERC20(tokenAddress);
        require(token.balanceOf(address(this)) >= amount, "Not enough funds in contract");
        require(amount > 0, "Amount must be greater than zero");

        token.safeTransfer(recipient, amount);
        emit ERC20TokensWithdrawn(
          recipient,
          tokenAddress,
          amount
        );
    }

    function getPendingRequests(
        uint256 start,
        uint256 end
    ) public view returns (L1Update memory) {
        L1Update memory result;

        result.chain = chain;
        uint256 depositsCounter = 0;
        uint256 cancelsCounter = 0;

        for (uint256 requestId = start; requestId <= end; requestId++) {
            if (deposits[requestId].requestId.id != 0) {
                depositsCounter++;
            } else if (cancelResolutions[requestId].requestId.id != 0) {
                cancelsCounter++;
            }else {
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
                result.pendingCancelResolutions[
                    cancelsCounter++
                ] = cancelResolutions[requestId];
            } else {
                break;
            }
        }

        return result;
    }
}

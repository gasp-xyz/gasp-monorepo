// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
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
    RolldownStorage
{

    address public constant ETH_TOKEN_ADDRESS =
        0x0000000000000000000000000000000000000001;

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

    function withdraw_pending_eth_to_recipient(uint256 amount, address payable recipient) private whenNotPaused {
        require(amount > 0, "Amount must be greater than zero");
        require(pendingEthWithdrawals[recipient] >= amount, "not enough pending withdraw amount");
        require(payable(address(this)).balance >= amount, "not enough eth funds");

        // important to set this here before .sendValue
        // to prevent reentrancy
        pendingEthWithdrawals[recipient] -= amount;

        emit PendingEthWithdrawn(recipient, amount);

        // send value uses call (gas unbounded)
        // and reverts upon failure
        Address.sendValue(recipient, amount);
    }


    function withdraw_pending_eth(uint256 amount) external whenNotPaused {
      withdraw_pending_eth_to_recipient(amount, payable(msg.sender));
    }

    function deposit_native() external payable whenNotPaused {
        require(msg.value > 0, "msg value must be greater that 0");
        address depositRecipient = msg.sender;
        uint amount = msg.value;

        uint256 timeStamp = block.timestamp;
        Deposit memory depositRequest = Deposit({
            requestId: RequestId({origin: Origin.L1, id: counter++}),
            depositRecipient: depositRecipient,
            tokenAddress: ETH_TOKEN_ADDRESS,
            amount: amount,
            timeStamp: timeStamp
        });
        // Add the new request to the mapping
        deposits[depositRequest.requestId.id] = depositRequest;
        emit DepositAcceptedIntoQueue(
            depositRequest.requestId.id,
            depositRecipient,
            ETH_TOKEN_ADDRESS,
            amount
        );
    }

    function deposit(address tokenAddress, uint256 amount) public whenNotPaused {
        deposit_erc20(tokenAddress, amount);
    }

    function deposit_erc20(address tokenAddress, uint256 amount) public whenNotPaused {
        require(tokenAddress != address(0), "Invalid token address");
        require(amount > 0, "Amount must be greater than zero");
        address depositRecipient = msg.sender;

        IERC20 token = IERC20(tokenAddress);
        require(
            token.transferFrom(msg.sender, address(this), amount),
            "Token transfer failed"
        );

        uint256 timeStamp = block.timestamp;
        Deposit memory depositRequest = Deposit({
            requestId: RequestId({origin: Origin.L1, id: counter++}),
            depositRecipient: depositRecipient,
            tokenAddress: tokenAddress,
            amount: amount,
            timeStamp: timeStamp
        });
        // Add the new request to the mapping
        deposits[depositRequest.requestId.id] = depositRequest;
        emit DepositAcceptedIntoQueue(
            depositRequest.requestId.id,
            depositRecipient,
            tokenAddress,
            amount
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

    function close_withdrawal(Withdrawal calldata withdrawal, bytes32 merkle_root, bytes32[] calldata proof) public {
        Range memory r = merkleRootRange[merkle_root];
        require(r.start != 0 && r.end != 0, "Unknown merkle root"); 

        bytes32 withdrawal_hash = keccak256(abi.encode(withdrawal));
        require(processedL2Requests[withdrawal.requestId.id] == false, "Already processed");

        uint32 leaves_count = uint32(r.end - r.start + 1);
        uint32 pos = uint32(withdrawal.requestId.id - r.start);
        require(
          calculate_root(withdrawal_hash, pos, proof, leaves_count) == merkle_root,
          "Invalid proof"
        );
        process_l2_update_withdrawal(withdrawal);
        processedL2Requests[withdrawal.requestId.id] = true;
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

    function close_cancel(Cancel calldata cancel, bytes32 merkle_root, bytes32[] calldata proof) public {
        Range memory r = merkleRootRange[merkle_root];
        require(r.start != 0 && r.end != 0, "Unknown merkle root"); 

        bytes32 cancel_hash = keccak256(abi.encode(cancel));
        require(processedL2Requests[cancel.requestId.id] == false, "Already processed");
        uint32 leaves_count = uint32(r.end - r.start + 1);
        uint32 pos = uint32(r.start - cancel.requestId.id);
        require(
          calculate_root(cancel_hash, pos, proof, leaves_count) == merkle_root,
          "Invalid proof"
        );
        process_l2_update_cancels(cancel);
        processedL2Requests[cancel.requestId.id] = true;
    }

    // TODO:
    // - verify that merkle_root is correct (passing TaskResponse along with the merkle root?)
    // - verify that range is correct and belongs to particular merkle_root
    function update_l1_from_l2(bytes32 merkle_root, Range calldata range /*,TaskResponse calldata response ??? */) external whenNotPaused {
        require(msg.sender == updaterAccount, "Not the owner");
        require(range.end > lastProcessedUpdate_origin_l2, "Update brings no new data");
        require(range.start - 1 <= lastProcessedUpdate_origin_l2, "Previous update missing");
        require(range.end >= range.start, "Invalid range");
        roots.push(merkle_root);
        merkleRootRange[merkle_root] = range;
        lastProcessedUpdate_origin_l2 = range.end;
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

    function process_l2_update_withdrawal( Withdrawal calldata withdrawal) private {
        if (withdrawal.tokenAddress == ETH_TOKEN_ADDRESS){
            process_eth_withdrawal(withdrawal);
        }
        else {
            process_erc20_withdrawal(withdrawal);
        }
    }

    function process_eth_withdrawal( Withdrawal calldata withdrawal) private {
        bool enought_funds_in_contract = payable(address(this)).balance >= withdrawal.amount;
        bool is_account = withdrawal.withdrawalRecipient.code.length == 0;
        bool status = enought_funds_in_contract && is_account;
        uint256 timeStamp = block.timestamp;

        WithdrawalResolution memory resolution = WithdrawalResolution({
            requestId: RequestId({origin: Origin.L1, id: counter++}),
            l2RequestId: withdrawal.requestId.id,
            status: status,
            timeStamp: timeStamp
        });

        withdrawalResolutions[resolution.requestId.id] = resolution;
        emit WithdrawalResolutionAcceptedIntoQueue(
            resolution.requestId.id,
            status
        );

        if (status) {
            pendingEthWithdrawals[withdrawal.withdrawalRecipient] += withdrawal.amount;
            emit EthWithdrawPending(
                withdrawal.withdrawalRecipient,
                pendingEthWithdrawals[withdrawal.withdrawalRecipient]
            );
            withdraw_pending_eth_to_recipient(withdrawal.amount, payable(withdrawal.withdrawalRecipient));
        }
    }

    function process_erc20_withdrawal( Withdrawal calldata withdrawal) private {
        IERC20 token = IERC20(withdrawal.tokenAddress);
        bool status = token.balanceOf(address(this)) >= withdrawal.amount;
        uint256 timeStamp = block.timestamp;

        WithdrawalResolution memory resolution = WithdrawalResolution({
            requestId: RequestId({origin: Origin.L1, id: counter++}),
            l2RequestId: withdrawal.requestId.id,
            status: status,
            timeStamp: timeStamp
        });

        withdrawalResolutions[resolution.requestId.id] = resolution;
        emit WithdrawalResolutionAcceptedIntoQueue(
            resolution.requestId.id,
            status
        );

        if (status) {
            token.transfer(withdrawal.withdrawalRecipient, withdrawal.amount);
            emit FundsWithdrawn(
                withdrawal.withdrawalRecipient,
                withdrawal.tokenAddress,
                withdrawal.amount
            );
        }
    }

    function getPendingRequests(
        uint256 start,
        uint256 end
    ) public view returns (L1Update memory) {
        L1Update memory result;

        result.chain = chain;
        uint256 depositsCounter = 0;
        uint256 withdrawalsCounter = 0;
        uint256 cancelsCounter = 0;
        uint256 updatesToBeRemovedCounter = 0;

        for (uint256 requestId = start; requestId <= end; requestId++) {
            if (deposits[requestId].requestId.id != 0) {
                depositsCounter++;
            } else if (l2UpdatesToRemove[requestId].requestId.id != 0) {
                updatesToBeRemovedCounter++;
            } else if (withdrawalResolutions[requestId].requestId.id != 0) {
                withdrawalsCounter++;
            } else if (cancelResolutions[requestId].requestId.id != 0) {
                cancelsCounter++;
            }
        }

        result.pendingDeposits = new Deposit[](depositsCounter);
        result.pendingCancelResolutions = new CancelResolution[](cancelsCounter);
        result.pendingWithdrawalResolutions = new WithdrawalResolution[](
            withdrawalsCounter
        );
        result.pendingL2UpdatesToRemove = new L2UpdatesToRemove[](
            updatesToBeRemovedCounter
        );

        withdrawalsCounter = 0;
        depositsCounter = 0;
        cancelsCounter = 0;
        updatesToBeRemovedCounter = 0;

        for (uint256 requestId = start; requestId <= end; requestId++) {
            if (deposits[requestId].requestId.id > 0) {
                result.pendingDeposits[depositsCounter++] = deposits[requestId];
            } else if (withdrawalResolutions[requestId].requestId.id > 0) {
                result.pendingWithdrawalResolutions[
                    withdrawalsCounter++
                ] = withdrawalResolutions[requestId];
            } else if (l2UpdatesToRemove[requestId].requestId.id > 0) {
                result.pendingL2UpdatesToRemove[
                    updatesToBeRemovedCounter++
                ] = l2UpdatesToRemove[requestId];
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

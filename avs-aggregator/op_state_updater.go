package aggregator

import (
	"context"
	"math/big"
	"sort"
	"bytes"
	"time"

	"github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/core/chainio"
	"github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/types"
	blsagg "github.com/Layr-Labs/eigensdk-go/services/bls_aggregation"
	"github.com/Layr-Labs/eigensdk-go/services/avsregistry"
	sdktypes "github.com/Layr-Labs/eigensdk-go/types"
)

type OpStateUpdater struct {
	logger       logging.logger
	checkpointedAvsQuorumStakes map[sdktypes.QuorumNum]sdktypes.StakeAmount
	checkpointedAvsOpState map[sdktypes.OperatorId]types.OperatorAvsState
	currentOpState map[sdktypes.OperatorId]types.OperatorAvsState
	checkpointedBlock sdktypes.BlockNum
	quorumDiff map[sdktypes.QuorumNum]types.QuorumStakeDiff
	operatorIdsToBeUpdated map[types.OperatorId]bool
	quorumsToBeUpdated map[sdktypes.QuorumNum]bool
	quorumPosPercThreshold uint8
	quorumNegPercThreshold uint8
	updateFullQuorumThresholdPerc uint8
	ethRpc           *chainio.EthRpc
	// aggregation related fields
	avsRegistryService   *avsregistry.AvsRegistryServiceChainCaller

	// syncPaused bool
	// trackedQuorumNumbers asdf
	// checkpointedStragiesPerTrackedQuorum asdf

}

// type WatchedEvent struct {
// 	logger       logging.logger
// 	checkpointedAvsOpState asdf
// 	currentOpState asdf
// 	checkpointedBlock asdf
// 	// syncPaused bool
// 	// trackedQuorumNumbers asdf
// 	// checkpointedStragiesPerTrackedQuorum asdf

// }

func NewOpStakeUpdater(logger logging.Logger, ethRpc *chainio.EthRpc, avsRegistryService   *avsregistry.AvsRegistryServiceChainCaller) (*StakeUpdate, error) {
	return &StakeUpdate{
		logger:       logger,
		ethRpc:                  ethRpc,
		avsRegistryService:   avsRegistryService,
		ethRpc:       ethRpc,
	}, nil
}


func getEventID(c *bind.BoundContract, eventName string) common.Hash {
    return c.abi.Events[eventName].ID
}

func (osu *OpStateUpdater) startAsyncOpStateUpdater(ctx context.Context, sendNewOpTaskC chan sendNewOpTaskType) error {
	agg.logger.Infof("Starting aggregator.")
	agg.logger.Infof("Starting aggregator rpc server.")

	// TODO set a starting block and process everything from that point on!!
	// Then set this reference block to the checkpointedBlock

	lastCompletedOpTaskCreatedBlock, err := osu.ethRpc.AvsReader.LastCompletedOpTaskCreatedBlock(context.Background());
	if err != nil {
		osu.logger.Error("Aggregator failed to get LastCompletedOpTaskCreatedBlock", "err", err)
		return err
	}

	if lastCompletedOpTaskCreatedBlock == 0{
		osu.triggerOpStateUpdate = true
		osu.resetTrackingQuorums = true
	} else {
		osu.checkpointedBlock = lastCompletedOpTaskCreatedBlock
	}

	// So that on local testnet the finalizer can register 
	// Maybe first wait here so that there is atleast 1 operator that is registered
	time.Sleep(60*seconds);

	triggerEvents := map[common.Address][]common.Hash{
        delegationManagerContractAddress: {getEventID(boundDelegationManagerContract.contract, "OperatorSharesIncreased"), getEventID(boundDelegationManagerContract.contract, "OperatorSharesDecreased")}, // Contract 1
        stakeRegistryContractAddress: {getEventID(boundStakeRegistryContract.contract, "OperatorStakeUpdate"), getEventID(boundStakeRegistryContract.contract, "StrategyMultiplierUpdated"), getEventID(boundStakeRegistryContract.contract, "MinimumStakeForQuorumUpdated") },                 // Contract 2
        taskManagerContractAddress: {getEventID(boundTaskManagerContract.contract, "PauseTrackingOpState"), /*getEventID(boundTaskManagerContract.contract, "ResumeTrackingOpState"),*/ getEventID(boundTaskManagerContract.contract, "OpTaskCompleted") },                 // Contract 3
    }

	// Prepare the subscription
	query := ethereum.FilterQuery{
		FromBlock: ,
		Addresses: []common.Address{delegationManagerContractAddress, stakeRegistryContractAddress, taskManagerContractAddress},
		Topics:    [][]common.Hash{
			[]common.Hash{
				getEventID(boundDelegationManagerContract.contract, "OperatorSharesIncreased"), getEventID(boundDelegationManagerContract.contract, "OperatorSharesDecreased"),
				getEventID(boundStakeRegistryContract.contract, "OperatorStakeUpdate"), getEventID(boundStakeRegistryContract.contract, "StrategyMultiplierUpdated"), getEventID(boundStakeRegistryContract.contract, "MinimumStakeForQuorumUpdated"),
				getEventID(boundTaskManagerContract.contract, "PauseTrackingOpState"), /*getEventID(boundTaskManagerContract.contract, "ResumeTrackingOpState"),*/ /*getEventID(boundTaskManagerContract.contract, "OpTaskCompleted")*/,
			}
		},
	}

	for {

		for {
			switch{
			case osu.paused == true: {
				eventC := make(chan taskmanager.ContractFinalizerTaskManagerResumeTrackingOpState)
				sub, err := osu.ethRpc.AvsSubscriber.SubscribeToResumeTrackingOpState(eventC)
				if err != nil {
					return fmt.Errorf("failed to subscribe to logs: %v", err)
				}
				defer sub.Unsubscribe();

				for {
					select {
					case <-ctx.Done():
						return ctx.Err();
					case err := <-sub.Err():
						osu.logger.Fatalf("Subscription error: %v", err);
					case event := <-eventC:
						osu.logger.Info("Received resume event: %v", eventC);
						if event.ResetTrackingQuorums{
							osu.resetTrackingQuorums = true;
							osu.triggerOpStateUpdate = true;
						}
						osu.paused = false;
						break;
					}
				}
				sub.Unsubscribe();
			}
			case osu.triggerOpStateUpdate: {
				
				// Figure out if updating select operators only or all of them...
				// Then update, then proceed
				// What if checkpointedBlock is 0 here? Update everything and proceed
				// If all avs states are null then pause

				if osu.resetTrackingQuorums {
					err := osu.UpdateStakeFull();
					if err != nil {
						osu.logger.Error("Failed to UpdateStakeFull")
						osu.paused = true;
						osu.resetTrackingQuorums = false;
						osu.triggerOpStateUpdate = false;
						continue;
					}
					osu.resetTrackingQuorums = false;
				} else {
					err := osu.UpdateStake();
					if err != nil {
						osu.logger.Error("Failed to UpdateStake")
						osu.paused = true;
						osu.resetTrackingQuorums = false;
						osu.triggerOpStateUpdate = false;
						continue;
					}
				}

				osu.operatorIdsToBeUpdated = make(map[types.OperatorId]bool)

				sendNewOpTaskReturnC := make(chan SendNewOpTaskReturn)
				var sendNewOpTaskReturn SendNewOpTaskReturn
				// send the task to the goroutine processing this task
				// and return the error (if any) returned by the signature verification routine
				select {
				// Here we do not require to use select since this function is synchronous and is run as a go routine only once, but we will in case that changes 
				// we need to send this as part of select because if the goroutine is processing another triggerOpStateUpdate
				// and cannot receive this one, we want the context to be able to cancel the request
				case sendNewOpTaskC <- types.sendNewOpTaskType{
					SendNewOpTaskReturnC: sendNewOpTaskReturnC,
				}:
					// note that we need to wait synchronously here for this response because we want to get the result
					sendNewOpTaskReturn <- SendNewOpTaskReturnC
				case <-ctx.Done():
					return ctx.Err()
				}

				if sendNewOpTaskReturn.SendNewOpTaskError != nil {
					osu.paused = true;
				} else {
					eventC := make(chan taskmanager.ContractFinalizerTaskManagerOpTaskCompleted)
					sub, err := osu.ethRpc.AvsSubscriber.SubscribeToOpTaskCompleted(uint64(sendNewOpTaskReturn.OpTask.TaskCreatedBlock), eventC)
					if err != nil {
						return fmt.Errorf("failed to subscribe to logs: %v", err)
					}
					defer sub.Unsubscribe();

					timer := time.NewTimer(time.Hour)
					defer timer.Stop()

					for {
						select {
						case <-ctx.Done():
							return ctx.Err();
						case err := <-sub.Err():
							osu.logger.Fatalf("Subscription error: %v", err);
						case <- timer.C:
							osu.paused = true;
							break;
						case event := <-eventC:
							{
								osu.logger.Info("Received OpTaskCompleted event: %v", eventC);

								if sendNewOpTaskReturn.OpTask.TaskCreatedBlock < event.TaskIndex {
									osu.logger.Error("Failed to get the OpTaskCompleted event", "TaskIndex", sendNewOpTaskReturn.OpTask.TaskCreatedBlock)
									osu.paused = true;
									break;
								}

								// This branch is to account for the case where
								// a task is completed in a block and another task is created
								// in the same block and then that one is also completed in the same block
								if sendNewOpTaskReturn.OpTask.TaskCreatedBlock > event.TaskIndex {
									continue;
								}

								osu.logger.Info("Got the expected OpTaskCompleted event", "TaskIndex", sendNewOpTaskReturn.OpTask.TaskCreatedBlock)

								osu.checkpointedBlock = sendNewOpTaskReturn.OpTask.TaskCreatedBlock

								// Get checkpointed avs state here
								// Get the eigen state here
								// applyDiff
								// Check for threshold
								// If still crosses threshold pause
								// If all avs states are null then pause?


								osu.checkpointedAvsOpState, err := osu.avsRegistryService.GetOperatorsAvsStateAtBlock(context.Background(), types.TRACKED_QUORUM_NUMBERS, osu.checkpointedBlock)
								if err != nil {
									// TODO: how should we handle such an error?
									a.logger.Fatal("AggregatorService failed to get operators state from avs registry", "err", err, "blockNumber", osu.checkpointedBlock)
								}

								osu.currentOpState = osu.checkpointedAvsOpState;

								keys := make([]common.Address, 0, len(osu.checkpointedAvsOpState))

								// Iterate over the map and collect keys
								for key := range checkpointedAvsOpState {
									keys = append(keys, key.operator)
								}


								opStateUpdate, err := osu.ethRpc.AvsReader.GetTypedOperatorsStakesForQuorumAtBlock(context.Background(), registryCoordinatorAddr, types.TRACKED_QUORUM_NUMBERS, keys, osu.checkpointedBlock);
								if err != nil {
									a.logger.Fatal("Aggregator failed to get opStateUpdate", "err", err)
								}

								osu.applyOpStateUpdate(opStateUpdate);

								break;
							}
						}
					}
					sub.Unsubscribe();
				}

				osu.triggerOpStateUpdate = false;
				// clear(osu.quorumsToBeUpdated);

			}
			default: break;
			}
		}

		// TODO make this subscription from the point of the checkpointed block!!
		// Subscribe
		logs := make(chan ethereum.Log)
		iquery := query
		iquery.FromBlock =  
		sub, err := osu.ethRpc.Clients.EthWsClient.SubscribeFilterLogs(context.Background(), query, logs)
		if err != nil {
			return fmt.Errorf("failed to subscribe to logs: %v", err)
		}
		defer sub.Unsubscribe();

		// watch the subscription
		for {
			select {
			case <-ctx.Done():
				return ctx.Err();
			case err := <-sub.Err():
				osu.logger.Fatalf("Subscription error: %v", err)
			case vLog := <-logs:
				fmt.Printf("Received log: %+v\n", vLog)
				switch{ 
					case vLog.Address == delegationManagerContractAddress && vLog.Topics[0] == getEventID(boundDelegationManagerContract.contract, "OperatorSharesIncreased"): {
						fmt.Printf("Event %s from contract %s\n", "OperatorSharesIncreased", vLog.Address.Hex())
						// Process the log here based on event signature and ABI
						ContractDelegationManagerOperatorSharesIncreased, err := delegationManagerContract.ContractDelegationManagerFilterer.ParseOperatorSharesIncreased(vLog);
						if err != nil {
							osu.logger.Error("Failed to ParseOperatorSharesIncreased - Pasuing trackingOpState", "err", err)
							osu.paused = true;
							break; 
						}
						osu.processOpDelegationStateChange(ContractDelegationManagerOperatorSharesIncreased.Operator, uint32(vLog.BlockNumber.Int64()));

						if osu.paused || osu.triggerOpStateUpdate {
							break;
						}
					}
					case vLog.Address == delegationManagerContractAddress && vLog.Topics[0] == getEventID(boundDelegationManagerContract.contract, "OperatorSharesDecreased"): {
						fmt.Printf("Event %s from contract %s\n", "OperatorSharesDecreased", vLog.Address.Hex())
						// Process the log here based on event signature and ABI
						ContractDelegationManagerOperatorSharesDecreased, err := delegationManagerContract.ContractDelegationManagerFilterer.ParseOperatorSharesDecreased(vLog);
						if err != nil {
							osu.logger.Error("Failed to ParseOperatorSharesDecreased - Pasuing trackingOpState", "err", err)
							osu.paused = true;
							break; 
						}
						osu.processOpDelegationStateChange(ContractDelegationManagerOperatorSharesIncreased.Operator, uint32(vLog.BlockNumber.Int64()));

						if osu.paused || osu.triggerOpStateUpdate {
							break;
						}
					}
					case vLog.Address == stakeRegistryContractAddress && vLog.Topics[0] == getEventID(boundStakeRegistryContract.contract, "OperatorStakeUpdate"): {
						fmt.Printf("Event %s from contract %s\n", "OperatorStakeUpdate", vLog.Address.Hex())
						// Process the log here based on event signature and ABI
						ContractStakeRegistryOperatorStakeUpdate, err := stakeRegistryContract.ContractStakeRegistryFilterer.ParseOperatorStakeUpdate(vLog);
						if err != nil {
							osu.logger.Error("Failed to ParseOperatorStakeUpdate - Pasuing trackingOpState", "err", err)
							osu.paused = true;
							break; 
						}
						osu.processOpStakeRegistryStateChange(ContractStakeRegistryOperatorStakeUpdate.OperatorId, ContractStakeRegistryOperatorStakeUpdate.QuorumNumber, ContractStakeRegistryOperatorStakeUpdate.Stake);

						if osu.paused || osu.triggerOpStateUpdate {
							break;
						}
					}
					case vLog.Address == stakeRegistryContractAddress && vLog.Topics[0] == getEventID(boundStakeRegistryContract.contract, "StrategyMultiplierUpdated"): {
						fmt.Printf("Event %s from contract %s\n", "StrategyMultiplierUpdated", vLog.Address.Hex())
						// Process the log here based on event signature and ABI
						ContractStakeRegistryStrategyMultiplierUpdated, err := stakeRegistryContract.ContractStakeRegistryFilterer.ParseStrategyMultiplierUpdated(vLog);
						if err != nil {
							osu.logger.Error("Failed to ParseStrategyMultiplierUpdated - Pasuing trackingOpState", "err", err)
							osu.paused = true;
							break; 
						}
						osu.logger.Info("Pausing trackingOpState upon event trigger", "trigger", ContractStakeRegistryStrategyMultiplierUpdated)
						osu.paused = true;
						break; 
					}
					case vLog.Address == stakeRegistryContractAddress && vLog.Topics[0] == getEventID(boundStakeRegistryContract.contract, "MinimumStakeForQuorumUpdated"): {
						fmt.Printf("Event %s from contract %s\n", "MinimumStakeForQuorumUpdated", vLog.Address.Hex())
						// Process the log here based on event signature and ABI
						ContractStakeRegistryMinimumStakeForQuorumUpdated, err := stakeRegistryContract.ContractStakeRegistryFilterer.ParseMinimumStakeForQuorumUpdated(vLog);
						if err != nil {
							osu.logger.Error("Failed to ParseMinimumStakeForQuorumUpdated - Pasuing trackingOpState", "err", err)
							osu.paused = true;
							break; 
						}
						osu.logger.Info("Pausing trackingOpState upon event trigger", "trigger", ContractStakeRegistryMinimumStakeForQuorumUpdated)
						osu.paused = true;
						break; 
					}
					case vLog.Address == taskManagerContractAddress && vLog.Topics[0] == getEventID(boundTaskManagerContract.contract, "PauseTrackingOpState"): {
						fmt.Printf("Event %s from contract %s\n", "PauseTrackingOpState", vLog.Address.Hex())
						// Process the log here based on event signature and ABI
						ContractFinalizerTaskManagerPauseTrackingOpState, err := taskManagerContract.ContractFinalizerTaskManagerFilterer.ParsePauseTrackingOpState(vLog);
						if err != nil {
							osu.logger.Error("Failed to ParsePauseTrackingOpState - Pasuing trackingOpState", "err", err)
							osu.paused = true;
							break; 
						}
						osu.logger.Info("Pausing trackingOpState upon event trigger", "trigger", ContractFinalizerTaskManagerPauseTrackingOpState)
						osu.paused = true;
						break; 
					}
				}
			}
		}
		sub.Unsubscribe()
	}


}


if last_completed_op_task_created_block == 0
then wait for op task = true
while {
	set_states
	while {
		get events{
		case 1/2: applyOpStateDiff, check_quorum_stake_diff
			if go then set wait for opTask to true and break
			case 2

		case opTaskCompleted:
			break
		case pause:
			paused = true
			break
		}
	}
	unsub here
	// This waiting is done in set_state
	// if paused{
	// 	loop watch for resume event{
	// 		case got it

	// 	}
	// }
}


func (osu *OpStateUpdater) processOpStakeRegistryStateChange(operatorId sdktypes.OperatorId, quorumNumber sdktypes.BlockNum, stake sdktypes.StakeAmount) error {

	osu.currentOpState[operatorId].StakePerQuorum[quorum] = stake;
	osu.operatorIdsToBeUpdated[operatorId] = true;
	osu.checkQuorumThresholds();
}

func (osu *OpStateUpdater) processOpDelegationStateChange(operator common.Address, blockNumber sdktypes.BlockNum) error {
	
	operatorId, err := osu.ethRpc.Clients.AvsRegistryChainReader.GetOperatorId(&bind.CallOpts{}, operator);
	if err != nil {
		a.logger.Fatal("Aggregator failed to get operatorId", "err", err)
	}
	if bytes.Equal(operatorId, ZERO_OPERATOR_ID){
		return nil
	}
	if currentOpState, ok := osu.currentOpState[operatorId]; ok{
		opStateUpdate, err := osu.ethRpc.AvsReader.GetTypedOperatorsStakesForQuorumAtBlock(context.Background(), registryCoordinatorAddr, types.TRACKED_QUORUM_NUMBERS, keys, blockNumber);
		if err != nil {
			a.logger.Fatal("Aggregator failed to get opStateUpdate", "err", err)
		}
		osu.applyOpStateUpdate(opStateUpdate);

		osu.operatorIdsToBeUpdated[operatorId] = true;
		osu.checkQuorumThresholds();
	}

	return nil
}

func (osu *OpStateUpdater) applyOpStateUpdate(opStateUpdate map[sdktypes.OperatorId]types.OperatorAvsState) error {
	for operatorId, opUpdate := range opStateUpdate{
		if currentOpState, ok := osu.currentOpState[operatorId]; ok {
			currentOpState.StakePerQuorum = opUpdate.StakePerQuorum
			osu.currentOpState[operatorId] = currentOpState
		} else {
			osu.currentOpState[operatorId] = types.OperatorAvsState{
				OperatorId:     operatorId,
				StakePerQuorum: opUpdate.StakePerQuorum,
				BlockNumber:    opUpdate.BlockNumber,
			}
		}
	}
	return nil
}

func (osu *OpStateUpdater) checkQuorumThresholds() error {
	// TODO
	// Maybe just pass in the quorumDiff rather than update state...
	osu.makeQuorumsStakeDiff();
	osu.checkQuorumsStakeDiff();
}

func (osu *OpStateUpdater) makeQuorumsStakeDiff() error {

	osu.quorumDiff = make(map[sdktypes.QuorumNum]types.QuorumStakeDiff)

	for operatorId, opState := range osu.checkpointedAvsOpState {
		for _, quorum := range types.TRACKED_QUORUM_NUMBERS {
			updatedStake = osu.currentOpState[operatorId].StakePerQuorum[quorum];
			prevStake = osu.checkpointedAvsOpState[operatorId].StakePerQuorum[quorum];
			if prevStake > updatedStake{
				diff := big.NewInt(0).Sub(prevStake, updatedStake);
				osu.quorumDiff[quorum].negDiff.Add(osu.quorumDiff[quorum].negDiff, diff)
			}
		}
	}

	for operatorId, opState := range osu.currentOpState {
		for _, quorum := range types.TRACKED_QUORUM_NUMBERS {
			updatedStake = osu.currentOpState[operatorId].StakePerQuorum[quorum];
			prevStake = osu.checkpointedAvsOpState[operatorId].StakePerQuorum[quorum];
			if prevStake < updatedStake{
				diff := big.NewInt(0).Sub(updatedStake, prevStake);
				osu.quorumDiff[quorum].posDiff.Add(osu.quorumDiff[quorum].posDiff, diff)
			}
		}
	}

}

func (osu *OpStateUpdater) checkQuorumsStakeDiff() error {

	osu.quorumsToBeUpdated = make(map[sdktypes.QuorumNum]bool)

	for i, qn := range types.TRACKED_QUORUM_NUMBERS {
		checkpointedQuorumStake = osu.checkpointedAvsQuorumStakes[qn];
		if checkpointedQuorumStake !=0 {
			if osu.quorumDiff[qn].negDiff * 100 > checkpointedQuorumStake * osu.quorumNegPercThreshold{
				osu.quorumsToBeUpdated[qn] = true;
				osu.triggerOpStateUpdate = true;
			}
			if osu.quorumDiff[qn].posDiff * 100 > checkpointedQuorumStake * osu.quorumPosPercThreshold{
				osu.quorumsToBeUpdated[qn] = true;
				osu.triggerOpStateUpdate = true;
			}
		} else {
			if osu.quorumDiff[qn].posDiff > 0{
				osu.quorumsToBeUpdated[qn] = true;
				osu.triggerOpStateUpdate = true;
			}
		}
	}
}

// set_states(){
// 	** probably should loop this
// 	loop {
// 		ctr = 0
// 		switch {
// 		case paused: { ;;; continue}
// 		case wait for op task: {;; ctr++; continue}
// 		case _: break;
// 		}
// 	}
// 	if paused wait here for the resume event
// 	check the last_completed_op_task_created_block
// 	if 0
// 	if ctr here is 1 then set paused to true and then wait for resume event
// 	then wait here and watch for optask completed event and get last_completed_op_task_created_block
// 	so now with last_completed_op_task_created_block
// 	query the operators_avs_state 
// 	copy the operators_avs_state to the current_op_state
// 	query the operatorsStakesForQuorum
// 	then applyOpStateDiff upon the current_op_state (maybe while considering the checkpointed_avs_op_state) and while updating the number of altered operators
// 	and the quorums stake diff

	
// }

// applyOpStateDiff(){


// 	should be followed by check_quorum_stake_diff
// }

func (osu *OpStateUpdater) UpdateStake() error {

	// Optimized for a single quorum for now
	// Doesn't consider untracked quorums
	if len(osu.operatorIdsToBeUpdated) * 100 < osu.currentOpState * osu.updateFullQuorumThresholdPerc{
		keys := make([]string, 0, len(osu.operatorIdsToBeUpdated))

		for key := range osu.operatorIdsToBeUpdated {
			keys = append(keys, key)
		}
		return osu.UpdateStateForOperatorIds(keys)
	} else {
		keys := make([]string, 0, len(osu.quorumsToBeUpdated))

		for key := range osu.quorumsToBeUpdated {
			keys = append(keys, key)
		}
		return osu.UpdateStateForEntireQuorums(keys)
	}

	return nil
}

func (osu *OpStateUpdater) UpdateStakeFull() error {
	return osu.UpdateStateForEntireQuorums(types.TRACKED_QUORUM_NUMBERS)
}

func (osu *OpStateUpdater) UpdateStateForEntireQuorums(quorumNums sdktypes.QuorumNums) error {
	osu.logger.Info("Running Operator Stake Update check")

	currentBlock, err := osu.ethRpc.Clients.EthHttpClient.BlockNumber(context.Background())
	if err != nil {
		osu.logger.Error("Cannot get current block number", "err", err)
		return err
	}

	var operatorAddresses[][]common.Address{}
	for i, qn := range types.TRACKED_QUORUM_NUMBERS {
		operatorIds, err := osu.ethRpc.Clients.AvsRegistryChainReader.GetOperatorIdList(&bind.CallOpts{}, qn, uint32(currentBlock))
		if err != nil {
			osu.logger.Error("Cannot get current operator list", "err", err)
			return err
		}

		operatorAdrresses, err := osu.ethRpc.AvsReader.GetOperatorsFromIds(&bind.CallOpts{}, operatorIds)
		if err != nil {
			osu.logger.Error("Cannot get operator addresses from operator ids")
			return err
		}

		sort.Slice(operatorAdrresses, func(i, j int) bool {
			a := big.NewInt(0).SetBytes(operatorAdrresses[i][:])
			b := big.NewInt(0).SetBytes(operatorAdrresses[j][:])
			return a.Cmp(b) < 0
		})

		operatorAddresses.append(operatorAddresses, operatorAdrresses)

	}

	_, err = osu.ethRpc.Clients.AvsRegistryChainWriter.UpdateStakesOfEntireOperatorSetForQuorums(context.Background(), operatorAddresses, types.TRACKED_QUORUM_NUMBERS)
	if err != nil {
		osu.logger.Error("Cannot update stakes", "err", err)
		return err
	}

	osu.logger.Info("Operator stakes update successfull", "addresses", operatorAdrresses)

	return nil
}

func (osu *OpStateUpdater) UpdateStateForOperatorIds(operatorIds []sdktypes.OperatorId) error {
	osu.logger.Info("Running Operator Stake Update check")

	currentBlock, err := osu.ethRpc.Clients.EthHttpClient.BlockNumber(context.Background())
	if err != nil {
		osu.logger.Error("Cannot get current block number", "err", err)
		return err
	}

	operatorAdrresses, err := osu.ethRpc.AvsReader.GetOperatorsFromIds(&bind.CallOpts{}, operatorIds)
	if err != nil {
		osu.logger.Error("Cannot get operator addresses from operator ids")
		return err
	}

	_, err = osu.ethRpc.Clients.AvsRegistryChainWriter.UpdateStakesOfOperatorSubsetForAllQuorums(context.Background(), operatorAddresses)
	if err != nil {
		osu.logger.Error("Cannot update stakes", "err", err)
		return err
	}

	osu.logger.Info("Operator stakes update successfull", "addresses", operatorAdrresses)

	return nil
}
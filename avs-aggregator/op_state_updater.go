package aggregator

import (
	"bytes"
	"context"
	"fmt"
	"math/big"
	"sort"
	"time"

	"github.com/Layr-Labs/eigensdk-go/logging"
	ethereum "github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/core/chainio"
	"github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/types"

	// blsagg "github.com/Layr-Labs/eigensdk-go/services/bls_aggregation"
	"github.com/Layr-Labs/eigensdk-go/services/avsregistry"
	sdktypes "github.com/Layr-Labs/eigensdk-go/types"
	ethtypes "github.com/ethereum/go-ethereum/core/types"

	delegationManager "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/DelegationManager"
	taskmanager "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/FinalizerTaskManager"
	stakeRegistry "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/StakeRegistry"
	// operatorStateRetrieverExtended "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/OperatorStateRetrieverExtended"
)

type OpStateUpdater struct {
	logger                        logging.Logger
	checkpointedAvsQuorumStakes   map[sdktypes.QuorumNum]sdktypes.StakeAmount
	checkpointedAvsOpState        map[sdktypes.OperatorId]types.OperatorAvsState
	currentOpState                map[sdktypes.OperatorId]types.OperatorAvsState
	checkpointedBlock             sdktypes.BlockNum
	atBlock                       sdktypes.BlockNum
	quorumDiff                    map[sdktypes.QuorumNum]types.QuorumStakeDiff
	operatorIdsToBeUpdated        map[sdktypes.OperatorId]bool
	quorumsToBeUpdated            map[sdktypes.QuorumNum]bool
	quorumPosPercThreshold        uint8
	quorumNegPercThreshold        uint8
	updateFullQuorumThresholdPerc uint8
	triggerOpStateUpdate          bool
	resetTrackedQuorums           bool
	paused                        bool
	pauseReasonV                  string
	lastOpStateUpdateTime         time.Time
	triggerOpStateUpdateWindow    time.Duration
	errorC                        chan error
	ethRpc                        *chainio.EthRpc
	avsRegistryService            *avsregistry.AvsRegistryServiceChainCaller
}

func NewOpStateUpdater(logger logging.Logger, ethRpc *chainio.EthRpc, avsRegistryService *avsregistry.AvsRegistryServiceChainCaller, minOpUpdateInterval int) (*OpStateUpdater, error) {
	return &OpStateUpdater{
		logger:                        logger,
		ethRpc:                        ethRpc,
		avsRegistryService:            avsRegistryService,
		checkpointedAvsQuorumStakes:   make(map[sdktypes.QuorumNum]sdktypes.StakeAmount),
		checkpointedAvsOpState:        make(map[sdktypes.OperatorId]types.OperatorAvsState),
		currentOpState:                make(map[sdktypes.OperatorId]types.OperatorAvsState),
		checkpointedBlock:             0,
		atBlock:                       0,
		quorumDiff:                    make(map[sdktypes.QuorumNum]types.QuorumStakeDiff),
		operatorIdsToBeUpdated:        make(map[sdktypes.OperatorId]bool),
		quorumsToBeUpdated:            make(map[sdktypes.QuorumNum]bool),
		quorumPosPercThreshold:        20,
		quorumNegPercThreshold:        20,
		updateFullQuorumThresholdPerc: 30,
		triggerOpStateUpdate:          false,
		resetTrackedQuorums:           false,
		paused:                        false,
		pauseReasonV:                  "",
		lastOpStateUpdateTime:         time.Time{},
		triggerOpStateUpdateWindow:    time.Duration(minOpUpdateInterval) * time.Minute,
	}, nil
}

// TODO
// error handle here?
func getEventID(abi *abi.ABI, eventName string) common.Hash {
	return abi.Events[eventName].ID
}

func (osu *OpStateUpdater) startAsyncOpStateUpdater(ctx context.Context, sendNewOpTaskC chan types.SendNewOpTaskType, errorC chan error) {
	osu.logger.Infof("Starting startAsyncOpStateUpdater")

	// This is the errorC which we will send to incase we
	// encounter an error we can't proceed past
	// This error will be stored on the aggregator
	osu.errorC = errorC

	delegationManagerContractAddress := osu.ethRpc.AvsReader.AvsServiceBindings.DelegationManagerAddress
	stakeRegistryContractAddress := osu.ethRpc.AvsReader.AvsServiceBindings.StakeRegistryAddress
	taskManagerContractAddress := osu.ethRpc.AvsReader.AvsServiceBindings.TaskManagerAddress

	delegationManagerAbi, _ := delegationManager.ContractDelegationManagerMetaData.GetAbi()
	stakeRegistryAbi, _ := stakeRegistry.ContractStakeRegistryMetaData.GetAbi()
	taskmanagerAbi, _ := taskmanager.ContractFinalizerTaskManagerMetaData.GetAbi()

	osu.logger.Debug("OpStateUpdater -", "delegationManagerContractAddress", delegationManagerContractAddress)
	osu.logger.Debug("OpStateUpdater -", "stakeRegistryContractAddress", stakeRegistryContractAddress)
	osu.logger.Debug("OpStateUpdater -", "taskManagerContractAddress", taskManagerContractAddress)

	osu.logger.Debug("OpStateUpdater - Getting state from chain")

	lastCompletedOpTaskCreatedBlock, err := osu.ethRpc.AvsReader.LastCompletedOpTaskCreatedBlock(context.Background())
	if err != nil {
		osu.errorC <- fmt.Errorf("OpStateUpdater failed to LastCompletedOpTaskCreatedBlock: err: %v", err)
		return
	}

	if lastCompletedOpTaskCreatedBlock == 0 {
		osu.logger.Debug("lastCompletedOpTaskCreatedBlock is zero")
		osu.logger.Debug("Will resetTrackedQuorums and triggerOpStateUpdate")
		// Since we don't have an opTask yet
		// We want to reset/refresh all tracked quorums and then create an opTask
		// We do this as soon as we enter the main loop and then into the control loop
		osu.triggerOpStateUpdate = true
		osu.resetTrackedQuorums = true

		osu.logger.Debug("Checking if any operators are already registered")
		// Wait here till atleast 1 operator has registered
		// So that on local testnet the finalizer can register
		currentBlock, err := osu.ethRpc.Clients.EthHttpClient.BlockNumber(context.Background())
		if err != nil {
			osu.errorC <- fmt.Errorf("OpStateUpdater failed to get BlockNumber: err: %v", err)
			return
		}
		Ids := make(map[sdktypes.OperatorId]bool)
		for _, quorum := range types.TRACKED_QUORUM_NUMBERS {
			operatorIds, err := osu.ethRpc.Clients.AvsRegistryChainReader.GetOperatorIdList(&bind.CallOpts{}, quorum, uint32(currentBlock))
			if err != nil {
				osu.errorC <- fmt.Errorf("OpStateUpdater failed to GetOperatorIdList: err: %v", err)
				return
			}
			for _, opId := range operatorIds {
				Ids[opId] = true
			}
		}

		if len(Ids) == 0 {
			osu.logger.Debug("No operators registered yet")
			osu.logger.Debug("Waiting for atleast 1 operator to register")
			fromBlock := currentBlock + 1
			query := ethereum.FilterQuery{
				Addresses: []common.Address{stakeRegistryContractAddress},
				Topics: [][]common.Hash{
					[]common.Hash{
						getEventID(stakeRegistryAbi, "OperatorStakeUpdate"),
					},
				},
				FromBlock: big.NewInt(int64(fromBlock)),
			}
			rawLogsC, sub, err := osu.ethRpc.AvsSubscriber.StreamSubscriber.StreamQueryWithHistory(context.Background(), &query)
			if err != nil {
				osu.errorC <- fmt.Errorf("OpStateUpdater failed to SubscribeToOperatorStakeUpdate: err: %v, start: %v", err, fromBlock)
				return
			}
			defer sub.Unsubscribe()

		waitForRegisterLoop:
			for {
				select {
				case <-ctx.Done():
					osu.errorC <- ctx.Err()
					return
				case err := <-sub.Err():
					osu.errorC <- fmt.Errorf("OpStateUpdater encountered subscription error in waitForRegisterLoop: err: %v", err)
					return
				case vLog := <-rawLogsC:
					event, err := osu.ethRpc.AvsReader.AvsServiceBindings.StakeRegistry.ContractStakeRegistryFilterer.ParseOperatorStakeUpdate(vLog)
					if err != nil {
						osu.errorC <- fmt.Errorf("Failed to ParseOpTaskCompleted: err: %v, atBlock: %v", err, vLog.BlockNumber)
						return
					}
					osu.logger.Info("Received operatorStakeUpdate event:", "event", event)

					isQuorumTracked := false
					for _, quorum := range types.TRACKED_QUORUM_NUMBERS {
						if uint8(quorum) == event.QuorumNumber {
							isQuorumTracked = true
							break waitForRegisterLoop
						}
					}

					if isQuorumTracked {
						break waitForRegisterLoop
					}
				}
			}
			sub.Unsubscribe()
			osu.logger.Debug("An operator registered")
		}
	} else {
		osu.logger.Debug("Getting state at the checkpoint", "lastCompletedOpTaskCreatedBlock", lastCompletedOpTaskCreatedBlock)
		// If atleast 1 opTask was completed we have an opState
		// So we get the lastCompletedOpTaskCreatedBlock and start reading
		// events from that point on...
		osu.checkpointedBlock = lastCompletedOpTaskCreatedBlock
		osu.atBlock = lastCompletedOpTaskCreatedBlock

		err := osu.updateOpStates()
		if err != nil {
			osu.errorC <- fmt.Errorf("OpStateUpdater failed to updateOpStates: err: %v, checkpointedBlock: %v, atBlock: %v", err, osu.checkpointedBlock, osu.atBlock)
			return
		}
	}

	osu.logger.Debug("Sleeping to let the operators subscribe")
	// We need this here to let the finalizer subscribe
	// or the first opTask goes unanswered and the agg stalls
	time.Sleep(2 * time.Minute)

	// Prepare the subscription
	baseQuery := ethereum.FilterQuery{
		Addresses: []common.Address{delegationManagerContractAddress, stakeRegistryContractAddress, taskManagerContractAddress},
		Topics: [][]common.Hash{
			[]common.Hash{
				getEventID(delegationManagerAbi, "OperatorSharesIncreased"), getEventID(delegationManagerAbi, "OperatorSharesDecreased"),
				getEventID(stakeRegistryAbi, "OperatorStakeUpdate"), getEventID(stakeRegistryAbi, "StrategyMultiplierUpdated"), getEventID(stakeRegistryAbi, "MinimumStakeForQuorumUpdated"),
				getEventID(taskmanagerAbi, "PauseTrackingOpState"), getEventID(taskmanagerAbi, "OpTaskCompleted"),
			},
		},
	}

	for {

	controlLoop:
		for {
			switch {
			case osu.paused == true:
				{
					osu.logger.Info("OpStateUpdater is paused", "pauseReasonV", osu.pauseReasonV)
					osu.logger.Info("OpStateUpdater waiting for resume event")

					query := ethereum.FilterQuery{
						Addresses: []common.Address{taskManagerContractAddress},
						Topics: [][]common.Hash{
							[]common.Hash{
								getEventID(taskmanagerAbi, "ResumeTrackingOpState"),
							},
						},
						FromBlock: big.NewInt(int64(osu.atBlock+1)),
					}
					rawLogsC, sub, err := osu.ethRpc.AvsSubscriber.StreamSubscriber.StreamQueryWithHistory(context.Background(), &query)
					if err != nil {
						osu.errorC <- fmt.Errorf("OpStateUpdater failed to SubscribeToResumeTrackingOpState: err: %v, atBlock+1: %v", err, osu.checkpointedBlock, osu.atBlock+1)
						return
					}
					defer sub.Unsubscribe()
					ticker := time.NewTicker(time.Minute)
					defer ticker.Stop()
				watchForResumeLoop:
					for {
						select {
						case <-ticker.C:
							osu.logger.Info("The OpStateUpdater is paused due to: %v", osu.pauseReasonV)
						case <-ctx.Done():
							osu.errorC <- ctx.Err()
							return
						case err := <-sub.Err():
							osu.errorC <- fmt.Errorf("OpStateUpdater encountered subscription error in watchForResumeLoop: err: %v", err)
							return
						case vLog := <-rawLogsC:
							event, err := osu.ethRpc.AvsReader.AvsServiceBindings.TaskManager.ContractFinalizerTaskManagerFilterer.ParseResumeTrackingOpState(vLog)
							if err != nil {
								osu.errorC <- fmt.Errorf("Failed to ParseResumeTrackingOpState: err: %v, atBlock: %v", err, vLog.BlockNumber)
								return
							}
							osu.logger.Info("Received resume event: %v", event)
							if event.ResetTrackedQuorums {
								osu.logger.Debug("OpStateUpdater received resume event with resetTrackedQuorums = true")
								osu.resetTrackedQuorums = true
								osu.triggerOpStateUpdate = true
							} else {
								osu.logger.Debug("Getting states at the checkpoint and at the resume event block", "lastCompletedOpTaskCreatedBlock", lastCompletedOpTaskCreatedBlock, "atBlock", uint32(event.Raw.BlockNumber))
								lastCompletedOpTaskCreatedBlock, err := osu.ethRpc.AvsReader.LastCompletedOpTaskCreatedBlockAtBlock(context.Background(), uint64(event.Raw.BlockNumber))
								if err != nil {
									osu.errorC <- fmt.Errorf("OpStateUpdater failed to LastCompletedOpTaskCreatedBlock: err: %v, atBlock: %v", event.Raw.BlockNumber)
									return
								}
								osu.checkpointedBlock = lastCompletedOpTaskCreatedBlock
								osu.atBlock = uint32(event.Raw.BlockNumber)

								err = osu.updateOpStates()
								if err != nil {
									osu.errorC <- fmt.Errorf("OpStateUpdater failed to updateOpStates: err: %v, checkpointedBlock: %v, atBlock: %v", err, osu.checkpointedBlock, osu.atBlock)
									return
								}

							}
							osu.paused = false
							break watchForResumeLoop
						}
					}
					osu.logger.Info("OpStateUpdater received resume event")
					sub.Unsubscribe()
				}
			case osu.triggerOpStateUpdate:
				{
					osu.logger.Info("OpStateUpdater triggerOpStateUpdate")
					// Timer here to check if the last triggerOpStateUpdate was not too recent
					// We don't want sm1 changing stakes in the early stages of the project
					// to cost us a lot of eth
					if time.Since(osu.lastOpStateUpdateTime) < osu.triggerOpStateUpdateWindow {
						osu.paused = true
						osu.pauseReasonV = fmt.Sprintf("OpStateUpdater osu.triggerOpStateUpdate called again too soon: since: %v, now: %v", osu.lastOpStateUpdateTime, time.Now())
						continue
					}

					osu.logger.Info("OpStateUpdater triggering OpState update")

					// TODO
					// Here instead of updating stake everytime there is triggerOpStateUpdate
					// We could check if the avs and eigen states differ enough before doing so
					// if they don't we could skip updating stake and directly just create an opTask

					// Only two ways of triggerOpStateUpdate
					// Either via resetTrackedQuorums or via checkQuorumThresholds
					// The former doesn't require quorumDiff and quorumsToBeUpdated
					// The later will always have it
					if osu.resetTrackedQuorums {
						osu.logger.Info("OpStateUpdater UpdateStakeFull")
						osu.logger.Info("Resetting (updating) all tracked quorum's OpState")
						err := osu.UpdateStakeFull()
						if err != nil {
							osu.errorC <- fmt.Errorf("OpStateUpdater failed to UpdateStakeFull: err: %v, atBlock: %v", err, osu.atBlock)
							return
						}
						osu.resetTrackedQuorums = false
					} else {
						osu.logger.Info("OpStateUpdater UpdateStake")
						err := osu.UpdateStake()
						if err != nil {
							osu.errorC <- fmt.Errorf("OpStateUpdater failed to UpdateStake: err: %v, atBlock: %v", err, osu.atBlock)
							return
						}
					}

					osu.operatorIdsToBeUpdated = make(map[sdktypes.OperatorId]bool)
					osu.quorumDiff = make(map[sdktypes.QuorumNum]types.QuorumStakeDiff)
					osu.quorumsToBeUpdated = make(map[sdktypes.QuorumNum]bool)

					osu.logger.Info("OpStateUpdater Sending new OpTask")
					sendNewOpTaskReturnC := make(chan types.SendNewOpTaskReturn)
					var sendNewOpTaskReturn types.SendNewOpTaskReturn
					// send the task to the goroutine processing this task
					// and return the error (if any) returned by the signature verification routine
					select {
					// Here we do not require to use select since this function is synchronous and is run as a go routine only once, but we will in case that changes
					// we need to send this as part of select because if the goroutine is processing another triggerOpStateUpdate
					// and cannot receive this one, we want the context to be able to cancel the request
					case sendNewOpTaskC <- types.SendNewOpTaskType{
						SendNewOpTaskReturnC: sendNewOpTaskReturnC,
					}:
						// note that we need to wait synchronously here for this response because we want to get the result
						sendNewOpTaskReturn = <-sendNewOpTaskReturnC
					case <-ctx.Done():
						osu.errorC <- ctx.Err()
						return
					}

					osu.logger.Debug("OpStateUpdater -", "sendNewOpTaskReturn", sendNewOpTaskReturn)

					if sendNewOpTaskReturn.SendNewOpTaskError != nil {
						osu.errorC <- sendNewOpTaskReturn.SendNewOpTaskError
						return
					} else {
						osu.logger.Debug("OpStateUpdater waiting for opTask to complete")

						query := ethereum.FilterQuery{
							Addresses: []common.Address{taskManagerContractAddress},
							Topics: [][]common.Hash{
								[]common.Hash{
									getEventID(taskmanagerAbi, "OpTaskCompleted"),
								},
							},
							FromBlock: big.NewInt(int64(sendNewOpTaskReturn.OpTask.TaskCreatedBlock)),
						}
						rawLogsC, sub, err := osu.ethRpc.AvsSubscriber.StreamSubscriber.StreamQueryWithHistory(context.Background(), &query)
						if err != nil {
							osu.errorC <- fmt.Errorf("OpStateUpdater failed to SubscribeToOpTaskCompleted: err: %v, atBlock: %v", err, sendNewOpTaskReturn.OpTask.TaskCreatedBlock)
							return
						}
						defer sub.Unsubscribe()

						timer := time.NewTimer(time.Hour)
						defer timer.Stop()

						osu.logger.Debug("OpStateUpdater waiting for opTask to complete - 2")
					watchForOpTaskCompletedLoop:
						for {
							select {
							case <-ctx.Done():
								osu.errorC <- ctx.Err()
								return
							case err := <-sub.Err():
								osu.errorC <- fmt.Errorf("OpStateUpdater encountered subscription error in watchForOpTaskCompletedLoop: err: %v", err)
								return
							case <-timer.C:
								osu.errorC <- fmt.Errorf("OpStateUpdater timed out in watchForOpTaskCompletedLoop")
								return
							case vLog := <-rawLogsC:
								{
									event, err := osu.ethRpc.AvsReader.AvsServiceBindings.TaskManager.ContractFinalizerTaskManagerFilterer.ParseOpTaskCompleted(vLog)
									if err != nil {
										osu.errorC <- fmt.Errorf("Failed to ParseOpTaskCompleted: err: %v, atBlock: %v", err, vLog.BlockNumber)
										return
									}
									osu.logger.Debugf("OpStateUpdater - Received OpTaskCompleted event: %v", event)

									// TODO
									// If get an OpTaskCompleted event with taskIndex higher than
									// what we expect then perhaps our expected task didn't complete
									// And another one was init and then completed so, we shouldn't
									// error out here instead set the osu.checkpointedBlock and osu.atBlock
									// then break out of the loop and let updateOpStates update accordingly
									// Similar to how we do it when we see an OpTaskCompleted event when watching
									// triggers
									if sendNewOpTaskReturn.OpTask.TaskNum < event.TaskIndex {
										osu.logger.Debugf("OpStateUpdater - Received OpTaskCompleted event has task with higher than expted taskIndex: %v", event)
										lastCompletedOpTaskCreatedBlock, err := osu.ethRpc.AvsReader.LastCompletedOpTaskCreatedBlockAtBlock(context.Background(), event.Raw.BlockNumber)
										if err != nil {
											osu.errorC <- fmt.Errorf("OpStateUpdater failed to LastCompletedOpTaskCreatedBlock: err: %v, atBlock: %v", event.Raw.BlockNumber)
											return
										}
										osu.checkpointedBlock = lastCompletedOpTaskCreatedBlock
										osu.atBlock = uint32(event.Raw.BlockNumber)
									} else if sendNewOpTaskReturn.OpTask.TaskNum > event.TaskIndex {

									// This branch is to account for the case where
									// a task is completed in a block and another task is created
									// in the same block and then that one is also completed in the same block
										continue
									} else {

									osu.logger.Info("OpStateUpdater - Got the expected OpTaskCompleted event", "TaskIndex", sendNewOpTaskReturn.OpTask.TaskCreatedBlock)

									// TODO
									// Since we are here upon observing an OpTaskCompleted event
									// probably should set atBlock to the current block (at which we
									// found the OpTaskCompleted event) rather than the OpTask.TaskCreatedBlock
									// Similar to how we do it when we see an OpTaskCompleted event when watching
									// triggers
									// In any case when we come to the OpTaskCompleted event it will do the above anyway
									osu.checkpointedBlock = sendNewOpTaskReturn.OpTask.TaskCreatedBlock
									osu.atBlock = uint32(event.Raw.BlockNumber)
									}
									break watchForOpTaskCompletedLoop
								}
							}
						}
						sub.Unsubscribe()
						osu.lastOpStateUpdateTime = time.Now()
						osu.logger.Debug("OpStateUpdater done waiting for opTask to complete")

						err = osu.updateOpStates()
						if err != nil {
							osu.errorC <- fmt.Errorf("OpStateUpdater failed to updateOpStates: err: %v, checkpointedBlock: %v, atBlock: %v", err, osu.checkpointedBlock, osu.atBlock)
							return
						}
					}

				}
			default:
				break controlLoop
			}
		}

		osu.logger.Info("OpStateUpdater Listening to trigger events", "osu.atBlock + 1", osu.atBlock+1)

		// Subscribe
		var logs chan ethtypes.Log
		iquery := baseQuery
		iquery.FromBlock = big.NewInt(int64(osu.atBlock + 1))

		var sub ethereum.Subscription
		var err error
		const maxRetries = 5
		const retryDelay = time.Minute

		// Loop to retry subscription on error
		for attempt := 0; attempt < maxRetries; attempt++ {
			logs, sub, err = osu.ethRpc.AvsSubscriber.StreamSubscriber.StreamQueryWithHistory(context.Background(), &iquery)
			if err == nil {
				break // Successfully subscribed, exit loop
			}

			osu.logger.Error("Failed to get new head from substrate, retrying...", "err", err, "attempt", attempt+1)
			select {
			case <-ctx.Done():
				osu.errorC <- ctx.Err()
				return
			case <-time.After(retryDelay): // Wait before retrying
				continue
			}
		}
		if err != nil {
			osu.errorC <- fmt.Errorf("failed to subscribe to new heads from substrate after %d attempts: %w", maxRetries, err)
			return
		}
		defer sub.Unsubscribe()
		osu.logger.Info("OpStateUpdater Subscribed to trigger events", "osu.atBlock + 1", osu.atBlock+1)

		// watch the subscription
	watchForTriggersLoop:
		for {
			select {
			case <-ctx.Done():
				osu.errorC <- ctx.Err()
				return
			case err := <-sub.Err():
				osu.logger.Error("watchForTriggersLoop subscription error: %v - retrying atBlock %v", err, osu.atBlock)
				// We do this so because the subscription will start from  atBlock+1
				osu.atBlock = osu.atBlock - 1
				break watchForTriggersLoop
			case vLog := <-logs:
				osu.logger.Debugf("Received log: %+v\n", vLog)
				switch {
				case vLog.Address == delegationManagerContractAddress && vLog.Topics[0] == getEventID(delegationManagerAbi, "OperatorSharesIncreased"):
					{
						osu.logger.Debugf("Event %s from contract %s\n", "OperatorSharesIncreased", vLog.Address.Hex())
						osu.atBlock = uint32(vLog.BlockNumber)
						// Process the log here based on event signature and ABI
						ContractDelegationManagerOperatorSharesIncreased, err := osu.ethRpc.AvsReader.AvsServiceBindings.DelegationManager.ContractDelegationManagerFilterer.ParseOperatorSharesIncreased(vLog)
						if err != nil {
							osu.errorC <- fmt.Errorf("Failed to ParseOperatorSharesIncreased: err: %v, atBlock: %v", err, vLog.BlockNumber)
							return
						}
						err = osu.processOpDelegationStateChange(ContractDelegationManagerOperatorSharesIncreased.Operator, uint32(vLog.BlockNumber))
						if err != nil {
							osu.errorC <- fmt.Errorf("OpStateUpdater failed to processOpDelegationStateChange: err: %v, atBlock: %v", err, vLog.BlockNumber)
							return
						}

						if osu.paused || osu.triggerOpStateUpdate {
							break watchForTriggersLoop
						}
					}
				case vLog.Address == delegationManagerContractAddress && vLog.Topics[0] == getEventID(delegationManagerAbi, "OperatorSharesDecreased"):
					{
						osu.logger.Debugf("Event %s from contract %s\n", "OperatorSharesDecreased", vLog.Address.Hex())
						osu.atBlock = uint32(vLog.BlockNumber)
						// Process the log here based on event signature and ABI
						ContractDelegationManagerOperatorSharesDecreased, err := osu.ethRpc.AvsReader.AvsServiceBindings.DelegationManager.ContractDelegationManagerFilterer.ParseOperatorSharesDecreased(vLog)
						if err != nil {
							osu.errorC <- fmt.Errorf("Failed to ParseOperatorSharesDecreased: err: %v, atBlock: %v", err, vLog.BlockNumber)
							return
						}
						err = osu.processOpDelegationStateChange(ContractDelegationManagerOperatorSharesDecreased.Operator, uint32(vLog.BlockNumber))
						if err != nil {
							osu.errorC <- fmt.Errorf("OpStateUpdater failed to processOpDelegationStateChange: err: %v, atBlock: %v", err, vLog.BlockNumber)
							return
						}

						if osu.paused || osu.triggerOpStateUpdate {
							break watchForTriggersLoop
						}
					}
				case vLog.Address == stakeRegistryContractAddress && vLog.Topics[0] == getEventID(stakeRegistryAbi, "OperatorStakeUpdate"):
					{
						osu.logger.Debugf("Event %s from contract %s\n", "OperatorStakeUpdate", vLog.Address.Hex())
						osu.atBlock = uint32(vLog.BlockNumber)
						// Process the log here based on event signature and ABI
						ContractStakeRegistryOperatorStakeUpdate, err := osu.ethRpc.AvsReader.AvsServiceBindings.StakeRegistry.ContractStakeRegistryFilterer.ParseOperatorStakeUpdate(vLog)
						if err != nil {
							osu.errorC <- fmt.Errorf("Failed to ParseOperatorStakeUpdate: err: %v, atBlock: %v", err, vLog.BlockNumber)
							return
						}
						err = osu.processOpStakeRegistryStateChange(ContractStakeRegistryOperatorStakeUpdate.OperatorId, sdktypes.QuorumNum(ContractStakeRegistryOperatorStakeUpdate.QuorumNumber), ContractStakeRegistryOperatorStakeUpdate.Stake, uint32(vLog.BlockNumber))
						if err != nil {
							osu.errorC <- fmt.Errorf("OpStateUpdater failed to processOpStakeRegistryStateChange: err: %v, atBlock: %v", err, vLog.BlockNumber)
							return
						}

						if osu.paused || osu.triggerOpStateUpdate {
							break watchForTriggersLoop
						}
					}
				case vLog.Address == stakeRegistryContractAddress && vLog.Topics[0] == getEventID(stakeRegistryAbi, "StrategyMultiplierUpdated"):
					{
						osu.logger.Debugf("Event %s from contract %s\n", "StrategyMultiplierUpdated", vLog.Address.Hex())
						osu.atBlock = uint32(vLog.BlockNumber)
						// Process the log here based on event signature and ABI
						ContractStakeRegistryStrategyMultiplierUpdated, err := osu.ethRpc.AvsReader.AvsServiceBindings.StakeRegistry.ContractStakeRegistryFilterer.ParseStrategyMultiplierUpdated(vLog)
						if err != nil {
							osu.errorC <- fmt.Errorf("Failed to ParseStrategyMultiplierUpdated: err: %v, atBlock: %v", err, vLog.BlockNumber)
							return
						}
						osu.logger.Info("Pausing trackingOpState upon event trigger", "trigger", ContractStakeRegistryStrategyMultiplierUpdated)
						osu.paused = true
						osu.pauseReasonV = fmt.Sprintf("Pausing trackingOpState upon event trigger: %v", ContractStakeRegistryStrategyMultiplierUpdated)
						break watchForTriggersLoop
					}
				case vLog.Address == stakeRegistryContractAddress && vLog.Topics[0] == getEventID(stakeRegistryAbi, "MinimumStakeForQuorumUpdated"):
					{
						osu.logger.Debugf("Event %s from contract %s\n", "MinimumStakeForQuorumUpdated", vLog.Address.Hex())
						osu.atBlock = uint32(vLog.BlockNumber)
						// Process the log here based on event signature and ABI
						ContractStakeRegistryMinimumStakeForQuorumUpdated, err := osu.ethRpc.AvsReader.AvsServiceBindings.StakeRegistry.ContractStakeRegistryFilterer.ParseMinimumStakeForQuorumUpdated(vLog)
						if err != nil {
							osu.errorC <- fmt.Errorf("Failed to ParseMinimumStakeForQuorumUpdated: err: %v, atBlock: %v", err, vLog.BlockNumber)
							return
						}
						osu.logger.Info("Pausing trackingOpState upon event trigger", "trigger", ContractStakeRegistryMinimumStakeForQuorumUpdated)
						osu.paused = true
						osu.pauseReasonV = fmt.Sprintf("Pausing trackingOpState upon event trigger: %v", ContractStakeRegistryMinimumStakeForQuorumUpdated)
						break watchForTriggersLoop
					}
				case vLog.Address == taskManagerContractAddress && vLog.Topics[0] == getEventID(taskmanagerAbi, "PauseTrackingOpState"):
					{
						osu.logger.Debugf("Event %s from contract %s\n", "PauseTrackingOpState", vLog.Address.Hex())
						osu.atBlock = uint32(vLog.BlockNumber)
						// Process the log here based on event signature and ABI
						ContractFinalizerTaskManagerPauseTrackingOpState, err := osu.ethRpc.AvsReader.AvsServiceBindings.TaskManager.ContractFinalizerTaskManagerFilterer.ParsePauseTrackingOpState(vLog)
						if err != nil {
							osu.errorC <- fmt.Errorf("Failed to ParsePauseTrackingOpState: err: %v, atBlock: %v", err, vLog.BlockNumber)
							return
						}
						osu.logger.Info("Pausing trackingOpState upon event trigger", "trigger", ContractFinalizerTaskManagerPauseTrackingOpState)
						osu.paused = true
						osu.pauseReasonV = fmt.Sprintf("Pausing trackingOpState upon event trigger: %v", ContractFinalizerTaskManagerPauseTrackingOpState)
						break watchForTriggersLoop
					}
				case vLog.Address == taskManagerContractAddress && vLog.Topics[0] == getEventID(taskmanagerAbi, "OpTaskCompleted"):
					{
						osu.logger.Debugf("Event %s from contract %s\n", "OpTaskCompleted", vLog.Address.Hex())
						osu.atBlock = uint32(vLog.BlockNumber)
						// Process the log here based on event signature and ABI
						ContractFinalizerTaskManagerOpTaskCompleted, err := osu.ethRpc.AvsReader.AvsServiceBindings.TaskManager.ContractFinalizerTaskManagerFilterer.ParseOpTaskCompleted(vLog)
						if err != nil {
							osu.errorC <- fmt.Errorf("Failed to ParseOpTaskCompleted: err: %v, atBlock: %v", err, vLog.BlockNumber)
							return
						}

						osu.logger.Info("Received OpTaskCompleted", "event", ContractFinalizerTaskManagerOpTaskCompleted)

						lastCompletedOpTaskCreatedBlock, err := osu.ethRpc.AvsReader.LastCompletedOpTaskCreatedBlockAtBlock(context.Background(), vLog.BlockNumber)
						if err != nil {
							osu.errorC <- fmt.Errorf("OpStateUpdater failed to LastCompletedOpTaskCreatedBlock: err: %v, atBlock: %v", vLog.BlockNumber)
							return
						}
						osu.checkpointedBlock = lastCompletedOpTaskCreatedBlock

						err = osu.updateOpStates()
						if err != nil {
							osu.errorC <- fmt.Errorf("OpStateUpdater failed to updateOpStates: err: %v, checkpointedBlock: %v, atBlock: %v", err, osu.checkpointedBlock, osu.atBlock)
							return
						}
						break watchForTriggersLoop
					}
				}
			}
		}
		sub.Unsubscribe()
	}

}

func (osu *OpStateUpdater) updateOperatorIdsToBeUpdated() {

	osu.operatorIdsToBeUpdated = make(map[sdktypes.OperatorId]bool)

	for operatorId, _ := range osu.checkpointedAvsOpState {
		for _, quorum := range types.TRACKED_QUORUM_NUMBERS {
			updatedStake := osu.currentOpState[operatorId].StakePerQuorum[quorum]
			if updatedStake == nil {
				updatedStake = big.NewInt(0)
			}
			prevStake := osu.checkpointedAvsOpState[operatorId].StakePerQuorum[quorum]
			if prevStake == nil {
				prevStake = big.NewInt(0)
			}
			if prevStake.Cmp(updatedStake) != 0 {
				osu.operatorIdsToBeUpdated[operatorId] = true
			}
		}
	}

	for operatorId, _ := range osu.currentOpState {
		for _, quorum := range types.TRACKED_QUORUM_NUMBERS {
			updatedStake := osu.currentOpState[operatorId].StakePerQuorum[quorum]
			if updatedStake == nil {
				updatedStake = big.NewInt(0)
			}
			prevStake := osu.checkpointedAvsOpState[operatorId].StakePerQuorum[quorum]
			if prevStake == nil {
				prevStake = big.NewInt(0)
			}
			if prevStake.Cmp(updatedStake) != 0 {
				osu.operatorIdsToBeUpdated[operatorId] = true
			}
		}
	}
}

func (osu *OpStateUpdater) updateOpStates() error {

	// We get the relevant avs and eigen state at the
	// checkpointed block and if we require an opStateUpdate
	// we pause because if since the time that avs stake was updated
	// to the time the opTask was completed the stake has changed enough
	// to trip the thresholds - that's extraordinary
	// and we should not auto create tasks here to prevent
	// some guy from changing his stake a lot in the early days of the mainnet
	// to cost us a lot of eth

	osu.triggerOpStateUpdate = false
	var err error
	osu.logger.Debug("OpStateUpdater - getCheckpointedOpState")
	err = osu.getCheckpointedOpState(osu.checkpointedBlock)
	if err != nil {
		return err
	}
	osu.logger.Debug("OpStateUpdater - getCurrentOpState")
	err = osu.getCurrentOpState(osu.atBlock)
	if err != nil {
		return err
	}
	osu.logger.Debug("OpStateUpdater - checkQuorumThresholds")
	osu.checkQuorumThresholds()
	if osu.triggerOpStateUpdate {
		osu.paused = true
		osu.pauseReasonV = fmt.Sprintf("triggerOpStateUpdate true in updateOpStates")
	}
	osu.logger.Debug("OpStateUpdater - isAnyQuorumInCheckpointedAvsStateEmpty")
	if osu.isAnyQuorumInCheckpointedAvsStateEmpty() {
		osu.paused = true
		osu.pauseReasonV = fmt.Sprintf("isAnyQuorumInCheckpointedAvsStateEmpty true in updateOpStates")
	}

	// We update operatorIdsToBeUpdated here because
	// of possible eigen state changes since the last avs state update
	// till the the checkpoint
	osu.updateOperatorIdsToBeUpdated()
	return nil
}

func (osu *OpStateUpdater) getCheckpointedOpState(atBlock sdktypes.BlockNum) error {
	checkpointedAvsOpState, err := osu.ethRpc.AvsReader.GetOperatorsAvsStateAtBlock(context.Background(), osu.ethRpc.AvsReader.AvsServiceBindings.RegistryCoordinatorAddress, types.TRACKED_QUORUM_NUMBERS, atBlock)
	if err != nil {
		ierr := fmt.Errorf("OpStateUpdater failed to getCheckpointedOpState: err: %v, atBlock: %v", err, atBlock)
		osu.logger.Error(ierr.Error())
		return ierr
	}
	osu.checkpointedAvsOpState = checkpointedAvsOpState
	osu.updateCheckpointedAvsQuorumStakes()
	return nil
}

// TODO
// Probably can make this better by having GetTypedOperatorsStakesForQuorumAtBlock
// accept [][]address ([quorum][operator]) along with the quorumNums, internally
// this can be passed onto the contract query which can return the Operator[][]
// this can then be processed into the final type required
// This way we can avoid the annoying "if (!BitmapUtils.orderedBytesArrayToBitmap(quorumBytes).isSubsetOf(registryCoordinator.getCurrentQuorumBitmap(operatorId)))"
// in the view function
// This would also be more in line with the way the operatorStateRetriever's view
// functions work
// However, the annoying if condition seems to have to be in the view function
// that serves processOpDelegationStateChange, since attempting to findout the
// operator's quorums here (in go code - rather than in the view function) is
// pretty annoying too
// Thus, I am gonna leave this as it is for now :)
func (osu *OpStateUpdater) getCurrentOpState(atBlock sdktypes.BlockNum) error {
	Ids := make(map[sdktypes.OperatorId]bool)
	for _, quorum := range types.TRACKED_QUORUM_NUMBERS {
		operatorIds, err := osu.ethRpc.Clients.AvsRegistryChainReader.GetOperatorIdList(&bind.CallOpts{}, quorum, uint32(atBlock))
		if err != nil {
			ierr := fmt.Errorf("OpStateUpdater failed to GetOperatorIdList: err: %v, atBlock: %v", err, atBlock)
			osu.logger.Error(ierr.Error())
			return ierr
		}
		for _, opId := range operatorIds {
			Ids[opId] = true
		}
	}

	keys := make([]sdktypes.OperatorId, 0, len(Ids))
	for key, _ := range osu.checkpointedAvsOpState {
		keys = append(keys, key)
	}

	operatorAddresses, err := osu.ethRpc.AvsReader.GetOperatorsFromIds(&bind.CallOpts{Context: context.Background(), BlockNumber: big.NewInt(int64(atBlock))}, osu.ethRpc.AvsReader.AvsServiceBindings.RegistryCoordinatorAddress, keys)
	if err != nil {
		ierr := fmt.Errorf("OpStateUpdater failed to GetOperatorsFromIds: err: %v, atBlock: %v", err, atBlock)
		osu.logger.Error(ierr.Error())
		return ierr
	}

	opState, err := osu.ethRpc.AvsReader.GetTypedOperatorsStakesForQuorumAtBlock(context.Background(), osu.ethRpc.AvsReader.AvsServiceBindings.RegistryCoordinatorAddress, types.TRACKED_QUORUM_NUMBERS, operatorAddresses, atBlock)
	if err != nil {
		ierr := fmt.Errorf("OpStateUpdater failed to GetTypedOperatorsStakesForQuorumAtBlock: err: %v, atBlock: %v", err, atBlock)
		osu.logger.Error(ierr.Error())
		return ierr
	}

	osu.currentOpState = opState
	return nil
}

func (osu *OpStateUpdater) isAnyQuorumInCheckpointedAvsStateEmpty() bool {
	isAnyQuorumEmpty := false
	for _, quorum := range types.TRACKED_QUORUM_NUMBERS {
		stake := big.NewInt(0)
		for _, opState := range osu.checkpointedAvsOpState {
			opQuorumStake := opState.StakePerQuorum[quorum]
			if opQuorumStake == nil {
				opQuorumStake = big.NewInt(0)
			}
			stake.Add(stake, opQuorumStake)
		}
		if stake == big.NewInt(0) {
			isAnyQuorumEmpty = true
		}
	}
	return isAnyQuorumEmpty
}

func (osu *OpStateUpdater) processOpStakeRegistryStateChange(operatorId sdktypes.OperatorId, quorumNumber sdktypes.QuorumNum, stake sdktypes.StakeAmount, blockNumber sdktypes.BlockNum) error {

	isQuorumTracked := false

	for _, quorum := range types.TRACKED_QUORUM_NUMBERS {
		if quorum == quorumNumber {
			isQuorumTracked = true
			break
		}
	}

	osu.logger.Debugf("OpStateUpdater - processOpStakeRegistryStateChange isQuorumTracked: %v", isQuorumTracked)

	if isQuorumTracked {

		operator, err := osu.ethRpc.Clients.AvsRegistryChainReader.GetOperatorFromId(&bind.CallOpts{}, operatorId)
		if err != nil {
			ierr := fmt.Errorf("OpStateUpdater failed to GetOperatorFromId: err: %v, operator: %v, atBlock: %v", err, operatorId, blockNumber)
			osu.logger.Error(ierr.Error())
			return ierr
		}
		if bytes.Equal(operator.Bytes(), types.ZERO_ADDRESS[:]) {
			// Zero operator here means that the operator is not registered with us at all
			// So we should just continue... Which is unlikely here since no blsKey records are ever deleted
			// and this event should only be acessible for those op that are registered
			// so we are here something has gone very wrong!
			// But for now we just continue
			osu.logger.Error("OpStateUpdater zero operator from GetOperatorFromId call inside processOpStakeRegistryStateChange:, operator: %v, atBlock: %v", operatorId, blockNumber)
			return nil
		}

		if currentOpState, ok := osu.currentOpState[operatorId]; ok {
			if currentOpState.StakePerQuorum == nil {
				currentOpState.StakePerQuorum = make(map[sdktypes.QuorumNum]sdktypes.StakeAmount)
				osu.currentOpState[operatorId] = currentOpState
			}
		} else {
			osu.currentOpState[operatorId] = types.OperatorAvsState{
				OperatorId:     operatorId,
				Operator:       operator,
				StakePerQuorum: make(map[sdktypes.QuorumNum]sdktypes.StakeAmount),
			}
		}

		osu.currentOpState[operatorId].StakePerQuorum[quorumNumber] = stake
		// maybe unnecessary here
		osu.operatorIdsToBeUpdated[operatorId] = true
		osu.checkQuorumThresholds()
	}
	return nil
}

func (osu *OpStateUpdater) processOpDelegationStateChange(operator common.Address, blockNumber sdktypes.BlockNum) error {

	operatorId, err := osu.ethRpc.Clients.AvsRegistryChainReader.GetOperatorId(&bind.CallOpts{}, operator)
	if err != nil {
		ierr := fmt.Errorf("OpStateUpdater failed to GetOperatorId: err: %v, operator: %v, atBlock: %v", err, operator, blockNumber)
		osu.logger.Error(ierr.Error())
		return ierr
	}
	if bytes.Equal(operatorId[:], types.ZERO_OPERATOR_ID[:]) {
		// Zero operatorId here means that the operator is not registered with us at all
		// So we should just continue...
		return nil
	}
	if _, ok := osu.currentOpState[operatorId]; ok {
		opStateUpdate, err := osu.ethRpc.AvsReader.GetTypedOperatorsStakesForQuorumAtBlock(context.Background(), osu.ethRpc.AvsReader.AvsServiceBindings.RegistryCoordinatorAddress, types.TRACKED_QUORUM_NUMBERS, []common.Address{operator}, blockNumber)
		if err != nil {
			ierr := fmt.Errorf("OpStateUpdater failed to GetTypedOperatorsStakesForQuorumAtBlock: err: %v, operator: %v, atBlock: %v", err, operator, blockNumber)
			osu.logger.Error(ierr.Error())
			return ierr
		}

		osu.applyOpStateUpdate(opStateUpdate)
		osu.operatorIdsToBeUpdated[operatorId] = true

		osu.checkQuorumThresholds()
	}

	return nil
}

func (osu *OpStateUpdater) applyOpStateUpdate(opStateUpdate map[sdktypes.OperatorId]types.OperatorAvsState) {
	for operatorId, opUpdate := range opStateUpdate {
		if currentOpState, ok := osu.currentOpState[operatorId]; ok {
			currentOpState.StakePerQuorum = opUpdate.StakePerQuorum
			osu.currentOpState[operatorId] = currentOpState
		} else {
			osu.currentOpState[operatorId] = types.OperatorAvsState{
				OperatorId:     operatorId,
				Operator:       opUpdate.Operator,
				StakePerQuorum: opUpdate.StakePerQuorum,
			}
		}
	}
}

func (osu *OpStateUpdater) updateCheckpointedAvsQuorumStakes() {

	osu.checkpointedAvsQuorumStakes = make(map[sdktypes.QuorumNum]sdktypes.StakeAmount)

	for _, quorum := range types.TRACKED_QUORUM_NUMBERS {
		osu.checkpointedAvsQuorumStakes[quorum] = big.NewInt(0)
	}

	for operatorId, _ := range osu.checkpointedAvsOpState {
		for _, quorum := range types.TRACKED_QUORUM_NUMBERS {
			stake := osu.checkpointedAvsOpState[operatorId].StakePerQuorum[quorum]
			if stake == nil {
				stake = big.NewInt(0)
			}
			osu.checkpointedAvsQuorumStakes[quorum].Add(osu.checkpointedAvsQuorumStakes[quorum], stake)
		}
	}

}

func (osu *OpStateUpdater) checkQuorumThresholds() {
	osu.makeQuorumsStakeDiff()
	osu.checkQuorumsStakeDiff()
}

func (osu *OpStateUpdater) makeQuorumsStakeDiff() {

	osu.quorumDiff = make(map[sdktypes.QuorumNum]types.QuorumStakeDiff)

	for _, quorum := range types.TRACKED_QUORUM_NUMBERS {
		osu.quorumDiff[quorum] = types.QuorumStakeDiff{NegDiff: big.NewInt(0), PosDiff: big.NewInt(0)}
	}

	for operatorId, _ := range osu.checkpointedAvsOpState {
		for _, quorum := range types.TRACKED_QUORUM_NUMBERS {
			updatedStake := osu.currentOpState[operatorId].StakePerQuorum[quorum]
			if updatedStake == nil {
				updatedStake = big.NewInt(0)
			}
			prevStake := osu.checkpointedAvsOpState[operatorId].StakePerQuorum[quorum]
			if prevStake == nil {
				prevStake = big.NewInt(0)
			}
			if prevStake.Cmp(updatedStake) == 1 {
				diff := big.NewInt(0).Sub(prevStake, updatedStake)
				osu.quorumDiff[quorum].NegDiff.Add(osu.quorumDiff[quorum].NegDiff, diff)
			}
		}
	}

	for operatorId, _ := range osu.currentOpState {
		for _, quorum := range types.TRACKED_QUORUM_NUMBERS {
			updatedStake := osu.currentOpState[operatorId].StakePerQuorum[quorum]
			if updatedStake == nil {
				updatedStake = big.NewInt(0)
			}
			prevStake := osu.checkpointedAvsOpState[operatorId].StakePerQuorum[quorum]
			if prevStake == nil {
				prevStake = big.NewInt(0)
			}
			if prevStake.Cmp(updatedStake) == -1 {
				diff := big.NewInt(0).Sub(updatedStake, prevStake)
				osu.quorumDiff[quorum].PosDiff.Add(osu.quorumDiff[quorum].PosDiff, diff)
			}
		}
	}

	osu.logger.Debugf("OpStateUpdater - makeQuorumsStakeDiff quorumDiff: %v", osu.quorumDiff)
}

func (osu *OpStateUpdater) checkQuorumsStakeDiff() {

	osu.quorumsToBeUpdated = make(map[sdktypes.QuorumNum]bool)

	for _, qn := range types.TRACKED_QUORUM_NUMBERS {
		checkpointedQuorumStake := uint32(osu.checkpointedAvsQuorumStakes[qn].Int64())
		if checkpointedQuorumStake != 0 {
			if uint32(osu.quorumDiff[qn].NegDiff.Int64())*100 > checkpointedQuorumStake*uint32(osu.quorumNegPercThreshold) {
				osu.quorumsToBeUpdated[qn] = true
				osu.triggerOpStateUpdate = true
			}
			if uint32(osu.quorumDiff[qn].PosDiff.Int64())*100 > checkpointedQuorumStake*uint32(osu.quorumPosPercThreshold) {
				osu.quorumsToBeUpdated[qn] = true
				osu.triggerOpStateUpdate = true
			}
		} else {
			if uint32(osu.quorumDiff[qn].PosDiff.Int64()) > 0 {
				osu.quorumsToBeUpdated[qn] = true
				osu.triggerOpStateUpdate = true
			}
		}
	}

	osu.logger.Debugf("OpStateUpdater - checkQuorumsStakeDiff checkpointedAvsQuorumStakes: %v", osu.checkpointedAvsQuorumStakes)
	osu.logger.Debugf("OpStateUpdater - checkQuorumsStakeDiff quorumsToBeUpdated: %v", osu.quorumsToBeUpdated)
	osu.logger.Debugf("OpStateUpdater - checkQuorumsStakeDiff triggerOpStateUpdate: %v", osu.triggerOpStateUpdate)
}

func (osu *OpStateUpdater) UpdateStake() error {

	// Optimized for a single quorum for now
	// Doesn't consider untracked quorums
	if uint32(len(osu.operatorIdsToBeUpdated))*100 < uint32(len(osu.currentOpState))*uint32(osu.updateFullQuorumThresholdPerc) {
		keys := make([]sdktypes.OperatorId, 0, len(osu.operatorIdsToBeUpdated))

		for key := range osu.operatorIdsToBeUpdated {
			keys = append(keys, key)
		}
		return osu.UpdateStateForOperatorIds(keys)
	} else {
		keys := make([]sdktypes.QuorumNum, 0, len(osu.quorumsToBeUpdated))

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
		ierr := fmt.Errorf("OpStateUpdater failed to get BlockNumber: err: %v, atBlock: %v, quorumNums: %v", err, currentBlock, quorumNums)
		osu.logger.Error(ierr.Error())
		return ierr
	}

	var operatorAddresses [][]common.Address
	for _, qn := range quorumNums {
		operatorIds, err := osu.ethRpc.Clients.AvsRegistryChainReader.GetOperatorIdList(&bind.CallOpts{}, qn, uint32(currentBlock))
		if err != nil {
			ierr := fmt.Errorf("OpStateUpdater failed to GetOperatorIdList: err: %v, atBlock: %v, quorumNums: %v", err, currentBlock, qn)
			osu.logger.Error(ierr.Error())
			return ierr
		}

		operatorAdrresses0, err := osu.ethRpc.AvsReader.GetOperatorsFromIds(&bind.CallOpts{}, osu.ethRpc.AvsReader.AvsServiceBindings.RegistryCoordinatorAddress, operatorIds)
		if err != nil {
			ierr := fmt.Errorf("OpStateUpdater failed to GetOperatorsFromIds: err: %v, atBlock: %v, quorumNums: %v", err, currentBlock, qn)
			osu.logger.Error(ierr.Error())
			return ierr
		}

		sort.Slice(operatorAdrresses0, func(i, j int) bool {
			a := big.NewInt(0).SetBytes(operatorAdrresses0[i][:])
			b := big.NewInt(0).SetBytes(operatorAdrresses0[j][:])
			return a.Cmp(b) < 0
		})

		operatorAddresses = append(operatorAddresses, operatorAdrresses0)

	}

	_, err = osu.ethRpc.Clients.AvsRegistryChainWriter.UpdateStakesOfEntireOperatorSetForQuorums(context.Background(), operatorAddresses, quorumNums)
	if err != nil {
		ierr := fmt.Errorf("OpStateUpdater failed to UpdateStakesOfEntireOperatorSetForQuorums: err: %v, atBlock: %v, quorumNums: %v", err, currentBlock, quorumNums)
		osu.logger.Error(ierr.Error())
		return ierr
	}

	osu.logger.Info("UpdateStateForEntireQuorums successfull", "quorumNums", quorumNums)

	return nil
}

func (osu *OpStateUpdater) UpdateStateForOperatorIds(operatorIds []sdktypes.OperatorId) error {
	osu.logger.Info("Running Operator Stake Update check")

	// currentBlock, err := osu.ethRpc.Clients.EthHttpClient.BlockNumber(context.Background())
	// if err != nil {
	// 	osu.logger.Error("Cannot get current block number", "err", err)
	// 	return err
	// }

	operatorAdrresses, err := osu.ethRpc.AvsReader.GetOperatorsFromIds(&bind.CallOpts{}, osu.ethRpc.AvsReader.AvsServiceBindings.RegistryCoordinatorAddress, operatorIds)
	if err != nil {
		ierr := fmt.Errorf("OpStateUpdater failed to GetOperatorsFromIds: err: %v", err)
		osu.logger.Error(ierr.Error())
		return ierr
	}

	_, err = osu.ethRpc.Clients.AvsRegistryChainWriter.UpdateStakesOfOperatorSubsetForAllQuorums(context.Background(), operatorAdrresses)
	if err != nil {
		ierr := fmt.Errorf("OpStateUpdater failed to UpdateStakesOfOperatorSubsetForAllQuorums: err: %v", err)
		osu.logger.Error(ierr.Error())
		return ierr
	}

	osu.logger.Info("UpdateStateForOperatorIds successfull", "addresses len", len(operatorAdrresses))

	return nil
}

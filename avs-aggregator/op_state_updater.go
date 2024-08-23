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
	errorC                        chan error
	ethRpc                        *chainio.EthRpc
	avsRegistryService            *avsregistry.AvsRegistryServiceChainCaller
}

func NewOpStateUpdater(logger logging.Logger, ethRpc *chainio.EthRpc, avsRegistryService *avsregistry.AvsRegistryServiceChainCaller) (*OpStateUpdater, error) {
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
		quorumPosPercThreshold:        50,
		quorumNegPercThreshold:        20,
		updateFullQuorumThresholdPerc: 30,
		triggerOpStateUpdate:          false,
		resetTrackedQuorums:           false,
		paused:                        false,
		pauseReasonV:                  "",
		lastOpStateUpdateTime:         time.Time{},
	}, nil
}

// TODO!
// Fix atBlock everywhere all queries and subscriptions etc...
// Fix error handling...
// Add USEFUL logging everywhere

// TODO
// error handle here?
func getEventID(abi *abi.ABI, eventName string) common.Hash {
	return abi.Events[eventName].ID
}

func (osu *OpStateUpdater) startAsyncOpStateUpdater(ctx context.Context, sendNewOpTaskC chan types.SendNewOpTaskType, errorC chan error) error {
	osu.logger.Infof("Starting aggregator startAsyncOpStateUpdater")

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

	lastCompletedOpTaskCreatedBlock, err := osu.ethRpc.AvsReader.LastCompletedOpTaskCreatedBlock(context.Background())
	if err != nil {
		osu.logger.Error("Aggregator failed to get LastCompletedOpTaskCreatedBlock", "err", err)
		return err
	}

	if lastCompletedOpTaskCreatedBlock == 0 {
		// Since we don't have an opTask yet
		// We want to reset/refresh all tracked quorums and then create an opTask
		// We do this as soon as we enter the main loop and then into the control loop
		osu.triggerOpStateUpdate = true
		osu.resetTrackedQuorums = true

		// Wait here till atleast 1 operator has registered
		// So that on local testnet the finalizer can register
		currentBlock, err := osu.ethRpc.Clients.EthHttpClient.BlockNumber(context.Background())
		if err != nil {
			osu.logger.Error("Cannot get current block number", "err", err)
			return err
		}
		Ids := make(map[sdktypes.OperatorId]bool)
		for _, quorum := range types.TRACKED_QUORUM_NUMBERS {
			operatorIds, err := osu.ethRpc.Clients.AvsRegistryChainReader.GetOperatorIdList(&bind.CallOpts{}, quorum, uint32(currentBlock))
			if err != nil {
				osu.logger.Error("Cannot get current operator list", "err", err)
				return err
			}
			for _, opId := range operatorIds {
				Ids[opId] = true
			}
		}

		if len(Ids) == 0 {
			eventC := make(chan *stakeRegistry.ContractStakeRegistryOperatorStakeUpdate)
			fromBlock := currentBlock + 1
			sub, err := osu.ethRpc.AvsSubscriber.SubscribeToOperatorStakeUpdate(bind.WatchOpts{Start: &fromBlock}, eventC)
			if err != nil {
				return fmt.Errorf("failed to subscribe to logs: %v", err)
			}
			defer sub.Unsubscribe()

		waitForRegisterLoop:
			for {
				select {
				case <-ctx.Done():
					return ctx.Err()
				case err := <-sub.Err():
					osu.logger.Fatalf("Subscription error: %v", err)
				case event := <-eventC:
					osu.logger.Info("Received operatorStakeUpdate event: %v", eventC)

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
		}
	} else {
		// If atleast 1 opTask was completed we have an opState
		// So we get the lastCompletedOpTaskCreatedBlock and start reading
		// events from that point on...
		osu.checkpointedBlock = lastCompletedOpTaskCreatedBlock
		osu.atBlock = lastCompletedOpTaskCreatedBlock

		osu.updateOpStates()
	}

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
				getEventID(taskmanagerAbi, "PauseTrackingOpState"), getEventID(taskmanagerAbi, "OpTaskCompleted"), /*getEventID(boundTaskManagerContract.contract, "ResumeTrackingOpState"),*/ /*getEventID(boundTaskManagerContract.contract, "OpTaskCompleted")*/
			},
		},
	}

	for {

	controlLoop:
		for {
			switch {
			case osu.paused == true:
				{
					eventC := make(chan *taskmanager.ContractFinalizerTaskManagerResumeTrackingOpState)
					sub, err := osu.ethRpc.AvsSubscriber.SubscribeToResumeTrackingOpState(eventC, uint64(osu.atBlock+1))
					if err != nil {
						return fmt.Errorf("failed to subscribe to logs: %v", err)
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
							return ctx.Err()
						case err := <-sub.Err():
							osu.logger.Fatalf("Subscription error: %v", err)
						case event := <-eventC:
							osu.logger.Info("Received resume event: %v", eventC)
							if event.ResetTrackedQuorums {
								osu.resetTrackedQuorums = true
								osu.triggerOpStateUpdate = true
							} else {

								lastCompletedOpTaskCreatedBlock, err := osu.ethRpc.AvsReader.LastCompletedOpTaskCreatedBlockAtBlock(context.Background(), uint64(event.Raw.BlockNumber))
								if err != nil {
									osu.logger.Error("Aggregator failed to get LastCompletedOpTaskCreatedBlockAtBlock", "err", err)
									return err
								}
								osu.checkpointedBlock = lastCompletedOpTaskCreatedBlock
								osu.atBlock = uint32(event.Raw.BlockNumber)

								osu.updateOpStates()

							}
							osu.paused = false
							break watchForResumeLoop
						}
					}
					sub.Unsubscribe()
				}
			case osu.triggerOpStateUpdate:
				{
					// Timer here to check if the last triggerOpStateUpdate was not too recent
					// We don't want sm1 changing stakes in the early stages of the project
					// to cost us a lot of eth
					if time.Since(osu.lastOpStateUpdateTime) < time.Hour {
						osu.paused = true
						continue
					}

					osu.logger.Info("Triggering OpState update")

					// Only two ways of triggerOpStateUpdate
					// Either via resetTrackedQuorums or via checkQuorumThresholds
					// The former doesn't require quorumDiff and quorumsToBeUpdated
					// The later will always have it
					if osu.resetTrackedQuorums {
						osu.logger.Info("Resetting (updating) all tracked quorum's OpState")
						err := osu.UpdateStakeFull()
						if err != nil {
							osu.logger.Error("Failed to UpdateStakeFull")
							osu.paused = true
							continue
						}
						osu.resetTrackedQuorums = false
					} else {
						err := osu.UpdateStake()
						if err != nil {
							osu.logger.Error("Failed to UpdateStake")
							osu.paused = true
							continue
						}
					}

					osu.operatorIdsToBeUpdated = make(map[sdktypes.OperatorId]bool)
					osu.quorumDiff = make(map[sdktypes.QuorumNum]types.QuorumStakeDiff)
					osu.quorumsToBeUpdated = make(map[sdktypes.QuorumNum]bool)

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
						return ctx.Err()
					}

					if sendNewOpTaskReturn.SendNewOpTaskError != nil {
						osu.paused = true
					} else {
						eventC := make(chan *taskmanager.ContractFinalizerTaskManagerOpTaskCompleted)
						sub, err := osu.ethRpc.AvsSubscriber.SubscribeToOpTaskCompleted(uint64(sendNewOpTaskReturn.OpTask.TaskCreatedBlock), eventC)
						if err != nil {
							return fmt.Errorf("failed to subscribe to logs: %v", err)
						}
						defer sub.Unsubscribe()

						timer := time.NewTimer(time.Hour)
						defer timer.Stop()

					watchForOpTaskCompletedLoop:
						for {
							select {
							case <-ctx.Done():
								return ctx.Err()
							case err := <-sub.Err():
								osu.logger.Fatalf("Subscription error: %v", err)
							case <-timer.C:
								osu.paused = true
								break watchForOpTaskCompletedLoop
							case event := <-eventC:
								{
									osu.logger.Info("Received OpTaskCompleted event: %v", eventC)

									if sendNewOpTaskReturn.OpTask.TaskCreatedBlock < event.TaskIndex {
										osu.logger.Error("Failed to get the OpTaskCompleted event", "TaskIndex", sendNewOpTaskReturn.OpTask.TaskCreatedBlock)
										osu.paused = true
										break watchForOpTaskCompletedLoop
									}

									// This branch is to account for the case where
									// a task is completed in a block and another task is created
									// in the same block and then that one is also completed in the same block
									if sendNewOpTaskReturn.OpTask.TaskCreatedBlock > event.TaskIndex {
										continue
									}

									osu.logger.Info("Got the expected OpTaskCompleted event", "TaskIndex", sendNewOpTaskReturn.OpTask.TaskCreatedBlock)

									osu.checkpointedBlock = sendNewOpTaskReturn.OpTask.TaskCreatedBlock
									osu.atBlock = sendNewOpTaskReturn.OpTask.TaskCreatedBlock

									break watchForOpTaskCompletedLoop
								}
							}
						}
						sub.Unsubscribe()
						osu.triggerOpStateUpdate = false

						osu.updateOpStates()
					}

				}
			default:
				break controlLoop
			}
		}

		// Subscribe
		logs := make(chan ethtypes.Log)
		iquery := baseQuery
		iquery.FromBlock = big.NewInt(int64(osu.atBlock + 1))


		var sub ethereum.Subscription
		var err error
		const maxRetries = 5
		const retryDelay = time.Minute

		// Loop to retry subscription on error
		for attempt := 0; attempt < maxRetries; attempt++ {
			sub, err = osu.ethRpc.Clients.EthWsClient.SubscribeFilterLogs(context.Background(), iquery, logs)
			if err == nil {
				break // Successfully subscribed, exit loop
			}

			osu.logger.Error("Failed to get new head from substrate, retrying...", "err", err, "attempt", attempt+1)
			select {
			case <-ctx.Done():
				return nil
			case <-time.After(retryDelay): // Wait before retrying
				continue
			}
		}
		if err != nil {
			return fmt.Errorf("failed to subscribe to new heads from substrate after %d attempts: %w", maxRetries, err)
		}
		defer sub.Unsubscribe()

		// watch the subscription
	watchForTriggersLoop:
		for {
			select {
			case <-ctx.Done():
				return ctx.Err()
			case err := <-sub.Err():
				// We do this so because the subscription will start from  atBlock+1
				osu.atBlock = osu.atBlock - 1
				osu.logger.Error("watchForTriggersLoop subscription error: %v - retrying atBlock %v", err, osu.atBlock)
				break watchForTriggersLoop
			case vLog := <-logs:
				fmt.Printf("Received log: %+v\n", vLog)
				switch {
				case vLog.Address == delegationManagerContractAddress && vLog.Topics[0] == getEventID(delegationManagerAbi, "OperatorSharesIncreased"):
					{
						fmt.Printf("Event %s from contract %s\n", "OperatorSharesIncreased", vLog.Address.Hex())
						osu.atBlock = uint32(vLog.BlockNumber)
						// Process the log here based on event signature and ABI
						ContractDelegationManagerOperatorSharesIncreased, err := osu.ethRpc.AvsReader.AvsServiceBindings.DelegationManager.ContractDelegationManagerFilterer.ParseOperatorSharesIncreased(vLog)
						if err != nil {
							osu.logger.Error("Failed to ParseOperatorSharesIncreased - Pausing trackingOpState", "err", err)
							osu.paused = true
							break watchForTriggersLoop
						}
						osu.processOpDelegationStateChange(ContractDelegationManagerOperatorSharesIncreased.Operator, uint32(vLog.BlockNumber))

						if osu.paused || osu.triggerOpStateUpdate {
							break watchForTriggersLoop
						}
					}
				case vLog.Address == delegationManagerContractAddress && vLog.Topics[0] == getEventID(delegationManagerAbi, "OperatorSharesDecreased"):
					{
						fmt.Printf("Event %s from contract %s\n", "OperatorSharesDecreased", vLog.Address.Hex())
						osu.atBlock = uint32(vLog.BlockNumber)
						// Process the log here based on event signature and ABI
						ContractDelegationManagerOperatorSharesDecreased, err := osu.ethRpc.AvsReader.AvsServiceBindings.DelegationManager.ContractDelegationManagerFilterer.ParseOperatorSharesDecreased(vLog)
						if err != nil {
							osu.logger.Error("Failed to ParseOperatorSharesDecreased - Pausing trackingOpState", "err", err)
							osu.paused = true
							break watchForTriggersLoop
						}
						osu.processOpDelegationStateChange(ContractDelegationManagerOperatorSharesDecreased.Operator, uint32(vLog.BlockNumber))

						if osu.paused || osu.triggerOpStateUpdate {
							break watchForTriggersLoop
						}
					}
				case vLog.Address == stakeRegistryContractAddress && vLog.Topics[0] == getEventID(stakeRegistryAbi, "OperatorStakeUpdate"):
					{
						fmt.Printf("Event %s from contract %s\n", "OperatorStakeUpdate", vLog.Address.Hex())
						osu.atBlock = uint32(vLog.BlockNumber)
						// Process the log here based on event signature and ABI
						ContractStakeRegistryOperatorStakeUpdate, err := osu.ethRpc.AvsReader.AvsServiceBindings.StakeRegistry.ContractStakeRegistryFilterer.ParseOperatorStakeUpdate(vLog)
						if err != nil {
							osu.logger.Error("Failed to ParseOperatorStakeUpdate - Pausing trackingOpState", "err", err)
							osu.paused = true
							break watchForTriggersLoop
						}
						osu.processOpStakeRegistryStateChange(ContractStakeRegistryOperatorStakeUpdate.OperatorId, sdktypes.QuorumNum(ContractStakeRegistryOperatorStakeUpdate.QuorumNumber), ContractStakeRegistryOperatorStakeUpdate.Stake, uint32(vLog.BlockNumber))

						if osu.paused || osu.triggerOpStateUpdate {
							break watchForTriggersLoop
						}
					}
				case vLog.Address == stakeRegistryContractAddress && vLog.Topics[0] == getEventID(stakeRegistryAbi, "StrategyMultiplierUpdated"):
					{
						fmt.Printf("Event %s from contract %s\n", "StrategyMultiplierUpdated", vLog.Address.Hex())
						osu.atBlock = uint32(vLog.BlockNumber)
						// Process the log here based on event signature and ABI
						ContractStakeRegistryStrategyMultiplierUpdated, err := osu.ethRpc.AvsReader.AvsServiceBindings.StakeRegistry.ContractStakeRegistryFilterer.ParseStrategyMultiplierUpdated(vLog)
						if err != nil {
							osu.logger.Error("Failed to ParseStrategyMultiplierUpdated - Pausing trackingOpState", "err", err)
							osu.paused = true
							break watchForTriggersLoop
						}
						osu.logger.Info("Pausing trackingOpState upon event trigger", "trigger", ContractStakeRegistryStrategyMultiplierUpdated)
						osu.paused = true
						break watchForTriggersLoop
					}
				case vLog.Address == stakeRegistryContractAddress && vLog.Topics[0] == getEventID(stakeRegistryAbi, "MinimumStakeForQuorumUpdated"):
					{
						fmt.Printf("Event %s from contract %s\n", "MinimumStakeForQuorumUpdated", vLog.Address.Hex())
						osu.atBlock = uint32(vLog.BlockNumber)
						// Process the log here based on event signature and ABI
						ContractStakeRegistryMinimumStakeForQuorumUpdated, err := osu.ethRpc.AvsReader.AvsServiceBindings.StakeRegistry.ContractStakeRegistryFilterer.ParseMinimumStakeForQuorumUpdated(vLog)
						if err != nil {
							osu.logger.Error("Failed to ParseMinimumStakeForQuorumUpdated - Pausing trackingOpState", "err", err)
							osu.paused = true
							break watchForTriggersLoop
						}
						osu.logger.Info("Pausing trackingOpState upon event trigger", "trigger", ContractStakeRegistryMinimumStakeForQuorumUpdated)
						osu.paused = true
						break watchForTriggersLoop
					}
				case vLog.Address == taskManagerContractAddress && vLog.Topics[0] == getEventID(taskmanagerAbi, "PauseTrackingOpState"):
					{
						fmt.Printf("Event %s from contract %s\n", "PauseTrackingOpState", vLog.Address.Hex())
						osu.atBlock = uint32(vLog.BlockNumber)
						// Process the log here based on event signature and ABI
						ContractFinalizerTaskManagerPauseTrackingOpState, err := osu.ethRpc.AvsReader.AvsServiceBindings.TaskManager.ContractFinalizerTaskManagerFilterer.ParsePauseTrackingOpState(vLog)
						if err != nil {
							osu.logger.Error("Failed to ParsePauseTrackingOpState - Pausing trackingOpState", "err", err)
							osu.paused = true
							break watchForTriggersLoop
						}
						osu.logger.Info("Pausing trackingOpState upon event trigger", "trigger", ContractFinalizerTaskManagerPauseTrackingOpState)
						osu.paused = true
						break watchForTriggersLoop
					}
				case vLog.Address == taskManagerContractAddress && vLog.Topics[0] == getEventID(taskmanagerAbi, "OpTaskCompleted"):
					{
						fmt.Printf("Event %s from contract %s\n", "OpTaskCompleted", vLog.Address.Hex())
						osu.atBlock = uint32(vLog.BlockNumber)
						// Process the log here based on event signature and ABI
						ContractFinalizerTaskManagerOpTaskCompleted, err := osu.ethRpc.AvsReader.AvsServiceBindings.TaskManager.ContractFinalizerTaskManagerFilterer.ParseOpTaskCompleted(vLog)
						if err != nil {
							osu.logger.Error("Failed to OpTaskCompleted - Pausing trackingOpState", "err", err)
							osu.paused = true
							break watchForTriggersLoop
						}

						osu.logger.Info("Received OpTaskCompleted", "event", ContractFinalizerTaskManagerOpTaskCompleted)

						lastCompletedOpTaskCreatedBlock, err := osu.ethRpc.AvsReader.LastCompletedOpTaskCreatedBlockAtBlock(context.Background(), vLog.BlockNumber)
						if err != nil {
							osu.logger.Error("Aggregator failed to get LastCompletedOpTaskCreatedBlockAtBlock", "err", err)
							return err
						}
						osu.checkpointedBlock = lastCompletedOpTaskCreatedBlock

						osu.updateOpStates()
						break watchForTriggersLoop
					}
				}
			}
		}
		sub.Unsubscribe()
	}

}

func (osu *OpStateUpdater) updateOperatorIdsToBeUpdated() error {

	osu.operatorIdsToBeUpdated = make(map[sdktypes.OperatorId]bool)

	for operatorId, _ := range osu.checkpointedAvsOpState {
		for _, quorum := range types.TRACKED_QUORUM_NUMBERS {
			updatedStake := osu.currentOpState[operatorId].StakePerQuorum[quorum]
			prevStake := osu.checkpointedAvsOpState[operatorId].StakePerQuorum[quorum]
			if prevStake.Cmp(updatedStake) != 0 {
				osu.operatorIdsToBeUpdated[operatorId] = true
			}
		}
	}

	for operatorId, _ := range osu.currentOpState {
		for _, quorum := range types.TRACKED_QUORUM_NUMBERS {
			updatedStake := osu.currentOpState[operatorId].StakePerQuorum[quorum]
			prevStake := osu.checkpointedAvsOpState[operatorId].StakePerQuorum[quorum]
			if prevStake.Cmp(updatedStake) != 0 {
				osu.operatorIdsToBeUpdated[operatorId] = true
			}
		}
	}

	return nil
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
	osu.getCheckpointedOpState(osu.checkpointedBlock)
	osu.getCurrentOpState(osu.atBlock)
	osu.checkQuorumThresholds()
	if osu.triggerOpStateUpdate {
		osu.paused = true
	}
	if osu.isAnyQuorumInCheckpointedAvsStateEmpty() {
		osu.paused = true
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
		// TODO: how should we handle such an error?
		osu.logger.Fatal("AggregatorService failed to get operators state from avs registry", "err", err, "blockNumber", osu.checkpointedBlock)
	}
	osu.checkpointedAvsOpState = checkpointedAvsOpState
	return nil
}

func (osu *OpStateUpdater) getCurrentOpState(atBlock sdktypes.BlockNum) error {
	Ids := make(map[sdktypes.OperatorId]bool)
	for _, quorum := range types.TRACKED_QUORUM_NUMBERS {
		operatorIds, err := osu.ethRpc.Clients.AvsRegistryChainReader.GetOperatorIdList(&bind.CallOpts{}, quorum, uint32(atBlock))
		if err != nil {
			osu.logger.Error("Cannot get current operator list", "err", err)
			return err
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
		osu.logger.Error("Cannot get operator addresses from operator ids")
		return err
	}

	opState, err := osu.ethRpc.AvsReader.GetTypedOperatorsStakesForQuorumAtBlock(context.Background(), osu.ethRpc.AvsReader.AvsServiceBindings.RegistryCoordinatorAddress, types.TRACKED_QUORUM_NUMBERS, operatorAddresses, osu.atBlock)
	if err != nil {
		osu.logger.Fatal("Aggregator failed to get opState", "err", err)
	}

	osu.currentOpState = opState
	return nil
}

func (osu *OpStateUpdater) isAnyQuorumInCheckpointedAvsStateEmpty() bool {
	isAnyQuorumEmpty := false
	for _, quorum := range types.TRACKED_QUORUM_NUMBERS {
		stake := big.NewInt(0)
		for _, opState := range osu.checkpointedAvsOpState {
			stake.Add(stake, opState.StakePerQuorum[quorum])
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

	if isQuorumTracked {
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
		osu.logger.Fatal("Aggregator failed to get operatorId", "err", err)
	}
	if bytes.Equal(operatorId[:], types.ZERO_OPERATOR_ID[:]) {
		return nil
	}
	if _, ok := osu.currentOpState[operatorId]; ok {
		opStateUpdate, err := osu.ethRpc.AvsReader.GetTypedOperatorsStakesForQuorumAtBlock(context.Background(), osu.ethRpc.AvsReader.AvsServiceBindings.RegistryCoordinatorAddress, types.TRACKED_QUORUM_NUMBERS, []common.Address{operator}, blockNumber)
		if err != nil {
			osu.logger.Fatal("Aggregator failed to get opStateUpdate", "err", err)
		}
		osu.applyOpStateUpdate(opStateUpdate)

		osu.operatorIdsToBeUpdated[operatorId] = true
		osu.checkQuorumThresholds()
	}

	return nil
}

func (osu *OpStateUpdater) applyOpStateUpdate(opStateUpdate map[sdktypes.OperatorId]types.OperatorAvsState) error {
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
	return nil
}

func (osu *OpStateUpdater) checkQuorumThresholds() error {
	osu.makeQuorumsStakeDiff()
	osu.checkQuorumsStakeDiff()
	return nil
}

func (osu *OpStateUpdater) makeQuorumsStakeDiff() error {

	osu.quorumDiff = make(map[sdktypes.QuorumNum]types.QuorumStakeDiff)

	for operatorId, _ := range osu.checkpointedAvsOpState {
		for _, quorum := range types.TRACKED_QUORUM_NUMBERS {
			updatedStake := osu.currentOpState[operatorId].StakePerQuorum[quorum]
			prevStake := osu.checkpointedAvsOpState[operatorId].StakePerQuorum[quorum]
			if prevStake.Cmp(updatedStake) == 1 {
				diff := big.NewInt(0).Sub(prevStake, updatedStake)
				osu.quorumDiff[quorum].NegDiff.Add(osu.quorumDiff[quorum].NegDiff, diff)
			}
		}
	}

	for operatorId, _ := range osu.currentOpState {
		for _, quorum := range types.TRACKED_QUORUM_NUMBERS {
			updatedStake := osu.currentOpState[operatorId].StakePerQuorum[quorum]
			prevStake := osu.checkpointedAvsOpState[operatorId].StakePerQuorum[quorum]
			if prevStake.Cmp(updatedStake) == -1 {
				diff := big.NewInt(0).Sub(updatedStake, prevStake)
				osu.quorumDiff[quorum].PosDiff.Add(osu.quorumDiff[quorum].PosDiff, diff)
			}
		}
	}
	return nil
}

func (osu *OpStateUpdater) checkQuorumsStakeDiff() error {

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
	return nil
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
		osu.logger.Error("Cannot get current block number", "err", err)
		return err
	}

	var operatorAddresses [][]common.Address
	for _, qn := range types.TRACKED_QUORUM_NUMBERS {
		operatorIds, err := osu.ethRpc.Clients.AvsRegistryChainReader.GetOperatorIdList(&bind.CallOpts{}, qn, uint32(currentBlock))
		if err != nil {
			osu.logger.Error("Cannot get current operator list", "err", err)
			return err
		}

		operatorAdrresses0, err := osu.ethRpc.AvsReader.GetOperatorsFromIds(&bind.CallOpts{}, osu.ethRpc.AvsReader.AvsServiceBindings.RegistryCoordinatorAddress, operatorIds)
		if err != nil {
			osu.logger.Error("Cannot get operator addresses from operator ids")
			return err
		}

		sort.Slice(operatorAdrresses0, func(i, j int) bool {
			a := big.NewInt(0).SetBytes(operatorAdrresses0[i][:])
			b := big.NewInt(0).SetBytes(operatorAdrresses0[j][:])
			return a.Cmp(b) < 0
		})

		operatorAddresses = append(operatorAddresses, operatorAdrresses0)

	}

	_, err = osu.ethRpc.Clients.AvsRegistryChainWriter.UpdateStakesOfEntireOperatorSetForQuorums(context.Background(), operatorAddresses, types.TRACKED_QUORUM_NUMBERS)
	if err != nil {
		osu.logger.Error("Cannot update stakes", "err", err)
		return err
	}

	osu.logger.Info("Operator stakes update successfull", "addresses", operatorAddresses)

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
		osu.logger.Error("Cannot get operator addresses from operator ids")
		return err
	}

	_, err = osu.ethRpc.Clients.AvsRegistryChainWriter.UpdateStakesOfOperatorSubsetForAllQuorums(context.Background(), operatorAdrresses)
	if err != nil {
		osu.logger.Error("Cannot update stakes", "err", err)
		return err
	}

	osu.logger.Info("Operator stakes update successfull", "addresses", operatorAdrresses)

	return nil
}

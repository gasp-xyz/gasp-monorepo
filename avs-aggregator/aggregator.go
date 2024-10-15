package aggregator

import (
	"context"
	"encoding/hex"
	"errors"
	// "math/big"
	"sync"
	"time"
	"fmt"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"

	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/services/avsregistry"
	blsagg "github.com/Layr-Labs/eigensdk-go/services/bls_aggregation"
	oprsinfoserv "github.com/Layr-Labs/eigensdk-go/services/operatorsinfo"
	sdktypes "github.com/Layr-Labs/eigensdk-go/types"
	"github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/core"
	"github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/core/chainio"
	"github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/types"

	taskmanager "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/FinalizerTaskManager"

	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
	gsrpcrpcchain "github.com/centrifuge/go-substrate-rpc-client/v4/rpc/chain"
	gsrpctypes "github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

// Aggregator sends tasks (numbers to square) onchain, then listens for operator signed TaskResponses.
// It aggregates responses signatures, and if any of the TaskResponses reaches the QuorumThresholdPercentage for each quorum
// (currently we only use a single quorum of the ERC20Mock token), it sends the aggregated TaskResponse and signature onchain.
//
// The signature is checked in the BLSSignatureChecker.sol contract, which expects a
//
//	struct NonSignerStakesAndSignature {
//		uint32[] nonSignerQuorumBitmapIndices;
//		BN254.G1Point[] nonSignerPubkeys;
//		BN254.G1Point[] quorumApks;
//		BN254.G2Point apkG2;
//		BN254.G1Point sigma;
//		uint32[] quorumApkIndices;
//		uint32[] totalStakeIndices;
//		uint32[][] nonSignerStakeIndices; // nonSignerStakeIndices[quorumNumberIndex][nonSignerIndex]
//	}
//
// A task can only be responded onchain by having enough operators sign on it such that their stake in each quorum reaches the QuorumThresholdPercentage.
// In order to verify this onchain, the Registry contracts store the history of stakes and aggregate pubkeys (apks) for each operators and each quorum. These are
// updated everytime an operator registers or deregisters with the BLSRegistryCoordinatorWithIndices.sol contract, or calls UpdateStakes() on the StakeRegistry.sol contract,
// after having received new delegated shares or having delegated shares removed by stakers queuing withdrawals. Each of these pushes to their respective datatype array a new entry.
//
// This is true for quorumBitmaps (represent the quorums each operator is opted into), quorumApks (apks per quorum), totalStakes (total stake per quorum), and nonSignerStakes (stake per quorum per operator).
// The 4 "indices" in NonSignerStakesAndSignature basically represent the index at which to fetch their respective data, given a blockNumber at which the task was created.
// Note that different data types might have different indices, since for eg QuorumBitmaps are updated for operators registering/deregistering, but not for UpdateStakes.
// Thankfully, we have deployed a helper contract BLSOperatorStateRetriever.sol whose function getCheckSignaturesIndices() can be used to fetch the indices given a block number.
//
// The 4 other fields nonSignerPubkeys, quorumApks, apkG2, and sigma, however, must be computed individually.
// apkG2 and sigma are just the aggregated signature and pubkeys of the operators who signed the task response (aggregated over all quorums, so individual signatures might be duplicated).
// quorumApks are the G1 aggregated pubkeys of the operators who signed the task response, but one per quorum, as opposed to apkG2 which is summed over all quorums.
// nonSignerPubkeys are the G1 pubkeys of the operators who did not sign the task response, but were opted into the quorum at the blocknumber at which the task was created.
// Upon sending a task onchain (or receiving a NewTaskCreated Event if the tasks were sent by an external task generator), the aggregator can get the list of all operators opted into each quorum at that
// block number by calling the getOperatorState() function of the BLSOperatorStateRetriever.sol contract.
type Aggregator struct {
	logger           sdklogging.Logger
	serverIpPortAddr string
	blockPeriod      uint32
	blockPeriodOpsTask uint32
	expiration       uint32
	ethRpc           *chainio.EthRpc
	// aggregation related fields
	blsAggregationService   blsagg.BlsAggregationService
	tasks                   map[sdktypes.TaskId]interface{}
	tasksMu                 sync.RWMutex
	taskResponses           map[sdktypes.TaskId]map[sdktypes.TaskResponseDigest]interface{}
	taskResponsesMu         sync.RWMutex
	substrateClient         gsrpc.SubstrateAPI
	taskResponseWindowBlock uint32
	asyncOpStateUpdaterError error
	retryNumber             uint8
	maxRetryNumber          uint8

	kicker  *Kicker
	opStateUpdater *OpStateUpdater
}

const waitForReceipt = bool(true)

// NewAggregator creates a new Aggregator with the provided config.
func NewAggregator(c *Config) (*Aggregator, error) {
	logger, err := sdklogging.NewZapLogger(c.LogLevel)
	if err != nil {
		return nil, err
	}

	logger.Debug("creating new aggregator", "config", c)

	ethRpc, err := chainio.NewEthRpc(
		c.AvsRegistryCoordinatorAddr,
		c.EthRpcUrl,
		c.EthWsUrl,
		c.SignerFn,
		c.Address,
		"mangata-finalizer",
		"0.0.0.0:8888",
		logger,
	)
	if err != nil {
		logger.Error("Failed to create EthRpc clients", "err", err)
		return nil, err
	}

	chainId, err := ethRpc.AvsReader.AvsServiceBindings.EthClient.ChainID(context.Background())
	if err != nil {
		logger.Error("Cannot get chainId", "err", err)
		return nil, err
	}
	if c.ChainId.Cmp(chainId) != 0 {
		logger.Error("chainId in arguments does not match ethRpcClient chain id", "eth chainId", chainId, "config chainId", c.ChainId)
		return nil, errors.New("config.chainId & ethRpcClient.chainId mismatch")
	}

	taskResponseWindowBlock, err := ethRpc.AvsWriter.AvsContractBindings.TaskManager.TaskResponseWindowBlock(&bind.CallOpts{})
	if err != nil {
		logger.Error("Cannot get taskChallengeWindowBlock from TaskManager contract", "err", err)
		return nil, err
	}

	operatorPubkeysService := oprsinfoserv.NewOperatorsInfoServiceInMemory(
		context.Background(),
		ethRpc.Clients.AvsRegistryChainSubscriber,
		ethRpc.Clients.AvsRegistryChainReader,
		nil,
		oprsinfoserv.Opts{},
		logger,
	)
	avsRegistryService := avsregistry.NewAvsRegistryServiceChainCaller(ethRpc.Clients.AvsRegistryChainReader, operatorPubkeysService, logger)
	blsAggregationService := blsagg.NewBlsAggregatorService(avsRegistryService, c.DebounceRpc, logger)

	substrateRpc, err := gsrpc.NewSubstrateAPI(c.SubstrateWsRpcUrl)
	if err != nil {
		logger.Error("Cannot create substrate RPC", "url", c.SubstrateWsRpcUrl, "err", err)
		return nil, err
	}

	var kicker *Kicker

	if c.EnableKicker {
		kicker, err = NewKicker(logger, *ethRpc, uint32(c.KickPeriod), uint32(c.BlockPeriod))
		if err != nil {
			logger.Error("Cannot create operator active list filter", "err", err)
			return nil, err
		}
	}

	opStateUpdater, err := NewOpStateUpdater(logger, ethRpc, c.MinOpUpdateInterval)
	if err != nil {
		logger.Error("Cannot create operator stakes updateer", "err", err)
		return nil, err
	}

	return &Aggregator{
		logger:                  logger,
		serverIpPortAddr:        c.ServerAddressPort,
		ethRpc:                  ethRpc,
		blsAggregationService:   blsAggregationService,
		tasks:                   make(map[sdktypes.TaskId]interface{}),
		taskResponses:           make(map[sdktypes.TaskId]map[sdktypes.TaskResponseDigest]interface{}),
		substrateClient:         *substrateRpc,
		taskResponseWindowBlock: taskResponseWindowBlock,
		blockPeriod:             uint32(c.BlockPeriod),
		blockPeriodOpsTask:		 uint32(c.BlockPeriodOpsTask),
		kicker:                  kicker,
		opStateUpdater:			 opStateUpdater,
		expiration:              uint32(c.Expiration),
	}, nil
}

func (agg *Aggregator) Start(ctx context.Context) error {
	agg.logger.Infof("Starting aggregator.")
	agg.logger.Infof("Starting aggregator rpc server.")
	go agg.startServer(ctx)

	err := agg.checkAndProcessPendingTasks()
	if err != nil{
		return fmt.Errorf("Aggregator failed to checkAndProcessPendingTasks: err: %v", err)
	}

	sendNewOpTaskC := make(chan types.SendNewOpTaskType)
	asyncOpStateUpdaterErrorC := make(chan error)
	go agg.opStateUpdater.startAsyncOpStateUpdater(ctx, sendNewOpTaskC, asyncOpStateUpdaterErrorC)

	var sub *gsrpcrpcchain.NewHeadsSubscription
	const maxRetries = 5
	const retryDelay = time.Minute

	// Loop to retry subscription on error
	for attempt := 0; attempt < maxRetries; attempt++ {
		sub, err = agg.substrateClient.RPC.Chain.SubscribeNewHeads()
		if err == nil {
			break // Successfully subscribed, exit loop
		}

		agg.logger.Error("Failed to get new head from substrate, retrying...", "err", err, "attempt", attempt+1)
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

	for {
		select {
		case asyncOpStateUpdaterError := <-asyncOpStateUpdaterErrorC:
			agg.asyncOpStateUpdaterError = asyncOpStateUpdaterError
			agg.logger.Error("asyncOpStateUpdater has crashed with the following error", "err", asyncOpStateUpdaterError)
		case <-ctx.Done():
			return nil
		case head := <-sub.Chan():
			// agg.logger.Info("Received new substrate header", "head.Number", head.Number)
			err := agg.maybeSendNewRdTask(uint32(head.Number))
			if err != nil {
				// We return here because if we can't send out an rdTask then we should stop the aggregator
				return err
			}
		case sendNewOpTask := <-sendNewOpTaskC:
			OpTask, err := agg.startNewOpTask()
			if err != nil {
				// we log the errors inside sendNewTask() so here we just continue to the next task
				sendNewOpTask.SendNewOpTaskReturnC <- types.SendNewOpTaskReturn{
					SendNewOpTaskError: err,
				}
				// Since we want serial processingwe abort here
				// The above errC is unnecessary
				return err
			}
			sendNewOpTask.SendNewOpTaskReturnC <- types.SendNewOpTaskReturn{
				OpTask: OpTask,
				SendNewOpTaskError: nil,
			}
		case err := <-sub.Err():
			agg.logger.Error("Subscription error, retrying subscription...", "err", err)
			// Reset the subscription on error
			sub.Unsubscribe() // Unsubscribe before retrying

			// Retry subscription with a maximum number of attempts
			for attempt := 0; attempt < maxRetries; attempt++ {
				sub, err = agg.substrateClient.RPC.Chain.SubscribeNewHeads()
				if err == nil {
					break // Successfully subscribed, exit loop
				}

				agg.logger.Error("Failed to get new head from substrate, retrying...", "err", err, "attempt", attempt+1)
				select {
				case <-ctx.Done():
					return nil
				case <-time.After(retryDelay): // Wait before retrying
					continue
				}
			}

			if err != nil {
				return fmt.Errorf("failed to resubscribe to new heads after %d attempts: %w", maxRetries, err)
			}
		}
		
	}
}

func (agg *Aggregator) checkAndProcessPendingTasks() (error) {

	isTaskPending, err := agg.ethRpc.AvsReader.IsTaskPending(context.Background())
	if err != nil {
		return fmt.Errorf("Aggregator failed to IsTaskPending: err: %v", err)
	}
	if !isTaskPending {
		return nil
	}

	latestOpTaskNum, err := agg.ethRpc.AvsReader.LatestOpTaskNum(context.Background())
	if err != nil {
		return fmt.Errorf("Aggregator failed to LatestOpTaskNum: err: %v", err)
	}
	latestRdTaskNum, err := agg.ethRpc.AvsReader.LatestRdTaskNum(context.Background())
	if err != nil {
		return fmt.Errorf("Aggregator failed to LatestRdTaskNum: err: %v", err)
	}

	var isOpTaskPending bool
	var isRdTaskPending bool

	if latestOpTaskNum != 0 {
		latestOpTaskNum = latestOpTaskNum - 1
		latestOpTaskStatus, err := agg.ethRpc.AvsReader.IdToTaskStatus(context.Background(), sdktypes.TaskType(0), latestOpTaskNum)
		if err != nil {
			return fmt.Errorf("Aggregator failed to IdToTaskStatus: err: %v", err)
		}
		if latestOpTaskStatus == types.TASK_STATUS_INITIALIZED {
			isOpTaskPending = true
		}
	}

	if latestRdTaskNum != 0 {
		latestRdTaskNum = latestRdTaskNum - 1
		latestRdTaskStatus, err := agg.ethRpc.AvsReader.IdToTaskStatus(context.Background(), sdktypes.TaskType(1), latestRdTaskNum)
		if err != nil {
			return fmt.Errorf("Aggregator failed to IdToTaskStatus: err: %v", err)
		}
		if latestRdTaskStatus == types.TASK_STATUS_INITIALIZED {
			isRdTaskPending = true
		}
	}
	
	switch{
	case isOpTaskPending && isRdTaskPending:
		return fmt.Errorf("Both latestOpTaskStatus and latestRdTaskStatus are Pending")
	case !isOpTaskPending && !isRdTaskPending:
		return fmt.Errorf("Both latestOpTaskStatus and latestRdTaskStatus are NOT Pending but isTaskPending is true!")
	case isOpTaskPending && !isRdTaskPending:
		{
			lastOpTaskCreatedBlock, err := agg.ethRpc.AvsReader.LastOpTaskCreatedBlock(context.Background())
			if err != nil {
				return fmt.Errorf("Aggregator failed to LastOpTaskCreatedBlock: err: %v", err)
			}
			err = agg.getAndProcessPendingOpTask(latestOpTaskNum, lastOpTaskCreatedBlock)
			if err != nil {
				return fmt.Errorf("Aggregator failed to getAndProcessPendingRdTask: err: %v", err)
			}
		}
	case !isOpTaskPending && isRdTaskPending:
		{
			lastRdTaskCreatedBlock, err := agg.ethRpc.AvsReader.LastRdTaskCreatedBlock(context.Background())
			if err != nil {
				return fmt.Errorf("Aggregator failed to LastRdTaskCreatedBlock: err: %v", err)
			}
			err = agg.getAndProcessPendingRdTask(latestRdTaskNum, lastRdTaskCreatedBlock)
			if err != nil {
				return fmt.Errorf("Aggregator failed to getAndProcessPendingRdTask: err: %v", err)
			}
		}
	}
	return nil

}


func (agg *Aggregator) getAndProcessPendingOpTask(latestOpTaskNum uint32, lastOpTaskCreatedBlock uint32) (error) {
	EndBlock := uint64(lastOpTaskCreatedBlock)
	eventIter, err := agg.ethRpc.AvsReader.AvsServiceBindings.TaskManager.FilterNewOpTaskCreated(
		&bind.FilterOpts{Start: uint64(lastOpTaskCreatedBlock), End:&EndBlock, Context: context.Background()}, []uint32{latestOpTaskNum},
	)
	if err != nil {
		return fmt.Errorf("Aggregator failed to FilterNewOpTaskCreated: err: %v", err)
	}

	eventIterBool := eventIter.Next()
	if eventIterBool == false {
		return fmt.Errorf("Aggregator failed to find the opTask via FilterNewOpTaskCreated: latestOpTaskNum: %v, lastOpTaskCreatedBlock: %v", latestOpTaskNum, lastOpTaskCreatedBlock)
	}
	pendingOpTask := eventIter.Event.Task

	pendingOpTaskId := sdktypes.TaskId{
		TaskType: sdktypes.TaskType(0),
		TaskIndex: sdktypes.TaskIndex(latestOpTaskNum),
		}
	err = agg.processPendingOpTask(pendingOpTask, pendingOpTaskId)
	if err != nil {
		return fmt.Errorf("Aggregator failed to processPendingOpTask: err: %v", err)
	}
	return nil
}


func (agg *Aggregator) processPendingOpTask(newTask taskmanager.IFinalizerTaskManagerOpTask, taskId sdktypes.TaskId) (error) {

	success, err := agg.processCreatedOpTask(newTask, taskId)
	if err != nil{
		return fmt.Errorf("Aggregator failed to processCreatedOpTask: err: %v", err)
	}
	if success{
		return nil
	}

	newTask, err = agg.createAndProcessOpTask(2)
	if err != nil{
		return fmt.Errorf("Aggregator failed to createAndProcessOpTask: err: %v", err)
	}
	return nil
}

func (agg *Aggregator) getAndProcessPendingRdTask(latestRdTaskNum uint32, lastRdTaskCreatedBlock uint32) (error) {
	EndBlock := uint64(lastRdTaskCreatedBlock)
	eventIter, err := agg.ethRpc.AvsReader.AvsServiceBindings.TaskManager.FilterNewRdTaskCreated(
		&bind.FilterOpts{Start: uint64(lastRdTaskCreatedBlock), End:&EndBlock, Context: context.Background()}, []uint32{latestRdTaskNum},
	)
	if err != nil {
		return fmt.Errorf("Aggregator failed to FilterNewRdTaskCreated: err: %v", err)
	}

	eventIterBool := eventIter.Next()
	if eventIterBool == false {
		return fmt.Errorf("Aggregator failed to find the rdTask via FilterNewRdTaskCreated: latestRdTaskNum: %v, lastRdTaskCreatedBlock: %v", latestRdTaskNum, lastRdTaskCreatedBlock)
	}
	pendingRdTask := eventIter.Event.Task

	pendingRdTaskId := sdktypes.TaskId{
		TaskType: sdktypes.TaskType(1),
		TaskIndex: sdktypes.TaskIndex(latestRdTaskNum),
		}
	err = agg.processPendingRdTask(pendingRdTask, pendingRdTaskId)
	if err != nil {
		return fmt.Errorf("Aggregator failed to ProcessPendingRdTask: err: %v", err)
	}
	return nil

}

func (agg *Aggregator) processPendingRdTask(newTask taskmanager.IFinalizerTaskManagerRdTask, taskId sdktypes.TaskId) (error) {

	chainToUpdate := newTask.ChainId
	chainBatchIdToUpdate := newTask.BatchId

	success, err := agg.processCreatedRdTask(newTask, taskId)
	if err != nil{
		return fmt.Errorf("Aggregator failed to processCreatedRdTask: err: %v", err)
	}
	if success{
		return nil
	}

	err = agg.createAndProcessRdTask(chainToUpdate, chainBatchIdToUpdate, 2)
	if err != nil{
		return fmt.Errorf("Aggregator failed to createAndProcessRdTask: err: %v", err)
	}
	return nil
}

func (agg *Aggregator) sendAggregatedResponseToContract(blsAggServiceResp blsagg.BlsAggregationServiceResponse, expectedTaskId sdktypes.TaskId) (bool, error) {
	if blsAggServiceResp.Err != nil && blsAggServiceResp.Err != blsagg.TaskExpiredError {
		return false, fmt.Errorf("bls aggregation error, err: %v", blsAggServiceResp.Err)
	}
	nonSignerPubkeys := []taskmanager.BN254G1Point{}
	log := []string{}
	for _, nonSignerPubkey := range blsAggServiceResp.NonSignersPubkeysG1 {
		nonSignerPubkeys = append(nonSignerPubkeys, core.ConvertToBN254G1Point(nonSignerPubkey))
		id := nonSignerPubkey.GetOperatorID()
		log = append(log, hex.EncodeToString(id[:]))
	}
	agg.logger.Info("Response non signer ids", "nonSignerIds", log)
	quorumApks := []taskmanager.BN254G1Point{}
	for _, quorumApk := range blsAggServiceResp.QuorumApksG1 {
		quorumApks = append(quorumApks, core.ConvertToBN254G1Point(quorumApk))
	}
	nonSignerStakesAndSignature := taskmanager.IBLSSignatureCheckerNonSignerStakesAndSignature{
		NonSignerPubkeys:             nonSignerPubkeys,
		QuorumApks:                   quorumApks,
		ApkG2:                        core.ConvertToBN254G2Point(blsAggServiceResp.SignersApkG2),
		Sigma:                        core.ConvertToBN254G1Point(blsAggServiceResp.SignersAggSigG1.G1Point),
		NonSignerQuorumBitmapIndices: blsAggServiceResp.NonSignerQuorumBitmapIndices,
		QuorumApkIndices:             blsAggServiceResp.QuorumApkIndices,
		TotalStakeIndices:            blsAggServiceResp.TotalStakeIndices,
		NonSignerStakeIndices:        blsAggServiceResp.NonSignerStakeIndices,
	}

	// We can hard expect that here we will get only what we expect
	// since the signedTaskResponseC is deleted before another response can be accepted via select
	if blsAggServiceResp.TaskId != expectedTaskId{
		// This is not the task we were expecting so don't even send it
		return false, fmt.Errorf("FATAL: blsAggServiceResp.TaskId != expectedTaskId,blsAggServiceResp.TaskId: %v, expectedTaskId: %v", blsAggServiceResp.TaskId, expectedTaskId)
	}

	agg.logger.Info("sending aggregated response onchain.", "TaskId", blsAggServiceResp.TaskId)
	agg.tasksMu.RLock()
	task := agg.tasks[blsAggServiceResp.TaskId]
	agg.tasksMu.RUnlock()
	agg.taskResponsesMu.RLock()
	taskResponse := agg.taskResponses[blsAggServiceResp.TaskId][blsAggServiceResp.TaskResponseDigest]
	agg.taskResponsesMu.RUnlock()

	if blsAggServiceResp.TaskId.TaskType == 0 {

		opTask, ok := task.(taskmanager.IFinalizerTaskManagerOpTask)
		if !ok {
			return false, fmt.Errorf("FATAL: Aggregator failed to decode opTask, task: %v", task)
		}
		opTaskResponse, ok := taskResponse.(taskmanager.IFinalizerTaskManagerOpTaskResponse)
		if !ok {
			return false, fmt.Errorf("FATAL: Aggregator failed to decode opTaskResponse, taskResponse: %v", taskResponse)
		}
		r, err := agg.ethRpc.AvsWriter.SendAggregatedOpTaskResponse(context.Background(), opTask, opTaskResponse, nonSignerStakesAndSignature)
		if err != nil {
			return false, fmt.Errorf("Aggregator failed to SendAggregatedOpTaskResponse, task: %v, err: %v", task, err)
		}
		var success bool
		for _, log := range r.Logs{
			_, err := agg.ethRpc.AvsWriter.AvsContractBindings.TaskManager.ContractFinalizerTaskManagerFilterer.ParseOpTaskCompleted(*log)
			if err == nil {
				success = true
			}
		}
		agg.logger.Debug("Aggreagted Response sent to contract", "receipt", r, "success", success)
		return success, nil

	} else if blsAggServiceResp.TaskId.TaskType == 1 {

		rdTask, ok := task.(taskmanager.IFinalizerTaskManagerRdTask)
		if !ok {
			return false, fmt.Errorf("FATAL: Aggregator failed to decode rdTask, task: %v", task)
		}
		rdTaskResponse, ok := taskResponse.(taskmanager.IFinalizerTaskManagerRdTaskResponse)
		if !ok {
			return false, fmt.Errorf("FATAL: Aggregator failed to decode rdTaskResponse, taskResponse: %v", taskResponse)
		}
		r, err := agg.ethRpc.AvsWriter.SendAggregatedRdTaskResponse(context.Background(), rdTask, rdTaskResponse, nonSignerStakesAndSignature)
		if err != nil {
			return false, fmt.Errorf("Aggregator failed to respond to task, task: %v, err: %v", task, err)
		}
		var success bool
		for _, log := range r.Logs{
			_, err := agg.ethRpc.AvsWriter.AvsContractBindings.TaskManager.ContractFinalizerTaskManagerFilterer.ParseRdTaskCompleted(*log)
			if err == nil {
				success = true
			}
		}
		agg.logger.Debug("Aggreagted Response sent to contract", "receipt", r, "success", success)
		return success, nil

	} else {
		return false, fmt.Errorf("FATAL: Aggregator failed to recognize TaskType, blsAggServiceResp.TaskId: %v", blsAggServiceResp.TaskId)
	}

	return true, nil

}


func (agg *Aggregator) createOpTask()(taskmanager.IFinalizerTaskManagerOpTask, sdktypes.TaskId, error) {
	// Make sure that the taskManager is ready to accept another task
	isTaskPending, err := agg.ethRpc.AvsReader.IsTaskPending(context.Background())
	if err != nil {
		return taskmanager.IFinalizerTaskManagerOpTask{}, sdktypes.TaskId{}, fmt.Errorf("Aggregator failed to IsTaskPending: err: %v", err)
	}
	if isTaskPending {
		return taskmanager.IFinalizerTaskManagerOpTask{}, sdktypes.TaskId{}, fmt.Errorf("Aggregator in createOpTask but isTaskPending is true!!!")
	}

	agg.logger.Info("Aggregator sending new OpTask")
	// Send number to square to the task manager contract
	newTask, taskIndex, err := agg.ethRpc.AvsWriter.SendNewOpTask(context.Background(), types.QUORUM_THRESHOLD_NUMERATOR, types.QUORUM_NUMBERS)
	if err != nil {
		agg.logger.Error("Aggregator failed to send block number to verify", "err", err)
		return taskmanager.IFinalizerTaskManagerOpTask{}, sdktypes.TaskId{}, err
	}

	taskId := sdktypes.TaskId{
		TaskType: sdktypes.TaskType(0),
		TaskIndex: sdktypes.TaskIndex(taskIndex),
		}
	return newTask, taskId, nil
}


func (agg *Aggregator) processCreatedOpTask(newTask taskmanager.IFinalizerTaskManagerOpTask, taskId sdktypes.TaskId ) (bool, error) {

	agg.tasksMu.Lock()
	agg.tasks[taskId] = newTask
	agg.tasksMu.Unlock()

	quorumThresholdPercentages := make(sdktypes.QuorumThresholdPercentages, len(newTask.LastCompletedOpTaskQuorumNumbers))
	for i, _ := range newTask.LastCompletedOpTaskQuorumNumbers {
		quorumThresholdPercentages[i] = sdktypes.QuorumThresholdPercentage(newTask.LastCompletedOpTaskQuorumThresholdPercentage)
	}
	quorumNums := make(sdktypes.QuorumNums, len(newTask.LastCompletedOpTaskQuorumNumbers))
	for i, n := range newTask.LastCompletedOpTaskQuorumNumbers {
		quorumNums[i] = sdktypes.QuorumNum(n)
	}
	taskTimeToExpiry := time.Second * time.Duration(agg.expiration) * 2
	err := agg.blsAggregationService.InitializeNewTask(taskId, newTask.LastCompletedOpTaskCreatedBlock, quorumNums, quorumThresholdPercentages, taskTimeToExpiry)
	if err != nil{
		return false, fmt.Errorf("Aggregator failed to InitializeNewTask, err: %v", err)
	}
	agg.logger.Info("Aggregator initialized new operator state task", "taskId", taskId, "expiry", taskTimeToExpiry)


	// wait here synchronously
	agg.logger.Info("Aggregator in processCreatedOpTask waiting for response from blsagg", "taskId", taskId)
	blsAggServiceResp := <-agg.blsAggregationService.GetResponseChannel()
	agg.logger.Info("Received response from blsAggregationService", "blsAggServiceResp", blsAggServiceResp)
	success, err := agg.sendAggregatedResponseToContract(blsAggServiceResp, taskId)
	if err != nil{
		return false, fmt.Errorf("sendAggregatedResponseToContract err: %v", err)
	}

	return success, nil
}


func (agg *Aggregator) createRdTask(chainToUpdate uint8, chainBatchIdToUpdate uint32)(taskmanager.IFinalizerTaskManagerRdTask, sdktypes.TaskId, error) {
	// Make sure that the taskManager is ready to accept another task
	isTaskPending, err := agg.ethRpc.AvsReader.IsTaskPending(context.Background())
	if err != nil {
		return taskmanager.IFinalizerTaskManagerRdTask{}, sdktypes.TaskId{}, fmt.Errorf("Aggregator failed to IsTaskPending: err: %v", err)
	}
	if isTaskPending {
		return taskmanager.IFinalizerTaskManagerRdTask{}, sdktypes.TaskId{}, fmt.Errorf("Aggregator in createRdTask with isTaskPending true!!!")
	}

	agg.logger.Info("Aggregator sending new RdTask", "chainToUpdate", chainToUpdate, "chainBatchIdToUpdate", chainBatchIdToUpdate)
	// Send number to square to the task manager contract
	newTask, taskIndex, err := agg.ethRpc.AvsWriter.SendNewRdTask(context.Background(), chainToUpdate, chainBatchIdToUpdate)
	if err != nil {
		agg.logger.Error("Aggregator failed to SendNewRdTask", "err", err)
		return taskmanager.IFinalizerTaskManagerRdTask{}, sdktypes.TaskId{}, err
	}

	taskId := sdktypes.TaskId{
		TaskType: sdktypes.TaskType(1),
		TaskIndex: sdktypes.TaskIndex(taskIndex),
		}
	
	return newTask, taskId, nil
}

func (agg *Aggregator) processCreatedRdTask(newTask taskmanager.IFinalizerTaskManagerRdTask, taskId sdktypes.TaskId ) (bool, error) {
	agg.tasksMu.Lock()
	agg.tasks[taskId] = newTask
	agg.tasksMu.Unlock()

	if agg.kicker != nil {
		agg.kicker.TriggerNewTask(taskId.TaskIndex)
	}

	quorumThresholdPercentages := make(sdktypes.QuorumThresholdPercentages, len(newTask.LastCompletedOpTaskQuorumNumbers))
	for i, _ := range newTask.LastCompletedOpTaskQuorumNumbers {
		quorumThresholdPercentages[i] = sdktypes.QuorumThresholdPercentage(newTask.LastCompletedOpTaskQuorumThresholdPercentage)
	}
	quorumNums := make(sdktypes.QuorumNums, len(newTask.LastCompletedOpTaskQuorumNumbers))
	for i, n := range newTask.LastCompletedOpTaskQuorumNumbers {
		quorumNums[i] = sdktypes.QuorumNum(n)
	}
	taskTimeToExpiry := time.Second * time.Duration(agg.expiration)
	err := agg.blsAggregationService.InitializeNewTask(taskId, newTask.LastCompletedOpTaskCreatedBlock, quorumNums, quorumThresholdPercentages, taskTimeToExpiry)
	if err != nil{
		return false, fmt.Errorf("Aggregator failed to InitializeNewTask, err: %v", err)
	}
	agg.logger.Info("Aggregator initialized new rolldown update task", "newTask", newTask, "taskId", taskId, "expiry", taskTimeToExpiry)


	// wait here synchronously
	agg.logger.Info("Aggregator in processCreatedRdTask waiting for response from blsagg", "taskId", taskId)
	blsAggServiceResp := <-agg.blsAggregationService.GetResponseChannel()
	agg.logger.Info("Received response from blsAggregationService", "blsAggServiceResp", blsAggServiceResp)
	success, err := agg.sendAggregatedResponseToContract(blsAggServiceResp, taskId)
	if err != nil{
		return false, fmt.Errorf("sendAggregatedResponseToContract err: %v", err)
	}

	return success, nil
}


func (agg *Aggregator) startNewOpTask() (taskmanager.IFinalizerTaskManagerOpTask, error) {
	newTask, err := agg.createAndProcessOpTask(3)
	if err!=nil{
		return taskmanager.IFinalizerTaskManagerOpTask{}, fmt.Errorf("Aggregator failed to createAndProcessOpTask, err: %v", err)
	}
	return newTask, nil
}

// sendNewTask sends a new task to the task manager contract, and updates the Task dict struct
// with the information of operators opted into quorum 0 at the block of task creation.
func (agg *Aggregator) maybeSendNewRdTask(blockNumber uint32) error {

	if agg.asyncOpStateUpdaterError != nil{
		agg.logger.Error("asyncOpStateUpdater has crashed with the following error - but aggregator is still processing rdTasks", "err", agg.asyncOpStateUpdaterError)
	}

	isRduTask := blockNumber%agg.blockPeriod == 0

	if !isRduTask {
		return nil
	}

	// Why this check?
	latest, err := agg.substrateClient.RPC.Chain.GetHeaderLatest()
	if uint32(latest.Number) != blockNumber || err != nil {
		return nil
	}

	lastCompletedOpTaskCreatedBlock, err := agg.ethRpc.AvsReader.LastCompletedOpTaskCreatedBlock(context.Background());
	if err != nil {
		agg.logger.Error("Aggregator failed to get LastCompletedOpTaskCreatedBlock", "err", err)
		return err
	}

	isOpsInit := lastCompletedOpTaskCreatedBlock != 0

	if !isOpsInit {
		return nil
	}


	isUpdate, chainToUpdate, chainBatchIdToUpdate, err := agg.getL1BatchUpdateInfo(blockNumber)
	if err != nil {
		return fmt.Errorf("Aggregator in maybeSendNewRdTask failed to getL1BatchUpdateInfo: err: %v", err)
	}

	if !isUpdate{
		agg.logger.Info("Aggregator in maybeSendNewRdTask found no new updates at", "block number", blockNumber)
		return nil
	}


	err = agg.createAndProcessRdTask(chainToUpdate, chainBatchIdToUpdate, 3)
	if err != nil{
		return fmt.Errorf("Aggregator failed to createAndProcessRdTask: err: %v", err)
	}	

	return nil
}

func (agg *Aggregator) createAndProcessOpTask(maxAttempts uint8) (taskmanager.IFinalizerTaskManagerOpTask, error) {

	var success bool
	var newTask taskmanager.IFinalizerTaskManagerOpTask
	var taskId sdktypes.TaskId
	var err error
	for attempt := 0; attempt < int(maxAttempts); attempt++ {
	
		newTask, taskId, err = agg.createOpTask()
		if err != nil {
			return taskmanager.IFinalizerTaskManagerOpTask{}, fmt.Errorf("Aggregator failed to createOpTask, err: %v", err)
		}

		success, err = agg.processCreatedOpTask(newTask, taskId)
		if err != nil {
			return taskmanager.IFinalizerTaskManagerOpTask{}, fmt.Errorf("Aggregator failed to processCreatedOpTask, err: %v", err)
		}
		if success {
			break;
		}
	}
	if success!=true{
		return taskmanager.IFinalizerTaskManagerOpTask{}, fmt.Errorf("Aggregator failed to succesfully complete opTask after 3 attempts", "newTask", newTask)
	}
	return newTask, nil
	
}

func (agg *Aggregator) createAndProcessRdTask(chainToUpdate uint8, chainBatchIdToUpdate uint32, maxAttempts uint8) (error) {
	var success bool
	var newTask taskmanager.IFinalizerTaskManagerRdTask
	var taskId sdktypes.TaskId
	var err error
	for attempt := 0; attempt < int(maxAttempts); attempt++ {
	agg.logger.Info("Aggregator new RdTask", "chainToUpdate", chainToUpdate, "chainBatchIdToUpdate", chainBatchIdToUpdate, "attempt", attempt)

	newTask, taskId, err = agg.createRdTask(chainToUpdate, chainBatchIdToUpdate)
	if err != nil{
		return fmt.Errorf("Aggregator failed to createRdTask: err: %v", err)
	}

	success, err = agg.processCreatedRdTask(newTask, taskId)
	if err != nil{
		return fmt.Errorf("Aggregator failed to processCreatedRdTask: err: %v", err)
	}
	if success {
		break;
	}
	}
	if success!=true{
		return fmt.Errorf("Aggregator failed to succesfully complete rdTask after 3 attempts, chainToUpdate: %v, chainBatchIdToUpdate: %v", chainToUpdate, chainBatchIdToUpdate)
	}
	return nil
}

func (agg *Aggregator) getL1BatchUpdateInfo(blockNumber uint32) (bool, uint8, uint32, error) {

		// Check if on gasp any L1 has any new batches
		
		atBlockHash, err := agg.substrateClient.RPC.Chain.GetBlockHash(uint64(blockNumber))
		if err != nil {
			return false, 0, 0, fmt.Errorf("Aggregator in maybeSendNewRdTask failed to GetBlockHash: err: %v", err)
		}

		meta, err := agg.substrateClient.RPC.State.GetMetadata(atBlockHash)
		if err != nil {
			return false, 0, 0, fmt.Errorf("Aggregator in maybeSendNewRdTask failed to GetMetadata: err: %v", err)
		}

		var substrateL2RequestsBatchLast types.SubstrateL2RequestsBatchLast
		var isUpdate bool
		var chainToUpdate uint8
		var chainBatchIdToUpdate uint32

		key, err := gsrpctypes.CreateStorageKey(meta, "Rolldown", "L2RequestsBatchLast", nil, nil)
		if err != nil {
			return false, 0, 0, fmt.Errorf("Aggregator in maybeSendNewRdTask failed to CreateStorageKey: err: %v", err)
		}

		agg.logger.Debug("Aggregator in maybeSendNewRdTask after CreateStorageKey", "key", hex.EncodeToString(key))

		raw, err := agg.substrateClient.RPC.State.GetStorageRaw(key, atBlockHash)
		if err != nil {
			return false, 0, 0, fmt.Errorf("Aggregator in maybeSendNewRdTask failed to GetStorage: err: %v", err)
		}

		agg.logger.Debug("Aggregator in maybeSendNewRdTask after GetStorageRaw", "raw", raw)

		ok, err := agg.substrateClient.RPC.State.GetStorage(key, &substrateL2RequestsBatchLast, atBlockHash)
		if err != nil {
			return false, 0, 0, fmt.Errorf("Aggregator in maybeSendNewRdTask failed to GetStorage: err: %v", err)
		}

		if !ok {
			agg.logger.Debug("Aggregator in maybeSendNewRdTask after GetStorage", "ok", ok)
			return false, 0, 0, nil
		}

		// if substrateL2RequestsBatchLast == nil {
		// 	agg.logger.Debug("Aggregator in maybeSendNewRdTask after GetStorage", "substrateL2RequestsBatchLast", substrateL2RequestsBatchLast)
		// 	return false, 0, 0, nil
		// }

		for _, lastBatchByL1 := range substrateL2RequestsBatchLast{
			
			chainRdBatchNonce, err := agg.ethRpc.AvsReader.ChainRdBatchNonce(context.Background(), uint8(lastBatchByL1.Key))
			if err != nil {
				return false, 0, 0, fmt.Errorf("Aggregator in maybeSendNewRdTask failed to ChainRdBatchNonce: err: %v", err)
			}

			if uint64(lastBatchByL1.Value.BatchId.Int64()) >= uint64(chainRdBatchNonce) {
				isUpdate = true
				chainToUpdate = uint8(lastBatchByL1.Key)
				if chainRdBatchNonce == 0 {
					chainBatchIdToUpdate = 1
				} else {
					chainBatchIdToUpdate = chainRdBatchNonce
				}
				break
			}
		}

		return isUpdate, chainToUpdate, chainBatchIdToUpdate, nil
}
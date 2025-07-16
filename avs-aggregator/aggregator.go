package aggregator

import (
	"bytes"
	"context"
	"encoding/binary"
	"encoding/hex"
	"errors"
	"fmt"
	"sync"
	"time"
	"math/big"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"

	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/services/avsregistry"
	blsagg "github.com/Layr-Labs/eigensdk-go/services/bls_aggregation"
	oprsinfoserv "github.com/Layr-Labs/eigensdk-go/services/operatorsinfo"
	sdktypes "github.com/Layr-Labs/eigensdk-go/types"
	"github.com/gasp-xyz/gasp-monorepo/avs-aggregator/core"
	"github.com/gasp-xyz/gasp-monorepo/avs-aggregator/core/chainio"
	"github.com/gasp-xyz/gasp-monorepo/avs-aggregator/types"

	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
	gsrpcrpcchain "github.com/centrifuge/go-substrate-rpc-client/v4/rpc/chain"
	gsrpctypes "github.com/centrifuge/go-substrate-rpc-client/v4/types"
	taskmanager "github.com/gasp-xyz/gasp-monorepo/avs-aggregator/bindings/FinalizerTaskManager"
)

// Aggregator sends tasks onchain, then listens for operator signed TaskResponses.
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
	logger             sdklogging.Logger
	serverIpPortAddr   string
	blockPeriod        uint32
	blockPeriodOpsTask uint32
	expiration         uint32
	ethRpc             *chainio.EthRpc
	// aggregation related fields
	blsAggregationService    blsagg.BlsAggregationService
	tasks                    map[sdktypes.TaskId]interface{}
	tasksMu                  *sync.RWMutex
	taskResponses            map[sdktypes.TaskId]map[sdktypes.TaskResponseDigest]interface{}
	taskResponsesMu          *sync.RWMutex
	substrateClient          gsrpc.SubstrateAPI
	taskResponseWindowBlock  uint32
	asyncOpStateUpdaterError error
	retryNumber              uint8
	maxRetryNumber           uint8

	startIdle          bool
	apiKey             string

	kicker         *Kicker
	opStateUpdater *OpStateUpdater
	rpcServer		*RpcServer

	enableTraceLogs bool
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
		c.AggSSFetchTimeout,
	)
	if err != nil {
		logger.Error("Failed to create EthRpc clients", "err", err)
		return nil, err
	}

	chainId, err := ethRpc.AvsReader.ChainID(context.Background())
	if err != nil {
		logger.Error("Cannot get chainId", "err", err)
		return nil, err
	}
	if c.ChainId.Cmp(chainId) != 0 {
		logger.Error("chainId in arguments does not match ethRpcClient chain id", "eth chainId", chainId, "config chainId", c.ChainId)
		return nil, errors.New("config.chainId & ethRpcClient.chainId mismatch")
	}

	taskResponseWindowBlock, err := ethRpc.AvsReader.TaskResponseWindowBlock(&bind.CallOpts{})
	if err != nil {
		logger.Error("Cannot get taskChallengeWindowBlock from TaskManager contract", "err", err)
		return nil, err
	}

	operatorPubkeysService := oprsinfoserv.NewOperatorsInfoServiceInMemory(
		context.Background(),
		ethRpc.Clients.AvsRegistryChainSubscriber(),
		ethRpc.Clients.AvsRegistryChainReader(),
		nil,
		oprsinfoserv.Opts{StartBlock: big.NewInt(int64(c.AvsDeploymentBlock))},
		logger,
	)
	avsRegistryService := avsregistry.NewAvsRegistryServiceChainCaller(ethRpc.Clients.AvsRegistryChainReader(), operatorPubkeysService, logger)
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

	tasks := make(map[sdktypes.TaskId]interface{})
	tasksMu :=  &sync.RWMutex{}
	taskResponses := make(map[sdktypes.TaskId]map[sdktypes.TaskResponseDigest]interface{})
	taskResponsesMu := &sync.RWMutex{}

	opStateUpdater, err := NewOpStateUpdater(logger, ethRpc, c.MinOpUpdateInterval, c.ReinitOpStateAtInit, c.CheckTriggerOpStateUpdate, c.CheckTriggerOpStateUpdateWindow, c.EnableTraceLogs)
	if err != nil {
		logger.Error("Cannot create operator stakes updateer", "err", err)
		return nil, err
	}

	rpcServer, err := NewRpcServer(logger, tasks, tasksMu, taskResponses, taskResponsesMu, blsAggregationService, c.ServerAddressPort)
	if err != nil {
		logger.Error("Cannot create rpcServer", "err", err)
		return nil, err
	}

	return &Aggregator{
		logger:                  logger,
		serverIpPortAddr:        c.ServerAddressPort,
		ethRpc:                  ethRpc,
		blsAggregationService:   blsAggregationService,
		tasks:                   tasks,
		tasksMu:                 tasksMu,
		taskResponses:           taskResponses,
		taskResponsesMu:         taskResponsesMu,
		substrateClient:         *substrateRpc,
		taskResponseWindowBlock: taskResponseWindowBlock,
		blockPeriod:             uint32(c.BlockPeriod),
		blockPeriodOpsTask:      uint32(c.BlockPeriodOpsTask),
		kicker:                  kicker,
		opStateUpdater:          opStateUpdater,
		rpcServer:				 rpcServer,
		expiration:              uint32(c.Expiration),
		startIdle:			   	 c.AggIdleStart,
		apiKey:			   	     c.AggRunTriggerApiKey,	
		enableTraceLogs:		 c.EnableTraceLogs,	
	}, nil
}

func (agg *Aggregator) Start(ctx context.Context) error {
	agg.logger.Infof("ALERT:INFO Starting aggregator.")
	agg.logger.Infof("Starting aggregator rpc server.")
	runTriggerC := make(chan struct{})

	// Apparently golang allows calling functions on nil values
	// And it seems that the function would be run on the struct assuming default values
	if agg.rpcServer != nil {
		go agg.rpcServer.startServer(ctx, agg.apiKey, runTriggerC)
	}
	
	recordMetrics(agg.logger, agg.ethRpc)

	if agg.startIdle {
		// blocking wait for the run trigger
		agg.logger.Infof("ALERT:INFO Aggregator awaiting run trigger.")
		recordRunTriggerTimesEventMetric(true)
		<- runTriggerC
		agg.logger.Infof("ALERT:INFO Aggregator received run trigger. Continuing.")
		recordRunTriggerTimesEventMetric(false)
	}

	err := agg.checkAndProcessPendingTasks()
	if err != nil {
		return fmt.Errorf("Aggregator failed to checkAndProcessPendingTasks: err: %v", err)
	}

	sendNewOpTaskC := make(chan types.SendNewOpTaskType)
	asyncOpStateUpdaterErrorC := make(chan error)

	// Apparently golang allows calling functions on nil values
	// And it seems that the function would be run on the struct assuming default values
	if agg.opStateUpdater != nil {
		go agg.opStateUpdater.startAsyncOpStateUpdater(ctx, sendNewOpTaskC, asyncOpStateUpdaterErrorC)
	}

	var sub *gsrpcrpcchain.NewHeadsSubscription
	const maxRetries = 5
	const retryDelay = time.Minute

	// Loop to retry subscription on error
	for attempt := 0; attempt < maxRetries; attempt++ {
		sub, err = agg.substrateClient.RPC.Chain.SubscribeNewHeads()
		if err == nil {
			break // Successfully subscribed, exit loop
		}

		agg.logger.Error("ALERT::WARNING Failed to get new head from substrate, retrying...", "err", err, "attempt", attempt+1)
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
				OpTask:             OpTask,
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

				agg.logger.Error("ALERT:WARNING Failed to get new head from substrate, retrying...", "err", err, "attempt", attempt+1)
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

func (agg *Aggregator) checkAndProcessPendingTasks() error {

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

	switch {
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

func (agg *Aggregator) getAndProcessPendingOpTask(latestOpTaskNum uint32, lastOpTaskCreatedBlock uint32) error {
	EndBlock := uint64(lastOpTaskCreatedBlock)
	pendingOpTask, err := agg.ethRpc.AvsReader.GetFirstFilterNewOpTaskCreated(
		&bind.FilterOpts{Start: uint64(lastOpTaskCreatedBlock), End: &EndBlock, Context: context.Background()}, []uint32{latestOpTaskNum},
	)
	if err != nil {
		return fmt.Errorf("Aggregator failed to GetFirstFilterNewOpTaskCreated: err: %v", err)
	}

	pendingOpTaskId := sdktypes.TaskId{
		TaskType:  sdktypes.TaskType(0),
		TaskIndex: sdktypes.TaskIndex(latestOpTaskNum),
	}
	err = agg.processPendingOpTask(pendingOpTask, pendingOpTaskId)
	if err != nil {
		return fmt.Errorf("Aggregator failed to processPendingOpTask: err: %v", err)
	}
	return nil
}

func (agg *Aggregator) processPendingOpTask(newTask taskmanager.IFinalizerTaskManagerOpTask, taskId sdktypes.TaskId) error {

	success, err := agg.processCreatedOpTask(newTask, taskId)
	if err != nil{
		return fmt.Errorf("Aggregator failed to processCreatedOpTask: err: %v", err)
	}
	if success {
		return nil
	}

	newTask, err = agg.createAndProcessOpTask(2)
	if err != nil {
		return fmt.Errorf("Aggregator failed to createAndProcessOpTask: err: %v", err)
	}
	return nil
}

func (agg *Aggregator) getAndProcessPendingRdTask(latestRdTaskNum uint32, lastRdTaskCreatedBlock uint32) error {
	EndBlock := uint64(lastRdTaskCreatedBlock)
	pendingRdTask, err := agg.ethRpc.AvsReader.GetFirstFilterNewRdTaskCreated(
		&bind.FilterOpts{Start: uint64(lastRdTaskCreatedBlock), End: &EndBlock, Context: context.Background()}, []uint32{latestRdTaskNum},
	)
	if err != nil {
		return fmt.Errorf("Aggregator failed to GetFirstFilterNewRdTaskCreated: err: %v", err)
	}

	pendingRdTaskId := sdktypes.TaskId{
		TaskType:  sdktypes.TaskType(1),
		TaskIndex: sdktypes.TaskIndex(latestRdTaskNum),
	}
	err = agg.processPendingRdTask(pendingRdTask, pendingRdTaskId)
	if err != nil {
		return fmt.Errorf("Aggregator failed to ProcessPendingRdTask: err: %v", err)
	}
	return nil

}

func (agg *Aggregator) processPendingRdTask(newTask taskmanager.IFinalizerTaskManagerRdTask, taskId sdktypes.TaskId) error {

	chainToUpdate := newTask.ChainId
	chainBatchIdToUpdate := newTask.BatchId

	success, err := agg.processCreatedRdTask(newTask, taskId)
	if err != nil{
		return fmt.Errorf("Aggregator failed to processCreatedRdTask: err: %v", err)
	}
	if success {
		return nil
	}

	err = agg.createAndProcessRdTask(chainToUpdate, chainBatchIdToUpdate, 2)
	if err != nil {
		return fmt.Errorf("Aggregator failed to createAndProcessRdTask: err: %v", err)
	}
	return nil
}

func (agg *Aggregator) sendAggregatedResponseToContract(blsAggServiceResp blsagg.BlsAggregationServiceResponse, expectedTaskId sdktypes.TaskId) (bool, error) {
	if blsAggServiceResp.Err != nil && blsAggServiceResp.Err == blsagg.TaskNotRespondedError {
		agg.logger.Info("ALERT:INFO Aggregator received no responses - cancelling task", "taskId", expectedTaskId)
		err := agg.ethRpc.AvsWriter.CancelPendingTask(context.Background())
		if err != nil{
			return false, fmt.Errorf("Aggregator failed to CancelPendingTask: err: %v", err)
		}
		return false, nil
	}
	if blsAggServiceResp.Err != nil && blsAggServiceResp.Err != blsagg.TaskExpiredError {
		return false, fmt.Errorf("ALERT:WARNING bls aggregation error, err: %v", blsAggServiceResp.Err)
	}

	// We can hard expect that here we will get only what we expect
	// since the signedTaskResponseC is deleted before another response can be accepted via select
	if blsAggServiceResp.TaskId != expectedTaskId {
		// This is not the task we were expecting so don't even send it
		return false, fmt.Errorf("ALERT:ERROR: blsAggServiceResp.TaskId != expectedTaskId,blsAggServiceResp.TaskId: %v, expectedTaskId: %v", blsAggServiceResp.TaskId, expectedTaskId)
	}

	taskStatus, err := agg.ethRpc.AvsReader.IdToTaskStatus(context.Background(), expectedTaskId.TaskType, expectedTaskId.TaskIndex)
	if err != nil {
		return false, fmt.Errorf("Aggregator in sendAggregatedResponseToContract failed to IdToTaskStatus: err: %v", err)
	}
	if taskStatus != types.TASK_STATUS_INITIALIZED {
		return taskStatus == types.TASK_STATUS_COMPLETED, nil
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

	agg.logger.Info("ALERT:INFO sending aggregated response onchain.", "TaskId", blsAggServiceResp.TaskId)
	agg.tasksMu.RLock()
	task := agg.tasks[blsAggServiceResp.TaskId]
	agg.tasksMu.RUnlock()
	agg.taskResponsesMu.RLock()
	taskResponse := agg.taskResponses[blsAggServiceResp.TaskId][blsAggServiceResp.TaskResponseDigest]
	agg.taskResponsesMu.RUnlock()

	if blsAggServiceResp.TaskId.TaskType == 0 {

		opTask, ok := task.(taskmanager.IFinalizerTaskManagerOpTask)
		if !ok {
			return false, fmt.Errorf("ALERT:ERROR: Aggregator failed to decode opTask, task: %v", task)
		}
		opTaskResponse, ok := taskResponse.(taskmanager.IFinalizerTaskManagerOpTaskResponse)
		if !ok {
			return false, fmt.Errorf("ALERT:ERROR: Aggregator failed to decode opTaskResponse, taskResponse: %v", taskResponse)
		}
		r, err := agg.ethRpc.AvsWriter.SendAggregatedOpTaskResponse(context.Background(), opTask, opTaskResponse, nonSignerStakesAndSignature)
		if err != nil {
			return false, fmt.Errorf("Aggregator failed to SendAggregatedOpTaskResponse, task: %v, err: %v", task, err)
		}
		var success bool
		for _, log := range r.Logs {
			_, err := agg.ethRpc.AvsReader.ParseOpTaskCompleted(*log)
			if err == nil {
				success = true
			}
		}
		agg.logger.Debug("Aggreagted Response sent to contract", "receipt", r, "success", success)

		if success {
			recordLastTaskRespondedMetric(blsAggServiceResp.TaskId.TaskType, blsAggServiceResp.TaskId.TaskIndex)
			recordLastTaskCompletedMetric(blsAggServiceResp.TaskId.TaskType, blsAggServiceResp.TaskId.TaskIndex)
		} else {
			recordRespondedButUncompletedTasksMetric()
			recordLastTaskRespondedMetric(blsAggServiceResp.TaskId.TaskType, blsAggServiceResp.TaskId.TaskIndex)
		}

		return success, nil

	} else if blsAggServiceResp.TaskId.TaskType == 1 {

		rdTask, ok := task.(taskmanager.IFinalizerTaskManagerRdTask)
		if !ok {
			return false, fmt.Errorf("ALERT:ERROR: Aggregator failed to decode rdTask, task: %v", task)
		}
		rdTaskResponse, ok := taskResponse.(taskmanager.IFinalizerTaskManagerRdTaskResponse)
		if !ok {
			return false, fmt.Errorf("ALERT:ERROR: Aggregator failed to decode rdTaskResponse, taskResponse: %v", taskResponse)
		}

		err := agg.verifyTaskResponseExistsOnL2(rdTaskResponse)
		if err != nil {
			return false, fmt.Errorf("ALERT:ERROR: Aggregator rdTaskResponse verification error %v", err)
		}

		r, err := agg.ethRpc.AvsWriter.SendAggregatedRdTaskResponse(context.Background(), rdTask, rdTaskResponse, nonSignerStakesAndSignature)
		if err != nil {
			return false, fmt.Errorf("Aggregator failed to respond to task, task: %v, err: %v", task, err)
		}
		var success bool
		for _, log := range r.Logs {
			_, err := agg.ethRpc.AvsReader.ParseRdTaskCompleted(*log)
			if err == nil {
				success = true
			}
		}
		agg.logger.Debug("Aggreagted Response sent to contract", "receipt", r, "success", success)

		if success {
			recordLatestBatchProcessedOnL1PerL1Metric(rdTask.ChainId, rdTask.BatchId)
			recordLastTaskRespondedMetric(blsAggServiceResp.TaskId.TaskType, blsAggServiceResp.TaskId.TaskIndex)
			recordLastTaskCompletedMetric(blsAggServiceResp.TaskId.TaskType, blsAggServiceResp.TaskId.TaskIndex)
		} else {
			recordRespondedButUncompletedTasksMetric()
			recordLastTaskRespondedMetric(blsAggServiceResp.TaskId.TaskType, blsAggServiceResp.TaskId.TaskIndex)
		}

		return success, nil

	} else {
		return false, fmt.Errorf("ALERT:ERROR: Aggregator failed to recognize TaskType, blsAggServiceResp.TaskId: %v", blsAggServiceResp.TaskId)
	}

}

func (agg *Aggregator) createOpTask() (taskmanager.IFinalizerTaskManagerOpTask, sdktypes.TaskId, error) {
	// Make sure that the taskManager is ready to accept another task
	isTaskPending, err := agg.ethRpc.AvsReader.IsTaskPending(context.Background())
	if err != nil {
		return taskmanager.IFinalizerTaskManagerOpTask{}, sdktypes.TaskId{}, fmt.Errorf("Aggregator failed to IsTaskPending: err: %v", err)
	}
	if isTaskPending {
		return taskmanager.IFinalizerTaskManagerOpTask{}, sdktypes.TaskId{}, fmt.Errorf("Aggregator in createOpTask but isTaskPending is true!!!")
	}

	agg.logger.Info("ALERT:INFO Aggregator sending new OpTask")
	newTask, taskIndex, err := agg.ethRpc.AvsWriter.SendNewOpTask(context.Background(), types.QUORUM_THRESHOLD_NUMERATOR, types.QUORUM_NUMBERS)
	if err != nil {
		agg.logger.Error("Aggregator failed to send block number to verify", "err", err)
		return taskmanager.IFinalizerTaskManagerOpTask{}, sdktypes.TaskId{}, err
	}

	taskId := sdktypes.TaskId{
		TaskType:  sdktypes.TaskType(0),
		TaskIndex: sdktypes.TaskIndex(taskIndex),
	}

	recordLastTaskCreatedMetric(taskId.TaskType, taskId.TaskIndex)

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
	agg.logger.Info("ALERT:INFO Aggregator initialized new operator state task", "taskId", taskId, "expiry", taskTimeToExpiry)

	// wait here synchronously
	agg.logger.Info("Aggregator in processCreatedOpTask waiting for response from blsagg", "taskId", taskId)
	blsAggServiceResp := <-agg.blsAggregationService.GetResponseChannel()
	agg.logger.Info("Received response from blsAggregationService", "blsAggServiceResp", blsAggServiceResp)
	success, err := agg.sendAggregatedResponseToContract(blsAggServiceResp, taskId)
	if err != nil{
		return false, fmt.Errorf("ALERT:WARNING sendAggregatedResponseToContract err: %v", err)
	}

	return success, nil
}

func (agg *Aggregator) createRdTask(chainToUpdate uint8, chainBatchIdToUpdate uint32) (taskmanager.IFinalizerTaskManagerRdTask, sdktypes.TaskId, error) {
	// Make sure that the taskManager is ready to accept another task
	isTaskPending, err := agg.ethRpc.AvsReader.IsTaskPending(context.Background())
	if err != nil {
		return taskmanager.IFinalizerTaskManagerRdTask{}, sdktypes.TaskId{}, fmt.Errorf("Aggregator failed to IsTaskPending: err: %v", err)
	}
	if isTaskPending {
		return taskmanager.IFinalizerTaskManagerRdTask{}, sdktypes.TaskId{}, fmt.Errorf("Aggregator in createRdTask with isTaskPending true!!!")
	}

	chainRdBatchNonce, err := agg.ethRpc.AvsReader.ChainRdBatchNonce(context.Background(), chainToUpdate)
	if err != nil {
		return taskmanager.IFinalizerTaskManagerRdTask{}, sdktypes.TaskId{}, fmt.Errorf("Aggregator in maybeSendNewRdTask failed to ChainRdBatchNonce: err: %v", err)
	}
	if ((chainRdBatchNonce != 0) && (chainRdBatchNonce != chainBatchIdToUpdate)){
		return taskmanager.IFinalizerTaskManagerRdTask{}, sdktypes.TaskId{}, fmt.Errorf("Aggregator in maybeSendNewRdTask failed to ((chainRdBatchNonce != 0) && (chainRdBatchNonce != chainBatchIdToUpdate)): chainRdBatchNonce: %v, chainBatchIdToUpdate: %v", chainRdBatchNonce, chainBatchIdToUpdate)
	}

	agg.logger.Info("ALERT:INFO Aggregator sending new RdTask", "chainToUpdate", chainToUpdate, "chainBatchIdToUpdate", chainBatchIdToUpdate)
	// Send new rdTask to the task manager contract
	newTask, taskIndex, err := agg.ethRpc.AvsWriter.SendNewRdTask(context.Background(), chainToUpdate, chainBatchIdToUpdate)
	if err != nil {
		agg.logger.Error("Aggregator failed to SendNewRdTask", "err", err)
		return taskmanager.IFinalizerTaskManagerRdTask{}, sdktypes.TaskId{}, err
	}

	taskId := sdktypes.TaskId{
		TaskType:  sdktypes.TaskType(1),
		TaskIndex: sdktypes.TaskIndex(taskIndex),
	}

	recordLastTaskCreatedMetric(taskId.TaskType, taskId.TaskIndex)

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
	agg.logger.Info("ALERT:INFO Aggregator initialized new rolldown update task", "newTask", newTask, "taskId", taskId, "expiry", taskTimeToExpiry)

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
	if err != nil {
		return taskmanager.IFinalizerTaskManagerOpTask{}, fmt.Errorf("Aggregator failed to createAndProcessOpTask, err: %v", err)
	}
	return newTask, nil
}

// sendNewTask sends a new task to the task manager contract, and updates the Task dict struct
// with the information of operators opted into quorum 0 at the block of task creation.
func (agg *Aggregator) maybeSendNewRdTask(blockNumber uint32) error {

	isRduTask := blockNumber%agg.blockPeriod == 0

	if !isRduTask {
		return nil
	}

	if agg.asyncOpStateUpdaterError != nil {
		agg.logger.Error("asyncOpStateUpdater has crashed with the following error - but aggregator is still processing rdTasks", "err", agg.asyncOpStateUpdaterError)
	}

	// Why this check?
	latest, err := agg.substrateClient.RPC.Chain.GetHeaderLatest()
	if uint32(latest.Number) != blockNumber || err != nil {
		return nil
	}

	lastCompletedOpTaskCreatedBlock, err := agg.ethRpc.AvsReader.LastCompletedOpTaskCreatedBlock(context.Background())
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

	if !isUpdate {
		agg.logger.Info("Aggregator in maybeSendNewRdTask found no new updates at", "block number", blockNumber)
		return nil
	}

	err = agg.createAndProcessRdTask(chainToUpdate, chainBatchIdToUpdate, 3)
	if err != nil {
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
		agg.logger.Info("Aggregator new OpTask", "attempt", attempt)
		newTask, taskId, err = agg.createOpTask()
		if err != nil {
			return taskmanager.IFinalizerTaskManagerOpTask{}, fmt.Errorf("Aggregator failed to createOpTask, err: %v", err)
		}

		success, err = agg.processCreatedOpTask(newTask, taskId)
		if err != nil {
			return taskmanager.IFinalizerTaskManagerOpTask{}, fmt.Errorf("Aggregator failed to processCreatedOpTask, err: %v", err)
		}

		if success {
			break
		}
	}
	if success != true {
		return taskmanager.IFinalizerTaskManagerOpTask{}, fmt.Errorf("Aggregator failed to succesfully complete opTask after 3 attempts, newTask: %v", newTask)
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

	for _, lastBatchByL1 := range substrateL2RequestsBatchLast {

		chainRdBatchNonce, err := agg.ethRpc.AvsReader.ChainRdBatchNonce(context.Background(), uint8(lastBatchByL1.Key))
		if err != nil {
			return false, 0, 0, fmt.Errorf("Aggregator in maybeSendNewRdTask failed to ChainRdBatchNonce: err: %v", err)
		}
		recordLatestBatchOnL2PerL1Metric(uint8(lastBatchByL1.Key), uint32(lastBatchByL1.Value.BatchId.Int64()))
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

	agg.logger.Debug("Aggregator in maybeSendNewRdTask after substrateL2RequestsBatchLast loop", "isUpdate", isUpdate, "chainToUpdate", chainToUpdate, "chainBatchIdToUpdate", chainBatchIdToUpdate)

	return isUpdate, chainToUpdate, chainBatchIdToUpdate, nil
}

func (agg *Aggregator) verifyTaskResponseExistsOnL2(rdTaskResponse taskmanager.IFinalizerTaskManagerRdTaskResponse) error {

	meta, err := agg.substrateClient.RPC.State.GetMetadataLatest()
	if err != nil {
		return fmt.Errorf("Aggregator::verifyTaskResponseExistsOnL2 GetMetadata failed err: %v", err)
	}

	batchIdBytes := make([]byte, 17)
	batchIdBytes[0] = byte(rdTaskResponse.ChainId)
	binary.LittleEndian.PutUint32(batchIdBytes[1:], rdTaskResponse.BatchId)
	key, err := gsrpctypes.CreateStorageKey(meta, "Rolldown", "L2RequestsBatch", batchIdBytes, nil)

	if err != nil {
		return fmt.Errorf("Aggregator::verifyTaskResponseExistsOnL2 CreateStorageKey failed : err: %v", err)
	}

	agg.logger.Debug("Aggregator::verifyTaskResponseExistsOnL2", "l2batch_raw_storage_key", hex.EncodeToString(key))

	raw, err := agg.substrateClient.RPC.State.GetStorageRawLatest(key)
	if err != nil {
		return fmt.Errorf("Aggregator::verifyTaskResponseExistsOnL2 GetStorage failed err: %v", err)
	}

	agg.logger.Debug("Aggregator::verifyTaskResponseExistsOnL2", "l2batch_raw_storage_value", raw.Hex())
	var substrateL2RequestsBatch types.SubstrateL2RequestsBatch

	ok, err := agg.substrateClient.RPC.State.GetStorageLatest(key, &substrateL2RequestsBatch)
	if err != nil {
		return fmt.Errorf("Aggregator::verifyTaskResponseExistsOnL2 L2RequestsBatch read failed: err: %v", err)
	}

	agg.logger.Debug("Aggregator::verifyTaskResponseExistsOnL2 ", "substrateL2RequestsBatch", substrateL2RequestsBatch)

	if !ok {
		return fmt.Errorf("Aggregator::verifyTaskResponseExistsOnL2 GetStorage staus is NOK")
	}

	// Check if substrateL2RequestsBatch range start and end are the same as in rdTaskResponse
	if ((rdTaskResponse.RangeStart.Cmp(substrateL2RequestsBatch.BatchRange.Start.Int) !=0 ) || (rdTaskResponse.RangeEnd.Cmp(substrateL2RequestsBatch.BatchRange.End.Int) !=0 )) {
		return fmt.Errorf("Aggregator::verifyTaskResponseExistsOnL2 substrateL2RequestsBatch and rdTaskResponse range do not match: substrateL2RequestsBatch: %v, rdTaskResponse: %v", substrateL2RequestsBatch, rdTaskResponse)
	}

	merkleRoot := ""
	chain := ""
	
	// Tag
	// EthChainIdType
	if rdTaskResponse.ChainId == 0 {
		chain = "Ethereum"
	} else if rdTaskResponse.ChainId == 1 {
		chain = "Arbitrum"
	} else if rdTaskResponse.ChainId == 2 {
		chain = "Base"
	} else if rdTaskResponse.ChainId == 3 {
		chain = "Monad"
	} else if rdTaskResponse.ChainId == 4 {
		chain = "MegaEth"
	} else if rdTaskResponse.ChainId == 5 {
		chain = "Sonic"
	} else {
		return fmt.Errorf("Aggregator::verifyTaskResponseExistsOnL2 unknown chain id %v", rdTaskResponse.ChainId)
	}

	rangeStart := gsrpctypes.NewU128(*rdTaskResponse.RangeStart)
	rangeEnd := gsrpctypes.NewU128(*rdTaskResponse.RangeEnd)
	err = agg.substrateClient.Client.Call(&merkleRoot, "rolldown_get_merkle_root", chain, [...]gsrpctypes.U128{rangeStart, rangeEnd})

	if err != nil {
		return fmt.Errorf("Aggregator::verifyTaskResponseExistsOnL2 get merkle root failed: err %v", err)
	}

	agg.logger.Debug("Aggregator::verifyTaskResponseExistsOnL2 ", "merkle_root", merkleRoot)

	decoded, err := hex.DecodeString(merkleRoot[2:])

	if err != nil {
		return fmt.Errorf("Aggregator::rolldown_get_merkle_root cannot decode root as bytes %s", merkleRoot)
	}

	if !bytes.Equal(decoded, rdTaskResponse.RdUpdate[:]) {
		return fmt.Errorf("Aggregator %v does not match %v", rdTaskResponse.RdUpdate, decoded)
	}
	agg.logger.Info("Aggregator::verifyTaskResponseExistsOnL2 rdTaskResponse verified successfully", "batch_id", rdTaskResponse.BatchId, "chain", chain)

	return nil
}

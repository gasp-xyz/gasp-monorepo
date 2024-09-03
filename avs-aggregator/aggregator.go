package aggregator

import (
	"context"
	"encoding/hex"
	"errors"
	"math/big"
	"sync"
	"time"
	"fmt"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"

	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/services/avsregistry"
	blsagg "github.com/Layr-Labs/eigensdk-go/services/bls_aggregation"
	operatorpubkeys "github.com/Layr-Labs/eigensdk-go/services/operatorpubkeys"
	sdktypes "github.com/Layr-Labs/eigensdk-go/types"
	"github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/core"
	"github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/core/chainio"
	"github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/types"

	taskmanager "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/FinalizerTaskManager"

	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
	gsrpcrpcchain "github.com/centrifuge/go-substrate-rpc-client/v4/rpc/chain"
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

	kicker  *Kicker
	opStateUpdater *OpStateUpdater
}

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

	chainId, err := ethRpc.Clients.EthHttpClient.ChainID(context.Background())
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

	pubkeyService := operatorpubkeys.NewOperatorPubkeysServiceInMemory(
		context.Background(),
		ethRpc.Clients.AvsRegistryChainSubscriber,
		ethRpc.Clients.AvsRegistryChainReader,
		ethRpc.Clients.EthHttpClient,
		c.AvsDeploymentBlock,
		50_000,
		logger,
	)
	avsRegistryService := avsregistry.NewAvsRegistryServiceChainCaller(ethRpc.Clients.AvsRegistryChainReader, pubkeyService, logger)
	blsAggregationService := blsagg.NewBlsAggregatorService(avsRegistryService, c.DebounceRpc, logger)

	substrateRpc, err := gsrpc.NewSubstrateAPI(c.SubstrateWsRpcUrl)
	if err != nil {
		logger.Error("Cannot create substrate RPC", "url", c.SubstrateWsRpcUrl, "err", err)
		return nil, err
	}

	kicker, err := NewKicker(logger, *ethRpc, uint32(c.KickPeriod), uint32(c.BlockPeriod))
	if err != nil {
		logger.Error("Cannot create operator active list filter", "err", err)
		return nil, err
	}

	opStateUpdater, err := NewOpStateUpdater(logger, ethRpc, avsRegistryService, c.MinOpUpdateInterval)
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
	sendNewOpTaskC := make(chan types.SendNewOpTaskType)
	asyncOpStateUpdaterErrorC := make(chan error)
	go agg.opStateUpdater.startAsyncOpStateUpdater(ctx, sendNewOpTaskC, asyncOpStateUpdaterErrorC)

	var sub *gsrpcrpcchain.NewHeadsSubscription
	var err error
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
		case blsAggServiceResp := <-agg.blsAggregationService.GetResponseChannel():
			agg.logger.Info("Received response from blsAggregationService", "blsAggServiceResp", blsAggServiceResp)
			agg.sendAggregatedResponseToContract(blsAggServiceResp)
		case head := <-sub.Chan():
			// agg.logger.Info("Received new substrate header", "head.Number", head.Number)
			err := agg.maybeSendNewRdTask(uint32(head.Number))
			if err != nil {
				// we log the errors inside sendNewTask() so here we just continue to the next task
				continue
			}
		case sendNewOpTask := <-sendNewOpTaskC:
			OpTask, err := agg.sendNewOpTask()
			if err != nil {
				// we log the errors inside sendNewTask() so here we just continue to the next task
				sendNewOpTask.SendNewOpTaskReturnC <- types.SendNewOpTaskReturn{
					SendNewOpTaskError: err,
				}
				continue
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

func (agg *Aggregator) sendAggregatedResponseToContract(blsAggServiceResp blsagg.BlsAggregationServiceResponse) {
	if blsAggServiceResp.Err != nil && blsAggServiceResp.Err != blsagg.TaskExpiredError {
		agg.logger.Warn("bls aggregation error", "err", blsAggServiceResp.Err)
		return
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
			agg.logger.Error("FATAL: Aggregator failed to decode opTask", "task", task)
			return
		}
		opTaskResponse, ok := taskResponse.(taskmanager.IFinalizerTaskManagerOpTaskResponse)
		if !ok {
			agg.logger.Error("FATAL: Aggregator failed to decode opTaskResponse", "taskResponse", taskResponse)
			return
		}
		r, err := agg.ethRpc.AvsWriter.SendAggregatedOpTaskResponse(context.Background(), opTask, opTaskResponse, nonSignerStakesAndSignature)
		if err != nil {
			agg.logger.Error("Aggregator failed to respond to task", "task", task, "err", err)
		}
		agg.logger.Debug("Aggreagted Response sent to contract", "receipt", r)

	} else if blsAggServiceResp.TaskId.TaskType == 1 {

		rdTask, ok := task.(taskmanager.IFinalizerTaskManagerRdTask)
		if !ok {
			agg.logger.Error("FATAL: Aggregator failed to decode rdTask", "task", task)
			return
		}
		rdTaskResponse, ok := taskResponse.(taskmanager.IFinalizerTaskManagerRdTaskResponse)
		if !ok {
			agg.logger.Error("FATAL: Aggregator failed to decode rdTaskResponse", "taskResponse", taskResponse)
			return
		}
		r, err := agg.ethRpc.AvsWriter.SendAggregatedRdTaskResponse(context.Background(), rdTask, rdTaskResponse, nonSignerStakesAndSignature)
		if err != nil {
			agg.logger.Error("Aggregator failed to respond to task", "task", task, "err", err)
		}
		agg.logger.Debug("Aggreagted Response sent to contract", "receipt", r)

	} else {
		agg.logger.Error("FATAL: Aggregator failed to recognize TaskType", "blsAggServiceResp.TaskId", blsAggServiceResp.TaskId)
		return
	}

}

func (agg *Aggregator) sendNewOpTask() (taskmanager.IFinalizerTaskManagerOpTask, error) {

	agg.logger.Info("Aggregator sending new task")
	// Send number to square to the task manager contract
	newTask, taskIndex, err := agg.ethRpc.AvsWriter.SendNewOpTask(context.Background(), types.QUORUM_THRESHOLD_NUMERATOR, types.QUORUM_NUMBERS)
	if err != nil {
		agg.logger.Error("Aggregator failed to send block number to verify", "err", err)
		return newTask, err
	}

	taskId := sdktypes.TaskId{
		TaskType: sdktypes.TaskType(0),
		TaskIndex: sdktypes.TaskIndex(taskIndex),
		}

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
	agg.blsAggregationService.InitializeNewTask(taskId, newTask.LastCompletedOpTaskCreatedBlock, quorumNums, quorumThresholdPercentages, taskTimeToExpiry)
	agg.logger.Info("Aggregator initialized new operator state task", "task index", taskIndex, "expiry", taskTimeToExpiry)

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


	if isRduTask && isOpsInit{
		agg.logger.Info("Aggregator sending new task", "block number", blockNumber)
		// Send number to square to the task manager contract
		newTask, taskIndex, err := agg.ethRpc.AvsWriter.SendNewRdTask(context.Background(), big.NewInt(int64(blockNumber)))
		if err != nil {
			agg.logger.Error("Aggregator failed to send block number to verify", "err", err)
			return err
		}

		taskId := sdktypes.TaskId{
			TaskType: sdktypes.TaskType(1),
			TaskIndex: sdktypes.TaskIndex(taskIndex),
			}

		agg.tasksMu.Lock()
		agg.tasks[taskId] = newTask
		agg.tasksMu.Unlock()

		agg.kicker.TriggerNewTask(taskIndex)

		quorumThresholdPercentages := make(sdktypes.QuorumThresholdPercentages, len(newTask.LastCompletedOpTaskQuorumNumbers))
		for i, _ := range newTask.LastCompletedOpTaskQuorumNumbers {
			quorumThresholdPercentages[i] = sdktypes.QuorumThresholdPercentage(newTask.LastCompletedOpTaskQuorumThresholdPercentage)
		}
		quorumNums := make(sdktypes.QuorumNums, len(newTask.LastCompletedOpTaskQuorumNumbers))
		for i, n := range newTask.LastCompletedOpTaskQuorumNumbers {
			quorumNums[i] = sdktypes.QuorumNum(n)
		}
		taskTimeToExpiry := time.Second * time.Duration(agg.expiration)
		agg.blsAggregationService.InitializeNewTask(taskId, newTask.LastCompletedOpTaskCreatedBlock, quorumNums, quorumThresholdPercentages, taskTimeToExpiry)
		agg.logger.Info("Aggregator initialized new rolldown update task", "block number", blockNumber, "task index", taskIndex, "expiry", taskTimeToExpiry)
	}

	return nil
}

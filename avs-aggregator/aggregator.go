package aggregator

import (
	"context"
	"encoding/hex"
	"errors"
	"math/big"
	"sync"
	"time"

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
	expiration       uint32
	ethRpc           *chainio.EthRpc
	// aggregation related fields
	blsAggregationService   blsagg.BlsAggregationService
	tasks                   map[types.TaskIndex]taskmanager.IFinalizerTaskManagerTask
	tasksMu                 sync.RWMutex
	taskResponses           map[types.TaskIndex]map[sdktypes.TaskResponseDigest]taskmanager.IFinalizerTaskManagerTaskResponse
	taskResponsesMu         sync.RWMutex
	substrateClient         gsrpc.SubstrateAPI
	taskResponseWindowBlock uint32

	kicker  *Kicker
	updater *StakeUpdate
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

	taskResponseWindowBlock, err := ethRpc.AvsWriter.AvsContractBindings.TaskManager.GetTaskResponseWindowBlock(&bind.CallOpts{})
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
	updater, err := NewStakeUpdate(logger, *ethRpc, uint32(c.UpdatePeriod), uint32(c.BlockPeriod))
	if err != nil {
		logger.Error("Cannot create operator stakes updateer", "err", err)
		return nil, err
	}

	return &Aggregator{
		logger:                  logger,
		serverIpPortAddr:        c.ServerAddressPort,
		ethRpc:                  ethRpc,
		blsAggregationService:   blsAggregationService,
		tasks:                   make(map[types.TaskIndex]taskmanager.IFinalizerTaskManagerTask),
		taskResponses:           make(map[types.TaskIndex]map[sdktypes.TaskResponseDigest]taskmanager.IFinalizerTaskManagerTaskResponse),
		substrateClient:         *substrateRpc,
		taskResponseWindowBlock: taskResponseWindowBlock,
		blockPeriod:             uint32(c.BlockPeriod),
		kicker:                  kicker,
		updater:                 updater,
		expiration:              uint32(c.Expiration),
	}, nil
}

func (agg *Aggregator) Start(ctx context.Context) error {
	agg.logger.Infof("Starting aggregator.")
	agg.logger.Infof("Starting aggregator rpc server.")
	go agg.startServer(ctx)

	var sub *Subscription
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
		case <-ctx.Done():
			return nil
		case blsAggServiceResp := <-agg.blsAggregationService.GetResponseChannel():
			agg.logger.Info("Received response from blsAggregationService", "blsAggServiceResp", blsAggServiceResp)
			agg.sendAggregatedResponseToContract(blsAggServiceResp)
		case head := <-sub.Chan():
			err := agg.sendNewTask(uint32(head.Number))
			if err != nil {
				// we log the errors inside sendNewTask() so here we just continue to the next task
				continue
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

	agg.logger.Info("sending aggregated response onchain.", "taskIndex", blsAggServiceResp.TaskIndex)
	agg.tasksMu.RLock()
	task := agg.tasks[blsAggServiceResp.TaskIndex]
	agg.tasksMu.RUnlock()
	agg.taskResponsesMu.RLock()
	taskResponse := agg.taskResponses[blsAggServiceResp.TaskIndex][blsAggServiceResp.TaskResponseDigest]
	agg.taskResponsesMu.RUnlock()

	r, err := agg.ethRpc.AvsWriter.SendAggregatedResponse(context.Background(), task, taskResponse, nonSignerStakesAndSignature)
	if err != nil {
		agg.logger.Error("Aggregator failed to respond to task", "task", task, "err", err)
	}
	agg.logger.Debug("Aggreagted Response sent to contract", "receipt", r)
}

// sendNewTask sends a new task to the task manager contract, and updates the Task dict struct
// with the information of operators opted into quorum 0 at the block of task creation.
func (agg *Aggregator) sendNewTask(blockNumber uint32) error {
	if blockNumber%agg.blockPeriod != 0 {
		return nil
	}
	latest, err := agg.substrateClient.RPC.Chain.GetHeaderLatest()
	if uint32(latest.Number) != blockNumber || err != nil {
		return nil
	}

	agg.logger.Info("Aggregator sending new task", "block number", blockNumber)
	// Send number to square to the task manager contract
	newTask, taskIndex, err := agg.ethRpc.AvsWriter.SendNewTaskVerifyBlock(context.Background(), big.NewInt(int64(blockNumber)), types.QUORUM_THRESHOLD_NUMERATOR, types.QUORUM_NUMBERS)
	if err != nil {
		agg.logger.Error("Aggregator failed to send block number to verify", "err", err)
		return err
	}

	agg.tasksMu.Lock()
	agg.tasks[taskIndex] = newTask
	agg.tasksMu.Unlock()

	agg.kicker.TriggerNewTask(taskIndex)
	agg.updater.TriggerNewTask(taskIndex)

	quorumThresholdPercentages := make(sdktypes.QuorumThresholdPercentages, len(newTask.QuorumNumbers))
	for i, _ := range newTask.QuorumNumbers {
		quorumThresholdPercentages[i] = sdktypes.QuorumThresholdPercentage(newTask.QuorumThresholdPercentage)
	}
	quorumNums := make(sdktypes.QuorumNums, len(newTask.QuorumNumbers))
	for i, n := range newTask.QuorumNumbers {
		quorumNums[i] = sdktypes.QuorumNum(n)
	}
	taskTimeToExpiry := time.Second * time.Duration(agg.expiration)
	agg.blsAggregationService.InitializeNewTask(taskIndex, newTask.TaskCreatedBlock, quorumNums, quorumThresholdPercentages, taskTimeToExpiry)
	agg.logger.Info("Aggregator initialized new task", "block number", blockNumber, "task index", taskIndex, "expiry", taskTimeToExpiry)
	return nil
}

package operator

import (
	"context"
	"fmt"
	"math/big"

	"github.com/ethereum/go-ethereum/common"
	"github.com/prometheus/client_golang/prometheus"

	"github.com/mangata-finance/eigen-layer-monorepo/aggregator"
	taskmanager "github.com/mangata-finance/eigen-layer-monorepo/contracts/bindings/MangataTaskManager"
	"github.com/mangata-finance/eigen-layer-monorepo/core"
	"github.com/mangata-finance/eigen-layer-monorepo/core/chainio"
	"github.com/mangata-finance/eigen-layer-monorepo/metrics"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	"github.com/Layr-Labs/eigensdk-go/logging"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	sdkmetrics "github.com/Layr-Labs/eigensdk-go/metrics"
	"github.com/Layr-Labs/eigensdk-go/metrics/collectors/economic"
	rpccalls "github.com/Layr-Labs/eigensdk-go/metrics/collectors/rpc_calls"
	"github.com/Layr-Labs/eigensdk-go/nodeapi"
	sdktypes "github.com/Layr-Labs/eigensdk-go/types"

	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
)

const AVS_NAME = "mangata-finalizer"
const SEM_VER = "0.0.1"

type Operator struct {
	config          Config
	logger          logging.Logger
	substrateClient *gsrpc.SubstrateAPI
	metricsReg      *prometheus.Registry
	metrics         metrics.Metrics
	nodeApi         *nodeapi.NodeApi
	ethRpc          *chainio.EthRpc
	operatorId      bls.OperatorId
	operatorAddr    common.Address
	// receive new tasks in this chan (typically from listening to onchain event)
	newTaskCreatedChan chan *taskmanager.ContractMangataTaskManagerNewTaskCreated
	// rpc client to send signed task responses to aggregator
	aggregatorRpcClient AggregatorRpcClienter
}

func NewOperatorFromConfig(c Config) (*Operator, error) {
	logger, err := sdklogging.NewZapLogger(c.LogLevel)
	if err != nil {
		return nil, err
	}

	operatorAddr := c.EcdsaSigner.GetTxOpts().From

	reg := prometheus.NewRegistry()
	eigenMetrics := sdkmetrics.NewEigenMetrics(AVS_NAME, c.EigenMetricsIpPortAddress, reg, logger)
	avsAndEigenMetrics := metrics.NewAvsAndEigenMetrics(AVS_NAME, eigenMetrics, reg)

	// Setup Node Api
	nodeApi := nodeapi.NewNodeApi(AVS_NAME, SEM_VER, c.NodeApiIpPortAddress, logger)

	var ethRpcClient, ethWsClient eth.EthClient
	if c.EnableMetrics {
		rpcCallsCollector := rpccalls.NewCollector(AVS_NAME, reg)
		ethRpcClient, err = eth.NewInstrumentedClient(c.EthRpcUrl, rpcCallsCollector)
		if err != nil {
			logger.Errorf("Cannot create http ethclient", "err", err)
			return nil, err
		}
		ethWsClient, err = eth.NewInstrumentedClient(c.EthWsUrl, rpcCallsCollector)
		if err != nil {
			logger.Errorf("Cannot create ws ethclient", "err", err)
			return nil, err
		}
	} else {
		ethRpcClient, err = eth.NewClient(c.EthRpcUrl)
		if err != nil {
			logger.Errorf("Cannot create http ethclient", "err", err)
			return nil, err
		}
		ethWsClient, err = eth.NewClient(c.EthWsUrl)
		if err != nil {
			logger.Errorf("Cannot create ws ethclient", "err", err)
			return nil, err
		}
	}

	ethRpc, err := chainio.NewEthRpc(c.ServiceManagerAddr, c.BlsOperatorStateRetrieverAddr, c.BlsCompendiumAddr, ethRpcClient, ethWsClient, c.EcdsaSigner, logger)
	if err != nil {
		logger.Error("Cannot create ethRpc", "err", err)
		return nil, err
	}

	// We must register the economic metrics separately because they are exported metrics (from jsonrpc or subgraph calls)
	// and not instrumented metrics: see https://prometheus.io/docs/instrumenting/writing_clientlibs/#overall-structure
	quorumNames := map[sdktypes.QuorumNum]string{
		0: "quorum0",
	}
	economicMetricsCollector := economic.NewCollector(ethRpc.ElReader, ethRpc.AvsReader.AvsRegistryReader, AVS_NAME, logger, operatorAddr, quorumNames)
	reg.MustRegister(economicMetricsCollector)

	aggregatorRpcClient, err := NewAggregatorRpcClient(c.AggregatorUrl, logger, avsAndEigenMetrics)
	if err != nil {
		logger.Error("Cannot create AggregatorRpcClient. Is aggregator running?", "err", err)
		return nil, err
	}

	api, err := gsrpc.NewSubstrateAPI(c.SubstrateWsRpcUrl)
	if err != nil {
		logger.Error("Cannot create substrate RPC", "url", c.SubstrateWsRpcUrl, "err", err)
		return nil, err
	}

	// OperatorId is set in contract during registration so we get it after registering operator.
	operatorId, err := ethRpc.AvsReader.AvsRegistryReader.GetOperatorId(context.Background(), operatorAddr)
	if err != nil {
		logger.Error("Cannot get operator id", "err", err)
		return nil, err
	}

	operator := &Operator{
		config:              c,
		logger:              logger,
		metricsReg:          reg,
		metrics:             avsAndEigenMetrics,
		nodeApi:             nodeApi,
		substrateClient:     api,
		ethRpc:              ethRpc,
		aggregatorRpcClient: aggregatorRpcClient,
		newTaskCreatedChan:  make(chan *taskmanager.ContractMangataTaskManagerNewTaskCreated),
		operatorId:          operatorId,
		operatorAddr:        operatorAddr,
	}

	operator.PrintOperatorStatus()

	// used for local CI deploy
	if c.RegisterAtStartup && operatorId == [32]byte{} {
		operator.RegisterAtStartup()
		
		operatorId, err := ethRpc.AvsReader.AvsRegistryReader.GetOperatorId(context.Background(), operatorAddr)
		if err != nil {
			logger.Error("Cannot get operator id", "err", err)
			return nil, err
		}
		operator.operatorId = operatorId

		err = operator.PrintOperatorStatus()
		if err != nil {
			operator.logger.Error("Error while printing operator status")
		}
	}

	return operator, nil
}

func (o *Operator) Start(ctx context.Context) error {
	o.logger.Infof("Starting operator.")

	operatorIsRegistered := o.operatorId != [32]byte{}
	if !operatorIsRegistered {
		return fmt.Errorf("operator is not registered. Registering operator using the operator-cli before starting operator")
	}

	if o.config.EnableNodeApi {
		o.nodeApi.Start()
	}
	var metricsErrChan <-chan error
	if o.config.EnableMetrics {
		metricsErrChan = o.metrics.Start(ctx, o.metricsReg)
	} else {
		metricsErrChan = make(chan error, 1)
	}

	// TODO(samlaf): wrap this call with increase in avs-node-spec metric
	sub := o.ethRpc.AvsSubscriber.SubscribeToNewTasks(o.newTaskCreatedChan)
	for {
		select {
		case <-ctx.Done():
			return nil
		case err := <-metricsErrChan:
			// TODO(samlaf); we should also register the service as unhealthy in the node api
			// https://eigen.nethermind.io/docs/spec/api/
			o.logger.Fatal("Error in metrics server", "err", err)
		case err := <-sub.Err():
			o.logger.Error("Error in websocket subscription", "err", err)
			// TODO(samlaf): write unit tests to check if this fixed the issues we were seeing
			sub.Unsubscribe()
			// TODO(samlaf): wrap this call with increase in avs-node-spec metric
			sub = o.ethRpc.AvsSubscriber.SubscribeToNewTasks(o.newTaskCreatedChan)
		case newTaskCreatedLog := <-o.newTaskCreatedChan:
			o.metrics.IncNumTasksReceived()
			taskResponse, err := o.ProcessNewTaskCreatedLog(newTaskCreatedLog)
			if err != nil {
				continue
			}
			signedTaskResponse, err := o.SignTaskResponse(taskResponse)
			if err != nil {
				continue
			}
			go o.aggregatorRpcClient.SendSignedTaskResponseToAggregator(signedTaskResponse)
		}
	}
}

// Takes a NewTaskCreatedLog struct as input and returns a TaskResponseHeader struct.
// The TaskResponseHeader struct is the struct that is signed and sent to the contract as a task response.
func (o *Operator) ProcessNewTaskCreatedLog(newTaskCreatedLog *taskmanager.ContractMangataTaskManagerNewTaskCreated) (*taskmanager.IMangataTaskManagerTaskResponse, error) {
	o.logger.Debug("Received new task", "task", newTaskCreatedLog)
	o.logger.Info("Received new task",
		"blockNumber", newTaskCreatedLog.Task.BlockNumber,
		"taskIndex", newTaskCreatedLog.TaskIndex,
		"taskCreatedBlock", newTaskCreatedLog.Task.TaskCreatedBlock,
		"quorumNumbers", newTaskCreatedLog.Task.QuorumNumbers,
		"QuorumThresholdPercentage", newTaskCreatedLog.Task.QuorumThresholdPercentage,
	)

	verifyBlockNum := newTaskCreatedLog.Task.BlockNumber

	sub, err := o.substrateClient.RPC.Chain.SubscribeFinalizedHeads()
	if err != nil {
		o.logger.Error("Failed to get new head from substrate", "err", err)
		panic(err)
	}
	defer sub.Unsubscribe()

	for {
		head := <-sub.Chan()
		finalizedBlockNumber := big.NewInt(int64(head.Number))
		if finalizedBlockNumber.Cmp(verifyBlockNum) >= 0 {
			sub.Unsubscribe()
			break
		}
	}

	hash, err := o.substrateClient.RPC.Chain.GetBlockHash(verifyBlockNum.Uint64())
	if err != nil {
		o.logger.Error("Cannot fetch block hash", "blockNumber", verifyBlockNum, "err", err)
		return nil, err
	}
	// block, err := o.substrateClient.RPC.Chain.GetBlock(hash)
	// if err != nil {
	// 	o.logger.Error("Cannot fetch block", "block hash", hash, "err", err)
	// 	return nil, err
	// }

	taskResponse := &taskmanager.IMangataTaskManagerTaskResponse{
		ReferenceTaskIndex: newTaskCreatedLog.TaskIndex,
		// StateRoot:          block.Block.Header.StateRoot,
		BlockHash: hash,
	}
	o.logger.Debug("TaskResponse processed", "block hash", common.Bytes2Hex(taskResponse.BlockHash[:]))
	return taskResponse, nil
}

func (o *Operator) SignTaskResponse(taskResponse *taskmanager.IMangataTaskManagerTaskResponse) (*aggregator.SignedTaskResponse, error) {
	taskResponseHash, err := core.GetTaskResponseDigest(taskResponse)
	if err != nil {
		o.logger.Error("Error getting task response header hash. skipping task (this is not expected and should be investigated)", "err", err)
		return nil, err
	}
	blsSignature := o.config.BlsKeyPair.SignMessage(taskResponseHash)
	signedTaskResponse := &aggregator.SignedTaskResponse{
		TaskResponse: *taskResponse,
		BlsSignature: *blsSignature,
		OperatorId:   o.operatorId,
	}
	o.logger.Debug("Signed task response", "signedTaskResponse", signedTaskResponse)
	return signedTaskResponse, nil
}

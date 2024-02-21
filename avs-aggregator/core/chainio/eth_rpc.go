package chainio

import (
	"github.com/Layr-Labs/eigensdk-go/chainio/clients"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/avsregistry"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/elcontracts"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/chainio/txmgr"
	"github.com/Layr-Labs/eigensdk-go/chainio/utils"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/metrics"
	rpccalls "github.com/Layr-Labs/eigensdk-go/metrics/collectors/rpc_calls"
	"github.com/Layr-Labs/eigensdk-go/signerv2"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/prometheus/client_golang/prometheus"
)

type EthRpc struct {
	AvsReader     *AvsReader
	AvsWriter     *AvsWriter
	AvsSubscriber *AvsSubscriber
	ElClients     *clients.Clients
	Client        eth.EthClient
}

func NewEthRpc(
	registryAddr common.Address,
	ethHttpUrl string,
	ethWsUrl string,
	signer signerv2.SignerFn,
	address common.Address,
	avsName string,
	metricsIpPort string,
	logger sdklogging.Logger,
) (*EthRpc, error) {
	// Create the metrics server
	promReg := prometheus.NewRegistry()
	eigenMetrics := metrics.NewEigenMetrics(avsName, metricsIpPort, promReg, logger)

	// creating two types of Eth clients: HTTP and WS
	var ethHttpClient, ethWsClient eth.EthClient
	rpcCallsCollector := rpccalls.NewCollector(avsName, promReg)
	ethHttpClient, err := eth.NewInstrumentedClient(ethHttpUrl, rpcCallsCollector)
	if err != nil {
		logger.Error("Failed to create Eth Http client", "err", err)
		return nil, err
	}

	ethWsClient, err = eth.NewInstrumentedClient(ethWsUrl, rpcCallsCollector)
	if err != nil {
		logger.Error("Failed to create Eth WS client", "err", err)
		return nil, err
	}

	txMgr := txmgr.NewSimpleTxManager(ethHttpClient, logger, signer, address)
	avsReader, err := NewAvsReaderFromConfig(registryAddr, ethHttpClient, logger)
	if err != nil {
		logger.Error("Cannot create AvsReader", "err", err)
		return nil, err
	}

	avsWriter, err := NewAvsWriter(txMgr, registryAddr, ethHttpClient, logger)
	if err != nil {
		logger.Error("Cannot create AvsWriter", "err", err)
		return nil, err
	}

	avsSubscriber, err := NewAvsSubscriber(registryAddr, ethWsClient, logger)
	if err != nil {
		logger.Error("Cannot create AvsSubscriber", "err", err)
		return nil, err
	}

	// creating EL clients: Reader, Writer and Subscriber
	avsRegistryContractBindings, err := utils.NewAVSRegistryContractBindings(
		registryAddr,
		avsReader.AvsServiceBindings.OperatorStateRetriever,
		ethHttpClient,
		logger,
	)
	if err != nil {
		logger.Error("Failed to create AVSRegistryContractBindings", "err", err)
		return nil, err
	}

	delegationAddr, err := avsRegistryContractBindings.StakeRegistry.Delegation(&bind.CallOpts{})
	if err != nil {
		logger.Fatal("Failed to fetch Delegation contract address", "err", err)
	}

	elContractBindings, err := utils.NewEigenlayerContractBindings(
		delegationAddr,
		ethHttpClient,
		logger,
	)
	if err != nil {
		logger.Error("Failed to create EigenlayerContractBindings", "err", err)
		return nil, err
	}

	// get the Reader for the EL contracts
	elChainReader := elcontracts.NewELChainReader(
		elContractBindings.Slasher,
		elContractBindings.DelegationManager,
		elContractBindings.StrategyManager,
		logger,
		ethHttpClient,
	)

	elChainWriter := elcontracts.NewELChainWriter(
		elContractBindings.Slasher,
		elContractBindings.DelegationManager,
		elContractBindings.StrategyManager,
		elContractBindings.StrategyManagerAddr,
		elChainReader,
		ethHttpClient,
		logger,
		eigenMetrics,
		txMgr,
	)
	if err != nil {
		logger.Error("Failed to create ELChainWriter", "err", err)
		return nil, err
	}

	// creating AVS clients: Reader and Writer
	avsRegistryChainReader, err := avsregistry.BuildAvsRegistryChainReader(
		avsRegistryContractBindings.RegistryCoordinatorAddr,
		avsRegistryContractBindings.OperatorStateRetrieverAddr,
		ethHttpClient,
		logger,
	)
	if err != nil {
		logger.Error("Failed to create AVSRegistryChainReader", "err", err)
		return nil, err
	}

	avsRegistryChainWriter, err := avsregistry.BuildAvsRegistryChainWriter(
		avsRegistryContractBindings.RegistryCoordinatorAddr,
		avsRegistryContractBindings.OperatorStateRetrieverAddr,
		logger,
		ethHttpClient,
		txMgr,
	)
	if err != nil {
		logger.Error("Failed to create AVSRegistryChainWriter", "err", err)
		return nil, err
	}

	avsRegistryChainSubscriber, err := avsregistry.BuildAvsRegistryChainSubscriber(
		avsRegistryContractBindings.BlsApkRegistryAddr,
		ethWsClient,
		logger,
	)
	if err != nil {
		logger.Error("Failed to create avsRegistryChainSubscriber", "err", err)
		return nil, err
	}

	return &EthRpc{
		AvsReader:     avsReader,
		AvsWriter:     avsWriter,
		AvsSubscriber: avsSubscriber,
		ElClients: &clients.Clients{
			AvsRegistryChainReader:     avsRegistryChainReader,
			AvsRegistryChainWriter:     avsRegistryChainWriter,
			AvsRegistryChainSubscriber: avsRegistryChainSubscriber,
			ElChainReader:              elChainReader,
			ElChainWriter:              elChainWriter,
			Metrics:                    eigenMetrics,
			PrometheusRegistry:         promReg,
		},
		Client: ethHttpClient,
	}, nil
}

package chainio

import (
	"github.com/Layr-Labs/eigensdk-go/chainio/clients"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/avsregistry"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/elcontracts"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/chainio/txmgr"
	"github.com/Layr-Labs/eigensdk-go/chainio/utils"
	blspubkeycompendium "github.com/Layr-Labs/eigensdk-go/contracts/bindings/BLSPublicKeyCompendium"
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
	serviceManagerAddr common.Address,
	blsOperatorStateRetrieverAddr common.Address,
	blsCompendiumAddr common.Address,
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
	avsReader, err := NewAvsReaderFromConfig(serviceManagerAddr, blsOperatorStateRetrieverAddr, ethHttpClient, logger)
	if err != nil {
		logger.Error("Cannot create AvsReader", "err", err)
		return nil, err
	}

	avsWriter, err := NewAvsWriter(txMgr, serviceManagerAddr, blsCompendiumAddr, ethHttpClient, logger)
	if err != nil {
		logger.Error("Cannot create AvsWriter", "err", err)
		return nil, err
	}

	avsSubscriber, err := NewAvsSubscriber(serviceManagerAddr, blsOperatorStateRetrieverAddr, ethWsClient, logger)
	if err != nil {
		logger.Error("Cannot create AvsSubscriber", "err", err)
		return nil, err
	}

	// creating EL clients: Reader, Writer and Subscriber
	avsRegistryContractBindings, err := utils.NewAVSRegistryContractBindings(
		avsReader.AvsServiceBindings.RegistryCoordinator,
		blsOperatorStateRetrieverAddr,
		ethHttpClient,
		logger,
	)
	if err != nil {
		logger.Error("Failed to create AVSRegistryContractBindings", "err", err)
		return nil, err
	}

	slasherAddr, err := avsRegistryContractBindings.RegistryCoordinator.Slasher(&bind.CallOpts{})
	if err != nil {
		logger.Fatal("Failed to fetch Slasher contract", "err", err)
	}

	elContractBindings, err := utils.NewEigenlayerContractBindings(
		slasherAddr,
		avsRegistryContractBindings.BlsPubkeyCompendiumAddr,
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
		elContractBindings.BlsPubkeyCompendium,
		elContractBindings.BlspubkeyCompendiumAddr,
		logger,
		ethHttpClient,
	)

	// get the Subscriber for the EL contracts
	contractBlsPubkeyCompendiumWs, err := blspubkeycompendium.NewContractBLSPublicKeyCompendium(
		elContractBindings.BlspubkeyCompendiumAddr,
		ethWsClient,
	)
	if err != nil {
		logger.Fatal("Failed to fetch BLSPublicKeyCompendium contract", "err", err)
	}
	elChainSubscriber, err := elcontracts.NewELChainSubscriber(
		contractBlsPubkeyCompendiumWs,
		logger,
	)
	if err != nil {
		logger.Error("Failed to create ELChainSubscriber", "err", err)
		return nil, err
	}

	elChainWriter := elcontracts.NewELChainWriter(
		elContractBindings.Slasher,
		elContractBindings.DelegationManager,
		elContractBindings.StrategyManager,
		elContractBindings.StrategyManagerAddr,
		elContractBindings.BlsPubkeyCompendium,
		elContractBindings.BlspubkeyCompendiumAddr,
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
	avsRegistryChainReader, err := avsregistry.NewAvsRegistryReader(
		avsRegistryContractBindings.RegistryCoordinatorAddr,
		avsRegistryContractBindings.RegistryCoordinator,
		avsRegistryContractBindings.BlsOperatorStateRetriever,
		avsRegistryContractBindings.StakeRegistry,
		avsRegistryContractBindings.IndexRegistry,
		logger,
		ethHttpClient,
	)
	if err != nil {
		logger.Error("Failed to create AVSRegistryChainReader", "err", err)
		return nil, err
	}

	avsRegistryChainWriter, err := avsregistry.NewAvsRegistryWriter(
		avsRegistryContractBindings.RegistryCoordinator,
		avsRegistryContractBindings.BlsOperatorStateRetriever,
		avsRegistryContractBindings.StakeRegistry,
		avsRegistryContractBindings.BlsPubkeyRegistry,
		logger,
		ethHttpClient,
		txMgr,
	)
	if err != nil {
		logger.Error("Failed to create AVSRegistryChainWriter", "err", err)
		return nil, err
	}

	return &EthRpc{
		AvsReader:     avsReader,
		AvsWriter:     avsWriter,
		AvsSubscriber: avsSubscriber,
		ElClients: &clients.Clients{
			AvsRegistryChainReader: avsRegistryChainReader,
			AvsRegistryChainWriter: avsRegistryChainWriter,
			ElChainReader:          elChainReader,
			ElChainWriter:          elChainWriter,
			ElChainSubscriber:      elChainSubscriber,
			Metrics:                eigenMetrics,
			PrometheusRegistry:     promReg,
		},
		Client: ethHttpClient,
	}, nil
}

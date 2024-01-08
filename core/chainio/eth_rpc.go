package chainio

import (
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/chainio/elcontracts"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/metrics"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"

	sdkclients "github.com/Layr-Labs/eigensdk-go/chainio/clients"
	"github.com/Layr-Labs/eigensdk-go/signer"
)

type EthRpc struct {
	AvsReader     *AvsReader
	AvsWriter     *AvsWriter
	AvsSubscriber *AvsSubscriber
	ElReader      *elcontracts.ELChainReader
	ElWriter      *elcontracts.ELChainWriter
	ElSubscriber  *elcontracts.ELChainSubscriber
	Client        eth.EthClient
}

func NewEthRpc(
	serviceManagerAddr common.Address,
	blsOperatorStateRetrieverAddr common.Address,
	blsCompendiumAddr common.Address,
	ethRpcClient eth.EthClient,
	ethWsClient eth.EthClient,
	signer signer.Signer,
	logger sdklogging.Logger,
) (*EthRpc, error) {
	avsReader, err := NewAvsReaderFromConfig(serviceManagerAddr, blsOperatorStateRetrieverAddr, ethRpcClient, logger)
	if err != nil {
		logger.Error("Cannot create AvsReader", "err", err)
		return nil, err
	}

	avsWriter, err := NewAvsWriter(signer, serviceManagerAddr, blsCompendiumAddr, ethRpcClient, logger)
	if err != nil {
		logger.Error("Cannot create AvsWriter", "err", err)
		return nil, err
	}

	avsSubscriber, err := NewAvsSubscriber(serviceManagerAddr, blsOperatorStateRetrieverAddr, ethWsClient, logger)
	if err != nil {
		logger.Error("Cannot create AvsSubscriber", "err", err)
		return nil, err
	}

	slasherAddr, err := avsReader.AvsServiceBindings.ServiceManager.Slasher(&bind.CallOpts{})
	if err != nil {
		logger.Error("Cannot get slasher address", "err", err)
		return nil, err
	}

	elContractsClient, err := sdkclients.NewELContractsChainClient(slasherAddr, blsCompendiumAddr, ethRpcClient, ethWsClient, logger)
	if err != nil {
		logger.Error("Cannot create ELContractsChainClient", "err", err)
		return nil, err
	}

	eigenlayerReader, err := elcontracts.NewELChainReader(elContractsClient, logger, ethRpcClient)
	if err != nil {
		logger.Error("Cannot create EigenLayerReader", "err", err)
		return nil, err
	}

	eigenlayerWriter := elcontracts.NewELChainWriter(elContractsClient, ethRpcClient, signer, logger, metrics.NewNoopMetrics())

	eigenSubscriber, err := elcontracts.NewELChainSubscriber(elContractsClient, logger)
	if err != nil {
		logger.Error("Cannot create EigenLayerWriter", "err", err)
		return nil, err
	}

	return &EthRpc{
		AvsReader:     avsReader,
		AvsWriter:     avsWriter,
		AvsSubscriber: avsSubscriber,
		ElReader:      eigenlayerReader,
		ElWriter:      eigenlayerWriter,
		ElSubscriber:  eigenSubscriber,
		Client:        ethRpcClient,
	}, nil
}

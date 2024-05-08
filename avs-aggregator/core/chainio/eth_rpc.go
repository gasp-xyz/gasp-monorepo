package chainio

import (
	"github.com/Layr-Labs/eigensdk-go/chainio/clients"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/signerv2"

	"github.com/ethereum/go-ethereum/common"
)

type EthRpc struct {
	AvsReader     *AvsReader
	AvsWriter     *AvsWriter
	AvsSubscriber *AvsSubscriber
	Clients       *clients.Clients
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

	// tmp to get OperatorStateRetriever address
	client, err := eth.NewClient(ethHttpUrl)
	if err != nil {
		logger.Error("Failed to create Eth Http client", "err", err)
		return nil, err
	}
	avs, err := NewAvsReaderFromConfig(registryAddr, client, logger)
	if err != nil {
		logger.Error("Cannot create AvsReader", "err", err)
		return nil, err
	}

	config := clients.BuildAllConfig{
		EthHttpUrl:                 ethHttpUrl,
		EthWsUrl:                   ethWsUrl,
		RegistryCoordinatorAddr:    registryAddr.String(),
		OperatorStateRetrieverAddr: avs.AvsServiceBindings.OperatorStateRetriever.String(),
		AvsName:                    avsName,
		PromMetricsIpPortAddress:   metricsIpPort,
	}
	clients, err := clients.BuildAll(config, address, signer, logger)
	if err != nil {
		logger.Error("Cannot create eth Clients", "err", err)
		return nil, err
	}

	avsReader, err := NewAvsReaderFromConfig(registryAddr, clients.EthHttpClient, logger)
	if err != nil {
		logger.Error("Cannot create AvsReader", "err", err)
		return nil, err
	}

	avsWriter, err := NewAvsWriter(clients.TxMgr, registryAddr, clients.EthHttpClient, logger)
	if err != nil {
		logger.Error("Cannot create AvsWriter", "err", err)
		return nil, err
	}

	avsSubscriber, err := NewAvsSubscriber(registryAddr, clients.EthWsClient, logger)
	if err != nil {
		logger.Error("Cannot create AvsSubscriber", "err", err)
		return nil, err
	}

	return &EthRpc{
		AvsReader:     avsReader,
		AvsWriter:     avsWriter,
		AvsSubscriber: avsSubscriber,
		Clients:       clients,
	}, nil
}

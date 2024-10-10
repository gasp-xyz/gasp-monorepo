package chainio

import (
	"fmt"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/signerv2"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/ethclient"
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
	client, err := ethclient.Dial(ethHttpUrl)
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

	httpethclient, ok := (clients.EthHttpClient).(*ethclient.Client)
	if !ok {
		logger.Error("EthHttpClient assertion failed")
		return nil, fmt.Errorf("EthHttpClient assertion failed")
	}
	wsethclient, ok := (clients.EthWsClient).(*ethclient.Client)
	if !ok {
		logger.Error("EthHttpClient assertion failed")
		return nil, fmt.Errorf("EthHttpClient assertion failed")
	}

	avsReader, err := NewAvsReaderFromConfig(registryAddr, httpethclient, logger)
	if err != nil {
		logger.Error("Cannot create AvsReader", "err", err)
		return nil, err
	}

	avsWriter, err := NewAvsWriter(clients.TxManager, registryAddr, httpethclient, logger)
	if err != nil {
		logger.Error("Cannot create AvsWriter", "err", err)
		return nil, err
	}

	avsSubscriber, err := NewAvsSubscriber(registryAddr, wsethclient, logger)
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

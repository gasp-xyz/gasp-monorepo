package chainio

import (
	"context"
	"fmt"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients/avsregistry"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/signerv2"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/ethereum/go-ethereum/core/types"
	sdktypes "github.com/Layr-Labs/eigensdk-go/types"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
)


type EthRpcClientsInterface interface {
	BlockNumber(ctx context.Context) (uint64, error)
	GetOperatorFromId(
		opts *bind.CallOpts,
		operatorId sdktypes.OperatorId,
	) (common.Address, error)
	GetOperatorId(
		opts *bind.CallOpts,
		operatorAddress common.Address,
	) ([32]byte, error)
	UpdateStakesOfEntireOperatorSetForQuorums(
		ctx context.Context,
		operatorsPerQuorum [][]common.Address,
		quorumNumbers sdktypes.QuorumNums,
		waitForReceipt bool,
	) (*types.Receipt, error)
	UpdateStakesOfOperatorSubsetForAllQuorums(
		ctx context.Context,
		operators []common.Address,
		waitForReceipt bool,
	) (*types.Receipt, error)
	SenderAddress(ctx context.Context) (common.Address, error)
	AvsRegistryChainReader() (*avsregistry.ChainReader)
	AvsRegistryChainSubscriber() (*avsregistry.ChainSubscriber)
}

type EthRpcClientsWrapper struct {
	Clients       *clients.Clients
}

func (c *EthRpcClientsWrapper) BlockNumber (ctx context.Context) (uint64, error){
	v, e := c.Clients.EthHttpClient.BlockNumber(ctx)
	return v, e
}

func (c *EthRpcClientsWrapper) GetOperatorFromId(
	opts *bind.CallOpts,
	operatorId sdktypes.OperatorId,
) (common.Address, error) {
	v, e := c.Clients.AvsRegistryChainReader.GetOperatorFromId(opts, operatorId)
	return v, e
}

func (c *EthRpcClientsWrapper) GetOperatorId(
	opts *bind.CallOpts,
	operatorAddress common.Address,
) ([32]byte, error) {
	v, e := c.Clients.AvsRegistryChainReader.GetOperatorId(opts, operatorAddress)
	return v, e
}

func (c *EthRpcClientsWrapper) UpdateStakesOfEntireOperatorSetForQuorums(
	ctx context.Context,
	operatorsPerQuorum [][]common.Address,
	quorumNumbers sdktypes.QuorumNums,
	waitForReceipt bool,
) (*types.Receipt, error){
	v, e := c.Clients.AvsRegistryChainWriter.UpdateStakesOfEntireOperatorSetForQuorums(ctx, operatorsPerQuorum, quorumNumbers, waitForReceipt)
	return v, e
}

func (c *EthRpcClientsWrapper) UpdateStakesOfOperatorSubsetForAllQuorums(
	ctx context.Context,
	operators []common.Address,
	waitForReceipt bool,
) (*types.Receipt, error){
	v, e := c.Clients.AvsRegistryChainWriter.UpdateStakesOfOperatorSubsetForAllQuorums(ctx, operators, waitForReceipt)
	return v, e
}

func (c *EthRpcClientsWrapper) SenderAddress(ctx context.Context) (common.Address, error) {
	v, e := c.Clients.Wallet.SenderAddress(ctx)
	return v, e
}

func (c *EthRpcClientsWrapper) AvsRegistryChainReader() (*avsregistry.ChainReader) {
	return c.Clients.AvsRegistryChainReader
}

func (c *EthRpcClientsWrapper) AvsRegistryChainSubscriber() (*avsregistry.ChainSubscriber) {
	return c.Clients.AvsRegistryChainSubscriber
}

var _ EthRpcClientsInterface = (*EthRpcClientsWrapper)(nil)

type EthRpc struct {
	AvsReader     AvsReaderer
	AvsWriter     AvsWriterer
	AvsSubscriber AvsSubscriberer
	Clients       EthRpcClientsInterface
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
	aggSSFetchTimeout int,
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

	avsSubscriber, err := NewAvsSubscriber(registryAddr, wsethclient, logger, aggSSFetchTimeout)
	if err != nil {
		logger.Error("Cannot create AvsSubscriber", "err", err)
		return nil, err
	}

	return &EthRpc{
		AvsReader:     avsReader,
		AvsWriter:     avsWriter,
		AvsSubscriber: avsSubscriber,
		Clients:       &EthRpcClientsWrapper{Clients: clients},
	}, nil
}
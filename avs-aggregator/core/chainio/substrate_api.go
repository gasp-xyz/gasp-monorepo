package chainio

// DO NOT DELETE unless using the mockery version in mocks package works 

// import (
// 	"context"
// 	"fmt"

// 	"github.com/Layr-Labs/eigensdk-go/chainio/clients/avsregistry"
// 	"github.com/Layr-Labs/eigensdk-go/chainio/clients"
// 	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
// 	"github.com/Layr-Labs/eigensdk-go/signerv2"

// 	"github.com/ethereum/go-ethereum/common"
// 	"github.com/ethereum/go-ethereum/ethclient"
// 	"github.com/ethereum/go-ethereum/core/types"
// 	sdktypes "github.com/Layr-Labs/eigensdk-go/types"
// 	"github.com/ethereum/go-ethereum/accounts/abi/bind"
// )


// type SubstrateAPIInterface interface {
// 	BlockNumber(ctx context.Context) (uint64, error)
// 	GetOperatorFromId(
// 		opts *bind.CallOpts,
// 		operatorId sdktypes.OperatorId,
// 	) (common.Address, error)
// 	GetOperatorId(
// 		opts *bind.CallOpts,
// 		operatorAddress common.Address,
// 	) ([32]byte, error)
// 	UpdateStakesOfEntireOperatorSetForQuorums(
// 		ctx context.Context,
// 		operatorsPerQuorum [][]common.Address,
// 		quorumNumbers sdktypes.QuorumNums,
// 		waitForReceipt bool,
// 	) (*types.Receipt, error)
// 	UpdateStakesOfOperatorSubsetForAllQuorums(
// 		ctx context.Context,
// 		operators []common.Address,
// 		waitForReceipt bool,
// 	) (*types.Receipt, error)
// 	SenderAddress(ctx context.Context) (common.Address, error)
// 	AvsRegistryChainReader() (*avsregistry.ChainReader)
// 	AvsRegistryChainSubscriber() (*avsregistry.ChainSubscriber)
// }

// type SubstrateAPIWrapper struct {
// 	substrateApi       *gsrpc.SubstrateAPI
// }

// func (c *EthRpcClientsWrapper) BlockNumber (ctx context.Context) (uint64, error){
// 	v, e := c.Clients.EthHttpClient.BlockNumber(ctx)
// 	return v, e
// }

// func (c *EthRpcClientsWrapper) GetOperatorFromId(
// 	opts *bind.CallOpts,
// 	operatorId sdktypes.OperatorId,
// ) (common.Address, error) {
// 	v, e := c.Clients.AvsRegistryChainReader.GetOperatorFromId(opts, operatorId)
// 	return v, e
// }

// func (c *EthRpcClientsWrapper) GetOperatorId(
// 	opts *bind.CallOpts,
// 	operatorAddress common.Address,
// ) ([32]byte, error) {
// 	v, e := c.Clients.AvsRegistryChainReader.GetOperatorId(opts, operatorAddress)
// 	return v, e
// }

// func (c *EthRpcClientsWrapper) UpdateStakesOfEntireOperatorSetForQuorums(
// 	ctx context.Context,
// 	operatorsPerQuorum [][]common.Address,
// 	quorumNumbers sdktypes.QuorumNums,
// 	waitForReceipt bool,
// ) (*types.Receipt, error){
// 	v, e := c.Clients.AvsRegistryChainWriter.UpdateStakesOfEntireOperatorSetForQuorums(ctx, operatorsPerQuorum, quorumNumbers, waitForReceipt)
// 	return v, e
// }

// func (c *EthRpcClientsWrapper) UpdateStakesOfOperatorSubsetForAllQuorums(
// 	ctx context.Context,
// 	operators []common.Address,
// 	waitForReceipt bool,
// ) (*types.Receipt, error){
// 	v, e := c.Clients.AvsRegistryChainWriter.UpdateStakesOfOperatorSubsetForAllQuorums(ctx, operators, waitForReceipt)
// 	return v, e
// }

// func (c *EthRpcClientsWrapper) SenderAddress(ctx context.Context) (common.Address, error) {
// 	v, e := c.Clients.Wallet.SenderAddress(ctx)
// 	return v, e
// }

// func (c *EthRpcClientsWrapper) AvsRegistryChainReader() (*avsregistry.ChainReader) {
// 	return c.Clients.AvsRegistryChainReader
// }

// func (c *EthRpcClientsWrapper) AvsRegistryChainSubscriber() (*avsregistry.ChainSubscriber) {
// 	return c.Clients.AvsRegistryChainSubscriber
// }

// var _ SubstrateAPIInterface = (*SubstrateAPIWrapper)(nil)

// func NewSubstrateAPIWrapper(
// 	substrateWsRpcUrl string,
// ) (*SubstrateAPIWrapper, error) {
// 	substrateApi, err := gsrpc.NewSubstrateAPI(substrateWsRpcUrl)
// 	if err != nil {
// 		logger.Error("Cannot create substrate RPC", "url", substrateWsRpcUrl, "err", err)
// 		return nil, err
// 	}
	
// 	return &SubstrateAPIWrapper{
// 		substrateApi:     substrateApi,
// 	}, nil
// }
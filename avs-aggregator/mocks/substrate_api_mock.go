package mocks

import (
	// "context"
	// "fmt"

	// "github.com/Layr-Labs/eigensdk-go/chainio/clients/avsregistry"
	// "github.com/Layr-Labs/eigensdk-go/chainio/clients"
	// sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	// "github.com/Layr-Labs/eigensdk-go/signerv2"

	// "github.com/ethereum/go-ethereum/common"
	// "github.com/ethereum/go-ethereum/ethclient"
	// "github.com/ethereum/go-ethereum/core/types"
	// sdktypes "github.com/Layr-Labs/eigensdk-go/types"
	// "github.com/ethereum/go-ethereum/accounts/abi/bind"
	
	// "github.com/gasp-xyz/gasp-monorepo/avs-aggregator/core/chainio"
	gsrpcrpcauthormocks "github.com/centrifuge/go-substrate-rpc-client/v4/rpc/author/mocks"
	gsrpcrpcbeefymocks "github.com/centrifuge/go-substrate-rpc-client/v4/rpc/beefy/mocks"
	gsrpcrpcchainmocks "github.com/centrifuge/go-substrate-rpc-client/v4/rpc/chain/mocks"
	gsrpcrpcmmrmocks "github.com/centrifuge/go-substrate-rpc-client/v4/rpc/mmr/mocks"
	gsrpcrpcoffchainmocks "github.com/centrifuge/go-substrate-rpc-client/v4/rpc/offchain/mocks"
	gsrpcrpcstatemocks "github.com/centrifuge/go-substrate-rpc-client/v4/rpc/state/mocks"
	gsrpcrpcsystemmocks "github.com/centrifuge/go-substrate-rpc-client/v4/rpc/system/mocks"
	gsrpcrpc "github.com/centrifuge/go-substrate-rpc-client/v4/rpc"
	gsrpcclientmocks "github.com/centrifuge/go-substrate-rpc-client/v4/client/mocks"
	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
)

// func NewEthRpcFromParts(
// 	avsReader     AvsReaderer,
// 	avsWriter     AvsWriterer,
// 	avsSubscriber AvsSubscriberer,
// 	clients       EthRpcClientsInterface,
// ) (*EthRpc, error) {
// 	return &EthRpc{
// 		AvsReader:     avsReader,
// 		AvsWriter:     avsWriter,
// 		AvsSubscriber: avsSubscriber,
// 		Clients:       clients,
// 	}, nil
// }

func NewMockSubstrateAPI() (*gsrpc.SubstrateAPI, error) {
	return &gsrpc.SubstrateAPI{
		RPC:     &gsrpcrpc.RPC{
			Author:   &gsrpcrpcauthormocks.Author{},
			Beefy:    &gsrpcrpcbeefymocks.Beefy{},
			Chain:    &gsrpcrpcchainmocks.Chain{},
			MMR:      &gsrpcrpcmmrmocks.MMR{},
			Offchain: &gsrpcrpcoffchainmocks.Offchain{},
			State:    &gsrpcrpcstatemocks.State{},
			System:   &gsrpcrpcsystemmocks.System{},
		},
		Client:  &gsrpcclientmocks.Client{},
	}, nil
}
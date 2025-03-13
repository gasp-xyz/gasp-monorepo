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
	
	"github.com/gasp-xyz/gasp-monorepo/avs-aggregator/core/chainio"
	gomock "go.uber.org/mock/gomock"
)

func NewMockEthRpc (ctrl *gomock.Controller) (*chainio.EthRpc, error) {
	return &chainio.EthRpc{
		AvsReader:     NewMockAvsReaderer(ctrl),
		AvsWriter:     NewMockAvsWriterer(ctrl),
		AvsSubscriber: NewMockAvsSubscriberer(ctrl),
		Clients:       NewMockEthRpcClientsInterface(ctrl),
	}, nil
}
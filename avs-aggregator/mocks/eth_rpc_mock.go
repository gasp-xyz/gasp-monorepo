package mocks

import (
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
package aggregator

import (

	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"

	sdktypes "github.com/Layr-Labs/eigensdk-go/types"
	
	mocks "github.com/gasp-xyz/gasp-monorepo/avs-aggregator/mocks"
	gomock "go.uber.org/mock/gomock"
)

type MockAggConfigExt struct {
	taskResponseWindowBlock uint32
}

// We have to put it here instead of with the mocks because Aggregator fields are private
// Kicker, Osu and RpcServer will have to be treated the same way
func NewMockAggregator (ctrl *gomock.Controller, c *Config, mce *MockAggConfigExt) (*Aggregator, error) {
	logger, err := sdklogging.NewZapLogger(c.LogLevel)
	if err != nil {
		return nil, err
	}

	ethRpc, err := mocks.NewMockEthRpc(ctrl)
	if err != nil {
		logger.Error("Failed to create ethRpc", "err", err)
		return nil, err
	}

	blsAggregationService := mocks.NewMockBlsAggregationService(ctrl)
	if err != nil {
		logger.Error("Cannot create blsAggregationService", "url", c.SubstrateWsRpcUrl, "err", err)
		return nil, err
	}

	substrateRpc, err := mocks.NewMockSubstrateAPI()
	if err != nil {
		logger.Error("Cannot create substrate RPC", "url", c.SubstrateWsRpcUrl, "err", err)
		return nil, err
	}

	return &Aggregator{
		logger:                  logger,
		serverIpPortAddr:        c.ServerAddressPort,
		ethRpc:                  ethRpc,
		blsAggregationService:   blsAggregationService,
		tasks:                   make(map[sdktypes.TaskId]interface{}),
		taskResponses:           make(map[sdktypes.TaskId]map[sdktypes.TaskResponseDigest]interface{}),
		substrateClient:         *substrateRpc,
		taskResponseWindowBlock: mce.taskResponseWindowBlock,
		blockPeriod:             uint32(c.BlockPeriod),
		blockPeriodOpsTask:      uint32(c.BlockPeriodOpsTask),
		kicker:                  nil,
		opStateUpdater:          nil,
		rpcServer:				 nil,
		expiration:              uint32(c.Expiration),
		startIdle:			   	 c.AggIdleStart,
		apiKey:			   	     c.AggRunTriggerApiKey,	
		enableTraceLogs:		 c.EnableTraceLogs,	
	}, nil
}
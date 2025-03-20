package aggregator

import (
	// "context"
	// "math/big"
	"testing"
	// "time"

	// "github.com/ethereum/go-ethereum/accounts/abi/bind/backends"
	// "github.com/ethereum/go-ethereum/common"
	// gethcore "github.com/ethereum/go-ethereum/core"
	"github.com/stretchr/testify/assert"
	"go.uber.org/mock/gomock"

	// "github.com/Layr-Labs/eigensdk-go/crypto/bls"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	// sdktypes "github.com/Layr-Labs/eigensdk-go/types"

	"github.com/gasp-xyz/gasp-monorepo/avs-aggregator/mocks"
	// "github.com/gasp-xyz/gasp-monorepo/avs-aggregator/types"
)


func getDefaultAggConfig() (Config, MockAggConfigExt) {
	return Config{
		LogLevel: sdklogging.Development,
		AggIdleStart: false,
		EnableTraceLogs: true,
	}, MockAggConfigExt{}
}

func TestCheckAndProcessPendingTasks_NoPendingTasks_ExitsEarly(t *testing.T) {
	mockCtrl := gomock.NewController(t)
	defer mockCtrl.Finish()

	config, mockAggConfigExt := getDefaultAggConfig()

	var err error
	agg, err := NewMockAggregator(mockCtrl, &config, &mockAggConfigExt)
	assert.Nil(t, err)

	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IsTaskPending(gomock.Any()).Return(false, nil)

	err = agg.checkAndProcessPendingTasks()
	assert.Nil(t, err)
}

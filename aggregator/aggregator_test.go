package aggregator

import (
	"context"
	"math/big"
	"testing"
	"time"

	"github.com/ethereum/go-ethereum/accounts/abi/bind/backends"
	"github.com/ethereum/go-ethereum/common"
	gethcore "github.com/ethereum/go-ethereum/core"
	"github.com/stretchr/testify/assert"
	"go.uber.org/mock/gomock"

	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	blsaggservmock "github.com/Layr-Labs/eigensdk-go/services/mocks/blsagg"
	sdktypes "github.com/Layr-Labs/eigensdk-go/types"

	taskmanager "github.com/mangata-finance/eigen-layer-monorepo/aggregator/bindings/MangataTaskManager"
	chainiomocks "github.com/mangata-finance/eigen-layer-monorepo/aggregator/core/chainio/mocks"
	"github.com/mangata-finance/eigen-layer-monorepo/aggregator/mocks"
	"github.com/mangata-finance/eigen-layer-monorepo/aggregator/types"
)

var MOCK_OPERATOR_ID = [32]byte{207, 73, 226, 221, 104, 100, 123, 41, 192, 3, 9, 119, 90, 83, 233, 159, 231, 151, 245, 96, 150, 48, 144, 27, 102, 253, 39, 101, 1, 26, 135, 173}
var MOCK_OPERATOR_STAKE = big.NewInt(100)
var MOCK_OPERATOR_BLS_PRIVATE_KEY_STRING = "50"

type MockTask struct {
	TaskNum            uint32
	CreatedBlockNumber uint32
	BlockNumber        uint32
}

func TestSendNewTask(t *testing.T) {
	mockCtrl := gomock.NewController(t)
	defer mockCtrl.Finish()

	MOCK_OPERATOR_BLS_PRIVATE_KEY, err := bls.NewPrivateKey(MOCK_OPERATOR_BLS_PRIVATE_KEY_STRING)
	assert.Nil(t, err)
	MOCK_OPERATOR_KEYPAIR := bls.NewKeyPair(MOCK_OPERATOR_BLS_PRIVATE_KEY)
	MOCK_OPERATOR_G1PUBKEY := MOCK_OPERATOR_KEYPAIR.GetPubKeyG1()
	MOCK_OPERATOR_G2PUBKEY := MOCK_OPERATOR_KEYPAIR.GetPubKeyG2()

	operatorPubkeyDict := map[bls.OperatorId]types.OperatorInfo{
		MOCK_OPERATOR_ID: {
			OperatorPubkeys: sdktypes.OperatorPubkeys{
				G1Pubkey: MOCK_OPERATOR_G1PUBKEY,
				G2Pubkey: MOCK_OPERATOR_G2PUBKEY,
			},
			OperatorAddr: common.Address{},
		},
	}

	aggregator, mockAvsWriterer, mockBlsAggService, err := createMockAggregator(mockCtrl, operatorPubkeyDict)
	assert.Nil(t, err)

	var TASK_INDEX = uint32(0)
	var CREATED_BLOCK_NUMBER = uint32(100)
	var BLOCK_NUMBER = uint32(30)
	var BLOCK_NUMBER_BN = big.NewInt(int64(BLOCK_NUMBER))

	mockAvsWriterer.EXPECT().SendNewTaskVerifyBlock(
		context.Background(), BLOCK_NUMBER_BN, types.QUORUM_THRESHOLD_NUMERATOR, types.QUORUM_NUMBERS,
	).Return(mocks.MockSendNewTaskVerifyBlock(CREATED_BLOCK_NUMBER, TASK_INDEX, BLOCK_NUMBER_BN))

	// 100 blocks, each takes 12 seconds. We hardcode for now since aggregator also hardcodes this value
	taskTimeToExpiry := 100 * 12 * time.Second
	// make sure that initializeNewTask was called on the blsAggService
	// maybe there's a better way to do this? There's a saying "don't mock 3rd party code"
	// see https://hynek.me/articles/what-to-mock-in-5-mins/
	mockBlsAggService.EXPECT().InitializeNewTask(TASK_INDEX, CREATED_BLOCK_NUMBER, types.QUORUM_NUMBERS, []uint32{types.QUORUM_THRESHOLD_NUMERATOR}, taskTimeToExpiry)

	err = aggregator.sendNewTask(BLOCK_NUMBER)
	assert.Nil(t, err)
}

func createMockAggregator(
	mockCtrl *gomock.Controller, operatorPubkeyDict map[bls.OperatorId]types.OperatorInfo,
) (*Aggregator, *chainiomocks.MockAvsWriterer, *blsaggservmock.MockBlsAggregationService, error) {
	logger := sdklogging.NewNoopLogger()
	mockAvsWriter := chainiomocks.NewMockAvsWriterer(mockCtrl)
	mockBlsAggregationService := blsaggservmock.NewMockBlsAggregationService(mockCtrl)

	aggregator := &Aggregator{
		logger:                  logger,
		avsWriter:               mockAvsWriter,
		blsAggregationService:   mockBlsAggregationService,
		tasks:                   make(map[types.TaskIndex]taskmanager.IMangataTaskManagerTask),
		taskResponses:           make(map[types.TaskIndex]map[sdktypes.TaskResponseDigest]taskmanager.IMangataTaskManagerTaskResponse),
		taskResponseWindowBlock: 100,
	}
	return aggregator, mockAvsWriter, mockBlsAggregationService, nil
}

// just a mock ethclient to pass to bindings
// so that we can access abi methods
func createMockEthClient() *backends.SimulatedBackend {
	genesisAlloc := map[common.Address]gethcore.GenesisAccount{}
	blockGasLimit := uint64(1000000)
	client := backends.NewSimulatedBackend(genesisAlloc, blockGasLimit)
	return client
}

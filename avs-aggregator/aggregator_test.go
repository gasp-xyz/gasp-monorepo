package aggregator

import (
	"context"
	"math/big"
	// "testing"
	"time"
	"reflect"

	"github.com/ethereum/go-ethereum/accounts/abi/bind/backends"
	"github.com/ethereum/go-ethereum/common"
	gethcore "github.com/ethereum/go-ethereum/core"
	// "github.com/stretchr/testify/assert"
	"go.uber.org/mock/gomock"

	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	sdktypes "github.com/Layr-Labs/eigensdk-go/types"

	blsagg "github.com/Layr-Labs/eigensdk-go/services/bls_aggregation"

	taskmanager "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/FinalizerTaskManager"
	chainiomocks "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/core/chainio/mocks"
	// "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/mocks"
	"github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/types"
)

var MOCK_OPERATOR_ID = [32]byte{207, 73, 226, 221, 104, 100, 123, 41, 192, 3, 9, 119, 90, 83, 233, 159, 231, 151, 245, 96, 150, 48, 144, 27, 102, 253, 39, 101, 1, 26, 135, 173}
var MOCK_OPERATOR_STAKE = big.NewInt(100)
var MOCK_OPERATOR_BLS_PRIVATE_KEY_STRING = "50"

// MockBlsAggregationService is a mock of BlsAggregationService interface.
type MockBlsAggregationService struct {
	ctrl     *gomock.Controller
	recorder *MockBlsAggregationServiceMockRecorder
}

// MockBlsAggregationServiceMockRecorder is the mock recorder for MockBlsAggregationService.
type MockBlsAggregationServiceMockRecorder struct {
	mock *MockBlsAggregationService
}

// NewMockBlsAggregationService creates a new mock instance.
func NewMockBlsAggregationService(ctrl *gomock.Controller) *MockBlsAggregationService {
	mock := &MockBlsAggregationService{ctrl: ctrl}
	mock.recorder = &MockBlsAggregationServiceMockRecorder{mock}
	return mock
}

// EXPECT returns an object that allows the caller to indicate expected use.
func (m *MockBlsAggregationService) EXPECT() *MockBlsAggregationServiceMockRecorder {
	return m.recorder
}

// GetResponseChannel mocks base method.
func (m *MockBlsAggregationService) GetResponseChannel() <-chan blsagg.BlsAggregationServiceResponse {
	m.ctrl.T.Helper()
	ret := m.ctrl.Call(m, "GetResponseChannel")
	ret0, _ := ret[0].(<-chan blsagg.BlsAggregationServiceResponse)
	return ret0
}

// GetResponseChannel indicates an expected call of GetResponseChannel.
func (mr *MockBlsAggregationServiceMockRecorder) GetResponseChannel() *gomock.Call {
	mr.mock.ctrl.T.Helper()
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "GetResponseChannel", reflect.TypeOf((*MockBlsAggregationService)(nil).GetResponseChannel))
}

// InitializeNewTask mocks base method.
func (m *MockBlsAggregationService) InitializeNewTask(arg0, arg1 uint32, arg2 sdktypes.QuorumNums, arg3 sdktypes.QuorumThresholdPercentages, arg4 time.Duration) error {
	m.ctrl.T.Helper()
	ret := m.ctrl.Call(m, "InitializeNewTask", arg0, arg1, arg2, arg3, arg4)
	ret0, _ := ret[0].(error)
	return ret0
}

// InitializeNewTask indicates an expected call of InitializeNewTask.
func (mr *MockBlsAggregationServiceMockRecorder) InitializeNewTask(arg0, arg1, arg2, arg3, arg4 any) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "InitializeNewTask", reflect.TypeOf((*MockBlsAggregationService)(nil).InitializeNewTask), arg0, arg1, arg2, arg3, arg4)
}

// ProcessNewSignature mocks base method.
func (m *MockBlsAggregationService) ProcessNewSignature(arg0 context.Context, arg1 uint32, arg2 interface{}, arg3 *bls.Signature, arg4 sdktypes.Bytes32) error {
	m.ctrl.T.Helper()
	ret := m.ctrl.Call(m, "ProcessNewSignature", arg0, arg1, arg2, arg3, arg4)
	ret0, _ := ret[0].(error)
	return ret0
}

// ProcessNewSignature indicates an expected call of ProcessNewSignature.
func (mr *MockBlsAggregationServiceMockRecorder) ProcessNewSignature(arg0, arg1, arg2, arg3, arg4 any) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "ProcessNewSignature", reflect.TypeOf((*MockBlsAggregationService)(nil).ProcessNewSignature), arg0, arg1, arg2, arg3, arg4)
}

// type MockTask struct {
// 	TaskNum            uint32
// 	CreatedBlockNumber uint32
// 	BlockNumber        uint32
// }

// func TestSendNewTask(t *testing.T) {
// 	mockCtrl := gomock.NewController(t)
// 	defer mockCtrl.Finish()

// 	MOCK_OPERATOR_BLS_PRIVATE_KEY, err := bls.NewPrivateKey(MOCK_OPERATOR_BLS_PRIVATE_KEY_STRING)
// 	assert.Nil(t, err)
// 	MOCK_OPERATOR_KEYPAIR := bls.NewKeyPair(MOCK_OPERATOR_BLS_PRIVATE_KEY)
// 	MOCK_OPERATOR_G1PUBKEY := MOCK_OPERATOR_KEYPAIR.GetPubKeyG1()
// 	MOCK_OPERATOR_G2PUBKEY := MOCK_OPERATOR_KEYPAIR.GetPubKeyG2()

// 	operatorPubkeyDict := map[bls.OperatorId]types.OperatorInfo{
// 		MOCK_OPERATOR_ID: {
// 			OperatorPubkeys: sdktypes.OperatorPubkeys{
// 				G1Pubkey: MOCK_OPERATOR_G1PUBKEY,
// 				G2Pubkey: MOCK_OPERATOR_G2PUBKEY,
// 			},
// 			OperatorAddr: common.Address{},
// 		},
// 	}

// 	aggregator, mockAvsWriterer, mockBlsAggService, err := createMockAggregator(mockCtrl, operatorPubkeyDict)
// 	assert.Nil(t, err)

// 	var TASK_INDEX = uint32(0)
// 	var CREATED_BLOCK_NUMBER = uint32(100)
// 	var BLOCK_NUMBER = uint32(30)
// 	var BLOCK_NUMBER_BN = big.NewInt(int64(BLOCK_NUMBER))

// 	mockAvsWriterer.EXPECT().SendNewTaskVerifyBlock(
// 		context.Background(), BLOCK_NUMBER_BN, types.QUORUM_THRESHOLD_NUMERATOR, types.QUORUM_NUMBERS,
// 	).Return(mocks.MockSendNewTaskVerifyBlock(CREATED_BLOCK_NUMBER, TASK_INDEX, BLOCK_NUMBER_BN))

// 	// 100 blocks, each takes 12 seconds. We hardcode for now since aggregator also hardcodes this value
// 	taskTimeToExpiry := 100 * 12 * time.Second
// 	// make sure that initializeNewTask was called on the blsAggService
// 	// maybe there's a better way to do this? There's a saying "don't mock 3rd party code"
// 	// see https://hynek.me/articles/what-to-mock-in-5-mins/
// 	mockBlsAggService.EXPECT().InitializeNewTask(TASK_INDEX, CREATED_BLOCK_NUMBER, types.QUORUM_NUMBERS, []uint32{types.QUORUM_THRESHOLD_NUMERATOR}, taskTimeToExpiry)

// 	err = aggregator.sendNewTask(BLOCK_NUMBER)
// 	assert.Nil(t, err)
// }

func createMockAggregator(
	mockCtrl *gomock.Controller, operatorPubkeyDict map[sdktypes.OperatorId]types.OperatorInfo,
) (*Aggregator, *chainiomocks.MockAvsWriterer, *MockBlsAggregationService, error) {
	logger := sdklogging.NewNoopLogger()
	mockAvsWriter := chainiomocks.NewMockAvsWriterer(mockCtrl)
	mockBlsAggregationService := NewMockBlsAggregationService(mockCtrl)

	aggregator := &Aggregator{
		logger:                  logger,
		avsWriter:               mockAvsWriter,
		blsAggregationService:   mockBlsAggregationService,
		tasks:                   make(map[types.TaskIndex]taskmanager.IMangataTaskManagerTask),
		taskResponses:           make(map[types.TaskIndex]map[sdktypes.TaskResponseDigest]taskmanager.IMangataTaskManagerTaskResponse),
		taskResponseWindowBlock: 100,
	}
	{
		logger:                  logger,
		serverIpPortAddr:        c.ServerAddressPort,
		ethRpc:                  ethRpc,
		blsAggregationService:   blsAggregationService,
		tasks:                   make(map[sdktypes.TaskId]interface{}),
		taskResponses:           make(map[sdktypes.TaskId]map[sdktypes.TaskResponseDigest]interface{}),
		substrateClient:         *substrateRpc,
		taskResponseWindowBlock: taskResponseWindowBlock,
		blockPeriod:             uint32(c.BlockPeriod),
		blockPeriodOpsTask:      uint32(c.BlockPeriodOpsTask),
		kicker:                  kicker,
		opStateUpdater:          opStateUpdater,
		expiration:              uint32(c.Expiration),
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

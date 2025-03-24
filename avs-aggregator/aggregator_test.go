package aggregator

import (
	"context"
	"errors"
	"math/big"
	"testing"
	// "time"
	"reflect"
	"encoding/hex"
	"github.com/saman-org/go-saman/common/bytesutil"

	ethtypes "github.com/ethereum/go-ethereum/core/types"
	// "github.com/ethereum/go-ethereum/accounts/abi/bind/backends"
	// "github.com/ethereum/go-ethereum/common"
	// gethcore "github.com/ethereum/go-ethereum/core"
	testifymock "github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/assert"
	"go.uber.org/mock/gomock"

	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	sdktypes "github.com/Layr-Labs/eigensdk-go/types"

	"github.com/gasp-xyz/gasp-monorepo/avs-aggregator/mocks"
	"github.com/gasp-xyz/gasp-monorepo/avs-aggregator/types"
	taskmanager "github.com/gasp-xyz/gasp-monorepo/avs-aggregator/bindings/FinalizerTaskManager"
	blsagg "github.com/Layr-Labs/eigensdk-go/services/bls_aggregation"

	// gsrpcrpcauthormocks "github.com/centrifuge/go-substrate-rpc-client/v4/rpc/author/mocks"
	// gsrpcrpcbeefymocks "github.com/centrifuge/go-substrate-rpc-client/v4/rpc/beefy/mocks"
	gsrpcrpcchainmocks "github.com/centrifuge/go-substrate-rpc-client/v4/rpc/chain/mocks"
	// gsrpcrpcmmrmocks "github.com/centrifuge/go-substrate-rpc-client/v4/rpc/mmr/mocks"
	// gsrpcrpcoffchainmocks "github.com/centrifuge/go-substrate-rpc-client/v4/rpc/offchain/mocks"
	gsrpcrpcstatemocks "github.com/centrifuge/go-substrate-rpc-client/v4/rpc/state/mocks"
	// gsrpcrpcsystemmocks "github.com/centrifuge/go-substrate-rpc-client/v4/rpc/system/mocks"
	// gsrpcrpc "github.com/centrifuge/go-substrate-rpc-client/v4/rpc"
	gsrpcclientmocks "github.com/centrifuge/go-substrate-rpc-client/v4/client/mocks"
	// gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
	gsrpctypes "github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

var DEFAULT_RETRIES = 3
var DEFAULT_RETRIES_SUB_ONE = DEFAULT_RETRIES - 1

var DUMMY_SUBSTRATE_BLOCK_NUMBER = uint32(10000)

var DUMMY_SUBSTRATE_L2_REQUESTS_BATCH_LAST = []types.SubstrateL2RequestsBatchLastItem{
	types.SubstrateL2RequestsBatchLastItem{Key: gsrpctypes.NewU8(1),
	Value: types.SubstrateL2RequestsBatchLastValue{
		BlockNumber: gsrpctypes.NewU32(8000),
		BatchId: gsrpctypes.NewU128(*big.NewInt(int64(500))),
		BatchRange: types.SubstrateL2RequestsBatchLastRange{
			Start: gsrpctypes.NewU128(*big.NewInt(int64(200))),
			End: gsrpctypes.NewU128(*big.NewInt(int64(400))),
		},
	}},
	types.SubstrateL2RequestsBatchLastItem{Key: gsrpctypes.NewU8(DUMMY_CHAIN_ID),
	Value: types.SubstrateL2RequestsBatchLastValue{
		BlockNumber: gsrpctypes.NewU32(DUMMY_SUBSTRATE_BLOCK_NUMBER),
		BatchId: gsrpctypes.NewU128(*big.NewInt(int64(DUMMY_BATCH_ID))),
		BatchRange: types.SubstrateL2RequestsBatchLastRange{
			Start: gsrpctypes.NewU128(*DUMMY_RANGE_START),
			End: gsrpctypes.NewU128(*DUMMY_RANGE_END),
		},
	}},
}

var DUMMY_LAST_OP_TASK_CREATED_BLOCK = uint32(42)
var DUMMY_LAST_RD_TASK_CREATED_BLOCK = uint32(42)
var DUMMY_LATEST_OP_TASK_NUM = uint32(52)
var DUMMY_LATEST_RD_TASK_NUM = uint32(54)

var DUMMY_LOG = ethtypes.Log{}

var DUMMY_RECEIPT = ethtypes.Receipt {
	Logs: []*ethtypes.Log{&DUMMY_LOG},
}

var DUMMY_OP_TASK_ID = sdktypes.TaskId{
	TaskType:  sdktypes.TaskType(0),
	TaskIndex: sdktypes.TaskIndex(DUMMY_LATEST_OP_TASK_NUM),
}                 

var DUMMY_TASK_CREATED_BLOCK = uint32(40)                             
var DUMMY_LAST_COMPLETED_OP_TASK_NUM = uint32(56)                       
var DUMMY_LAST_COMPLETED_OP_TASK_CREATED_BLOCK = uint32(41)              
var DUMMY_QUORUM_NUMBERS = []byte{0}          
var DUMMY_QUORUM_THRESHOLD_PERCENTAGE = uint32(70)
var DUMMY_LAST_COMPLETED_OP_TASK_QUORUM_NUMBERS = []byte{0}          
var DUMMY_LAST_COMPLETED_OP_TASK_QUORUM_THRESHOLD_PERCENTAGE = uint32(70)

var DUMMY_OP_TASK = taskmanager.IFinalizerTaskManagerOpTask {
	TaskNum:                                      DUMMY_LATEST_OP_TASK_NUM,
	TaskCreatedBlock:                             DUMMY_TASK_CREATED_BLOCK,
	LastCompletedOpTaskNum:                       DUMMY_LAST_COMPLETED_OP_TASK_NUM,
	LastCompletedOpTaskCreatedBlock:              DUMMY_LAST_COMPLETED_OP_TASK_CREATED_BLOCK,
	QuorumNumbers:             					  DUMMY_QUORUM_NUMBERS,
	QuorumThresholdPercentage: 					  DUMMY_QUORUM_THRESHOLD_PERCENTAGE,
	LastCompletedOpTaskQuorumNumbers:             DUMMY_LAST_COMPLETED_OP_TASK_QUORUM_NUMBERS,
	LastCompletedOpTaskQuorumThresholdPercentage: DUMMY_LAST_COMPLETED_OP_TASK_QUORUM_THRESHOLD_PERCENTAGE,
}

var DUMMY_RD_TASK_ID = sdktypes.TaskId{
	TaskType:  sdktypes.TaskType(1),
	TaskIndex: sdktypes.TaskIndex(DUMMY_LATEST_RD_TASK_NUM),
}
                              
var DUMMY_CHAIN_ID = uint8(9)                                      
var DUMMY_BATCH_ID = uint32(777)

var DUMMY_RD_TASK = taskmanager.IFinalizerTaskManagerRdTask {
	TaskNum:                                      DUMMY_LATEST_RD_TASK_NUM,
	ChainId:                                      DUMMY_CHAIN_ID,
	BatchId:                                      DUMMY_BATCH_ID,
	TaskCreatedBlock:                             DUMMY_TASK_CREATED_BLOCK,
	LastCompletedOpTaskNum:                       DUMMY_LAST_COMPLETED_OP_TASK_NUM,
	LastCompletedOpTaskCreatedBlock:              DUMMY_LAST_COMPLETED_OP_TASK_CREATED_BLOCK,
	LastCompletedOpTaskQuorumNumbers:             DUMMY_LAST_COMPLETED_OP_TASK_QUORUM_NUMBERS,
	LastCompletedOpTaskQuorumThresholdPercentage: DUMMY_LAST_COMPLETED_OP_TASK_QUORUM_THRESHOLD_PERCENTAGE,
}

var DUMMY_OP_TASK_RESPONSE = taskmanager.IFinalizerTaskManagerOpTaskResponse{
}

var DUMMY_MERKLE_ROOT_BAD = "0x3333333333333333333333333333333333333333333333333333333333333333"
var DUMMY_MERKLE_ROOT = "0x1234123412341234123412341234123412341234123412341234123412341234"
var DUMMY_RD_UPDATE_ARRAY, _ = hex.DecodeString(DUMMY_MERKLE_ROOT[2:])
var DUMMY_RD_UPDATE = bytesutil.ToBytes32(DUMMY_RD_UPDATE_ARRAY)

var DUMMY_RANGE_START = big.NewInt(int64(1000))
var DUMMY_RANGE_END = big.NewInt(int64(2000))
var DUMMY_RD_TASK_RESPONSE = taskmanager.IFinalizerTaskManagerRdTaskResponse{
	RdUpdate: DUMMY_RD_UPDATE,
	RangeStart: DUMMY_RANGE_START,
	RangeEnd: DUMMY_RANGE_END,
}

var DUMMY_SIG_G1 = bls.NewZeroG1Point()
var DUMMY_SIG_G2 = bls.NewZeroG2Point()
var DUMMY_SIGNATURE = bls.NewZeroSignature()

var DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_OP_TASK = blsagg.BlsAggregationServiceResponse {
	TaskId						 : DUMMY_OP_TASK_ID,
	SignersApkG2                 : DUMMY_SIG_G2,
	SignersAggSigG1              : DUMMY_SIGNATURE,
}
var DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_RD_TASK = blsagg.BlsAggregationServiceResponse {
	TaskId						 : DUMMY_RD_TASK_ID,
	SignersApkG2                 : DUMMY_SIG_G2,
	SignersAggSigG1              : DUMMY_SIGNATURE,
}

func getDefaultAggConfig() (Config, MockAggConfigExt) {
	return Config{
		LogLevel: sdklogging.Development,
		AggIdleStart: false,
		EnableTraceLogs: true,
		BlockPeriod: 10,
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

func TestCheckAndProcessPendingTasks_BothOpAndRdPendingTasksFound_Fails(t *testing.T) {
	mockCtrl := gomock.NewController(t)
	defer mockCtrl.Finish()

	config, mockAggConfigExt := getDefaultAggConfig()

	var err error
	agg, err := NewMockAggregator(mockCtrl, &config, &mockAggConfigExt)
	assert.Nil(t, err)

	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IsTaskPending(gomock.Any()).Return(true, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().LatestOpTaskNum(gomock.Any()).Return(DUMMY_LATEST_OP_TASK_NUM + 1, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().LatestRdTaskNum(gomock.Any()).Return(DUMMY_LATEST_RD_TASK_NUM + 1, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IdToTaskStatus(gomock.Any(), gomock.Eq(sdktypes.TaskType(0)), gomock.Any()).Return(types.TASK_STATUS_INITIALIZED, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IdToTaskStatus(gomock.Any(), gomock.Eq(sdktypes.TaskType(1)), gomock.Any()).Return(types.TASK_STATUS_INITIALIZED, nil)
	
	err = agg.checkAndProcessPendingTasks()
	assert.NotNil(t, err)
}

func TestCheckAndProcessPendingTasks_NoOpAndRdPendingTasksFound_Fails(t *testing.T) {
	mockCtrl := gomock.NewController(t)
	defer mockCtrl.Finish()

	config, mockAggConfigExt := getDefaultAggConfig()

	var err error
	agg, err := NewMockAggregator(mockCtrl, &config, &mockAggConfigExt)
	assert.Nil(t, err)

	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IsTaskPending(gomock.Any()).Return(true, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().LatestOpTaskNum(gomock.Any()).Return(DUMMY_LATEST_OP_TASK_NUM + 1, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().LatestRdTaskNum(gomock.Any()).Return(DUMMY_LATEST_RD_TASK_NUM + 1, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IdToTaskStatus(gomock.Any(), gomock.Eq(sdktypes.TaskType(0)), gomock.Any()).Return(types.TASK_STATUS_NOT_INITIALIZED, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IdToTaskStatus(gomock.Any(), gomock.Eq(sdktypes.TaskType(1)), gomock.Any()).Return(types.TASK_STATUS_NOT_INITIALIZED, nil)

	err = agg.checkAndProcessPendingTasks()
	assert.NotNil(t, err)
}

func TestCheckAndProcessPendingTasks_RdPendingTasks_WorksFully(t *testing.T) {
	mockCtrl := gomock.NewController(t)
	defer mockCtrl.Finish()

	var DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL = make(chan blsagg.BlsAggregationServiceResponse, 1)

	config, mockAggConfigExt := getDefaultAggConfig()

	var err error
	agg, err := NewMockAggregator(mockCtrl, &config, &mockAggConfigExt)
	assert.Nil(t, err)

	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IsTaskPending(gomock.Any()).Return(true, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().LatestOpTaskNum(gomock.Any()).Return(DUMMY_LATEST_OP_TASK_NUM + 1, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().LatestRdTaskNum(gomock.Any()).Return(DUMMY_LATEST_RD_TASK_NUM + 1, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IdToTaskStatus(gomock.Any(), gomock.Eq(sdktypes.TaskType(0)), gomock.Any()).Return(types.TASK_STATUS_NOT_INITIALIZED, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IdToTaskStatus(gomock.Any(), gomock.Eq(sdktypes.TaskType(1)), gomock.Any()).Return(types.TASK_STATUS_INITIALIZED, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().LastRdTaskCreatedBlock(gomock.Any()).Return(DUMMY_LAST_RD_TASK_CREATED_BLOCK, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().GetFirstFilterNewRdTaskCreated(gomock.Any(), gomock.Any()).Return(DUMMY_RD_TASK, nil)
	agg.blsAggregationService.(*mocks.MockBlsAggregationService).EXPECT().InitializeNewTask(gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any()).Return(nil)
	DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL <- DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_RD_TASK
	agg.taskResponses[DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_RD_TASK.TaskId] = make(map[sdktypes.TaskResponseDigest]interface{})
	agg.taskResponses[DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_RD_TASK.TaskId][DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_RD_TASK.TaskResponseDigest] = DUMMY_RD_TASK_RESPONSE
	agg.blsAggregationService.(*mocks.MockBlsAggregationService).EXPECT().GetResponseChannel().Return(DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL)
	agg.ethRpc.AvsWriter.(*mocks.MockAvsWriterer).EXPECT().SendAggregatedRdTaskResponse(gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any()).Return(&DUMMY_RECEIPT, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().ParseRdTaskCompleted(gomock.Any()).Return(nil, nil)
	
	metadata, err := mocks.GetFakeSubstrateMetadata()
	assert.EqualValues(t, metadata.Version, 14)
	assert.NoError(t, err)

	agg.substrateClient.RPC.State.(*gsrpcrpcstatemocks.State).On("GetMetadataLatest", testifymock.Anything).Return(&metadata, nil)
	agg.substrateClient.RPC.State.(*gsrpcrpcstatemocks.State).On("GetStorageRawLatest", testifymock.Anything).Return(&gsrpctypes.StorageDataRaw{}, nil)
	agg.substrateClient.RPC.State.(*gsrpcrpcstatemocks.State).On("GetStorageLatest", testifymock.Anything, testifymock.Anything).Return(true, nil)
	agg.substrateClient.Client.(*gsrpcclientmocks.Client).On("Call", testifymock.Anything, testifymock.Anything, testifymock.Anything, testifymock.Anything).Return(nil).Run(func(args testifymock.Arguments) {
		val := reflect.ValueOf(args.Get(0))
		elem := val.Elem()
		elem.SetString(DUMMY_MERKLE_ROOT)
	})
	
	err = agg.checkAndProcessPendingTasks()
	assert.Nil(t, err)
}

func TestCheckAndProcessPendingTasks_RdPendingTasks_NoSuccess_Fails(t *testing.T) {
	mockCtrl := gomock.NewController(t)
	defer mockCtrl.Finish()

	var DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL_LARGE_BUFFER = make(chan blsagg.BlsAggregationServiceResponse, 10)

	config, mockAggConfigExt := getDefaultAggConfig()

	var err error
	agg, err := NewMockAggregator(mockCtrl, &config, &mockAggConfigExt)
	assert.Nil(t, err)

	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IsTaskPending(gomock.Any()).Return(true, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().LatestOpTaskNum(gomock.Any()).Return(DUMMY_LATEST_OP_TASK_NUM + 1, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().LatestRdTaskNum(gomock.Any()).Return(DUMMY_LATEST_RD_TASK_NUM + 1, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IdToTaskStatus(gomock.Any(), gomock.Eq(sdktypes.TaskType(0)), gomock.Any()).Return(types.TASK_STATUS_NOT_INITIALIZED, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IdToTaskStatus(gomock.Any(), gomock.Eq(sdktypes.TaskType(1)), gomock.Any()).Return(types.TASK_STATUS_INITIALIZED, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().LastRdTaskCreatedBlock(gomock.Any()).Return(DUMMY_LAST_RD_TASK_CREATED_BLOCK, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().GetFirstFilterNewRdTaskCreated(gomock.Any(), gomock.Any()).Return(DUMMY_RD_TASK, nil)
	agg.blsAggregationService.(*mocks.MockBlsAggregationService).EXPECT().InitializeNewTask(gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any()).Return(nil).Times(DEFAULT_RETRIES)
	for i := 0; i < DEFAULT_RETRIES; i++{ 
		DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL_LARGE_BUFFER <- DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_RD_TASK 
	} 
	agg.taskResponses[DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_RD_TASK.TaskId] = make(map[sdktypes.TaskResponseDigest]interface{})
	agg.taskResponses[DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_RD_TASK.TaskId][DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_RD_TASK.TaskResponseDigest] = DUMMY_RD_TASK_RESPONSE
	agg.blsAggregationService.(*mocks.MockBlsAggregationService).EXPECT().GetResponseChannel().Return(DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL_LARGE_BUFFER).Times(DEFAULT_RETRIES)
	agg.ethRpc.AvsWriter.(*mocks.MockAvsWriterer).EXPECT().SendAggregatedRdTaskResponse(gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any()).Return(&DUMMY_RECEIPT, nil).Times(DEFAULT_RETRIES)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().ParseRdTaskCompleted(gomock.Any()).Return(nil, errors.New("Failed to ParseRdTaskCompleted")).Times(DEFAULT_RETRIES)
	
	metadata, err := mocks.GetFakeSubstrateMetadata()
	assert.EqualValues(t, metadata.Version, 14)
	assert.NoError(t, err)

	agg.substrateClient.RPC.State.(*gsrpcrpcstatemocks.State).On("GetMetadataLatest", testifymock.Anything).Return(&metadata, nil).Times(DEFAULT_RETRIES)
	agg.substrateClient.RPC.State.(*gsrpcrpcstatemocks.State).On("GetStorageRawLatest", testifymock.Anything).Return(&gsrpctypes.StorageDataRaw{}, nil).Times(DEFAULT_RETRIES)
	agg.substrateClient.RPC.State.(*gsrpcrpcstatemocks.State).On("GetStorageLatest", testifymock.Anything, testifymock.Anything).Return(true, nil).Times(DEFAULT_RETRIES)
	agg.substrateClient.Client.(*gsrpcclientmocks.Client).On("Call", testifymock.Anything, testifymock.Anything, testifymock.Anything, testifymock.Anything).Return(nil).Run(func(args testifymock.Arguments) {
		val := reflect.ValueOf(args.Get(0))
		elem := val.Elem()
		elem.SetString(DUMMY_MERKLE_ROOT)
	}).Times(DEFAULT_RETRIES)
	
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IsTaskPending(gomock.Any()).Return(false, nil).Times(DEFAULT_RETRIES_SUB_ONE)
	agg.ethRpc.AvsWriter.(*mocks.MockAvsWriterer).EXPECT().SendNewRdTask(gomock.Any(), gomock.Any(), gomock.Any()).Return(DUMMY_RD_TASK, DUMMY_RD_TASK.TaskNum, nil).Times(DEFAULT_RETRIES_SUB_ONE)

	err = agg.checkAndProcessPendingTasks()
	assert.NotNil(t, err)
}

func TestCheckAndProcessPendingTasks_OpPendingTasks_WorksFully(t *testing.T) {
	mockCtrl := gomock.NewController(t)
	defer mockCtrl.Finish()

	var DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL = make(chan blsagg.BlsAggregationServiceResponse, 1)
	
	config, mockAggConfigExt := getDefaultAggConfig()

	var err error
	agg, err := NewMockAggregator(mockCtrl, &config, &mockAggConfigExt)
	assert.Nil(t, err)

	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IsTaskPending(gomock.Any()).Return(true, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().LatestOpTaskNum(gomock.Any()).Return(DUMMY_LATEST_OP_TASK_NUM + 1, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().LatestRdTaskNum(gomock.Any()).Return(DUMMY_LATEST_RD_TASK_NUM + 1, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IdToTaskStatus(gomock.Any(), gomock.Eq(sdktypes.TaskType(0)), gomock.Any()).Return(types.TASK_STATUS_INITIALIZED, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IdToTaskStatus(gomock.Any(), gomock.Eq(sdktypes.TaskType(1)), gomock.Any()).Return(types.TASK_STATUS_NOT_INITIALIZED, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().LastOpTaskCreatedBlock(gomock.Any()).Return(DUMMY_LAST_OP_TASK_CREATED_BLOCK, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().GetFirstFilterNewOpTaskCreated(gomock.Any(), gomock.Any()).Return(DUMMY_OP_TASK, nil)
	agg.blsAggregationService.(*mocks.MockBlsAggregationService).EXPECT().InitializeNewTask(gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any()).Return(nil)
	DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL <- DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_OP_TASK
	agg.taskResponses[DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_OP_TASK.TaskId] = make(map[sdktypes.TaskResponseDigest]interface{})
	agg.taskResponses[DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_OP_TASK.TaskId][DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_OP_TASK.TaskResponseDigest] = DUMMY_OP_TASK_RESPONSE
	agg.blsAggregationService.(*mocks.MockBlsAggregationService).EXPECT().GetResponseChannel().Return(DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL)
	agg.ethRpc.AvsWriter.(*mocks.MockAvsWriterer).EXPECT().SendAggregatedOpTaskResponse(gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any()).Return(&DUMMY_RECEIPT, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().ParseOpTaskCompleted(gomock.Any()).Return(nil, nil)
		
	err = agg.checkAndProcessPendingTasks()
	assert.Nil(t, err)
}

func TestCheckAndProcessPendingTasks_OpPendingTasks_NoSuccess_Fails(t *testing.T) {
	mockCtrl := gomock.NewController(t)
	defer mockCtrl.Finish()

	var DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL_LARGE_BUFFER = make(chan blsagg.BlsAggregationServiceResponse, 10)
	
	config, mockAggConfigExt := getDefaultAggConfig()

	var err error
	agg, err := NewMockAggregator(mockCtrl, &config, &mockAggConfigExt)
	assert.Nil(t, err)

	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IsTaskPending(gomock.Any()).Return(true, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().LatestOpTaskNum(gomock.Any()).Return(DUMMY_LATEST_OP_TASK_NUM + 1, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().LatestRdTaskNum(gomock.Any()).Return(DUMMY_LATEST_RD_TASK_NUM + 1, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IdToTaskStatus(gomock.Any(), gomock.Eq(sdktypes.TaskType(0)), gomock.Any()).Return(types.TASK_STATUS_INITIALIZED, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IdToTaskStatus(gomock.Any(), gomock.Eq(sdktypes.TaskType(1)), gomock.Any()).Return(types.TASK_STATUS_NOT_INITIALIZED, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().LastOpTaskCreatedBlock(gomock.Any()).Return(DUMMY_LAST_OP_TASK_CREATED_BLOCK, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().GetFirstFilterNewOpTaskCreated(gomock.Any(), gomock.Any()).Return(DUMMY_OP_TASK, nil)
	agg.blsAggregationService.(*mocks.MockBlsAggregationService).EXPECT().InitializeNewTask(gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any()).Return(nil).Times(DEFAULT_RETRIES)
	for i := 0; i < DEFAULT_RETRIES; i++{ 
		DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL_LARGE_BUFFER <- DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_OP_TASK 
	} 
	agg.taskResponses[DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_OP_TASK.TaskId] = make(map[sdktypes.TaskResponseDigest]interface{})
	agg.taskResponses[DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_OP_TASK.TaskId][DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_OP_TASK.TaskResponseDigest] = DUMMY_OP_TASK_RESPONSE
	agg.blsAggregationService.(*mocks.MockBlsAggregationService).EXPECT().GetResponseChannel().Return(DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL_LARGE_BUFFER).Times(DEFAULT_RETRIES)
	agg.ethRpc.AvsWriter.(*mocks.MockAvsWriterer).EXPECT().SendAggregatedOpTaskResponse(gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any()).Return(&DUMMY_RECEIPT, nil).Times(DEFAULT_RETRIES)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().ParseOpTaskCompleted(gomock.Any()).Return(nil, errors.New("Failed to ParseOpTaskCompleted")).Times(DEFAULT_RETRIES)
	
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IsTaskPending(gomock.Any()).Return(false, nil).Times(DEFAULT_RETRIES_SUB_ONE)
	agg.ethRpc.AvsWriter.(*mocks.MockAvsWriterer).EXPECT().SendNewOpTask(gomock.Any(), gomock.Any(), gomock.Any()).Return(DUMMY_OP_TASK, DUMMY_OP_TASK.TaskNum, nil).Times(DEFAULT_RETRIES_SUB_ONE)

	err = agg.checkAndProcessPendingTasks()
	assert.NotNil(t, err)
}

func TestSendAggregatedResponseToContract_BadRootRdTask_Fails(t *testing.T) {
	mockCtrl := gomock.NewController(t)
	defer mockCtrl.Finish()

	var DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL = make(chan blsagg.BlsAggregationServiceResponse, 1)
	
	config, mockAggConfigExt := getDefaultAggConfig()

	var err error
	agg, err := NewMockAggregator(mockCtrl, &config, &mockAggConfigExt)
	assert.Nil(t, err)

	DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL <- DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_RD_TASK
	agg.taskResponses[DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_RD_TASK.TaskId] = make(map[sdktypes.TaskResponseDigest]interface{})
	agg.taskResponses[DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_RD_TASK.TaskId][DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_RD_TASK.TaskResponseDigest] = DUMMY_RD_TASK_RESPONSE
	
	metadata, err := mocks.GetFakeSubstrateMetadata()
	assert.EqualValues(t, metadata.Version, 14)
	assert.NoError(t, err)

	agg.substrateClient.RPC.State.(*gsrpcrpcstatemocks.State).On("GetMetadataLatest", testifymock.Anything).Return(&metadata, nil)
	agg.substrateClient.RPC.State.(*gsrpcrpcstatemocks.State).On("GetStorageRawLatest", testifymock.Anything).Return(&gsrpctypes.StorageDataRaw{}, nil)
	agg.substrateClient.RPC.State.(*gsrpcrpcstatemocks.State).On("GetStorageLatest", testifymock.Anything, testifymock.Anything).Return(true, nil)
	agg.substrateClient.Client.(*gsrpcclientmocks.Client).On("Call", testifymock.Anything, testifymock.Anything, testifymock.Anything, testifymock.Anything).Return(nil).Run(func(args testifymock.Arguments) {
		val := reflect.ValueOf(args.Get(0))
		elem := val.Elem()
		elem.SetString(DUMMY_MERKLE_ROOT_BAD)
	})
	
	success, err := agg.sendAggregatedResponseToContract(DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_RD_TASK, DUMMY_RD_TASK_ID)
	assert.NotNil(t, err)
	assert.Equal(t, success, false)
}

func TestMaybeSendNewRdTask_WorksFully(t *testing.T) {
	mockCtrl := gomock.NewController(t)
	defer mockCtrl.Finish()

	var DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL = make(chan blsagg.BlsAggregationServiceResponse, 1)
	
	var err error
	metadata, err := mocks.GetFakeSubstrateMetadata()
	assert.EqualValues(t, metadata.Version, 14)
	assert.NoError(t, err)

	config, mockAggConfigExt := getDefaultAggConfig()

	agg, err := NewMockAggregator(mockCtrl, &config, &mockAggConfigExt)
	assert.Nil(t, err)

	agg.substrateClient.RPC.Chain.(*gsrpcrpcchainmocks.Chain).On("GetHeaderLatest").Return(&gsrpctypes.Header{}, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().LastCompletedOpTaskCreatedBlock(gomock.Any()).Return(uint32(DUMMY_LAST_COMPLETED_OP_TASK_CREATED_BLOCK), nil)
	agg.substrateClient.RPC.Chain.(*gsrpcrpcchainmocks.Chain).On("GetBlockHash", testifymock.Anything).Return(gsrpctypes.Hash{}, nil)
	agg.substrateClient.RPC.State.(*gsrpcrpcstatemocks.State).On("GetMetadata", testifymock.Anything).Return(&metadata, nil)
	agg.substrateClient.RPC.State.(*gsrpcrpcstatemocks.State).On("GetStorageRaw", testifymock.Anything, testifymock.Anything).Return(&gsrpctypes.StorageDataRaw{}, nil)
	agg.substrateClient.RPC.State.(*gsrpcrpcstatemocks.State).On("GetStorage", testifymock.Anything, testifymock.Anything, testifymock.Anything).Return(true, nil).Run(func(args testifymock.Arguments) {
		val := reflect.ValueOf(args.Get(1))
		elem := val.Elem()
		elem.Set(reflect.ValueOf(DUMMY_SUBSTRATE_L2_REQUESTS_BATCH_LAST))
	})
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().ChainRdBatchNonce(gomock.Any(), gomock.Any()).DoAndReturn(func(ctx context.Context, chainIndex uint8) (uint32, error) {
        if chainIndex == 1 {
            return 400, nil
        } else if chainIndex == 2 {
            return 600, nil
        }
        return 0, errors.New("unexpected input")
    })

	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IsTaskPending(gomock.Any()).Return(false, nil)
	agg.ethRpc.AvsWriter.(*mocks.MockAvsWriterer).EXPECT().SendNewRdTask(gomock.Any(), gomock.Any(), gomock.Any()).Return(DUMMY_RD_TASK, DUMMY_RD_TASK.TaskNum, nil)
	agg.blsAggregationService.(*mocks.MockBlsAggregationService).EXPECT().InitializeNewTask(gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any()).Return(nil)
	DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL <- DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_RD_TASK
	agg.taskResponses[DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_RD_TASK.TaskId] = make(map[sdktypes.TaskResponseDigest]interface{})
	agg.taskResponses[DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_RD_TASK.TaskId][DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_RD_TASK.TaskResponseDigest] = DUMMY_RD_TASK_RESPONSE
	agg.blsAggregationService.(*mocks.MockBlsAggregationService).EXPECT().GetResponseChannel().Return(DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL)
	agg.ethRpc.AvsWriter.(*mocks.MockAvsWriterer).EXPECT().SendAggregatedRdTaskResponse(gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any()).Return(&DUMMY_RECEIPT, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().ParseRdTaskCompleted(gomock.Any()).Return(nil, nil)
	

	agg.substrateClient.RPC.State.(*gsrpcrpcstatemocks.State).On("GetMetadataLatest", testifymock.Anything).Return(&metadata, nil)
	agg.substrateClient.RPC.State.(*gsrpcrpcstatemocks.State).On("GetStorageRawLatest", testifymock.Anything).Return(&gsrpctypes.StorageDataRaw{}, nil)
	agg.substrateClient.RPC.State.(*gsrpcrpcstatemocks.State).On("GetStorageLatest", testifymock.Anything, testifymock.Anything).Return(true, nil)
	agg.substrateClient.Client.(*gsrpcclientmocks.Client).On("Call", testifymock.Anything, testifymock.Anything, testifymock.Anything, testifymock.Anything).Return(nil).Run(func(args testifymock.Arguments) {
		val := reflect.ValueOf(args.Get(0))
		elem := val.Elem()
		elem.SetString(DUMMY_MERKLE_ROOT)
	})
	
	err = agg.maybeSendNewRdTask(uint32(0))
	assert.Nil(t, err)
}

func TestMaybeSendNewRdTask_NoSuccess_Fails(t *testing.T) {
	mockCtrl := gomock.NewController(t)
	defer mockCtrl.Finish()

	var DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL_LARGE_BUFFER = make(chan blsagg.BlsAggregationServiceResponse, 10)
	
	var err error
	metadata, err := mocks.GetFakeSubstrateMetadata()
	assert.EqualValues(t, metadata.Version, 14)
	assert.NoError(t, err)

	config, mockAggConfigExt := getDefaultAggConfig()

	agg, err := NewMockAggregator(mockCtrl, &config, &mockAggConfigExt)
	assert.Nil(t, err)

	agg.substrateClient.RPC.Chain.(*gsrpcrpcchainmocks.Chain).On("GetHeaderLatest").Return(&gsrpctypes.Header{}, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().LastCompletedOpTaskCreatedBlock(gomock.Any()).Return(uint32(DUMMY_LAST_COMPLETED_OP_TASK_CREATED_BLOCK), nil)
	agg.substrateClient.RPC.Chain.(*gsrpcrpcchainmocks.Chain).On("GetBlockHash", testifymock.Anything).Return(gsrpctypes.Hash{}, nil)
	agg.substrateClient.RPC.State.(*gsrpcrpcstatemocks.State).On("GetMetadata", testifymock.Anything).Return(&metadata, nil)
	agg.substrateClient.RPC.State.(*gsrpcrpcstatemocks.State).On("GetStorageRaw", testifymock.Anything, testifymock.Anything).Return(&gsrpctypes.StorageDataRaw{}, nil)
	agg.substrateClient.RPC.State.(*gsrpcrpcstatemocks.State).On("GetStorage", testifymock.Anything, testifymock.Anything, testifymock.Anything).Return(true, nil).Run(func(args testifymock.Arguments) {
		val := reflect.ValueOf(args.Get(1))
		elem := val.Elem()
		elem.Set(reflect.ValueOf(DUMMY_SUBSTRATE_L2_REQUESTS_BATCH_LAST))
	})
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().ChainRdBatchNonce(gomock.Any(), gomock.Any()).DoAndReturn(func(ctx context.Context, chainIndex uint8) (uint32, error) {
        if chainIndex == 1 {
            return 400, nil
        } else if chainIndex == 2 {
            return 600, nil
        }
        return 0, errors.New("unexpected input")
    })

	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IsTaskPending(gomock.Any()).Return(false, nil).Times(DEFAULT_RETRIES)
	agg.ethRpc.AvsWriter.(*mocks.MockAvsWriterer).EXPECT().SendNewRdTask(gomock.Any(), gomock.Any(), gomock.Any()).Return(DUMMY_RD_TASK, DUMMY_RD_TASK.TaskNum, nil).Times(DEFAULT_RETRIES)
	agg.blsAggregationService.(*mocks.MockBlsAggregationService).EXPECT().InitializeNewTask(gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any()).Return(nil).Times(DEFAULT_RETRIES)
	for i := 0; i < DEFAULT_RETRIES; i++{ 
		DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL_LARGE_BUFFER <- DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_RD_TASK 
	} 
	agg.taskResponses[DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_RD_TASK.TaskId] = make(map[sdktypes.TaskResponseDigest]interface{})
	agg.taskResponses[DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_RD_TASK.TaskId][DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_RD_TASK.TaskResponseDigest] = DUMMY_RD_TASK_RESPONSE
	agg.blsAggregationService.(*mocks.MockBlsAggregationService).EXPECT().GetResponseChannel().Return(DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL_LARGE_BUFFER).Times(DEFAULT_RETRIES)
	agg.ethRpc.AvsWriter.(*mocks.MockAvsWriterer).EXPECT().SendAggregatedRdTaskResponse(gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any()).Return(&DUMMY_RECEIPT, nil).Times(DEFAULT_RETRIES)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().ParseRdTaskCompleted(gomock.Any()).Return(nil, errors.New("Failed to ParseRdTaskCompleted")).Times(DEFAULT_RETRIES)
	

	agg.substrateClient.RPC.State.(*gsrpcrpcstatemocks.State).On("GetMetadataLatest", testifymock.Anything).Return(&metadata, nil).Times(DEFAULT_RETRIES)
	agg.substrateClient.RPC.State.(*gsrpcrpcstatemocks.State).On("GetStorageRawLatest", testifymock.Anything).Return(&gsrpctypes.StorageDataRaw{}, nil).Times(DEFAULT_RETRIES)
	agg.substrateClient.RPC.State.(*gsrpcrpcstatemocks.State).On("GetStorageLatest", testifymock.Anything, testifymock.Anything).Return(true, nil).Times(DEFAULT_RETRIES)
	agg.substrateClient.Client.(*gsrpcclientmocks.Client).On("Call", testifymock.Anything, testifymock.Anything, testifymock.Anything, testifymock.Anything).Return(nil).Run(func(args testifymock.Arguments) {
		val := reflect.ValueOf(args.Get(0))
		elem := val.Elem()
		elem.SetString(DUMMY_MERKLE_ROOT)
	}).Times(DEFAULT_RETRIES)
	
	err = agg.maybeSendNewRdTask(uint32(0))
	assert.NotNil(t, err)
}

func TestMaybeSendNewRdTask_IsTaskPending_Fails(t *testing.T) {
	mockCtrl := gomock.NewController(t)
	defer mockCtrl.Finish()

	var err error
	metadata, err := mocks.GetFakeSubstrateMetadata()
	assert.EqualValues(t, metadata.Version, 14)
	assert.NoError(t, err)

	config, mockAggConfigExt := getDefaultAggConfig()

	agg, err := NewMockAggregator(mockCtrl, &config, &mockAggConfigExt)
	assert.Nil(t, err)

	agg.substrateClient.RPC.Chain.(*gsrpcrpcchainmocks.Chain).On("GetHeaderLatest").Return(&gsrpctypes.Header{}, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().LastCompletedOpTaskCreatedBlock(gomock.Any()).Return(uint32(DUMMY_LAST_COMPLETED_OP_TASK_CREATED_BLOCK), nil)
	agg.substrateClient.RPC.Chain.(*gsrpcrpcchainmocks.Chain).On("GetBlockHash", testifymock.Anything).Return(gsrpctypes.Hash{}, nil)
	agg.substrateClient.RPC.State.(*gsrpcrpcstatemocks.State).On("GetMetadata", testifymock.Anything).Return(&metadata, nil)
	agg.substrateClient.RPC.State.(*gsrpcrpcstatemocks.State).On("GetStorageRaw", testifymock.Anything, testifymock.Anything).Return(&gsrpctypes.StorageDataRaw{}, nil)
	agg.substrateClient.RPC.State.(*gsrpcrpcstatemocks.State).On("GetStorage", testifymock.Anything, testifymock.Anything, testifymock.Anything).Return(true, nil).Run(func(args testifymock.Arguments) {
		val := reflect.ValueOf(args.Get(1))
		elem := val.Elem()
		elem.Set(reflect.ValueOf(DUMMY_SUBSTRATE_L2_REQUESTS_BATCH_LAST))
	})
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().ChainRdBatchNonce(gomock.Any(), gomock.Any()).DoAndReturn(func(ctx context.Context, chainIndex uint8) (uint32, error) {
        if chainIndex == 1 {
            return 400, nil
        } else if chainIndex == 2 {
            return 600, nil
        }
        return 0, errors.New("unexpected input")
    })

	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IsTaskPending(gomock.Any()).Return(true, nil)

	err = agg.maybeSendNewRdTask(uint32(0))
	assert.NotNil(t, err)
}

func TestStartNewOpTask_WorksFully(t *testing.T) {
	mockCtrl := gomock.NewController(t)
	defer mockCtrl.Finish()

	var DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL = make(chan blsagg.BlsAggregationServiceResponse, 1)
	
	config, mockAggConfigExt := getDefaultAggConfig()

	agg, err := NewMockAggregator(mockCtrl, &config, &mockAggConfigExt)
	assert.Nil(t, err)

	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IsTaskPending(gomock.Any()).Return(false, nil)
	agg.ethRpc.AvsWriter.(*mocks.MockAvsWriterer).EXPECT().SendNewOpTask(gomock.Any(), gomock.Any(), gomock.Any()).Return(DUMMY_OP_TASK, DUMMY_OP_TASK.TaskNum, nil)
	agg.blsAggregationService.(*mocks.MockBlsAggregationService).EXPECT().InitializeNewTask(gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any()).Return(nil)
	DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL <- DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_OP_TASK
	agg.taskResponses[DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_OP_TASK.TaskId] = make(map[sdktypes.TaskResponseDigest]interface{})
	agg.taskResponses[DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_OP_TASK.TaskId][DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_OP_TASK.TaskResponseDigest] = DUMMY_OP_TASK_RESPONSE
	agg.blsAggregationService.(*mocks.MockBlsAggregationService).EXPECT().GetResponseChannel().Return(DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL)
	agg.ethRpc.AvsWriter.(*mocks.MockAvsWriterer).EXPECT().SendAggregatedOpTaskResponse(gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any()).Return(&DUMMY_RECEIPT, nil)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().ParseOpTaskCompleted(gomock.Any()).Return(nil, nil)
	
	task, err := agg.startNewOpTask()
	assert.Nil(t, err)
	assert.NotNil(t, task)
}

func TestStartNewOpTask_NoSuccess_Fails(t *testing.T) {
	mockCtrl := gomock.NewController(t)
	defer mockCtrl.Finish()

	var DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL_LARGE_BUFFER = make(chan blsagg.BlsAggregationServiceResponse, 10)
	
	config, mockAggConfigExt := getDefaultAggConfig()

	agg, err := NewMockAggregator(mockCtrl, &config, &mockAggConfigExt)
	assert.Nil(t, err)

	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IsTaskPending(gomock.Any()).Return(false, nil).Times(DEFAULT_RETRIES)
	agg.ethRpc.AvsWriter.(*mocks.MockAvsWriterer).EXPECT().SendNewOpTask(gomock.Any(), gomock.Any(), gomock.Any()).Return(DUMMY_OP_TASK, DUMMY_OP_TASK.TaskNum, nil).Times(DEFAULT_RETRIES)
	agg.blsAggregationService.(*mocks.MockBlsAggregationService).EXPECT().InitializeNewTask(gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any()).Return(nil).Times(DEFAULT_RETRIES)
	for i := 0; i < DEFAULT_RETRIES; i++{ 
		DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL_LARGE_BUFFER <- DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_OP_TASK 
	} 
	agg.taskResponses[DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_OP_TASK.TaskId] = make(map[sdktypes.TaskResponseDigest]interface{})
	agg.taskResponses[DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_OP_TASK.TaskId][DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_OP_TASK.TaskResponseDigest] = DUMMY_OP_TASK_RESPONSE
	agg.blsAggregationService.(*mocks.MockBlsAggregationService).EXPECT().GetResponseChannel().Return(DUMMY_BLS_AGGREGATION_SERVICE_RESPONSE_CHANNEL_LARGE_BUFFER).Times(DEFAULT_RETRIES)
	agg.ethRpc.AvsWriter.(*mocks.MockAvsWriterer).EXPECT().SendAggregatedOpTaskResponse(gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any()).Return(&DUMMY_RECEIPT, nil).Times(DEFAULT_RETRIES)
	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().ParseOpTaskCompleted(gomock.Any()).Return(nil, errors.New("Failed to ParseOpTaskCompleted")).Times(DEFAULT_RETRIES)
	
	task, err := agg.startNewOpTask()
	assert.NotNil(t, err)
	assert.Equal(t, task, taskmanager.IFinalizerTaskManagerOpTask{})
}

func TestStartNewOpTask_IsTaskPending_Fails(t *testing.T) {
	mockCtrl := gomock.NewController(t)
	defer mockCtrl.Finish()

	config, mockAggConfigExt := getDefaultAggConfig()

	agg, err := NewMockAggregator(mockCtrl, &config, &mockAggConfigExt)
	assert.Nil(t, err)

	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IsTaskPending(gomock.Any()).Return(true, nil)
	
	task, err := agg.startNewOpTask()
	assert.NotNil(t, err)
	assert.Equal(t, task, taskmanager.IFinalizerTaskManagerOpTask{})
}

func TestCreateOpTask_IsTaskPending_Fails(t *testing.T) {
	mockCtrl := gomock.NewController(t)
	defer mockCtrl.Finish()

	config, mockAggConfigExt := getDefaultAggConfig()

	agg, err := NewMockAggregator(mockCtrl, &config, &mockAggConfigExt)
	assert.Nil(t, err)

	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IsTaskPending(gomock.Any()).Return(true, nil)
	
	task, _, err := agg.createOpTask()
	assert.NotNil(t, err)
	assert.Equal(t, task, taskmanager.IFinalizerTaskManagerOpTask{})
}

func TestCreateRdTask_IsTaskPending_Fails(t *testing.T) {
	mockCtrl := gomock.NewController(t)
	defer mockCtrl.Finish()

	config, mockAggConfigExt := getDefaultAggConfig()

	agg, err := NewMockAggregator(mockCtrl, &config, &mockAggConfigExt)
	assert.Nil(t, err)

	agg.ethRpc.AvsReader.(*mocks.MockAvsReaderer).EXPECT().IsTaskPending(gomock.Any()).Return(true, nil)
	
	task, _, err := agg.createRdTask(DUMMY_CHAIN_ID, DUMMY_BATCH_ID)
	assert.NotNil(t, err)
	assert.Equal(t, task, taskmanager.IFinalizerTaskManagerRdTask{})
}
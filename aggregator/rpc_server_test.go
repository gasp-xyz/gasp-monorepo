package aggregator

import (
	"context"
	"testing"

	"github.com/ethereum/go-ethereum/common"
	"github.com/stretchr/testify/assert"
	"go.uber.org/mock/gomock"

	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	sdktypes "github.com/Layr-Labs/eigensdk-go/types"
	"github.com/mangata-finance/eigen-layer-monorepo/aggregator/types"
	taskmanager "github.com/mangata-finance/eigen-layer-monorepo/contracts/bindings/MangataTaskManager"
	"github.com/mangata-finance/eigen-layer-monorepo/core"

	gsrpc_types "github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

func TestProcessSignedTaskResponse(t *testing.T) {
	mockCtrl := gomock.NewController(t)
	defer mockCtrl.Finish()

	var TASK_INDEX = uint32(0)
	var CREATED_BLOCK_NUMBER = uint32(100)
	var BLOCK_TO_VERIFY = gsrpc_types.BlockNumber(3)

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
	aggregator, _, mockBlsAggServ, err := createMockAggregator(mockCtrl, operatorPubkeyDict)
	assert.Nil(t, err)

	signedTaskResponse, err := createMockSignedTaskResponse(MockTask{
		TaskNum:            TASK_INDEX,
		CreatedBlockNumber: CREATED_BLOCK_NUMBER,
		BlockNumber:        uint32(BLOCK_TO_VERIFY),
	}, *MOCK_OPERATOR_KEYPAIR)
	assert.Nil(t, err)
	signedTaskResponseDigest, err := core.GetTaskResponseDigest(&signedTaskResponse.TaskResponse)
	assert.Nil(t, err)

	// TODO(samlaf): is this the right way to test writing to external service?
	// or is there some wisdom to "don't mock 3rd party code"?
	// see https://hynek.me/articles/what-to-mock-in-5-mins/
	mockBlsAggServ.EXPECT().ProcessNewSignature(context.Background(), TASK_INDEX, signedTaskResponseDigest,
		&signedTaskResponse.BlsSignature, signedTaskResponse.OperatorId)
	err = aggregator.ProcessSignedTaskResponse(signedTaskResponse, nil)
	assert.Nil(t, err)
}

// mocks an operator signing on a task response
func createMockSignedTaskResponse(mockTask MockTask, keypair bls.KeyPair) (*SignedTaskResponse, error) {
	taskResponse := &taskmanager.IMangataTaskManagerTaskResponse{
		ReferenceTaskIndex: mockTask.TaskNum,
		BlockHash:          [32]byte{},
	}
	taskResponseHash, err := core.GetTaskResponseDigest(taskResponse)
	if err != nil {
		return nil, err
	}
	blsSignature := keypair.SignMessage(taskResponseHash)
	signedTaskResponse := &SignedTaskResponse{
		TaskResponse: *taskResponse,
		BlsSignature: *blsSignature,
		OperatorId:   MOCK_OPERATOR_ID,
	}
	return signedTaskResponse, nil
}

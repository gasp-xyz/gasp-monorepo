package mocks

import (
	"math/big"

	opstateretriever "github.com/Layr-Labs/eigensdk-go/contracts/bindings/BLSOperatorStateRetriever"
	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	taskmanager "github.com/gasp-xyz/gasp-monorepo/avs-aggregator/bindings/MangataTaskManager"
	"github.com/gasp-xyz/gasp-monorepo/avs-aggregator/types"
)

// ====== TaskManager Mocks ======

func MockSendNewTaskVerifyBlock(blockNum, taskNum uint32, block *big.Int) (taskmanager.IMangataTaskManagerTask, uint32, error) {
	task := taskmanager.IMangataTaskManagerTask{
		BlockNumber:               block,
		TaskCreatedBlock:          blockNum,
		QuorumNumbers:             types.QUORUM_NUMBERS,
		QuorumThresholdPercentage: types.QUORUM_THRESHOLD_NUMERATOR,
	}

	return task, taskNum, nil
}

// ======= BLSOperatorStateRetriever Mocks =======
type MockOperatorState struct {
	OperatorId [32]byte
	Stake      *big.Int
	G1Pubkey   *bls.G1Point
	G2Pubkey   *bls.G2Point
}
type MockRegistry struct {
	OperatorsState []MockOperatorState
}

func (r *MockRegistry) GetOperatorsId() [][32]byte {
	operatorIds := make([][32]byte, len(r.OperatorsState))
	for i, operatorState := range r.OperatorsState {
		operatorIds[i] = operatorState.OperatorId
	}
	return operatorIds
}

func (r *MockRegistry) GetOperatorsTotalStake() *big.Int {
	totalStake := big.NewInt(0)
	for _, operatorState := range r.OperatorsState {
		totalStake.Add(totalStake, operatorState.Stake)
	}
	return totalStake
}

// returns an array of operator states for each quorum number (only [0][] is used since we use a single quorum only)
// currently hardcoded for a single operator
func (r *MockRegistry) MockGetOperatorStateCall() [][]opstateretriever.BLSOperatorStateRetrieverOperator {
	quorum0OperatorStakes := make([]opstateretriever.BLSOperatorStateRetrieverOperator, len(r.OperatorsState))
	for i, operatorState := range r.OperatorsState {
		quorum0OperatorStakes[i] = opstateretriever.BLSOperatorStateRetrieverOperator{
			OperatorId: operatorState.OperatorId,
			Stake:      operatorState.Stake,
		}
	}
	return [][]opstateretriever.BLSOperatorStateRetrieverOperator{
		quorum0OperatorStakes,
	}
}

// Aggregates the g1 pubkey of all operators in the mockregistry, assuming they are all registered for quorum0
func (r *MockRegistry) AggregateG1Pubkey() *bls.G1Point {
	aggregateG1Pubkey := bls.NewG1Point(big.NewInt(0), big.NewInt(0))
	for _, operatorState := range r.OperatorsState {
		aggregateG1Pubkey.Add(operatorState.G1Pubkey)
	}
	return aggregateG1Pubkey
}

package types

import (
	sdktypes "github.com/Layr-Labs/eigensdk-go/types"
	gsrpctypes "github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/ethereum/go-ethereum/common"
	taskmanager "github.com/gasp-xyz/gasp-monorepo/avs-aggregator/bindings/FinalizerTaskManager"
)

// TODO: Hardcoded for now
// 66% operators in quorum0 must sign the task response in order for it to be accepted
const QUORUM_THRESHOLD_NUMERATOR = uint32(66)
const QUORUM_THRESHOLD_DENOMINATOR = uint32(100)
var ZERO_OPERATOR_ID [32]byte
var ZERO_ADDRESS [20]byte

const QUERY_FILTER_FROM_BLOCK = uint64(1)

// For now we set them to be equal
// but the TRACKED.. (and also QUORUM_NUMBERS) will need to be 
// updated when we are introducing a new quorum
var TRACKED_QUORUM_NUMBERS = QUORUM_NUMBERS

// we only use a single quorum (quorum 0)
var QUORUM_NUMBERS = sdktypes.QuorumNums{QUORUM_NUMBER}
var QUORUM_NUMBER = sdktypes.QuorumNum(0)

const TASK_STATUS_INITIALIZED = uint8(1)

// type BlockNumber = uint32
type TaskIndex = uint32

type OperatorInfo struct {
	OperatorPubkeys sdktypes.OperatorPubkeys
	OperatorAddr    common.Address
}

type TriggerOpStateUpdatePing struct{
	TriggerOpStateUpdateErrorC chan error `json:"-"`
}

type SendNewOpTaskType struct{
	SendNewOpTaskReturnC chan SendNewOpTaskReturn `json:"-"`
}

type SendNewOpTaskReturn struct{
	OpTask taskmanager.IFinalizerTaskManagerOpTask
	SendNewOpTaskError error
}

type OperatorAvsState struct {
	OperatorId sdktypes.OperatorId
	Operator   common.Address
	StakePerQuorum map[sdktypes.QuorumNum]sdktypes.StakeAmount
}

type QuorumStakeDiff struct{
	PosDiff sdktypes.StakeAmount
	NegDiff sdktypes.StakeAmount
}

type SubstrateL2RequestsBatchLastItem struct {
	Key gsrpctypes.U8
	Value SubstrateL2RequestsBatchLastValue
}

type SubstrateL2RequestsBatchLastValue struct {
	BlockNumber gsrpctypes.U32
	BatchId gsrpctypes.U128
	BatchRange SubstrateL2RequestsBatchLastRange
}

type SubstrateL2RequestsBatchLastRange struct {
	Start gsrpctypes.U128
	End gsrpctypes.U128
}

type SubstrateL2RequestsBatchLast = []SubstrateL2RequestsBatchLastItem

type SubstrateL2RequestsBatch struct {
	BlockNumber gsrpctypes.U32
	BatchRange SubstrateL2RequestsBatchLastRange
	OperatorAddr common.Address
}


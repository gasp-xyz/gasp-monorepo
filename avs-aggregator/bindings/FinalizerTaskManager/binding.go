// Code generated - DO NOT EDIT.
// This file is a generated binding and any manual changes will be lost.

package contractFinalizerTaskManager

import (
	"errors"
	"math/big"
	"strings"

	ethereum "github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/event"
)

// Reference imports to suppress errors if they are not otherwise used.
var (
	_ = errors.New
	_ = big.NewInt
	_ = strings.NewReader
	_ = ethereum.NotFound
	_ = bind.Bind
	_ = common.Big1
	_ = types.BloomLookup
	_ = event.NewSubscription
	_ = abi.ConvertType
)

// BN254G1Point is an auto generated low-level Go binding around an user-defined struct.
type BN254G1Point struct {
	X *big.Int
	Y *big.Int
}

// BN254G2Point is an auto generated low-level Go binding around an user-defined struct.
type BN254G2Point struct {
	X [2]*big.Int
	Y [2]*big.Int
}

// IBLSSignatureCheckerNonSignerStakesAndSignature is an auto generated low-level Go binding around an user-defined struct.
type IBLSSignatureCheckerNonSignerStakesAndSignature struct {
	NonSignerQuorumBitmapIndices []uint32
	NonSignerPubkeys             []BN254G1Point
	QuorumApks                   []BN254G1Point
	ApkG2                        BN254G2Point
	Sigma                        BN254G1Point
	QuorumApkIndices             []uint32
	TotalStakeIndices            []uint32
	NonSignerStakeIndices        [][]uint32
}

// IBLSSignatureCheckerQuorumStakeTotals is an auto generated low-level Go binding around an user-defined struct.
type IBLSSignatureCheckerQuorumStakeTotals struct {
	SignedStakeForQuorum []*big.Int
	TotalStakeForQuorum  []*big.Int
}

// IFinalizerTaskManagerOpTask is an auto generated low-level Go binding around an user-defined struct.
type IFinalizerTaskManagerOpTask struct {
	TaskNum                                      uint32
	TaskCreatedBlock                             uint32
	LastCompletedOpTaskCreatedBlock              uint32
	QuorumNumbers                                []byte
	QuorumThresholdPercentage                    uint32
	LastCompletedOpTaskQuorumNumbers             []byte
	LastCompletedOpTaskQuorumThresholdPercentage uint32
}

// IFinalizerTaskManagerOpTaskResponse is an auto generated low-level Go binding around an user-defined struct.
type IFinalizerTaskManagerOpTaskResponse struct {
	ReferenceTaskIndex     uint32
	ReferenceTaskHash      [32]byte
	OperatorsStateInfoHash [32]byte
}

// IFinalizerTaskManagerRdTask is an auto generated low-level Go binding around an user-defined struct.
type IFinalizerTaskManagerRdTask struct {
	TaskNum                                      uint32
	ChainId                                      uint8
	BatchId                                      uint32
	TaskCreatedBlock                             uint32
	LastCompletedOpTaskCreatedBlock              uint32
	LastCompletedOpTaskQuorumNumbers             []byte
	LastCompletedOpTaskQuorumThresholdPercentage uint32
}

// IFinalizerTaskManagerRdTaskResponse is an auto generated low-level Go binding around an user-defined struct.
type IFinalizerTaskManagerRdTaskResponse struct {
	ReferenceTaskIndex uint32
	ReferenceTaskHash  [32]byte
	ChainId            uint8
	BatchId            uint32
	RdUpdate           [32]byte
	RangeStart         *big.Int
	RangeEnd           *big.Int
	Updater            common.Address
}

// IFinalizerTaskManagerTaskResponseMetadata is an auto generated low-level Go binding around an user-defined struct.
type IFinalizerTaskManagerTaskResponseMetadata struct {
	TaskResponsedBlock uint32
	HashOfNonSigners   [32]byte
	QuroumStakeTotals  []*big.Int
	QuroumStakeSigned  []*big.Int
}

// IGaspMultiRollupServicePrimitivesOperatorStateInfo is an auto generated low-level Go binding around an user-defined struct.
type IGaspMultiRollupServicePrimitivesOperatorStateInfo struct {
	OperatorsStateChanged      bool
	QuorumsRemoved             []uint8
	QuorumsAdded               []IGaspMultiRollupServicePrimitivesQuorumsAdded
	QuorumsStakeUpdate         []IGaspMultiRollupServicePrimitivesQuorumsStakeUpdate
	QuorumsApkUpdate           []IGaspMultiRollupServicePrimitivesQuorumsApkUpdate
	OperatorsRemoved           [][32]byte
	OperatorsAdded             []IGaspMultiRollupServicePrimitivesOperatorsAdded
	OperatorsStakeUpdate       []IGaspMultiRollupServicePrimitivesOperatorsStakeUpdate
	OperatorsQuorumCountUpdate []IGaspMultiRollupServicePrimitivesOperatorsQuorumCountUpdate
}

// IGaspMultiRollupServicePrimitivesOperatorsAdded is an auto generated low-level Go binding around an user-defined struct.
type IGaspMultiRollupServicePrimitivesOperatorsAdded struct {
	OperatorId      [32]byte
	QuorumForStakes []uint8
	QuorumStakes    []*big.Int
	QuorumCount     uint8
}

// IGaspMultiRollupServicePrimitivesOperatorsQuorumCountUpdate is an auto generated low-level Go binding around an user-defined struct.
type IGaspMultiRollupServicePrimitivesOperatorsQuorumCountUpdate struct {
	OperatorId  [32]byte
	QuorumCount uint8
}

// IGaspMultiRollupServicePrimitivesOperatorsStakeUpdate is an auto generated low-level Go binding around an user-defined struct.
type IGaspMultiRollupServicePrimitivesOperatorsStakeUpdate struct {
	OperatorId      [32]byte
	QuorumForStakes []uint8
	QuorumStakes    []*big.Int
}

// IGaspMultiRollupServicePrimitivesQuorumsAdded is an auto generated low-level Go binding around an user-defined struct.
type IGaspMultiRollupServicePrimitivesQuorumsAdded struct {
	QuorumNumber uint8
	QuorumStake  *big.Int
	QuorumApk    BN254G1Point
}

// IGaspMultiRollupServicePrimitivesQuorumsApkUpdate is an auto generated low-level Go binding around an user-defined struct.
type IGaspMultiRollupServicePrimitivesQuorumsApkUpdate struct {
	QuorumNumber uint8
	QuorumApk    BN254G1Point
}

// IGaspMultiRollupServicePrimitivesQuorumsStakeUpdate is an auto generated low-level Go binding around an user-defined struct.
type IGaspMultiRollupServicePrimitivesQuorumsStakeUpdate struct {
	QuorumNumber uint8
	QuorumStake  *big.Int
}

// OperatorStateRetrieverCheckSignaturesIndices is an auto generated low-level Go binding around an user-defined struct.
type OperatorStateRetrieverCheckSignaturesIndices struct {
	NonSignerQuorumBitmapIndices []uint32
	QuorumApkIndices             []uint32
	TotalStakeIndices            []uint32
	NonSignerStakeIndices        [][]uint32
}

// OperatorStateRetrieverOperator is an auto generated low-level Go binding around an user-defined struct.
type OperatorStateRetrieverOperator struct {
	Operator   common.Address
	OperatorId [32]byte
	Stake      *big.Int
}

// ContractFinalizerTaskManagerMetaData contains all meta data concerning the ContractFinalizerTaskManager contract.
var ContractFinalizerTaskManagerMetaData = &bind.MetaData{
	ABI: "[{\"type\":\"function\",\"name\":\"THRESHOLD_DENOMINATOR\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"aggregator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allTaskHashes\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumIFinalizerTaskManager.TaskType\"},{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allTaskResponses\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumIFinalizerTaskManager.TaskType\"},{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allowNonRootInit\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"blsApkRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIBLSApkRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"blsSignatureChecker\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractBLSSignatureChecker\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"chainRdBatchNonce\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"checkSignatures\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"referenceBlockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.QuorumStakeTotals\",\"components\":[{\"name\":\"signedStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"totalStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"createNewOpTask\",\"inputs\":[{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"createNewRdTask\",\"inputs\":[{\"name\":\"chainId\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"},{\"name\":\"batchId\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"delegation\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIDelegationManager\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"dummyForOperatorStateInfoType\",\"inputs\":[{\"name\":\"_operatorStateInfo\",\"type\":\"tuple\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorStateInfo\",\"components\":[{\"name\":\"operatorsStateChanged\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"quorumsRemoved\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsAdded[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"quorumsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsStakeUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]},{\"name\":\"quorumsApkUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsApkUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"operatorsRemoved\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"operatorsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsAdded[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]},{\"name\":\"operatorsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsStakeUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"operatorsQuorumCountUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsQuorumCountUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]}]}],\"outputs\":[],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"dummyForQuorumStakeTotalsType\",\"inputs\":[{\"name\":\"_quorumStakeTotals\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.QuorumStakeTotals\",\"components\":[{\"name\":\"signedStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"totalStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]}],\"outputs\":[],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"forceCancelPendingTasks\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"forceCreateNewOpTask\",\"inputs\":[{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"forceRespondToOpTask\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"generator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getCheckSignaturesIndices\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"referenceBlockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"nonSignerOperatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structOperatorStateRetriever.CheckSignaturesIndices\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getOperatorState\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple[][]\",\"internalType\":\"structOperatorStateRetriever.Operator[][]\",\"components\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"stake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getOperatorState\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"\",\"type\":\"tuple[][]\",\"internalType\":\"structOperatorStateRetriever.Operator[][]\",\"components\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"stake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getQuorumBitmapsAtBlockNumber\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256[]\",\"internalType\":\"uint256[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"idToTaskStatus\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumIFinalizerTaskManager.TaskType\"},{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumIFinalizerTaskManager.TaskStatus\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"_pauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"initialOwner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_aggregator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_generator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_allowNonRootInit\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"_blsSignatureCheckerAddress\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_taskResponseWindowBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"_operatorStateRetrieverExtended\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_rolldown\",\"type\":\"address\",\"internalType\":\"contractIRolldown\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"isTaskPending\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastCompletedOpTaskCreatedBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastCompletedOpTaskNum\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastOpTaskCreatedBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestOpTaskNum\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestRdTaskNum\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"operatorStateRetrieverExtended\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"operatorsStateInfoHash\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"pauseAll\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"pauseTrackingOpState\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[{\"name\":\"index\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pauserRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"registryCoordinator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"respondToOpTask\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"respondToRdTask\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.RdTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"chainId\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"},{\"name\":\"batchId\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.RdTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"chainId\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"},{\"name\":\"batchId\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"rdUpdate\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"rangeStart\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"rangeEnd\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"updater\",\"type\":\"address\",\"internalType\":\"address\"}]},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"resumeTrackingQuorums\",\"inputs\":[{\"name\":\"resetTrackedQuorums\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"rolldown\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIRolldown\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"setPauserRegistry\",\"inputs\":[{\"name\":\"newPauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setRolldown\",\"inputs\":[{\"name\":\"_rolldown\",\"type\":\"address\",\"internalType\":\"contractIRolldown\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"stakeRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIStakeRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"taskResponseWindowBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"unpause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateBlsSignatureCheckerAddress\",\"inputs\":[{\"name\":\"_blsSignatureCheckerAddress\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"BLSSignatureCheckerAddressUpdated\",\"inputs\":[{\"name\":\"blsSignatureCheckerAddress\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Initialized\",\"inputs\":[{\"name\":\"version\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewOpTaskCreated\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"task\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.OpTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewOpTaskForceCreated\",\"inputs\":[],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewRdTaskCreated\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"task\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.RdTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"chainId\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"},{\"name\":\"batchId\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OpTaskCancelled\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OpTaskCompleted\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OpTaskForceCompleted\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OpTaskResponded\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"taskResponseMetadata\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.TaskResponseMetadata\",\"components\":[{\"name\":\"taskResponsedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"hashOfNonSigners\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quroumStakeTotals\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quroumStakeSigned\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"PauseTrackingOpState\",\"inputs\":[],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Paused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"PauserRegistrySet\",\"inputs\":[{\"name\":\"pauserRegistry\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"newPauserRegistry\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractIPauserRegistry\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RdTaskCancelled\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RdTaskCompleted\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.RdTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"chainId\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"},{\"name\":\"batchId\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"rdUpdate\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"rangeStart\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"rangeEnd\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"updater\",\"type\":\"address\",\"internalType\":\"address\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RdTaskResponded\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.RdTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"chainId\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"},{\"name\":\"batchId\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"rdUpdate\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"rangeStart\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"rangeEnd\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"updater\",\"type\":\"address\",\"internalType\":\"address\"}]},{\"name\":\"taskResponseMetadata\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.TaskResponseMetadata\",\"components\":[{\"name\":\"taskResponsedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"hashOfNonSigners\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quroumStakeTotals\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quroumStakeSigned\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"ResumeTrackingOpState\",\"inputs\":[{\"name\":\"resetTrackedQuorums\",\"type\":\"bool\",\"indexed\":false,\"internalType\":\"bool\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RolldownTargetUpdated\",\"inputs\":[{\"name\":\"rolldownAddress\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"StaleStakesForbiddenUpdate\",\"inputs\":[{\"name\":\"value\",\"type\":\"bool\",\"indexed\":false,\"internalType\":\"bool\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Unpaused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false}]",
	Bin: "0x608060405234801561001057600080fd5b50615ecf80620000216000396000f3fe608060405234801561001057600080fd5b50600436106103425760003560e01c80636d14a987116101b8578063930390d911610104578063e70c2623116100a2578063f2fde38b1161007c578063f2fde38b1461079e578063f5640cf8146107b1578063fabc1cbc146107c4578063fdc15de8146107d757600080fd5b8063e70c262314610773578063e72ddf1014610783578063ef0244581461079657600080fd5b8063bf2315ed116100de578063bf2315ed146106fc578063cefdc1d414610737578063de43483814610758578063df5cf7231461076b57600080fd5b8063930390d9146106b6578063a69563a9146106dc578063adfcb048146106f357600080fd5b806379badf73116101715780638380acbd1161014b5780638380acbd14610668578063886f11951461067b5780638da5cb5b1461068e5780638fc8729a1461069f57600080fd5b806379badf73146106365780637afa1eed1461063e5780637afdd54b1461065157600080fd5b80636d14a987146105cc5780636e125ff4146105d45780636efb4636146105e75780636f25481914610608578063715018a61461061b578063723114ab1461062357600080fd5b806341789d5711610292578063595c6a67116102305780635c975abb1161020a5780635c975abb146105ac5780635df45946146105b457806360202fc0146105bc57806368304835146105c457600080fd5b8063595c6a67146105615780635ac86ab7146105695780635c1556621461058c57600080fd5b80634f739f741161026c5780634f739f7414610509578063516a722714610529578063537a29291461053c57806354d127de1461055357600080fd5b806341789d57146104bc57806345265b7a146104e85780634d7a7116146104f957600080fd5b80631ac27297116102ff5780632830e8f9116102d95780632830e8f9146104605780633563b0d11461047557806336f78ed8146104955780633d9fb00c146104a957600080fd5b80631ac27297146103f05780631c178e9c1461041b578063245a7bfc1461044657600080fd5b806301a3f013146103475780630ee0fdbd1461035c57806310d67a2f1461037e578063136439dd1461039157806313f815ed146103a4578063191aac7a146103dd575b600080fd5b61035a6103553660046142e5565b6107ea565b005b60a3546103699060ff1681565b60405190151581526020015b60405180910390f35b61035a61038c366004614348565b610cc9565b61035a61039f366004614365565b610d79565b6103cf6103b23660046143ad565b609960209081526000928352604080842090915290825290205481565b604051908152602001610375565b61035a6103eb3660046143f4565b610eb8565b6103cf6103fe3660046143ad565b609a60209081526000928352604080842090915290825290205481565b60975461042e906001600160a01b031681565b6040516001600160a01b039091168152602001610375565b609e5461042e90600160201b90046001600160a01b031681565b610468610ef9565b604051610375919061445e565b610488610483366004614502565b610f87565b604051610375919061465d565b60a05461036990600160a01b900460ff1681565b60a05461042e906001600160a01b031681565b6098546104d390600160c01b900463ffffffff1681565b60405163ffffffff9091168152602001610375565b61035a6104f6366004614670565b50565b609c546104d39063ffffffff1681565b61051c6105173660046146f2565b61141f565b604051610375919061484b565b61035a610537366004614c2d565b611b45565b609c546104d390600160401b900463ffffffff1681565b61035a6104f6366004614ca1565b61035a61228b565b610369610577366004614cdc565b606654600160ff9092169190911b9081161490565b61059f61059a366004614cff565b612352565b6040516103759190614dab565b6066546103cf565b61042e61251a565b61035a61258d565b61042e6125cd565b61042e612617565b61035a6105e2366004614def565b612661565b6105fa6105f5366004614e43565b612704565b604051610375929190614f03565b61035a6106163660046143ad565b6127a4565b61035a612ae1565b61035a610631366004614348565b612af3565b61035a612b49565b609f5461042e906001600160a01b031681565b6098546104d390600160e01b900463ffffffff1681565b60985461042e906001600160a01b031681565b60655461042e906001600160a01b031681565b6033546001600160a01b031661042e565b609c546104d390600160201b900463ffffffff1681565b6104d36106c4366004614f4c565b60a16020526000908152604090205463ffffffff1681565b6098546104d390600160a01b900463ffffffff1681565b6103cf60a25481565b61072a61070a3660046143ad565b609b60209081526000928352604080842090915290825290205460ff1681565b6040516103759190614f7f565b61074a610745366004614f99565b612b7c565b604051610375929190614fdb565b61035a610766366004614ffc565b612d0e565b61042e612ec6565b609e546104d39063ffffffff1681565b61035a6107913660046150b8565b612f10565b6103cf606481565b61035a6107ac366004614348565b6136c3565b61035a6107bf366004614def565b613739565b61035a6107d2366004614365565b613794565b61035a6107e5366004614348565b6138f0565b6107f2613996565b60006108016020830183615130565b905060006108156060850160408601615130565b905060006108296040860160208701615130565b905036600061083b60a088018861514d565b9092509050600061085260e0890160c08a01615130565b60a054909150600160a01b900460ff16151560011461088c5760405162461bcd60e51b815260040161088390615193565b60405180910390fd5b6000808052609960209081527f235d629dc802037ded8c61cb27fb29e40fa01b299719d8f991ffe20bdcc59f4f91906108c7908a018a615130565b63ffffffff1663ffffffff16815260200190815260200160002054886040516020016108f3919061522a565b60405160208183030381529060405280519060200120146109265760405162461bcd60e51b815260040161088390615304565b6000808052609b6020908152600191600080516020615e5a83398151915291610951908b018b615130565b63ffffffff16815260208101919091526040016000205460ff16600481111561097c5761097c614f69565b146109995760405162461bcd60e51b81526004016108839061532b565b6000808052609a60209081527fbe6620bd3346e5d7f8387fbec0981aa0d6289d22efa7c935f9ef6841bf2a98c79082906109d5908b018b615130565b63ffffffff1663ffffffff1681526020019081526020016000205414610a0d5760405162461bcd60e51b815260040161088390615353565b609854610a2790600160a01b900463ffffffff168561538d565b63ffffffff164363ffffffff161115610a525760405162461bcd60e51b8152600401610883906153b5565b60408051808201909152606080825260208201526040805160808101825263ffffffff431681526000602080830182905284810151838501528451606084015292519092610aa4918c91849101615447565b60408051601f1981840301815291905280516020918201206000808052609a835290917fbe6620bd3346e5d7f8387fbec0981aa0d6289d22efa7c935f9ef6841bf2a98c79190610af6908e018e615130565b63ffffffff16815260208082019290925260409081016000908120939093558c013560a255818052609b8152600491600080516020615e5a83398151915291610b41908e018e615130565b63ffffffff16815260208101919091526040016000205460ff166004811115610b6c57610b6c614f69565b50610b7f905060408c0160208d01615130565b609c805463ffffffff92909216600160401b0263ffffffff60401b19909216919091179055610bb160608c018c61514d565b610bbd91609d91614222565b50610bce60a08c0160808d01615130565b609e805463ffffffff191663ffffffff92909216919091179055610bf560208c018c615130565b609c805463ffffffff92909216600160201b0267ffffffff000000001990921691909117905560a0805460ff60a01b19169055610c3560208b018b615130565b63ffffffff167fff2908483d74b6b70053dd473260acf1b09e0ba0781bf94100bb8277581749de8b604051610c6a9190615467565b60405180910390a2610c7f60208b018b615130565b63ffffffff167fdf22f3558e4841b63d77179546b3eae63e4e343bbe752746b093162bc526be4c8b604051610cb49190615467565b60405180910390a25050505050505050505050565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610d1c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d409190615475565b6001600160a01b0316336001600160a01b031614610d705760405162461bcd60e51b815260040161088390615492565b6104f6816139f0565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa158015610dc1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610de591906154dc565b610e015760405162461bcd60e51b8152600401610883906154f9565b60665481811614610e7a5760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e70617573653a20696e76616c696420617474656d70742060448201527f746f20756e70617573652066756e6374696f6e616c69747900000000000000006064820152608401610883565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b610ec0613996565b60405181151581527f6af4ae1f481aff20ce571abd65375b67b22359883a823d1ddf4bd8f2879ff7ba906020015b60405180910390a150565b609d8054610f0690615541565b80601f0160208091040260200160405190810160405280929190818152602001828054610f3290615541565b8015610f7f5780601f10610f5457610100808354040283529160200191610f7f565b820191906000526020600020905b815481529060010190602001808311610f6257829003601f168201915b505050505081565b60606000846001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa158015610fc9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610fed9190615475565b90506000856001600160a01b0316639e9923c26040518163ffffffff1660e01b8152600401602060405180830381865afa15801561102f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906110539190615475565b90506000866001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa158015611095573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906110b99190615475565b9050600086516001600160401b038111156110d6576110d6614471565b60405190808252806020026020018201604052801561110957816020015b60608152602001906001900390816110f45790505b50905060005b875181101561141157600088828151811061112c5761112c615576565b0160200151604051638902624560e01b815260f89190911c6004820181905263ffffffff8a16602483015291506000906001600160a01b03871690638902624590604401600060405180830381865afa15801561118d573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526111b5919081019061558c565b905080516001600160401b038111156111d0576111d0614471565b60405190808252806020026020018201604052801561121b57816020015b60408051606081018252600080825260208083018290529282015282526000199092019101816111ee5790505b5084848151811061122e5761122e615576565b602002602001018190525060005b81518110156113fb576040518060600160405280876001600160a01b03166347b314e885858151811061127157611271615576565b60200260200101516040518263ffffffff1660e01b815260040161129791815260200190565b602060405180830381865afa1580156112b4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906112d89190615475565b6001600160a01b031681526020018383815181106112f8576112f8615576565b60200260200101518152602001896001600160a01b031663fa28c62785858151811061132657611326615576565b60209081029190910101516040516001600160e01b031960e084901b168152600481019190915260ff8816602482015263ffffffff8f166044820152606401602060405180830381865afa158015611382573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906113a69190615633565b6001600160601b03168152508585815181106113c4576113c4615576565b602002602001015182815181106113dd576113dd615576565b602002602001018190525080806113f39061564e565b91505061123c565b50505080806114099061564e565b91505061110f565b5093505050505b9392505050565b61144a6040518060800160405280606081526020016060815260200160608152602001606081525090565b6000876001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa15801561148a573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906114ae9190615475565b90506114db6040518060800160405280606081526020016060815260200160608152602001606081525090565b6040516361c8a12f60e11b81526001600160a01b038a169063c391425e9061150b908b9089908990600401615669565b600060405180830381865afa158015611528573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261155091908101906156b3565b81526040516340e03a8160e11b81526001600160a01b038316906381c0750290611582908b908b908b90600401615741565b600060405180830381865afa15801561159f573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526115c791908101906156b3565b6040820152856001600160401b038111156115e4576115e4614471565b60405190808252806020026020018201604052801561161757816020015b60608152602001906001900390816116025790505b50606082015260005b60ff8116871115611a56576000856001600160401b0381111561164557611645614471565b60405190808252806020026020018201604052801561166e578160200160208202803683370190505b5083606001518360ff168151811061168857611688615576565b602002602001018190525060005b868110156119565760008c6001600160a01b03166304ec63518a8a858181106116c1576116c1615576565b905060200201358e886000015186815181106116df576116df615576565b60200260200101516040518463ffffffff1660e01b815260040161171c9392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa158015611739573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061175d9190615761565b90506001600160c01b0381166118015760405162461bcd60e51b815260206004820152605c60248201527f4f70657261746f7253746174655265747269657665722e676574436865636b5360448201527f69676e617475726573496e64696365733a206f70657261746f72206d7573742060648201527f6265207265676973746572656420617420626c6f636b6e756d62657200000000608482015260a401610883565b8a8a8560ff1681811061181657611816615576565b6001600160c01b03841692013560f81c9190911c60019081161415905061194357856001600160a01b031663dd9846b98a8a8581811061185857611858615576565b905060200201358d8d8860ff1681811061187457611874615576565b6040516001600160e01b031960e087901b1681526004810194909452919091013560f81c60248301525063ffffffff8f166044820152606401602060405180830381865afa1580156118ca573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906118ee919061578a565b85606001518560ff168151811061190757611907615576565b6020026020010151848151811061192057611920615576565b63ffffffff909216602092830291909101909101528261193f8161564e565b9350505b508061194e8161564e565b915050611696565b506000816001600160401b0381111561197157611971614471565b60405190808252806020026020018201604052801561199a578160200160208202803683370190505b50905060005b82811015611a1b5784606001518460ff16815181106119c1576119c1615576565b602002602001015181815181106119da576119da615576565b60200260200101518282815181106119f4576119f4615576565b63ffffffff9092166020928302919091019091015280611a138161564e565b9150506119a0565b508084606001518460ff1681518110611a3657611a36615576565b602002602001018190525050508080611a4e906157a7565b915050611620565b506000896001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa158015611a97573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611abb9190615475565b60405163354952a360e21b81529091506001600160a01b0382169063d5254a8c90611aee908b908b908e906004016157c7565b600060405180830381865afa158015611b0b573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611b3391908101906156b3565b60208301525098975050505050505050565b609c54600160401b900463ffffffff16156000611b686060860160408701615130565b90508115611bf35760a35460ff1615611bb157609e54600160201b90046001600160a01b03163314611bac5760405162461bcd60e51b8152600401610883906157f1565b611c24565b6033546001600160a01b03163314611bac5760405162461bcd60e51b815260206004820152600560248201526420baba341960d91b6044820152606401610883565b609e54600160201b90046001600160a01b03163314611c245760405162461bcd60e51b8152600401610883906157f1565b6000611c366040870160208801615130565b9050366000611c4860a089018961514d565b90925090506000611c5f60e08a0160c08b01615130565b60a054909150600160a01b900460ff161515600114611c905760405162461bcd60e51b815260040161088390615193565b6000808052609960209081527f235d629dc802037ded8c61cb27fb29e40fa01b299719d8f991ffe20bdcc59f4f9190611ccb908b018b615130565b63ffffffff1663ffffffff1681526020019081526020016000205489604051602001611cf7919061522a565b6040516020818303038152906040528051906020012014611d2a5760405162461bcd60e51b815260040161088390615304565b6000808052609b6020908152600191600080516020615e5a83398151915291611d55908c018c615130565b63ffffffff16815260208101919091526040016000205460ff166004811115611d8057611d80614f69565b14611d9d5760405162461bcd60e51b81526004016108839061532b565b6000808052609a60209081527fbe6620bd3346e5d7f8387fbec0981aa0d6289d22efa7c935f9ef6841bf2a98c7908290611dd9908c018c615130565b63ffffffff1663ffffffff1681526020019081526020016000205414611e115760405162461bcd60e51b815260040161088390615353565b609854611e2b90600160a01b900463ffffffff168561538d565b63ffffffff164363ffffffff161115611e565760405162461bcd60e51b8152600401610883906153b5565b600088604051602001611e699190615467565b60408051601f198184030181528282528051602091820120838301909252606080845290830152915060008815611f2e57609760009054906101000a90046001600160a01b03166001600160a01b0316636efb46368488888c8f6040518663ffffffff1660e01b8152600401611ee395949392919061589a565b600060405180830381865afa158015611f00573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611f2891908101906159fa565b90925090505b6040805160808101825263ffffffff43168152602080820184905284810151828401528451606083015291519091611f6a918e91849101615447565b60408051808303601f190181529190528051602090910120609a600080815260200190815260200160002060008e6000016020810190611faa9190615130565b63ffffffff1681526020810191909152604001600020556003609b6000806001811115611fd957611fd9614f69565b815260200190815260200160002060008e6000016020810190611ffc9190615130565b63ffffffff16815260208101919091526040016000205460ff16600481111561202757612027614f69565b50612037905060208e018e615130565b63ffffffff167f47adacb0b6bbd726ae39ac6c006cca1c2006c9aedaa882dcba7c4804db7c41ce8d8360405161206e929190615447565b60405180910390a260a0805460ff60a01b1916905589156121265760005b86811015612124578560ff16846020015182815181106120ae576120ae615576565b60200260200101516120c09190615a96565b6001600160601b03166064856000015183815181106120e1576120e1615576565b60200260200101516001600160601b03166120fc9190615ac5565b1015612112575050505050505050505050505050565b8061211c8161564e565b91505061208c565b505b60408c013560a2556004609b600080815260200190815260200160002060008e60000160208101906121589190615130565b63ffffffff16815260208101919091526040016000205460ff16600481111561218357612183614f69565b50612196905060408e0160208f01615130565b609c805463ffffffff92909216600160401b0263ffffffff60401b199092169190911790556121c860608e018e61514d565b6121d491609d91614222565b506121e560a08e0160808f01615130565b609e805463ffffffff191663ffffffff9290921691909117905561220c60208e018e615130565b609c805463ffffffff92909216600160201b0267ffffffff000000001990921691909117905561223f60208d018d615130565b63ffffffff167fff2908483d74b6b70053dd473260acf1b09e0ba0781bf94100bb8277581749de8d6040516122749190615467565b60405180910390a250505050505050505050505050565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa1580156122d3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906122f791906154dc565b6123135760405162461bcd60e51b8152600401610883906154f9565b600019606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b60606000846001600160a01b031663c391425e84866040518363ffffffff1660e01b8152600401612384929190615ae4565b600060405180830381865afa1580156123a1573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526123c991908101906156b3565b9050600084516001600160401b038111156123e6576123e6614471565b60405190808252806020026020018201604052801561240f578160200160208202803683370190505b50905060005b855181101561251057866001600160a01b03166304ec635187838151811061243f5761243f615576565b60200260200101518786858151811061245a5761245a615576565b60200260200101516040518463ffffffff1660e01b81526004016124979392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa1580156124b4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906124d89190615761565b6001600160c01b03168282815181106124f3576124f3615576565b6020908102919091010152806125088161564e565b915050612415565b5095945050505050565b60975460408051632efa2ca360e11b815290516000926001600160a01b031691635df459469160048083019260209291908290030181865afa158015612564573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906125889190615475565b905090565b612595613996565b60a054600160a01b900460ff1615156001146125c35760405162461bcd60e51b815260040161088390615193565b6125cb613ae7565b565b60975460408051636830483560e01b815290516000926001600160a01b03169163683048359160048083019260209291908290030181865afa158015612564573d6000803e3d6000fd5b60975460408051636d14a98760e01b815290516000926001600160a01b031691636d14a9879160048083019260209291908290030181865afa158015612564573d6000803e3d6000fd5b609f546001600160a01b031633146126a35760405162461bcd60e51b8152602060048201526005602482015264417574683160d81b6044820152606401610883565b60a054600160a01b900460ff16156126f45760405162461bcd60e51b81526020600482015260146024820152735461736b20616c72656164792070656e64696e6760601b6044820152606401610883565b6126ff838383613c90565b505050565b604080518082019091526060808252602082015260975460405163377da31b60e11b81526000916001600160a01b031690636efb463690612751908a908a908a908a908a9060040161589a565b600060405180830381865afa15801561276e573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261279691908101906159fa565b915091509550959350505050565b609f546001600160a01b031633146127e65760405162461bcd60e51b8152602060048201526005602482015264417574683160d81b6044820152606401610883565b60a054600160a01b900460ff16156128375760405162461bcd60e51b81526020600482015260146024820152735461736b20616c72656164792070656e64696e6760601b6044820152606401610883565b609c54600160401b900463ffffffff161580159061285457504315155b6128925760405162461bcd60e51b815260206004820152600f60248201526e13dc0814dd185d19481d5b9a5b9a5d608a1b6044820152606401610883565b60985463ffffffff600160e01b8204811691600160c01b9004166128ec6040805160e08101909152600080825260208201908152600060208201819052604082018190526060808301829052608083015260a09091015290565b63ffffffff831681526020810185600181111561290b5761290b614f69565b9081600181111561291e5761291e614f69565b90525063ffffffff80851660408301524381166060830152609e541660c0820152609d805461294c90615541565b80601f016020809104026020016040519081016040528092919081815260200182805461297890615541565b80156129c55780601f1061299a576101008083540402835291602001916129c5565b820191906000526020600020905b8154815290600101906020018083116129a857829003601f168201915b505050505060a0820152609c54600160401b900463ffffffff1660808201526040516129f5908290602001615b4c565b60408051601f19818403018152828252805160209182012063ffffffff871660008181527fbb86fbc034f4e382929974bcd8419ed626b0ea647f962d89ba2fb6bd28785ab9845284812092909255600080516020615e7a83398151915290925291909120805460ff1916600117905560a08054600160a01b60ff60a01b19909116179055907f584637a8f9d0f91a80c9f709b2b09d7db1d770fc7294e20d9d2495c378586cd290612aa7908490615b4c565b60405180910390a2612aba83600161538d565b6098601c6101000a81548163ffffffff021916908363ffffffff1602179055505050505050565b612ae9613996565b6125cb6000613fe9565b612afb613996565b609780546001600160a01b0319166001600160a01b0383169081179091556040519081527f901a654dc830c94e8a12c9a3bc0a92ac11b5cf28046ca8d190691cdaf520901690602001610eee565b612b51613996565b6040517f4d60154266b2ea0c8f091d257eac5abc941c46cb54d0c3069a830f6339fe1da190600090a1565b6040805160018082528183019092526000916060918391602080830190803683370190505090508481600081518110612bb757612bb7615576565b60209081029190910101526040516361c8a12f60e11b81526000906001600160a01b0388169063c391425e90612bf39088908690600401615ae4565b600060405180830381865afa158015612c10573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052612c3891908101906156b3565b600081518110612c4a57612c4a615576565b60209081029190910101516040516304ec635160e01b81526004810188905263ffffffff87811660248301529091166044820181905291506000906001600160a01b038916906304ec635190606401602060405180830381865afa158015612cb6573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612cda9190615761565b6001600160c01b031690506000612cf08261403b565b905081612cfe8a838a610f87565b9550955050505050935093915050565b600054610100900460ff1615808015612d2e5750600054600160ff909116105b80612d485750303b158015612d48575060005460ff166001145b612dab5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610883565b6000805460ff191660011790558015612dce576000805461ff0019166101001790555b612dd98a6000614107565b612de289613fe9565b609e8054640100000000600160c01b031916600160201b6001600160a01b038b81169190910291909117909155609f80546001600160a01b03199081168a84161790915560a3805460ff1916891515179055609780548216888416179055609880548684166001600160c01b031990911617600160a01b63ffffffff89160217905560a080549091169184169190911790558015612eba576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050505050505050565b6097546040805163df5cf72360e01b815290516000926001600160a01b03169163df5cf7239160048083019260209291908290030181865afa158015612564573d6000803e3d6000fd5b609e54600160201b90046001600160a01b03163314612f415760405162461bcd60e51b8152600401610883906157f1565b6000612f5360a0850160808601615130565b90506000612f676080860160608701615130565b9050366000612f7960a088018861514d565b90925090506000612f9060e0890160c08a01615130565b905060a16000612fa660608a0160408b01614f4c565b6001811115612fb757612fb7614f69565b6001811115612fc857612fc8614f69565b815260208101919091526040016000205463ffffffff16612fef6080890160608a01615130565b63ffffffff16146130425760405162461bcd60e51b815260206004820152601a60248201527f636861696e526442617463684e6f6e6365206d69736d617463680000000000006044820152606401610883565b60a054600160a01b900460ff1615156001146130705760405162461bcd60e51b815260040161088390615193565b60016000908152609960209081527fbb86fbc034f4e382929974bcd8419ed626b0ea647f962d89ba2fb6bd28785ab991906130ad908a018a615130565b63ffffffff1663ffffffff16815260200190815260200160002054886040516020016130d99190615bca565b604051602081830303815290604052805190602001201461310c5760405162461bcd60e51b815260040161088390615304565b60016000818152609b6020908152600080516020615e7a8339815191529190613137908b018b615130565b63ffffffff16815260208101919091526040016000205460ff16600481111561316257613162614f69565b1461317f5760405162461bcd60e51b81526004016108839061532b565b60016000908152609a60209081527f5b542b52981c4f2fa9965514d5bb7f37f1b7bc0902a6a4dc6b04dc05be85586b9082906131bd908b018b615130565b63ffffffff1663ffffffff16815260200190815260200160002054146131f55760405162461bcd60e51b815260040161088390615353565b60985461320f90600160a01b900463ffffffff168561538d565b63ffffffff164363ffffffff16111561323a5760405162461bcd60e51b8152600401610883906153b5565b60008760405160200161324d9190615d16565b604051602081830303815290604052805190602001209050600080609760009054906101000a90046001600160a01b03166001600160a01b0316636efb46368488888c8e6040518663ffffffff1660e01b81526004016132b195949392919061589a565b600060405180830381865afa1580156132ce573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526132f691908101906159fa565b6040805160808101825263ffffffff43168152602080820184905280850151828401528451606083015291519395509193509091613338918d91849101615d25565b60408051601f19818403018152919052805160209182012060016000908152609a835290917f5b542b52981c4f2fa9965514d5bb7f37f1b7bc0902a6a4dc6b04dc05be85586b919061338c908f018f615130565b63ffffffff1681526020810191909152604001600020556003609b60006001808111156133bb576133bb614f69565b815260200190815260200160002060008d60000160208101906133de9190615130565b63ffffffff16815260208101919091526040016000205460ff16600481111561340957613409614f69565b50613419905060208d018d615130565b63ffffffff167f82e5c8e9447510b867d248c892385ba34fa6c2d4c4c26ff6868499ae4027f2c68c83604051613450929190615d25565b60405180910390a260a0805460ff60a01b1916905560005b868110156134ff578560ff168460200151828151811061348a5761348a615576565b602002602001015161349c9190615a96565b6001600160601b03166064856000015183815181106134bd576134bd615576565b60200260200101516001600160601b03166134d89190615ac5565b10156134ed5750505050505050505050505050565b806134f78161564e565b915050613468565b5060016000908152609b6020908152600491600080516020615e7a8339815191529161352d908f018f615130565b63ffffffff16815260208101919091526040016000205460ff16600481111561355857613558614f69565b505060408051808201909152600080825260208201528b60a001358160000181815250508b60c0013581602001818152505060a060009054906101000a90046001600160a01b03166001600160a01b03166308f42d408d60800135836040518363ffffffff1660e01b81526004016135d1929190615d47565b600060405180830381600087803b1580156135eb57600080fd5b505af11580156135ff573d6000803e3d6000fd5b506136149250505060808d0160608e01615130565b61361f90600161538d565b60a160008e60400160208101906136369190614f4c565b600181111561364757613647614f69565b600181111561365857613658614f69565b8152602080820192909252604001600020805463ffffffff191663ffffffff939093169290921790915561368e908d018d615130565b63ffffffff167f1797ca59e06ea4a0efe10ac0fb51b58c8acf5cfedbc15fae51c10021dcb906e68d6040516122749190615d16565b6136cb613996565b6001600160a01b0381166137305760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610883565b6104f681613fe9565b613741613996565b60a054600160a01b900460ff161561375b5761375b613ae7565b613766838383613c90565b6040517f4ee987e5f1be19cabfb1a243e5c423889f060f33266753953ff0cf9db89966ab90600090a1505050565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156137e7573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061380b9190615475565b6001600160a01b0316336001600160a01b03161461383b5760405162461bcd60e51b815260040161088390615492565b6066541981196066541916146138b95760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e756e70617573653a20696e76616c696420617474656d7060448201527f7420746f2070617573652066756e6374696f6e616c69747900000000000000006064820152608401610883565b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c90602001610ead565b606654156139405760405162461bcd60e51b815260206004820152601c60248201527f5061757361626c653a20636f6e747261637420697320706175736564000000006044820152606401610883565b613948613996565b60a080546001600160a01b0319166001600160a01b0383169081179091556040519081527f2f20cf1bda67739044c5bf577353970c3dbc183b2c7274d1e8584a102692326790602001610eee565b6033546001600160a01b031633146125cb5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610883565b6001600160a01b038116613a7e5760405162461bcd60e51b815260206004820152604960248201527f5061757361626c652e5f73657450617573657252656769737472793a206e657760448201527f50617573657252656769737472792063616e6e6f7420626520746865207a65726064820152686f206164647265737360b81b608482015260a401610883565b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b609854600160c01b900463ffffffff1615613bb457609854600090613b1b90600190600160c01b900463ffffffff16615d65565b63ffffffff81166000908152600080516020615e5a833981519152602052604090205490915060019060ff166004811115613b5857613b58614f69565b1415613bb25763ffffffff81166000818152600080516020615e5a8339815191526020526040808220805460ff19166002179055517fd6a4e0ff9f3a053708757c7a124abee31ced61f43f17e6e1cf11943ec59e60719190a25b505b609854600160e01b900463ffffffff1615613c8157609854600090613be890600190600160e01b900463ffffffff16615d65565b63ffffffff81166000908152600080516020615e7a833981519152602052604090205490915060019060ff166004811115613c2557613c25614f69565b1415613c7f5763ffffffff81166000818152600080516020615e7a8339815191526020526040808220805460ff19166002179055517f0bf46bfca6e2137d35b893c295add8c33bcfbffafdef93252cb51aed7538ba0c9190a25b505b60a0805460ff60a01b19169055565b609c5463ffffffff600160401b909104164314801590613caf57504315155b613d0c5760405162461bcd60e51b815260206004820152602860248201527f43616e277420696e206c617374436f6d706c657465644f705461736b43726561604482015267746564426c6f636b60c01b6064820152608401610883565b6040805160e0810182526000818301819052606080830181905260a083015260c0820152609854600160c01b900463ffffffff908116825243811660208084019190915290861660808301528251601f850182900482028101820190935283835290919084908490819084018382808284376000920191909152505050506060820152609c54600160401b900463ffffffff16613dfb5763ffffffff431660408083019190915280516020601f850181900481028201810190925283815290849084908190840183828082843760009201919091525050505060a082015263ffffffff841660c0820152613eaf565b609c54600160401b900463ffffffff166040820152609d8054613e1d90615541565b80601f0160208091040260200160405190810160405280929190818152602001828054613e4990615541565b8015613e965780601f10613e6b57610100808354040283529160200191613e96565b820191906000526020600020905b815481529060010190602001808311613e7957829003601f168201915b505050505060a0820152609e5463ffffffff1660c08201525b80604051602001613ec09190615d8a565b60408051808303601f19018152828252805160209182012060988054600160c01b9081900463ffffffff90811660009081527f235d629dc802037ded8c61cb27fb29e40fa01b299719d8f991ffe20bdcc59f4f865286812094909455825482900481168452600080516020615e5a83398151915290945293909120805460ff19166001179055609c805463ffffffff191643841617905560a08054600160a01b60ff60a01b19909116179055549190910416907ffaf4b2054479d0f83e909b73cde2a6cb18ec2a93ba8ad5a62329001c86b1f3ea90613fa0908490615d8a565b60405180910390a2609854613fc390600160c01b900463ffffffff16600161538d565b609860186101000a81548163ffffffff021916908363ffffffff16021790555050505050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6060600080614049846141f1565b61ffff166001600160401b0381111561406457614064614471565b6040519080825280601f01601f19166020018201604052801561408e576020820181803683370190505b5090506000805b8251821080156140a6575061010081105b156140fd576001811b9350858416156140ed578060f81b8383815181106140cf576140cf615576565b60200101906001600160f81b031916908160001a9053508160010191505b6140f68161564e565b9050614095565b5090949350505050565b6065546001600160a01b031615801561412857506001600160a01b03821615155b6141aa5760405162461bcd60e51b815260206004820152604760248201527f5061757361626c652e5f696e697469616c697a655061757365723a205f696e6960448201527f7469616c697a6550617573657228292063616e206f6e6c792062652063616c6c6064820152666564206f6e636560c81b608482015260a401610883565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a26141ed826139f0565b5050565b6000805b821561421c57614206600184615e20565b909216918061421481615e37565b9150506141f5565b92915050565b82805461422e90615541565b90600052602060002090601f0160209004810192826142505760008555614296565b82601f106142695782800160ff19823516178555614296565b82800160010185558215614296579182015b8281111561429657823582559160200191906001019061427b565b506142a29291506142a6565b5090565b5b808211156142a257600081556001016142a7565b600060e082840312156142cd57600080fd5b50919050565b6000606082840312156142cd57600080fd5b600080608083850312156142f857600080fd5b82356001600160401b0381111561430e57600080fd5b61431a858286016142bb565b92505061432a84602085016142d3565b90509250929050565b6001600160a01b03811681146104f657600080fd5b60006020828403121561435a57600080fd5b813561141881614333565b60006020828403121561437757600080fd5b5035919050565b600281106104f657600080fd5b63ffffffff811681146104f657600080fd5b80356143a88161438b565b919050565b600080604083850312156143c057600080fd5b82356143cb8161437e565b915060208301356143db8161438b565b809150509250929050565b80151581146104f657600080fd5b60006020828403121561440657600080fd5b8135611418816143e6565b6000815180845260005b818110156144375760208185018101518683018201520161441b565b81811115614449576000602083870101525b50601f01601f19169290920160200192915050565b6020815260006114186020830184614411565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b03811182821017156144a9576144a9614471565b60405290565b60405161010081016001600160401b03811182821017156144a9576144a9614471565b604051601f8201601f191681016001600160401b03811182821017156144fa576144fa614471565b604052919050565b60008060006060848603121561451757600080fd5b833561452281614333565b92506020848101356001600160401b038082111561453f57600080fd5b818701915087601f83011261455357600080fd5b81358181111561456557614565614471565b614577601f8201601f191685016144d2565b9150808252888482850101111561458d57600080fd5b80848401858401376000848284010152508094505050506145b06040850161439d565b90509250925092565b600081518084526020808501808196508360051b810191508286016000805b8681101561464f578385038a52825180518087529087019087870190845b8181101561463a57835180516001600160a01b031684528a8101518b8501526040908101516001600160601b031690840152928901926060909201916001016145f6565b50509a87019a955050918501916001016145d8565b509298975050505050505050565b60208152600061141860208301846145b9565b60006020828403121561468257600080fd5b81356001600160401b0381111561469857600080fd5b82016040818503121561141857600080fd5b60008083601f8401126146bc57600080fd5b5081356001600160401b038111156146d357600080fd5b6020830191508360208285010111156146eb57600080fd5b9250929050565b6000806000806000806080878903121561470b57600080fd5b863561471681614333565b955060208701356147268161438b565b945060408701356001600160401b038082111561474257600080fd5b61474e8a838b016146aa565b9096509450606089013591508082111561476757600080fd5b818901915089601f83011261477b57600080fd5b81358181111561478a57600080fd5b8a60208260051b850101111561479f57600080fd5b6020830194508093505050509295509295509295565b600081518084526020808501945080840160005b838110156147eb57815163ffffffff16875295820195908201906001016147c9565b509495945050505050565b600081518084526020808501808196508360051b8101915082860160005b8581101561483e57828403895261482c8483516147b5565b98850198935090840190600101614814565b5091979650505050505050565b60208152600082516080602084015261486760a08401826147b5565b90506020840151601f198085840301604086015261488583836147b5565b925060408601519150808584030160608601526148a283836147b5565b92506060860151915080858403016080860152506148c082826147f6565b95945050505050565b60006001600160401b038211156148e2576148e2614471565b5060051b60200190565b600082601f8301126148fd57600080fd5b8135602061491261490d836148c9565b6144d2565b82815260059290921b8401810191818101908684111561493157600080fd5b8286015b848110156149555780356149488161438b565b8352918301918301614935565b509695505050505050565b60006040828403121561497257600080fd5b61497a614487565b9050813581526020820135602082015292915050565b600082601f8301126149a157600080fd5b813560206149b161490d836148c9565b82815260069290921b840181019181810190868411156149d057600080fd5b8286015b84811015614955576149e68882614960565b8352918301916040016149d4565b600082601f830112614a0557600080fd5b614a0d614487565b806040840185811115614a1f57600080fd5b845b81811015614a39578035845260209384019301614a21565b509095945050505050565b600060808284031215614a5657600080fd5b614a5e614487565b9050614a6a83836149f4565b8152614a7983604084016149f4565b602082015292915050565b600082601f830112614a9557600080fd5b81356020614aa561490d836148c9565b82815260059290921b84018101918181019086841115614ac457600080fd5b8286015b848110156149555780356001600160401b03811115614ae75760008081fd5b614af58986838b01016148ec565b845250918301918301614ac8565b60006101808284031215614b1657600080fd5b614b1e6144af565b905081356001600160401b0380821115614b3757600080fd5b614b43858386016148ec565b83526020840135915080821115614b5957600080fd5b614b6585838601614990565b60208401526040840135915080821115614b7e57600080fd5b614b8a85838601614990565b6040840152614b9c8560608601614a44565b6060840152614bae8560e08601614960565b6080840152610120840135915080821115614bc857600080fd5b614bd4858386016148ec565b60a0840152610140840135915080821115614bee57600080fd5b614bfa858386016148ec565b60c0840152610160840135915080821115614c1457600080fd5b50614c2184828501614a84565b60e08301525092915050565b600080600060a08486031215614c4257600080fd5b83356001600160401b0380821115614c5957600080fd5b614c65878388016142bb565b9450614c7487602088016142d3565b93506080860135915080821115614c8a57600080fd5b50614c9786828701614b03565b9150509250925092565b600060208284031215614cb357600080fd5b81356001600160401b03811115614cc957600080fd5b8201610120818503121561141857600080fd5b600060208284031215614cee57600080fd5b813560ff8116811461141857600080fd5b600080600060608486031215614d1457600080fd5b8335614d1f81614333565b92506020848101356001600160401b03811115614d3b57600080fd5b8501601f81018713614d4c57600080fd5b8035614d5a61490d826148c9565b81815260059190911b82018301908381019089831115614d7957600080fd5b928401925b82841015614d9757833582529284019290840190614d7e565b80965050505050506145b06040850161439d565b6020808252825182820181905260009190848201906040850190845b81811015614de357835183529284019291840191600101614dc7565b50909695505050505050565b600080600060408486031215614e0457600080fd5b8335614e0f8161438b565b925060208401356001600160401b03811115614e2a57600080fd5b614e36868287016146aa565b9497909650939450505050565b600080600080600060808688031215614e5b57600080fd5b8535945060208601356001600160401b0380821115614e7957600080fd5b614e8589838a016146aa565b909650945060408801359150614e9a8261438b565b90925060608701359080821115614eb057600080fd5b50614ebd88828901614b03565b9150509295509295909350565b600081518084526020808501945080840160005b838110156147eb5781516001600160601b031687529582019590820190600101614ede565b6040815260008351604080840152614f1e6080840182614eca565b90506020850151603f19848303016060850152614f3b8282614eca565b925050508260208301529392505050565b600060208284031215614f5e57600080fd5b81356114188161437e565b634e487b7160e01b600052602160045260246000fd5b6020810160058310614f9357614f93614f69565b91905290565b600080600060608486031215614fae57600080fd5b8335614fb981614333565b9250602084013591506040840135614fd08161438b565b809150509250925092565b828152604060208201526000614ff460408301846145b9565b949350505050565b60008060008060008060008060006101208a8c03121561501b57600080fd5b893561502681614333565b985060208a013561503681614333565b975060408a013561504681614333565b965060608a013561505681614333565b955060808a0135615066816143e6565b945060a08a013561507681614333565b935060c08a01356150868161438b565b925060e08a013561509681614333565b91506101008a01356150a781614333565b809150509295985092959850929598565b60008060008385036101408112156150cf57600080fd5b84356001600160401b03808211156150e657600080fd5b6150f2888389016142bb565b9550610100601f198401121561510757600080fd5b60208701945061012087013592508083111561512257600080fd5b5050614c9786828701614b03565b60006020828403121561514257600080fd5b81356114188161438b565b6000808335601e1984360301811261516457600080fd5b8301803591506001600160401b0382111561517e57600080fd5b6020019150368190038213156146eb57600080fd5b6020808252600f908201526e4e6f207461736b2070656e64696e6760881b604082015260600190565b6000808335601e198436030181126151d357600080fd5b83016020810192503590506001600160401b038111156151f257600080fd5b8036038313156146eb57600080fd5b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b602081526000823561523b8161438b565b63ffffffff811660208401525060208301356152568161438b565b63ffffffff811660408401525061526f6040840161439d565b63ffffffff811660608401525061528960608401846151bc565b60e0608085015261529f61010085018284615201565b9150506152ae6080850161439d565b63ffffffff811660a0850152506152c860a08501856151bc565b848303601f190160c08601526152df838284615201565b925050506152ef60c0850161439d565b63ffffffff811660e08501525b509392505050565b6020808252600d908201526c0a8c2e6d640dad2e6dac2e8c6d609b1b604082015260600190565b6020808252600e908201526d4e6f7420496e697420737461746560901b604082015260600190565b6020808252600a90820152690416c72647920526573760b41b604082015260600190565b634e487b7160e01b600052601160045260246000fd5b600063ffffffff8083168185168083038211156153ac576153ac615377565b01949350505050565b602080825260089082015267546f6f206c61746560c01b604082015260600190565b80356153e28161438b565b63ffffffff16825260208181013590830152604090810135910152565b63ffffffff815116825260208101516020830152600060408201516080604085015261542e6080850182614eca565b9050606083015184820360608601526148c08282614eca565b61545181846153d7565b608060608201526000614ff460808301846153ff565b6060810161421c82846153d7565b60006020828403121561548757600080fd5b815161141881614333565b6020808252602a908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526939903ab73830bab9b2b960b11b606082015260800190565b6000602082840312156154ee57600080fd5b8151611418816143e6565b60208082526028908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526739903830bab9b2b960c11b606082015260800190565b600181811c9082168061555557607f821691505b602082108114156142cd57634e487b7160e01b600052602260045260246000fd5b634e487b7160e01b600052603260045260246000fd5b6000602080838503121561559f57600080fd5b82516001600160401b038111156155b557600080fd5b8301601f810185136155c657600080fd5b80516155d461490d826148c9565b81815260059190911b820183019083810190878311156155f357600080fd5b928401925b82841015615611578351825292840192908401906155f8565b979650505050505050565b80516001600160601b03811681146143a857600080fd5b60006020828403121561564557600080fd5b6114188261561c565b600060001982141561566257615662615377565b5060010190565b63ffffffff84168152604060208201819052810182905260006001600160fb1b0383111561569657600080fd5b8260051b8085606085013760009201606001918252509392505050565b600060208083850312156156c657600080fd5b82516001600160401b038111156156dc57600080fd5b8301601f810185136156ed57600080fd5b80516156fb61490d826148c9565b81815260059190911b8201830190838101908783111561571a57600080fd5b928401925b828410156156115783516157328161438b565b8252928401929084019061571f565b63ffffffff841681526040602082015260006148c0604083018486615201565b60006020828403121561577357600080fd5b81516001600160c01b038116811461141857600080fd5b60006020828403121561579c57600080fd5b81516114188161438b565b600060ff821660ff8114156157be576157be615377565b60010192915050565b6040815260006157db604083018587615201565b905063ffffffff83166020830152949350505050565b602080825260059082015264041757468360dc1b604082015260600190565b600081518084526020808501945080840160005b838110156147eb5761584187835180518252602090810151910152565b6040969096019590820190600101615824565b8060005b6002811015615877578151845260209384019390910190600101615858565b50505050565b615888828251615854565b60208101516126ff6040840182615854565b8581526080602082015260006158b4608083018688615201565b63ffffffff85166040840152828103606084015261018084518183526158dc828401826147b5565b915050602085015182820360208401526158f68282615810565b915050604085015182820360408401526159108282615810565b9150506060850151615925606084018261587d565b506080850151805160e08401526020015161010083015260a085015182820361012084015261595482826147b5565b91505060c085015182820361014084015261596f82826147b5565b91505060e085015182820361016084015261598a82826147f6565b9a9950505050505050505050565b600082601f8301126159a957600080fd5b815160206159b961490d836148c9565b82815260059290921b840181019181810190868411156159d857600080fd5b8286015b84811015614955576159ed8161561c565b83529183019183016159dc565b60008060408385031215615a0d57600080fd5b82516001600160401b0380821115615a2457600080fd5b9084019060408287031215615a3857600080fd5b615a40614487565b825182811115615a4f57600080fd5b615a5b88828601615998565b825250602083015182811115615a7057600080fd5b615a7c88828601615998565b602083015250809450505050602083015190509250929050565b60006001600160601b0380831681851681830481118215151615615abc57615abc615377565b02949350505050565b6000816000190483118215151615615adf57615adf615377565b500290565b60006040820163ffffffff851683526020604081850152818551808452606086019150828701935060005b81811015615b2b57845183529383019391830191600101615b0f565b5090979650505050505050565b60028110615b4857615b48614f69565b9052565b60208152600063ffffffff8084511660208401526020840151615b726040850182615b38565b508060408501511660608401528060608501511660808401528060808501511660a084015260a084015160e060c0850152615bb1610100850182614411565b90508160c08601511660e0850152809250505092915050565b6020815260008235615bdb8161438b565b63ffffffff808216602085015260208501359150615bf88261437e565b615c056040850183615b38565b60408501359150615c158261438b565b808216606085015260608501359150615c2d8261438b565b80821660808501525050615c436080840161439d565b63ffffffff811660a084015250615c5d60a08401846151bc565b60e060c0850152615c7361010085018284615201565b9150506152ef60c0850161439d565b8035615c8d8161438b565b63ffffffff80821684526020830135602085015260408301359150615cb18261437e565b615cbe6040850183615b38565b60608301359150615cce8261438b565b1660608301526080818101359083015260a0808201359083015260c0808201359083015260e0810135615d0081614333565b6001600160a01b031660e0929092019190915250565b610100810161421c8284615c82565b6000610120615d348386615c82565b806101008401526148c0818401856153ff565b82815260608101611418602083018480518252602090810151910152565b600063ffffffff83811690831681811015615d8257615d82615377565b039392505050565b60208152600063ffffffff80845116602084015280602085015116604084015280604085015116606084015250606083015160e06080840152615dd1610100840182614411565b90506080840151615dea60a085018263ffffffff169052565b5060a0840151838203601f190160c0850152615e068282614411565b91505060c08401516152fc60e085018263ffffffff169052565b600082821015615e3257615e32615377565b500390565b600061ffff80831681811415615e4f57615e4f615377565b600101939250505056fe10afac9233b4ccc54d6404ffc1cf3b47515a2b8edbf675d15eddce05a027dcbd298c800d0881dd208d705ebc03eb18189f38118259f27dd43b4c60d61c607e87a264697066735822122096744ba35f1aeb7abea6230a0899025d4e79850452a051378efb05164f62606564736f6c634300080c0033",
}

// ContractFinalizerTaskManagerABI is the input ABI used to generate the binding from.
// Deprecated: Use ContractFinalizerTaskManagerMetaData.ABI instead.
var ContractFinalizerTaskManagerABI = ContractFinalizerTaskManagerMetaData.ABI

// ContractFinalizerTaskManagerBin is the compiled bytecode used for deploying new contracts.
// Deprecated: Use ContractFinalizerTaskManagerMetaData.Bin instead.
var ContractFinalizerTaskManagerBin = ContractFinalizerTaskManagerMetaData.Bin

// DeployContractFinalizerTaskManager deploys a new Ethereum contract, binding an instance of ContractFinalizerTaskManager to it.
func DeployContractFinalizerTaskManager(auth *bind.TransactOpts, backend bind.ContractBackend) (common.Address, *types.Transaction, *ContractFinalizerTaskManager, error) {
	parsed, err := ContractFinalizerTaskManagerMetaData.GetAbi()
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	if parsed == nil {
		return common.Address{}, nil, nil, errors.New("GetABI returned nil")
	}

	address, tx, contract, err := bind.DeployContract(auth, *parsed, common.FromHex(ContractFinalizerTaskManagerBin), backend)
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	return address, tx, &ContractFinalizerTaskManager{ContractFinalizerTaskManagerCaller: ContractFinalizerTaskManagerCaller{contract: contract}, ContractFinalizerTaskManagerTransactor: ContractFinalizerTaskManagerTransactor{contract: contract}, ContractFinalizerTaskManagerFilterer: ContractFinalizerTaskManagerFilterer{contract: contract}}, nil
}

// ContractFinalizerTaskManager is an auto generated Go binding around an Ethereum contract.
type ContractFinalizerTaskManager struct {
	ContractFinalizerTaskManagerCaller     // Read-only binding to the contract
	ContractFinalizerTaskManagerTransactor // Write-only binding to the contract
	ContractFinalizerTaskManagerFilterer   // Log filterer for contract events
}

// ContractFinalizerTaskManagerCaller is an auto generated read-only Go binding around an Ethereum contract.
type ContractFinalizerTaskManagerCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractFinalizerTaskManagerTransactor is an auto generated write-only Go binding around an Ethereum contract.
type ContractFinalizerTaskManagerTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractFinalizerTaskManagerFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type ContractFinalizerTaskManagerFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractFinalizerTaskManagerSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type ContractFinalizerTaskManagerSession struct {
	Contract     *ContractFinalizerTaskManager // Generic contract binding to set the session for
	CallOpts     bind.CallOpts                 // Call options to use throughout this session
	TransactOpts bind.TransactOpts             // Transaction auth options to use throughout this session
}

// ContractFinalizerTaskManagerCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type ContractFinalizerTaskManagerCallerSession struct {
	Contract *ContractFinalizerTaskManagerCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts                       // Call options to use throughout this session
}

// ContractFinalizerTaskManagerTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type ContractFinalizerTaskManagerTransactorSession struct {
	Contract     *ContractFinalizerTaskManagerTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts                       // Transaction auth options to use throughout this session
}

// ContractFinalizerTaskManagerRaw is an auto generated low-level Go binding around an Ethereum contract.
type ContractFinalizerTaskManagerRaw struct {
	Contract *ContractFinalizerTaskManager // Generic contract binding to access the raw methods on
}

// ContractFinalizerTaskManagerCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type ContractFinalizerTaskManagerCallerRaw struct {
	Contract *ContractFinalizerTaskManagerCaller // Generic read-only contract binding to access the raw methods on
}

// ContractFinalizerTaskManagerTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type ContractFinalizerTaskManagerTransactorRaw struct {
	Contract *ContractFinalizerTaskManagerTransactor // Generic write-only contract binding to access the raw methods on
}

// NewContractFinalizerTaskManager creates a new instance of ContractFinalizerTaskManager, bound to a specific deployed contract.
func NewContractFinalizerTaskManager(address common.Address, backend bind.ContractBackend) (*ContractFinalizerTaskManager, error) {
	contract, err := bindContractFinalizerTaskManager(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManager{ContractFinalizerTaskManagerCaller: ContractFinalizerTaskManagerCaller{contract: contract}, ContractFinalizerTaskManagerTransactor: ContractFinalizerTaskManagerTransactor{contract: contract}, ContractFinalizerTaskManagerFilterer: ContractFinalizerTaskManagerFilterer{contract: contract}}, nil
}

// NewContractFinalizerTaskManagerCaller creates a new read-only instance of ContractFinalizerTaskManager, bound to a specific deployed contract.
func NewContractFinalizerTaskManagerCaller(address common.Address, caller bind.ContractCaller) (*ContractFinalizerTaskManagerCaller, error) {
	contract, err := bindContractFinalizerTaskManager(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerCaller{contract: contract}, nil
}

// NewContractFinalizerTaskManagerTransactor creates a new write-only instance of ContractFinalizerTaskManager, bound to a specific deployed contract.
func NewContractFinalizerTaskManagerTransactor(address common.Address, transactor bind.ContractTransactor) (*ContractFinalizerTaskManagerTransactor, error) {
	contract, err := bindContractFinalizerTaskManager(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerTransactor{contract: contract}, nil
}

// NewContractFinalizerTaskManagerFilterer creates a new log filterer instance of ContractFinalizerTaskManager, bound to a specific deployed contract.
func NewContractFinalizerTaskManagerFilterer(address common.Address, filterer bind.ContractFilterer) (*ContractFinalizerTaskManagerFilterer, error) {
	contract, err := bindContractFinalizerTaskManager(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerFilterer{contract: contract}, nil
}

// bindContractFinalizerTaskManager binds a generic wrapper to an already deployed contract.
func bindContractFinalizerTaskManager(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := ContractFinalizerTaskManagerMetaData.GetAbi()
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, *parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _ContractFinalizerTaskManager.Contract.ContractFinalizerTaskManagerCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.ContractFinalizerTaskManagerTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.ContractFinalizerTaskManagerTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _ContractFinalizerTaskManager.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.contract.Transact(opts, method, params...)
}

// THRESHOLDDENOMINATOR is a free data retrieval call binding the contract method 0xef024458.
//
// Solidity: function THRESHOLD_DENOMINATOR() view returns(uint256)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) THRESHOLDDENOMINATOR(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "THRESHOLD_DENOMINATOR")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// THRESHOLDDENOMINATOR is a free data retrieval call binding the contract method 0xef024458.
//
// Solidity: function THRESHOLD_DENOMINATOR() view returns(uint256)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) THRESHOLDDENOMINATOR() (*big.Int, error) {
	return _ContractFinalizerTaskManager.Contract.THRESHOLDDENOMINATOR(&_ContractFinalizerTaskManager.CallOpts)
}

// THRESHOLDDENOMINATOR is a free data retrieval call binding the contract method 0xef024458.
//
// Solidity: function THRESHOLD_DENOMINATOR() view returns(uint256)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) THRESHOLDDENOMINATOR() (*big.Int, error) {
	return _ContractFinalizerTaskManager.Contract.THRESHOLDDENOMINATOR(&_ContractFinalizerTaskManager.CallOpts)
}

// Aggregator is a free data retrieval call binding the contract method 0x245a7bfc.
//
// Solidity: function aggregator() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) Aggregator(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "aggregator")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Aggregator is a free data retrieval call binding the contract method 0x245a7bfc.
//
// Solidity: function aggregator() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) Aggregator() (common.Address, error) {
	return _ContractFinalizerTaskManager.Contract.Aggregator(&_ContractFinalizerTaskManager.CallOpts)
}

// Aggregator is a free data retrieval call binding the contract method 0x245a7bfc.
//
// Solidity: function aggregator() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) Aggregator() (common.Address, error) {
	return _ContractFinalizerTaskManager.Contract.Aggregator(&_ContractFinalizerTaskManager.CallOpts)
}

// AllTaskHashes is a free data retrieval call binding the contract method 0x13f815ed.
//
// Solidity: function allTaskHashes(uint8 , uint32 ) view returns(bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) AllTaskHashes(opts *bind.CallOpts, arg0 uint8, arg1 uint32) ([32]byte, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "allTaskHashes", arg0, arg1)

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// AllTaskHashes is a free data retrieval call binding the contract method 0x13f815ed.
//
// Solidity: function allTaskHashes(uint8 , uint32 ) view returns(bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) AllTaskHashes(arg0 uint8, arg1 uint32) ([32]byte, error) {
	return _ContractFinalizerTaskManager.Contract.AllTaskHashes(&_ContractFinalizerTaskManager.CallOpts, arg0, arg1)
}

// AllTaskHashes is a free data retrieval call binding the contract method 0x13f815ed.
//
// Solidity: function allTaskHashes(uint8 , uint32 ) view returns(bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) AllTaskHashes(arg0 uint8, arg1 uint32) ([32]byte, error) {
	return _ContractFinalizerTaskManager.Contract.AllTaskHashes(&_ContractFinalizerTaskManager.CallOpts, arg0, arg1)
}

// AllTaskResponses is a free data retrieval call binding the contract method 0x1ac27297.
//
// Solidity: function allTaskResponses(uint8 , uint32 ) view returns(bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) AllTaskResponses(opts *bind.CallOpts, arg0 uint8, arg1 uint32) ([32]byte, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "allTaskResponses", arg0, arg1)

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// AllTaskResponses is a free data retrieval call binding the contract method 0x1ac27297.
//
// Solidity: function allTaskResponses(uint8 , uint32 ) view returns(bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) AllTaskResponses(arg0 uint8, arg1 uint32) ([32]byte, error) {
	return _ContractFinalizerTaskManager.Contract.AllTaskResponses(&_ContractFinalizerTaskManager.CallOpts, arg0, arg1)
}

// AllTaskResponses is a free data retrieval call binding the contract method 0x1ac27297.
//
// Solidity: function allTaskResponses(uint8 , uint32 ) view returns(bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) AllTaskResponses(arg0 uint8, arg1 uint32) ([32]byte, error) {
	return _ContractFinalizerTaskManager.Contract.AllTaskResponses(&_ContractFinalizerTaskManager.CallOpts, arg0, arg1)
}

// AllowNonRootInit is a free data retrieval call binding the contract method 0x0ee0fdbd.
//
// Solidity: function allowNonRootInit() view returns(bool)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) AllowNonRootInit(opts *bind.CallOpts) (bool, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "allowNonRootInit")

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// AllowNonRootInit is a free data retrieval call binding the contract method 0x0ee0fdbd.
//
// Solidity: function allowNonRootInit() view returns(bool)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) AllowNonRootInit() (bool, error) {
	return _ContractFinalizerTaskManager.Contract.AllowNonRootInit(&_ContractFinalizerTaskManager.CallOpts)
}

// AllowNonRootInit is a free data retrieval call binding the contract method 0x0ee0fdbd.
//
// Solidity: function allowNonRootInit() view returns(bool)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) AllowNonRootInit() (bool, error) {
	return _ContractFinalizerTaskManager.Contract.AllowNonRootInit(&_ContractFinalizerTaskManager.CallOpts)
}

// BlsApkRegistry is a free data retrieval call binding the contract method 0x5df45946.
//
// Solidity: function blsApkRegistry() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) BlsApkRegistry(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "blsApkRegistry")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// BlsApkRegistry is a free data retrieval call binding the contract method 0x5df45946.
//
// Solidity: function blsApkRegistry() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) BlsApkRegistry() (common.Address, error) {
	return _ContractFinalizerTaskManager.Contract.BlsApkRegistry(&_ContractFinalizerTaskManager.CallOpts)
}

// BlsApkRegistry is a free data retrieval call binding the contract method 0x5df45946.
//
// Solidity: function blsApkRegistry() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) BlsApkRegistry() (common.Address, error) {
	return _ContractFinalizerTaskManager.Contract.BlsApkRegistry(&_ContractFinalizerTaskManager.CallOpts)
}

// BlsSignatureChecker is a free data retrieval call binding the contract method 0x1c178e9c.
//
// Solidity: function blsSignatureChecker() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) BlsSignatureChecker(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "blsSignatureChecker")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// BlsSignatureChecker is a free data retrieval call binding the contract method 0x1c178e9c.
//
// Solidity: function blsSignatureChecker() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) BlsSignatureChecker() (common.Address, error) {
	return _ContractFinalizerTaskManager.Contract.BlsSignatureChecker(&_ContractFinalizerTaskManager.CallOpts)
}

// BlsSignatureChecker is a free data retrieval call binding the contract method 0x1c178e9c.
//
// Solidity: function blsSignatureChecker() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) BlsSignatureChecker() (common.Address, error) {
	return _ContractFinalizerTaskManager.Contract.BlsSignatureChecker(&_ContractFinalizerTaskManager.CallOpts)
}

// ChainRdBatchNonce is a free data retrieval call binding the contract method 0x930390d9.
//
// Solidity: function chainRdBatchNonce(uint8 ) view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) ChainRdBatchNonce(opts *bind.CallOpts, arg0 uint8) (uint32, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "chainRdBatchNonce", arg0)

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// ChainRdBatchNonce is a free data retrieval call binding the contract method 0x930390d9.
//
// Solidity: function chainRdBatchNonce(uint8 ) view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) ChainRdBatchNonce(arg0 uint8) (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.ChainRdBatchNonce(&_ContractFinalizerTaskManager.CallOpts, arg0)
}

// ChainRdBatchNonce is a free data retrieval call binding the contract method 0x930390d9.
//
// Solidity: function chainRdBatchNonce(uint8 ) view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) ChainRdBatchNonce(arg0 uint8) (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.ChainRdBatchNonce(&_ContractFinalizerTaskManager.CallOpts, arg0)
}

// CheckSignatures is a free data retrieval call binding the contract method 0x6efb4636.
//
// Solidity: function checkSignatures(bytes32 msgHash, bytes quorumNumbers, uint32 referenceBlockNumber, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) view returns((uint96[],uint96[]), bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) CheckSignatures(opts *bind.CallOpts, msgHash [32]byte, quorumNumbers []byte, referenceBlockNumber uint32, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (IBLSSignatureCheckerQuorumStakeTotals, [32]byte, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "checkSignatures", msgHash, quorumNumbers, referenceBlockNumber, nonSignerStakesAndSignature)

	if err != nil {
		return *new(IBLSSignatureCheckerQuorumStakeTotals), *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new(IBLSSignatureCheckerQuorumStakeTotals)).(*IBLSSignatureCheckerQuorumStakeTotals)
	out1 := *abi.ConvertType(out[1], new([32]byte)).(*[32]byte)

	return out0, out1, err

}

// CheckSignatures is a free data retrieval call binding the contract method 0x6efb4636.
//
// Solidity: function checkSignatures(bytes32 msgHash, bytes quorumNumbers, uint32 referenceBlockNumber, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) view returns((uint96[],uint96[]), bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) CheckSignatures(msgHash [32]byte, quorumNumbers []byte, referenceBlockNumber uint32, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (IBLSSignatureCheckerQuorumStakeTotals, [32]byte, error) {
	return _ContractFinalizerTaskManager.Contract.CheckSignatures(&_ContractFinalizerTaskManager.CallOpts, msgHash, quorumNumbers, referenceBlockNumber, nonSignerStakesAndSignature)
}

// CheckSignatures is a free data retrieval call binding the contract method 0x6efb4636.
//
// Solidity: function checkSignatures(bytes32 msgHash, bytes quorumNumbers, uint32 referenceBlockNumber, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) view returns((uint96[],uint96[]), bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) CheckSignatures(msgHash [32]byte, quorumNumbers []byte, referenceBlockNumber uint32, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (IBLSSignatureCheckerQuorumStakeTotals, [32]byte, error) {
	return _ContractFinalizerTaskManager.Contract.CheckSignatures(&_ContractFinalizerTaskManager.CallOpts, msgHash, quorumNumbers, referenceBlockNumber, nonSignerStakesAndSignature)
}

// Delegation is a free data retrieval call binding the contract method 0xdf5cf723.
//
// Solidity: function delegation() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) Delegation(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "delegation")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Delegation is a free data retrieval call binding the contract method 0xdf5cf723.
//
// Solidity: function delegation() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) Delegation() (common.Address, error) {
	return _ContractFinalizerTaskManager.Contract.Delegation(&_ContractFinalizerTaskManager.CallOpts)
}

// Delegation is a free data retrieval call binding the contract method 0xdf5cf723.
//
// Solidity: function delegation() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) Delegation() (common.Address, error) {
	return _ContractFinalizerTaskManager.Contract.Delegation(&_ContractFinalizerTaskManager.CallOpts)
}

// DummyForOperatorStateInfoType is a free data retrieval call binding the contract method 0x54d127de.
//
// Solidity: function dummyForOperatorStateInfoType((bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) _operatorStateInfo) view returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) DummyForOperatorStateInfoType(opts *bind.CallOpts, _operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo) error {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "dummyForOperatorStateInfoType", _operatorStateInfo)

	if err != nil {
		return err
	}

	return err

}

// DummyForOperatorStateInfoType is a free data retrieval call binding the contract method 0x54d127de.
//
// Solidity: function dummyForOperatorStateInfoType((bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) _operatorStateInfo) view returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) DummyForOperatorStateInfoType(_operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo) error {
	return _ContractFinalizerTaskManager.Contract.DummyForOperatorStateInfoType(&_ContractFinalizerTaskManager.CallOpts, _operatorStateInfo)
}

// DummyForOperatorStateInfoType is a free data retrieval call binding the contract method 0x54d127de.
//
// Solidity: function dummyForOperatorStateInfoType((bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) _operatorStateInfo) view returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) DummyForOperatorStateInfoType(_operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo) error {
	return _ContractFinalizerTaskManager.Contract.DummyForOperatorStateInfoType(&_ContractFinalizerTaskManager.CallOpts, _operatorStateInfo)
}

// DummyForQuorumStakeTotalsType is a free data retrieval call binding the contract method 0x45265b7a.
//
// Solidity: function dummyForQuorumStakeTotalsType((uint96[],uint96[]) _quorumStakeTotals) view returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) DummyForQuorumStakeTotalsType(opts *bind.CallOpts, _quorumStakeTotals IBLSSignatureCheckerQuorumStakeTotals) error {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "dummyForQuorumStakeTotalsType", _quorumStakeTotals)

	if err != nil {
		return err
	}

	return err

}

// DummyForQuorumStakeTotalsType is a free data retrieval call binding the contract method 0x45265b7a.
//
// Solidity: function dummyForQuorumStakeTotalsType((uint96[],uint96[]) _quorumStakeTotals) view returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) DummyForQuorumStakeTotalsType(_quorumStakeTotals IBLSSignatureCheckerQuorumStakeTotals) error {
	return _ContractFinalizerTaskManager.Contract.DummyForQuorumStakeTotalsType(&_ContractFinalizerTaskManager.CallOpts, _quorumStakeTotals)
}

// DummyForQuorumStakeTotalsType is a free data retrieval call binding the contract method 0x45265b7a.
//
// Solidity: function dummyForQuorumStakeTotalsType((uint96[],uint96[]) _quorumStakeTotals) view returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) DummyForQuorumStakeTotalsType(_quorumStakeTotals IBLSSignatureCheckerQuorumStakeTotals) error {
	return _ContractFinalizerTaskManager.Contract.DummyForQuorumStakeTotalsType(&_ContractFinalizerTaskManager.CallOpts, _quorumStakeTotals)
}

// Generator is a free data retrieval call binding the contract method 0x7afa1eed.
//
// Solidity: function generator() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) Generator(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "generator")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Generator is a free data retrieval call binding the contract method 0x7afa1eed.
//
// Solidity: function generator() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) Generator() (common.Address, error) {
	return _ContractFinalizerTaskManager.Contract.Generator(&_ContractFinalizerTaskManager.CallOpts)
}

// Generator is a free data retrieval call binding the contract method 0x7afa1eed.
//
// Solidity: function generator() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) Generator() (common.Address, error) {
	return _ContractFinalizerTaskManager.Contract.Generator(&_ContractFinalizerTaskManager.CallOpts)
}

// GetCheckSignaturesIndices is a free data retrieval call binding the contract method 0x4f739f74.
//
// Solidity: function getCheckSignaturesIndices(address registryCoordinator, uint32 referenceBlockNumber, bytes quorumNumbers, bytes32[] nonSignerOperatorIds) view returns((uint32[],uint32[],uint32[],uint32[][]))
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) GetCheckSignaturesIndices(opts *bind.CallOpts, registryCoordinator common.Address, referenceBlockNumber uint32, quorumNumbers []byte, nonSignerOperatorIds [][32]byte) (OperatorStateRetrieverCheckSignaturesIndices, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "getCheckSignaturesIndices", registryCoordinator, referenceBlockNumber, quorumNumbers, nonSignerOperatorIds)

	if err != nil {
		return *new(OperatorStateRetrieverCheckSignaturesIndices), err
	}

	out0 := *abi.ConvertType(out[0], new(OperatorStateRetrieverCheckSignaturesIndices)).(*OperatorStateRetrieverCheckSignaturesIndices)

	return out0, err

}

// GetCheckSignaturesIndices is a free data retrieval call binding the contract method 0x4f739f74.
//
// Solidity: function getCheckSignaturesIndices(address registryCoordinator, uint32 referenceBlockNumber, bytes quorumNumbers, bytes32[] nonSignerOperatorIds) view returns((uint32[],uint32[],uint32[],uint32[][]))
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) GetCheckSignaturesIndices(registryCoordinator common.Address, referenceBlockNumber uint32, quorumNumbers []byte, nonSignerOperatorIds [][32]byte) (OperatorStateRetrieverCheckSignaturesIndices, error) {
	return _ContractFinalizerTaskManager.Contract.GetCheckSignaturesIndices(&_ContractFinalizerTaskManager.CallOpts, registryCoordinator, referenceBlockNumber, quorumNumbers, nonSignerOperatorIds)
}

// GetCheckSignaturesIndices is a free data retrieval call binding the contract method 0x4f739f74.
//
// Solidity: function getCheckSignaturesIndices(address registryCoordinator, uint32 referenceBlockNumber, bytes quorumNumbers, bytes32[] nonSignerOperatorIds) view returns((uint32[],uint32[],uint32[],uint32[][]))
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) GetCheckSignaturesIndices(registryCoordinator common.Address, referenceBlockNumber uint32, quorumNumbers []byte, nonSignerOperatorIds [][32]byte) (OperatorStateRetrieverCheckSignaturesIndices, error) {
	return _ContractFinalizerTaskManager.Contract.GetCheckSignaturesIndices(&_ContractFinalizerTaskManager.CallOpts, registryCoordinator, referenceBlockNumber, quorumNumbers, nonSignerOperatorIds)
}

// GetOperatorState is a free data retrieval call binding the contract method 0x3563b0d1.
//
// Solidity: function getOperatorState(address registryCoordinator, bytes quorumNumbers, uint32 blockNumber) view returns((address,bytes32,uint96)[][])
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) GetOperatorState(opts *bind.CallOpts, registryCoordinator common.Address, quorumNumbers []byte, blockNumber uint32) ([][]OperatorStateRetrieverOperator, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "getOperatorState", registryCoordinator, quorumNumbers, blockNumber)

	if err != nil {
		return *new([][]OperatorStateRetrieverOperator), err
	}

	out0 := *abi.ConvertType(out[0], new([][]OperatorStateRetrieverOperator)).(*[][]OperatorStateRetrieverOperator)

	return out0, err

}

// GetOperatorState is a free data retrieval call binding the contract method 0x3563b0d1.
//
// Solidity: function getOperatorState(address registryCoordinator, bytes quorumNumbers, uint32 blockNumber) view returns((address,bytes32,uint96)[][])
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) GetOperatorState(registryCoordinator common.Address, quorumNumbers []byte, blockNumber uint32) ([][]OperatorStateRetrieverOperator, error) {
	return _ContractFinalizerTaskManager.Contract.GetOperatorState(&_ContractFinalizerTaskManager.CallOpts, registryCoordinator, quorumNumbers, blockNumber)
}

// GetOperatorState is a free data retrieval call binding the contract method 0x3563b0d1.
//
// Solidity: function getOperatorState(address registryCoordinator, bytes quorumNumbers, uint32 blockNumber) view returns((address,bytes32,uint96)[][])
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) GetOperatorState(registryCoordinator common.Address, quorumNumbers []byte, blockNumber uint32) ([][]OperatorStateRetrieverOperator, error) {
	return _ContractFinalizerTaskManager.Contract.GetOperatorState(&_ContractFinalizerTaskManager.CallOpts, registryCoordinator, quorumNumbers, blockNumber)
}

// GetOperatorState0 is a free data retrieval call binding the contract method 0xcefdc1d4.
//
// Solidity: function getOperatorState(address registryCoordinator, bytes32 operatorId, uint32 blockNumber) view returns(uint256, (address,bytes32,uint96)[][])
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) GetOperatorState0(opts *bind.CallOpts, registryCoordinator common.Address, operatorId [32]byte, blockNumber uint32) (*big.Int, [][]OperatorStateRetrieverOperator, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "getOperatorState0", registryCoordinator, operatorId, blockNumber)

	if err != nil {
		return *new(*big.Int), *new([][]OperatorStateRetrieverOperator), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)
	out1 := *abi.ConvertType(out[1], new([][]OperatorStateRetrieverOperator)).(*[][]OperatorStateRetrieverOperator)

	return out0, out1, err

}

// GetOperatorState0 is a free data retrieval call binding the contract method 0xcefdc1d4.
//
// Solidity: function getOperatorState(address registryCoordinator, bytes32 operatorId, uint32 blockNumber) view returns(uint256, (address,bytes32,uint96)[][])
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) GetOperatorState0(registryCoordinator common.Address, operatorId [32]byte, blockNumber uint32) (*big.Int, [][]OperatorStateRetrieverOperator, error) {
	return _ContractFinalizerTaskManager.Contract.GetOperatorState0(&_ContractFinalizerTaskManager.CallOpts, registryCoordinator, operatorId, blockNumber)
}

// GetOperatorState0 is a free data retrieval call binding the contract method 0xcefdc1d4.
//
// Solidity: function getOperatorState(address registryCoordinator, bytes32 operatorId, uint32 blockNumber) view returns(uint256, (address,bytes32,uint96)[][])
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) GetOperatorState0(registryCoordinator common.Address, operatorId [32]byte, blockNumber uint32) (*big.Int, [][]OperatorStateRetrieverOperator, error) {
	return _ContractFinalizerTaskManager.Contract.GetOperatorState0(&_ContractFinalizerTaskManager.CallOpts, registryCoordinator, operatorId, blockNumber)
}

// GetQuorumBitmapsAtBlockNumber is a free data retrieval call binding the contract method 0x5c155662.
//
// Solidity: function getQuorumBitmapsAtBlockNumber(address registryCoordinator, bytes32[] operatorIds, uint32 blockNumber) view returns(uint256[])
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) GetQuorumBitmapsAtBlockNumber(opts *bind.CallOpts, registryCoordinator common.Address, operatorIds [][32]byte, blockNumber uint32) ([]*big.Int, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "getQuorumBitmapsAtBlockNumber", registryCoordinator, operatorIds, blockNumber)

	if err != nil {
		return *new([]*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new([]*big.Int)).(*[]*big.Int)

	return out0, err

}

// GetQuorumBitmapsAtBlockNumber is a free data retrieval call binding the contract method 0x5c155662.
//
// Solidity: function getQuorumBitmapsAtBlockNumber(address registryCoordinator, bytes32[] operatorIds, uint32 blockNumber) view returns(uint256[])
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) GetQuorumBitmapsAtBlockNumber(registryCoordinator common.Address, operatorIds [][32]byte, blockNumber uint32) ([]*big.Int, error) {
	return _ContractFinalizerTaskManager.Contract.GetQuorumBitmapsAtBlockNumber(&_ContractFinalizerTaskManager.CallOpts, registryCoordinator, operatorIds, blockNumber)
}

// GetQuorumBitmapsAtBlockNumber is a free data retrieval call binding the contract method 0x5c155662.
//
// Solidity: function getQuorumBitmapsAtBlockNumber(address registryCoordinator, bytes32[] operatorIds, uint32 blockNumber) view returns(uint256[])
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) GetQuorumBitmapsAtBlockNumber(registryCoordinator common.Address, operatorIds [][32]byte, blockNumber uint32) ([]*big.Int, error) {
	return _ContractFinalizerTaskManager.Contract.GetQuorumBitmapsAtBlockNumber(&_ContractFinalizerTaskManager.CallOpts, registryCoordinator, operatorIds, blockNumber)
}

// IdToTaskStatus is a free data retrieval call binding the contract method 0xbf2315ed.
//
// Solidity: function idToTaskStatus(uint8 , uint32 ) view returns(uint8)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) IdToTaskStatus(opts *bind.CallOpts, arg0 uint8, arg1 uint32) (uint8, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "idToTaskStatus", arg0, arg1)

	if err != nil {
		return *new(uint8), err
	}

	out0 := *abi.ConvertType(out[0], new(uint8)).(*uint8)

	return out0, err

}

// IdToTaskStatus is a free data retrieval call binding the contract method 0xbf2315ed.
//
// Solidity: function idToTaskStatus(uint8 , uint32 ) view returns(uint8)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) IdToTaskStatus(arg0 uint8, arg1 uint32) (uint8, error) {
	return _ContractFinalizerTaskManager.Contract.IdToTaskStatus(&_ContractFinalizerTaskManager.CallOpts, arg0, arg1)
}

// IdToTaskStatus is a free data retrieval call binding the contract method 0xbf2315ed.
//
// Solidity: function idToTaskStatus(uint8 , uint32 ) view returns(uint8)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) IdToTaskStatus(arg0 uint8, arg1 uint32) (uint8, error) {
	return _ContractFinalizerTaskManager.Contract.IdToTaskStatus(&_ContractFinalizerTaskManager.CallOpts, arg0, arg1)
}

// IsTaskPending is a free data retrieval call binding the contract method 0x36f78ed8.
//
// Solidity: function isTaskPending() view returns(bool)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) IsTaskPending(opts *bind.CallOpts) (bool, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "isTaskPending")

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// IsTaskPending is a free data retrieval call binding the contract method 0x36f78ed8.
//
// Solidity: function isTaskPending() view returns(bool)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) IsTaskPending() (bool, error) {
	return _ContractFinalizerTaskManager.Contract.IsTaskPending(&_ContractFinalizerTaskManager.CallOpts)
}

// IsTaskPending is a free data retrieval call binding the contract method 0x36f78ed8.
//
// Solidity: function isTaskPending() view returns(bool)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) IsTaskPending() (bool, error) {
	return _ContractFinalizerTaskManager.Contract.IsTaskPending(&_ContractFinalizerTaskManager.CallOpts)
}

// LastCompletedOpTaskCreatedBlock is a free data retrieval call binding the contract method 0x537a2929.
//
// Solidity: function lastCompletedOpTaskCreatedBlock() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) LastCompletedOpTaskCreatedBlock(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "lastCompletedOpTaskCreatedBlock")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// LastCompletedOpTaskCreatedBlock is a free data retrieval call binding the contract method 0x537a2929.
//
// Solidity: function lastCompletedOpTaskCreatedBlock() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) LastCompletedOpTaskCreatedBlock() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.LastCompletedOpTaskCreatedBlock(&_ContractFinalizerTaskManager.CallOpts)
}

// LastCompletedOpTaskCreatedBlock is a free data retrieval call binding the contract method 0x537a2929.
//
// Solidity: function lastCompletedOpTaskCreatedBlock() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) LastCompletedOpTaskCreatedBlock() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.LastCompletedOpTaskCreatedBlock(&_ContractFinalizerTaskManager.CallOpts)
}

// LastCompletedOpTaskNum is a free data retrieval call binding the contract method 0x8fc8729a.
//
// Solidity: function lastCompletedOpTaskNum() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) LastCompletedOpTaskNum(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "lastCompletedOpTaskNum")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// LastCompletedOpTaskNum is a free data retrieval call binding the contract method 0x8fc8729a.
//
// Solidity: function lastCompletedOpTaskNum() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) LastCompletedOpTaskNum() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.LastCompletedOpTaskNum(&_ContractFinalizerTaskManager.CallOpts)
}

// LastCompletedOpTaskNum is a free data retrieval call binding the contract method 0x8fc8729a.
//
// Solidity: function lastCompletedOpTaskNum() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) LastCompletedOpTaskNum() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.LastCompletedOpTaskNum(&_ContractFinalizerTaskManager.CallOpts)
}

// LastCompletedOpTaskQuorumNumbers is a free data retrieval call binding the contract method 0x2830e8f9.
//
// Solidity: function lastCompletedOpTaskQuorumNumbers() view returns(bytes)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) LastCompletedOpTaskQuorumNumbers(opts *bind.CallOpts) ([]byte, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "lastCompletedOpTaskQuorumNumbers")

	if err != nil {
		return *new([]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([]byte)).(*[]byte)

	return out0, err

}

// LastCompletedOpTaskQuorumNumbers is a free data retrieval call binding the contract method 0x2830e8f9.
//
// Solidity: function lastCompletedOpTaskQuorumNumbers() view returns(bytes)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) LastCompletedOpTaskQuorumNumbers() ([]byte, error) {
	return _ContractFinalizerTaskManager.Contract.LastCompletedOpTaskQuorumNumbers(&_ContractFinalizerTaskManager.CallOpts)
}

// LastCompletedOpTaskQuorumNumbers is a free data retrieval call binding the contract method 0x2830e8f9.
//
// Solidity: function lastCompletedOpTaskQuorumNumbers() view returns(bytes)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) LastCompletedOpTaskQuorumNumbers() ([]byte, error) {
	return _ContractFinalizerTaskManager.Contract.LastCompletedOpTaskQuorumNumbers(&_ContractFinalizerTaskManager.CallOpts)
}

// LastCompletedOpTaskQuorumThresholdPercentage is a free data retrieval call binding the contract method 0xe70c2623.
//
// Solidity: function lastCompletedOpTaskQuorumThresholdPercentage() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) LastCompletedOpTaskQuorumThresholdPercentage(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "lastCompletedOpTaskQuorumThresholdPercentage")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// LastCompletedOpTaskQuorumThresholdPercentage is a free data retrieval call binding the contract method 0xe70c2623.
//
// Solidity: function lastCompletedOpTaskQuorumThresholdPercentage() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) LastCompletedOpTaskQuorumThresholdPercentage() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.LastCompletedOpTaskQuorumThresholdPercentage(&_ContractFinalizerTaskManager.CallOpts)
}

// LastCompletedOpTaskQuorumThresholdPercentage is a free data retrieval call binding the contract method 0xe70c2623.
//
// Solidity: function lastCompletedOpTaskQuorumThresholdPercentage() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) LastCompletedOpTaskQuorumThresholdPercentage() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.LastCompletedOpTaskQuorumThresholdPercentage(&_ContractFinalizerTaskManager.CallOpts)
}

// LastOpTaskCreatedBlock is a free data retrieval call binding the contract method 0x4d7a7116.
//
// Solidity: function lastOpTaskCreatedBlock() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) LastOpTaskCreatedBlock(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "lastOpTaskCreatedBlock")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// LastOpTaskCreatedBlock is a free data retrieval call binding the contract method 0x4d7a7116.
//
// Solidity: function lastOpTaskCreatedBlock() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) LastOpTaskCreatedBlock() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.LastOpTaskCreatedBlock(&_ContractFinalizerTaskManager.CallOpts)
}

// LastOpTaskCreatedBlock is a free data retrieval call binding the contract method 0x4d7a7116.
//
// Solidity: function lastOpTaskCreatedBlock() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) LastOpTaskCreatedBlock() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.LastOpTaskCreatedBlock(&_ContractFinalizerTaskManager.CallOpts)
}

// LatestOpTaskNum is a free data retrieval call binding the contract method 0x41789d57.
//
// Solidity: function latestOpTaskNum() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) LatestOpTaskNum(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "latestOpTaskNum")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// LatestOpTaskNum is a free data retrieval call binding the contract method 0x41789d57.
//
// Solidity: function latestOpTaskNum() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) LatestOpTaskNum() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.LatestOpTaskNum(&_ContractFinalizerTaskManager.CallOpts)
}

// LatestOpTaskNum is a free data retrieval call binding the contract method 0x41789d57.
//
// Solidity: function latestOpTaskNum() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) LatestOpTaskNum() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.LatestOpTaskNum(&_ContractFinalizerTaskManager.CallOpts)
}

// LatestRdTaskNum is a free data retrieval call binding the contract method 0x7afdd54b.
//
// Solidity: function latestRdTaskNum() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) LatestRdTaskNum(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "latestRdTaskNum")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// LatestRdTaskNum is a free data retrieval call binding the contract method 0x7afdd54b.
//
// Solidity: function latestRdTaskNum() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) LatestRdTaskNum() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.LatestRdTaskNum(&_ContractFinalizerTaskManager.CallOpts)
}

// LatestRdTaskNum is a free data retrieval call binding the contract method 0x7afdd54b.
//
// Solidity: function latestRdTaskNum() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) LatestRdTaskNum() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.LatestRdTaskNum(&_ContractFinalizerTaskManager.CallOpts)
}

// OperatorStateRetrieverExtended is a free data retrieval call binding the contract method 0x8380acbd.
//
// Solidity: function operatorStateRetrieverExtended() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) OperatorStateRetrieverExtended(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "operatorStateRetrieverExtended")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// OperatorStateRetrieverExtended is a free data retrieval call binding the contract method 0x8380acbd.
//
// Solidity: function operatorStateRetrieverExtended() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) OperatorStateRetrieverExtended() (common.Address, error) {
	return _ContractFinalizerTaskManager.Contract.OperatorStateRetrieverExtended(&_ContractFinalizerTaskManager.CallOpts)
}

// OperatorStateRetrieverExtended is a free data retrieval call binding the contract method 0x8380acbd.
//
// Solidity: function operatorStateRetrieverExtended() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) OperatorStateRetrieverExtended() (common.Address, error) {
	return _ContractFinalizerTaskManager.Contract.OperatorStateRetrieverExtended(&_ContractFinalizerTaskManager.CallOpts)
}

// OperatorsStateInfoHash is a free data retrieval call binding the contract method 0xadfcb048.
//
// Solidity: function operatorsStateInfoHash() view returns(bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) OperatorsStateInfoHash(opts *bind.CallOpts) ([32]byte, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "operatorsStateInfoHash")

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// OperatorsStateInfoHash is a free data retrieval call binding the contract method 0xadfcb048.
//
// Solidity: function operatorsStateInfoHash() view returns(bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) OperatorsStateInfoHash() ([32]byte, error) {
	return _ContractFinalizerTaskManager.Contract.OperatorsStateInfoHash(&_ContractFinalizerTaskManager.CallOpts)
}

// OperatorsStateInfoHash is a free data retrieval call binding the contract method 0xadfcb048.
//
// Solidity: function operatorsStateInfoHash() view returns(bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) OperatorsStateInfoHash() ([32]byte, error) {
	return _ContractFinalizerTaskManager.Contract.OperatorsStateInfoHash(&_ContractFinalizerTaskManager.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) Owner(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "owner")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) Owner() (common.Address, error) {
	return _ContractFinalizerTaskManager.Contract.Owner(&_ContractFinalizerTaskManager.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) Owner() (common.Address, error) {
	return _ContractFinalizerTaskManager.Contract.Owner(&_ContractFinalizerTaskManager.CallOpts)
}

// Paused is a free data retrieval call binding the contract method 0x5ac86ab7.
//
// Solidity: function paused(uint8 index) view returns(bool)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) Paused(opts *bind.CallOpts, index uint8) (bool, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "paused", index)

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// Paused is a free data retrieval call binding the contract method 0x5ac86ab7.
//
// Solidity: function paused(uint8 index) view returns(bool)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) Paused(index uint8) (bool, error) {
	return _ContractFinalizerTaskManager.Contract.Paused(&_ContractFinalizerTaskManager.CallOpts, index)
}

// Paused is a free data retrieval call binding the contract method 0x5ac86ab7.
//
// Solidity: function paused(uint8 index) view returns(bool)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) Paused(index uint8) (bool, error) {
	return _ContractFinalizerTaskManager.Contract.Paused(&_ContractFinalizerTaskManager.CallOpts, index)
}

// Paused0 is a free data retrieval call binding the contract method 0x5c975abb.
//
// Solidity: function paused() view returns(uint256)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) Paused0(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "paused0")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// Paused0 is a free data retrieval call binding the contract method 0x5c975abb.
//
// Solidity: function paused() view returns(uint256)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) Paused0() (*big.Int, error) {
	return _ContractFinalizerTaskManager.Contract.Paused0(&_ContractFinalizerTaskManager.CallOpts)
}

// Paused0 is a free data retrieval call binding the contract method 0x5c975abb.
//
// Solidity: function paused() view returns(uint256)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) Paused0() (*big.Int, error) {
	return _ContractFinalizerTaskManager.Contract.Paused0(&_ContractFinalizerTaskManager.CallOpts)
}

// PauserRegistry is a free data retrieval call binding the contract method 0x886f1195.
//
// Solidity: function pauserRegistry() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) PauserRegistry(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "pauserRegistry")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// PauserRegistry is a free data retrieval call binding the contract method 0x886f1195.
//
// Solidity: function pauserRegistry() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) PauserRegistry() (common.Address, error) {
	return _ContractFinalizerTaskManager.Contract.PauserRegistry(&_ContractFinalizerTaskManager.CallOpts)
}

// PauserRegistry is a free data retrieval call binding the contract method 0x886f1195.
//
// Solidity: function pauserRegistry() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) PauserRegistry() (common.Address, error) {
	return _ContractFinalizerTaskManager.Contract.PauserRegistry(&_ContractFinalizerTaskManager.CallOpts)
}

// RegistryCoordinator is a free data retrieval call binding the contract method 0x6d14a987.
//
// Solidity: function registryCoordinator() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) RegistryCoordinator(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "registryCoordinator")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// RegistryCoordinator is a free data retrieval call binding the contract method 0x6d14a987.
//
// Solidity: function registryCoordinator() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) RegistryCoordinator() (common.Address, error) {
	return _ContractFinalizerTaskManager.Contract.RegistryCoordinator(&_ContractFinalizerTaskManager.CallOpts)
}

// RegistryCoordinator is a free data retrieval call binding the contract method 0x6d14a987.
//
// Solidity: function registryCoordinator() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) RegistryCoordinator() (common.Address, error) {
	return _ContractFinalizerTaskManager.Contract.RegistryCoordinator(&_ContractFinalizerTaskManager.CallOpts)
}

// Rolldown is a free data retrieval call binding the contract method 0x3d9fb00c.
//
// Solidity: function rolldown() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) Rolldown(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "rolldown")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Rolldown is a free data retrieval call binding the contract method 0x3d9fb00c.
//
// Solidity: function rolldown() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) Rolldown() (common.Address, error) {
	return _ContractFinalizerTaskManager.Contract.Rolldown(&_ContractFinalizerTaskManager.CallOpts)
}

// Rolldown is a free data retrieval call binding the contract method 0x3d9fb00c.
//
// Solidity: function rolldown() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) Rolldown() (common.Address, error) {
	return _ContractFinalizerTaskManager.Contract.Rolldown(&_ContractFinalizerTaskManager.CallOpts)
}

// StakeRegistry is a free data retrieval call binding the contract method 0x68304835.
//
// Solidity: function stakeRegistry() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) StakeRegistry(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "stakeRegistry")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// StakeRegistry is a free data retrieval call binding the contract method 0x68304835.
//
// Solidity: function stakeRegistry() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) StakeRegistry() (common.Address, error) {
	return _ContractFinalizerTaskManager.Contract.StakeRegistry(&_ContractFinalizerTaskManager.CallOpts)
}

// StakeRegistry is a free data retrieval call binding the contract method 0x68304835.
//
// Solidity: function stakeRegistry() view returns(address)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) StakeRegistry() (common.Address, error) {
	return _ContractFinalizerTaskManager.Contract.StakeRegistry(&_ContractFinalizerTaskManager.CallOpts)
}

// TaskResponseWindowBlock is a free data retrieval call binding the contract method 0xa69563a9.
//
// Solidity: function taskResponseWindowBlock() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) TaskResponseWindowBlock(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "taskResponseWindowBlock")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// TaskResponseWindowBlock is a free data retrieval call binding the contract method 0xa69563a9.
//
// Solidity: function taskResponseWindowBlock() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) TaskResponseWindowBlock() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.TaskResponseWindowBlock(&_ContractFinalizerTaskManager.CallOpts)
}

// TaskResponseWindowBlock is a free data retrieval call binding the contract method 0xa69563a9.
//
// Solidity: function taskResponseWindowBlock() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) TaskResponseWindowBlock() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.TaskResponseWindowBlock(&_ContractFinalizerTaskManager.CallOpts)
}

// CreateNewOpTask is a paid mutator transaction binding the contract method 0x6e125ff4.
//
// Solidity: function createNewOpTask(uint32 quorumThresholdPercentage, bytes quorumNumbers) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) CreateNewOpTask(opts *bind.TransactOpts, quorumThresholdPercentage uint32, quorumNumbers []byte) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "createNewOpTask", quorumThresholdPercentage, quorumNumbers)
}

// CreateNewOpTask is a paid mutator transaction binding the contract method 0x6e125ff4.
//
// Solidity: function createNewOpTask(uint32 quorumThresholdPercentage, bytes quorumNumbers) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) CreateNewOpTask(quorumThresholdPercentage uint32, quorumNumbers []byte) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.CreateNewOpTask(&_ContractFinalizerTaskManager.TransactOpts, quorumThresholdPercentage, quorumNumbers)
}

// CreateNewOpTask is a paid mutator transaction binding the contract method 0x6e125ff4.
//
// Solidity: function createNewOpTask(uint32 quorumThresholdPercentage, bytes quorumNumbers) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) CreateNewOpTask(quorumThresholdPercentage uint32, quorumNumbers []byte) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.CreateNewOpTask(&_ContractFinalizerTaskManager.TransactOpts, quorumThresholdPercentage, quorumNumbers)
}

// CreateNewRdTask is a paid mutator transaction binding the contract method 0x6f254819.
//
// Solidity: function createNewRdTask(uint8 chainId, uint32 batchId) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) CreateNewRdTask(opts *bind.TransactOpts, chainId uint8, batchId uint32) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "createNewRdTask", chainId, batchId)
}

// CreateNewRdTask is a paid mutator transaction binding the contract method 0x6f254819.
//
// Solidity: function createNewRdTask(uint8 chainId, uint32 batchId) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) CreateNewRdTask(chainId uint8, batchId uint32) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.CreateNewRdTask(&_ContractFinalizerTaskManager.TransactOpts, chainId, batchId)
}

// CreateNewRdTask is a paid mutator transaction binding the contract method 0x6f254819.
//
// Solidity: function createNewRdTask(uint8 chainId, uint32 batchId) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) CreateNewRdTask(chainId uint8, batchId uint32) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.CreateNewRdTask(&_ContractFinalizerTaskManager.TransactOpts, chainId, batchId)
}

// ForceCancelPendingTasks is a paid mutator transaction binding the contract method 0x60202fc0.
//
// Solidity: function forceCancelPendingTasks() returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) ForceCancelPendingTasks(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "forceCancelPendingTasks")
}

// ForceCancelPendingTasks is a paid mutator transaction binding the contract method 0x60202fc0.
//
// Solidity: function forceCancelPendingTasks() returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) ForceCancelPendingTasks() (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.ForceCancelPendingTasks(&_ContractFinalizerTaskManager.TransactOpts)
}

// ForceCancelPendingTasks is a paid mutator transaction binding the contract method 0x60202fc0.
//
// Solidity: function forceCancelPendingTasks() returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) ForceCancelPendingTasks() (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.ForceCancelPendingTasks(&_ContractFinalizerTaskManager.TransactOpts)
}

// ForceCreateNewOpTask is a paid mutator transaction binding the contract method 0xf5640cf8.
//
// Solidity: function forceCreateNewOpTask(uint32 quorumThresholdPercentage, bytes quorumNumbers) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) ForceCreateNewOpTask(opts *bind.TransactOpts, quorumThresholdPercentage uint32, quorumNumbers []byte) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "forceCreateNewOpTask", quorumThresholdPercentage, quorumNumbers)
}

// ForceCreateNewOpTask is a paid mutator transaction binding the contract method 0xf5640cf8.
//
// Solidity: function forceCreateNewOpTask(uint32 quorumThresholdPercentage, bytes quorumNumbers) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) ForceCreateNewOpTask(quorumThresholdPercentage uint32, quorumNumbers []byte) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.ForceCreateNewOpTask(&_ContractFinalizerTaskManager.TransactOpts, quorumThresholdPercentage, quorumNumbers)
}

// ForceCreateNewOpTask is a paid mutator transaction binding the contract method 0xf5640cf8.
//
// Solidity: function forceCreateNewOpTask(uint32 quorumThresholdPercentage, bytes quorumNumbers) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) ForceCreateNewOpTask(quorumThresholdPercentage uint32, quorumNumbers []byte) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.ForceCreateNewOpTask(&_ContractFinalizerTaskManager.TransactOpts, quorumThresholdPercentage, quorumNumbers)
}

// ForceRespondToOpTask is a paid mutator transaction binding the contract method 0x01a3f013.
//
// Solidity: function forceRespondToOpTask((uint32,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32) taskResponse) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) ForceRespondToOpTask(opts *bind.TransactOpts, task IFinalizerTaskManagerOpTask, taskResponse IFinalizerTaskManagerOpTaskResponse) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "forceRespondToOpTask", task, taskResponse)
}

// ForceRespondToOpTask is a paid mutator transaction binding the contract method 0x01a3f013.
//
// Solidity: function forceRespondToOpTask((uint32,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32) taskResponse) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) ForceRespondToOpTask(task IFinalizerTaskManagerOpTask, taskResponse IFinalizerTaskManagerOpTaskResponse) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.ForceRespondToOpTask(&_ContractFinalizerTaskManager.TransactOpts, task, taskResponse)
}

// ForceRespondToOpTask is a paid mutator transaction binding the contract method 0x01a3f013.
//
// Solidity: function forceRespondToOpTask((uint32,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32) taskResponse) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) ForceRespondToOpTask(task IFinalizerTaskManagerOpTask, taskResponse IFinalizerTaskManagerOpTaskResponse) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.ForceRespondToOpTask(&_ContractFinalizerTaskManager.TransactOpts, task, taskResponse)
}

// Initialize is a paid mutator transaction binding the contract method 0xde434838.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _aggregator, address _generator, bool _allowNonRootInit, address _blsSignatureCheckerAddress, uint32 _taskResponseWindowBlock, address _operatorStateRetrieverExtended, address _rolldown) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) Initialize(opts *bind.TransactOpts, _pauserRegistry common.Address, initialOwner common.Address, _aggregator common.Address, _generator common.Address, _allowNonRootInit bool, _blsSignatureCheckerAddress common.Address, _taskResponseWindowBlock uint32, _operatorStateRetrieverExtended common.Address, _rolldown common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "initialize", _pauserRegistry, initialOwner, _aggregator, _generator, _allowNonRootInit, _blsSignatureCheckerAddress, _taskResponseWindowBlock, _operatorStateRetrieverExtended, _rolldown)
}

// Initialize is a paid mutator transaction binding the contract method 0xde434838.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _aggregator, address _generator, bool _allowNonRootInit, address _blsSignatureCheckerAddress, uint32 _taskResponseWindowBlock, address _operatorStateRetrieverExtended, address _rolldown) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) Initialize(_pauserRegistry common.Address, initialOwner common.Address, _aggregator common.Address, _generator common.Address, _allowNonRootInit bool, _blsSignatureCheckerAddress common.Address, _taskResponseWindowBlock uint32, _operatorStateRetrieverExtended common.Address, _rolldown common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.Initialize(&_ContractFinalizerTaskManager.TransactOpts, _pauserRegistry, initialOwner, _aggregator, _generator, _allowNonRootInit, _blsSignatureCheckerAddress, _taskResponseWindowBlock, _operatorStateRetrieverExtended, _rolldown)
}

// Initialize is a paid mutator transaction binding the contract method 0xde434838.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _aggregator, address _generator, bool _allowNonRootInit, address _blsSignatureCheckerAddress, uint32 _taskResponseWindowBlock, address _operatorStateRetrieverExtended, address _rolldown) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) Initialize(_pauserRegistry common.Address, initialOwner common.Address, _aggregator common.Address, _generator common.Address, _allowNonRootInit bool, _blsSignatureCheckerAddress common.Address, _taskResponseWindowBlock uint32, _operatorStateRetrieverExtended common.Address, _rolldown common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.Initialize(&_ContractFinalizerTaskManager.TransactOpts, _pauserRegistry, initialOwner, _aggregator, _generator, _allowNonRootInit, _blsSignatureCheckerAddress, _taskResponseWindowBlock, _operatorStateRetrieverExtended, _rolldown)
}

// Pause is a paid mutator transaction binding the contract method 0x136439dd.
//
// Solidity: function pause(uint256 newPausedStatus) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) Pause(opts *bind.TransactOpts, newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "pause", newPausedStatus)
}

// Pause is a paid mutator transaction binding the contract method 0x136439dd.
//
// Solidity: function pause(uint256 newPausedStatus) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) Pause(newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.Pause(&_ContractFinalizerTaskManager.TransactOpts, newPausedStatus)
}

// Pause is a paid mutator transaction binding the contract method 0x136439dd.
//
// Solidity: function pause(uint256 newPausedStatus) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) Pause(newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.Pause(&_ContractFinalizerTaskManager.TransactOpts, newPausedStatus)
}

// PauseAll is a paid mutator transaction binding the contract method 0x595c6a67.
//
// Solidity: function pauseAll() returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) PauseAll(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "pauseAll")
}

// PauseAll is a paid mutator transaction binding the contract method 0x595c6a67.
//
// Solidity: function pauseAll() returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) PauseAll() (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.PauseAll(&_ContractFinalizerTaskManager.TransactOpts)
}

// PauseAll is a paid mutator transaction binding the contract method 0x595c6a67.
//
// Solidity: function pauseAll() returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) PauseAll() (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.PauseAll(&_ContractFinalizerTaskManager.TransactOpts)
}

// PauseTrackingOpState is a paid mutator transaction binding the contract method 0x79badf73.
//
// Solidity: function pauseTrackingOpState() returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) PauseTrackingOpState(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "pauseTrackingOpState")
}

// PauseTrackingOpState is a paid mutator transaction binding the contract method 0x79badf73.
//
// Solidity: function pauseTrackingOpState() returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) PauseTrackingOpState() (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.PauseTrackingOpState(&_ContractFinalizerTaskManager.TransactOpts)
}

// PauseTrackingOpState is a paid mutator transaction binding the contract method 0x79badf73.
//
// Solidity: function pauseTrackingOpState() returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) PauseTrackingOpState() (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.PauseTrackingOpState(&_ContractFinalizerTaskManager.TransactOpts)
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) RenounceOwnership(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "renounceOwnership")
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) RenounceOwnership() (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.RenounceOwnership(&_ContractFinalizerTaskManager.TransactOpts)
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) RenounceOwnership() (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.RenounceOwnership(&_ContractFinalizerTaskManager.TransactOpts)
}

// RespondToOpTask is a paid mutator transaction binding the contract method 0x516a7227.
//
// Solidity: function respondToOpTask((uint32,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) RespondToOpTask(opts *bind.TransactOpts, task IFinalizerTaskManagerOpTask, taskResponse IFinalizerTaskManagerOpTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "respondToOpTask", task, taskResponse, nonSignerStakesAndSignature)
}

// RespondToOpTask is a paid mutator transaction binding the contract method 0x516a7227.
//
// Solidity: function respondToOpTask((uint32,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) RespondToOpTask(task IFinalizerTaskManagerOpTask, taskResponse IFinalizerTaskManagerOpTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.RespondToOpTask(&_ContractFinalizerTaskManager.TransactOpts, task, taskResponse, nonSignerStakesAndSignature)
}

// RespondToOpTask is a paid mutator transaction binding the contract method 0x516a7227.
//
// Solidity: function respondToOpTask((uint32,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) RespondToOpTask(task IFinalizerTaskManagerOpTask, taskResponse IFinalizerTaskManagerOpTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.RespondToOpTask(&_ContractFinalizerTaskManager.TransactOpts, task, taskResponse, nonSignerStakesAndSignature)
}

// RespondToRdTask is a paid mutator transaction binding the contract method 0xe72ddf10.
//
// Solidity: function respondToRdTask((uint32,uint8,uint32,uint32,uint32,bytes,uint32) task, (uint32,bytes32,uint8,uint32,bytes32,uint256,uint256,address) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) RespondToRdTask(opts *bind.TransactOpts, task IFinalizerTaskManagerRdTask, taskResponse IFinalizerTaskManagerRdTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "respondToRdTask", task, taskResponse, nonSignerStakesAndSignature)
}

// RespondToRdTask is a paid mutator transaction binding the contract method 0xe72ddf10.
//
// Solidity: function respondToRdTask((uint32,uint8,uint32,uint32,uint32,bytes,uint32) task, (uint32,bytes32,uint8,uint32,bytes32,uint256,uint256,address) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) RespondToRdTask(task IFinalizerTaskManagerRdTask, taskResponse IFinalizerTaskManagerRdTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.RespondToRdTask(&_ContractFinalizerTaskManager.TransactOpts, task, taskResponse, nonSignerStakesAndSignature)
}

// RespondToRdTask is a paid mutator transaction binding the contract method 0xe72ddf10.
//
// Solidity: function respondToRdTask((uint32,uint8,uint32,uint32,uint32,bytes,uint32) task, (uint32,bytes32,uint8,uint32,bytes32,uint256,uint256,address) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) RespondToRdTask(task IFinalizerTaskManagerRdTask, taskResponse IFinalizerTaskManagerRdTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.RespondToRdTask(&_ContractFinalizerTaskManager.TransactOpts, task, taskResponse, nonSignerStakesAndSignature)
}

// ResumeTrackingQuorums is a paid mutator transaction binding the contract method 0x191aac7a.
//
// Solidity: function resumeTrackingQuorums(bool resetTrackedQuorums) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) ResumeTrackingQuorums(opts *bind.TransactOpts, resetTrackedQuorums bool) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "resumeTrackingQuorums", resetTrackedQuorums)
}

// ResumeTrackingQuorums is a paid mutator transaction binding the contract method 0x191aac7a.
//
// Solidity: function resumeTrackingQuorums(bool resetTrackedQuorums) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) ResumeTrackingQuorums(resetTrackedQuorums bool) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.ResumeTrackingQuorums(&_ContractFinalizerTaskManager.TransactOpts, resetTrackedQuorums)
}

// ResumeTrackingQuorums is a paid mutator transaction binding the contract method 0x191aac7a.
//
// Solidity: function resumeTrackingQuorums(bool resetTrackedQuorums) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) ResumeTrackingQuorums(resetTrackedQuorums bool) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.ResumeTrackingQuorums(&_ContractFinalizerTaskManager.TransactOpts, resetTrackedQuorums)
}

// SetPauserRegistry is a paid mutator transaction binding the contract method 0x10d67a2f.
//
// Solidity: function setPauserRegistry(address newPauserRegistry) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) SetPauserRegistry(opts *bind.TransactOpts, newPauserRegistry common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "setPauserRegistry", newPauserRegistry)
}

// SetPauserRegistry is a paid mutator transaction binding the contract method 0x10d67a2f.
//
// Solidity: function setPauserRegistry(address newPauserRegistry) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) SetPauserRegistry(newPauserRegistry common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.SetPauserRegistry(&_ContractFinalizerTaskManager.TransactOpts, newPauserRegistry)
}

// SetPauserRegistry is a paid mutator transaction binding the contract method 0x10d67a2f.
//
// Solidity: function setPauserRegistry(address newPauserRegistry) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) SetPauserRegistry(newPauserRegistry common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.SetPauserRegistry(&_ContractFinalizerTaskManager.TransactOpts, newPauserRegistry)
}

// SetRolldown is a paid mutator transaction binding the contract method 0xfdc15de8.
//
// Solidity: function setRolldown(address _rolldown) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) SetRolldown(opts *bind.TransactOpts, _rolldown common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "setRolldown", _rolldown)
}

// SetRolldown is a paid mutator transaction binding the contract method 0xfdc15de8.
//
// Solidity: function setRolldown(address _rolldown) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) SetRolldown(_rolldown common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.SetRolldown(&_ContractFinalizerTaskManager.TransactOpts, _rolldown)
}

// SetRolldown is a paid mutator transaction binding the contract method 0xfdc15de8.
//
// Solidity: function setRolldown(address _rolldown) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) SetRolldown(_rolldown common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.SetRolldown(&_ContractFinalizerTaskManager.TransactOpts, _rolldown)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) TransferOwnership(opts *bind.TransactOpts, newOwner common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "transferOwnership", newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) TransferOwnership(newOwner common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.TransferOwnership(&_ContractFinalizerTaskManager.TransactOpts, newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) TransferOwnership(newOwner common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.TransferOwnership(&_ContractFinalizerTaskManager.TransactOpts, newOwner)
}

// Unpause is a paid mutator transaction binding the contract method 0xfabc1cbc.
//
// Solidity: function unpause(uint256 newPausedStatus) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) Unpause(opts *bind.TransactOpts, newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "unpause", newPausedStatus)
}

// Unpause is a paid mutator transaction binding the contract method 0xfabc1cbc.
//
// Solidity: function unpause(uint256 newPausedStatus) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) Unpause(newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.Unpause(&_ContractFinalizerTaskManager.TransactOpts, newPausedStatus)
}

// Unpause is a paid mutator transaction binding the contract method 0xfabc1cbc.
//
// Solidity: function unpause(uint256 newPausedStatus) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) Unpause(newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.Unpause(&_ContractFinalizerTaskManager.TransactOpts, newPausedStatus)
}

// UpdateBlsSignatureCheckerAddress is a paid mutator transaction binding the contract method 0x723114ab.
//
// Solidity: function updateBlsSignatureCheckerAddress(address _blsSignatureCheckerAddress) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) UpdateBlsSignatureCheckerAddress(opts *bind.TransactOpts, _blsSignatureCheckerAddress common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "updateBlsSignatureCheckerAddress", _blsSignatureCheckerAddress)
}

// UpdateBlsSignatureCheckerAddress is a paid mutator transaction binding the contract method 0x723114ab.
//
// Solidity: function updateBlsSignatureCheckerAddress(address _blsSignatureCheckerAddress) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) UpdateBlsSignatureCheckerAddress(_blsSignatureCheckerAddress common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.UpdateBlsSignatureCheckerAddress(&_ContractFinalizerTaskManager.TransactOpts, _blsSignatureCheckerAddress)
}

// UpdateBlsSignatureCheckerAddress is a paid mutator transaction binding the contract method 0x723114ab.
//
// Solidity: function updateBlsSignatureCheckerAddress(address _blsSignatureCheckerAddress) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) UpdateBlsSignatureCheckerAddress(_blsSignatureCheckerAddress common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.UpdateBlsSignatureCheckerAddress(&_ContractFinalizerTaskManager.TransactOpts, _blsSignatureCheckerAddress)
}

// ContractFinalizerTaskManagerBLSSignatureCheckerAddressUpdatedIterator is returned from FilterBLSSignatureCheckerAddressUpdated and is used to iterate over the raw logs and unpacked data for BLSSignatureCheckerAddressUpdated events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerBLSSignatureCheckerAddressUpdatedIterator struct {
	Event *ContractFinalizerTaskManagerBLSSignatureCheckerAddressUpdated // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractFinalizerTaskManagerBLSSignatureCheckerAddressUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerBLSSignatureCheckerAddressUpdated)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractFinalizerTaskManagerBLSSignatureCheckerAddressUpdated)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractFinalizerTaskManagerBLSSignatureCheckerAddressUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerBLSSignatureCheckerAddressUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerBLSSignatureCheckerAddressUpdated represents a BLSSignatureCheckerAddressUpdated event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerBLSSignatureCheckerAddressUpdated struct {
	BlsSignatureCheckerAddress common.Address
	Raw                        types.Log // Blockchain specific contextual infos
}

// FilterBLSSignatureCheckerAddressUpdated is a free log retrieval operation binding the contract event 0x901a654dc830c94e8a12c9a3bc0a92ac11b5cf28046ca8d190691cdaf5209016.
//
// Solidity: event BLSSignatureCheckerAddressUpdated(address blsSignatureCheckerAddress)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterBLSSignatureCheckerAddressUpdated(opts *bind.FilterOpts) (*ContractFinalizerTaskManagerBLSSignatureCheckerAddressUpdatedIterator, error) {

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "BLSSignatureCheckerAddressUpdated")
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerBLSSignatureCheckerAddressUpdatedIterator{contract: _ContractFinalizerTaskManager.contract, event: "BLSSignatureCheckerAddressUpdated", logs: logs, sub: sub}, nil
}

// WatchBLSSignatureCheckerAddressUpdated is a free log subscription operation binding the contract event 0x901a654dc830c94e8a12c9a3bc0a92ac11b5cf28046ca8d190691cdaf5209016.
//
// Solidity: event BLSSignatureCheckerAddressUpdated(address blsSignatureCheckerAddress)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchBLSSignatureCheckerAddressUpdated(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerBLSSignatureCheckerAddressUpdated) (event.Subscription, error) {

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "BLSSignatureCheckerAddressUpdated")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerBLSSignatureCheckerAddressUpdated)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "BLSSignatureCheckerAddressUpdated", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseBLSSignatureCheckerAddressUpdated is a log parse operation binding the contract event 0x901a654dc830c94e8a12c9a3bc0a92ac11b5cf28046ca8d190691cdaf5209016.
//
// Solidity: event BLSSignatureCheckerAddressUpdated(address blsSignatureCheckerAddress)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParseBLSSignatureCheckerAddressUpdated(log types.Log) (*ContractFinalizerTaskManagerBLSSignatureCheckerAddressUpdated, error) {
	event := new(ContractFinalizerTaskManagerBLSSignatureCheckerAddressUpdated)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "BLSSignatureCheckerAddressUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractFinalizerTaskManagerInitializedIterator is returned from FilterInitialized and is used to iterate over the raw logs and unpacked data for Initialized events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerInitializedIterator struct {
	Event *ContractFinalizerTaskManagerInitialized // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractFinalizerTaskManagerInitializedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerInitialized)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractFinalizerTaskManagerInitialized)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractFinalizerTaskManagerInitializedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerInitializedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerInitialized represents a Initialized event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerInitialized struct {
	Version uint8
	Raw     types.Log // Blockchain specific contextual infos
}

// FilterInitialized is a free log retrieval operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterInitialized(opts *bind.FilterOpts) (*ContractFinalizerTaskManagerInitializedIterator, error) {

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerInitializedIterator{contract: _ContractFinalizerTaskManager.contract, event: "Initialized", logs: logs, sub: sub}, nil
}

// WatchInitialized is a free log subscription operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchInitialized(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerInitialized) (event.Subscription, error) {

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerInitialized)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "Initialized", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseInitialized is a log parse operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParseInitialized(log types.Log) (*ContractFinalizerTaskManagerInitialized, error) {
	event := new(ContractFinalizerTaskManagerInitialized)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "Initialized", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractFinalizerTaskManagerNewOpTaskCreatedIterator is returned from FilterNewOpTaskCreated and is used to iterate over the raw logs and unpacked data for NewOpTaskCreated events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerNewOpTaskCreatedIterator struct {
	Event *ContractFinalizerTaskManagerNewOpTaskCreated // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractFinalizerTaskManagerNewOpTaskCreatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerNewOpTaskCreated)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractFinalizerTaskManagerNewOpTaskCreated)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractFinalizerTaskManagerNewOpTaskCreatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerNewOpTaskCreatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerNewOpTaskCreated represents a NewOpTaskCreated event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerNewOpTaskCreated struct {
	TaskIndex uint32
	Task      IFinalizerTaskManagerOpTask
	Raw       types.Log // Blockchain specific contextual infos
}

// FilterNewOpTaskCreated is a free log retrieval operation binding the contract event 0xfaf4b2054479d0f83e909b73cde2a6cb18ec2a93ba8ad5a62329001c86b1f3ea.
//
// Solidity: event NewOpTaskCreated(uint32 indexed taskIndex, (uint32,uint32,uint32,bytes,uint32,bytes,uint32) task)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterNewOpTaskCreated(opts *bind.FilterOpts, taskIndex []uint32) (*ContractFinalizerTaskManagerNewOpTaskCreatedIterator, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "NewOpTaskCreated", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerNewOpTaskCreatedIterator{contract: _ContractFinalizerTaskManager.contract, event: "NewOpTaskCreated", logs: logs, sub: sub}, nil
}

// WatchNewOpTaskCreated is a free log subscription operation binding the contract event 0xfaf4b2054479d0f83e909b73cde2a6cb18ec2a93ba8ad5a62329001c86b1f3ea.
//
// Solidity: event NewOpTaskCreated(uint32 indexed taskIndex, (uint32,uint32,uint32,bytes,uint32,bytes,uint32) task)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchNewOpTaskCreated(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerNewOpTaskCreated, taskIndex []uint32) (event.Subscription, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "NewOpTaskCreated", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerNewOpTaskCreated)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "NewOpTaskCreated", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseNewOpTaskCreated is a log parse operation binding the contract event 0xfaf4b2054479d0f83e909b73cde2a6cb18ec2a93ba8ad5a62329001c86b1f3ea.
//
// Solidity: event NewOpTaskCreated(uint32 indexed taskIndex, (uint32,uint32,uint32,bytes,uint32,bytes,uint32) task)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParseNewOpTaskCreated(log types.Log) (*ContractFinalizerTaskManagerNewOpTaskCreated, error) {
	event := new(ContractFinalizerTaskManagerNewOpTaskCreated)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "NewOpTaskCreated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractFinalizerTaskManagerNewOpTaskForceCreatedIterator is returned from FilterNewOpTaskForceCreated and is used to iterate over the raw logs and unpacked data for NewOpTaskForceCreated events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerNewOpTaskForceCreatedIterator struct {
	Event *ContractFinalizerTaskManagerNewOpTaskForceCreated // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractFinalizerTaskManagerNewOpTaskForceCreatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerNewOpTaskForceCreated)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractFinalizerTaskManagerNewOpTaskForceCreated)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractFinalizerTaskManagerNewOpTaskForceCreatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerNewOpTaskForceCreatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerNewOpTaskForceCreated represents a NewOpTaskForceCreated event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerNewOpTaskForceCreated struct {
	Raw types.Log // Blockchain specific contextual infos
}

// FilterNewOpTaskForceCreated is a free log retrieval operation binding the contract event 0x4ee987e5f1be19cabfb1a243e5c423889f060f33266753953ff0cf9db89966ab.
//
// Solidity: event NewOpTaskForceCreated()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterNewOpTaskForceCreated(opts *bind.FilterOpts) (*ContractFinalizerTaskManagerNewOpTaskForceCreatedIterator, error) {

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "NewOpTaskForceCreated")
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerNewOpTaskForceCreatedIterator{contract: _ContractFinalizerTaskManager.contract, event: "NewOpTaskForceCreated", logs: logs, sub: sub}, nil
}

// WatchNewOpTaskForceCreated is a free log subscription operation binding the contract event 0x4ee987e5f1be19cabfb1a243e5c423889f060f33266753953ff0cf9db89966ab.
//
// Solidity: event NewOpTaskForceCreated()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchNewOpTaskForceCreated(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerNewOpTaskForceCreated) (event.Subscription, error) {

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "NewOpTaskForceCreated")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerNewOpTaskForceCreated)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "NewOpTaskForceCreated", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseNewOpTaskForceCreated is a log parse operation binding the contract event 0x4ee987e5f1be19cabfb1a243e5c423889f060f33266753953ff0cf9db89966ab.
//
// Solidity: event NewOpTaskForceCreated()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParseNewOpTaskForceCreated(log types.Log) (*ContractFinalizerTaskManagerNewOpTaskForceCreated, error) {
	event := new(ContractFinalizerTaskManagerNewOpTaskForceCreated)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "NewOpTaskForceCreated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractFinalizerTaskManagerNewRdTaskCreatedIterator is returned from FilterNewRdTaskCreated and is used to iterate over the raw logs and unpacked data for NewRdTaskCreated events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerNewRdTaskCreatedIterator struct {
	Event *ContractFinalizerTaskManagerNewRdTaskCreated // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractFinalizerTaskManagerNewRdTaskCreatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerNewRdTaskCreated)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractFinalizerTaskManagerNewRdTaskCreated)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractFinalizerTaskManagerNewRdTaskCreatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerNewRdTaskCreatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerNewRdTaskCreated represents a NewRdTaskCreated event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerNewRdTaskCreated struct {
	TaskIndex uint32
	Task      IFinalizerTaskManagerRdTask
	Raw       types.Log // Blockchain specific contextual infos
}

// FilterNewRdTaskCreated is a free log retrieval operation binding the contract event 0x584637a8f9d0f91a80c9f709b2b09d7db1d770fc7294e20d9d2495c378586cd2.
//
// Solidity: event NewRdTaskCreated(uint32 indexed taskIndex, (uint32,uint8,uint32,uint32,uint32,bytes,uint32) task)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterNewRdTaskCreated(opts *bind.FilterOpts, taskIndex []uint32) (*ContractFinalizerTaskManagerNewRdTaskCreatedIterator, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "NewRdTaskCreated", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerNewRdTaskCreatedIterator{contract: _ContractFinalizerTaskManager.contract, event: "NewRdTaskCreated", logs: logs, sub: sub}, nil
}

// WatchNewRdTaskCreated is a free log subscription operation binding the contract event 0x584637a8f9d0f91a80c9f709b2b09d7db1d770fc7294e20d9d2495c378586cd2.
//
// Solidity: event NewRdTaskCreated(uint32 indexed taskIndex, (uint32,uint8,uint32,uint32,uint32,bytes,uint32) task)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchNewRdTaskCreated(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerNewRdTaskCreated, taskIndex []uint32) (event.Subscription, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "NewRdTaskCreated", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerNewRdTaskCreated)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "NewRdTaskCreated", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseNewRdTaskCreated is a log parse operation binding the contract event 0x584637a8f9d0f91a80c9f709b2b09d7db1d770fc7294e20d9d2495c378586cd2.
//
// Solidity: event NewRdTaskCreated(uint32 indexed taskIndex, (uint32,uint8,uint32,uint32,uint32,bytes,uint32) task)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParseNewRdTaskCreated(log types.Log) (*ContractFinalizerTaskManagerNewRdTaskCreated, error) {
	event := new(ContractFinalizerTaskManagerNewRdTaskCreated)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "NewRdTaskCreated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractFinalizerTaskManagerOpTaskCancelledIterator is returned from FilterOpTaskCancelled and is used to iterate over the raw logs and unpacked data for OpTaskCancelled events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerOpTaskCancelledIterator struct {
	Event *ContractFinalizerTaskManagerOpTaskCancelled // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractFinalizerTaskManagerOpTaskCancelledIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerOpTaskCancelled)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractFinalizerTaskManagerOpTaskCancelled)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractFinalizerTaskManagerOpTaskCancelledIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerOpTaskCancelledIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerOpTaskCancelled represents a OpTaskCancelled event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerOpTaskCancelled struct {
	TaskIndex uint32
	Raw       types.Log // Blockchain specific contextual infos
}

// FilterOpTaskCancelled is a free log retrieval operation binding the contract event 0xd6a4e0ff9f3a053708757c7a124abee31ced61f43f17e6e1cf11943ec59e6071.
//
// Solidity: event OpTaskCancelled(uint32 indexed taskIndex)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterOpTaskCancelled(opts *bind.FilterOpts, taskIndex []uint32) (*ContractFinalizerTaskManagerOpTaskCancelledIterator, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "OpTaskCancelled", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerOpTaskCancelledIterator{contract: _ContractFinalizerTaskManager.contract, event: "OpTaskCancelled", logs: logs, sub: sub}, nil
}

// WatchOpTaskCancelled is a free log subscription operation binding the contract event 0xd6a4e0ff9f3a053708757c7a124abee31ced61f43f17e6e1cf11943ec59e6071.
//
// Solidity: event OpTaskCancelled(uint32 indexed taskIndex)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchOpTaskCancelled(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerOpTaskCancelled, taskIndex []uint32) (event.Subscription, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "OpTaskCancelled", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerOpTaskCancelled)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "OpTaskCancelled", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseOpTaskCancelled is a log parse operation binding the contract event 0xd6a4e0ff9f3a053708757c7a124abee31ced61f43f17e6e1cf11943ec59e6071.
//
// Solidity: event OpTaskCancelled(uint32 indexed taskIndex)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParseOpTaskCancelled(log types.Log) (*ContractFinalizerTaskManagerOpTaskCancelled, error) {
	event := new(ContractFinalizerTaskManagerOpTaskCancelled)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "OpTaskCancelled", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractFinalizerTaskManagerOpTaskCompletedIterator is returned from FilterOpTaskCompleted and is used to iterate over the raw logs and unpacked data for OpTaskCompleted events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerOpTaskCompletedIterator struct {
	Event *ContractFinalizerTaskManagerOpTaskCompleted // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractFinalizerTaskManagerOpTaskCompletedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerOpTaskCompleted)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractFinalizerTaskManagerOpTaskCompleted)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractFinalizerTaskManagerOpTaskCompletedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerOpTaskCompletedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerOpTaskCompleted represents a OpTaskCompleted event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerOpTaskCompleted struct {
	TaskIndex    uint32
	TaskResponse IFinalizerTaskManagerOpTaskResponse
	Raw          types.Log // Blockchain specific contextual infos
}

// FilterOpTaskCompleted is a free log retrieval operation binding the contract event 0xff2908483d74b6b70053dd473260acf1b09e0ba0781bf94100bb8277581749de.
//
// Solidity: event OpTaskCompleted(uint32 indexed taskIndex, (uint32,bytes32,bytes32) taskResponse)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterOpTaskCompleted(opts *bind.FilterOpts, taskIndex []uint32) (*ContractFinalizerTaskManagerOpTaskCompletedIterator, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "OpTaskCompleted", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerOpTaskCompletedIterator{contract: _ContractFinalizerTaskManager.contract, event: "OpTaskCompleted", logs: logs, sub: sub}, nil
}

// WatchOpTaskCompleted is a free log subscription operation binding the contract event 0xff2908483d74b6b70053dd473260acf1b09e0ba0781bf94100bb8277581749de.
//
// Solidity: event OpTaskCompleted(uint32 indexed taskIndex, (uint32,bytes32,bytes32) taskResponse)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchOpTaskCompleted(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerOpTaskCompleted, taskIndex []uint32) (event.Subscription, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "OpTaskCompleted", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerOpTaskCompleted)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "OpTaskCompleted", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseOpTaskCompleted is a log parse operation binding the contract event 0xff2908483d74b6b70053dd473260acf1b09e0ba0781bf94100bb8277581749de.
//
// Solidity: event OpTaskCompleted(uint32 indexed taskIndex, (uint32,bytes32,bytes32) taskResponse)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParseOpTaskCompleted(log types.Log) (*ContractFinalizerTaskManagerOpTaskCompleted, error) {
	event := new(ContractFinalizerTaskManagerOpTaskCompleted)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "OpTaskCompleted", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractFinalizerTaskManagerOpTaskForceCompletedIterator is returned from FilterOpTaskForceCompleted and is used to iterate over the raw logs and unpacked data for OpTaskForceCompleted events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerOpTaskForceCompletedIterator struct {
	Event *ContractFinalizerTaskManagerOpTaskForceCompleted // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractFinalizerTaskManagerOpTaskForceCompletedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerOpTaskForceCompleted)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractFinalizerTaskManagerOpTaskForceCompleted)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractFinalizerTaskManagerOpTaskForceCompletedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerOpTaskForceCompletedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerOpTaskForceCompleted represents a OpTaskForceCompleted event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerOpTaskForceCompleted struct {
	TaskIndex    uint32
	TaskResponse IFinalizerTaskManagerOpTaskResponse
	Raw          types.Log // Blockchain specific contextual infos
}

// FilterOpTaskForceCompleted is a free log retrieval operation binding the contract event 0xdf22f3558e4841b63d77179546b3eae63e4e343bbe752746b093162bc526be4c.
//
// Solidity: event OpTaskForceCompleted(uint32 indexed taskIndex, (uint32,bytes32,bytes32) taskResponse)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterOpTaskForceCompleted(opts *bind.FilterOpts, taskIndex []uint32) (*ContractFinalizerTaskManagerOpTaskForceCompletedIterator, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "OpTaskForceCompleted", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerOpTaskForceCompletedIterator{contract: _ContractFinalizerTaskManager.contract, event: "OpTaskForceCompleted", logs: logs, sub: sub}, nil
}

// WatchOpTaskForceCompleted is a free log subscription operation binding the contract event 0xdf22f3558e4841b63d77179546b3eae63e4e343bbe752746b093162bc526be4c.
//
// Solidity: event OpTaskForceCompleted(uint32 indexed taskIndex, (uint32,bytes32,bytes32) taskResponse)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchOpTaskForceCompleted(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerOpTaskForceCompleted, taskIndex []uint32) (event.Subscription, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "OpTaskForceCompleted", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerOpTaskForceCompleted)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "OpTaskForceCompleted", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseOpTaskForceCompleted is a log parse operation binding the contract event 0xdf22f3558e4841b63d77179546b3eae63e4e343bbe752746b093162bc526be4c.
//
// Solidity: event OpTaskForceCompleted(uint32 indexed taskIndex, (uint32,bytes32,bytes32) taskResponse)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParseOpTaskForceCompleted(log types.Log) (*ContractFinalizerTaskManagerOpTaskForceCompleted, error) {
	event := new(ContractFinalizerTaskManagerOpTaskForceCompleted)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "OpTaskForceCompleted", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractFinalizerTaskManagerOpTaskRespondedIterator is returned from FilterOpTaskResponded and is used to iterate over the raw logs and unpacked data for OpTaskResponded events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerOpTaskRespondedIterator struct {
	Event *ContractFinalizerTaskManagerOpTaskResponded // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractFinalizerTaskManagerOpTaskRespondedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerOpTaskResponded)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractFinalizerTaskManagerOpTaskResponded)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractFinalizerTaskManagerOpTaskRespondedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerOpTaskRespondedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerOpTaskResponded represents a OpTaskResponded event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerOpTaskResponded struct {
	TaskIndex            uint32
	TaskResponse         IFinalizerTaskManagerOpTaskResponse
	TaskResponseMetadata IFinalizerTaskManagerTaskResponseMetadata
	Raw                  types.Log // Blockchain specific contextual infos
}

// FilterOpTaskResponded is a free log retrieval operation binding the contract event 0x47adacb0b6bbd726ae39ac6c006cca1c2006c9aedaa882dcba7c4804db7c41ce.
//
// Solidity: event OpTaskResponded(uint32 indexed taskIndex, (uint32,bytes32,bytes32) taskResponse, (uint32,bytes32,uint96[],uint96[]) taskResponseMetadata)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterOpTaskResponded(opts *bind.FilterOpts, taskIndex []uint32) (*ContractFinalizerTaskManagerOpTaskRespondedIterator, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "OpTaskResponded", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerOpTaskRespondedIterator{contract: _ContractFinalizerTaskManager.contract, event: "OpTaskResponded", logs: logs, sub: sub}, nil
}

// WatchOpTaskResponded is a free log subscription operation binding the contract event 0x47adacb0b6bbd726ae39ac6c006cca1c2006c9aedaa882dcba7c4804db7c41ce.
//
// Solidity: event OpTaskResponded(uint32 indexed taskIndex, (uint32,bytes32,bytes32) taskResponse, (uint32,bytes32,uint96[],uint96[]) taskResponseMetadata)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchOpTaskResponded(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerOpTaskResponded, taskIndex []uint32) (event.Subscription, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "OpTaskResponded", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerOpTaskResponded)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "OpTaskResponded", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseOpTaskResponded is a log parse operation binding the contract event 0x47adacb0b6bbd726ae39ac6c006cca1c2006c9aedaa882dcba7c4804db7c41ce.
//
// Solidity: event OpTaskResponded(uint32 indexed taskIndex, (uint32,bytes32,bytes32) taskResponse, (uint32,bytes32,uint96[],uint96[]) taskResponseMetadata)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParseOpTaskResponded(log types.Log) (*ContractFinalizerTaskManagerOpTaskResponded, error) {
	event := new(ContractFinalizerTaskManagerOpTaskResponded)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "OpTaskResponded", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractFinalizerTaskManagerOwnershipTransferredIterator is returned from FilterOwnershipTransferred and is used to iterate over the raw logs and unpacked data for OwnershipTransferred events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerOwnershipTransferredIterator struct {
	Event *ContractFinalizerTaskManagerOwnershipTransferred // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractFinalizerTaskManagerOwnershipTransferredIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerOwnershipTransferred)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractFinalizerTaskManagerOwnershipTransferred)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractFinalizerTaskManagerOwnershipTransferredIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerOwnershipTransferredIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerOwnershipTransferred represents a OwnershipTransferred event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerOwnershipTransferred struct {
	PreviousOwner common.Address
	NewOwner      common.Address
	Raw           types.Log // Blockchain specific contextual infos
}

// FilterOwnershipTransferred is a free log retrieval operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterOwnershipTransferred(opts *bind.FilterOpts, previousOwner []common.Address, newOwner []common.Address) (*ContractFinalizerTaskManagerOwnershipTransferredIterator, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerOwnershipTransferredIterator{contract: _ContractFinalizerTaskManager.contract, event: "OwnershipTransferred", logs: logs, sub: sub}, nil
}

// WatchOwnershipTransferred is a free log subscription operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchOwnershipTransferred(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerOwnershipTransferred, previousOwner []common.Address, newOwner []common.Address) (event.Subscription, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerOwnershipTransferred)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseOwnershipTransferred is a log parse operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParseOwnershipTransferred(log types.Log) (*ContractFinalizerTaskManagerOwnershipTransferred, error) {
	event := new(ContractFinalizerTaskManagerOwnershipTransferred)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractFinalizerTaskManagerPauseTrackingOpStateIterator is returned from FilterPauseTrackingOpState and is used to iterate over the raw logs and unpacked data for PauseTrackingOpState events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerPauseTrackingOpStateIterator struct {
	Event *ContractFinalizerTaskManagerPauseTrackingOpState // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractFinalizerTaskManagerPauseTrackingOpStateIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerPauseTrackingOpState)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractFinalizerTaskManagerPauseTrackingOpState)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractFinalizerTaskManagerPauseTrackingOpStateIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerPauseTrackingOpStateIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerPauseTrackingOpState represents a PauseTrackingOpState event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerPauseTrackingOpState struct {
	Raw types.Log // Blockchain specific contextual infos
}

// FilterPauseTrackingOpState is a free log retrieval operation binding the contract event 0x4d60154266b2ea0c8f091d257eac5abc941c46cb54d0c3069a830f6339fe1da1.
//
// Solidity: event PauseTrackingOpState()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterPauseTrackingOpState(opts *bind.FilterOpts) (*ContractFinalizerTaskManagerPauseTrackingOpStateIterator, error) {

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "PauseTrackingOpState")
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerPauseTrackingOpStateIterator{contract: _ContractFinalizerTaskManager.contract, event: "PauseTrackingOpState", logs: logs, sub: sub}, nil
}

// WatchPauseTrackingOpState is a free log subscription operation binding the contract event 0x4d60154266b2ea0c8f091d257eac5abc941c46cb54d0c3069a830f6339fe1da1.
//
// Solidity: event PauseTrackingOpState()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchPauseTrackingOpState(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerPauseTrackingOpState) (event.Subscription, error) {

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "PauseTrackingOpState")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerPauseTrackingOpState)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "PauseTrackingOpState", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParsePauseTrackingOpState is a log parse operation binding the contract event 0x4d60154266b2ea0c8f091d257eac5abc941c46cb54d0c3069a830f6339fe1da1.
//
// Solidity: event PauseTrackingOpState()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParsePauseTrackingOpState(log types.Log) (*ContractFinalizerTaskManagerPauseTrackingOpState, error) {
	event := new(ContractFinalizerTaskManagerPauseTrackingOpState)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "PauseTrackingOpState", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractFinalizerTaskManagerPausedIterator is returned from FilterPaused and is used to iterate over the raw logs and unpacked data for Paused events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerPausedIterator struct {
	Event *ContractFinalizerTaskManagerPaused // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractFinalizerTaskManagerPausedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerPaused)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractFinalizerTaskManagerPaused)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractFinalizerTaskManagerPausedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerPausedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerPaused represents a Paused event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerPaused struct {
	Account         common.Address
	NewPausedStatus *big.Int
	Raw             types.Log // Blockchain specific contextual infos
}

// FilterPaused is a free log retrieval operation binding the contract event 0xab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d.
//
// Solidity: event Paused(address indexed account, uint256 newPausedStatus)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterPaused(opts *bind.FilterOpts, account []common.Address) (*ContractFinalizerTaskManagerPausedIterator, error) {

	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "Paused", accountRule)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerPausedIterator{contract: _ContractFinalizerTaskManager.contract, event: "Paused", logs: logs, sub: sub}, nil
}

// WatchPaused is a free log subscription operation binding the contract event 0xab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d.
//
// Solidity: event Paused(address indexed account, uint256 newPausedStatus)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchPaused(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerPaused, account []common.Address) (event.Subscription, error) {

	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "Paused", accountRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerPaused)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "Paused", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParsePaused is a log parse operation binding the contract event 0xab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d.
//
// Solidity: event Paused(address indexed account, uint256 newPausedStatus)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParsePaused(log types.Log) (*ContractFinalizerTaskManagerPaused, error) {
	event := new(ContractFinalizerTaskManagerPaused)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "Paused", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractFinalizerTaskManagerPauserRegistrySetIterator is returned from FilterPauserRegistrySet and is used to iterate over the raw logs and unpacked data for PauserRegistrySet events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerPauserRegistrySetIterator struct {
	Event *ContractFinalizerTaskManagerPauserRegistrySet // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractFinalizerTaskManagerPauserRegistrySetIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerPauserRegistrySet)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractFinalizerTaskManagerPauserRegistrySet)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractFinalizerTaskManagerPauserRegistrySetIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerPauserRegistrySetIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerPauserRegistrySet represents a PauserRegistrySet event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerPauserRegistrySet struct {
	PauserRegistry    common.Address
	NewPauserRegistry common.Address
	Raw               types.Log // Blockchain specific contextual infos
}

// FilterPauserRegistrySet is a free log retrieval operation binding the contract event 0x6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6.
//
// Solidity: event PauserRegistrySet(address pauserRegistry, address newPauserRegistry)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterPauserRegistrySet(opts *bind.FilterOpts) (*ContractFinalizerTaskManagerPauserRegistrySetIterator, error) {

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "PauserRegistrySet")
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerPauserRegistrySetIterator{contract: _ContractFinalizerTaskManager.contract, event: "PauserRegistrySet", logs: logs, sub: sub}, nil
}

// WatchPauserRegistrySet is a free log subscription operation binding the contract event 0x6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6.
//
// Solidity: event PauserRegistrySet(address pauserRegistry, address newPauserRegistry)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchPauserRegistrySet(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerPauserRegistrySet) (event.Subscription, error) {

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "PauserRegistrySet")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerPauserRegistrySet)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "PauserRegistrySet", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParsePauserRegistrySet is a log parse operation binding the contract event 0x6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6.
//
// Solidity: event PauserRegistrySet(address pauserRegistry, address newPauserRegistry)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParsePauserRegistrySet(log types.Log) (*ContractFinalizerTaskManagerPauserRegistrySet, error) {
	event := new(ContractFinalizerTaskManagerPauserRegistrySet)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "PauserRegistrySet", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractFinalizerTaskManagerRdTaskCancelledIterator is returned from FilterRdTaskCancelled and is used to iterate over the raw logs and unpacked data for RdTaskCancelled events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerRdTaskCancelledIterator struct {
	Event *ContractFinalizerTaskManagerRdTaskCancelled // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractFinalizerTaskManagerRdTaskCancelledIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerRdTaskCancelled)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractFinalizerTaskManagerRdTaskCancelled)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractFinalizerTaskManagerRdTaskCancelledIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerRdTaskCancelledIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerRdTaskCancelled represents a RdTaskCancelled event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerRdTaskCancelled struct {
	TaskIndex uint32
	Raw       types.Log // Blockchain specific contextual infos
}

// FilterRdTaskCancelled is a free log retrieval operation binding the contract event 0x0bf46bfca6e2137d35b893c295add8c33bcfbffafdef93252cb51aed7538ba0c.
//
// Solidity: event RdTaskCancelled(uint32 indexed taskIndex)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterRdTaskCancelled(opts *bind.FilterOpts, taskIndex []uint32) (*ContractFinalizerTaskManagerRdTaskCancelledIterator, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "RdTaskCancelled", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerRdTaskCancelledIterator{contract: _ContractFinalizerTaskManager.contract, event: "RdTaskCancelled", logs: logs, sub: sub}, nil
}

// WatchRdTaskCancelled is a free log subscription operation binding the contract event 0x0bf46bfca6e2137d35b893c295add8c33bcfbffafdef93252cb51aed7538ba0c.
//
// Solidity: event RdTaskCancelled(uint32 indexed taskIndex)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchRdTaskCancelled(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerRdTaskCancelled, taskIndex []uint32) (event.Subscription, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "RdTaskCancelled", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerRdTaskCancelled)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "RdTaskCancelled", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseRdTaskCancelled is a log parse operation binding the contract event 0x0bf46bfca6e2137d35b893c295add8c33bcfbffafdef93252cb51aed7538ba0c.
//
// Solidity: event RdTaskCancelled(uint32 indexed taskIndex)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParseRdTaskCancelled(log types.Log) (*ContractFinalizerTaskManagerRdTaskCancelled, error) {
	event := new(ContractFinalizerTaskManagerRdTaskCancelled)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "RdTaskCancelled", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractFinalizerTaskManagerRdTaskCompletedIterator is returned from FilterRdTaskCompleted and is used to iterate over the raw logs and unpacked data for RdTaskCompleted events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerRdTaskCompletedIterator struct {
	Event *ContractFinalizerTaskManagerRdTaskCompleted // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractFinalizerTaskManagerRdTaskCompletedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerRdTaskCompleted)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractFinalizerTaskManagerRdTaskCompleted)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractFinalizerTaskManagerRdTaskCompletedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerRdTaskCompletedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerRdTaskCompleted represents a RdTaskCompleted event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerRdTaskCompleted struct {
	TaskIndex    uint32
	TaskResponse IFinalizerTaskManagerRdTaskResponse
	Raw          types.Log // Blockchain specific contextual infos
}

// FilterRdTaskCompleted is a free log retrieval operation binding the contract event 0x1797ca59e06ea4a0efe10ac0fb51b58c8acf5cfedbc15fae51c10021dcb906e6.
//
// Solidity: event RdTaskCompleted(uint32 indexed taskIndex, (uint32,bytes32,uint8,uint32,bytes32,uint256,uint256,address) taskResponse)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterRdTaskCompleted(opts *bind.FilterOpts, taskIndex []uint32) (*ContractFinalizerTaskManagerRdTaskCompletedIterator, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "RdTaskCompleted", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerRdTaskCompletedIterator{contract: _ContractFinalizerTaskManager.contract, event: "RdTaskCompleted", logs: logs, sub: sub}, nil
}

// WatchRdTaskCompleted is a free log subscription operation binding the contract event 0x1797ca59e06ea4a0efe10ac0fb51b58c8acf5cfedbc15fae51c10021dcb906e6.
//
// Solidity: event RdTaskCompleted(uint32 indexed taskIndex, (uint32,bytes32,uint8,uint32,bytes32,uint256,uint256,address) taskResponse)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchRdTaskCompleted(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerRdTaskCompleted, taskIndex []uint32) (event.Subscription, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "RdTaskCompleted", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerRdTaskCompleted)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "RdTaskCompleted", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseRdTaskCompleted is a log parse operation binding the contract event 0x1797ca59e06ea4a0efe10ac0fb51b58c8acf5cfedbc15fae51c10021dcb906e6.
//
// Solidity: event RdTaskCompleted(uint32 indexed taskIndex, (uint32,bytes32,uint8,uint32,bytes32,uint256,uint256,address) taskResponse)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParseRdTaskCompleted(log types.Log) (*ContractFinalizerTaskManagerRdTaskCompleted, error) {
	event := new(ContractFinalizerTaskManagerRdTaskCompleted)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "RdTaskCompleted", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractFinalizerTaskManagerRdTaskRespondedIterator is returned from FilterRdTaskResponded and is used to iterate over the raw logs and unpacked data for RdTaskResponded events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerRdTaskRespondedIterator struct {
	Event *ContractFinalizerTaskManagerRdTaskResponded // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractFinalizerTaskManagerRdTaskRespondedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerRdTaskResponded)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractFinalizerTaskManagerRdTaskResponded)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractFinalizerTaskManagerRdTaskRespondedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerRdTaskRespondedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerRdTaskResponded represents a RdTaskResponded event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerRdTaskResponded struct {
	TaskIndex            uint32
	TaskResponse         IFinalizerTaskManagerRdTaskResponse
	TaskResponseMetadata IFinalizerTaskManagerTaskResponseMetadata
	Raw                  types.Log // Blockchain specific contextual infos
}

// FilterRdTaskResponded is a free log retrieval operation binding the contract event 0x82e5c8e9447510b867d248c892385ba34fa6c2d4c4c26ff6868499ae4027f2c6.
//
// Solidity: event RdTaskResponded(uint32 indexed taskIndex, (uint32,bytes32,uint8,uint32,bytes32,uint256,uint256,address) taskResponse, (uint32,bytes32,uint96[],uint96[]) taskResponseMetadata)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterRdTaskResponded(opts *bind.FilterOpts, taskIndex []uint32) (*ContractFinalizerTaskManagerRdTaskRespondedIterator, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "RdTaskResponded", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerRdTaskRespondedIterator{contract: _ContractFinalizerTaskManager.contract, event: "RdTaskResponded", logs: logs, sub: sub}, nil
}

// WatchRdTaskResponded is a free log subscription operation binding the contract event 0x82e5c8e9447510b867d248c892385ba34fa6c2d4c4c26ff6868499ae4027f2c6.
//
// Solidity: event RdTaskResponded(uint32 indexed taskIndex, (uint32,bytes32,uint8,uint32,bytes32,uint256,uint256,address) taskResponse, (uint32,bytes32,uint96[],uint96[]) taskResponseMetadata)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchRdTaskResponded(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerRdTaskResponded, taskIndex []uint32) (event.Subscription, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "RdTaskResponded", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerRdTaskResponded)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "RdTaskResponded", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseRdTaskResponded is a log parse operation binding the contract event 0x82e5c8e9447510b867d248c892385ba34fa6c2d4c4c26ff6868499ae4027f2c6.
//
// Solidity: event RdTaskResponded(uint32 indexed taskIndex, (uint32,bytes32,uint8,uint32,bytes32,uint256,uint256,address) taskResponse, (uint32,bytes32,uint96[],uint96[]) taskResponseMetadata)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParseRdTaskResponded(log types.Log) (*ContractFinalizerTaskManagerRdTaskResponded, error) {
	event := new(ContractFinalizerTaskManagerRdTaskResponded)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "RdTaskResponded", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractFinalizerTaskManagerResumeTrackingOpStateIterator is returned from FilterResumeTrackingOpState and is used to iterate over the raw logs and unpacked data for ResumeTrackingOpState events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerResumeTrackingOpStateIterator struct {
	Event *ContractFinalizerTaskManagerResumeTrackingOpState // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractFinalizerTaskManagerResumeTrackingOpStateIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerResumeTrackingOpState)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractFinalizerTaskManagerResumeTrackingOpState)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractFinalizerTaskManagerResumeTrackingOpStateIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerResumeTrackingOpStateIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerResumeTrackingOpState represents a ResumeTrackingOpState event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerResumeTrackingOpState struct {
	ResetTrackedQuorums bool
	Raw                 types.Log // Blockchain specific contextual infos
}

// FilterResumeTrackingOpState is a free log retrieval operation binding the contract event 0x6af4ae1f481aff20ce571abd65375b67b22359883a823d1ddf4bd8f2879ff7ba.
//
// Solidity: event ResumeTrackingOpState(bool resetTrackedQuorums)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterResumeTrackingOpState(opts *bind.FilterOpts) (*ContractFinalizerTaskManagerResumeTrackingOpStateIterator, error) {

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "ResumeTrackingOpState")
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerResumeTrackingOpStateIterator{contract: _ContractFinalizerTaskManager.contract, event: "ResumeTrackingOpState", logs: logs, sub: sub}, nil
}

// WatchResumeTrackingOpState is a free log subscription operation binding the contract event 0x6af4ae1f481aff20ce571abd65375b67b22359883a823d1ddf4bd8f2879ff7ba.
//
// Solidity: event ResumeTrackingOpState(bool resetTrackedQuorums)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchResumeTrackingOpState(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerResumeTrackingOpState) (event.Subscription, error) {

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "ResumeTrackingOpState")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerResumeTrackingOpState)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "ResumeTrackingOpState", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseResumeTrackingOpState is a log parse operation binding the contract event 0x6af4ae1f481aff20ce571abd65375b67b22359883a823d1ddf4bd8f2879ff7ba.
//
// Solidity: event ResumeTrackingOpState(bool resetTrackedQuorums)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParseResumeTrackingOpState(log types.Log) (*ContractFinalizerTaskManagerResumeTrackingOpState, error) {
	event := new(ContractFinalizerTaskManagerResumeTrackingOpState)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "ResumeTrackingOpState", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractFinalizerTaskManagerRolldownTargetUpdatedIterator is returned from FilterRolldownTargetUpdated and is used to iterate over the raw logs and unpacked data for RolldownTargetUpdated events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerRolldownTargetUpdatedIterator struct {
	Event *ContractFinalizerTaskManagerRolldownTargetUpdated // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractFinalizerTaskManagerRolldownTargetUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerRolldownTargetUpdated)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractFinalizerTaskManagerRolldownTargetUpdated)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractFinalizerTaskManagerRolldownTargetUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerRolldownTargetUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerRolldownTargetUpdated represents a RolldownTargetUpdated event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerRolldownTargetUpdated struct {
	RolldownAddress common.Address
	Raw             types.Log // Blockchain specific contextual infos
}

// FilterRolldownTargetUpdated is a free log retrieval operation binding the contract event 0x2f20cf1bda67739044c5bf577353970c3dbc183b2c7274d1e8584a1026923267.
//
// Solidity: event RolldownTargetUpdated(address rolldownAddress)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterRolldownTargetUpdated(opts *bind.FilterOpts) (*ContractFinalizerTaskManagerRolldownTargetUpdatedIterator, error) {

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "RolldownTargetUpdated")
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerRolldownTargetUpdatedIterator{contract: _ContractFinalizerTaskManager.contract, event: "RolldownTargetUpdated", logs: logs, sub: sub}, nil
}

// WatchRolldownTargetUpdated is a free log subscription operation binding the contract event 0x2f20cf1bda67739044c5bf577353970c3dbc183b2c7274d1e8584a1026923267.
//
// Solidity: event RolldownTargetUpdated(address rolldownAddress)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchRolldownTargetUpdated(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerRolldownTargetUpdated) (event.Subscription, error) {

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "RolldownTargetUpdated")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerRolldownTargetUpdated)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "RolldownTargetUpdated", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseRolldownTargetUpdated is a log parse operation binding the contract event 0x2f20cf1bda67739044c5bf577353970c3dbc183b2c7274d1e8584a1026923267.
//
// Solidity: event RolldownTargetUpdated(address rolldownAddress)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParseRolldownTargetUpdated(log types.Log) (*ContractFinalizerTaskManagerRolldownTargetUpdated, error) {
	event := new(ContractFinalizerTaskManagerRolldownTargetUpdated)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "RolldownTargetUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractFinalizerTaskManagerStaleStakesForbiddenUpdateIterator is returned from FilterStaleStakesForbiddenUpdate and is used to iterate over the raw logs and unpacked data for StaleStakesForbiddenUpdate events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerStaleStakesForbiddenUpdateIterator struct {
	Event *ContractFinalizerTaskManagerStaleStakesForbiddenUpdate // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractFinalizerTaskManagerStaleStakesForbiddenUpdateIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerStaleStakesForbiddenUpdate)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractFinalizerTaskManagerStaleStakesForbiddenUpdate)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractFinalizerTaskManagerStaleStakesForbiddenUpdateIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerStaleStakesForbiddenUpdateIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerStaleStakesForbiddenUpdate represents a StaleStakesForbiddenUpdate event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerStaleStakesForbiddenUpdate struct {
	Value bool
	Raw   types.Log // Blockchain specific contextual infos
}

// FilterStaleStakesForbiddenUpdate is a free log retrieval operation binding the contract event 0x40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc.
//
// Solidity: event StaleStakesForbiddenUpdate(bool value)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterStaleStakesForbiddenUpdate(opts *bind.FilterOpts) (*ContractFinalizerTaskManagerStaleStakesForbiddenUpdateIterator, error) {

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "StaleStakesForbiddenUpdate")
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerStaleStakesForbiddenUpdateIterator{contract: _ContractFinalizerTaskManager.contract, event: "StaleStakesForbiddenUpdate", logs: logs, sub: sub}, nil
}

// WatchStaleStakesForbiddenUpdate is a free log subscription operation binding the contract event 0x40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc.
//
// Solidity: event StaleStakesForbiddenUpdate(bool value)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchStaleStakesForbiddenUpdate(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerStaleStakesForbiddenUpdate) (event.Subscription, error) {

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "StaleStakesForbiddenUpdate")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerStaleStakesForbiddenUpdate)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "StaleStakesForbiddenUpdate", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseStaleStakesForbiddenUpdate is a log parse operation binding the contract event 0x40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc.
//
// Solidity: event StaleStakesForbiddenUpdate(bool value)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParseStaleStakesForbiddenUpdate(log types.Log) (*ContractFinalizerTaskManagerStaleStakesForbiddenUpdate, error) {
	event := new(ContractFinalizerTaskManagerStaleStakesForbiddenUpdate)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "StaleStakesForbiddenUpdate", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractFinalizerTaskManagerUnpausedIterator is returned from FilterUnpaused and is used to iterate over the raw logs and unpacked data for Unpaused events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerUnpausedIterator struct {
	Event *ContractFinalizerTaskManagerUnpaused // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractFinalizerTaskManagerUnpausedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerUnpaused)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractFinalizerTaskManagerUnpaused)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractFinalizerTaskManagerUnpausedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerUnpausedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerUnpaused represents a Unpaused event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerUnpaused struct {
	Account         common.Address
	NewPausedStatus *big.Int
	Raw             types.Log // Blockchain specific contextual infos
}

// FilterUnpaused is a free log retrieval operation binding the contract event 0x3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c.
//
// Solidity: event Unpaused(address indexed account, uint256 newPausedStatus)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterUnpaused(opts *bind.FilterOpts, account []common.Address) (*ContractFinalizerTaskManagerUnpausedIterator, error) {

	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "Unpaused", accountRule)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerUnpausedIterator{contract: _ContractFinalizerTaskManager.contract, event: "Unpaused", logs: logs, sub: sub}, nil
}

// WatchUnpaused is a free log subscription operation binding the contract event 0x3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c.
//
// Solidity: event Unpaused(address indexed account, uint256 newPausedStatus)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchUnpaused(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerUnpaused, account []common.Address) (event.Subscription, error) {

	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "Unpaused", accountRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerUnpaused)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "Unpaused", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseUnpaused is a log parse operation binding the contract event 0x3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c.
//
// Solidity: event Unpaused(address indexed account, uint256 newPausedStatus)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParseUnpaused(log types.Log) (*ContractFinalizerTaskManagerUnpaused, error) {
	event := new(ContractFinalizerTaskManagerUnpaused)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "Unpaused", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

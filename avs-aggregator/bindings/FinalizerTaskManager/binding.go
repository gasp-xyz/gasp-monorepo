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
	ABI: "[{\"type\":\"function\",\"name\":\"THRESHOLD_DENOMINATOR\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"_validateTaskResponse\",\"inputs\":[{\"name\":\"taskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"taskType\",\"type\":\"uint8\",\"internalType\":\"enumIFinalizerTaskManager.TaskType\"},{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"aggregator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allTaskHashes\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumIFinalizerTaskManager.TaskType\"},{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allTaskResponses\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumIFinalizerTaskManager.TaskType\"},{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allowNonRootInit\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"blsApkRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIBLSApkRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"blsSignatureChecker\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractBLSSignatureChecker\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"cancelPendingTasks\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"chainRdBatchNonce\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"checkSignatures\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"referenceBlockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.QuorumStakeTotals\",\"components\":[{\"name\":\"signedStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"totalStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"createNewOpTask\",\"inputs\":[{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"createNewRdTask\",\"inputs\":[{\"name\":\"chainId\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"},{\"name\":\"batchId\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"delegation\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIDelegationManager\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"dummyForOperatorStateInfoType\",\"inputs\":[{\"name\":\"_operatorStateInfo\",\"type\":\"tuple\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorStateInfo\",\"components\":[{\"name\":\"operatorsStateChanged\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"quorumsRemoved\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsAdded[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"quorumsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsStakeUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]},{\"name\":\"quorumsApkUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsApkUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"operatorsRemoved\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"operatorsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsAdded[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]},{\"name\":\"operatorsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsStakeUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"operatorsQuorumCountUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsQuorumCountUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]}]}],\"outputs\":[],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"dummyForQuorumStakeTotalsType\",\"inputs\":[{\"name\":\"_quorumStakeTotals\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.QuorumStakeTotals\",\"components\":[{\"name\":\"signedStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"totalStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]}],\"outputs\":[],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"forceCancelPendingTasks\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"forceCreateNewOpTask\",\"inputs\":[{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"forceRespondToOpTask\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"generator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getBatchOperatorFromId\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"}],\"outputs\":[{\"name\":\"operators\",\"type\":\"address[]\",\"internalType\":\"address[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getBatchOperatorId\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operators\",\"type\":\"address[]\",\"internalType\":\"address[]\"}],\"outputs\":[{\"name\":\"operatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getCheckSignaturesIndices\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"referenceBlockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"nonSignerOperatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structOperatorStateRetriever.CheckSignaturesIndices\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getOperatorState\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple[][]\",\"internalType\":\"structOperatorStateRetriever.Operator[][]\",\"components\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"stake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getOperatorState\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"\",\"type\":\"tuple[][]\",\"internalType\":\"structOperatorStateRetriever.Operator[][]\",\"components\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"stake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getQuorumBitmapsAtBlockNumber\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256[]\",\"internalType\":\"uint256[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"idToTaskStatus\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumIFinalizerTaskManager.TaskType\"},{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumIFinalizerTaskManager.TaskStatus\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"_pauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"initialOwner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_aggregator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_generator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_allowNonRootInit\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"_blsSignatureCheckerAddress\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_taskResponseWindowBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"_operatorStateRetrieverExtended\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_rolldown\",\"type\":\"address\",\"internalType\":\"contractIRolldown\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"isTaskPending\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastCompletedOpTaskCreatedBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastCompletedOpTaskNum\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastOpTaskCreatedBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastRdTaskCreatedBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestOpTaskNum\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestRdTaskNum\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"operatorStateRetrieverExtended\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"operatorsStateInfoHash\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"pauseAll\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"pauseTrackingOpState\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[{\"name\":\"index\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pauserRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"registryCoordinator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"respondToOpTask\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"respondToRdTask\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.RdTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"chainId\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"},{\"name\":\"batchId\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.RdTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"chainId\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"},{\"name\":\"batchId\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"rdUpdate\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"rangeStart\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"rangeEnd\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"updater\",\"type\":\"address\",\"internalType\":\"address\"}]},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"resumeTrackingQuorums\",\"inputs\":[{\"name\":\"resetTrackedQuorums\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"rolldown\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIRolldown\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"setAggregator\",\"inputs\":[{\"name\":\"_aggregator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setGenerator\",\"inputs\":[{\"name\":\"_generator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setPauserRegistry\",\"inputs\":[{\"name\":\"newPauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setRolldown\",\"inputs\":[{\"name\":\"_rolldown\",\"type\":\"address\",\"internalType\":\"contractIRolldown\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"stakeRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIStakeRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"taskResponseWindowBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"unpause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateBlsSignatureCheckerAddress\",\"inputs\":[{\"name\":\"_blsSignatureCheckerAddress\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"AggregatorUpdated\",\"inputs\":[{\"name\":\"aggregatorAddress\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"BLSSignatureCheckerAddressUpdated\",\"inputs\":[{\"name\":\"blsSignatureCheckerAddress\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"GeneratorUpdated\",\"inputs\":[{\"name\":\"generatorAddress\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Initialized\",\"inputs\":[{\"name\":\"version\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewOpTaskCreated\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"task\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.OpTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewOpTaskForceCreated\",\"inputs\":[],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewRdTaskCreated\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"task\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.RdTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"chainId\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"},{\"name\":\"batchId\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OpTaskCancelled\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OpTaskCompleted\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OpTaskForceCompleted\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OpTaskResponded\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"taskResponseMetadata\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.TaskResponseMetadata\",\"components\":[{\"name\":\"taskResponsedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"hashOfNonSigners\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quroumStakeTotals\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quroumStakeSigned\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"PauseTrackingOpState\",\"inputs\":[],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Paused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"PauserRegistrySet\",\"inputs\":[{\"name\":\"pauserRegistry\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"newPauserRegistry\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractIPauserRegistry\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RdTaskCancelled\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RdTaskCompleted\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.RdTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"chainId\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"},{\"name\":\"batchId\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"rdUpdate\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"rangeStart\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"rangeEnd\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"updater\",\"type\":\"address\",\"internalType\":\"address\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RdTaskResponded\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.RdTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"chainId\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"},{\"name\":\"batchId\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"rdUpdate\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"rangeStart\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"rangeEnd\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"updater\",\"type\":\"address\",\"internalType\":\"address\"}]},{\"name\":\"taskResponseMetadata\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.TaskResponseMetadata\",\"components\":[{\"name\":\"taskResponsedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"hashOfNonSigners\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quroumStakeTotals\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quroumStakeSigned\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"ResumeTrackingOpState\",\"inputs\":[{\"name\":\"resetTrackedQuorums\",\"type\":\"bool\",\"indexed\":false,\"internalType\":\"bool\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RolldownTargetUpdated\",\"inputs\":[{\"name\":\"rolldownAddress\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"StaleStakesForbiddenUpdate\",\"inputs\":[{\"name\":\"value\",\"type\":\"bool\",\"indexed\":false,\"internalType\":\"bool\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Unpaused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false}]",
	Bin: "0x608060405234801561001057600080fd5b50615de680620000216000396000f3fe608060405234801561001057600080fd5b50600436106103af5760003560e01c806368304835116101f4578063930390d91161011a578063e70c2623116100ad578063f5640cf81161007c578063f5640cf8146108aa578063f9120af6146108bd578063fabc1cbc146108d0578063fdc15de8146108e357600080fd5b8063e70c26231461086c578063e72ddf101461087c578063ef0244581461088f578063f2fde38b1461089757600080fd5b8063bf2315ed116100e9578063bf2315ed146107f5578063cefdc1d414610830578063de43483814610851578063df5cf7231461086457600080fd5b8063930390d91461079c578063a69563a9146107c2578063adfcb048146107d9578063b1edc8b4146107e257600080fd5b806379badf7311610192578063886f119511610161578063886f1195146107445780638c82af5e146107575780638da5cb5b1461076f5780638fc8729a1461078057600080fd5b806379badf73146106ff5780637afa1eed146107075780637afdd54b1461071a5780638380acbd1461073157600080fd5b80636efb4636116101ce5780636efb4636146106b05780636f254819146106d1578063715018a6146106e4578063723114ab146106ec57600080fd5b8063683048351461068d5780636d14a987146106955780636e125ff41461069d57600080fd5b806341789d57116102d9578063537a2929116102775780635c155662116102465780635c155662146106555780635c975abb146106755780635df459461461067d57806360202fc01461068557600080fd5b8063537a29291461060557806354d127de1461061c578063595c6a671461062a5780635ac86ab71461063257600080fd5b80634d2b57fe116102b35780634d2b57fe146105a25780634d7a7116146105c25780634f739f74146105d2578063516a7227146105f257600080fd5b806341789d571461055257806345265b7a1461057e5780634a7c7e4b1461058f57600080fd5b80631c178e9c1161035157806331b36bd91161032057806331b36bd9146104eb5780633563b0d11461050b57806336f78ed81461052b5780633d9fb00c1461053f57600080fd5b80631c178e9c1461048857806321e78062146104b3578063245a7bfc146104bb5780632830e8f9146104d657600080fd5b8063136439dd1161038d578063136439dd146103fe57806313f815ed14610411578063191aac7a1461044a5780631ac272971461045d57600080fd5b806301a3f013146103b45780630ee0fdbd146103c957806310d67a2f146103eb575b600080fd5b6103c76103c23660046140e6565b6108f6565b005b60a3546103d69060ff1681565b60405190151581526020015b60405180910390f35b6103c76103f9366004614149565b610a79565b6103c761040c366004614166565b610b32565b61043c61041f3660046141b0565b609960209081526000928352604080842090915290825290205481565b6040519081526020016103e2565b6103c76104583660046141f5565b610c71565b61043c61046b3660046141b0565b609a60209081526000928352604080842090915290825290205481565b60975461049b906001600160a01b031681565b6040516001600160a01b0390911681526020016103e2565b6103c7610cb2565b609e5461049b9064010000000090046001600160a01b031681565b6104de610d36565b6040516103e2919061425f565b6104fe6104f9366004614326565b610dc4565b6040516103e29190614414565b61051e610519366004614427565b610ee0565b6040516103e29190614585565b60a0546103d690600160a01b900460ff1681565b60a05461049b906001600160a01b031681565b60985461056990600160c01b900463ffffffff1681565b60405163ffffffff90911681526020016103e2565b6103c761058c366004614598565b50565b6103c761059d366004614149565b611378565b6105b56105b0366004614638565b6113ee565b6040516103e29190614687565b609c546105699063ffffffff1681565b6105e56105e036600461471c565b611503565b6040516103e2919061486d565b6103c7610600366004614c1c565b611c2b565b609c5461056990600160601b900463ffffffff1681565b6103c761058c366004614c90565b6103c7611ebe565b6103d6610640366004614ccb565b606654600160ff9092169190911b9081161490565b610668610663366004614cee565b611f85565b6040516103e29190614d51565b60665461043c565b61049b61214d565b6103c76121c0565b61049b6121c8565b61049b612212565b6103c76106ab366004614d89565b61225c565b6106c36106be366004614ddd565b6122e2565b6040516103e2929190614e9d565b6103c76106df366004614ef5565b612382565b6103c7612623565b6103c76106fa366004614149565b612635565b6103c761268b565b609f5461049b906001600160a01b031681565b60985461056990600160e01b900463ffffffff1681565b60985461049b906001600160a01b031681565b60655461049b906001600160a01b031681565b609c5461056990640100000000900463ffffffff1681565b6033546001600160a01b031661049b565b609c546105699068010000000000000000900463ffffffff1681565b6105696107aa366004614f11565b60a16020526000908152604090205463ffffffff1681565b60985461056990600160a01b900463ffffffff1681565b61043c60a25481565b6103c76107f0366004614f2c565b6126be565b6108236108033660046141b0565b609b60209081526000928352604080842090915290825290205460ff1681565b6040516103e29190614f93565b61084361083e366004614fad565b612932565b6040516103e2929190614fe4565b6103c761085f366004615005565b612ac4565b61049b612c7d565b609e546105699063ffffffff1681565b6103c761088a3660046150c1565b612cc7565b61043c606481565b6103c76108a5366004614149565b6130e9565b6103c76108b8366004614d89565b61315f565b6103c76108cb366004614149565b6131ba565b6103c76108de366004614166565b61323e565b6103c76108f1366004614149565b61339a565b6108fe613410565b61094a8260405160200161091291906151a7565b60408051601f1981840301815291905280516020918201209060009061093a90850185615281565b6107f06040870160208801615281565b60408051808201909152606080825260208201526040805160808101825263ffffffff4316815260006020808301829052848101519383019390935283516060830152916109d09083906109a090870187615281565b86846040516020016109b392919061530e565b60405160208183030381529060405280519060200120600461346a565b6109de84604001358661353e565b6109eb6020850185615281565b63ffffffff167fff2908483d74b6b70053dd473260acf1b09e0ba0781bf94100bb8277581749de85604051610a20919061532e565b60405180910390a2610a356020850185615281565b63ffffffff167fdf22f3558e4841b63d77179546b3eae63e4e343bbe752746b093162bc526be4c85604051610a6a919061532e565b60405180910390a25050505050565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610acc573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610af0919061533c565b6001600160a01b0316336001600160a01b031614610b295760405162461bcd60e51b8152600401610b2090615359565b60405180910390fd5b61058c816135ed565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa158015610b7a573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b9e91906153a3565b610bba5760405162461bcd60e51b8152600401610b20906153c0565b60665481811614610c335760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e70617573653a20696e76616c696420617474656d70742060448201527f746f20756e70617573652066756e6374696f6e616c69747900000000000000006064820152608401610b20565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b610c79613410565b60405181151581527f6af4ae1f481aff20ce571abd65375b67b22359883a823d1ddf4bd8f2879ff7ba906020015b60405180910390a150565b609f546001600160a01b03163314610cdc5760405162461bcd60e51b8152600401610b2090615408565b60a054600160a01b900460ff161515600114610d2c5760405162461bcd60e51b815260206004820152600f60248201526e4e6f207461736b2070656e64696e6760881b6044820152606401610b20565b610d346136e4565b565b609d8054610d4390615427565b80601f0160208091040260200160405190810160405280929190818152602001828054610d6f90615427565b8015610dbc5780601f10610d9157610100808354040283529160200191610dbc565b820191906000526020600020905b815481529060010190602001808311610d9f57829003601f168201915b505050505081565b606081516001600160401b03811115610ddf57610ddf614272565b604051908082528060200260200182016040528015610e08578160200160208202803683370190505b50905060005b8251811015610ed957836001600160a01b03166313542a4e848381518110610e3857610e3861545b565b60200260200101516040518263ffffffff1660e01b8152600401610e6b91906001600160a01b0391909116815260200190565b602060405180830381865afa158015610e88573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610eac9190615471565b828281518110610ebe57610ebe61545b565b6020908102919091010152610ed2816154a0565b9050610e0e565b5092915050565b60606000846001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa158015610f22573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610f46919061533c565b90506000856001600160a01b0316639e9923c26040518163ffffffff1660e01b8152600401602060405180830381865afa158015610f88573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610fac919061533c565b90506000866001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa158015610fee573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611012919061533c565b9050600086516001600160401b0381111561102f5761102f614272565b60405190808252806020026020018201604052801561106257816020015b606081526020019060019003908161104d5790505b50905060005b875181101561136a5760008882815181106110855761108561545b565b0160200151604051638902624560e01b815260f89190911c6004820181905263ffffffff8a16602483015291506000906001600160a01b03871690638902624590604401600060405180830381865afa1580156110e6573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261110e91908101906154b9565b905080516001600160401b0381111561112957611129614272565b60405190808252806020026020018201604052801561117457816020015b60408051606081018252600080825260208083018290529282015282526000199092019101816111475790505b508484815181106111875761118761545b565b602002602001018190525060005b8151811015611354576040518060600160405280876001600160a01b03166347b314e88585815181106111ca576111ca61545b565b60200260200101516040518263ffffffff1660e01b81526004016111f091815260200190565b602060405180830381865afa15801561120d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611231919061533c565b6001600160a01b031681526020018383815181106112515761125161545b565b60200260200101518152602001896001600160a01b031663fa28c62785858151811061127f5761127f61545b565b60209081029190910101516040516001600160e01b031960e084901b168152600481019190915260ff8816602482015263ffffffff8f166044820152606401602060405180830381865afa1580156112db573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906112ff9190615560565b6001600160601b031681525085858151811061131d5761131d61545b565b602002602001015182815181106113365761133661545b565b6020026020010181905250808061134c906154a0565b915050611195565b5050508080611362906154a0565b915050611068565b5093505050505b9392505050565b606654156113985760405162461bcd60e51b8152600401610b209061557b565b6113a0613410565b609f80546001600160a01b0319166001600160a01b0383169081179091556040519081527f60f5ac48a13f8b5bf4b213de190dd2de2592c9f6f5ac7dc14e3d726b95ded2da90602001610ca7565b606081516001600160401b0381111561140957611409614272565b604051908082528060200260200182016040528015611432578160200160208202803683370190505b50905060005b8251811015610ed957836001600160a01b031663296bb0648483815181106114625761146261545b565b60200260200101516040518263ffffffff1660e01b815260040161148891815260200190565b602060405180830381865afa1580156114a5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906114c9919061533c565b8282815181106114db576114db61545b565b6001600160a01b03909216602092830291909101909101526114fc816154a0565b9050611438565b61152e6040518060800160405280606081526020016060815260200160608152602001606081525090565b6000876001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa15801561156e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611592919061533c565b90506115bf6040518060800160405280606081526020016060815260200160608152602001606081525090565b6040516361c8a12f60e11b81526001600160a01b038a169063c391425e906115ef908b90899089906004016155b2565b600060405180830381865afa15801561160c573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261163491908101906155fc565b81526040516340e03a8160e11b81526001600160a01b038316906381c0750290611666908b908b908b9060040161568a565b600060405180830381865afa158015611683573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526116ab91908101906155fc565b6040820152856001600160401b038111156116c8576116c8614272565b6040519080825280602002602001820160405280156116fb57816020015b60608152602001906001900390816116e65790505b50606082015260005b60ff8116871115611b3c576000856001600160401b0381111561172957611729614272565b604051908082528060200260200182016040528015611752578160200160208202803683370190505b5083606001518360ff168151811061176c5761176c61545b565b602002602001018190525060005b86811015611a3c5760008c6001600160a01b03166304ec63518a8a858181106117a5576117a561545b565b905060200201358e886000015186815181106117c3576117c361545b565b60200260200101516040518463ffffffff1660e01b81526004016118009392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa15801561181d573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061184191906156aa565b9050806001600160c01b03166000036118e85760405162461bcd60e51b815260206004820152605c60248201527f4f70657261746f7253746174655265747269657665722e676574436865636b5360448201527f69676e617475726573496e64696365733a206f70657261746f72206d7573742060648201527f6265207265676973746572656420617420626c6f636b6e756d62657200000000608482015260a401610b20565b8a8a8560ff168181106118fd576118fd61545b565b60016001600160c01b038516919093013560f81c1c82169091039050611a2957856001600160a01b031663dd9846b98a8a8581811061193e5761193e61545b565b905060200201358d8d8860ff1681811061195a5761195a61545b565b6040516001600160e01b031960e087901b1681526004810194909452919091013560f81c60248301525063ffffffff8f166044820152606401602060405180830381865afa1580156119b0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906119d491906156d3565b85606001518560ff16815181106119ed576119ed61545b565b60200260200101518481518110611a0657611a0661545b565b63ffffffff9092166020928302919091019091015282611a25816154a0565b9350505b5080611a34816154a0565b91505061177a565b506000816001600160401b03811115611a5757611a57614272565b604051908082528060200260200182016040528015611a80578160200160208202803683370190505b50905060005b82811015611b015784606001518460ff1681518110611aa757611aa761545b565b60200260200101518181518110611ac057611ac061545b565b6020026020010151828281518110611ada57611ada61545b565b63ffffffff9092166020928302919091019091015280611af9816154a0565b915050611a86565b508084606001518460ff1681518110611b1c57611b1c61545b565b602002602001018190525050508080611b34906156f0565b915050611704565b506000896001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa158015611b7d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611ba1919061533c565b60405163354952a360e21b81529091506001600160a01b0382169063d5254a8c90611bd4908b908b908e9060040161570f565b600060405180830381865afa158015611bf1573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611c1991908101906155fc565b60208301525098975050505050505050565b609e5464010000000090046001600160a01b03163314611c755760405162461bcd60e51b8152602060048201526005602482015264041757468360dc1b6044820152606401610b20565b609c54600160601b900463ffffffff16151580611c94575060a35460ff165b611cd05760405162461bcd60e51b815260206004820152600d60248201526c1d5cd9481c9bdbdd081a5b9a5d609a1b6044820152606401610b20565b366000611ce060a0860186615739565b90925090506000611cf760e0870160c08801615281565b9050611d4586604051602001611d0d91906151a7565b60408051601f19818403018152919052805160209182012090600090611d3590890189615281565b6107f060408b0160208c01615281565b600080611d8e87604051602001611d5c919061532e565b60405160208183030381529060405280519060200120896040016020810190611d859190615281565b888888886138d3565b9092509050611da06020890189615281565b63ffffffff167f47adacb0b6bbd726ae39ac6c006cca1c2006c9aedaa882dcba7c4804db7c41ce8883604051611dd792919061530e565b60405180910390a28115611e0f57611e0a6000611df760208a018a615281565b89846040516020016109b392919061530e565b611e5b565b611e516000611e2160208a018a615281565b8984604051602001611e3492919061530e565b60405160208183030381529060405280519060200120600361346a565b5050505050505050565b611e6987604001358961353e565b611e766020880188615281565b63ffffffff167fff2908483d74b6b70053dd473260acf1b09e0ba0781bf94100bb8277581749de88604051611eab919061532e565b60405180910390a250505050505b505050565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa158015611f06573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611f2a91906153a3565b611f465760405162461bcd60e51b8152600401610b20906153c0565b600019606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b60606000846001600160a01b031663c391425e84866040518363ffffffff1660e01b8152600401611fb792919061577f565b600060405180830381865afa158015611fd4573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611ffc91908101906155fc565b9050600084516001600160401b0381111561201957612019614272565b604051908082528060200260200182016040528015612042578160200160208202803683370190505b50905060005b855181101561214357866001600160a01b03166304ec63518783815181106120725761207261545b565b60200260200101518786858151811061208d5761208d61545b565b60200260200101516040518463ffffffff1660e01b81526004016120ca9392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa1580156120e7573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061210b91906156aa565b6001600160c01b03168282815181106121265761212661545b565b60209081029190910101528061213b816154a0565b915050612048565b5095945050505050565b60975460408051632efa2ca360e11b815290516000926001600160a01b031691635df459469160048083019260209291908290030181865afa158015612197573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906121bb919061533c565b905090565b610cdc613410565b60975460408051636830483560e01b815290516000926001600160a01b03169163683048359160048083019260209291908290030181865afa158015612197573d6000803e3d6000fd5b60975460408051636d14a98760e01b815290516000926001600160a01b031691636d14a9879160048083019260209291908290030181865afa158015612197573d6000803e3d6000fd5b609f546001600160a01b031633146122865760405162461bcd60e51b8152600401610b2090615408565b60a054600160a01b900460ff16156122d75760405162461bcd60e51b81526020600482015260146024820152735461736b20616c72656164792070656e64696e6760601b6044820152606401610b20565b611eb9838383613a49565b604080518082019091526060808252602082015260975460405163377da31b60e11b81526000916001600160a01b031690636efb46369061232f908a908a908a908a908a90600401615822565b600060405180830381865afa15801561234c573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526123749190810190615982565b915091509550959350505050565b609f546001600160a01b031633146123ac5760405162461bcd60e51b8152600401610b2090615408565b60a054600160a01b900460ff16156123fd5760405162461bcd60e51b81526020600482015260146024820152735461736b20616c72656164792070656e64696e6760601b6044820152606401610b20565b609c54600160601b900463ffffffff161580159061241a57504315155b6124585760405162461bcd60e51b815260206004820152600f60248201526e13dc0814dd185d19481d5b9a5b9a5d608a1b6044820152606401610b20565b6098546040805160e08101909152600160e01b90910463ffffffff16808252906000906020810185600281111561249157612491614f7d565b815263ffffffff80861660208301524381166040830152609c54600160601b9004166060820152609d80546080909201916124cb90615427565b80601f01602080910402602001604051908101604052809291908181526020018280546124f790615427565b80156125445780601f1061251957610100808354040283529160200191612544565b820191906000526020600020905b81548152906001019060200180831161252757829003601f168201915b5050509183525050609e5463ffffffff1660209182015260405191925061259391600191859161257691869101615a32565b604051602081830303815290604052805190602001206001613d10565b43609c60046101000a81548163ffffffff021916908363ffffffff1602179055508163ffffffff167f584637a8f9d0f91a80c9f709b2b09d7db1d770fc7294e20d9d2495c378586cd2826040516125ea9190615a32565b60405180910390a26125fd826001615ab0565b6098601c6101000a81548163ffffffff021916908363ffffffff16021790555050505050565b61262b613410565b610d346000613dea565b61263d613410565b609780546001600160a01b0319166001600160a01b0383169081179091556040519081527f901a654dc830c94e8a12c9a3bc0a92ac11b5cf28046ca8d190691cdaf520901690602001610ca7565b612693613410565b6040517f4d60154266b2ea0c8f091d257eac5abc941c46cb54d0c3069a830f6339fe1da190600090a1565b60a054600160a01b900460ff16151560011461270e5760405162461bcd60e51b815260206004820152600f60248201526e4e6f207461736b2070656e64696e6760881b6044820152606401610b20565b6099600084600181111561272457612724614f7d565b600181111561273557612735614f7d565b815260200190815260200160002060008363ffffffff1663ffffffff16815260200190815260200160002054841461279f5760405162461bcd60e51b815260206004820152600d60248201526c0a8c2e6d640dad2e6dac2e8c6d609b1b6044820152606401610b20565b6001609b60008560018111156127b7576127b7614f7d565b60018111156127c8576127c8614f7d565b81526020808201929092526040908101600090812063ffffffff8716825290925290205460ff16600481111561280057612800614f7d565b1461283e5760405162461bcd60e51b815260206004820152600e60248201526d4e6f7420496e697420737461746560901b6044820152606401610b20565b6000609a8185600181111561285557612855614f7d565b600181111561286657612866614f7d565b815260200190815260200160002060008463ffffffff1663ffffffff16815260200190815260200160002054146128cc5760405162461bcd60e51b815260206004820152600a6024820152690416c72647920526573760b41b6044820152606401610b20565b6098546128e690600160a01b900463ffffffff1682615ab0565b63ffffffff164363ffffffff16111561292c5760405162461bcd60e51b8152602060048201526008602482015267546f6f206c61746560c01b6044820152606401610b20565b50505050565b604080516001808252818301909252600091606091839160208083019080368337019050509050848160008151811061296d5761296d61545b565b60209081029190910101526040516361c8a12f60e11b81526000906001600160a01b0388169063c391425e906129a9908890869060040161577f565b600060405180830381865afa1580156129c6573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526129ee91908101906155fc565b600081518110612a0057612a0061545b565b60209081029190910101516040516304ec635160e01b81526004810188905263ffffffff87811660248301529091166044820181905291506000906001600160a01b038916906304ec635190606401602060405180830381865afa158015612a6c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612a9091906156aa565b6001600160c01b031690506000612aa682613e3c565b905081612ab48a838a610ee0565b9550955050505050935093915050565b600054610100900460ff1615808015612ae45750600054600160ff909116105b80612afe5750303b158015612afe575060005460ff166001145b612b615760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610b20565b6000805460ff191660011790558015612b84576000805461ff0019166101001790555b612b8f8a6000613f08565b612b9889613dea565b609e8054640100000000600160c01b0319166401000000006001600160a01b038b81169190910291909117909155609f80546001600160a01b03199081168a84161790915560a3805460ff1916891515179055609780548216888416179055609880548684166001600160c01b031990911617600160a01b63ffffffff89160217905560a080549091169184169190911790558015612c71576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050505050505050565b6097546040805163df5cf72360e01b815290516000926001600160a01b03169163df5cf7239160048083019260209291908290030181865afa158015612197573d6000803e3d6000fd5b609e5464010000000090046001600160a01b03163314612d115760405162461bcd60e51b8152602060048201526005602482015264041757468360dc1b6044820152606401610b20565b366000612d2160a0860186615739565b90925090506000612d3860e0870160c08801615281565b905060a16000612d4e6060880160408901614f11565b6002811115612d5f57612d5f614f7d565b6002811115612d7057612d70614f7d565b815260208101919091526040016000205463ffffffff161580612df3575060a16000612da26060880160408901614f11565b6002811115612db357612db3614f7d565b6002811115612dc457612dc4614f7d565b815260208101919091526040016000205463ffffffff16612deb6080870160608801615281565b63ffffffff16145b612e3f5760405162461bcd60e51b815260206004820152601a60248201527f636861696e526442617463684e6f6e6365206d69736d617463680000000000006044820152606401610b20565b612e8b86604051602001612e539190615ad8565b60408051601f19818403018152919052805160209182012090600190612e7b90890189615281565b6107f060808b0160608c01615281565b600080612eca87604051602001612ea29190615c20565b60408051601f198184030181529190528051602090910120611d8560a08b0160808c01615281565b9092509050612edc6020890189615281565b63ffffffff167f82e5c8e9447510b867d248c892385ba34fa6c2d4c4c26ff6868499ae4027f2c68883604051612f13929190615c2f565b60405180910390a28115612f4b57612f466001612f3360208a018a615281565b89846040516020016109b3929190615c2f565b612f70565b611e516001612f5d60208a018a615281565b8984604051602001611e34929190615c2f565b600080612f8360608a0160408b01614f11565b6002811115612f9457612f94614f7d565b0361301a5760408051808201825260a08a810135825260c08b013560208301525491516223d0b560e61b815290916001600160a01b0316906308f42d4090612fe69060808d0135908590600401615c51565b600060405180830381600087803b15801561300057600080fd5b505af1158015613014573d6000803e3d6000fd5b50505050505b61302a6080890160608a01615281565b613035906001615ab0565b60a1600061304960608c0160408d01614f11565b600281111561305a5761305a614f7d565b600281111561306b5761306b614f7d565b8152602080820192909252604001600020805463ffffffff191663ffffffff93909316929092179091556130a190890189615281565b63ffffffff167f1797ca59e06ea4a0efe10ac0fb51b58c8acf5cfedbc15fae51c10021dcb906e6896040516130d69190615c20565b60405180910390a2505050505050505050565b6130f1613410565b6001600160a01b0381166131565760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610b20565b61058c81613dea565b613167613410565b60a054600160a01b900460ff1615613181576131816136e4565b61318c838383613a49565b6040517f4ee987e5f1be19cabfb1a243e5c423889f060f33266753953ff0cf9db89966ab90600090a1505050565b606654156131da5760405162461bcd60e51b8152600401610b209061557b565b6131e2613410565b609e8054640100000000600160c01b0319166401000000006001600160a01b038416908102919091179091556040519081527f602cec4b1583b07d071161da5eb9589444d2459201e2fab7753dc941e9351c2190602001610ca7565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613291573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906132b5919061533c565b6001600160a01b0316336001600160a01b0316146132e55760405162461bcd60e51b8152600401610b2090615359565b6066541981196066541916146133635760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e756e70617573653a20696e76616c696420617474656d7060448201527f7420746f2070617573652066756e6374696f6e616c69747900000000000000006064820152608401610b20565b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c90602001610c66565b606654156133ba5760405162461bcd60e51b8152600401610b209061557b565b6133c2613410565b60a080546001600160a01b0319166001600160a01b0383169081179091556040519081527f2f20cf1bda67739044c5bf577353970c3dbc183b2c7274d1e8584a102692326790602001610ca7565b6033546001600160a01b03163314610d345760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610b20565b81609a600086600181111561348157613481614f7d565b600181111561349257613492614f7d565b815260200190815260200160002060008563ffffffff1663ffffffff1681526020019081526020016000208190555080609b60008660018111156134d8576134d8614f7d565b60018111156134e9576134e9614f7d565b81526020808201929092526040908101600090812063ffffffff881682529092529020805460ff1916600183600481111561352657613526614f7d565b02179055505060a0805460ff60a01b19169055505050565b60a28290556135536040820160208301615281565b609c805463ffffffff92909216600160601b0263ffffffff60601b199092169190911790556135856060820182615739565b61359191609d91614023565b506135a260a0820160808301615281565b609e805463ffffffff191663ffffffff929092169190911790556135c96020820182615281565b609c60086101000a81548163ffffffff021916908363ffffffff1602179055505050565b6001600160a01b03811661367b5760405162461bcd60e51b815260206004820152604960248201527f5061757361626c652e5f73657450617573657252656769737472793a206e657760448201527f50617573657252656769737472792063616e6e6f7420626520746865207a65726064820152686f206164647265737360b81b608482015260a401610b20565b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b609854600160c01b900463ffffffff16156137d45760985460009061371890600190600160c01b900463ffffffff16615c6f565b63ffffffff811660009081527f10afac9233b4ccc54d6404ffc1cf3b47515a2b8edbf675d15eddce05a027dcbd602052604090205490915060019060ff16600481111561376757613767614f7d565b036137d25763ffffffff811660008181527f10afac9233b4ccc54d6404ffc1cf3b47515a2b8edbf675d15eddce05a027dcbd6020526040808220805460ff19166002179055517fd6a4e0ff9f3a053708757c7a124abee31ced61f43f17e6e1cf11943ec59e60719190a25b505b609854600160e01b900463ffffffff16156138c45760985460009061380890600190600160e01b900463ffffffff16615c6f565b63ffffffff811660009081527f298c800d0881dd208d705ebc03eb18189f38118259f27dd43b4c60d61c607e87602052604090205490915060019060ff16600481111561385757613857614f7d565b036138c25763ffffffff811660008181527f298c800d0881dd208d705ebc03eb18189f38118259f27dd43b4c60d61c607e876020526040808220805460ff19166002179055517f0bf46bfca6e2137d35b893c295add8c33bcfbffafdef93252cb51aed7538ba0c9190a25b505b60a0805460ff60a01b19169055565b60408051608081018252600080825260208201819052606082840181905280830152609754925163377da31b60e11b81529092839182916001600160a01b031690636efb463690613930908d908b908b908f908f90600401615822565b600060405180830381865afa15801561394d573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526139759190810190615982565b6040805160808101825263ffffffff43168152602080820184905284015191810191909152825160608201529193509150600160005b88811015613a38578760ff16856020015182815181106139cd576139cd61545b565b60200260200101516139df9190615c94565b6001600160601b0316606486600001518381518110613a0057613a0061545b565b60200260200101516001600160601b0316613a1b9190615cc3565b1015613a2657600091505b80613a30816154a0565b9150506139ab565b509b909a5098505050505050505050565b609c5463ffffffff600160601b909104164314801590613a6857504315155b613ac55760405162461bcd60e51b815260206004820152602860248201527f43616e277420696e206c617374436f6d706c657465644f705461736b43726561604482015267746564426c6f636b60c01b6064820152608401610b20565b6098546040805160e0810182526000818301819052606080830181905260a083015260c082015263ffffffff600160c01b909304831680825243841660208084019190915293871660808301528251601f8601859004850281018501909352848352929091908590859081908401838280828437600092018290525060608601949094525050609c54600160601b900463ffffffff169091039050613bbc5763ffffffff431660408083019190915280516020601f860181900481028201810190925284815290859085908190840183828082843760009201919091525050505060a082015263ffffffff851660c0820152613c70565b609c54600160601b900463ffffffff166040820152609d8054613bde90615427565b80601f0160208091040260200160405190810160405280929190818152602001828054613c0a90615427565b8015613c575780601f10613c2c57610100808354040283529160200191613c57565b820191906000526020600020905b815481529060010190602001808311613c3a57829003601f168201915b505050505060a0820152609e5463ffffffff1660c08201525b613c87600083836040516020016125769190615ce2565b609c805463ffffffff19164363ffffffff90811691909117909155604051908316907ffaf4b2054479d0f83e909b73cde2a6cb18ec2a93ba8ad5a62329001c86b1f3ea90613cd6908490615ce2565b60405180910390a2613ce9826001615ab0565b609860186101000a81548163ffffffff021916908363ffffffff1602179055505050505050565b8160996000866001811115613d2757613d27614f7d565b6001811115613d3857613d38614f7d565b815260200190815260200160002060008563ffffffff1663ffffffff1681526020019081526020016000208190555080609b6000866001811115613d7e57613d7e614f7d565b6001811115613d8f57613d8f614f7d565b81526020808201929092526040908101600090812063ffffffff881682529092529020805460ff19166001836004811115613dcc57613dcc614f7d565b02179055505060a0805460ff60a01b1916600160a01b179055505050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6060600080613e4a84613ff2565b61ffff166001600160401b03811115613e6557613e65614272565b6040519080825280601f01601f191660200182016040528015613e8f576020820181803683370190505b5090506000805b825182108015613ea7575061010081105b15613efe576001811b935085841615613eee578060f81b838381518110613ed057613ed061545b565b60200101906001600160f81b031916908160001a9053508160010191505b613ef7816154a0565b9050613e96565b5090949350505050565b6065546001600160a01b0316158015613f2957506001600160a01b03821615155b613fab5760405162461bcd60e51b815260206004820152604760248201527f5061757361626c652e5f696e697469616c697a655061757365723a205f696e6960448201527f7469616c697a6550617573657228292063616e206f6e6c792062652063616c6c6064820152666564206f6e636560c81b608482015260a401610b20565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2613fee826135ed565b5050565b6000805b821561401d57614007600184615d78565b909216918061401581615d8f565b915050613ff6565b92915050565b82805461402f90615427565b90600052602060002090601f0160209004810192826140515760008555614097565b82601f1061406a5782800160ff19823516178555614097565b82800160010185558215614097579182015b8281111561409757823582559160200191906001019061407c565b506140a39291506140a7565b5090565b5b808211156140a357600081556001016140a8565b600060e082840312156140ce57600080fd5b50919050565b6000606082840312156140ce57600080fd5b600080608083850312156140f957600080fd5b82356001600160401b0381111561410f57600080fd5b61411b858286016140bc565b92505061412b84602085016140d4565b90509250929050565b6001600160a01b038116811461058c57600080fd5b60006020828403121561415b57600080fd5b813561137181614134565b60006020828403121561417857600080fd5b5035919050565b80356002811061418e57600080fd5b919050565b63ffffffff8116811461058c57600080fd5b803561418e81614193565b600080604083850312156141c357600080fd5b6141cc8361417f565b915060208301356141dc81614193565b809150509250929050565b801515811461058c57600080fd5b60006020828403121561420757600080fd5b8135611371816141e7565b6000815180845260005b818110156142385760208185018101518683018201520161421c565b8181111561424a576000602083870101525b50601f01601f19169290920160200192915050565b6020815260006113716020830184614212565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b03811182821017156142aa576142aa614272565b60405290565b60405161010081016001600160401b03811182821017156142aa576142aa614272565b604051601f8201601f191681016001600160401b03811182821017156142fb576142fb614272565b604052919050565b60006001600160401b0382111561431c5761431c614272565b5060051b60200190565b6000806040838503121561433957600080fd5b823561434481614134565b91506020838101356001600160401b0381111561436057600080fd5b8401601f8101861361437157600080fd5b803561438461437f82614303565b6142d3565b81815260059190911b820183019083810190888311156143a357600080fd5b928401925b828410156143ca5783356143bb81614134565b825292840192908401906143a8565b80955050505050509250929050565b600081518084526020808501945080840160005b83811015614409578151875295820195908201906001016143ed565b509495945050505050565b60208152600061137160208301846143d9565b60008060006060848603121561443c57600080fd5b833561444781614134565b92506020848101356001600160401b038082111561446457600080fd5b818701915087601f83011261447857600080fd5b81358181111561448a5761448a614272565b61449c601f8201601f191685016142d3565b915080825288848285010111156144b257600080fd5b80848401858401376000848284010152508094505050506144d5604085016141a5565b90509250925092565b600082825180855260208086019550808260051b8401018186016000805b8581101561457757868403601f19018a52825180518086529086019086860190845b8181101561456257835180516001600160a01b03168452898101518a8501526040908101516001600160601b0316908401529288019260609092019160010161451e565b50509a86019a945050918401916001016144fc565b509198975050505050505050565b60208152600061137160208301846144de565b6000602082840312156145aa57600080fd5b81356001600160401b038111156145c057600080fd5b82016040818503121561137157600080fd5b600082601f8301126145e357600080fd5b813560206145f361437f83614303565b82815260059290921b8401810191818101908684111561461257600080fd5b8286015b8481101561462d5780358352918301918301614616565b509695505050505050565b6000806040838503121561464b57600080fd5b823561465681614134565b915060208301356001600160401b0381111561467157600080fd5b61467d858286016145d2565b9150509250929050565b6020808252825182820181905260009190848201906040850190845b818110156146c85783516001600160a01b0316835292840192918401916001016146a3565b50909695505050505050565b60008083601f8401126146e657600080fd5b5081356001600160401b038111156146fd57600080fd5b60208301915083602082850101111561471557600080fd5b9250929050565b6000806000806000806080878903121561473557600080fd5b863561474081614134565b9550602087013561475081614193565b945060408701356001600160401b038082111561476c57600080fd5b6147788a838b016146d4565b9096509450606089013591508082111561479157600080fd5b818901915089601f8301126147a557600080fd5b8135818111156147b457600080fd5b8a60208260051b85010111156147c957600080fd5b6020830194508093505050509295509295509295565b600081518084526020808501945080840160005b8381101561440957815163ffffffff16875295820195908201906001016147f3565b600082825180855260208086019550808260051b84010181860160005b8481101561486057601f1986840301895261484e8383516147df565b98840198925090830190600101614832565b5090979650505050505050565b60208152600082516080602084015261488960a08401826147df565b90506020840151601f19808584030160408601526148a783836147df565b925060408601519150808584030160608601526148c483836147df565b92506060860151915080858403016080860152506148e28282614815565b95945050505050565b600082601f8301126148fc57600080fd5b8135602061490c61437f83614303565b82815260059290921b8401810191818101908684111561492b57600080fd5b8286015b8481101561462d57803561494281614193565b835291830191830161492f565b60006040828403121561496157600080fd5b614969614288565b9050813581526020820135602082015292915050565b600082601f83011261499057600080fd5b813560206149a061437f83614303565b82815260069290921b840181019181810190868411156149bf57600080fd5b8286015b8481101561462d576149d5888261494f565b8352918301916040016149c3565b600082601f8301126149f457600080fd5b6149fc614288565b806040840185811115614a0e57600080fd5b845b81811015614a28578035845260209384019301614a10565b509095945050505050565b600060808284031215614a4557600080fd5b614a4d614288565b9050614a5983836149e3565b8152614a6883604084016149e3565b602082015292915050565b600082601f830112614a8457600080fd5b81356020614a9461437f83614303565b82815260059290921b84018101918181019086841115614ab357600080fd5b8286015b8481101561462d5780356001600160401b03811115614ad65760008081fd5b614ae48986838b01016148eb565b845250918301918301614ab7565b60006101808284031215614b0557600080fd5b614b0d6142b0565b905081356001600160401b0380821115614b2657600080fd5b614b32858386016148eb565b83526020840135915080821115614b4857600080fd5b614b548583860161497f565b60208401526040840135915080821115614b6d57600080fd5b614b798583860161497f565b6040840152614b8b8560608601614a33565b6060840152614b9d8560e0860161494f565b6080840152610120840135915080821115614bb757600080fd5b614bc3858386016148eb565b60a0840152610140840135915080821115614bdd57600080fd5b614be9858386016148eb565b60c0840152610160840135915080821115614c0357600080fd5b50614c1084828501614a73565b60e08301525092915050565b600080600060a08486031215614c3157600080fd5b83356001600160401b0380821115614c4857600080fd5b614c54878388016140bc565b9450614c6387602088016140d4565b93506080860135915080821115614c7957600080fd5b50614c8686828701614af2565b9150509250925092565b600060208284031215614ca257600080fd5b81356001600160401b03811115614cb857600080fd5b8201610120818503121561137157600080fd5b600060208284031215614cdd57600080fd5b813560ff8116811461137157600080fd5b600080600060608486031215614d0357600080fd5b8335614d0e81614134565b925060208401356001600160401b03811115614d2957600080fd5b614d35868287016145d2565b9250506040840135614d4681614193565b809150509250925092565b6020808252825182820181905260009190848201906040850190845b818110156146c857835183529284019291840191600101614d6d565b600080600060408486031215614d9e57600080fd5b8335614da981614193565b925060208401356001600160401b03811115614dc457600080fd5b614dd0868287016146d4565b9497909650939450505050565b600080600080600060808688031215614df557600080fd5b8535945060208601356001600160401b0380821115614e1357600080fd5b614e1f89838a016146d4565b909650945060408801359150614e3482614193565b90925060608701359080821115614e4a57600080fd5b50614e5788828901614af2565b9150509295509295909350565b600081518084526020808501945080840160005b838110156144095781516001600160601b031687529582019590820190600101614e78565b6040815260008351604080840152614eb86080840182614e64565b90506020850151603f19848303016060850152614ed58282614e64565b925050508260208301529392505050565b80356003811061418e57600080fd5b60008060408385031215614f0857600080fd5b6141cc83614ee6565b600060208284031215614f2357600080fd5b61137182614ee6565b60008060008060808587031215614f4257600080fd5b84359350614f526020860161417f565b92506040850135614f6281614193565b91506060850135614f7281614193565b939692955090935050565b634e487b7160e01b600052602160045260246000fd5b6020810160058310614fa757614fa7614f7d565b91905290565b600080600060608486031215614fc257600080fd5b8335614fcd81614134565b9250602084013591506040840135614d4681614193565b828152604060208201526000614ffd60408301846144de565b949350505050565b60008060008060008060008060006101208a8c03121561502457600080fd5b893561502f81614134565b985060208a013561503f81614134565b975060408a013561504f81614134565b965060608a013561505f81614134565b955060808a013561506f816141e7565b945060a08a013561507f81614134565b935060c08a013561508f81614193565b925060e08a013561509f81614134565b91506101008a01356150b081614134565b809150509295985092959850929598565b60008060008385036101408112156150d857600080fd5b84356001600160401b03808211156150ef57600080fd5b6150fb888389016140bc565b9550610100601f198401121561511057600080fd5b60208701945061012087013592508083111561512b57600080fd5b5050614c8686828701614af2565b6000808335601e1984360301811261515057600080fd5b83016020810192503590506001600160401b0381111561516f57600080fd5b80360383131561471557600080fd5b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b60208152600082356151b881614193565b63ffffffff811660208401525060208301356151d381614193565b63ffffffff81166040840152506151ec604084016141a5565b63ffffffff81166060840152506152066060840184615139565b60e0608085015261521c6101008501828461517e565b91505061522b608085016141a5565b63ffffffff811660a08501525061524560a0850185615139565b848303601f190160c086015261525c83828461517e565b9250505061526c60c085016141a5565b63ffffffff811660e08501525b509392505050565b60006020828403121561529357600080fd5b813561137181614193565b80356152a981614193565b63ffffffff16825260208181013590830152604090810135910152565b63ffffffff81511682526020810151602083015260006040820151608060408501526152f56080850182614e64565b9050606083015184820360608601526148e28282614e64565b615318818461529e565b608060608201526000614ffd60808301846152c6565b6060810161401d828461529e565b60006020828403121561534e57600080fd5b815161137181614134565b6020808252602a908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526939903ab73830bab9b2b960b11b606082015260800190565b6000602082840312156153b557600080fd5b8151611371816141e7565b60208082526028908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526739903830bab9b2b960c11b606082015260800190565b602080825260059082015264417574683160d81b604082015260600190565b600181811c9082168061543b57607f821691505b6020821081036140ce57634e487b7160e01b600052602260045260246000fd5b634e487b7160e01b600052603260045260246000fd5b60006020828403121561548357600080fd5b5051919050565b634e487b7160e01b600052601160045260246000fd5b6000600182016154b2576154b261548a565b5060010190565b600060208083850312156154cc57600080fd5b82516001600160401b038111156154e257600080fd5b8301601f810185136154f357600080fd5b805161550161437f82614303565b81815260059190911b8201830190838101908783111561552057600080fd5b928401925b8284101561553e57835182529284019290840190615525565b979650505050505050565b80516001600160601b038116811461418e57600080fd5b60006020828403121561557257600080fd5b61137182615549565b6020808252601c908201527f5061757361626c653a20636f6e74726163742069732070617573656400000000604082015260600190565b63ffffffff84168152604060208201819052810182905260006001600160fb1b038311156155df57600080fd5b8260051b8085606085013760009201606001918252509392505050565b6000602080838503121561560f57600080fd5b82516001600160401b0381111561562557600080fd5b8301601f8101851361563657600080fd5b805161564461437f82614303565b81815260059190911b8201830190838101908783111561566357600080fd5b928401925b8284101561553e57835161567b81614193565b82529284019290840190615668565b63ffffffff841681526040602082015260006148e260408301848661517e565b6000602082840312156156bc57600080fd5b81516001600160c01b038116811461137157600080fd5b6000602082840312156156e557600080fd5b815161137181614193565b600060ff821660ff81036157065761570661548a565b60010192915050565b60408152600061572360408301858761517e565b905063ffffffff83166020830152949350505050565b6000808335601e1984360301811261575057600080fd5b8301803591506001600160401b0382111561576a57600080fd5b60200191503681900382131561471557600080fd5b63ffffffff83168152604060208201526000614ffd60408301846143d9565b600081518084526020808501945080840160005b83811015614409576157cf87835180518252602090810151910152565b60409690960195908201906001016157b2565b8060005b600281101561292c5781518452602093840193909101906001016157e6565b6158108282516157e2565b6020810151611eb960408401826157e2565b85815260806020820152600061583c60808301868861517e565b63ffffffff8516604084015282810360608401526101808451818352615864828401826147df565b9150506020850151828203602084015261587e828261579e565b91505060408501518282036040840152615898828261579e565b91505060608501516158ad6060840182615805565b506080850151805160e08401526020015161010083015260a08501518282036101208401526158dc82826147df565b91505060c08501518282036101408401526158f782826147df565b91505060e08501518282036101608401526159128282614815565b9a9950505050505050505050565b600082601f83011261593157600080fd5b8151602061594161437f83614303565b82815260059290921b8401810191818101908684111561596057600080fd5b8286015b8481101561462d5761597581615549565b8352918301918301615964565b6000806040838503121561599557600080fd5b82516001600160401b03808211156159ac57600080fd5b90840190604082870312156159c057600080fd5b6159c8614288565b8251828111156159d757600080fd5b6159e388828601615920565b8252506020830151828111156159f857600080fd5b615a0488828601615920565b602083015250809450505050602083015190509250929050565b60038110615a2e57615a2e614f7d565b9052565b60208152600063ffffffff8084511660208401526020840151615a586040850182615a1e565b508060408501511660608401528060608501511660808401528060808501511660a084015260a084015160e060c0850152615a97610100850182614212565b90508160c08601511660e0850152809250505092915050565b600063ffffffff808316818516808303821115615acf57615acf61548a565b01949350505050565b6020815260008235615ae981614193565b63ffffffff8082166020850152615b0260208601614ee6565b9150615b116040850183615a1e565b60408501359150615b2182614193565b808216606085015260608501359150615b3982614193565b16608083810191909152830135615b4f81614193565b63ffffffff811660a084015250615b6960a0840184615139565b60e060c0850152615b7f6101008501828461517e565b91505061526c60c085016141a5565b8035615b9981614193565b63ffffffff808216845260208301356020850152615bb960408401614ee6565b9150615bc86040850183615a1e565b60608301359150615bd882614193565b1660608301526080818101359083015260a0808201359083015260c0808201359083015260e0810135615c0a81614134565b6001600160a01b031660e0929092019190915250565b610100810161401d8284615b8e565b6000610120615c3e8386615b8e565b806101008401526148e2818401856152c6565b82815260608101611371602083018480518252602090810151910152565b600063ffffffff83811690831681811015615c8c57615c8c61548a565b039392505050565b60006001600160601b0380831681851681830481118215151615615cba57615cba61548a565b02949350505050565b6000816000190483118215151615615cdd57615cdd61548a565b500290565b60208152600063ffffffff80845116602084015280602085015116604084015280604085015116606084015250606083015160e06080840152615d29610100840182614212565b90506080840151615d4260a085018263ffffffff169052565b5060a0840151838203601f190160c0850152615d5e8282614212565b91505060c084015161527960e085018263ffffffff169052565b600082821015615d8a57615d8a61548a565b500390565b600061ffff808316818103615da657615da661548a565b600101939250505056fea26469706673582212206bdf9cadaf427f77bfb4ae87937357e4356619f2ec286ee673879fc355d0383464736f6c634300080d0033",
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

// ValidateTaskResponse is a free data retrieval call binding the contract method 0xb1edc8b4.
//
// Solidity: function _validateTaskResponse(bytes32 taskHash, uint8 taskType, uint32 referenceTaskIndex, uint32 taskCreatedBlock) view returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) ValidateTaskResponse(opts *bind.CallOpts, taskHash [32]byte, taskType uint8, referenceTaskIndex uint32, taskCreatedBlock uint32) error {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "_validateTaskResponse", taskHash, taskType, referenceTaskIndex, taskCreatedBlock)

	if err != nil {
		return err
	}

	return err

}

// ValidateTaskResponse is a free data retrieval call binding the contract method 0xb1edc8b4.
//
// Solidity: function _validateTaskResponse(bytes32 taskHash, uint8 taskType, uint32 referenceTaskIndex, uint32 taskCreatedBlock) view returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) ValidateTaskResponse(taskHash [32]byte, taskType uint8, referenceTaskIndex uint32, taskCreatedBlock uint32) error {
	return _ContractFinalizerTaskManager.Contract.ValidateTaskResponse(&_ContractFinalizerTaskManager.CallOpts, taskHash, taskType, referenceTaskIndex, taskCreatedBlock)
}

// ValidateTaskResponse is a free data retrieval call binding the contract method 0xb1edc8b4.
//
// Solidity: function _validateTaskResponse(bytes32 taskHash, uint8 taskType, uint32 referenceTaskIndex, uint32 taskCreatedBlock) view returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) ValidateTaskResponse(taskHash [32]byte, taskType uint8, referenceTaskIndex uint32, taskCreatedBlock uint32) error {
	return _ContractFinalizerTaskManager.Contract.ValidateTaskResponse(&_ContractFinalizerTaskManager.CallOpts, taskHash, taskType, referenceTaskIndex, taskCreatedBlock)
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

// GetBatchOperatorFromId is a free data retrieval call binding the contract method 0x4d2b57fe.
//
// Solidity: function getBatchOperatorFromId(address registryCoordinator, bytes32[] operatorIds) view returns(address[] operators)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) GetBatchOperatorFromId(opts *bind.CallOpts, registryCoordinator common.Address, operatorIds [][32]byte) ([]common.Address, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "getBatchOperatorFromId", registryCoordinator, operatorIds)

	if err != nil {
		return *new([]common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new([]common.Address)).(*[]common.Address)

	return out0, err

}

// GetBatchOperatorFromId is a free data retrieval call binding the contract method 0x4d2b57fe.
//
// Solidity: function getBatchOperatorFromId(address registryCoordinator, bytes32[] operatorIds) view returns(address[] operators)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) GetBatchOperatorFromId(registryCoordinator common.Address, operatorIds [][32]byte) ([]common.Address, error) {
	return _ContractFinalizerTaskManager.Contract.GetBatchOperatorFromId(&_ContractFinalizerTaskManager.CallOpts, registryCoordinator, operatorIds)
}

// GetBatchOperatorFromId is a free data retrieval call binding the contract method 0x4d2b57fe.
//
// Solidity: function getBatchOperatorFromId(address registryCoordinator, bytes32[] operatorIds) view returns(address[] operators)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) GetBatchOperatorFromId(registryCoordinator common.Address, operatorIds [][32]byte) ([]common.Address, error) {
	return _ContractFinalizerTaskManager.Contract.GetBatchOperatorFromId(&_ContractFinalizerTaskManager.CallOpts, registryCoordinator, operatorIds)
}

// GetBatchOperatorId is a free data retrieval call binding the contract method 0x31b36bd9.
//
// Solidity: function getBatchOperatorId(address registryCoordinator, address[] operators) view returns(bytes32[] operatorIds)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) GetBatchOperatorId(opts *bind.CallOpts, registryCoordinator common.Address, operators []common.Address) ([][32]byte, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "getBatchOperatorId", registryCoordinator, operators)

	if err != nil {
		return *new([][32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([][32]byte)).(*[][32]byte)

	return out0, err

}

// GetBatchOperatorId is a free data retrieval call binding the contract method 0x31b36bd9.
//
// Solidity: function getBatchOperatorId(address registryCoordinator, address[] operators) view returns(bytes32[] operatorIds)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) GetBatchOperatorId(registryCoordinator common.Address, operators []common.Address) ([][32]byte, error) {
	return _ContractFinalizerTaskManager.Contract.GetBatchOperatorId(&_ContractFinalizerTaskManager.CallOpts, registryCoordinator, operators)
}

// GetBatchOperatorId is a free data retrieval call binding the contract method 0x31b36bd9.
//
// Solidity: function getBatchOperatorId(address registryCoordinator, address[] operators) view returns(bytes32[] operatorIds)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) GetBatchOperatorId(registryCoordinator common.Address, operators []common.Address) ([][32]byte, error) {
	return _ContractFinalizerTaskManager.Contract.GetBatchOperatorId(&_ContractFinalizerTaskManager.CallOpts, registryCoordinator, operators)
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

// LastRdTaskCreatedBlock is a free data retrieval call binding the contract method 0x8c82af5e.
//
// Solidity: function lastRdTaskCreatedBlock() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) LastRdTaskCreatedBlock(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "lastRdTaskCreatedBlock")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// LastRdTaskCreatedBlock is a free data retrieval call binding the contract method 0x8c82af5e.
//
// Solidity: function lastRdTaskCreatedBlock() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) LastRdTaskCreatedBlock() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.LastRdTaskCreatedBlock(&_ContractFinalizerTaskManager.CallOpts)
}

// LastRdTaskCreatedBlock is a free data retrieval call binding the contract method 0x8c82af5e.
//
// Solidity: function lastRdTaskCreatedBlock() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) LastRdTaskCreatedBlock() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.LastRdTaskCreatedBlock(&_ContractFinalizerTaskManager.CallOpts)
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

// CancelPendingTasks is a paid mutator transaction binding the contract method 0x21e78062.
//
// Solidity: function cancelPendingTasks() returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) CancelPendingTasks(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "cancelPendingTasks")
}

// CancelPendingTasks is a paid mutator transaction binding the contract method 0x21e78062.
//
// Solidity: function cancelPendingTasks() returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) CancelPendingTasks() (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.CancelPendingTasks(&_ContractFinalizerTaskManager.TransactOpts)
}

// CancelPendingTasks is a paid mutator transaction binding the contract method 0x21e78062.
//
// Solidity: function cancelPendingTasks() returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) CancelPendingTasks() (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.CancelPendingTasks(&_ContractFinalizerTaskManager.TransactOpts)
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

// SetAggregator is a paid mutator transaction binding the contract method 0xf9120af6.
//
// Solidity: function setAggregator(address _aggregator) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) SetAggregator(opts *bind.TransactOpts, _aggregator common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "setAggregator", _aggregator)
}

// SetAggregator is a paid mutator transaction binding the contract method 0xf9120af6.
//
// Solidity: function setAggregator(address _aggregator) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) SetAggregator(_aggregator common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.SetAggregator(&_ContractFinalizerTaskManager.TransactOpts, _aggregator)
}

// SetAggregator is a paid mutator transaction binding the contract method 0xf9120af6.
//
// Solidity: function setAggregator(address _aggregator) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) SetAggregator(_aggregator common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.SetAggregator(&_ContractFinalizerTaskManager.TransactOpts, _aggregator)
}

// SetGenerator is a paid mutator transaction binding the contract method 0x4a7c7e4b.
//
// Solidity: function setGenerator(address _generator) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) SetGenerator(opts *bind.TransactOpts, _generator common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "setGenerator", _generator)
}

// SetGenerator is a paid mutator transaction binding the contract method 0x4a7c7e4b.
//
// Solidity: function setGenerator(address _generator) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) SetGenerator(_generator common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.SetGenerator(&_ContractFinalizerTaskManager.TransactOpts, _generator)
}

// SetGenerator is a paid mutator transaction binding the contract method 0x4a7c7e4b.
//
// Solidity: function setGenerator(address _generator) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) SetGenerator(_generator common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.SetGenerator(&_ContractFinalizerTaskManager.TransactOpts, _generator)
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

// ContractFinalizerTaskManagerAggregatorUpdatedIterator is returned from FilterAggregatorUpdated and is used to iterate over the raw logs and unpacked data for AggregatorUpdated events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerAggregatorUpdatedIterator struct {
	Event *ContractFinalizerTaskManagerAggregatorUpdated // Event containing the contract specifics and raw log

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
func (it *ContractFinalizerTaskManagerAggregatorUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerAggregatorUpdated)
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
		it.Event = new(ContractFinalizerTaskManagerAggregatorUpdated)
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
func (it *ContractFinalizerTaskManagerAggregatorUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerAggregatorUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerAggregatorUpdated represents a AggregatorUpdated event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerAggregatorUpdated struct {
	AggregatorAddress common.Address
	Raw               types.Log // Blockchain specific contextual infos
}

// FilterAggregatorUpdated is a free log retrieval operation binding the contract event 0x602cec4b1583b07d071161da5eb9589444d2459201e2fab7753dc941e9351c21.
//
// Solidity: event AggregatorUpdated(address aggregatorAddress)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterAggregatorUpdated(opts *bind.FilterOpts) (*ContractFinalizerTaskManagerAggregatorUpdatedIterator, error) {

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "AggregatorUpdated")
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerAggregatorUpdatedIterator{contract: _ContractFinalizerTaskManager.contract, event: "AggregatorUpdated", logs: logs, sub: sub}, nil
}

// WatchAggregatorUpdated is a free log subscription operation binding the contract event 0x602cec4b1583b07d071161da5eb9589444d2459201e2fab7753dc941e9351c21.
//
// Solidity: event AggregatorUpdated(address aggregatorAddress)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchAggregatorUpdated(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerAggregatorUpdated) (event.Subscription, error) {

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "AggregatorUpdated")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerAggregatorUpdated)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "AggregatorUpdated", log); err != nil {
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

// ParseAggregatorUpdated is a log parse operation binding the contract event 0x602cec4b1583b07d071161da5eb9589444d2459201e2fab7753dc941e9351c21.
//
// Solidity: event AggregatorUpdated(address aggregatorAddress)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParseAggregatorUpdated(log types.Log) (*ContractFinalizerTaskManagerAggregatorUpdated, error) {
	event := new(ContractFinalizerTaskManagerAggregatorUpdated)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "AggregatorUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
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

// ContractFinalizerTaskManagerGeneratorUpdatedIterator is returned from FilterGeneratorUpdated and is used to iterate over the raw logs and unpacked data for GeneratorUpdated events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerGeneratorUpdatedIterator struct {
	Event *ContractFinalizerTaskManagerGeneratorUpdated // Event containing the contract specifics and raw log

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
func (it *ContractFinalizerTaskManagerGeneratorUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerGeneratorUpdated)
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
		it.Event = new(ContractFinalizerTaskManagerGeneratorUpdated)
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
func (it *ContractFinalizerTaskManagerGeneratorUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerGeneratorUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerGeneratorUpdated represents a GeneratorUpdated event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerGeneratorUpdated struct {
	GeneratorAddress common.Address
	Raw              types.Log // Blockchain specific contextual infos
}

// FilterGeneratorUpdated is a free log retrieval operation binding the contract event 0x60f5ac48a13f8b5bf4b213de190dd2de2592c9f6f5ac7dc14e3d726b95ded2da.
//
// Solidity: event GeneratorUpdated(address generatorAddress)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterGeneratorUpdated(opts *bind.FilterOpts) (*ContractFinalizerTaskManagerGeneratorUpdatedIterator, error) {

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "GeneratorUpdated")
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerGeneratorUpdatedIterator{contract: _ContractFinalizerTaskManager.contract, event: "GeneratorUpdated", logs: logs, sub: sub}, nil
}

// WatchGeneratorUpdated is a free log subscription operation binding the contract event 0x60f5ac48a13f8b5bf4b213de190dd2de2592c9f6f5ac7dc14e3d726b95ded2da.
//
// Solidity: event GeneratorUpdated(address generatorAddress)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchGeneratorUpdated(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerGeneratorUpdated) (event.Subscription, error) {

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "GeneratorUpdated")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerGeneratorUpdated)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "GeneratorUpdated", log); err != nil {
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

// ParseGeneratorUpdated is a log parse operation binding the contract event 0x60f5ac48a13f8b5bf4b213de190dd2de2592c9f6f5ac7dc14e3d726b95ded2da.
//
// Solidity: event GeneratorUpdated(address generatorAddress)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParseGeneratorUpdated(log types.Log) (*ContractFinalizerTaskManagerGeneratorUpdated, error) {
	event := new(ContractFinalizerTaskManagerGeneratorUpdated)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "GeneratorUpdated", log); err != nil {
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

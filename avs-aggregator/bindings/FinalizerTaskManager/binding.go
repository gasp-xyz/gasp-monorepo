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
	ABI: "[{\"type\":\"function\",\"name\":\"THRESHOLD_DENOMINATOR\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"aggregator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allTaskHashes\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumIFinalizerTaskManager.TaskType\"},{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allTaskResponses\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumIFinalizerTaskManager.TaskType\"},{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allowNonRootInit\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"blsApkRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIBLSApkRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"blsSignatureChecker\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractBLSSignatureChecker\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"chainRdBatchNonce\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"checkSignatures\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"referenceBlockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.QuorumStakeTotals\",\"components\":[{\"name\":\"signedStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"totalStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"createNewOpTask\",\"inputs\":[{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"createNewRdTask\",\"inputs\":[{\"name\":\"chainId\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"},{\"name\":\"batchId\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"delegation\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIDelegationManager\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"dummyForOperatorStateInfoType\",\"inputs\":[{\"name\":\"_operatorStateInfo\",\"type\":\"tuple\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorStateInfo\",\"components\":[{\"name\":\"operatorsStateChanged\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"quorumsRemoved\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsAdded[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"quorumsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsStakeUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]},{\"name\":\"quorumsApkUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsApkUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"operatorsRemoved\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"operatorsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsAdded[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]},{\"name\":\"operatorsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsStakeUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"operatorsQuorumCountUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsQuorumCountUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]}]}],\"outputs\":[],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"dummyForQuorumStakeTotalsType\",\"inputs\":[{\"name\":\"_quorumStakeTotals\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.QuorumStakeTotals\",\"components\":[{\"name\":\"signedStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"totalStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]}],\"outputs\":[],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"forceCancelPendingTasks\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"forceCreateNewOpTask\",\"inputs\":[{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"forceRespondToOpTask\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"generator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getBatchOperatorFromId\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"}],\"outputs\":[{\"name\":\"operators\",\"type\":\"address[]\",\"internalType\":\"address[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getBatchOperatorId\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operators\",\"type\":\"address[]\",\"internalType\":\"address[]\"}],\"outputs\":[{\"name\":\"operatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getCheckSignaturesIndices\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"referenceBlockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"nonSignerOperatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structOperatorStateRetriever.CheckSignaturesIndices\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getOperatorState\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple[][]\",\"internalType\":\"structOperatorStateRetriever.Operator[][]\",\"components\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"stake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getOperatorState\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"\",\"type\":\"tuple[][]\",\"internalType\":\"structOperatorStateRetriever.Operator[][]\",\"components\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"stake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getQuorumBitmapsAtBlockNumber\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256[]\",\"internalType\":\"uint256[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"idToTaskStatus\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumIFinalizerTaskManager.TaskType\"},{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumIFinalizerTaskManager.TaskStatus\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"_pauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"initialOwner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_aggregator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_generator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_allowNonRootInit\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"_blsSignatureCheckerAddress\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_taskResponseWindowBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"_operatorStateRetrieverExtended\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_rolldown\",\"type\":\"address\",\"internalType\":\"contractIRolldown\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"isTaskPending\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastCompletedOpTaskCreatedBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastCompletedOpTaskNum\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastOpTaskCreatedBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastRdTaskCreatedBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestOpTaskNum\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestRdTaskNum\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"operatorStateRetrieverExtended\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"operatorsStateInfoHash\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"pauseAll\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"pauseTrackingOpState\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[{\"name\":\"index\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pauserRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"registryCoordinator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"respondToOpTask\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"respondToRdTask\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.RdTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"chainId\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"},{\"name\":\"batchId\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.RdTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"chainId\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"},{\"name\":\"batchId\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"rdUpdate\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"rangeStart\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"rangeEnd\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"updater\",\"type\":\"address\",\"internalType\":\"address\"}]},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"resumeTrackingQuorums\",\"inputs\":[{\"name\":\"resetTrackedQuorums\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"rolldown\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIRolldown\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"setPauserRegistry\",\"inputs\":[{\"name\":\"newPauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setRolldown\",\"inputs\":[{\"name\":\"_rolldown\",\"type\":\"address\",\"internalType\":\"contractIRolldown\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"stakeRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIStakeRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"taskResponseWindowBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"unpause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateBlsSignatureCheckerAddress\",\"inputs\":[{\"name\":\"_blsSignatureCheckerAddress\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"validateTaskResponse\",\"inputs\":[{\"name\":\"taskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"taskType\",\"type\":\"uint8\",\"internalType\":\"enumIFinalizerTaskManager.TaskType\"},{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[],\"stateMutability\":\"view\"},{\"type\":\"event\",\"name\":\"BLSSignatureCheckerAddressUpdated\",\"inputs\":[{\"name\":\"blsSignatureCheckerAddress\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Initialized\",\"inputs\":[{\"name\":\"version\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewOpTaskCreated\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"task\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.OpTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewOpTaskForceCreated\",\"inputs\":[],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewRdTaskCreated\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"task\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.RdTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"chainId\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"},{\"name\":\"batchId\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OpTaskCancelled\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OpTaskCompleted\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OpTaskForceCompleted\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OpTaskResponded\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"taskResponseMetadata\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.TaskResponseMetadata\",\"components\":[{\"name\":\"taskResponsedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"hashOfNonSigners\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quroumStakeTotals\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quroumStakeSigned\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"PauseTrackingOpState\",\"inputs\":[],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Paused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"PauserRegistrySet\",\"inputs\":[{\"name\":\"pauserRegistry\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"newPauserRegistry\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractIPauserRegistry\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RdTaskCancelled\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RdTaskCompleted\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.RdTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"chainId\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"},{\"name\":\"batchId\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"rdUpdate\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"rangeStart\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"rangeEnd\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"updater\",\"type\":\"address\",\"internalType\":\"address\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RdTaskResponded\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.RdTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"chainId\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"},{\"name\":\"batchId\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"rdUpdate\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"rangeStart\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"rangeEnd\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"updater\",\"type\":\"address\",\"internalType\":\"address\"}]},{\"name\":\"taskResponseMetadata\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.TaskResponseMetadata\",\"components\":[{\"name\":\"taskResponsedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"hashOfNonSigners\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quroumStakeTotals\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quroumStakeSigned\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"ResumeTrackingOpState\",\"inputs\":[{\"name\":\"resetTrackedQuorums\",\"type\":\"bool\",\"indexed\":false,\"internalType\":\"bool\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RolldownTargetUpdated\",\"inputs\":[{\"name\":\"rolldownAddress\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"StaleStakesForbiddenUpdate\",\"inputs\":[{\"name\":\"value\",\"type\":\"bool\",\"indexed\":false,\"internalType\":\"bool\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Unpaused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false}]",
	Bin: "0x608060405234801561001057600080fd5b50615f8180620000216000396000f3fe608060405234801561001057600080fd5b506004361061038e5760003560e01c806368304835116101de5780638fc8729a1161010f578063df5cf723116100ad578063f2fde38b1161007c578063f2fde38b14610856578063f5640cf814610869578063fabc1cbc1461087c578063fdc15de81461088f57600080fd5b8063df5cf72314610823578063e70c26231461082b578063e72ddf101461083b578063ef0244581461084e57600080fd5b8063adfcb048116100e9578063adfcb048146107ab578063bf2315ed146107b4578063cefdc1d4146107ef578063de4348381461081057600080fd5b80638fc8729a14610757578063930390d91461076e578063a69563a91461079457600080fd5b806379badf731161017c5780638380acbd116101565780638380acbd14610708578063886f11951461071b5780638c82af5e1461072e5780638da5cb5b1461074657600080fd5b806379badf73146106d65780637afa1eed146106de5780637afdd54b146106f157600080fd5b80636efb4636116101b85780636efb4636146106875780636f254819146106a8578063715018a6146106bb578063723114ab146106c357600080fd5b806368304835146106645780636d14a9871461066c5780636e125ff41461067457600080fd5b80633d9fb00c116102c3578063537a2929116102615780635c155662116102305780635c1556621461062c5780635c975abb1461064c5780635df459461461065457806360202fc01461065c57600080fd5b8063537a2929146105dc57806354d127de146105f3578063595c6a67146106015780635ac86ab71461060957600080fd5b80634d2b57fe1161029d5780634d2b57fe146105795780634d7a7116146105995780634f739f74146105a9578063516a7227146105c957600080fd5b80633d9fb00c1461052957806341789d571461053c57806345265b7a1461056857600080fd5b80631c178e9c1161033057806331b36bd91161030a57806331b36bd9146104c25780633563b0d1146104e257806336f78ed81461050257806337d347d11461051657600080fd5b80631c178e9c14610467578063245a7bfc146104925780632830e8f9146104ad57600080fd5b8063136439dd1161036c578063136439dd146103dd57806313f815ed146103f0578063191aac7a146104295780631ac272971461043c57600080fd5b806301a3f013146103935780630ee0fdbd146103a857806310d67a2f146103ca575b600080fd5b6103a66103a13660046142bb565b6108a2565b005b60a3546103b59060ff1681565b60405190151581526020015b60405180910390f35b6103a66103d836600461431e565b610b6d565b6103a66103eb36600461433b565b610c26565b61041b6103fe366004614383565b609960209081526000928352604080842090915290825290205481565b6040519081526020016103c1565b6103a66104373660046143ca565b610d65565b61041b61044a366004614383565b609a60209081526000928352604080842090915290825290205481565b60975461047a906001600160a01b031681565b6040516001600160a01b0390911681526020016103c1565b609e5461047a9064010000000090046001600160a01b031681565b6104b5610da6565b6040516103c19190614434565b6104d56104d03660046144fb565b610e34565b6040516103c191906145e9565b6104f56104f03660046145fc565b610f50565b6040516103c19190614757565b60a0546103b590600160a01b900460ff1681565b6103a661052436600461476a565b6113e8565b60a05461047a906001600160a01b031681565b60985461055390600160c01b900463ffffffff1681565b60405163ffffffff90911681526020016103c1565b6103a66105763660046147bd565b50565b61058c61058736600461485d565b61165c565b6040516103c191906148ac565b609c546105539063ffffffff1681565b6105bc6105b7366004614941565b611771565b6040516103c19190614a8f565b6103a66105d7366004614e3e565b611e97565b609c5461055390600160601b900463ffffffff1681565b6103a6610576366004614eb2565b6103a66123ac565b6103b5610617366004614eed565b606654600160ff9092169190911b9081161490565b61063f61063a366004614f10565b612473565b6040516103c19190614f73565b60665461041b565b61047a61263b565b6103a66126ae565b61047a612710565b61047a61275a565b6103a6610682366004614fab565b6127a4565b61069a610695366004614fff565b612847565b6040516103c19291906150bf565b6103a66106b6366004614383565b6128e7565b6103a6612bf8565b6103a66106d136600461431e565b612c0a565b6103a6612c60565b609f5461047a906001600160a01b031681565b60985461055390600160e01b900463ffffffff1681565b60985461047a906001600160a01b031681565b60655461047a906001600160a01b031681565b609c5461055390640100000000900463ffffffff1681565b6033546001600160a01b031661047a565b609c5461055390600160401b900463ffffffff1681565b61055361077c366004615108565b60a16020526000908152604090205463ffffffff1681565b60985461055390600160a01b900463ffffffff1681565b61041b60a25481565b6107e26107c2366004614383565b609b60209081526000928352604080842090915290825290205460ff1681565b6040516103c1919061513b565b6108026107fd366004615155565b612c93565b6040516103c192919061518c565b6103a661081e3660046151ad565b612e25565b61047a612fde565b609e546105539063ffffffff1681565b6103a6610849366004615269565b613028565b61041b606481565b6103a661086436600461431e565b613699565b6103a6610877366004614fab565b61370f565b6103a661088a36600461433b565b61376a565b6103a661089d36600461431e565b6138c6565b6108aa61396c565b6108f6826040516020016108be919061534f565b60408051601f198184030181529190528051602091820120906000906108e690850185615429565b6105246040870160208801615429565b60408051808201909152606080825260208201526040805160808101825263ffffffff4316815260006020808301829052848101518385015284516060840152925190926109489186918491016154b6565b60408051601f1981840301815291905280516020918201206000808052609a835290917fbe6620bd3346e5d7f8387fbec0981aa0d6289d22efa7c935f9ef6841bf2a98c7919061099a90880188615429565b63ffffffff168152602080820192909252604090810160009081209390935586013560a255818052609b8152600491600080516020615f0c833981519152916109e590880188615429565b63ffffffff1681526020810191909152604001600020805460ff19166001836004811115610a1557610a15615125565b0217905550610a2a6040860160208701615429565b609c805463ffffffff92909216600160601b0263ffffffff60601b19909216919091179055610a5c60608601866154d6565b610a6891609d916141f8565b50610a7960a0860160808701615429565b609e805463ffffffff191663ffffffff92909216919091179055610aa06020860186615429565b609c805463ffffffff92909216600160401b0263ffffffff60401b1990921691909117905560a0805460ff60a01b19169055610adf6020850185615429565b63ffffffff167fff2908483d74b6b70053dd473260acf1b09e0ba0781bf94100bb8277581749de85604051610b14919061551c565b60405180910390a2610b296020850185615429565b63ffffffff167fdf22f3558e4841b63d77179546b3eae63e4e343bbe752746b093162bc526be4c85604051610b5e919061551c565b60405180910390a25050505050565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610bc0573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610be4919061552a565b6001600160a01b0316336001600160a01b031614610c1d5760405162461bcd60e51b8152600401610c1490615547565b60405180910390fd5b610576816139c6565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa158015610c6e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c929190615591565b610cae5760405162461bcd60e51b8152600401610c14906155ae565b60665481811614610d275760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e70617573653a20696e76616c696420617474656d70742060448201527f746f20756e70617573652066756e6374696f6e616c69747900000000000000006064820152608401610c14565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b610d6d61396c565b60405181151581527f6af4ae1f481aff20ce571abd65375b67b22359883a823d1ddf4bd8f2879ff7ba906020015b60405180910390a150565b609d8054610db3906155f6565b80601f0160208091040260200160405190810160405280929190818152602001828054610ddf906155f6565b8015610e2c5780601f10610e0157610100808354040283529160200191610e2c565b820191906000526020600020905b815481529060010190602001808311610e0f57829003601f168201915b505050505081565b606081516001600160401b03811115610e4f57610e4f614447565b604051908082528060200260200182016040528015610e78578160200160208202803683370190505b50905060005b8251811015610f4957836001600160a01b03166313542a4e848381518110610ea857610ea861562b565b60200260200101516040518263ffffffff1660e01b8152600401610edb91906001600160a01b0391909116815260200190565b602060405180830381865afa158015610ef8573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610f1c9190615641565b828281518110610f2e57610f2e61562b565b6020908102919091010152610f4281615670565b9050610e7e565b5092915050565b60606000846001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa158015610f92573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610fb6919061552a565b90506000856001600160a01b0316639e9923c26040518163ffffffff1660e01b8152600401602060405180830381865afa158015610ff8573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061101c919061552a565b90506000866001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa15801561105e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611082919061552a565b9050600086516001600160401b0381111561109f5761109f614447565b6040519080825280602002602001820160405280156110d257816020015b60608152602001906001900390816110bd5790505b50905060005b87518110156113da5760008882815181106110f5576110f561562b565b0160200151604051638902624560e01b815260f89190911c6004820181905263ffffffff8a16602483015291506000906001600160a01b03871690638902624590604401600060405180830381865afa158015611156573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261117e919081019061568b565b905080516001600160401b0381111561119957611199614447565b6040519080825280602002602001820160405280156111e457816020015b60408051606081018252600080825260208083018290529282015282526000199092019101816111b75790505b508484815181106111f7576111f761562b565b602002602001018190525060005b81518110156113c4576040518060600160405280876001600160a01b03166347b314e885858151811061123a5761123a61562b565b60200260200101516040518263ffffffff1660e01b815260040161126091815260200190565b602060405180830381865afa15801561127d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906112a1919061552a565b6001600160a01b031681526020018383815181106112c1576112c161562b565b60200260200101518152602001896001600160a01b031663fa28c6278585815181106112ef576112ef61562b565b60209081029190910101516040516001600160e01b031960e084901b168152600481019190915260ff8816602482015263ffffffff8f166044820152606401602060405180830381865afa15801561134b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061136f9190615732565b6001600160601b031681525085858151811061138d5761138d61562b565b602002602001015182815181106113a6576113a661562b565b602002602001018190525080806113bc90615670565b915050611205565b50505080806113d290615670565b9150506110d8565b5093505050505b9392505050565b60a054600160a01b900460ff1615156001146114385760405162461bcd60e51b815260206004820152600f60248201526e4e6f207461736b2070656e64696e6760881b6044820152606401610c14565b6099600084600181111561144e5761144e615125565b600181111561145f5761145f615125565b815260200190815260200160002060008363ffffffff1663ffffffff1681526020019081526020016000205484146114c95760405162461bcd60e51b815260206004820152600d60248201526c0a8c2e6d640dad2e6dac2e8c6d609b1b6044820152606401610c14565b6001609b60008560018111156114e1576114e1615125565b60018111156114f2576114f2615125565b81526020808201929092526040908101600090812063ffffffff8716825290925290205460ff16600481111561152a5761152a615125565b146115685760405162461bcd60e51b815260206004820152600e60248201526d4e6f7420496e697420737461746560901b6044820152606401610c14565b6000609a8185600181111561157f5761157f615125565b600181111561159057611590615125565b815260200190815260200160002060008463ffffffff1663ffffffff16815260200190815260200160002054146115f65760405162461bcd60e51b815260206004820152600a6024820152690416c72647920526573760b41b6044820152606401610c14565b60985461161090600160a01b900463ffffffff168261574d565b63ffffffff164363ffffffff1611156116565760405162461bcd60e51b8152602060048201526008602482015267546f6f206c61746560c01b6044820152606401610c14565b50505050565b606081516001600160401b0381111561167757611677614447565b6040519080825280602002602001820160405280156116a0578160200160208202803683370190505b50905060005b8251811015610f4957836001600160a01b031663296bb0648483815181106116d0576116d061562b565b60200260200101516040518263ffffffff1660e01b81526004016116f691815260200190565b602060405180830381865afa158015611713573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611737919061552a565b8282815181106117495761174961562b565b6001600160a01b039092166020928302919091019091015261176a81615670565b90506116a6565b61179c6040518060800160405280606081526020016060815260200160608152602001606081525090565b6000876001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa1580156117dc573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611800919061552a565b905061182d6040518060800160405280606081526020016060815260200160608152602001606081525090565b6040516361c8a12f60e11b81526001600160a01b038a169063c391425e9061185d908b9089908990600401615775565b600060405180830381865afa15801561187a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526118a291908101906157bf565b81526040516340e03a8160e11b81526001600160a01b038316906381c07502906118d4908b908b908b9060040161584d565b600060405180830381865afa1580156118f1573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261191991908101906157bf565b6040820152856001600160401b0381111561193657611936614447565b60405190808252806020026020018201604052801561196957816020015b60608152602001906001900390816119545790505b50606082015260005b60ff8116871115611da8576000856001600160401b0381111561199757611997614447565b6040519080825280602002602001820160405280156119c0578160200160208202803683370190505b5083606001518360ff16815181106119da576119da61562b565b602002602001018190525060005b86811015611ca85760008c6001600160a01b03166304ec63518a8a85818110611a1357611a1361562b565b905060200201358e88600001518681518110611a3157611a3161562b565b60200260200101516040518463ffffffff1660e01b8152600401611a6e9392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa158015611a8b573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611aaf919061586d565b90506001600160c01b038116611b535760405162461bcd60e51b815260206004820152605c60248201527f4f70657261746f7253746174655265747269657665722e676574436865636b5360448201527f69676e617475726573496e64696365733a206f70657261746f72206d7573742060648201527f6265207265676973746572656420617420626c6f636b6e756d62657200000000608482015260a401610c14565b8a8a8560ff16818110611b6857611b6861562b565b6001600160c01b03841692013560f81c9190911c600190811614159050611c9557856001600160a01b031663dd9846b98a8a85818110611baa57611baa61562b565b905060200201358d8d8860ff16818110611bc657611bc661562b565b6040516001600160e01b031960e087901b1681526004810194909452919091013560f81c60248301525063ffffffff8f166044820152606401602060405180830381865afa158015611c1c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611c409190615896565b85606001518560ff1681518110611c5957611c5961562b565b60200260200101518481518110611c7257611c7261562b565b63ffffffff9092166020928302919091019091015282611c9181615670565b9350505b5080611ca081615670565b9150506119e8565b506000816001600160401b03811115611cc357611cc3614447565b604051908082528060200260200182016040528015611cec578160200160208202803683370190505b50905060005b82811015611d6d5784606001518460ff1681518110611d1357611d1361562b565b60200260200101518181518110611d2c57611d2c61562b565b6020026020010151828281518110611d4657611d4661562b565b63ffffffff9092166020928302919091019091015280611d6581615670565b915050611cf2565b508084606001518460ff1681518110611d8857611d8861562b565b602002602001018190525050508080611da0906158b3565b915050611972565b506000896001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa158015611de9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611e0d919061552a565b60405163354952a360e21b81529091506001600160a01b0382169063d5254a8c90611e40908b908b908e906004016158d3565b600060405180830381865afa158015611e5d573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611e8591908101906157bf565b60208301525098975050505050505050565b609c54600160601b900463ffffffff16151580611eb6575060a35460ff165b611ef25760405162461bcd60e51b815260206004820152600d60248201526c1d5cd9481c9bdbdd081a5b9a5d609a1b6044820152606401610c14565b366000611f0260a08601866154d6565b90925090506000611f1960e0870160c08801615429565b9050611f6786604051602001611f2f919061534f565b60408051601f19818403018152919052805160209182012090600090611f5790890189615429565b61052460408b0160208c01615429565b600085604051602001611f7a919061551c565b60408051601f19818403018152828252805160209182012083830190925260608084529083015291506000609760009054906101000a90046001600160a01b03166001600160a01b0316636efb46368488888d6040016020810190611fdf9190615429565b8c6040518663ffffffff1660e01b8152600401612000959493929190615981565b600060405180830381865afa15801561201d573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526120459190810190615ae1565b6040805160808101825263ffffffff43168152602080820184905280850151828401528451606083015291519395509193509091612087918b918491016154b6565b60408051601f1981840301815291905280516020918201206000808052609a835290917fbe6620bd3346e5d7f8387fbec0981aa0d6289d22efa7c935f9ef6841bf2a98c791906120d9908d018d615429565b63ffffffff168152602080820192909252604001600090812092909255818052609b8152600391600080516020615f0c8339815191529161211c908d018d615429565b63ffffffff1681526020810191909152604001600020805460ff1916600183600481111561214c5761214c615125565b021790555060a0805460ff60a01b1916905561216b60208b018b615429565b63ffffffff167f47adacb0b6bbd726ae39ac6c006cca1c2006c9aedaa882dcba7c4804db7c41ce8a836040516121a29291906154b6565b60405180910390a260005b86811015612242578560ff16846020015182815181106121cf576121cf61562b565b60200260200101516121e19190615b7d565b6001600160601b03166064856000015183815181106122025761220261562b565b60200260200101516001600160601b031661221d9190615bac565b1015612230575050505050505050505050565b8061223a81615670565b9150506121ad565b50604089013560a2556000808052609b6020908152600491600080516020615f0c83398151915291612276908d018d615429565b63ffffffff1681526020810191909152604001600020805460ff191660018360048111156122a6576122a6615125565b02179055506122bb60408b0160208c01615429565b609c805463ffffffff92909216600160601b0263ffffffff60601b199092169190911790556122ed60608b018b6154d6565b6122f991609d916141f8565b5061230a60a08b0160808c01615429565b609e805463ffffffff191663ffffffff9290921691909117905561233160208b018b615429565b609c805463ffffffff92909216600160401b0263ffffffff60401b1990921691909117905561236360208a018a615429565b63ffffffff167fff2908483d74b6b70053dd473260acf1b09e0ba0781bf94100bb8277581749de8a604051612398919061551c565b60405180910390a250505050505050505050565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa1580156123f4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906124189190615591565b6124345760405162461bcd60e51b8152600401610c14906155ae565b600019606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b60606000846001600160a01b031663c391425e84866040518363ffffffff1660e01b81526004016124a5929190615bcb565b600060405180830381865afa1580156124c2573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526124ea91908101906157bf565b9050600084516001600160401b0381111561250757612507614447565b604051908082528060200260200182016040528015612530578160200160208202803683370190505b50905060005b855181101561263157866001600160a01b03166304ec63518783815181106125605761256061562b565b60200260200101518786858151811061257b5761257b61562b565b60200260200101516040518463ffffffff1660e01b81526004016125b89392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa1580156125d5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906125f9919061586d565b6001600160c01b03168282815181106126145761261461562b565b60209081029190910101528061262981615670565b915050612536565b5095945050505050565b60975460408051632efa2ca360e11b815290516000926001600160a01b031691635df459469160048083019260209291908290030181865afa158015612685573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906126a9919061552a565b905090565b6126b661396c565b60a054600160a01b900460ff1615156001146127065760405162461bcd60e51b815260206004820152600f60248201526e4e6f207461736b2070656e64696e6760881b6044820152606401610c14565b61270e613abd565b565b60975460408051636830483560e01b815290516000926001600160a01b03169163683048359160048083019260209291908290030181865afa158015612685573d6000803e3d6000fd5b60975460408051636d14a98760e01b815290516000926001600160a01b031691636d14a9879160048083019260209291908290030181865afa158015612685573d6000803e3d6000fd5b609f546001600160a01b031633146127e65760405162461bcd60e51b8152602060048201526005602482015264417574683160d81b6044820152606401610c14565b60a054600160a01b900460ff16156128375760405162461bcd60e51b81526020600482015260146024820152735461736b20616c72656164792070656e64696e6760601b6044820152606401610c14565b612842838383613c66565b505050565b604080518082019091526060808252602082015260975460405163377da31b60e11b81526000916001600160a01b031690636efb463690612894908a908a908a908a908a90600401615981565b600060405180830381865afa1580156128b1573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526128d99190810190615ae1565b915091509550959350505050565b609f546001600160a01b031633146129295760405162461bcd60e51b8152602060048201526005602482015264417574683160d81b6044820152606401610c14565b60a054600160a01b900460ff161561297a5760405162461bcd60e51b81526020600482015260146024820152735461736b20616c72656164792070656e64696e6760601b6044820152606401610c14565b609c54600160601b900463ffffffff161580159061299757504315155b6129d55760405162461bcd60e51b815260206004820152600f60248201526e13dc0814dd185d19481d5b9a5b9a5d608a1b6044820152606401610c14565b6098546040805160e08101909152600160e01b90910463ffffffff168082529060009060208101856001811115612a0e57612a0e615125565b815263ffffffff80861660208301524381166040830152609c54600160601b9004166060820152609d8054608090920191612a48906155f6565b80601f0160208091040260200160405190810160405280929190818152602001828054612a74906155f6565b8015612ac15780601f10612a9657610100808354040283529160200191612ac1565b820191906000526020600020905b815481529060010190602001808311612aa457829003601f168201915b5050509183525050609e5463ffffffff16602091820152604051919250612aea91839101615bfe565b60408051808303601f19018152828252805160209182012063ffffffff86811660008181527fbb86fbc034f4e382929974bcd8419ed626b0ea647f962d89ba2fb6bd28785ab9855285812093909355600080516020615f2c8339815191529093529290208054600160ff19909116179055609c8054439093166401000000000267ffffffff00000000199093169290921790915560a0805460ff60a01b1916600160a01b179055907f584637a8f9d0f91a80c9f709b2b09d7db1d770fc7294e20d9d2495c378586cd290612bbf908490615bfe565b60405180910390a2612bd282600161574d565b6098601c6101000a81548163ffffffff021916908363ffffffff16021790555050505050565b612c0061396c565b61270e6000613fbf565b612c1261396c565b609780546001600160a01b0319166001600160a01b0383169081179091556040519081527f901a654dc830c94e8a12c9a3bc0a92ac11b5cf28046ca8d190691cdaf520901690602001610d9b565b612c6861396c565b6040517f4d60154266b2ea0c8f091d257eac5abc941c46cb54d0c3069a830f6339fe1da190600090a1565b6040805160018082528183019092526000916060918391602080830190803683370190505090508481600081518110612cce57612cce61562b565b60209081029190910101526040516361c8a12f60e11b81526000906001600160a01b0388169063c391425e90612d0a9088908690600401615bcb565b600060405180830381865afa158015612d27573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052612d4f91908101906157bf565b600081518110612d6157612d6161562b565b60209081029190910101516040516304ec635160e01b81526004810188905263ffffffff87811660248301529091166044820181905291506000906001600160a01b038916906304ec635190606401602060405180830381865afa158015612dcd573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612df1919061586d565b6001600160c01b031690506000612e0782614011565b905081612e158a838a610f50565b9550955050505050935093915050565b600054610100900460ff1615808015612e455750600054600160ff909116105b80612e5f5750303b158015612e5f575060005460ff166001145b612ec25760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610c14565b6000805460ff191660011790558015612ee5576000805461ff0019166101001790555b612ef08a60006140dd565b612ef989613fbf565b609e8054640100000000600160c01b0319166401000000006001600160a01b038b81169190910291909117909155609f80546001600160a01b03199081168a84161790915560a3805460ff1916891515179055609780548216888416179055609880548684166001600160c01b031990911617600160a01b63ffffffff89160217905560a080549091169184169190911790558015612fd2576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050505050505050565b6097546040805163df5cf72360e01b815290516000926001600160a01b03169163df5cf7239160048083019260209291908290030181865afa158015612685573d6000803e3d6000fd5b609e5464010000000090046001600160a01b031633146130725760405162461bcd60e51b8152602060048201526005602482015264041757468360dc1b6044820152606401610c14565b36600061308260a08601866154d6565b9092509050600061309960e0870160c08801615429565b905060a160006130af6060880160408901615108565b60018111156130c0576130c0615125565b60018111156130d1576130d1615125565b815260208101919091526040016000205463ffffffff161580613154575060a160006131036060880160408901615108565b600181111561311457613114615125565b600181111561312557613125615125565b815260208101919091526040016000205463ffffffff1661314c6080870160608801615429565b63ffffffff16145b6131a05760405162461bcd60e51b815260206004820152601a60248201527f636861696e526442617463684e6f6e6365206d69736d617463680000000000006044820152606401610c14565b6131ec866040516020016131b49190615c7c565b60408051601f198184030181529190528051602091820120906001906131dc90890189615429565b61052460808b0160608c01615429565b6000856040516020016131ff9190615dc8565b604051602081830303815290604052805190602001209050600080609760009054906101000a90046001600160a01b03166001600160a01b0316636efb46368488888d60800160208101906132549190615429565b8c6040518663ffffffff1660e01b8152600401613275959493929190615981565b600060405180830381865afa158015613292573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526132ba9190810190615ae1565b6040805160808101825263ffffffff431681526020808201849052808501518284015284516060830152915193955091935090916132fc918b91849101615dd7565b60408051601f19818403018152919052805160209182012060016000908152609a835290917f5b542b52981c4f2fa9965514d5bb7f37f1b7bc0902a6a4dc6b04dc05be85586b9190613350908d018d615429565b63ffffffff16815260208082019290925260400160009081209290925560018252609b8152600391600080516020615f2c83398151915291613394908d018d615429565b63ffffffff1681526020810191909152604001600020805460ff191660018360048111156133c4576133c4615125565b021790555060a0805460ff60a01b191690556133e360208b018b615429565b63ffffffff167f82e5c8e9447510b867d248c892385ba34fa6c2d4c4c26ff6868499ae4027f2c68a8360405161341a929190615dd7565b60405180910390a260005b868110156134ba578560ff16846020015182815181106134475761344761562b565b60200260200101516134599190615b7d565b6001600160601b031660648560000151838151811061347a5761347a61562b565b60200260200101516001600160601b03166134959190615bac565b10156134a8575050505050505050505050565b806134b281615670565b915050613425565b5060016000908152609b6020908152600491600080516020615f2c833981519152916134e8908d018d615429565b63ffffffff1681526020810191909152604001600020805460ff1916600183600481111561351857613518615125565b021790555060008061353060608c0160408d01615108565b600181111561354157613541615125565b14156135c85760408051808201825260a08c810135825260c08d013560208301525491516223d0b560e61b815290916001600160a01b0316906308f42d40906135949060808f0135908590600401615df9565b600060405180830381600087803b1580156135ae57600080fd5b505af11580156135c2573d6000803e3d6000fd5b50505050505b6135d860808b0160608c01615429565b6135e390600161574d565b60a160006135f760608e0160408f01615108565b600181111561360857613608615125565b600181111561361957613619615125565b8152602080820192909252604001600020805463ffffffff191663ffffffff939093169290921790915561364f908b018b615429565b63ffffffff167f1797ca59e06ea4a0efe10ac0fb51b58c8acf5cfedbc15fae51c10021dcb906e68b6040516136849190615dc8565b60405180910390a25050505050505050505050565b6136a161396c565b6001600160a01b0381166137065760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610c14565b61057681613fbf565b61371761396c565b60a054600160a01b900460ff161561373157613731613abd565b61373c838383613c66565b6040517f4ee987e5f1be19cabfb1a243e5c423889f060f33266753953ff0cf9db89966ab90600090a1505050565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156137bd573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906137e1919061552a565b6001600160a01b0316336001600160a01b0316146138115760405162461bcd60e51b8152600401610c1490615547565b60665419811960665419161461388f5760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e756e70617573653a20696e76616c696420617474656d7060448201527f7420746f2070617573652066756e6374696f6e616c69747900000000000000006064820152608401610c14565b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c90602001610d5a565b606654156139165760405162461bcd60e51b815260206004820152601c60248201527f5061757361626c653a20636f6e747261637420697320706175736564000000006044820152606401610c14565b61391e61396c565b60a080546001600160a01b0319166001600160a01b0383169081179091556040519081527f2f20cf1bda67739044c5bf577353970c3dbc183b2c7274d1e8584a102692326790602001610d9b565b6033546001600160a01b0316331461270e5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610c14565b6001600160a01b038116613a545760405162461bcd60e51b815260206004820152604960248201527f5061757361626c652e5f73657450617573657252656769737472793a206e657760448201527f50617573657252656769737472792063616e6e6f7420626520746865207a65726064820152686f206164647265737360b81b608482015260a401610c14565b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b609854600160c01b900463ffffffff1615613b8a57609854600090613af190600190600160c01b900463ffffffff16615e17565b63ffffffff81166000908152600080516020615f0c833981519152602052604090205490915060019060ff166004811115613b2e57613b2e615125565b1415613b885763ffffffff81166000818152600080516020615f0c8339815191526020526040808220805460ff19166002179055517fd6a4e0ff9f3a053708757c7a124abee31ced61f43f17e6e1cf11943ec59e60719190a25b505b609854600160e01b900463ffffffff1615613c5757609854600090613bbe90600190600160e01b900463ffffffff16615e17565b63ffffffff81166000908152600080516020615f2c833981519152602052604090205490915060019060ff166004811115613bfb57613bfb615125565b1415613c555763ffffffff81166000818152600080516020615f2c8339815191526020526040808220805460ff19166002179055517f0bf46bfca6e2137d35b893c295add8c33bcfbffafdef93252cb51aed7538ba0c9190a25b505b60a0805460ff60a01b19169055565b609c5463ffffffff600160601b909104164314801590613c8557504315155b613ce25760405162461bcd60e51b815260206004820152602860248201527f43616e277420696e206c617374436f6d706c657465644f705461736b43726561604482015267746564426c6f636b60c01b6064820152608401610c14565b6040805160e0810182526000818301819052606080830181905260a083015260c0820152609854600160c01b900463ffffffff908116825243811660208084019190915290861660808301528251601f850182900482028101820190935283835290919084908490819084018382808284376000920191909152505050506060820152609c54600160601b900463ffffffff16613dd15763ffffffff431660408083019190915280516020601f850181900481028201810190925283815290849084908190840183828082843760009201919091525050505060a082015263ffffffff841660c0820152613e85565b609c54600160601b900463ffffffff166040820152609d8054613df3906155f6565b80601f0160208091040260200160405190810160405280929190818152602001828054613e1f906155f6565b8015613e6c5780601f10613e4157610100808354040283529160200191613e6c565b820191906000526020600020905b815481529060010190602001808311613e4f57829003601f168201915b505050505060a0820152609e5463ffffffff1660c08201525b80604051602001613e969190615e3c565b60408051808303601f19018152828252805160209182012060988054600160c01b9081900463ffffffff90811660009081527f235d629dc802037ded8c61cb27fb29e40fa01b299719d8f991ffe20bdcc59f4f865286812094909455825482900481168452600080516020615f0c83398151915290945293909120805460ff19166001179055609c805463ffffffff191643841617905560a08054600160a01b60ff60a01b19909116179055549190910416907ffaf4b2054479d0f83e909b73cde2a6cb18ec2a93ba8ad5a62329001c86b1f3ea90613f76908490615e3c565b60405180910390a2609854613f9990600160c01b900463ffffffff16600161574d565b609860186101000a81548163ffffffff021916908363ffffffff16021790555050505050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b606060008061401f846141c7565b61ffff166001600160401b0381111561403a5761403a614447565b6040519080825280601f01601f191660200182016040528015614064576020820181803683370190505b5090506000805b82518210801561407c575061010081105b156140d3576001811b9350858416156140c3578060f81b8383815181106140a5576140a561562b565b60200101906001600160f81b031916908160001a9053508160010191505b6140cc81615670565b905061406b565b5090949350505050565b6065546001600160a01b03161580156140fe57506001600160a01b03821615155b6141805760405162461bcd60e51b815260206004820152604760248201527f5061757361626c652e5f696e697469616c697a655061757365723a205f696e6960448201527f7469616c697a6550617573657228292063616e206f6e6c792062652063616c6c6064820152666564206f6e636560c81b608482015260a401610c14565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a26141c3826139c6565b5050565b6000805b82156141f2576141dc600184615ed2565b90921691806141ea81615ee9565b9150506141cb565b92915050565b828054614204906155f6565b90600052602060002090601f016020900481019282614226576000855561426c565b82601f1061423f5782800160ff1982351617855561426c565b8280016001018555821561426c579182015b8281111561426c578235825591602001919060010190614251565b5061427892915061427c565b5090565b5b80821115614278576000815560010161427d565b600060e082840312156142a357600080fd5b50919050565b6000606082840312156142a357600080fd5b600080608083850312156142ce57600080fd5b82356001600160401b038111156142e457600080fd5b6142f085828601614291565b92505061430084602085016142a9565b90509250929050565b6001600160a01b038116811461057657600080fd5b60006020828403121561433057600080fd5b81356113e181614309565b60006020828403121561434d57600080fd5b5035919050565b6002811061057657600080fd5b63ffffffff8116811461057657600080fd5b803561437e81614361565b919050565b6000806040838503121561439657600080fd5b82356143a181614354565b915060208301356143b181614361565b809150509250929050565b801515811461057657600080fd5b6000602082840312156143dc57600080fd5b81356113e1816143bc565b6000815180845260005b8181101561440d576020818501810151868301820152016143f1565b8181111561441f576000602083870101525b50601f01601f19169290920160200192915050565b6020815260006113e160208301846143e7565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b038111828210171561447f5761447f614447565b60405290565b60405161010081016001600160401b038111828210171561447f5761447f614447565b604051601f8201601f191681016001600160401b03811182821017156144d0576144d0614447565b604052919050565b60006001600160401b038211156144f1576144f1614447565b5060051b60200190565b6000806040838503121561450e57600080fd5b823561451981614309565b91506020838101356001600160401b0381111561453557600080fd5b8401601f8101861361454657600080fd5b8035614559614554826144d8565b6144a8565b81815260059190911b8201830190838101908883111561457857600080fd5b928401925b8284101561459f57833561459081614309565b8252928401929084019061457d565b80955050505050509250929050565b600081518084526020808501945080840160005b838110156145de578151875295820195908201906001016145c2565b509495945050505050565b6020815260006113e160208301846145ae565b60008060006060848603121561461157600080fd5b833561461c81614309565b92506020848101356001600160401b038082111561463957600080fd5b818701915087601f83011261464d57600080fd5b81358181111561465f5761465f614447565b614671601f8201601f191685016144a8565b9150808252888482850101111561468757600080fd5b80848401858401376000848284010152508094505050506146aa60408501614373565b90509250925092565b600081518084526020808501808196508360051b810191508286016000805b86811015614749578385038a52825180518087529087019087870190845b8181101561473457835180516001600160a01b031684528a8101518b8501526040908101516001600160601b031690840152928901926060909201916001016146f0565b50509a87019a955050918501916001016146d2565b509298975050505050505050565b6020815260006113e160208301846146b3565b6000806000806080858703121561478057600080fd5b84359350602085013561479281614354565b925060408501356147a281614361565b915060608501356147b281614361565b939692955090935050565b6000602082840312156147cf57600080fd5b81356001600160401b038111156147e557600080fd5b8201604081850312156113e157600080fd5b600082601f83011261480857600080fd5b81356020614818614554836144d8565b82815260059290921b8401810191818101908684111561483757600080fd5b8286015b84811015614852578035835291830191830161483b565b509695505050505050565b6000806040838503121561487057600080fd5b823561487b81614309565b915060208301356001600160401b0381111561489657600080fd5b6148a2858286016147f7565b9150509250929050565b6020808252825182820181905260009190848201906040850190845b818110156148ed5783516001600160a01b0316835292840192918401916001016148c8565b50909695505050505050565b60008083601f84011261490b57600080fd5b5081356001600160401b0381111561492257600080fd5b60208301915083602082850101111561493a57600080fd5b9250929050565b6000806000806000806080878903121561495a57600080fd5b863561496581614309565b9550602087013561497581614361565b945060408701356001600160401b038082111561499157600080fd5b61499d8a838b016148f9565b909650945060608901359150808211156149b657600080fd5b818901915089601f8301126149ca57600080fd5b8135818111156149d957600080fd5b8a60208260051b85010111156149ee57600080fd5b6020830194508093505050509295509295509295565b600081518084526020808501945080840160005b838110156145de57815163ffffffff1687529582019590820190600101614a18565b600081518084526020808501808196508360051b8101915082860160005b85811015614a82578284038952614a70848351614a04565b98850198935090840190600101614a58565b5091979650505050505050565b602081526000825160806020840152614aab60a0840182614a04565b90506020840151601f1980858403016040860152614ac98383614a04565b92506040860151915080858403016060860152614ae68383614a04565b9250606086015191508085840301608086015250614b048282614a3a565b95945050505050565b600082601f830112614b1e57600080fd5b81356020614b2e614554836144d8565b82815260059290921b84018101918181019086841115614b4d57600080fd5b8286015b84811015614852578035614b6481614361565b8352918301918301614b51565b600060408284031215614b8357600080fd5b614b8b61445d565b9050813581526020820135602082015292915050565b600082601f830112614bb257600080fd5b81356020614bc2614554836144d8565b82815260069290921b84018101918181019086841115614be157600080fd5b8286015b8481101561485257614bf78882614b71565b835291830191604001614be5565b600082601f830112614c1657600080fd5b614c1e61445d565b806040840185811115614c3057600080fd5b845b81811015614c4a578035845260209384019301614c32565b509095945050505050565b600060808284031215614c6757600080fd5b614c6f61445d565b9050614c7b8383614c05565b8152614c8a8360408401614c05565b602082015292915050565b600082601f830112614ca657600080fd5b81356020614cb6614554836144d8565b82815260059290921b84018101918181019086841115614cd557600080fd5b8286015b848110156148525780356001600160401b03811115614cf85760008081fd5b614d068986838b0101614b0d565b845250918301918301614cd9565b60006101808284031215614d2757600080fd5b614d2f614485565b905081356001600160401b0380821115614d4857600080fd5b614d5485838601614b0d565b83526020840135915080821115614d6a57600080fd5b614d7685838601614ba1565b60208401526040840135915080821115614d8f57600080fd5b614d9b85838601614ba1565b6040840152614dad8560608601614c55565b6060840152614dbf8560e08601614b71565b6080840152610120840135915080821115614dd957600080fd5b614de585838601614b0d565b60a0840152610140840135915080821115614dff57600080fd5b614e0b85838601614b0d565b60c0840152610160840135915080821115614e2557600080fd5b50614e3284828501614c95565b60e08301525092915050565b600080600060a08486031215614e5357600080fd5b83356001600160401b0380821115614e6a57600080fd5b614e7687838801614291565b9450614e8587602088016142a9565b93506080860135915080821115614e9b57600080fd5b50614ea886828701614d14565b9150509250925092565b600060208284031215614ec457600080fd5b81356001600160401b03811115614eda57600080fd5b820161012081850312156113e157600080fd5b600060208284031215614eff57600080fd5b813560ff811681146113e157600080fd5b600080600060608486031215614f2557600080fd5b8335614f3081614309565b925060208401356001600160401b03811115614f4b57600080fd5b614f57868287016147f7565b9250506040840135614f6881614361565b809150509250925092565b6020808252825182820181905260009190848201906040850190845b818110156148ed57835183529284019291840191600101614f8f565b600080600060408486031215614fc057600080fd5b8335614fcb81614361565b925060208401356001600160401b03811115614fe657600080fd5b614ff2868287016148f9565b9497909650939450505050565b60008060008060006080868803121561501757600080fd5b8535945060208601356001600160401b038082111561503557600080fd5b61504189838a016148f9565b90965094506040880135915061505682614361565b9092506060870135908082111561506c57600080fd5b5061507988828901614d14565b9150509295509295909350565b600081518084526020808501945080840160005b838110156145de5781516001600160601b03168752958201959082019060010161509a565b60408152600083516040808401526150da6080840182615086565b90506020850151603f198483030160608501526150f78282615086565b925050508260208301529392505050565b60006020828403121561511a57600080fd5b81356113e181614354565b634e487b7160e01b600052602160045260246000fd5b602081016005831061514f5761514f615125565b91905290565b60008060006060848603121561516a57600080fd5b833561517581614309565b9250602084013591506040840135614f6881614361565b8281526040602082015260006151a560408301846146b3565b949350505050565b60008060008060008060008060006101208a8c0312156151cc57600080fd5b89356151d781614309565b985060208a01356151e781614309565b975060408a01356151f781614309565b965060608a013561520781614309565b955060808a0135615217816143bc565b945060a08a013561522781614309565b935060c08a013561523781614361565b925060e08a013561524781614309565b91506101008a013561525881614309565b809150509295985092959850929598565b600080600083850361014081121561528057600080fd5b84356001600160401b038082111561529757600080fd5b6152a388838901614291565b9550610100601f19840112156152b857600080fd5b6020870194506101208701359250808311156152d357600080fd5b5050614ea886828701614d14565b6000808335601e198436030181126152f857600080fd5b83016020810192503590506001600160401b0381111561531757600080fd5b80360383131561493a57600080fd5b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b602081526000823561536081614361565b63ffffffff8116602084015250602083013561537b81614361565b63ffffffff811660408401525061539460408401614373565b63ffffffff81166060840152506153ae60608401846152e1565b60e060808501526153c461010085018284615326565b9150506153d360808501614373565b63ffffffff811660a0850152506153ed60a08501856152e1565b848303601f190160c0860152615404838284615326565b9250505061541460c08501614373565b63ffffffff811660e08501525b509392505050565b60006020828403121561543b57600080fd5b81356113e181614361565b803561545181614361565b63ffffffff16825260208181013590830152604090810135910152565b63ffffffff815116825260208101516020830152600060408201516080604085015261549d6080850182615086565b905060608301518482036060860152614b048282615086565b6154c08184615446565b6080606082015260006151a5608083018461546e565b6000808335601e198436030181126154ed57600080fd5b8301803591506001600160401b0382111561550757600080fd5b60200191503681900382131561493a57600080fd5b606081016141f28284615446565b60006020828403121561553c57600080fd5b81516113e181614309565b6020808252602a908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526939903ab73830bab9b2b960b11b606082015260800190565b6000602082840312156155a357600080fd5b81516113e1816143bc565b60208082526028908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526739903830bab9b2b960c11b606082015260800190565b600181811c9082168061560a57607f821691505b602082108114156142a357634e487b7160e01b600052602260045260246000fd5b634e487b7160e01b600052603260045260246000fd5b60006020828403121561565357600080fd5b5051919050565b634e487b7160e01b600052601160045260246000fd5b60006000198214156156845761568461565a565b5060010190565b6000602080838503121561569e57600080fd5b82516001600160401b038111156156b457600080fd5b8301601f810185136156c557600080fd5b80516156d3614554826144d8565b81815260059190911b820183019083810190878311156156f257600080fd5b928401925b82841015615710578351825292840192908401906156f7565b979650505050505050565b80516001600160601b038116811461437e57600080fd5b60006020828403121561574457600080fd5b6113e18261571b565b600063ffffffff80831681851680830382111561576c5761576c61565a565b01949350505050565b63ffffffff84168152604060208201819052810182905260006001600160fb1b038311156157a257600080fd5b8260051b8085606085013760009201606001918252509392505050565b600060208083850312156157d257600080fd5b82516001600160401b038111156157e857600080fd5b8301601f810185136157f957600080fd5b8051615807614554826144d8565b81815260059190911b8201830190838101908783111561582657600080fd5b928401925b8284101561571057835161583e81614361565b8252928401929084019061582b565b63ffffffff84168152604060208201526000614b04604083018486615326565b60006020828403121561587f57600080fd5b81516001600160c01b03811681146113e157600080fd5b6000602082840312156158a857600080fd5b81516113e181614361565b600060ff821660ff8114156158ca576158ca61565a565b60010192915050565b6040815260006158e7604083018587615326565b905063ffffffff83166020830152949350505050565b600081518084526020808501945080840160005b838110156145de5761592e87835180518252602090810151910152565b6040969096019590820190600101615911565b8060005b6002811015611656578151845260209384019390910190600101615945565b61596f828251615941565b60208101516128426040840182615941565b85815260806020820152600061599b608083018688615326565b63ffffffff85166040840152828103606084015261018084518183526159c382840182614a04565b915050602085015182820360208401526159dd82826158fd565b915050604085015182820360408401526159f782826158fd565b9150506060850151615a0c6060840182615964565b506080850151805160e08401526020015161010083015260a0850151828203610120840152615a3b8282614a04565b91505060c0850151828203610140840152615a568282614a04565b91505060e0850151828203610160840152615a718282614a3a565b9a9950505050505050505050565b600082601f830112615a9057600080fd5b81516020615aa0614554836144d8565b82815260059290921b84018101918181019086841115615abf57600080fd5b8286015b8481101561485257615ad48161571b565b8352918301918301615ac3565b60008060408385031215615af457600080fd5b82516001600160401b0380821115615b0b57600080fd5b9084019060408287031215615b1f57600080fd5b615b2761445d565b825182811115615b3657600080fd5b615b4288828601615a7f565b825250602083015182811115615b5757600080fd5b615b6388828601615a7f565b602083015250809450505050602083015190509250929050565b60006001600160601b0380831681851681830481118215151615615ba357615ba361565a565b02949350505050565b6000816000190483118215151615615bc657615bc661565a565b500290565b63ffffffff831681526040602082015260006151a560408301846145ae565b60028110615bfa57615bfa615125565b9052565b60208152600063ffffffff8084511660208401526020840151615c246040850182615bea565b508060408501511660608401528060608501511660808401528060808501511660a084015260a084015160e060c0850152615c636101008501826143e7565b90508160c08601511660e0850152809250505092915050565b6020815260008235615c8d81614361565b63ffffffff808216602085015260208501359150615caa82614354565b615cb76040850183615bea565b60408501359150615cc782614361565b808216606085015260608501359150615cdf82614361565b80821660808501525050615cf560808401614373565b63ffffffff811660a084015250615d0f60a08401846152e1565b60e060c0850152615d2561010085018284615326565b91505061541460c08501614373565b8035615d3f81614361565b63ffffffff80821684526020830135602085015260408301359150615d6382614354565b615d706040850183615bea565b60608301359150615d8082614361565b1660608301526080818101359083015260a0808201359083015260c0808201359083015260e0810135615db281614309565b6001600160a01b031660e0929092019190915250565b61010081016141f28284615d34565b6000610120615de68386615d34565b80610100840152614b048184018561546e565b828152606081016113e1602083018480518252602090810151910152565b600063ffffffff83811690831681811015615e3457615e3461565a565b039392505050565b60208152600063ffffffff80845116602084015280602085015116604084015280604085015116606084015250606083015160e06080840152615e836101008401826143e7565b90506080840151615e9c60a085018263ffffffff169052565b5060a0840151838203601f190160c0850152615eb882826143e7565b91505060c084015161542160e085018263ffffffff169052565b600082821015615ee457615ee461565a565b500390565b600061ffff80831681811415615f0157615f0161565a565b600101939250505056fe10afac9233b4ccc54d6404ffc1cf3b47515a2b8edbf675d15eddce05a027dcbd298c800d0881dd208d705ebc03eb18189f38118259f27dd43b4c60d61c607e87a2646970667358221220d2a64e44810a7bf053963ea751b139876623dc204c8cfb14587a15e679ffb48764736f6c634300080c0033",
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

// ValidateTaskResponse is a free data retrieval call binding the contract method 0x37d347d1.
//
// Solidity: function validateTaskResponse(bytes32 taskHash, uint8 taskType, uint32 referenceTaskIndex, uint32 taskCreatedBlock) view returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) ValidateTaskResponse(opts *bind.CallOpts, taskHash [32]byte, taskType uint8, referenceTaskIndex uint32, taskCreatedBlock uint32) error {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "validateTaskResponse", taskHash, taskType, referenceTaskIndex, taskCreatedBlock)

	if err != nil {
		return err
	}

	return err

}

// ValidateTaskResponse is a free data retrieval call binding the contract method 0x37d347d1.
//
// Solidity: function validateTaskResponse(bytes32 taskHash, uint8 taskType, uint32 referenceTaskIndex, uint32 taskCreatedBlock) view returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) ValidateTaskResponse(taskHash [32]byte, taskType uint8, referenceTaskIndex uint32, taskCreatedBlock uint32) error {
	return _ContractFinalizerTaskManager.Contract.ValidateTaskResponse(&_ContractFinalizerTaskManager.CallOpts, taskHash, taskType, referenceTaskIndex, taskCreatedBlock)
}

// ValidateTaskResponse is a free data retrieval call binding the contract method 0x37d347d1.
//
// Solidity: function validateTaskResponse(bytes32 taskHash, uint8 taskType, uint32 referenceTaskIndex, uint32 taskCreatedBlock) view returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) ValidateTaskResponse(taskHash [32]byte, taskType uint8, referenceTaskIndex uint32, taskCreatedBlock uint32) error {
	return _ContractFinalizerTaskManager.Contract.ValidateTaskResponse(&_ContractFinalizerTaskManager.CallOpts, taskHash, taskType, referenceTaskIndex, taskCreatedBlock)
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

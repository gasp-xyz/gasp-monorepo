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
	BlockNumber                                  *big.Int
	TaskCreatedBlock                             uint32
	LastCompletedOpTaskCreatedBlock              uint32
	LastCompletedOpTaskQuorumNumbers             []byte
	LastCompletedOpTaskQuorumThresholdPercentage uint32
}

// IFinalizerTaskManagerRdTaskResponse is an auto generated low-level Go binding around an user-defined struct.
type IFinalizerTaskManagerRdTaskResponse struct {
	ReferenceTaskIndex uint32
	ReferenceTaskHash  [32]byte
	BlockHash          [32]byte
	StorageProofHash   [32]byte
	PendingStateHash   [32]byte
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
	ABI: "[{\"type\":\"function\",\"name\":\"THRESHOLD_DENOMINATOR\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"aggregator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allTaskHashes\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumFinalizerTaskManager.TaskType\"},{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allTaskResponses\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumFinalizerTaskManager.TaskType\"},{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allowNonRootInit\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"blsApkRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIBLSApkRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"blsSignatureChecker\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractBLSSignatureChecker\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"checkSignatures\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"referenceBlockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.QuorumStakeTotals\",\"components\":[{\"name\":\"signedStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"totalStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"createNewOpTask\",\"inputs\":[{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"createNewRdTask\",\"inputs\":[{\"name\":\"blockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"delegation\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIDelegationManager\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"dummyForOperatorStateInfoType\",\"inputs\":[{\"name\":\"_operatorStateInfo\",\"type\":\"tuple\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorStateInfo\",\"components\":[{\"name\":\"operatorsStateChanged\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"quorumsRemoved\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsAdded[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"quorumsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsStakeUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]},{\"name\":\"quorumsApkUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsApkUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"operatorsRemoved\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"operatorsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsAdded[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]},{\"name\":\"operatorsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsStakeUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"operatorsQuorumCountUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsQuorumCountUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]}]}],\"outputs\":[],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"dummyForQuorumStakeTotalsType\",\"inputs\":[{\"name\":\"_quorumStakeTotals\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.QuorumStakeTotals\",\"components\":[{\"name\":\"signedStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"totalStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]}],\"outputs\":[],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"forceCreateNewOpTask\",\"inputs\":[{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"forceRespondToOpTask\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"generator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getCheckSignaturesIndices\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"referenceBlockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"nonSignerOperatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structOperatorStateRetriever.CheckSignaturesIndices\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getOperatorState\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple[][]\",\"internalType\":\"structOperatorStateRetriever.Operator[][]\",\"components\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"stake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getOperatorState\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"\",\"type\":\"tuple[][]\",\"internalType\":\"structOperatorStateRetriever.Operator[][]\",\"components\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"stake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getQuorumBitmapsAtBlockNumber\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256[]\",\"internalType\":\"uint256[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"idToTaskStatus\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumFinalizerTaskManager.TaskType\"},{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumFinalizerTaskManager.TaskStatus\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"_pauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"initialOwner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_aggregator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_generator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_allowNonRootInit\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"_blsSignatureCheckerAddress\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_taskResponseWindowBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"_minOpTaskResponseWindowBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"lastCompletedOpTaskCreatedBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastCompletedOpTaskNum\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastOpTaskCreatedBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestOpTaskNum\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestPendingStateHash\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestRdTaskNum\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"minOpTaskResponseWindowBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"operatorsStateInfoHash\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"pauseAll\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[{\"name\":\"index\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pauserRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"registryCoordinator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"respondToOpTask\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"respondToRdTask\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.RdTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"blockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.RdTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"storageProofHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"pendingStateHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setPauserRegistry\",\"inputs\":[{\"name\":\"newPauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"stakeRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIStakeRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"taskResponseWindowBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"unpause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateBlsSignatureCheckerAddress\",\"inputs\":[{\"name\":\"_blsSignatureCheckerAddress\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"BLSSignatureCheckerAddressUpdated\",\"inputs\":[{\"name\":\"blsSignatureCheckerAddress\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Initialized\",\"inputs\":[{\"name\":\"version\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewOpTaskCreated\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"task\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.OpTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewOpTaskForceCreated\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"task\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.OpTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewRdTaskCreated\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"task\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.RdTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"blockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OpTaskCancelled\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OpTaskCompleted\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OpTaskForceCompleted\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OpTaskResponded\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"taskResponseMetadata\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.TaskResponseMetadata\",\"components\":[{\"name\":\"taskResponsedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"hashOfNonSigners\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quroumStakeTotals\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quroumStakeSigned\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Paused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"PauserRegistrySet\",\"inputs\":[{\"name\":\"pauserRegistry\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"newPauserRegistry\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractIPauserRegistry\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RdTaskCancelled\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RdTaskCompleted\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"blockHash\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.RdTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"storageProofHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"pendingStateHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RdTaskResponded\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.RdTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"storageProofHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"pendingStateHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"taskResponseMetadata\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.TaskResponseMetadata\",\"components\":[{\"name\":\"taskResponsedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"hashOfNonSigners\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quroumStakeTotals\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quroumStakeSigned\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"StaleStakesForbiddenUpdate\",\"inputs\":[{\"name\":\"value\",\"type\":\"bool\",\"indexed\":false,\"internalType\":\"bool\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Unpaused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false}]",
	Bin: "0x608060405234801561001057600080fd5b50615e0580620000216000396000f3fe608060405234801561001057600080fd5b50600436106102a05760003560e01c80635df4594611610167578063a69563a9116100ce578063df5cf72311610087578063df5cf72314610672578063e70c26231461067a578063ef0244581461068a578063f2fde38b14610692578063f5640cf8146106a5578063fabc1cbc146106b857600080fd5b8063a69563a9146105cc578063adfcb048146105e3578063b3ea184e146105ec578063bf2315ed146105ff578063c987de8e1461063a578063cefdc1d41461065157600080fd5b8063723114ab11610120578063723114ab1461055b5780637afa1eed1461056e5780637afdd54b14610581578063886f1195146105915780638da5cb5b146105a45780638fc8729a146105b557600080fd5b80635df4594614610507578063683048351461050f5780636d14a987146105175780636e125ff41461051f5780636efb463614610532578063715018a61461055357600080fd5b806341789d571161020b578063537a2929116101c4578063537a29291461048f57806354d127de146104a6578063595c6a67146104b45780635ac86ab7146104bc5780635c155662146104df5780635c975abb146104ff57600080fd5b806341789d571461040657806345265b7a146104325780634ae6b203146104435780634d7a71161461044c5780634f739f741461045c578063516a72271461047c57600080fd5b80631ac272971161025d5780631ac272971461034e5780631c178e9c14610379578063245a7bfc146103a45780632830e8f9146103be5780632db2e39b146103d35780633563b0d1146103e657600080fd5b806301a3f013146102a557806305c6b663146102ba5780630ee0fdbd146102cd57806310d67a2f146102ef578063136439dd1461030257806313f815ed14610315575b600080fd5b6102b86102b336600461425e565b6106cb565b005b6102b86102c83660046146c3565b610b5c565b60a2546102da9060ff1681565b60405190151581526020015b60405180910390f35b6102b86102fd366004614760565b611121565b6102b8610310366004614784565b6111d1565b61034061032336600461479d565b609960209081526000928352604080842090915290825290205481565b6040519081526020016102e6565b61034061035c36600461479d565b609a60209081526000928352604080842090915290825290205481565b60975461038c906001600160a01b031681565b6040516001600160a01b0390911681526020016102e6565b609e5461038c90600160201b90046001600160a01b031681565b6103c6611310565b6040516102e69190614827565b6102b86103e1366004614848565b61139e565b6103f96103f43660046148f1565b611557565b6040516102e69190614a4c565b60975461041d90600160e01b900463ffffffff1681565b60405163ffffffff90911681526020016102e6565b6102b8610440366004614a5f565b50565b61034060a05481565b609c5461041d9063ffffffff1681565b61046f61046a366004614ae1565b6119ed565b6040516102e69190614c3a565b6102b861048a366004614cb8565b612113565b609c5461041d90600160401b900463ffffffff1681565b6102b8610440366004614d22565b6102b861280b565b6102da6104ca366004614d5d565b606654600160ff9092169190911b9081161490565b6104f26104ed366004614d80565b6128d2565b6040516102e69190614e2c565b606654610340565b61038c612a9a565b61038c612b0d565b61038c612b57565b6102b861052d366004614e70565b612ba1565b610545610540366004614ec4565b613061565b6040516102e6929190614f84565b6102b8613101565b6102b8610569366004614760565b613115565b609f5461038c906001600160a01b031681565b60985461041d9063ffffffff1681565b60655461038c906001600160a01b031681565b6033546001600160a01b031661038c565b609c5461041d90600160201b900463ffffffff1681565b60975461041d90600160a01b900463ffffffff1681565b61034060a15481565b6102b86105fa366004614784565b613171565b61062d61060d36600461479d565b609b60209081526000928352604080842090915290825290205460ff1681565b6040516102e69190614fe3565b60975461041d90600160c01b900463ffffffff1681565b61066461065f36600461500b565b6135d6565b6040516102e692919061504d565b61038c613768565b609e5461041d9063ffffffff1681565b610340606481565b6102b86106a0366004614760565b6137b2565b6102b86106b3366004614e70565b613828565b6102b86106c6366004614784565b613cb5565b6106d3613e11565b60006106e2602083018361506e565b905060006106f6606085016040860161506e565b9050600061070a604086016020870161506e565b905036600061071c60a088018861508b565b9092509050600061073360e0890160c08a0161506e565b600080805260996020908152919250600080516020615db08339815191529161075e908a018a61506e565b63ffffffff1663ffffffff168152602001908152602001600020548860405160200161078a919061513f565b60405160208183030381529060405280519060200120146107c65760405162461bcd60e51b81526004016107bd90615219565b60405180910390fd5b6000808052609b6020908152600191600080516020615d70833981519152916107f1908b018b61506e565b63ffffffff16815260208101919091526040016000205460ff16600481111561081c5761081c614fcd565b146108395760405162461bcd60e51b81526004016107bd90615276565b6000808052609a60209081527fbe6620bd3346e5d7f8387fbec0981aa0d6289d22efa7c935f9ef6841bf2a98c7908290610875908b018b61506e565b63ffffffff1663ffffffff16815260200190815260200160002054146108ad5760405162461bcd60e51b81526004016107bd90615276565b6097546108c790600160a01b900463ffffffff16856152d8565b63ffffffff164363ffffffff1611156108f25760405162461bcd60e51b81526004016107bd90615300565b60408051808201909152606080825260208201526040805160808101825263ffffffff431681526000602080830182905284810151838501528451606084015292519092610944918c918491016153bd565b60408051601f1981840301815291905280516020918201206000808052609a835290917fbe6620bd3346e5d7f8387fbec0981aa0d6289d22efa7c935f9ef6841bf2a98c79190610996908e018e61506e565b63ffffffff16815260208082019290925260409081016000908120939093558c013560a155818052609b8152600491600080516020615d70833981519152916109e1908e018e61506e565b63ffffffff16815260208101919091526040016000205460ff166004811115610a0c57610a0c614fcd565b50610a1f905060408c0160208d0161506e565b609c805463ffffffff92909216600160401b0263ffffffff60401b19909216919091179055610a5160608c018c61508b565b610a5d91609d9161419b565b50610a6e60a08c0160808d0161506e565b609e805463ffffffff191663ffffffff92909216919091179055610a9560208c018c61506e565b609c805463ffffffff92909216600160201b0267ffffffff0000000019909216919091179055610ac860208b018b61506e565b63ffffffff167fff2908483d74b6b70053dd473260acf1b09e0ba0781bf94100bb8277581749de8b604051610afd91906153dd565b60405180910390a2610b1260208b018b61506e565b63ffffffff167fdf22f3558e4841b63d77179546b3eae63e4e343bbe752746b093162bc526be4c8b604051610b4791906153dd565b60405180910390a25050505050505050505050565b609e54600160201b90046001600160a01b03163314610b8d5760405162461bcd60e51b81526004016107bd906153eb565b6000610b9f608085016060860161506e565b90506000610bb3606086016040870161506e565b9050366000610bc5608088018861508b565b90925090506000610bdc60c0890160a08a0161506e565b60016000908152609960209081529192507fbb86fbc034f4e382929974bcd8419ed626b0ea647f962d89ba2fb6bd28785ab991610c1b908a018a61506e565b63ffffffff1663ffffffff1681526020019081526020016000205488604051602001610c47919061540a565b6040516020818303038152906040528051906020012014610c7a5760405162461bcd60e51b81526004016107bd90615219565b60016000818152609b6020908152600080516020615d908339815191529190610ca5908b018b61506e565b63ffffffff16815260208101919091526040016000205460ff166004811115610cd057610cd0614fcd565b14610ced5760405162461bcd60e51b81526004016107bd90615276565b60016000908152609a60209081527f5b542b52981c4f2fa9965514d5bb7f37f1b7bc0902a6a4dc6b04dc05be85586b908290610d2b908b018b61506e565b63ffffffff1663ffffffff1681526020019081526020016000205414610d635760405162461bcd60e51b81526004016107bd90615276565b609754610d7d90600160a01b900463ffffffff16856152d8565b63ffffffff164363ffffffff161115610da85760405162461bcd60e51b81526004016107bd90615300565b600087604051602001610dbb91906154e4565b604051602081830303815290604052805190602001209050600080609760009054906101000a90046001600160a01b03166001600160a01b0316636efb46368488888c8e6040518663ffffffff1660e01b8152600401610e1f95949392919061557c565b600060405180830381865afa158015610e3c573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610e6491908101906156f3565b6040805160808101825263ffffffff43168152602080820184905280850151828401528451606083015291519395509193509091610ea6918d9184910161578f565b60408051601f19818403018152919052805160209182012060016000908152609a835290917f5b542b52981c4f2fa9965514d5bb7f37f1b7bc0902a6a4dc6b04dc05be85586b9190610efa908f018f61506e565b63ffffffff1681526020810191909152604001600020556003609b6000600180811115610f2957610f29614fcd565b815260200190815260200160002060008d6000016020810190610f4c919061506e565b63ffffffff16815260208101919091526040016000205460ff166004811115610f7757610f77614fcd565b50610f87905060208d018d61506e565b63ffffffff167f07ba8d7c4209c14e6d2ed5fd4542e9d8f47e1af3da0cbd9c50e2d0d3e87dfff08c83604051610fbe92919061578f565b60405180910390a260005b86811015611060578560ff1684602001518281518110610feb57610feb6157af565b6020026020010151610ffd91906157c5565b6001600160601b031660648560000151838151811061101e5761101e6157af565b60200260200101516001600160601b031661103991906157f4565b101561104e5750505050505050505050505050565b8061105881615813565b915050610fc9565b5060808b013560a05560016000908152609b6020908152600491600080516020615d9083398151915291611096908f018f61506e565b63ffffffff16815260208101919091526040016000205460ff1660048111156110c1576110c1614fcd565b505060408b01356110d560208d018d61506e565b63ffffffff167f02046773dfd0e022ab6bf68cb07d73f080f87a4b82f7a4fccfe7d919d6c4ecb48d60405161110a91906154e4565b60405180910390a35050505050505050505b505050565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611174573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611198919061582e565b6001600160a01b0316336001600160a01b0316146111c85760405162461bcd60e51b81526004016107bd9061584b565b61044081613e6b565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa158015611219573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061123d9190615895565b6112595760405162461bcd60e51b81526004016107bd906158b2565b606654818116146112d25760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e70617573653a20696e76616c696420617474656d70742060448201527f746f20756e70617573652066756e6374696f6e616c697479000000000000000060648201526084016107bd565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b609d805461131d906158fa565b80601f0160208091040260200160405190810160405280929190818152602001828054611349906158fa565b80156113965780601f1061136b57610100808354040283529160200191611396565b820191906000526020600020905b81548152906001019060200180831161137957829003601f168201915b505050505081565b600054610100900460ff16158080156113be5750600054600160ff909116105b806113d85750303b1580156113d8575060005460ff166001145b61143b5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084016107bd565b6000805460ff19166001179055801561145e576000805461ff0019166101001790555b611469896000613f62565b6114728861404c565b609e80546001600160a01b03808a16600160201b02640100000000600160c01b031990921691909117909155609f80548883166001600160a01b031990911617905560a2805487151560ff199091161790556097805463ffffffff858116600160c01b0263ffffffff60c01b19918816600160a01b026001600160c01b0319909316948916949094179190911716919091179055801561154c576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b505050505050505050565b60606000846001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa158015611599573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115bd919061582e565b90506000856001600160a01b0316639e9923c26040518163ffffffff1660e01b8152600401602060405180830381865afa1580156115ff573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611623919061582e565b90506000866001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa158015611665573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611689919061582e565b9050600086516001600160401b038111156116a6576116a66142ac565b6040519080825280602002602001820160405280156116d957816020015b60608152602001906001900390816116c45790505b50905060005b87518110156119e15760008882815181106116fc576116fc6157af565b0160200151604051638902624560e01b815260f89190911c6004820181905263ffffffff8a16602483015291506000906001600160a01b03871690638902624590604401600060405180830381865afa15801561175d573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611785919081019061592f565b905080516001600160401b038111156117a0576117a06142ac565b6040519080825280602002602001820160405280156117eb57816020015b60408051606081018252600080825260208083018290529282015282526000199092019101816117be5790505b508484815181106117fe576117fe6157af565b602002602001018190525060005b81518110156119cb576040518060600160405280876001600160a01b03166347b314e8858581518110611841576118416157af565b60200260200101516040518263ffffffff1660e01b815260040161186791815260200190565b602060405180830381865afa158015611884573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906118a8919061582e565b6001600160a01b031681526020018383815181106118c8576118c86157af565b60200260200101518152602001896001600160a01b031663fa28c6278585815181106118f6576118f66157af565b60209081029190910101516040516001600160e01b031960e084901b168152600481019190915260ff8816602482015263ffffffff8f166044820152606401602060405180830381865afa158015611952573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061197691906159bf565b6001600160601b0316815250858581518110611994576119946157af565b602002602001015182815181106119ad576119ad6157af565b602002602001018190525080806119c390615813565b91505061180c565b50505080806119d990615813565b9150506116df565b50979650505050505050565b611a186040518060800160405280606081526020016060815260200160608152602001606081525090565b6000876001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa158015611a58573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611a7c919061582e565b9050611aa96040518060800160405280606081526020016060815260200160608152602001606081525090565b6040516361c8a12f60e11b81526001600160a01b038a169063c391425e90611ad9908b90899089906004016159da565b600060405180830381865afa158015611af6573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611b1e9190810190615a24565b81526040516340e03a8160e11b81526001600160a01b038316906381c0750290611b50908b908b908b90600401615ab2565b600060405180830381865afa158015611b6d573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611b959190810190615a24565b6040820152856001600160401b03811115611bb257611bb26142ac565b604051908082528060200260200182016040528015611be557816020015b6060815260200190600190039081611bd05790505b50606082015260005b60ff8116871115612024576000856001600160401b03811115611c1357611c136142ac565b604051908082528060200260200182016040528015611c3c578160200160208202803683370190505b5083606001518360ff1681518110611c5657611c566157af565b602002602001018190525060005b86811015611f245760008c6001600160a01b03166304ec63518a8a85818110611c8f57611c8f6157af565b905060200201358e88600001518681518110611cad57611cad6157af565b60200260200101516040518463ffffffff1660e01b8152600401611cea9392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa158015611d07573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611d2b9190615ad2565b90506001600160c01b038116611dcf5760405162461bcd60e51b815260206004820152605c60248201527f4f70657261746f7253746174655265747269657665722e676574436865636b5360448201527f69676e617475726573496e64696365733a206f70657261746f72206d7573742060648201527f6265207265676973746572656420617420626c6f636b6e756d62657200000000608482015260a4016107bd565b8a8a8560ff16818110611de457611de46157af565b6001600160c01b03841692013560f81c9190911c600190811614159050611f1157856001600160a01b031663dd9846b98a8a85818110611e2657611e266157af565b905060200201358d8d8860ff16818110611e4257611e426157af565b6040516001600160e01b031960e087901b1681526004810194909452919091013560f81c60248301525063ffffffff8f166044820152606401602060405180830381865afa158015611e98573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611ebc9190615afb565b85606001518560ff1681518110611ed557611ed56157af565b60200260200101518481518110611eee57611eee6157af565b63ffffffff9092166020928302919091019091015282611f0d81615813565b9350505b5080611f1c81615813565b915050611c64565b506000816001600160401b03811115611f3f57611f3f6142ac565b604051908082528060200260200182016040528015611f68578160200160208202803683370190505b50905060005b82811015611fe95784606001518460ff1681518110611f8f57611f8f6157af565b60200260200101518181518110611fa857611fa86157af565b6020026020010151828281518110611fc257611fc26157af565b63ffffffff9092166020928302919091019091015280611fe181615813565b915050611f6e565b508084606001518460ff1681518110612004576120046157af565b60200260200101819052505050808061201c90615b18565b915050611bee565b506000896001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa158015612065573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612089919061582e565b60405163354952a360e21b81529091506001600160a01b0382169063d5254a8c906120bc908b908b908e90600401615b38565b600060405180830381865afa1580156120d9573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526121019190810190615a24565b60208301525098975050505050505050565b609c54600160401b900463ffffffff16156000612136606086016040870161506e565b905081156121c15760a25460ff161561217f57609e54600160201b90046001600160a01b0316331461217a5760405162461bcd60e51b81526004016107bd906153eb565b6121f2565b6033546001600160a01b0316331461217a5760405162461bcd60e51b815260206004820152600560248201526420baba341960d91b60448201526064016107bd565b609e54600160201b90046001600160a01b031633146121f25760405162461bcd60e51b81526004016107bd906153eb565b6000612204604087016020880161506e565b905036600061221660a089018961508b565b9092509050600061222d60e08a0160c08b0161506e565b600080805260996020908152919250600080516020615db083398151915291612258908b018b61506e565b63ffffffff1663ffffffff1681526020019081526020016000205489604051602001612284919061513f565b60405160208183030381529060405280519060200120146122b75760405162461bcd60e51b81526004016107bd90615219565b6000808052609b6020908152600191600080516020615d70833981519152916122e2908c018c61506e565b63ffffffff16815260208101919091526040016000205460ff16600481111561230d5761230d614fcd565b1461232a5760405162461bcd60e51b81526004016107bd90615276565b6000808052609a60209081527fbe6620bd3346e5d7f8387fbec0981aa0d6289d22efa7c935f9ef6841bf2a98c7908290612366908c018c61506e565b63ffffffff1663ffffffff168152602001908152602001600020541461239e5760405162461bcd60e51b81526004016107bd90615276565b6097546123b890600160a01b900463ffffffff16856152d8565b63ffffffff164363ffffffff1611156123e35760405162461bcd60e51b81526004016107bd90615300565b6000886040516020016123f691906153dd565b60408051601f1981840301815282825280516020918201208383019092526060808452908301529150600088156124bb57609760009054906101000a90046001600160a01b03166001600160a01b0316636efb46368488888c8f6040518663ffffffff1660e01b815260040161247095949392919061557c565b600060405180830381865afa15801561248d573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526124b591908101906156f3565b90925090505b6040805160808101825263ffffffff431681526020808201849052848101518284015284516060830152915190916124f7918e918491016153bd565b60408051808303601f190181529190528051602090910120609a600080815260200190815260200160002060008e6000016020810190612537919061506e565b63ffffffff1681526020810191909152604001600020556003609b600080600181111561256657612566614fcd565b815260200190815260200160002060008e6000016020810190612589919061506e565b63ffffffff16815260208101919091526040016000205460ff1660048111156125b4576125b4614fcd565b506125c4905060208e018e61506e565b63ffffffff167f47adacb0b6bbd726ae39ac6c006cca1c2006c9aedaa882dcba7c4804db7c41ce8d836040516125fb9291906153bd565b60405180910390a289156126a65760005b868110156126a4578560ff168460200151828151811061262e5761262e6157af565b602002602001015161264091906157c5565b6001600160601b0316606485600001518381518110612661576126616157af565b60200260200101516001600160601b031661267c91906157f4565b1015612692575050505050505050505050505050565b8061269c81615813565b91505061260c565b505b60408c013560a1556004609b600080815260200190815260200160002060008e60000160208101906126d8919061506e565b63ffffffff16815260208101919091526040016000205460ff16600481111561270357612703614fcd565b50612716905060408e0160208f0161506e565b609c805463ffffffff92909216600160401b0263ffffffff60401b1990921691909117905561274860608e018e61508b565b61275491609d9161419b565b5061276560a08e0160808f0161506e565b609e805463ffffffff191663ffffffff9290921691909117905561278c60208e018e61506e565b609c805463ffffffff92909216600160201b0267ffffffff00000000199092169190911790556127bf60208d018d61506e565b63ffffffff167fff2908483d74b6b70053dd473260acf1b09e0ba0781bf94100bb8277581749de8d6040516127f491906153dd565b60405180910390a250505050505050505050505050565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa158015612853573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906128779190615895565b6128935760405162461bcd60e51b81526004016107bd906158b2565b600019606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b60606000846001600160a01b031663c391425e84866040518363ffffffff1660e01b8152600401612904929190615b62565b600060405180830381865afa158015612921573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526129499190810190615a24565b9050600084516001600160401b03811115612966576129666142ac565b60405190808252806020026020018201604052801561298f578160200160208202803683370190505b50905060005b8551811015612a9057866001600160a01b03166304ec63518783815181106129bf576129bf6157af565b6020026020010151878685815181106129da576129da6157af565b60200260200101516040518463ffffffff1660e01b8152600401612a179392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa158015612a34573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612a589190615ad2565b6001600160c01b0316828281518110612a7357612a736157af565b602090810291909101015280612a8881615813565b915050612995565b5095945050505050565b60975460408051632efa2ca360e11b815290516000926001600160a01b031691635df459469160048083019260209291908290030181865afa158015612ae4573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612b08919061582e565b905090565b60975460408051636830483560e01b815290516000926001600160a01b03169163683048359160048083019260209291908290030181865afa158015612ae4573d6000803e3d6000fd5b60975460408051636d14a98760e01b815290516000926001600160a01b031691636d14a9879160048083019260209291908290030181865afa158015612ae4573d6000803e3d6000fd5b609f546001600160a01b03163314612be35760405162461bcd60e51b8152602060048201526005602482015264417574683160d81b60448201526064016107bd565b609c5463ffffffff600160401b909104164314801590612c0257504315155b612c1e5760405162461bcd60e51b81526004016107bd90615bb6565b6040805160e0810182526000818301819052606080830181905260a083015260c0820152609754600160e01b900463ffffffff908116825243811660208084019190915290861660808301528251601f850182900482028101820190935283835290919084908490819084018382808284376000920191909152505050506060820152609c54600160401b900463ffffffff16612d0d5763ffffffff431660408083019190915280516020601f850181900481028201810190925283815290849084908190840183828082843760009201919091525050505060a082015263ffffffff841660c0820152612dc1565b609c54600160401b900463ffffffff166040820152609d8054612d2f906158fa565b80601f0160208091040260200160405190810160405280929190818152602001828054612d5b906158fa565b8015612da85780601f10612d7d57610100808354040283529160200191612da8565b820191906000526020600020905b815481529060010190602001808311612d8b57829003601f168201915b505050505060a0820152609e5463ffffffff1660c08201525b609754600160e01b900463ffffffff1615612e8e57609754600090612df590600190600160e01b900463ffffffff16615c13565b63ffffffff81166000908152600080516020615d70833981519152602052604090205490915060019060ff166004811115612e3257612e32614fcd565b1415612e8c5763ffffffff81166000818152600080516020615d708339815191526020526040808220805460ff19166002179055517fd6a4e0ff9f3a053708757c7a124abee31ced61f43f17e6e1cf11943ec59e60719190a25b505b60985463ffffffff1615612f4d57609854600090612eb49060019063ffffffff16615c13565b63ffffffff81166000908152600080516020615d90833981519152602052604090205490915060019060ff166004811115612ef157612ef1614fcd565b1415612f4b5763ffffffff81166000818152600080516020615d908339815191526020526040808220805460ff19166002179055517f0bf46bfca6e2137d35b893c295add8c33bcfbffafdef93252cb51aed7538ba0c9190a25b505b80604051602001612f5e9190615c38565b60408051601f1981840301815282825280516020918201206097805463ffffffff600160e01b9182900481166000908152600080516020615db0833981519152865286812094909455825482900481168452600080516020615d70833981519152909452939091208054600160ff19909116179055609c805463ffffffff1916438416179055549190910416907ffaf4b2054479d0f83e909b73cde2a6cb18ec2a93ba8ad5a62329001c86b1f3ea90613018908490615c38565b60405180910390a260975461303b90600160e01b900463ffffffff1660016152d8565b6097601c6101000a81548163ffffffff021916908363ffffffff16021790555050505050565b604080518082019091526060808252602082015260975460405163377da31b60e11b81526000916001600160a01b031690636efb4636906130ae908a908a908a908a908a9060040161557c565b600060405180830381865afa1580156130cb573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526130f391908101906156f3565b915091509550959350505050565b613109613e11565b613113600061404c565b565b61311d613e11565b609780546001600160a01b0319166001600160a01b0383169081179091556040519081527f901a654dc830c94e8a12c9a3bc0a92ac11b5cf28046ca8d190691cdaf52090169060200160405180910390a150565b609f546001600160a01b031633146131b35760405162461bcd60e51b8152602060048201526005602482015264417574683160d81b60448201526064016107bd565b609c54600160401b900463ffffffff16158015906131d057504315155b61320e5760405162461bcd60e51b815260206004820152600f60248201526e13dc0814dd185d19481d5b9a5b9a5d608a1b60448201526064016107bd565b6098546097546040805160c0810182526000606080830191909152608082015263ffffffff9384168082526020820186905243851692820192909252609e54841660a0820152609d80549294600160e01b909404909316929091613271906158fa565b80601f016020809104026020016040519081016040528092919081815260200182805461329d906158fa565b80156132ea5780601f106132bf576101008083540402835291602001916132ea565b820191906000526020600020905b8154815290600101906020018083116132cd57829003601f168201915b50505050506080820152609c54600160401b900463ffffffff90811660608301528316156133b857600061331f600185615c13565b63ffffffff81166000908152600080516020615d90833981519152602052604090205490915060019060ff16600481111561335c5761335c614fcd565b14156133b65763ffffffff81166000818152600080516020615d908339815191526020526040808220805460ff19166002179055517f0bf46bfca6e2137d35b893c295add8c33bcfbffafdef93252cb51aed7538ba0c9190a25b505b63ffffffff8216156134f55760006133d1600184615c13565b63ffffffff81166000908152600080516020615d70833981519152602052604090205490915060019060ff16600481111561340e5761340e614fcd565b14156134f357609754609c546134349163ffffffff600160c01b909104811691166152d8565b63ffffffff164363ffffffff16111561349f5760405162461bcd60e51b815260206004820152602760248201527f52645461736b2063616e2774207965742063616e63656c2074686520696e6974604482015266204f705461736b60c81b60648201526084016107bd565b63ffffffff81166000818152600080516020615d708339815191526020526040808220805460ff19166002179055517fd6a4e0ff9f3a053708757c7a124abee31ced61f43f17e6e1cf11943ec59e60719190a25b505b806040516020016135069190615cce565b60408051601f19818403018152828252805160209182012063ffffffff871660008181527fbb86fbc034f4e382929974bcd8419ed626b0ea647f962d89ba2fb6bd28785ab9845284812092909255600080516020615d9083398151915290925291909120805460ff19166001179055907f161faa5d92f6bf0262139c57c87ca6cc60e5b3c9c3341dc175cec97a2516cc7d906135a3908490615cce565b60405180910390a26135b68360016152d8565b6098805463ffffffff191663ffffffff9290921691909117905550505050565b6040805160018082528183019092526000916060918391602080830190803683370190505090508481600081518110613611576136116157af565b60209081029190910101526040516361c8a12f60e11b81526000906001600160a01b0388169063c391425e9061364d9088908690600401615b62565b600060405180830381865afa15801561366a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526136929190810190615a24565b6000815181106136a4576136a46157af565b60209081029190910101516040516304ec635160e01b81526004810188905263ffffffff87811660248301529091166044820181905291506000906001600160a01b038916906304ec635190606401602060405180830381865afa158015613710573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906137349190615ad2565b6001600160c01b03169050600061374a8261409e565b9050816137588a838a611557565b9550955050505050935093915050565b6097546040805163df5cf72360e01b815290516000926001600160a01b03169163df5cf7239160048083019260209291908290030181865afa158015612ae4573d6000803e3d6000fd5b6137ba613e11565b6001600160a01b03811661381f5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016107bd565b6104408161404c565b613830613e11565b609c5463ffffffff600160401b90910416431480159061384f57504315155b61386b5760405162461bcd60e51b81526004016107bd90615bb6565b6040805160e0810182526000818301819052606080830181905260a083015260c0820152609754600160e01b900463ffffffff908116825243811660208084019190915290861660808301528251601f850182900482028101820190935283835290919084908490819084018382808284376000920191909152505050506060820152609c54600160401b900463ffffffff1661395a5763ffffffff431660408083019190915280516020601f850181900481028201810190925283815290849084908190840183828082843760009201919091525050505060a082015263ffffffff841660c0820152613a0e565b609c54600160401b900463ffffffff166040820152609d805461397c906158fa565b80601f01602080910402602001604051908101604052809291908181526020018280546139a8906158fa565b80156139f55780601f106139ca576101008083540402835291602001916139f5565b820191906000526020600020905b8154815290600101906020018083116139d857829003601f168201915b505050505060a0820152609e5463ffffffff1660c08201525b609754600160e01b900463ffffffff1615613adb57609754600090613a4290600190600160e01b900463ffffffff16615c13565b63ffffffff81166000908152600080516020615d70833981519152602052604090205490915060019060ff166004811115613a7f57613a7f614fcd565b1415613ad95763ffffffff81166000818152600080516020615d708339815191526020526040808220805460ff19166002179055517fd6a4e0ff9f3a053708757c7a124abee31ced61f43f17e6e1cf11943ec59e60719190a25b505b60985463ffffffff1615613b9a57609854600090613b019060019063ffffffff16615c13565b63ffffffff81166000908152600080516020615d90833981519152602052604090205490915060019060ff166004811115613b3e57613b3e614fcd565b1415613b985763ffffffff81166000818152600080516020615d908339815191526020526040808220805460ff19166002179055517f0bf46bfca6e2137d35b893c295add8c33bcfbffafdef93252cb51aed7538ba0c9190a25b505b80604051602001613bab9190615c38565b60408051601f1981840301815282825280516020918201206097805463ffffffff600160e01b9182900481166000908152600080516020615db0833981519152865286812094909455825482900481168452600080516020615d70833981519152909452939091208054600160ff19909116179055609c805463ffffffff1916438416179055549190910416907ffaf4b2054479d0f83e909b73cde2a6cb18ec2a93ba8ad5a62329001c86b1f3ea90613c65908490615c38565b60405180910390a26097601c9054906101000a900463ffffffff1663ffffffff167fdc5ba3b66ec6491b585b3f13698ed711aa829071f43300d6ede6dba67e28d75f826040516130189190615c38565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613d08573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613d2c919061582e565b6001600160a01b0316336001600160a01b031614613d5c5760405162461bcd60e51b81526004016107bd9061584b565b606654198119606654191614613dda5760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e756e70617573653a20696e76616c696420617474656d7060448201527f7420746f2070617573652066756e6374696f6e616c697479000000000000000060648201526084016107bd565b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c90602001611305565b6033546001600160a01b031633146131135760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016107bd565b6001600160a01b038116613ef95760405162461bcd60e51b815260206004820152604960248201527f5061757361626c652e5f73657450617573657252656769737472793a206e657760448201527f50617573657252656769737472792063616e6e6f7420626520746865207a65726064820152686f206164647265737360b81b608482015260a4016107bd565b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b6065546001600160a01b0316158015613f8357506001600160a01b03821615155b6140055760405162461bcd60e51b815260206004820152604760248201527f5061757361626c652e5f696e697469616c697a655061757365723a205f696e6960448201527f7469616c697a6550617573657228292063616e206f6e6c792062652063616c6c6064820152666564206f6e636560c81b608482015260a4016107bd565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a261404882613e6b565b5050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b60606000806140ac8461416a565b61ffff166001600160401b038111156140c7576140c76142ac565b6040519080825280601f01601f1916602001820160405280156140f1576020820181803683370190505b5090506000805b825182108015614109575061010081105b15614160576001811b935085841615614150578060f81b838381518110614132576141326157af565b60200101906001600160f81b031916908160001a9053508160010191505b61415981615813565b90506140f8565b5090949350505050565b6000805b82156141955761417f600184615d36565b909216918061418d81615d4d565b91505061416e565b92915050565b8280546141a7906158fa565b90600052602060002090601f0160209004810192826141c9576000855561420f565b82601f106141e25782800160ff1982351617855561420f565b8280016001018555821561420f579182015b8281111561420f5782358255916020019190600101906141f4565b5061421b92915061421f565b5090565b5b8082111561421b5760008155600101614220565b600060e0828403121561424657600080fd5b50919050565b60006060828403121561424657600080fd5b6000806080838503121561427157600080fd5b82356001600160401b0381111561428757600080fd5b61429385828601614234565b9250506142a3846020850161424c565b90509250929050565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b03811182821017156142e4576142e46142ac565b60405290565b60405161010081016001600160401b03811182821017156142e4576142e46142ac565b604051601f8201601f191681016001600160401b0381118282101715614335576143356142ac565b604052919050565b60006001600160401b03821115614356576143566142ac565b5060051b60200190565b63ffffffff8116811461044057600080fd5b803561437d81614360565b919050565b600082601f83011261439357600080fd5b813560206143a86143a38361433d565b61430d565b82815260059290921b840181019181810190868411156143c757600080fd5b8286015b848110156143eb5780356143de81614360565b83529183019183016143cb565b509695505050505050565b60006040828403121561440857600080fd5b6144106142c2565b9050813581526020820135602082015292915050565b600082601f83011261443757600080fd5b813560206144476143a38361433d565b82815260069290921b8401810191818101908684111561446657600080fd5b8286015b848110156143eb5761447c88826143f6565b83529183019160400161446a565b600082601f83011261449b57600080fd5b6144a36142c2565b8060408401858111156144b557600080fd5b845b818110156144cf5780358452602093840193016144b7565b509095945050505050565b6000608082840312156144ec57600080fd5b6144f46142c2565b9050614500838361448a565b815261450f836040840161448a565b602082015292915050565b600082601f83011261452b57600080fd5b8135602061453b6143a38361433d565b82815260059290921b8401810191818101908684111561455a57600080fd5b8286015b848110156143eb5780356001600160401b0381111561457d5760008081fd5b61458b8986838b0101614382565b84525091830191830161455e565b600061018082840312156145ac57600080fd5b6145b46142ea565b905081356001600160401b03808211156145cd57600080fd5b6145d985838601614382565b835260208401359150808211156145ef57600080fd5b6145fb85838601614426565b6020840152604084013591508082111561461457600080fd5b61462085838601614426565b604084015261463285606086016144da565b60608401526146448560e086016143f6565b608084015261012084013591508082111561465e57600080fd5b61466a85838601614382565b60a084015261014084013591508082111561468457600080fd5b61469085838601614382565b60c08401526101608401359150808211156146aa57600080fd5b506146b78482850161451a565b60e08301525092915050565b600080600083850360e08112156146d957600080fd5b84356001600160401b03808211156146f057600080fd5b9086019060c0828903121561470457600080fd5b81955060a0601f198401121561471957600080fd5b60208701945060c087013592508083111561473357600080fd5b505061474186828701614599565b9150509250925092565b6001600160a01b038116811461044057600080fd5b60006020828403121561477257600080fd5b813561477d8161474b565b9392505050565b60006020828403121561479657600080fd5b5035919050565b600080604083850312156147b057600080fd5b8235600281106147bf57600080fd5b915060208301356147cf81614360565b809150509250929050565b6000815180845260005b81811015614800576020818501810151868301820152016147e4565b81811115614812576000602083870101525b50601f01601f19169290920160200192915050565b60208152600061477d60208301846147da565b801515811461044057600080fd5b600080600080600080600080610100898b03121561486557600080fd5b88356148708161474b565b975060208901356148808161474b565b965060408901356148908161474b565b955060608901356148a08161474b565b945060808901356148b08161483a565b935060a08901356148c08161474b565b925060c08901356148d081614360565b915060e08901356148e081614360565b809150509295985092959890939650565b60008060006060848603121561490657600080fd5b83356149118161474b565b92506020848101356001600160401b038082111561492e57600080fd5b818701915087601f83011261494257600080fd5b813581811115614954576149546142ac565b614966601f8201601f1916850161430d565b9150808252888482850101111561497c57600080fd5b808484018584013760008482840101525080945050505061499f60408501614372565b90509250925092565b600081518084526020808501808196508360051b810191508286016000805b86811015614a3e578385038a52825180518087529087019087870190845b81811015614a2957835180516001600160a01b031684528a8101518b8501526040908101516001600160601b031690840152928901926060909201916001016149e5565b50509a87019a955050918501916001016149c7565b509298975050505050505050565b60208152600061477d60208301846149a8565b600060208284031215614a7157600080fd5b81356001600160401b03811115614a8757600080fd5b82016040818503121561477d57600080fd5b60008083601f840112614aab57600080fd5b5081356001600160401b03811115614ac257600080fd5b602083019150836020828501011115614ada57600080fd5b9250929050565b60008060008060008060808789031215614afa57600080fd5b8635614b058161474b565b95506020870135614b1581614360565b945060408701356001600160401b0380821115614b3157600080fd5b614b3d8a838b01614a99565b90965094506060890135915080821115614b5657600080fd5b818901915089601f830112614b6a57600080fd5b813581811115614b7957600080fd5b8a60208260051b8501011115614b8e57600080fd5b6020830194508093505050509295509295509295565b600081518084526020808501945080840160005b83811015614bda57815163ffffffff1687529582019590820190600101614bb8565b509495945050505050565b600081518084526020808501808196508360051b8101915082860160005b85811015614c2d578284038952614c1b848351614ba4565b98850198935090840190600101614c03565b5091979650505050505050565b602081526000825160806020840152614c5660a0840182614ba4565b90506020840151601f1980858403016040860152614c748383614ba4565b92506040860151915080858403016060860152614c918383614ba4565b9250606086015191508085840301608086015250614caf8282614be5565b95945050505050565b600080600060a08486031215614ccd57600080fd5b83356001600160401b0380821115614ce457600080fd5b614cf087838801614234565b9450614cff876020880161424c565b93506080860135915080821115614d1557600080fd5b5061474186828701614599565b600060208284031215614d3457600080fd5b81356001600160401b03811115614d4a57600080fd5b8201610120818503121561477d57600080fd5b600060208284031215614d6f57600080fd5b813560ff8116811461477d57600080fd5b600080600060608486031215614d9557600080fd5b8335614da08161474b565b92506020848101356001600160401b03811115614dbc57600080fd5b8501601f81018713614dcd57600080fd5b8035614ddb6143a38261433d565b81815260059190911b82018301908381019089831115614dfa57600080fd5b928401925b82841015614e1857833582529284019290840190614dff565b809650505050505061499f60408501614372565b6020808252825182820181905260009190848201906040850190845b81811015614e6457835183529284019291840191600101614e48565b50909695505050505050565b600080600060408486031215614e8557600080fd5b8335614e9081614360565b925060208401356001600160401b03811115614eab57600080fd5b614eb786828701614a99565b9497909650939450505050565b600080600080600060808688031215614edc57600080fd5b8535945060208601356001600160401b0380821115614efa57600080fd5b614f0689838a01614a99565b909650945060408801359150614f1b82614360565b90925060608701359080821115614f3157600080fd5b50614f3e88828901614599565b9150509295509295909350565b600081518084526020808501945080840160005b83811015614bda5781516001600160601b031687529582019590820190600101614f5f565b6040815260008351604080840152614f9f6080840182614f4b565b90506020850151603f19848303016060850152614fbc8282614f4b565b925050508260208301529392505050565b634e487b7160e01b600052602160045260246000fd5b602081016005831061500557634e487b7160e01b600052602160045260246000fd5b91905290565b60008060006060848603121561502057600080fd5b833561502b8161474b565b925060208401359150604084013561504281614360565b809150509250925092565b82815260406020820152600061506660408301846149a8565b949350505050565b60006020828403121561508057600080fd5b813561477d81614360565b6000808335601e198436030181126150a257600080fd5b8301803591506001600160401b038211156150bc57600080fd5b602001915036819003821315614ada57600080fd5b6000808335601e198436030181126150e857600080fd5b83016020810192503590506001600160401b0381111561510757600080fd5b803603831315614ada57600080fd5b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b602081526000823561515081614360565b63ffffffff8116602084015250602083013561516b81614360565b63ffffffff811660408401525061518460408401614372565b63ffffffff811660608401525061519e60608401846150d1565b60e060808501526151b461010085018284615116565b9150506151c360808501614372565b63ffffffff811660a0850152506151dd60a08501856150d1565b848303601f190160c08601526151f4838284615116565b9250505061520460c08501614372565b63ffffffff811660e08501525b509392505050565b6020808252603d908201527f737570706c696564207461736b20646f6573206e6f74206d617463682074686560408201527f206f6e65207265636f7264656420696e2074686520636f6e7472616374000000606082015260800190565b6020808252602c908201527f41676772656761746f722068617320616c726561647920726573706f6e64656460408201526b20746f20746865207461736b60a01b606082015260800190565b634e487b7160e01b600052601160045260246000fd5b600063ffffffff8083168185168083038211156152f7576152f76152c2565b01949350505050565b6020808252602d908201527f41676772656761746f722068617320726573706f6e64656420746f207468652060408201526c7461736b20746f6f206c61746560981b606082015260800190565b803561535881614360565b63ffffffff16825260208181013590830152604090810135910152565b63ffffffff81511682526020810151602083015260006040820151608060408501526153a46080850182614f4b565b905060608301518482036060860152614caf8282614f4b565b6153c7818461534d565b6080606082015260006150666080830184615375565b60608101614195828461534d565b602080825260059082015264041757468360dc1b604082015260600190565b602081526000823561541b81614360565b63ffffffff8082166020850152602085013560408501526040850135915061544282614360565b80821660608501526060850135915061545a82614360565b808216608085015261546f60808601866150d1565b925060c060a086015261548660e086018483615116565b92505060a085013561549781614360565b1660c0939093019290925250919050565b80356154b381614360565b63ffffffff168252602081810135908301526040808201359083015260608082013590830152608090810135910152565b60a0810161419582846154a8565b600081518084526020808501945080840160005b83811015614bda5761552387835180518252602090810151910152565b6040969096019590820190600101615506565b8060005b600281101561555957815184526020938401939091019060010161553a565b50505050565b61556a828251615536565b602081015161111c6040840182615536565b858152608060208201526000615596608083018688615116565b63ffffffff85166040840152828103606084015261018084518183526155be82840182614ba4565b915050602085015182820360208401526155d882826154f2565b915050604085015182820360408401526155f282826154f2565b9150506060850151615607606084018261555f565b506080850151805160e08401526020015161010083015260a08501518282036101208401526156368282614ba4565b91505060c08501518282036101408401526156518282614ba4565b91505060e085015182820361016084015261566c8282614be5565b9a9950505050505050505050565b80516001600160601b038116811461437d57600080fd5b600082601f8301126156a257600080fd5b815160206156b26143a38361433d565b82815260059290921b840181019181810190868411156156d157600080fd5b8286015b848110156143eb576156e68161567a565b83529183019183016156d5565b6000806040838503121561570657600080fd5b82516001600160401b038082111561571d57600080fd5b908401906040828703121561573157600080fd5b6157396142c2565b82518281111561574857600080fd5b61575488828601615691565b82525060208301518281111561576957600080fd5b61577588828601615691565b602083015250809450505050602083015190509250929050565b61579981846154a8565b60c060a0820152600061506660c0830184615375565b634e487b7160e01b600052603260045260246000fd5b60006001600160601b03808316818516818304811182151516156157eb576157eb6152c2565b02949350505050565b600081600019048311821515161561580e5761580e6152c2565b500290565b6000600019821415615827576158276152c2565b5060010190565b60006020828403121561584057600080fd5b815161477d8161474b565b6020808252602a908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526939903ab73830bab9b2b960b11b606082015260800190565b6000602082840312156158a757600080fd5b815161477d8161483a565b60208082526028908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526739903830bab9b2b960c11b606082015260800190565b600181811c9082168061590e57607f821691505b6020821081141561424657634e487b7160e01b600052602260045260246000fd5b6000602080838503121561594257600080fd5b82516001600160401b0381111561595857600080fd5b8301601f8101851361596957600080fd5b80516159776143a38261433d565b81815260059190911b8201830190838101908783111561599657600080fd5b928401925b828410156159b45783518252928401929084019061599b565b979650505050505050565b6000602082840312156159d157600080fd5b61477d8261567a565b63ffffffff84168152604060208201819052810182905260006001600160fb1b03831115615a0757600080fd5b8260051b8085606085013760009201606001918252509392505050565b60006020808385031215615a3757600080fd5b82516001600160401b03811115615a4d57600080fd5b8301601f81018513615a5e57600080fd5b8051615a6c6143a38261433d565b81815260059190911b82018301908381019087831115615a8b57600080fd5b928401925b828410156159b4578351615aa381614360565b82529284019290840190615a90565b63ffffffff84168152604060208201526000614caf604083018486615116565b600060208284031215615ae457600080fd5b81516001600160c01b038116811461477d57600080fd5b600060208284031215615b0d57600080fd5b815161477d81614360565b600060ff821660ff811415615b2f57615b2f6152c2565b60010192915050565b604081526000615b4c604083018587615116565b905063ffffffff83166020830152949350505050565b60006040820163ffffffff851683526020604081850152818551808452606086019150828701935060005b81811015615ba957845183529383019391830191600101615b8d565b5090979650505050505050565b60208082526039908201527f43616e2774206372656174652061207461736b20696e207468652073616d652060408201527f626c6f636b206173206120636f6d706c65746564207461736b00000000000000606082015260800190565b600063ffffffff83811690831681811015615c3057615c306152c2565b039392505050565b60208152600063ffffffff80845116602084015280602085015116604084015280604085015116606084015250606083015160e06080840152615c7f6101008401826147da565b90506080840151615c9860a085018263ffffffff169052565b5060a0840151838203601f190160c0850152615cb482826147da565b91505060c084015161521160e085018263ffffffff169052565b60208152600063ffffffff80845116602084015260208401516040840152806040850151166060840152806060850151166080840152608084015160c060a0850152615d1d60e08501826147da565b90508160a08601511660c0850152809250505092915050565b600082821015615d4857615d486152c2565b500390565b600061ffff80831681811415615d6557615d656152c2565b600101939250505056fe10afac9233b4ccc54d6404ffc1cf3b47515a2b8edbf675d15eddce05a027dcbd298c800d0881dd208d705ebc03eb18189f38118259f27dd43b4c60d61c607e87235d629dc802037ded8c61cb27fb29e40fa01b299719d8f991ffe20bdcc59f4fa2646970667358221220b619e49a0f16e3b0ca05da7d300105fc91061904f37723c69e0ff87c4d8f250564736f6c634300080c0033",
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

// LatestPendingStateHash is a free data retrieval call binding the contract method 0x4ae6b203.
//
// Solidity: function latestPendingStateHash() view returns(bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) LatestPendingStateHash(opts *bind.CallOpts) ([32]byte, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "latestPendingStateHash")

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// LatestPendingStateHash is a free data retrieval call binding the contract method 0x4ae6b203.
//
// Solidity: function latestPendingStateHash() view returns(bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) LatestPendingStateHash() ([32]byte, error) {
	return _ContractFinalizerTaskManager.Contract.LatestPendingStateHash(&_ContractFinalizerTaskManager.CallOpts)
}

// LatestPendingStateHash is a free data retrieval call binding the contract method 0x4ae6b203.
//
// Solidity: function latestPendingStateHash() view returns(bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) LatestPendingStateHash() ([32]byte, error) {
	return _ContractFinalizerTaskManager.Contract.LatestPendingStateHash(&_ContractFinalizerTaskManager.CallOpts)
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

// MinOpTaskResponseWindowBlock is a free data retrieval call binding the contract method 0xc987de8e.
//
// Solidity: function minOpTaskResponseWindowBlock() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) MinOpTaskResponseWindowBlock(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "minOpTaskResponseWindowBlock")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// MinOpTaskResponseWindowBlock is a free data retrieval call binding the contract method 0xc987de8e.
//
// Solidity: function minOpTaskResponseWindowBlock() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) MinOpTaskResponseWindowBlock() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.MinOpTaskResponseWindowBlock(&_ContractFinalizerTaskManager.CallOpts)
}

// MinOpTaskResponseWindowBlock is a free data retrieval call binding the contract method 0xc987de8e.
//
// Solidity: function minOpTaskResponseWindowBlock() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) MinOpTaskResponseWindowBlock() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.MinOpTaskResponseWindowBlock(&_ContractFinalizerTaskManager.CallOpts)
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

// CreateNewRdTask is a paid mutator transaction binding the contract method 0xb3ea184e.
//
// Solidity: function createNewRdTask(uint256 blockNumber) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) CreateNewRdTask(opts *bind.TransactOpts, blockNumber *big.Int) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "createNewRdTask", blockNumber)
}

// CreateNewRdTask is a paid mutator transaction binding the contract method 0xb3ea184e.
//
// Solidity: function createNewRdTask(uint256 blockNumber) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) CreateNewRdTask(blockNumber *big.Int) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.CreateNewRdTask(&_ContractFinalizerTaskManager.TransactOpts, blockNumber)
}

// CreateNewRdTask is a paid mutator transaction binding the contract method 0xb3ea184e.
//
// Solidity: function createNewRdTask(uint256 blockNumber) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) CreateNewRdTask(blockNumber *big.Int) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.CreateNewRdTask(&_ContractFinalizerTaskManager.TransactOpts, blockNumber)
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

// Initialize is a paid mutator transaction binding the contract method 0x2db2e39b.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _aggregator, address _generator, bool _allowNonRootInit, address _blsSignatureCheckerAddress, uint32 _taskResponseWindowBlock, uint32 _minOpTaskResponseWindowBlock) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) Initialize(opts *bind.TransactOpts, _pauserRegistry common.Address, initialOwner common.Address, _aggregator common.Address, _generator common.Address, _allowNonRootInit bool, _blsSignatureCheckerAddress common.Address, _taskResponseWindowBlock uint32, _minOpTaskResponseWindowBlock uint32) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "initialize", _pauserRegistry, initialOwner, _aggregator, _generator, _allowNonRootInit, _blsSignatureCheckerAddress, _taskResponseWindowBlock, _minOpTaskResponseWindowBlock)
}

// Initialize is a paid mutator transaction binding the contract method 0x2db2e39b.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _aggregator, address _generator, bool _allowNonRootInit, address _blsSignatureCheckerAddress, uint32 _taskResponseWindowBlock, uint32 _minOpTaskResponseWindowBlock) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) Initialize(_pauserRegistry common.Address, initialOwner common.Address, _aggregator common.Address, _generator common.Address, _allowNonRootInit bool, _blsSignatureCheckerAddress common.Address, _taskResponseWindowBlock uint32, _minOpTaskResponseWindowBlock uint32) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.Initialize(&_ContractFinalizerTaskManager.TransactOpts, _pauserRegistry, initialOwner, _aggregator, _generator, _allowNonRootInit, _blsSignatureCheckerAddress, _taskResponseWindowBlock, _minOpTaskResponseWindowBlock)
}

// Initialize is a paid mutator transaction binding the contract method 0x2db2e39b.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _aggregator, address _generator, bool _allowNonRootInit, address _blsSignatureCheckerAddress, uint32 _taskResponseWindowBlock, uint32 _minOpTaskResponseWindowBlock) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) Initialize(_pauserRegistry common.Address, initialOwner common.Address, _aggregator common.Address, _generator common.Address, _allowNonRootInit bool, _blsSignatureCheckerAddress common.Address, _taskResponseWindowBlock uint32, _minOpTaskResponseWindowBlock uint32) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.Initialize(&_ContractFinalizerTaskManager.TransactOpts, _pauserRegistry, initialOwner, _aggregator, _generator, _allowNonRootInit, _blsSignatureCheckerAddress, _taskResponseWindowBlock, _minOpTaskResponseWindowBlock)
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

// RespondToRdTask is a paid mutator transaction binding the contract method 0x05c6b663.
//
// Solidity: function respondToRdTask((uint32,uint256,uint32,uint32,bytes,uint32) task, (uint32,bytes32,bytes32,bytes32,bytes32) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) RespondToRdTask(opts *bind.TransactOpts, task IFinalizerTaskManagerRdTask, taskResponse IFinalizerTaskManagerRdTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "respondToRdTask", task, taskResponse, nonSignerStakesAndSignature)
}

// RespondToRdTask is a paid mutator transaction binding the contract method 0x05c6b663.
//
// Solidity: function respondToRdTask((uint32,uint256,uint32,uint32,bytes,uint32) task, (uint32,bytes32,bytes32,bytes32,bytes32) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) RespondToRdTask(task IFinalizerTaskManagerRdTask, taskResponse IFinalizerTaskManagerRdTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.RespondToRdTask(&_ContractFinalizerTaskManager.TransactOpts, task, taskResponse, nonSignerStakesAndSignature)
}

// RespondToRdTask is a paid mutator transaction binding the contract method 0x05c6b663.
//
// Solidity: function respondToRdTask((uint32,uint256,uint32,uint32,bytes,uint32) task, (uint32,bytes32,bytes32,bytes32,bytes32) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) RespondToRdTask(task IFinalizerTaskManagerRdTask, taskResponse IFinalizerTaskManagerRdTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.RespondToRdTask(&_ContractFinalizerTaskManager.TransactOpts, task, taskResponse, nonSignerStakesAndSignature)
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
	TaskIndex uint32
	Task      IFinalizerTaskManagerOpTask
	Raw       types.Log // Blockchain specific contextual infos
}

// FilterNewOpTaskForceCreated is a free log retrieval operation binding the contract event 0xdc5ba3b66ec6491b585b3f13698ed711aa829071f43300d6ede6dba67e28d75f.
//
// Solidity: event NewOpTaskForceCreated(uint32 indexed taskIndex, (uint32,uint32,uint32,bytes,uint32,bytes,uint32) task)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterNewOpTaskForceCreated(opts *bind.FilterOpts, taskIndex []uint32) (*ContractFinalizerTaskManagerNewOpTaskForceCreatedIterator, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "NewOpTaskForceCreated", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerNewOpTaskForceCreatedIterator{contract: _ContractFinalizerTaskManager.contract, event: "NewOpTaskForceCreated", logs: logs, sub: sub}, nil
}

// WatchNewOpTaskForceCreated is a free log subscription operation binding the contract event 0xdc5ba3b66ec6491b585b3f13698ed711aa829071f43300d6ede6dba67e28d75f.
//
// Solidity: event NewOpTaskForceCreated(uint32 indexed taskIndex, (uint32,uint32,uint32,bytes,uint32,bytes,uint32) task)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchNewOpTaskForceCreated(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerNewOpTaskForceCreated, taskIndex []uint32) (event.Subscription, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "NewOpTaskForceCreated", taskIndexRule)
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

// ParseNewOpTaskForceCreated is a log parse operation binding the contract event 0xdc5ba3b66ec6491b585b3f13698ed711aa829071f43300d6ede6dba67e28d75f.
//
// Solidity: event NewOpTaskForceCreated(uint32 indexed taskIndex, (uint32,uint32,uint32,bytes,uint32,bytes,uint32) task)
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

// FilterNewRdTaskCreated is a free log retrieval operation binding the contract event 0x161faa5d92f6bf0262139c57c87ca6cc60e5b3c9c3341dc175cec97a2516cc7d.
//
// Solidity: event NewRdTaskCreated(uint32 indexed taskIndex, (uint32,uint256,uint32,uint32,bytes,uint32) task)
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

// WatchNewRdTaskCreated is a free log subscription operation binding the contract event 0x161faa5d92f6bf0262139c57c87ca6cc60e5b3c9c3341dc175cec97a2516cc7d.
//
// Solidity: event NewRdTaskCreated(uint32 indexed taskIndex, (uint32,uint256,uint32,uint32,bytes,uint32) task)
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

// ParseNewRdTaskCreated is a log parse operation binding the contract event 0x161faa5d92f6bf0262139c57c87ca6cc60e5b3c9c3341dc175cec97a2516cc7d.
//
// Solidity: event NewRdTaskCreated(uint32 indexed taskIndex, (uint32,uint256,uint32,uint32,bytes,uint32) task)
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
	BlockHash    [32]byte
	TaskResponse IFinalizerTaskManagerRdTaskResponse
	Raw          types.Log // Blockchain specific contextual infos
}

// FilterRdTaskCompleted is a free log retrieval operation binding the contract event 0x02046773dfd0e022ab6bf68cb07d73f080f87a4b82f7a4fccfe7d919d6c4ecb4.
//
// Solidity: event RdTaskCompleted(uint32 indexed taskIndex, bytes32 indexed blockHash, (uint32,bytes32,bytes32,bytes32,bytes32) taskResponse)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterRdTaskCompleted(opts *bind.FilterOpts, taskIndex []uint32, blockHash [][32]byte) (*ContractFinalizerTaskManagerRdTaskCompletedIterator, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}
	var blockHashRule []interface{}
	for _, blockHashItem := range blockHash {
		blockHashRule = append(blockHashRule, blockHashItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "RdTaskCompleted", taskIndexRule, blockHashRule)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerRdTaskCompletedIterator{contract: _ContractFinalizerTaskManager.contract, event: "RdTaskCompleted", logs: logs, sub: sub}, nil
}

// WatchRdTaskCompleted is a free log subscription operation binding the contract event 0x02046773dfd0e022ab6bf68cb07d73f080f87a4b82f7a4fccfe7d919d6c4ecb4.
//
// Solidity: event RdTaskCompleted(uint32 indexed taskIndex, bytes32 indexed blockHash, (uint32,bytes32,bytes32,bytes32,bytes32) taskResponse)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchRdTaskCompleted(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerRdTaskCompleted, taskIndex []uint32, blockHash [][32]byte) (event.Subscription, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}
	var blockHashRule []interface{}
	for _, blockHashItem := range blockHash {
		blockHashRule = append(blockHashRule, blockHashItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "RdTaskCompleted", taskIndexRule, blockHashRule)
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

// ParseRdTaskCompleted is a log parse operation binding the contract event 0x02046773dfd0e022ab6bf68cb07d73f080f87a4b82f7a4fccfe7d919d6c4ecb4.
//
// Solidity: event RdTaskCompleted(uint32 indexed taskIndex, bytes32 indexed blockHash, (uint32,bytes32,bytes32,bytes32,bytes32) taskResponse)
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

// FilterRdTaskResponded is a free log retrieval operation binding the contract event 0x07ba8d7c4209c14e6d2ed5fd4542e9d8f47e1af3da0cbd9c50e2d0d3e87dfff0.
//
// Solidity: event RdTaskResponded(uint32 indexed taskIndex, (uint32,bytes32,bytes32,bytes32,bytes32) taskResponse, (uint32,bytes32,uint96[],uint96[]) taskResponseMetadata)
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

// WatchRdTaskResponded is a free log subscription operation binding the contract event 0x07ba8d7c4209c14e6d2ed5fd4542e9d8f47e1af3da0cbd9c50e2d0d3e87dfff0.
//
// Solidity: event RdTaskResponded(uint32 indexed taskIndex, (uint32,bytes32,bytes32,bytes32,bytes32) taskResponse, (uint32,bytes32,uint96[],uint96[]) taskResponseMetadata)
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

// ParseRdTaskResponded is a log parse operation binding the contract event 0x07ba8d7c4209c14e6d2ed5fd4542e9d8f47e1af3da0cbd9c50e2d0d3e87dfff0.
//
// Solidity: event RdTaskResponded(uint32 indexed taskIndex, (uint32,bytes32,bytes32,bytes32,bytes32) taskResponse, (uint32,bytes32,uint96[],uint96[]) taskResponseMetadata)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParseRdTaskResponded(log types.Log) (*ContractFinalizerTaskManagerRdTaskResponded, error) {
	event := new(ContractFinalizerTaskManagerRdTaskResponded)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "RdTaskResponded", log); err != nil {
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

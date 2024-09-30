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
	ABI: "[{\"type\":\"function\",\"name\":\"THRESHOLD_DENOMINATOR\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"aggregator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allTaskHashes\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumFinalizerTaskManager.TaskType\"},{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allTaskResponses\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumFinalizerTaskManager.TaskType\"},{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allowNonRootInit\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"blsApkRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIBLSApkRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"blsSignatureChecker\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractBLSSignatureChecker\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"checkSignatures\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"referenceBlockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.QuorumStakeTotals\",\"components\":[{\"name\":\"signedStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"totalStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"createNewOpTask\",\"inputs\":[{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"createNewRdTask\",\"inputs\":[{\"name\":\"blockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"delegation\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIDelegationManager\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"dummyForOperatorStateInfoType\",\"inputs\":[{\"name\":\"_operatorStateInfo\",\"type\":\"tuple\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorStateInfo\",\"components\":[{\"name\":\"operatorsStateChanged\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"quorumsRemoved\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsAdded[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"quorumsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsStakeUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]},{\"name\":\"quorumsApkUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsApkUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"operatorsRemoved\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"operatorsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsAdded[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]},{\"name\":\"operatorsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsStakeUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"operatorsQuorumCountUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsQuorumCountUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]}]}],\"outputs\":[],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"dummyForQuorumStakeTotalsType\",\"inputs\":[{\"name\":\"_quorumStakeTotals\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.QuorumStakeTotals\",\"components\":[{\"name\":\"signedStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"totalStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]}],\"outputs\":[],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"forceCreateNewOpTask\",\"inputs\":[{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"forceRespondToOpTask\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"generator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getBatchOperatorFromId\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"}],\"outputs\":[{\"name\":\"operators\",\"type\":\"address[]\",\"internalType\":\"address[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getBatchOperatorId\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operators\",\"type\":\"address[]\",\"internalType\":\"address[]\"}],\"outputs\":[{\"name\":\"operatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getCheckSignaturesIndices\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"referenceBlockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"nonSignerOperatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structOperatorStateRetriever.CheckSignaturesIndices\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getOperatorState\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple[][]\",\"internalType\":\"structOperatorStateRetriever.Operator[][]\",\"components\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"stake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getOperatorState\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"\",\"type\":\"tuple[][]\",\"internalType\":\"structOperatorStateRetriever.Operator[][]\",\"components\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"stake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getQuorumBitmapsAtBlockNumber\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256[]\",\"internalType\":\"uint256[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"idToTaskStatus\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumFinalizerTaskManager.TaskType\"},{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumFinalizerTaskManager.TaskStatus\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"_pauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"initialOwner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_aggregator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_generator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_allowNonRootInit\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"_blsSignatureCheckerAddress\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_taskResponseWindowBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"_minOpTaskResponseWindowBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"_operatorStateRetrieverExtended\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"lastCompletedOpTaskCreatedBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastCompletedOpTaskNum\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastOpTaskCreatedBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestOpTaskNum\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestPendingStateHash\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestRdTaskNum\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"minOpTaskResponseWindowBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"operatorStateRetrieverExtended\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"operatorsStateInfoHash\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"pauseAll\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"pauseTrackingOpState\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[{\"name\":\"index\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pauserRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"registryCoordinator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"respondToOpTask\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"respondToRdTask\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.RdTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"blockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.RdTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"storageProofHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"pendingStateHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"resumeTrackingQuorums\",\"inputs\":[{\"name\":\"resetTrackedQuorums\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setPauserRegistry\",\"inputs\":[{\"name\":\"newPauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"stakeRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIStakeRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"taskResponseWindowBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"unpause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateBlsSignatureCheckerAddress\",\"inputs\":[{\"name\":\"_blsSignatureCheckerAddress\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"BLSSignatureCheckerAddressUpdated\",\"inputs\":[{\"name\":\"blsSignatureCheckerAddress\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Initialized\",\"inputs\":[{\"name\":\"version\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewOpTaskCreated\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"task\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.OpTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewOpTaskForceCreated\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"task\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.OpTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewRdTaskCreated\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"task\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.RdTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"blockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OpTaskCancelled\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OpTaskCompleted\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OpTaskForceCompleted\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OpTaskResponded\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"taskResponseMetadata\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.TaskResponseMetadata\",\"components\":[{\"name\":\"taskResponsedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"hashOfNonSigners\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quroumStakeTotals\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quroumStakeSigned\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"PauseTrackingOpState\",\"inputs\":[],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Paused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"PauserRegistrySet\",\"inputs\":[{\"name\":\"pauserRegistry\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"newPauserRegistry\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractIPauserRegistry\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RdTaskCancelled\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RdTaskCompleted\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"blockHash\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.RdTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"storageProofHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"pendingStateHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RdTaskResponded\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.RdTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"storageProofHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"pendingStateHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"taskResponseMetadata\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.TaskResponseMetadata\",\"components\":[{\"name\":\"taskResponsedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"hashOfNonSigners\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quroumStakeTotals\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quroumStakeSigned\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"ResumeTrackingOpState\",\"inputs\":[{\"name\":\"resetTrackedQuorums\",\"type\":\"bool\",\"indexed\":false,\"internalType\":\"bool\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"StaleStakesForbiddenUpdate\",\"inputs\":[{\"name\":\"value\",\"type\":\"bool\",\"indexed\":false,\"internalType\":\"bool\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Unpaused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false}]",
	Bin: "0x608060405234801561001057600080fd5b5061626280620000216000396000f3fe608060405234801561001057600080fd5b50600436106103275760003560e01c80635c975abb116101b85780638da5cb5b11610104578063cefdc1d4116100a2578063ef0244581161007c578063ef0244581461077f578063f2fde38b14610787578063f5640cf81461079a578063fabc1cbc146107ad57600080fd5b8063cefdc1d414610746578063df5cf72314610767578063e70c26231461076f57600080fd5b8063adfcb048116100de578063adfcb048146106d8578063b3ea184e146106e1578063bf2315ed146106f4578063c987de8e1461072f57600080fd5b80638da5cb5b146106995780638fc8729a146106aa578063a69563a9146106c157600080fd5b8063715018a6116101715780637afa1eed1161014b5780637afa1eed146106505780637afdd54b146106635780638380acbd14610673578063886f11951461068657600080fd5b8063715018a61461062d578063723114ab1461063557806379badf731461064857600080fd5b80635c975abb146105d95780635df45946146105e157806368304835146105e95780636d14a987146105f15780636e125ff4146105f95780636efb46361461060c57600080fd5b80633563b0d1116102775780634f739f741161023057806354d127de1161020a57806354d127de14610580578063595c6a671461058e5780635ac86ab7146105965780635c155662146105b957600080fd5b80634f739f7414610536578063516a722714610556578063537a29291461056957600080fd5b80633563b0d1146104a057806341789d57146104c057806345265b7a146104ec5780634ae6b203146104fd5780634d2b57fe146105065780634d7a71161461052657600080fd5b8063191aac7a116102e4578063245a7bfc116102be578063245a7bfc1461043e5780632830e8f9146104585780632d4713db1461046d57806331b36bd91461048057600080fd5b8063191aac7a146103d55780631ac27297146103e85780631c178e9c1461041357600080fd5b806301a3f0131461032c57806305c6b663146103415780630ee0fdbd1461035457806310d67a2f14610376578063136439dd1461038957806313f815ed1461039c575b600080fd5b61033f61033a36600461451f565b6107c0565b005b61033f61034f366004614984565b610c51565b60a3546103619060ff1681565b60405190151581526020015b60405180910390f35b61033f610384366004614a21565b611216565b61033f610397366004614a45565b6112c6565b6103c76103aa366004614a5e565b609a60209081526000928352604080842090915290825290205481565b60405190815260200161036d565b61033f6103e3366004614aa9565b611405565b6103c76103f6366004614a5e565b609b60209081526000928352604080842090915290825290205481565b609754610426906001600160a01b031681565b6040516001600160a01b03909116815260200161036d565b609f5461042690600160201b90046001600160a01b031681565b610460611446565b60405161036d9190614b13565b61033f61047b366004614b26565b6114d4565b61049361048e366004614be2565b61169e565b60405161036d9190614ccb565b6104b36104ae366004614cde565b6117ba565b60405161036d9190614e39565b6098546104d790600160e01b900463ffffffff1681565b60405163ffffffff909116815260200161036d565b61033f6104fa366004614e4c565b50565b6103c760a15481565b610519610514366004614ee1565b611c50565b60405161036d9190614f30565b609d546104d79063ffffffff1681565b610549610544366004614fc5565b611d65565b60405161036d9190615113565b61033f610564366004615191565b61248b565b609d546104d790600160401b900463ffffffff1681565b61033f6104fa3660046151fb565b61033f612b83565b6103616105a4366004615236565b606654600160ff9092169190911b9081161490565b6105cc6105c7366004615259565b612c4a565b60405161036d91906152bc565b6066546103c7565b610426612e12565b610426612e85565b610426612ecf565b61033f6106073660046152f4565b612f19565b61061f61061a366004615348565b61338d565b60405161036d929190615408565b61033f61342d565b61033f610643366004614a21565b613441565b61033f613497565b60a054610426906001600160a01b031681565b6099546104d79063ffffffff1681565b609854610426906001600160a01b031681565b606554610426906001600160a01b031681565b6033546001600160a01b0316610426565b609d546104d790600160201b900463ffffffff1681565b6098546104d790600160a01b900463ffffffff1681565b6103c760a25481565b61033f6106ef366004614a45565b6134ca565b610722610702366004614a5e565b609c60209081526000928352604080842090915290825290205460ff1681565b60405161036d9190615467565b6098546104d790600160c01b900463ffffffff1681565b61075961075436600461548f565b6138e3565b60405161036d9291906154c6565b610426613a75565b609f546104d79063ffffffff1681565b6103c7606481565b61033f610795366004614a21565b613abf565b61033f6107a83660046152f4565b613b35565b61033f6107bb366004614a45565b613f76565b6107c86140d2565b60006107d760208301836154e7565b905060006107eb60608501604086016154e7565b905060006107ff60408601602087016154e7565b905036600061081160a0880188615504565b9092509050600061082860e0890160c08a016154e7565b6000808052609a60209081529192506000805160206161ed83398151915291610853908a018a6154e7565b63ffffffff1663ffffffff168152602001908152602001600020548860405160200161087f91906155b8565b60405160208183030381529060405280519060200120146108bb5760405162461bcd60e51b81526004016108b290615692565b60405180910390fd5b6000808052609c602090815260019160008051602061620d833981519152916108e6908b018b6154e7565b63ffffffff16815260208101919091526040016000205460ff16600481111561091157610911615451565b1461092e5760405162461bcd60e51b81526004016108b2906156ef565b6000808052609b60209081527f10afac9233b4ccc54d6404ffc1cf3b47515a2b8edbf675d15eddce05a027dcbd90829061096a908b018b6154e7565b63ffffffff1663ffffffff16815260200190815260200160002054146109a25760405162461bcd60e51b81526004016108b2906156ef565b6098546109bc90600160a01b900463ffffffff1685615751565b63ffffffff164363ffffffff1611156109e75760405162461bcd60e51b81526004016108b290615779565b60408051808201909152606080825260208201526040805160808101825263ffffffff431681526000602080830182905284810151838501528451606084015292519092610a39918c91849101615836565b60408051601f1981840301815291905280516020918201206000808052609b835290917f10afac9233b4ccc54d6404ffc1cf3b47515a2b8edbf675d15eddce05a027dcbd9190610a8b908e018e6154e7565b63ffffffff16815260208082019290925260409081016000908120939093558c013560a255818052609c815260049160008051602061620d83398151915291610ad6908e018e6154e7565b63ffffffff16815260208101919091526040016000205460ff166004811115610b0157610b01615451565b50610b14905060408c0160208d016154e7565b609d805463ffffffff92909216600160401b0263ffffffff60401b19909216919091179055610b4660608c018c615504565b610b5291609e9161445c565b50610b6360a08c0160808d016154e7565b609f805463ffffffff191663ffffffff92909216919091179055610b8a60208c018c6154e7565b609d805463ffffffff92909216600160201b0267ffffffff0000000019909216919091179055610bbd60208b018b6154e7565b63ffffffff167fff2908483d74b6b70053dd473260acf1b09e0ba0781bf94100bb8277581749de8b604051610bf29190615856565b60405180910390a2610c0760208b018b6154e7565b63ffffffff167fdf22f3558e4841b63d77179546b3eae63e4e343bbe752746b093162bc526be4c8b604051610c3c9190615856565b60405180910390a25050505050505050505050565b609f54600160201b90046001600160a01b03163314610c825760405162461bcd60e51b81526004016108b290615864565b6000610c9460808501606086016154e7565b90506000610ca860608601604087016154e7565b9050366000610cba6080880188615504565b90925090506000610cd160c0890160a08a016154e7565b60016000908152609a60209081529192507f5b542b52981c4f2fa9965514d5bb7f37f1b7bc0902a6a4dc6b04dc05be85586b91610d10908a018a6154e7565b63ffffffff1663ffffffff1681526020019081526020016000205488604051602001610d3c9190615883565b6040516020818303038152906040528051906020012014610d6f5760405162461bcd60e51b81526004016108b290615692565b60016000818152609c60209081526000805160206161cd8339815191529190610d9a908b018b6154e7565b63ffffffff16815260208101919091526040016000205460ff166004811115610dc557610dc5615451565b14610de25760405162461bcd60e51b81526004016108b2906156ef565b60016000908152609b60209081527f298c800d0881dd208d705ebc03eb18189f38118259f27dd43b4c60d61c607e87908290610e20908b018b6154e7565b63ffffffff1663ffffffff1681526020019081526020016000205414610e585760405162461bcd60e51b81526004016108b2906156ef565b609854610e7290600160a01b900463ffffffff1685615751565b63ffffffff164363ffffffff161115610e9d5760405162461bcd60e51b81526004016108b290615779565b600087604051602001610eb0919061595d565b604051602081830303815290604052805190602001209050600080609760009054906101000a90046001600160a01b03166001600160a01b0316636efb46368488888c8e6040518663ffffffff1660e01b8152600401610f149594939291906159f5565b600060405180830381865afa158015610f31573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610f599190810190615b6c565b6040805160808101825263ffffffff43168152602080820184905280850151828401528451606083015291519395509193509091610f9b918d91849101615c08565b60408051601f19818403018152919052805160209182012060016000908152609b835290917f298c800d0881dd208d705ebc03eb18189f38118259f27dd43b4c60d61c607e879190610fef908f018f6154e7565b63ffffffff1681526020810191909152604001600020556003609c600060018081111561101e5761101e615451565b815260200190815260200160002060008d600001602081019061104191906154e7565b63ffffffff16815260208101919091526040016000205460ff16600481111561106c5761106c615451565b5061107c905060208d018d6154e7565b63ffffffff167f07ba8d7c4209c14e6d2ed5fd4542e9d8f47e1af3da0cbd9c50e2d0d3e87dfff08c836040516110b3929190615c08565b60405180910390a260005b86811015611155578560ff16846020015182815181106110e0576110e0615c28565b60200260200101516110f29190615c3e565b6001600160601b031660648560000151838151811061111357611113615c28565b60200260200101516001600160601b031661112e9190615c6d565b10156111435750505050505050505050505050565b8061114d81615c8c565b9150506110be565b5060808b013560a15560016000908152609c60209081526004916000805160206161cd8339815191529161118b908f018f6154e7565b63ffffffff16815260208101919091526040016000205460ff1660048111156111b6576111b6615451565b505060408b01356111ca60208d018d6154e7565b63ffffffff167f02046773dfd0e022ab6bf68cb07d73f080f87a4b82f7a4fccfe7d919d6c4ecb48d6040516111ff919061595d565b60405180910390a35050505050505050505b505050565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611269573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061128d9190615ca7565b6001600160a01b0316336001600160a01b0316146112bd5760405162461bcd60e51b81526004016108b290615cc4565b6104fa8161412c565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa15801561130e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906113329190615d0e565b61134e5760405162461bcd60e51b81526004016108b290615d2b565b606654818116146113c75760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e70617573653a20696e76616c696420617474656d70742060448201527f746f20756e70617573652066756e6374696f6e616c697479000000000000000060648201526084016108b2565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b61140d6140d2565b60405181151581527f6af4ae1f481aff20ce571abd65375b67b22359883a823d1ddf4bd8f2879ff7ba906020015b60405180910390a150565b609e805461145390615d73565b80601f016020809104026020016040519081016040528092919081815260200182805461147f90615d73565b80156114cc5780601f106114a1576101008083540402835291602001916114cc565b820191906000526020600020905b8154815290600101906020018083116114af57829003601f168201915b505050505081565b600054610100900460ff16158080156114f45750600054600160ff909116105b8061150e5750303b15801561150e575060005460ff166001145b6115715760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084016108b2565b6000805460ff191660011790558015611594576000805461ff0019166101001790555b61159f8a6000614223565b6115a88961430d565b609f80546001600160a01b03808b16600160201b02640100000000600160c01b03199092169190911790915560a080548983166001600160a01b03199182161790915560a3805489151560ff199091161790556097805488841692169190911790556098805463ffffffff868116600160c01b0263ffffffff60c01b19918916600160a01b026001600160c01b03199093169487169490941791909117169190911790558015611692576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050505050505050565b606081516001600160401b038111156116b9576116b961456d565b6040519080825280602002602001820160405280156116e2578160200160208202803683370190505b50905060005b82518110156117b357836001600160a01b03166313542a4e84838151811061171257611712615c28565b60200260200101516040518263ffffffff1660e01b815260040161174591906001600160a01b0391909116815260200190565b602060405180830381865afa158015611762573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906117869190615da8565b82828151811061179857611798615c28565b60209081029190910101526117ac81615c8c565b90506116e8565b5092915050565b60606000846001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa1580156117fc573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906118209190615ca7565b90506000856001600160a01b0316639e9923c26040518163ffffffff1660e01b8152600401602060405180830381865afa158015611862573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906118869190615ca7565b90506000866001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa1580156118c8573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906118ec9190615ca7565b9050600086516001600160401b038111156119095761190961456d565b60405190808252806020026020018201604052801561193c57816020015b60608152602001906001900390816119275790505b50905060005b8751811015611c4457600088828151811061195f5761195f615c28565b0160200151604051638902624560e01b815260f89190911c6004820181905263ffffffff8a16602483015291506000906001600160a01b03871690638902624590604401600060405180830381865afa1580156119c0573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526119e89190810190615dc1565b905080516001600160401b03811115611a0357611a0361456d565b604051908082528060200260200182016040528015611a4e57816020015b6040805160608101825260008082526020808301829052928201528252600019909201910181611a215790505b50848481518110611a6157611a61615c28565b602002602001018190525060005b8151811015611c2e576040518060600160405280876001600160a01b03166347b314e8858581518110611aa457611aa4615c28565b60200260200101516040518263ffffffff1660e01b8152600401611aca91815260200190565b602060405180830381865afa158015611ae7573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611b0b9190615ca7565b6001600160a01b03168152602001838381518110611b2b57611b2b615c28565b60200260200101518152602001896001600160a01b031663fa28c627858581518110611b5957611b59615c28565b60209081029190910101516040516001600160e01b031960e084901b168152600481019190915260ff8816602482015263ffffffff8f166044820152606401602060405180830381865afa158015611bb5573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611bd99190615e51565b6001600160601b0316815250858581518110611bf757611bf7615c28565b60200260200101518281518110611c1057611c10615c28565b60200260200101819052508080611c2690615c8c565b915050611a6f565b5050508080611c3c90615c8c565b915050611942565b50979650505050505050565b606081516001600160401b03811115611c6b57611c6b61456d565b604051908082528060200260200182016040528015611c94578160200160208202803683370190505b50905060005b82518110156117b357836001600160a01b031663296bb064848381518110611cc457611cc4615c28565b60200260200101516040518263ffffffff1660e01b8152600401611cea91815260200190565b602060405180830381865afa158015611d07573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611d2b9190615ca7565b828281518110611d3d57611d3d615c28565b6001600160a01b0390921660209283029190910190910152611d5e81615c8c565b9050611c9a565b611d906040518060800160405280606081526020016060815260200160608152602001606081525090565b6000876001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa158015611dd0573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611df49190615ca7565b9050611e216040518060800160405280606081526020016060815260200160608152602001606081525090565b6040516361c8a12f60e11b81526001600160a01b038a169063c391425e90611e51908b9089908990600401615e6c565b600060405180830381865afa158015611e6e573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611e969190810190615eb6565b81526040516340e03a8160e11b81526001600160a01b038316906381c0750290611ec8908b908b908b90600401615f44565b600060405180830381865afa158015611ee5573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611f0d9190810190615eb6565b6040820152856001600160401b03811115611f2a57611f2a61456d565b604051908082528060200260200182016040528015611f5d57816020015b6060815260200190600190039081611f485790505b50606082015260005b60ff811687111561239c576000856001600160401b03811115611f8b57611f8b61456d565b604051908082528060200260200182016040528015611fb4578160200160208202803683370190505b5083606001518360ff1681518110611fce57611fce615c28565b602002602001018190525060005b8681101561229c5760008c6001600160a01b03166304ec63518a8a8581811061200757612007615c28565b905060200201358e8860000151868151811061202557612025615c28565b60200260200101516040518463ffffffff1660e01b81526004016120629392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa15801561207f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906120a39190615f64565b90506001600160c01b0381166121475760405162461bcd60e51b815260206004820152605c60248201527f4f70657261746f7253746174655265747269657665722e676574436865636b5360448201527f69676e617475726573496e64696365733a206f70657261746f72206d7573742060648201527f6265207265676973746572656420617420626c6f636b6e756d62657200000000608482015260a4016108b2565b8a8a8560ff1681811061215c5761215c615c28565b6001600160c01b03841692013560f81c9190911c60019081161415905061228957856001600160a01b031663dd9846b98a8a8581811061219e5761219e615c28565b905060200201358d8d8860ff168181106121ba576121ba615c28565b6040516001600160e01b031960e087901b1681526004810194909452919091013560f81c60248301525063ffffffff8f166044820152606401602060405180830381865afa158015612210573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906122349190615f8d565b85606001518560ff168151811061224d5761224d615c28565b6020026020010151848151811061226657612266615c28565b63ffffffff909216602092830291909101909101528261228581615c8c565b9350505b508061229481615c8c565b915050611fdc565b506000816001600160401b038111156122b7576122b761456d565b6040519080825280602002602001820160405280156122e0578160200160208202803683370190505b50905060005b828110156123615784606001518460ff168151811061230757612307615c28565b6020026020010151818151811061232057612320615c28565b602002602001015182828151811061233a5761233a615c28565b63ffffffff909216602092830291909101909101528061235981615c8c565b9150506122e6565b508084606001518460ff168151811061237c5761237c615c28565b60200260200101819052505050808061239490615faa565b915050611f66565b506000896001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa1580156123dd573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906124019190615ca7565b60405163354952a360e21b81529091506001600160a01b0382169063d5254a8c90612434908b908b908e90600401615fca565b600060405180830381865afa158015612451573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526124799190810190615eb6565b60208301525098975050505050505050565b609d54600160401b900463ffffffff161560006124ae60608601604087016154e7565b905081156125395760a35460ff16156124f757609f54600160201b90046001600160a01b031633146124f25760405162461bcd60e51b81526004016108b290615864565b61256a565b6033546001600160a01b031633146124f25760405162461bcd60e51b815260206004820152600560248201526420baba341960d91b60448201526064016108b2565b609f54600160201b90046001600160a01b0316331461256a5760405162461bcd60e51b81526004016108b290615864565b600061257c60408701602088016154e7565b905036600061258e60a0890189615504565b909250905060006125a560e08a0160c08b016154e7565b6000808052609a60209081529192506000805160206161ed833981519152916125d0908b018b6154e7565b63ffffffff1663ffffffff16815260200190815260200160002054896040516020016125fc91906155b8565b604051602081830303815290604052805190602001201461262f5760405162461bcd60e51b81526004016108b290615692565b6000808052609c602090815260019160008051602061620d8339815191529161265a908c018c6154e7565b63ffffffff16815260208101919091526040016000205460ff16600481111561268557612685615451565b146126a25760405162461bcd60e51b81526004016108b2906156ef565b6000808052609b60209081527f10afac9233b4ccc54d6404ffc1cf3b47515a2b8edbf675d15eddce05a027dcbd9082906126de908c018c6154e7565b63ffffffff1663ffffffff16815260200190815260200160002054146127165760405162461bcd60e51b81526004016108b2906156ef565b60985461273090600160a01b900463ffffffff1685615751565b63ffffffff164363ffffffff16111561275b5760405162461bcd60e51b81526004016108b290615779565b60008860405160200161276e9190615856565b60408051601f19818403018152828252805160209182012083830190925260608084529083015291506000881561283357609760009054906101000a90046001600160a01b03166001600160a01b0316636efb46368488888c8f6040518663ffffffff1660e01b81526004016127e89594939291906159f5565b600060405180830381865afa158015612805573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261282d9190810190615b6c565b90925090505b6040805160808101825263ffffffff4316815260208082018490528481015182840152845160608301529151909161286f918e91849101615836565b60408051808303601f190181529190528051602090910120609b600080815260200190815260200160002060008e60000160208101906128af91906154e7565b63ffffffff1681526020810191909152604001600020556003609c60008060018111156128de576128de615451565b815260200190815260200160002060008e600001602081019061290191906154e7565b63ffffffff16815260208101919091526040016000205460ff16600481111561292c5761292c615451565b5061293c905060208e018e6154e7565b63ffffffff167f47adacb0b6bbd726ae39ac6c006cca1c2006c9aedaa882dcba7c4804db7c41ce8d83604051612973929190615836565b60405180910390a28915612a1e5760005b86811015612a1c578560ff16846020015182815181106129a6576129a6615c28565b60200260200101516129b89190615c3e565b6001600160601b03166064856000015183815181106129d9576129d9615c28565b60200260200101516001600160601b03166129f49190615c6d565b1015612a0a575050505050505050505050505050565b80612a1481615c8c565b915050612984565b505b60408c013560a2556004609c600080815260200190815260200160002060008e6000016020810190612a5091906154e7565b63ffffffff16815260208101919091526040016000205460ff166004811115612a7b57612a7b615451565b50612a8e905060408e0160208f016154e7565b609d805463ffffffff92909216600160401b0263ffffffff60401b19909216919091179055612ac060608e018e615504565b612acc91609e9161445c565b50612add60a08e0160808f016154e7565b609f805463ffffffff191663ffffffff92909216919091179055612b0460208e018e6154e7565b609d805463ffffffff92909216600160201b0267ffffffff0000000019909216919091179055612b3760208d018d6154e7565b63ffffffff167fff2908483d74b6b70053dd473260acf1b09e0ba0781bf94100bb8277581749de8d604051612b6c9190615856565b60405180910390a250505050505050505050505050565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa158015612bcb573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612bef9190615d0e565b612c0b5760405162461bcd60e51b81526004016108b290615d2b565b600019606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b60606000846001600160a01b031663c391425e84866040518363ffffffff1660e01b8152600401612c7c929190615ff4565b600060405180830381865afa158015612c99573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052612cc19190810190615eb6565b9050600084516001600160401b03811115612cde57612cde61456d565b604051908082528060200260200182016040528015612d07578160200160208202803683370190505b50905060005b8551811015612e0857866001600160a01b03166304ec6351878381518110612d3757612d37615c28565b602002602001015187868581518110612d5257612d52615c28565b60200260200101516040518463ffffffff1660e01b8152600401612d8f9392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa158015612dac573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612dd09190615f64565b6001600160c01b0316828281518110612deb57612deb615c28565b602090810291909101015280612e0081615c8c565b915050612d0d565b5095945050505050565b60975460408051632efa2ca360e11b815290516000926001600160a01b031691635df459469160048083019260209291908290030181865afa158015612e5c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612e809190615ca7565b905090565b60975460408051636830483560e01b815290516000926001600160a01b03169163683048359160048083019260209291908290030181865afa158015612e5c573d6000803e3d6000fd5b60975460408051636d14a98760e01b815290516000926001600160a01b031691636d14a9879160048083019260209291908290030181865afa158015612e5c573d6000803e3d6000fd5b60a0546001600160a01b03163314612f5b5760405162461bcd60e51b8152602060048201526005602482015264417574683160d81b60448201526064016108b2565b609d5463ffffffff600160401b909104164314801590612f7a57504315155b612f965760405162461bcd60e51b81526004016108b290616013565b6040805160e0810182526000818301819052606080830181905260a083015260c0820152609854600160e01b900463ffffffff908116825243811660208084019190915290861660808301528251601f850182900482028101820190935283835290919084908490819084018382808284376000920191909152505050506060820152609d54600160401b900463ffffffff166130855763ffffffff431660408083019190915280516020601f850181900481028201810190925283815290849084908190840183828082843760009201919091525050505060a082015263ffffffff841660c0820152613139565b609d54600160401b900463ffffffff166040820152609e80546130a790615d73565b80601f01602080910402602001604051908101604052809291908181526020018280546130d390615d73565b80156131205780601f106130f557610100808354040283529160200191613120565b820191906000526020600020905b81548152906001019060200180831161310357829003601f168201915b505050505060a0820152609f5463ffffffff1660c08201525b609854600160e01b900463ffffffff16156131e05760985460009061316d90600190600160e01b900463ffffffff16616070565b63ffffffff8116600090815260008051602061620d833981519152602052604090205490915060019060ff1660048111156131aa576131aa615451565b14156131de5763ffffffff8116600090815260008051602061620d83398151915260205260409020805460ff191660021790555b505b60995463ffffffff1615613279576099546000906132069060019063ffffffff16616070565b63ffffffff811660009081526000805160206161cd833981519152602052604090205490915060019060ff16600481111561324357613243615451565b14156132775763ffffffff811660009081526000805160206161cd83398151915260205260409020805460ff191660021790555b505b8060405160200161328a9190616095565b60408051601f1981840301815282825280516020918201206098805463ffffffff600160e01b91829004811660009081526000805160206161ed83398151915286528681209490945582548290048116845260008051602061620d833981519152909452939091208054600160ff19909116179055609d805463ffffffff1916438416179055549190910416907ffaf4b2054479d0f83e909b73cde2a6cb18ec2a93ba8ad5a62329001c86b1f3ea90613344908490616095565b60405180910390a260985461336790600160e01b900463ffffffff166001615751565b6098601c6101000a81548163ffffffff021916908363ffffffff16021790555050505050565b604080518082019091526060808252602082015260975460405163377da31b60e11b81526000916001600160a01b031690636efb4636906133da908a908a908a908a908a906004016159f5565b600060405180830381865afa1580156133f7573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261341f9190810190615b6c565b915091509550959350505050565b6134356140d2565b61343f600061430d565b565b6134496140d2565b609780546001600160a01b0319166001600160a01b0383169081179091556040519081527f901a654dc830c94e8a12c9a3bc0a92ac11b5cf28046ca8d190691cdaf52090169060200161143b565b61349f6140d2565b6040517f4d60154266b2ea0c8f091d257eac5abc941c46cb54d0c3069a830f6339fe1da190600090a1565b60a0546001600160a01b0316331461350c5760405162461bcd60e51b8152602060048201526005602482015264417574683160d81b60448201526064016108b2565b609d54600160401b900463ffffffff161580159061352957504315155b6135675760405162461bcd60e51b815260206004820152600f60248201526e13dc0814dd185d19481d5b9a5b9a5d608a1b60448201526064016108b2565b6099546098546040805160c0810182526000606080830191909152608082015263ffffffff9384168082526020820186905243851692820192909252609f54841660a0820152609e80549294600160e01b9094049093169290916135ca90615d73565b80601f01602080910402602001604051908101604052809291908181526020018280546135f690615d73565b80156136435780601f1061361857610100808354040283529160200191613643565b820191906000526020600020905b81548152906001019060200180831161362657829003601f168201915b50505050506080820152609d54600160401b900463ffffffff90811660608301528316156136eb576000613678600185616070565b63ffffffff811660009081526000805160206161cd833981519152602052604090205490915060019060ff1660048111156136b5576136b5615451565b14156136e95763ffffffff811660009081526000805160206161cd83398151915260205260409020805460ff191660021790555b505b63ffffffff821615613802576000613704600184616070565b63ffffffff8116600090815260008051602061620d833981519152602052604090205490915060019060ff16600481111561374157613741615451565b141561380057609854609d546137679163ffffffff600160c01b90910481169116615751565b63ffffffff164363ffffffff1611156137d25760405162461bcd60e51b815260206004820152602760248201527f52645461736b2063616e2774207965742063616e63656c2074686520696e6974604482015266204f705461736b60c81b60648201526084016108b2565b63ffffffff8116600090815260008051602061620d83398151915260205260409020805460ff191660021790555b505b80604051602001613813919061612b565b60408051601f19818403018152828252805160209182012063ffffffff871660008181527f5b542b52981c4f2fa9965514d5bb7f37f1b7bc0902a6a4dc6b04dc05be85586b8452848120929092556000805160206161cd83398151915290925291909120805460ff19166001179055907f161faa5d92f6bf0262139c57c87ca6cc60e5b3c9c3341dc175cec97a2516cc7d906138b090849061612b565b60405180910390a26138c3836001615751565b6099805463ffffffff191663ffffffff9290921691909117905550505050565b604080516001808252818301909252600091606091839160208083019080368337019050509050848160008151811061391e5761391e615c28565b60209081029190910101526040516361c8a12f60e11b81526000906001600160a01b0388169063c391425e9061395a9088908690600401615ff4565b600060405180830381865afa158015613977573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261399f9190810190615eb6565b6000815181106139b1576139b1615c28565b60209081029190910101516040516304ec635160e01b81526004810188905263ffffffff87811660248301529091166044820181905291506000906001600160a01b038916906304ec635190606401602060405180830381865afa158015613a1d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613a419190615f64565b6001600160c01b031690506000613a578261435f565b905081613a658a838a6117ba565b9550955050505050935093915050565b6097546040805163df5cf72360e01b815290516000926001600160a01b03169163df5cf7239160048083019260209291908290030181865afa158015612e5c573d6000803e3d6000fd5b613ac76140d2565b6001600160a01b038116613b2c5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016108b2565b6104fa8161430d565b613b3d6140d2565b609d5463ffffffff600160401b909104164314801590613b5c57504315155b613b785760405162461bcd60e51b81526004016108b290616013565b6040805160e0810182526000818301819052606080830181905260a083015260c0820152609854600160e01b900463ffffffff908116825243811660208084019190915290861660808301528251601f850182900482028101820190935283835290919084908490819084018382808284376000920191909152505050506060820152609d54600160401b900463ffffffff16613c675763ffffffff431660408083019190915280516020601f850181900481028201810190925283815290849084908190840183828082843760009201919091525050505060a082015263ffffffff841660c0820152613d1b565b609d54600160401b900463ffffffff166040820152609e8054613c8990615d73565b80601f0160208091040260200160405190810160405280929190818152602001828054613cb590615d73565b8015613d025780601f10613cd757610100808354040283529160200191613d02565b820191906000526020600020905b815481529060010190602001808311613ce557829003601f168201915b505050505060a0820152609f5463ffffffff1660c08201525b609854600160e01b900463ffffffff1615613dc257609854600090613d4f90600190600160e01b900463ffffffff16616070565b63ffffffff8116600090815260008051602061620d833981519152602052604090205490915060019060ff166004811115613d8c57613d8c615451565b1415613dc05763ffffffff8116600090815260008051602061620d83398151915260205260409020805460ff191660021790555b505b60995463ffffffff1615613e5b57609954600090613de89060019063ffffffff16616070565b63ffffffff811660009081526000805160206161cd833981519152602052604090205490915060019060ff166004811115613e2557613e25615451565b1415613e595763ffffffff811660009081526000805160206161cd83398151915260205260409020805460ff191660021790555b505b80604051602001613e6c9190616095565b60408051601f1981840301815282825280516020918201206098805463ffffffff600160e01b91829004811660009081526000805160206161ed83398151915286528681209490945582548290048116845260008051602061620d833981519152909452939091208054600160ff19909116179055609d805463ffffffff1916438416179055549190910416907ffaf4b2054479d0f83e909b73cde2a6cb18ec2a93ba8ad5a62329001c86b1f3ea90613f26908490616095565b60405180910390a26098601c9054906101000a900463ffffffff1663ffffffff167fdc5ba3b66ec6491b585b3f13698ed711aa829071f43300d6ede6dba67e28d75f826040516133449190616095565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613fc9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613fed9190615ca7565b6001600160a01b0316336001600160a01b03161461401d5760405162461bcd60e51b81526004016108b290615cc4565b60665419811960665419161461409b5760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e756e70617573653a20696e76616c696420617474656d7060448201527f7420746f2070617573652066756e6374696f6e616c697479000000000000000060648201526084016108b2565b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c906020016113fa565b6033546001600160a01b0316331461343f5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016108b2565b6001600160a01b0381166141ba5760405162461bcd60e51b815260206004820152604960248201527f5061757361626c652e5f73657450617573657252656769737472793a206e657760448201527f50617573657252656769737472792063616e6e6f7420626520746865207a65726064820152686f206164647265737360b81b608482015260a4016108b2565b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b6065546001600160a01b031615801561424457506001600160a01b03821615155b6142c65760405162461bcd60e51b815260206004820152604760248201527f5061757361626c652e5f696e697469616c697a655061757365723a205f696e6960448201527f7469616c697a6550617573657228292063616e206f6e6c792062652063616c6c6064820152666564206f6e636560c81b608482015260a4016108b2565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a26143098261412c565b5050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b606060008061436d8461442b565b61ffff166001600160401b038111156143885761438861456d565b6040519080825280601f01601f1916602001820160405280156143b2576020820181803683370190505b5090506000805b8251821080156143ca575061010081105b15614421576001811b935085841615614411578060f81b8383815181106143f3576143f3615c28565b60200101906001600160f81b031916908160001a9053508160010191505b61441a81615c8c565b90506143b9565b5090949350505050565b6000805b821561445657614440600184616193565b909216918061444e816161aa565b91505061442f565b92915050565b82805461446890615d73565b90600052602060002090601f01602090048101928261448a57600085556144d0565b82601f106144a35782800160ff198235161785556144d0565b828001600101855582156144d0579182015b828111156144d05782358255916020019190600101906144b5565b506144dc9291506144e0565b5090565b5b808211156144dc57600081556001016144e1565b600060e0828403121561450757600080fd5b50919050565b60006060828403121561450757600080fd5b6000806080838503121561453257600080fd5b82356001600160401b0381111561454857600080fd5b614554858286016144f5565b925050614564846020850161450d565b90509250929050565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b03811182821017156145a5576145a561456d565b60405290565b60405161010081016001600160401b03811182821017156145a5576145a561456d565b604051601f8201601f191681016001600160401b03811182821017156145f6576145f661456d565b604052919050565b60006001600160401b038211156146175761461761456d565b5060051b60200190565b63ffffffff811681146104fa57600080fd5b803561463e81614621565b919050565b600082601f83011261465457600080fd5b81356020614669614664836145fe565b6145ce565b82815260059290921b8401810191818101908684111561468857600080fd5b8286015b848110156146ac57803561469f81614621565b835291830191830161468c565b509695505050505050565b6000604082840312156146c957600080fd5b6146d1614583565b9050813581526020820135602082015292915050565b600082601f8301126146f857600080fd5b81356020614708614664836145fe565b82815260069290921b8401810191818101908684111561472757600080fd5b8286015b848110156146ac5761473d88826146b7565b83529183019160400161472b565b600082601f83011261475c57600080fd5b614764614583565b80604084018581111561477657600080fd5b845b81811015614790578035845260209384019301614778565b509095945050505050565b6000608082840312156147ad57600080fd5b6147b5614583565b90506147c1838361474b565b81526147d0836040840161474b565b602082015292915050565b600082601f8301126147ec57600080fd5b813560206147fc614664836145fe565b82815260059290921b8401810191818101908684111561481b57600080fd5b8286015b848110156146ac5780356001600160401b0381111561483e5760008081fd5b61484c8986838b0101614643565b84525091830191830161481f565b6000610180828403121561486d57600080fd5b6148756145ab565b905081356001600160401b038082111561488e57600080fd5b61489a85838601614643565b835260208401359150808211156148b057600080fd5b6148bc858386016146e7565b602084015260408401359150808211156148d557600080fd5b6148e1858386016146e7565b60408401526148f3856060860161479b565b60608401526149058560e086016146b7565b608084015261012084013591508082111561491f57600080fd5b61492b85838601614643565b60a084015261014084013591508082111561494557600080fd5b61495185838601614643565b60c084015261016084013591508082111561496b57600080fd5b50614978848285016147db565b60e08301525092915050565b600080600083850360e081121561499a57600080fd5b84356001600160401b03808211156149b157600080fd5b9086019060c082890312156149c557600080fd5b81955060a0601f19840112156149da57600080fd5b60208701945060c08701359250808311156149f457600080fd5b5050614a028682870161485a565b9150509250925092565b6001600160a01b03811681146104fa57600080fd5b600060208284031215614a3357600080fd5b8135614a3e81614a0c565b9392505050565b600060208284031215614a5757600080fd5b5035919050565b60008060408385031215614a7157600080fd5b823560028110614a8057600080fd5b91506020830135614a9081614621565b809150509250929050565b80151581146104fa57600080fd5b600060208284031215614abb57600080fd5b8135614a3e81614a9b565b6000815180845260005b81811015614aec57602081850181015186830182015201614ad0565b81811115614afe576000602083870101525b50601f01601f19169290920160200192915050565b602081526000614a3e6020830184614ac6565b60008060008060008060008060006101208a8c031215614b4557600080fd5b8935614b5081614a0c565b985060208a0135614b6081614a0c565b975060408a0135614b7081614a0c565b965060608a0135614b8081614a0c565b955060808a0135614b9081614a9b565b945060a08a0135614ba081614a0c565b935060c08a0135614bb081614621565b925060e08a0135614bc081614621565b91506101008a0135614bd181614a0c565b809150509295985092959850929598565b60008060408385031215614bf557600080fd5b8235614c0081614a0c565b91506020838101356001600160401b03811115614c1c57600080fd5b8401601f81018613614c2d57600080fd5b8035614c3b614664826145fe565b81815260059190911b82018301908381019088831115614c5a57600080fd5b928401925b82841015614c81578335614c7281614a0c565b82529284019290840190614c5f565b80955050505050509250929050565b600081518084526020808501945080840160005b83811015614cc057815187529582019590820190600101614ca4565b509495945050505050565b602081526000614a3e6020830184614c90565b600080600060608486031215614cf357600080fd5b8335614cfe81614a0c565b92506020848101356001600160401b0380821115614d1b57600080fd5b818701915087601f830112614d2f57600080fd5b813581811115614d4157614d4161456d565b614d53601f8201601f191685016145ce565b91508082528884828501011115614d6957600080fd5b8084840185840137600084828401015250809450505050614d8c60408501614633565b90509250925092565b600081518084526020808501808196508360051b810191508286016000805b86811015614e2b578385038a52825180518087529087019087870190845b81811015614e1657835180516001600160a01b031684528a8101518b8501526040908101516001600160601b03169084015292890192606090920191600101614dd2565b50509a87019a95505091850191600101614db4565b509298975050505050505050565b602081526000614a3e6020830184614d95565b600060208284031215614e5e57600080fd5b81356001600160401b03811115614e7457600080fd5b820160408185031215614a3e57600080fd5b600082601f830112614e9757600080fd5b81356020614ea7614664836145fe565b82815260059290921b84018101918181019086841115614ec657600080fd5b8286015b848110156146ac5780358352918301918301614eca565b60008060408385031215614ef457600080fd5b8235614eff81614a0c565b915060208301356001600160401b03811115614f1a57600080fd5b614f2685828601614e86565b9150509250929050565b6020808252825182820181905260009190848201906040850190845b81811015614f715783516001600160a01b031683529284019291840191600101614f4c565b50909695505050505050565b60008083601f840112614f8f57600080fd5b5081356001600160401b03811115614fa657600080fd5b602083019150836020828501011115614fbe57600080fd5b9250929050565b60008060008060008060808789031215614fde57600080fd5b8635614fe981614a0c565b95506020870135614ff981614621565b945060408701356001600160401b038082111561501557600080fd5b6150218a838b01614f7d565b9096509450606089013591508082111561503a57600080fd5b818901915089601f83011261504e57600080fd5b81358181111561505d57600080fd5b8a60208260051b850101111561507257600080fd5b6020830194508093505050509295509295509295565b600081518084526020808501945080840160005b83811015614cc057815163ffffffff168752958201959082019060010161509c565b600081518084526020808501808196508360051b8101915082860160005b858110156151065782840389526150f4848351615088565b988501989350908401906001016150dc565b5091979650505050505050565b60208152600082516080602084015261512f60a0840182615088565b90506020840151601f198085840301604086015261514d8383615088565b9250604086015191508085840301606086015261516a8383615088565b925060608601519150808584030160808601525061518882826150be565b95945050505050565b600080600060a084860312156151a657600080fd5b83356001600160401b03808211156151bd57600080fd5b6151c9878388016144f5565b94506151d8876020880161450d565b935060808601359150808211156151ee57600080fd5b50614a028682870161485a565b60006020828403121561520d57600080fd5b81356001600160401b0381111561522357600080fd5b82016101208185031215614a3e57600080fd5b60006020828403121561524857600080fd5b813560ff81168114614a3e57600080fd5b60008060006060848603121561526e57600080fd5b833561527981614a0c565b925060208401356001600160401b0381111561529457600080fd5b6152a086828701614e86565b92505060408401356152b181614621565b809150509250925092565b6020808252825182820181905260009190848201906040850190845b81811015614f71578351835292840192918401916001016152d8565b60008060006040848603121561530957600080fd5b833561531481614621565b925060208401356001600160401b0381111561532f57600080fd5b61533b86828701614f7d565b9497909650939450505050565b60008060008060006080868803121561536057600080fd5b8535945060208601356001600160401b038082111561537e57600080fd5b61538a89838a01614f7d565b90965094506040880135915061539f82614621565b909250606087013590808211156153b557600080fd5b506153c28882890161485a565b9150509295509295909350565b600081518084526020808501945080840160005b83811015614cc05781516001600160601b0316875295820195908201906001016153e3565b604081526000835160408084015261542360808401826153cf565b90506020850151603f1984830301606085015261544082826153cf565b925050508260208301529392505050565b634e487b7160e01b600052602160045260246000fd5b602081016005831061548957634e487b7160e01b600052602160045260246000fd5b91905290565b6000806000606084860312156154a457600080fd5b83356154af81614a0c565b92506020840135915060408401356152b181614621565b8281526040602082015260006154df6040830184614d95565b949350505050565b6000602082840312156154f957600080fd5b8135614a3e81614621565b6000808335601e1984360301811261551b57600080fd5b8301803591506001600160401b0382111561553557600080fd5b602001915036819003821315614fbe57600080fd5b6000808335601e1984360301811261556157600080fd5b83016020810192503590506001600160401b0381111561558057600080fd5b803603831315614fbe57600080fd5b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b60208152600082356155c981614621565b63ffffffff811660208401525060208301356155e481614621565b63ffffffff81166040840152506155fd60408401614633565b63ffffffff8116606084015250615617606084018461554a565b60e0608085015261562d6101008501828461558f565b91505061563c60808501614633565b63ffffffff811660a08501525061565660a085018561554a565b848303601f190160c086015261566d83828461558f565b9250505061567d60c08501614633565b63ffffffff811660e08501525b509392505050565b6020808252603d908201527f737570706c696564207461736b20646f6573206e6f74206d617463682074686560408201527f206f6e65207265636f7264656420696e2074686520636f6e7472616374000000606082015260800190565b6020808252602c908201527f41676772656761746f722068617320616c726561647920726573706f6e64656460408201526b20746f20746865207461736b60a01b606082015260800190565b634e487b7160e01b600052601160045260246000fd5b600063ffffffff8083168185168083038211156157705761577061573b565b01949350505050565b6020808252602d908201527f41676772656761746f722068617320726573706f6e64656420746f207468652060408201526c7461736b20746f6f206c61746560981b606082015260800190565b80356157d181614621565b63ffffffff16825260208181013590830152604090810135910152565b63ffffffff815116825260208101516020830152600060408201516080604085015261581d60808501826153cf565b90506060830151848203606086015261518882826153cf565b61584081846157c6565b6080606082015260006154df60808301846157ee565b6060810161445682846157c6565b602080825260059082015264041757468360dc1b604082015260600190565b602081526000823561589481614621565b63ffffffff808216602085015260208501356040850152604085013591506158bb82614621565b8082166060850152606085013591506158d382614621565b80821660808501526158e8608086018661554a565b925060c060a08601526158ff60e08601848361558f565b92505060a085013561591081614621565b1660c0939093019290925250919050565b803561592c81614621565b63ffffffff168252602081810135908301526040808201359083015260608082013590830152608090810135910152565b60a081016144568284615921565b600081518084526020808501945080840160005b83811015614cc05761599c87835180518252602090810151910152565b604096909601959082019060010161597f565b8060005b60028110156159d25781518452602093840193909101906001016159b3565b50505050565b6159e38282516159af565b602081015161121160408401826159af565b858152608060208201526000615a0f60808301868861558f565b63ffffffff8516604084015282810360608401526101808451818352615a3782840182615088565b91505060208501518282036020840152615a51828261596b565b91505060408501518282036040840152615a6b828261596b565b9150506060850151615a8060608401826159d8565b506080850151805160e08401526020015161010083015260a0850151828203610120840152615aaf8282615088565b91505060c0850151828203610140840152615aca8282615088565b91505060e0850151828203610160840152615ae582826150be565b9a9950505050505050505050565b80516001600160601b038116811461463e57600080fd5b600082601f830112615b1b57600080fd5b81516020615b2b614664836145fe565b82815260059290921b84018101918181019086841115615b4a57600080fd5b8286015b848110156146ac57615b5f81615af3565b8352918301918301615b4e565b60008060408385031215615b7f57600080fd5b82516001600160401b0380821115615b9657600080fd5b9084019060408287031215615baa57600080fd5b615bb2614583565b825182811115615bc157600080fd5b615bcd88828601615b0a565b825250602083015182811115615be257600080fd5b615bee88828601615b0a565b602083015250809450505050602083015190509250929050565b615c128184615921565b60c060a082015260006154df60c08301846157ee565b634e487b7160e01b600052603260045260246000fd5b60006001600160601b0380831681851681830481118215151615615c6457615c6461573b565b02949350505050565b6000816000190483118215151615615c8757615c8761573b565b500290565b6000600019821415615ca057615ca061573b565b5060010190565b600060208284031215615cb957600080fd5b8151614a3e81614a0c565b6020808252602a908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526939903ab73830bab9b2b960b11b606082015260800190565b600060208284031215615d2057600080fd5b8151614a3e81614a9b565b60208082526028908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526739903830bab9b2b960c11b606082015260800190565b600181811c90821680615d8757607f821691505b6020821081141561450757634e487b7160e01b600052602260045260246000fd5b600060208284031215615dba57600080fd5b5051919050565b60006020808385031215615dd457600080fd5b82516001600160401b03811115615dea57600080fd5b8301601f81018513615dfb57600080fd5b8051615e09614664826145fe565b81815260059190911b82018301908381019087831115615e2857600080fd5b928401925b82841015615e4657835182529284019290840190615e2d565b979650505050505050565b600060208284031215615e6357600080fd5b614a3e82615af3565b63ffffffff84168152604060208201819052810182905260006001600160fb1b03831115615e9957600080fd5b8260051b8085606085013760009201606001918252509392505050565b60006020808385031215615ec957600080fd5b82516001600160401b03811115615edf57600080fd5b8301601f81018513615ef057600080fd5b8051615efe614664826145fe565b81815260059190911b82018301908381019087831115615f1d57600080fd5b928401925b82841015615e46578351615f3581614621565b82529284019290840190615f22565b63ffffffff8416815260406020820152600061518860408301848661558f565b600060208284031215615f7657600080fd5b81516001600160c01b0381168114614a3e57600080fd5b600060208284031215615f9f57600080fd5b8151614a3e81614621565b600060ff821660ff811415615fc157615fc161573b565b60010192915050565b604081526000615fde60408301858761558f565b905063ffffffff83166020830152949350505050565b63ffffffff831681526040602082015260006154df6040830184614c90565b60208082526039908201527f43616e2774206372656174652061207461736b20696e207468652073616d652060408201527f626c6f636b206173206120636f6d706c65746564207461736b00000000000000606082015260800190565b600063ffffffff8381169083168181101561608d5761608d61573b565b039392505050565b60208152600063ffffffff80845116602084015280602085015116604084015280604085015116606084015250606083015160e060808401526160dc610100840182614ac6565b905060808401516160f560a085018263ffffffff169052565b5060a0840151838203601f190160c08501526161118282614ac6565b91505060c084015161568a60e085018263ffffffff169052565b60208152600063ffffffff80845116602084015260208401516040840152806040850151166060840152806060850151166080840152608084015160c060a085015261617a60e0850182614ac6565b90508160a08601511660c0850152809250505092915050565b6000828210156161a5576161a561573b565b500390565b600061ffff808316818114156161c2576161c261573b565b600101939250505056fe759fbb3c7535ea6cd791a268d7e54d5bf43408007315741bf51b70650b77f6a5be6620bd3346e5d7f8387fbec0981aa0d6289d22efa7c935f9ef6841bf2a98c721d5695aeb71770b4b420e85352fe1a012fa06ae92de02f7ee513765e0afa023a26469706673582212207330bbc6f2a890fc8febb2d74960d9f3cf7cb6c6f5a9d5cee52fbb01ac65fcc564736f6c634300080c0033",
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

// Initialize is a paid mutator transaction binding the contract method 0x2d4713db.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _aggregator, address _generator, bool _allowNonRootInit, address _blsSignatureCheckerAddress, uint32 _taskResponseWindowBlock, uint32 _minOpTaskResponseWindowBlock, address _operatorStateRetrieverExtended) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) Initialize(opts *bind.TransactOpts, _pauserRegistry common.Address, initialOwner common.Address, _aggregator common.Address, _generator common.Address, _allowNonRootInit bool, _blsSignatureCheckerAddress common.Address, _taskResponseWindowBlock uint32, _minOpTaskResponseWindowBlock uint32, _operatorStateRetrieverExtended common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "initialize", _pauserRegistry, initialOwner, _aggregator, _generator, _allowNonRootInit, _blsSignatureCheckerAddress, _taskResponseWindowBlock, _minOpTaskResponseWindowBlock, _operatorStateRetrieverExtended)
}

// Initialize is a paid mutator transaction binding the contract method 0x2d4713db.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _aggregator, address _generator, bool _allowNonRootInit, address _blsSignatureCheckerAddress, uint32 _taskResponseWindowBlock, uint32 _minOpTaskResponseWindowBlock, address _operatorStateRetrieverExtended) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) Initialize(_pauserRegistry common.Address, initialOwner common.Address, _aggregator common.Address, _generator common.Address, _allowNonRootInit bool, _blsSignatureCheckerAddress common.Address, _taskResponseWindowBlock uint32, _minOpTaskResponseWindowBlock uint32, _operatorStateRetrieverExtended common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.Initialize(&_ContractFinalizerTaskManager.TransactOpts, _pauserRegistry, initialOwner, _aggregator, _generator, _allowNonRootInit, _blsSignatureCheckerAddress, _taskResponseWindowBlock, _minOpTaskResponseWindowBlock, _operatorStateRetrieverExtended)
}

// Initialize is a paid mutator transaction binding the contract method 0x2d4713db.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _aggregator, address _generator, bool _allowNonRootInit, address _blsSignatureCheckerAddress, uint32 _taskResponseWindowBlock, uint32 _minOpTaskResponseWindowBlock, address _operatorStateRetrieverExtended) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) Initialize(_pauserRegistry common.Address, initialOwner common.Address, _aggregator common.Address, _generator common.Address, _allowNonRootInit bool, _blsSignatureCheckerAddress common.Address, _taskResponseWindowBlock uint32, _minOpTaskResponseWindowBlock uint32, _operatorStateRetrieverExtended common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.Initialize(&_ContractFinalizerTaskManager.TransactOpts, _pauserRegistry, initialOwner, _aggregator, _generator, _allowNonRootInit, _blsSignatureCheckerAddress, _taskResponseWindowBlock, _minOpTaskResponseWindowBlock, _operatorStateRetrieverExtended)
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

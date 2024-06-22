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
	NonSignerQuorumBitmapIndices                            []uint32
	NonSignerPubkeys                                        []BN254G1Point
	QuorumApks                                              []BN254G1Point
	ApkG2                                                   BN254G2Point
	Sigma                                                   BN254G1Point
	QuorumApkIndices                                        []uint32
	TotalStakeIndices                                       []uint32
	NonSignerStakeIndices                                   [][]uint32
	NonSignerPubkeysIndicesforOperatorIdsRemovedForOldState []uint32
	NonSignerPubkeysAddedForOldState                        []BN254G1Point
	ApkG2forOldState                                        BN254G2Point
	SigmaforOldSate                                         BN254G1Point
}

// IBLSSignatureCheckerQuorumStakeTotals is an auto generated low-level Go binding around an user-defined struct.
type IBLSSignatureCheckerQuorumStakeTotals struct {
	SignedStakeForQuorum []*big.Int
	TotalStakeForQuorum  []*big.Int
}

// IFinalizerTaskManagerOperatorStateInfo is an auto generated low-level Go binding around an user-defined struct.
type IFinalizerTaskManagerOperatorStateInfo struct {
	OperatorsStateChanged      bool
	OperatorsStateProvided     bool
	QuorumsRemoved             []uint8
	QuorumsAdded               []IGaspMultiRollupServicePrimitivesQuorumsAdded
	QuorumsStakeUpdate         []IGaspMultiRollupServicePrimitivesQuorumsStakeUpdate
	QuorumsApkUpdate           []IGaspMultiRollupServicePrimitivesQuorumsApkUpdate
	OperatorsRemoved           [][32]byte
	OperatorsAdded             []IGaspMultiRollupServicePrimitivesOperatorsAdded
	OperatorsStakeUpdate       []IGaspMultiRollupServicePrimitivesOperatorsStakeUpdate
	OperatorsQuorumCountUpdate []IGaspMultiRollupServicePrimitivesOperatorsQuorumCountUpdate
}

// IFinalizerTaskManagerTask is an auto generated low-level Go binding around an user-defined struct.
type IFinalizerTaskManagerTask struct {
	BlockNumber                                *big.Int
	TaskCreatedBlock                           uint32
	LastCompletedTaskCreatedBlock              uint32
	QuorumNumbers                              []byte
	QuorumThresholdPercentage                  uint32
	LastCompletedTaskQuorumNumbers             []byte
	LastCompletedTaskQuorumThresholdPercentage uint32
}

// IFinalizerTaskManagerTaskResponse is an auto generated low-level Go binding around an user-defined struct.
type IFinalizerTaskManagerTaskResponse struct {
	ReferenceTaskIndex     uint32
	ReferenceTaskHash      [32]byte
	OperatorsStateInfoHash [32]byte
	BlockHash              [32]byte
	StorageProofHash       [32]byte
	PendingStateHash       [32]byte
}

// IFinalizerTaskManagerTaskResponseMetadata is an auto generated low-level Go binding around an user-defined struct.
type IFinalizerTaskManagerTaskResponseMetadata struct {
	TaskResponsedBlock uint32
	HashOfNonSigners   [32]byte
	QuroumStakeTotals  []*big.Int
	QuroumStakeSigned  []*big.Int
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
	ABI: "[{\"type\":\"constructor\",\"inputs\":[{\"name\":\"_registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"_taskResponseWindowBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"aggregator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allTaskHashes\",\"inputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allTaskResponses\",\"inputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"blsApkRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIBLSApkRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"checkSignatures\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"referenceBlockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"params\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"},{\"name\":\"nonSignerPubkeysIndicesforOperatorIdsRemovedForOldState\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeysAddedForOldState\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2forOldState\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigmaforOldSate\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.QuorumStakeTotals\",\"components\":[{\"name\":\"signedStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"totalStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"createNewTask\",\"inputs\":[{\"name\":\"blockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"delegation\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIDelegationManager\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"dummyForOperatorStateInfoType\",\"inputs\":[{\"name\":\"_operatorStateInfo\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OperatorStateInfo\",\"components\":[{\"name\":\"operatorsStateChanged\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"operatorsStateProvided\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"quorumsRemoved\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsAdded[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"quorumsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsStakeUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]},{\"name\":\"quorumsApkUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsApkUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"OperatorsRemoved\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"operatorsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsAdded[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]},{\"name\":\"operatorsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsStakeUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"operatorsQuorumCountUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsQuorumCountUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]}]}],\"outputs\":[],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"generator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getCheckSignaturesIndices\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"referenceBlockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"nonSignerOperatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structOperatorStateRetriever.CheckSignaturesIndices\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getLatestPendingStateHash\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getOperatorState\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple[][]\",\"internalType\":\"structOperatorStateRetriever.Operator[][]\",\"components\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"stake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getOperatorState\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"\",\"type\":\"tuple[][]\",\"internalType\":\"structOperatorStateRetriever.Operator[][]\",\"components\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"stake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getQuorumBitmapsAtBlockNumber\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256[]\",\"internalType\":\"uint256[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getTaskResponseWindowBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"indexToTaskStatus\",\"inputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumFinalizerTaskManager.TaskStatus\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"_pauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"initialOwner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_aggregator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_generator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"latestTaskNum\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"pauseAll\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[{\"name\":\"index\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pauserRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"registryCoordinator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"respondToTask\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.Task\",\"components\":[{\"name\":\"blockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.TaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"storageProofHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"pendingStateHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"},{\"name\":\"nonSignerPubkeysIndicesforOperatorIdsRemovedForOldState\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeysAddedForOldState\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2forOldState\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigmaforOldSate\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setPauserRegistry\",\"inputs\":[{\"name\":\"newPauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setStaleStakesForbidden\",\"inputs\":[{\"name\":\"value\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"stakeRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIStakeRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"staleStakesForbidden\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"taskNumber\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"trySignatureAndApkVerification\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"apk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}],\"outputs\":[{\"name\":\"pairingSuccessful\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"siganatureIsValid\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"unpause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"Initialized\",\"inputs\":[{\"name\":\"version\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewTaskCreated\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"task\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.Task\",\"components\":[{\"name\":\"blockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Paused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"PauserRegistrySet\",\"inputs\":[{\"name\":\"pauserRegistry\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"newPauserRegistry\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractIPauserRegistry\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"StaleStakesForbiddenUpdate\",\"inputs\":[{\"name\":\"value\",\"type\":\"bool\",\"indexed\":false,\"internalType\":\"bool\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"TaskCompleted\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"blockHash\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"TaskResponded\",\"inputs\":[{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.TaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"storageProofHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"pendingStateHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"taskResponseMetadata\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.TaskResponseMetadata\",\"components\":[{\"name\":\"taskResponsedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"hashOfNonSigners\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quroumStakeTotals\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quroumStakeSigned\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Unpaused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false}]",
	Bin: "0x6101206040523480156200001257600080fd5b5060405162005d8b38038062005d8b8339810160408190526200003591620001f7565b81806001600160a01b03166080816001600160a01b031681525050806001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa1580156200008f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000b591906200023e565b6001600160a01b031660a0816001600160a01b031681525050806001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa1580156200010d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200013391906200023e565b6001600160a01b031660c0816001600160a01b03168152505060a0516001600160a01b031663df5cf7236040518163ffffffff1660e01b8152600401602060405180830381865afa1580156200018d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001b391906200023e565b6001600160a01b031660e052506097805460ff1916600117905563ffffffff16610100525062000265565b6001600160a01b0381168114620001f457600080fd5b50565b600080604083850312156200020b57600080fd5b82516200021881620001de565b602084015190925063ffffffff811681146200023357600080fd5b809150509250929050565b6000602082840312156200025157600080fd5b81516200025e81620001de565b9392505050565b60805160a05160c05160e05161010051615aa2620002e96000396000818161057d0152611e4801526000818161054601526129fb0152600081816103ad0152612bde0152600081816103d401528181612db40152612f7601526000818161040e01528181610de1015281816126c60152818161285e0152612a980152615aa26000f3fe608060405234801561001057600080fd5b50600436106102115760003560e01c80636d14a98711610125578063c3af8313116100ad578063df5cf7231161007c578063df5cf72314610541578063f2fde38b14610568578063f5c9899d1461057b578063f8c8765e146105a1578063fabc1cbc146105b457600080fd5b8063c3af8313146104db578063cab0e6fd146104ec578063cefdc1d4146104ff578063db67f7771461052057600080fd5b8063886f1195116100f4578063886f11951461046a5780638b00ce7c1461047d5780638da5cb5b1461048d57806399dba0c41461049e578063b98d0908146104ce57600080fd5b80636d14a98714610409578063715018a61461043057806372d18e8d146104385780637afa1eed1461045757600080fd5b8063416c7e5e116101a85780635c155662116101775780635c155662146103805780635c975abb146103a05780635df45946146103a857806368304835146103cf5780636b92787e146103f657600080fd5b8063416c7e5e146103125780634f739f7414610325578063595c6a67146103455780635ac86ab71461034d57600080fd5b8063245a7bfc116101e4578063245a7bfc1461027f5780632cb223d5146102b25780632d89f6fc146102d25780633563b0d1146102f257600080fd5b80630373408d1461021657806310d67a2f1461022d578063136439dd14610242578063171f1d5b14610255575b600080fd5b60d1545b6040519081526020015b60405180910390f35b61024061023b3660046143d7565b6105c7565b005b6102406102503660046143f4565b610680565b610268610263366004614572565b6107bf565b604080519215158352901515602083015201610224565b60cf5461029a9064010000000090046001600160a01b031681565b6040516001600160a01b039091168152602001610224565b61021a6102c03660046145e0565b60cb6020526000908152604090205481565b61021a6102e03660046145e0565b60ca6020526000908152604090205481565b6103056103003660046145fd565b610949565b6040516102249190614758565b610240610320366004614780565b610ddf565b6103386103333660046147e5565b610f54565b60405161022491906148e9565b61024061167a565b61037061035b3660046149b3565b606654600160ff9092169190911b9081161490565b6040519015158152602001610224565b61039361038e3660046149f3565b611741565b6040516102249190614aa4565b60665461021a565b61029a7f000000000000000000000000000000000000000000000000000000000000000081565b61029a7f000000000000000000000000000000000000000000000000000000000000000081565b610240610404366004614ae8565b611909565b61029a7f000000000000000000000000000000000000000000000000000000000000000081565b610240611c3d565b60c95463ffffffff165b60405163ffffffff9091168152602001610224565b60d05461029a906001600160a01b031681565b60655461029a906001600160a01b031681565b60c9546104429063ffffffff1681565b6033546001600160a01b031661029a565b6104c16104ac3660046145e0565b60cc6020526000908152604090205460ff1681565b6040516102249190614b59565b6097546103709060ff1681565b6102406104e9366004614b81565b50565b6102406104fa366004614eaa565b611c51565b61051261050d366004614f33565b612181565b604051610224929190614f75565b61053361052e366004614f96565b612313565b604051610224929190615056565b61029a7f000000000000000000000000000000000000000000000000000000000000000081565b6102406105763660046143d7565b61322b565b7f0000000000000000000000000000000000000000000000000000000000000000610442565b6102406105af36600461509f565b6132a1565b6102406105c23660046143f4565b613407565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561061a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061063e91906150fb565b6001600160a01b0316336001600160a01b0316146106775760405162461bcd60e51b815260040161066e90615118565b60405180910390fd5b6104e981613563565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa1580156106c8573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106ec9190615162565b6107085760405162461bcd60e51b815260040161066e9061517f565b606654818116146107815760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e70617573653a20696e76616c696420617474656d70742060448201527f746f20756e70617573652066756e6374696f6e616c6974790000000000000000606482015260840161066e565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b60008060007f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f000000187876000015188602001518860000151600060028110610807576108076151c7565b60200201518951600160200201518a6020015160006002811061082c5761082c6151c7565b60200201518b60200151600160028110610848576108486151c7565b602090810291909101518c518d8301516040516108a59a99989796959401988952602089019790975260408801959095526060870193909352608086019190915260a085015260c084015260e08301526101008201526101200190565b6040516020818303038152906040528051906020012060001c6108c891906151dd565b905061093b6108e16108da888461365a565b86906136f1565b6108e9613785565b6109316109228561091c604080518082018252600080825260209182015281518083019092526001825260029082015290565b9061365a565b61092b8c613845565b906136f1565b886201d4c06138d5565b909890975095505050505050565b60606000846001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa15801561098b573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109af91906150fb565b90506000856001600160a01b0316639e9923c26040518163ffffffff1660e01b8152600401602060405180830381865afa1580156109f1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a1591906150fb565b90506000866001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa158015610a57573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a7b91906150fb565b9050600086516001600160401b03811115610a9857610a9861440d565b604051908082528060200260200182016040528015610acb57816020015b6060815260200190600190039081610ab65790505b50905060005b8751811015610dd3576000888281518110610aee57610aee6151c7565b0160200151604051638902624560e01b815260f89190911c6004820181905263ffffffff8a16602483015291506000906001600160a01b03871690638902624590604401600060405180830381865afa158015610b4f573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610b7791908101906151ff565b905080516001600160401b03811115610b9257610b9261440d565b604051908082528060200260200182016040528015610bdd57816020015b6040805160608101825260008082526020808301829052928201528252600019909201910181610bb05790505b50848481518110610bf057610bf06151c7565b602002602001018190525060005b8151811015610dbd576040518060600160405280876001600160a01b03166347b314e8858581518110610c3357610c336151c7565b60200260200101516040518263ffffffff1660e01b8152600401610c5991815260200190565b602060405180830381865afa158015610c76573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c9a91906150fb565b6001600160a01b03168152602001838381518110610cba57610cba6151c7565b60200260200101518152602001896001600160a01b031663fa28c627858581518110610ce857610ce86151c7565b60209081029190910101516040516001600160e01b031960e084901b168152600481019190915260ff8816602482015263ffffffff8f166044820152606401602060405180830381865afa158015610d44573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d68919061528f565b6001600160601b0316815250858581518110610d8657610d866151c7565b60200260200101518281518110610d9f57610d9f6151c7565b60200260200101819052508080610db5906152ce565b915050610bfe565b5050508080610dcb906152ce565b915050610ad1565b50979650505050505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610e3d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610e6191906150fb565b6001600160a01b0316336001600160a01b031614610f0d5760405162461bcd60e51b815260206004820152605c60248201527f424c535369676e6174757265436865636b65722e6f6e6c79436f6f7264696e6160448201527f746f724f776e65723a2063616c6c6572206973206e6f7420746865206f776e6560648201527f72206f6620746865207265676973747279436f6f7264696e61746f7200000000608482015260a40161066e565b6097805460ff19168215159081179091556040519081527f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc9060200160405180910390a150565b610f7f6040518060800160405280606081526020016060815260200160608152602001606081525090565b6000876001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa158015610fbf573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610fe391906150fb565b90506110106040518060800160405280606081526020016060815260200160608152602001606081525090565b6040516361c8a12f60e11b81526001600160a01b038a169063c391425e90611040908b90899089906004016152e9565b600060405180830381865afa15801561105d573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526110859190810190615333565b81526040516340e03a8160e11b81526001600160a01b038316906381c07502906110b7908b908b908b906004016153ea565b600060405180830381865afa1580156110d4573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526110fc9190810190615333565b6040820152856001600160401b038111156111195761111961440d565b60405190808252806020026020018201604052801561114c57816020015b60608152602001906001900390816111375790505b50606082015260005b60ff811687111561158b576000856001600160401b0381111561117a5761117a61440d565b6040519080825280602002602001820160405280156111a3578160200160208202803683370190505b5083606001518360ff16815181106111bd576111bd6151c7565b602002602001018190525060005b8681101561148b5760008c6001600160a01b03166304ec63518a8a858181106111f6576111f66151c7565b905060200201358e88600001518681518110611214576112146151c7565b60200260200101516040518463ffffffff1660e01b81526004016112519392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa15801561126e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906112929190615413565b90506001600160c01b0381166113365760405162461bcd60e51b815260206004820152605c60248201527f4f70657261746f7253746174655265747269657665722e676574436865636b5360448201527f69676e617475726573496e64696365733a206f70657261746f72206d7573742060648201527f6265207265676973746572656420617420626c6f636b6e756d62657200000000608482015260a40161066e565b8a8a8560ff1681811061134b5761134b6151c7565b6001600160c01b03841692013560f81c9190911c60019081161415905061147857856001600160a01b031663dd9846b98a8a8581811061138d5761138d6151c7565b905060200201358d8d8860ff168181106113a9576113a96151c7565b6040516001600160e01b031960e087901b1681526004810194909452919091013560f81c60248301525063ffffffff8f166044820152606401602060405180830381865afa1580156113ff573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611423919061543c565b85606001518560ff168151811061143c5761143c6151c7565b60200260200101518481518110611455576114556151c7565b63ffffffff9092166020928302919091019091015282611474816152ce565b9350505b5080611483816152ce565b9150506111cb565b506000816001600160401b038111156114a6576114a661440d565b6040519080825280602002602001820160405280156114cf578160200160208202803683370190505b50905060005b828110156115505784606001518460ff16815181106114f6576114f66151c7565b6020026020010151818151811061150f5761150f6151c7565b6020026020010151828281518110611529576115296151c7565b63ffffffff9092166020928302919091019091015280611548816152ce565b9150506114d5565b508084606001518460ff168151811061156b5761156b6151c7565b60200260200101819052505050808061158390615459565b915050611155565b506000896001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa1580156115cc573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115f091906150fb565b60405163354952a360e21b81529091506001600160a01b0382169063d5254a8c90611623908b908b908e90600401615479565b600060405180830381865afa158015611640573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526116689190810190615333565b60208301525098975050505050505050565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa1580156116c2573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906116e69190615162565b6117025760405162461bcd60e51b815260040161066e9061517f565b600019606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b60606000846001600160a01b031663c391425e84866040518363ffffffff1660e01b81526004016117739291906154a3565b600060405180830381865afa158015611790573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526117b89190810190615333565b9050600084516001600160401b038111156117d5576117d561440d565b6040519080825280602002602001820160405280156117fe578160200160208202803683370190505b50905060005b85518110156118ff57866001600160a01b03166304ec635187838151811061182e5761182e6151c7565b602002602001015187868581518110611849576118496151c7565b60200260200101516040518463ffffffff1660e01b81526004016118869392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa1580156118a3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906118c79190615413565b6001600160c01b03168282815181106118e2576118e26151c7565b6020908102919091010152806118f7816152ce565b915050611804565b5095945050505050565b60d0546001600160a01b0316331461194b5760405162461bcd60e51b8152602060048201526005602482015264417574683160d81b604482015260640161066e565b60cd5463ffffffff16431480159061196257504315155b6119d45760405162461bcd60e51b815260206004820152603960248201527f43616e2774206372656174652061207461736b20696e207468652073616d652060448201527f626c6f636b206173206120636f6d706c65746564207461736b00000000000000606482015260840161066e565b6040805160e0810182526000818301819052606080830181905260a083015260c082015285815263ffffffff43811660208084019190915290861660808301528251601f85018290048202810182019093528383529091908490849081908401838280828437600092019190915250505050606082015260cd5463ffffffff16604082015260ce8054611a66906154f7565b80601f0160208091040260200160405190810160405280929190818152602001828054611a92906154f7565b8015611adf5780601f10611ab457610100808354040283529160200191611adf565b820191906000526020600020905b815481529060010190602001808311611ac257829003601f168201915b505050505060a082015260cf5463ffffffff90811660c083015260c9541615611b715760c954600090611b1a9060019063ffffffff16615532565b9050600163ffffffff8216600090815260cc602052604090205460ff166003811115611b4857611b48614b43565b1415611b6f5763ffffffff8116600090815260cc60205260409020805460ff191660021790555b505b80604051602001611b8291906155a4565b60408051808303601f19018152828252805160209182012060c9805463ffffffff908116600090815260ca85528581209390935581548116835260cc909352929020805460ff19166001179055905416907f933fbdf390ba7173f9bf83f5459edf1ae41ae5edbde6c8f9b374d979d30900be90611c009084906155a4565b60405180910390a260c954611c1c9063ffffffff166001615637565b60c9805463ffffffff191663ffffffff929092169190911790555050505050565b611c45613af9565b611c4f6000613b53565b565b60cf5464010000000090046001600160a01b03163314611c9b5760405162461bcd60e51b8152602060048201526005602482015264041757468360dc1b604482015260640161066e565b6000611cad60408501602086016145e0565b9050366000611cbf606087018761565f565b90925090506000611cd660a08801608089016145e0565b905060ca6000611ce960208901896145e0565b63ffffffff1663ffffffff1681526020019081526020016000205487604051602001611d1591906156ea565b6040516020818303038152906040528051906020012014611d9e5760405162461bcd60e51b815260206004820152603d60248201527f737570706c696564207461736b20646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e7472616374000000606482015260840161066e565b600160cc6000611db160208a018a6145e0565b63ffffffff16815260208101919091526040016000205460ff166003811115611ddc57611ddc614b43565b14611df95760405162461bcd60e51b815260040161066e906157b1565b600060cb81611e0b60208a018a6145e0565b63ffffffff1663ffffffff1681526020019081526020016000205414611e435760405162461bcd60e51b815260040161066e906157b1565b611e6d7f000000000000000000000000000000000000000000000000000000000000000085615637565b63ffffffff164363ffffffff161115611ede5760405162461bcd60e51b815260206004820152602d60248201527f41676772656761746f722068617320726573706f6e64656420746f207468652060448201526c7461736b20746f6f206c61746560981b606482015260840161066e565b600086604051602001611ef19190615848565b604051602081830303815290604052805190602001209050600080611f198387878a8c612313565b6040805160808101825263ffffffff43168152602080820184905280850151828401528451606083015291519395509193509091611f5b918c91849101615856565b6040516020818303038152906040528051906020012060cb60008c6000016020810190611f8891906145e0565b63ffffffff1663ffffffff168152602001908152602001600020819055507ff986623b08031c29efcbf1098118412d9c71ef7d32f63c701988d6bba251ce668a82604051611fd7929190615856565b60405180910390a160005b86811015612078578560ff1684602001518281518110612004576120046151c7565b602002602001015161201691906158c4565b6001600160601b0316606485600001518381518110612037576120376151c7565b60200260200101516001600160601b031661205291906158f3565b101561206657505050505050505050505050565b80612070816152ce565b915050611fe2565b5060a08a013560d155600360cc600061209460208e018e6145e0565b63ffffffff16815260208101919091526040016000205460ff1660038111156120bf576120bf614b43565b506120d2905060408c0160208d016145e0565b60cd805463ffffffff191663ffffffff929092169190911790556120f960608c018c61565f565b6121059160ce9161424f565b5061211660a08c0160808d016145e0565b60cf805463ffffffff191663ffffffff9290921691909117905560608a013561214260208c018c6145e0565b63ffffffff167f8378be8a33cf3a493910a16e275cd96af4f048c5eb1a2c2962d4066e697fea8060405160405180910390a35050505050505050505050565b60408051600180825281830190925260009160609183916020808301908036833701905050905084816000815181106121bc576121bc6151c7565b60209081029190910101526040516361c8a12f60e11b81526000906001600160a01b0388169063c391425e906121f890889086906004016154a3565b600060405180830381865afa158015612215573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261223d9190810190615333565b60008151811061224f5761224f6151c7565b60209081029190910101516040516304ec635160e01b81526004810188905263ffffffff87811660248301529091166044820181905291506000906001600160a01b038916906304ec635190606401602060405180830381865afa1580156122bb573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906122df9190615413565b6001600160c01b0316905060006122f582613ba5565b9050816123038a838a610949565b9550955050505050935093915050565b604080518082019091526060808252602082015260008461238a5760405162461bcd60e51b81526020600482015260376024820152600080516020615a4d83398151915260448201527f7265733a20656d7074792071756f72756d20696e707574000000000000000000606482015260840161066e565b604083015151851480156123a2575060a08301515185145b80156123b2575060c08301515185145b80156123c2575060e08301515185145b61242c5760405162461bcd60e51b81526020600482015260416024820152600080516020615a4d83398151915260448201527f7265733a20696e7075742071756f72756d206c656e677468206d69736d6174636064820152600d60fb1b608482015260a40161066e565b825151602084015151146124a45760405162461bcd60e51b815260206004820152604460248201819052600080516020615a4d833981519152908201527f7265733a20696e707574206e6f6e7369676e6572206c656e677468206d69736d6064820152630c2e8c6d60e31b608482015260a40161066e565b4363ffffffff168463ffffffff16106125135760405162461bcd60e51b815260206004820152603c6024820152600080516020615a4d83398151915260448201527f7265733a20696e76616c6964207265666572656e636520626c6f636b00000000606482015260840161066e565b6040805180820182526000808252602080830191909152825180840190935260608084529083015290866001600160401b038111156125545761255461440d565b60405190808252806020026020018201604052801561257d578160200160208202803683370190505b506020820152866001600160401b0381111561259b5761259b61440d565b6040519080825280602002602001820160405280156125c4578160200160208202803683370190505b50815260408051808201909152606080825260208201528560200151516001600160401b038111156125f8576125f861440d565b604051908082528060200260200182016040528015612621578160200160208202803683370190505b5081526020860151516001600160401b038111156126415761264161440d565b60405190808252806020026020018201604052801561266a578160200160208202803683370190505b508160200181905250600061273c8a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152505060408051639aa1653d60e01b815290516001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169350639aa1653d925060048083019260209291908290030181865afa158015612713573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906127379190615912565b613c71565b905060005b8760200151518110156129d75761278688602001518281518110612767576127676151c7565b6020026020010151805160009081526020918201519091526040902090565b8360200151828151811061279c5761279c6151c7565b6020908102919091010152801561285c5760208301516127bd60018361592f565b815181106127cd576127cd6151c7565b602002602001015160001c836020015182815181106127ee576127ee6151c7565b602002602001015160001c1161285c576040805162461bcd60e51b8152602060048201526024810191909152600080516020615a4d83398151915260448201527f7265733a206e6f6e5369676e65725075626b657973206e6f7420736f72746564606482015260840161066e565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166304ec6351846020015183815181106128a1576128a16151c7565b60200260200101518b8b6000015185815181106128c0576128c06151c7565b60200260200101516040518463ffffffff1660e01b81526004016128fd9392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa15801561291a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061293e9190615413565b6001600160c01b03168360000151828151811061295d5761295d6151c7565b6020026020010181815250506129c36108da6129978486600001518581518110612989576129896151c7565b602002602001015116613d04565b8a6020015184815181106129ad576129ad6151c7565b6020026020010151613d2f90919063ffffffff16565b9450806129cf816152ce565b915050612741565b50506129e283613e13565b60975490935060ff166000816129f9576000612a7b565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663c448feb86040518163ffffffff1660e01b8152600401602060405180830381865afa158015612a57573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612a7b9190615946565b905060005b8a8110156130fa578215612bdc578963ffffffff16827f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663249a0c428f8f86818110612ad757612ad76151c7565b60405160e085901b6001600160e01b031916815292013560f81c600483015250602401602060405180830381865afa158015612b17573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612b3b9190615946565b612b45919061595f565b1015612bdc5760405162461bcd60e51b81526020600482015260666024820152600080516020615a4d83398151915260448201527f7265733a205374616b6552656769737472792075706461746573206d7573742060648201527f62652077697468696e207769746864726177616c44656c6179426c6f636b732060848201526577696e646f7760d01b60a482015260c40161066e565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166368bccaac8d8d84818110612c1d57612c1d6151c7565b9050013560f81c60f81b60f81c8c8c60a001518581518110612c4157612c416151c7565b60209081029190910101516040516001600160e01b031960e086901b16815260ff909316600484015263ffffffff9182166024840152166044820152606401602060405180830381865afa158015612c9d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612cc19190615977565b6001600160401b031916612ce48a604001518381518110612767576127676151c7565b67ffffffffffffffff191614612d805760405162461bcd60e51b81526020600482015260616024820152600080516020615a4d83398151915260448201527f7265733a2071756f72756d41706b206861736820696e2073746f72616765206460648201527f6f6573206e6f74206d617463682070726f76696465642071756f72756d2061706084820152606b60f81b60a482015260c40161066e565b612db089604001518281518110612d9957612d996151c7565b6020026020010151876136f190919063ffffffff16565b95507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663c8294c568d8d84818110612df357612df36151c7565b9050013560f81c60f81b60f81c8c8c60c001518581518110612e1757612e176151c7565b60209081029190910101516040516001600160e01b031960e086901b16815260ff909316600484015263ffffffff9182166024840152166044820152606401602060405180830381865afa158015612e73573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612e97919061528f565b85602001518281518110612ead57612ead6151c7565b6001600160601b03909216602092830291909101820152850151805182908110612ed957612ed96151c7565b602002602001015185600001518281518110612ef757612ef76151c7565b60200260200101906001600160601b031690816001600160601b0316815250506000805b8a60200151518110156130e557612f6f86600001518281518110612f4157612f416151c7565b60200260200101518f8f86818110612f5b57612f5b6151c7565b600192013560f81c9290921c811614919050565b156130d3577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f2be94ae8f8f86818110612fb557612fb56151c7565b9050013560f81c60f81b60f81c8e89602001518581518110612fd957612fd96151c7565b60200260200101518f60e001518881518110612ff757612ff76151c7565b60200260200101518781518110613010576130106151c7565b60209081029190910101516040516001600160e01b031960e087901b16815260ff909416600485015263ffffffff92831660248501526044840191909152166064820152608401602060405180830381865afa158015613074573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613098919061528f565b87518051859081106130ac576130ac6151c7565b602002602001018181516130c091906159a2565b6001600160601b03169052506001909101905b806130dd816152ce565b915050612f1b565b505080806130f2906152ce565b915050612a80565b5050506000806131148c868a606001518b608001516107bf565b91509150816131855760405162461bcd60e51b81526020600482015260436024820152600080516020615a4d83398151915260448201527f7265733a2070616972696e6720707265636f6d70696c652063616c6c206661696064820152621b195960ea1b608482015260a40161066e565b806131e65760405162461bcd60e51b81526020600482015260396024820152600080516020615a4d83398151915260448201527f7265733a207369676e617475726520697320696e76616c696400000000000000606482015260840161066e565b505060008782602001516040516020016132019291906159c2565b60408051808303601f190181529190528051602090910120929b929a509198505050505050505050565b613233613af9565b6001600160a01b0381166132985760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b606482015260840161066e565b6104e981613b53565b600054610100900460ff16158080156132c15750600054600160ff909116105b806132db5750303b1580156132db575060005460ff166001145b61333e5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b606482015260840161066e565b6000805460ff191660011790558015613361576000805461ff0019166101001790555b61336c856000613eae565b61337584613b53565b60cf8054640100000000600160c01b0319166401000000006001600160a01b03868116919091029190911790915560d080546001600160a01b0319169184169190911790558015613400576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b5050505050565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561345a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061347e91906150fb565b6001600160a01b0316336001600160a01b0316146134ae5760405162461bcd60e51b815260040161066e90615118565b60665419811960665419161461352c5760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e756e70617573653a20696e76616c696420617474656d7060448201527f7420746f2070617573652066756e6374696f6e616c6974790000000000000000606482015260840161066e565b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c906020016107b4565b6001600160a01b0381166135f15760405162461bcd60e51b815260206004820152604960248201527f5061757361626c652e5f73657450617573657252656769737472793a206e657760448201527f50617573657252656769737472792063616e6e6f7420626520746865207a65726064820152686f206164647265737360b81b608482015260a40161066e565b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b60408051808201909152600080825260208201526136766142d3565b835181526020808501519082015260408082018490526000908360608460076107d05a03fa90508080156136a9576136ab565bfe5b50806136e95760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b604482015260640161066e565b505092915050565b604080518082019091526000808252602082015261370d6142f1565b835181526020808501518183015283516040808401919091529084015160608301526000908360808460066107d05a03fa90508080156136a95750806136e95760405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b604482015260640161066e565b61378d61430f565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b604080518082019091526000808252602082015260008080613875600080516020615a2d833981519152866151dd565b90505b61388181613f98565b9093509150600080516020615a2d8339815191528283098314156138bb576040805180820190915290815260208101919091529392505050565b600080516020615a2d833981519152600182089050613878565b604080518082018252868152602080820186905282518084019093528683528201849052600091829190613907614334565b60005b6002811015613acc5760006139208260066158f3565b9050848260028110613934576139346151c7565b6020020151518361394683600061595f565b600c8110613956576139566151c7565b602002015284826002811061396d5761396d6151c7565b60200201516020015183826001613984919061595f565b600c8110613994576139946151c7565b60200201528382600281106139ab576139ab6151c7565b60200201515151836139be83600261595f565b600c81106139ce576139ce6151c7565b60200201528382600281106139e5576139e56151c7565b60200201515160016020020151836139fe83600361595f565b600c8110613a0e57613a0e6151c7565b6020020152838260028110613a2557613a256151c7565b602002015160200151600060028110613a4057613a406151c7565b602002015183613a5183600461595f565b600c8110613a6157613a616151c7565b6020020152838260028110613a7857613a786151c7565b602002015160200151600160028110613a9357613a936151c7565b602002015183613aa483600561595f565b600c8110613ab457613ab46151c7565b60200201525080613ac4816152ce565b91505061390a565b50613ad5614353565b60006020826101808560088cfa9151919c9115159b50909950505050505050505050565b6033546001600160a01b03163314611c4f5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604482015260640161066e565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6060600080613bb384613d04565b61ffff166001600160401b03811115613bce57613bce61440d565b6040519080825280601f01601f191660200182016040528015613bf8576020820181803683370190505b5090506000805b825182108015613c10575061010081105b15613c67576001811b935085841615613c57578060f81b838381518110613c3957613c396151c7565b60200101906001600160f81b031916908160001a9053508160010191505b613c60816152ce565b9050613bff565b5090949350505050565b600080613c7d8461401a565b9050808360ff166001901b11613cfb5760405162461bcd60e51b815260206004820152603f60248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206269746d61702065786365656473206d61782076616c756500606482015260840161066e565b90505b92915050565b6000805b8215613cfe57613d1960018461592f565b9092169180613d2781615a0a565b915050613d08565b60408051808201909152600080825260208201526102008261ffff1610613d8b5760405162461bcd60e51b815260206004820152601060248201526f7363616c61722d746f6f2d6c6172676560801b604482015260640161066e565b8161ffff1660011415613d9f575081613cfe565b6040805180820190915260008082526020820181905284906001905b8161ffff168661ffff1610613e0857600161ffff871660ff83161c81161415613deb57613de884846136f1565b93505b613df583846136f1565b92506201fffe600192831b169101613dbb565b509195945050505050565b60408051808201909152600080825260208201528151158015613e3857506020820151155b15613e56575050604080518082019091526000808252602082015290565b604051806040016040528083600001518152602001600080516020615a2d8339815191528460200151613e8991906151dd565b613ea190600080516020615a2d83398151915261592f565b905292915050565b919050565b6065546001600160a01b0316158015613ecf57506001600160a01b03821615155b613f515760405162461bcd60e51b815260206004820152604760248201527f5061757361626c652e5f696e697469616c697a655061757365723a205f696e6960448201527f7469616c697a6550617573657228292063616e206f6e6c792062652063616c6c6064820152666564206f6e636560c81b608482015260a40161066e565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2613f9482613563565b5050565b60008080600080516020615a2d8339815191526003600080516020615a2d83398151915286600080516020615a2d83398151915288890909089050600061400e827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f52600080516020615a2d8339815191526141a7565b91959194509092505050565b6000610100825111156140a35760405162461bcd60e51b8152602060048201526044602482018190527f4269746d61705574696c732e6f72646572656442797465734172726179546f42908201527f69746d61703a206f7264657265644279746573417272617920697320746f6f206064820152636c6f6e6760e01b608482015260a40161066e565b81516140b157506000919050565b600080836000815181106140c7576140c76151c7565b0160200151600160f89190911c81901b92505b845181101561419e578481815181106140f5576140f56151c7565b0160200151600160f89190911c1b915082821161418a5760405162461bcd60e51b815260206004820152604760248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f72646572656442797465734172726179206973206e6f74206064820152661bdc99195c995960ca1b608482015260a40161066e565b91811791614197816152ce565b90506140da565b50909392505050565b6000806141b2614353565b6141ba614371565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa92508280156136a95750826142445760405162461bcd60e51b815260206004820152601a60248201527f424e3235342e6578704d6f643a2063616c6c206661696c757265000000000000604482015260640161066e565b505195945050505050565b82805461425b906154f7565b90600052602060002090601f01602090048101928261427d57600085556142c3565b82601f106142965782800160ff198235161785556142c3565b828001600101855582156142c3579182015b828111156142c35782358255916020019190600101906142a8565b506142cf92915061438f565b5090565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b60405180604001604052806143226143a4565b815260200161432f6143a4565b905290565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b5b808211156142cf5760008155600101614390565b60405180604001604052806002906020820280368337509192915050565b6001600160a01b03811681146104e957600080fd5b6000602082840312156143e957600080fd5b8135613cfb816143c2565b60006020828403121561440657600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b03811182821017156144455761444561440d565b60405290565b60405161018081016001600160401b03811182821017156144455761444561440d565b604051601f8201601f191681016001600160401b03811182821017156144965761449661440d565b604052919050565b6000604082840312156144b057600080fd5b6144b8614423565b9050813581526020820135602082015292915050565b600082601f8301126144df57600080fd5b604051604081018181106001600160401b03821117156145015761450161440d565b806040525080604084018581111561451857600080fd5b845b81811015613e0857803583526020928301920161451a565b60006080828403121561454457600080fd5b61454c614423565b905061455883836144ce565b815261456783604084016144ce565b602082015292915050565b600080600080610120858703121561458957600080fd5b8435935061459a866020870161449e565b92506145a98660608701614532565b91506145b88660e0870161449e565b905092959194509250565b63ffffffff811681146104e957600080fd5b8035613ea9816145c3565b6000602082840312156145f257600080fd5b8135613cfb816145c3565b60008060006060848603121561461257600080fd5b833561461d816143c2565b92506020848101356001600160401b038082111561463a57600080fd5b818701915087601f83011261464e57600080fd5b8135818111156146605761466061440d565b614672601f8201601f1916850161446e565b9150808252888482850101111561468857600080fd5b80848401858401376000848284010152508094505050506146ab604085016145d5565b90509250925092565b600081518084526020808501808196508360051b810191508286016000805b8681101561474a578385038a52825180518087529087019087870190845b8181101561473557835180516001600160a01b031684528a8101518b8501526040908101516001600160601b031690840152928901926060909201916001016146f1565b50509a87019a955050918501916001016146d3565b509298975050505050505050565b60208152600061476b60208301846146b4565b9392505050565b80151581146104e957600080fd5b60006020828403121561479257600080fd5b8135613cfb81614772565b60008083601f8401126147af57600080fd5b5081356001600160401b038111156147c657600080fd5b6020830191508360208285010111156147de57600080fd5b9250929050565b600080600080600080608087890312156147fe57600080fd5b8635614809816143c2565b95506020870135614819816145c3565b945060408701356001600160401b038082111561483557600080fd5b6148418a838b0161479d565b9096509450606089013591508082111561485a57600080fd5b818901915089601f83011261486e57600080fd5b81358181111561487d57600080fd5b8a60208260051b850101111561489257600080fd5b6020830194508093505050509295509295509295565b600081518084526020808501945080840160005b838110156148de57815163ffffffff16875295820195908201906001016148bc565b509495945050505050565b60006020808352835160808285015261490560a08501826148a8565b905081850151601f198086840301604087015261492283836148a8565b9250604087015191508086840301606087015261493f83836148a8565b60608801518782038301608089015280518083529194508501925084840190600581901b8501860160005b8281101561499657848783030184526149848287516148a8565b9588019593880193915060010161496a565b509998505050505050505050565b60ff811681146104e957600080fd5b6000602082840312156149c557600080fd5b8135613cfb816149a4565b60006001600160401b038211156149e9576149e961440d565b5060051b60200190565b600080600060608486031215614a0857600080fd5b8335614a13816143c2565b92506020848101356001600160401b03811115614a2f57600080fd5b8501601f81018713614a4057600080fd5b8035614a53614a4e826149d0565b61446e565b81815260059190911b82018301908381019089831115614a7257600080fd5b928401925b82841015614a9057833582529284019290840190614a77565b80965050505050506146ab604085016145d5565b6020808252825182820181905260009190848201906040850190845b81811015614adc57835183529284019291840191600101614ac0565b50909695505050505050565b60008060008060608587031215614afe57600080fd5b843593506020850135614b10816145c3565b925060408501356001600160401b03811115614b2b57600080fd5b614b378782880161479d565b95989497509550505050565b634e487b7160e01b600052602160045260246000fd5b6020810160048310614b7b57634e487b7160e01b600052602160045260246000fd5b91905290565b600060208284031215614b9357600080fd5b81356001600160401b03811115614ba957600080fd5b82016101408185031215613cfb57600080fd5b600082601f830112614bcd57600080fd5b81356020614bdd614a4e836149d0565b82815260059290921b84018101918181019086841115614bfc57600080fd5b8286015b84811015614c20578035614c13816145c3565b8352918301918301614c00565b509695505050505050565b600082601f830112614c3c57600080fd5b81356020614c4c614a4e836149d0565b82815260069290921b84018101918181019086841115614c6b57600080fd5b8286015b84811015614c2057614c81888261449e565b835291830191604001614c6f565b600082601f830112614ca057600080fd5b81356020614cb0614a4e836149d0565b82815260059290921b84018101918181019086841115614ccf57600080fd5b8286015b84811015614c205780356001600160401b03811115614cf25760008081fd5b614d008986838b0101614bbc565b845250918301918301614cd3565b60006102808284031215614d2157600080fd5b614d2961444b565b905081356001600160401b0380821115614d4257600080fd5b614d4e85838601614bbc565b83526020840135915080821115614d6457600080fd5b614d7085838601614c2b565b60208401526040840135915080821115614d8957600080fd5b614d9585838601614c2b565b6040840152614da78560608601614532565b6060840152614db98560e0860161449e565b608084015261012091508184013581811115614dd457600080fd5b614de086828701614bbc565b60a0850152506101408085013582811115614dfa57600080fd5b614e0687828801614bbc565b60c0860152506101608086013583811115614e2057600080fd5b614e2c88828901614c8f565b60e08701525061018086013583811115614e4557600080fd5b614e5188828901614bbc565b610100870152506101a086013583811115614e6b57600080fd5b614e7788828901614c2b565b8587015250614e8a876101c08801614532565b82860152614e9c87610240880161449e565b818601525050505092915050565b6000806000838503610100811215614ec157600080fd5b84356001600160401b0380821115614ed857600080fd5b9086019060e08289031215614eec57600080fd5b81955060c0601f1984011215614f0157600080fd5b60208701945060e0870135925080831115614f1b57600080fd5b5050614f2986828701614d0e565b9150509250925092565b600080600060608486031215614f4857600080fd5b8335614f53816143c2565b9250602084013591506040840135614f6a816145c3565b809150509250925092565b828152604060208201526000614f8e60408301846146b4565b949350505050565b600080600080600060808688031215614fae57600080fd5b8535945060208601356001600160401b0380821115614fcc57600080fd5b614fd889838a0161479d565b909650945060408801359150614fed826145c3565b9092506060870135908082111561500357600080fd5b5061501088828901614d0e565b9150509295509295909350565b600081518084526020808501945080840160005b838110156148de5781516001600160601b031687529582019590820190600101615031565b6040815260008351604080840152615071608084018261501d565b90506020850151603f1984830301606085015261508e828261501d565b925050508260208301529392505050565b600080600080608085870312156150b557600080fd5b84356150c0816143c2565b935060208501356150d0816143c2565b925060408501356150e0816143c2565b915060608501356150f0816143c2565b939692955090935050565b60006020828403121561510d57600080fd5b8151613cfb816143c2565b6020808252602a908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526939903ab73830bab9b2b960b11b606082015260800190565b60006020828403121561517457600080fd5b8151613cfb81614772565b60208082526028908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526739903830bab9b2b960c11b606082015260800190565b634e487b7160e01b600052603260045260246000fd5b6000826151fa57634e487b7160e01b600052601260045260246000fd5b500690565b6000602080838503121561521257600080fd5b82516001600160401b0381111561522857600080fd5b8301601f8101851361523957600080fd5b8051615247614a4e826149d0565b81815260059190911b8201830190838101908783111561526657600080fd5b928401925b828410156152845783518252928401929084019061526b565b979650505050505050565b6000602082840312156152a157600080fd5b81516001600160601b0381168114613cfb57600080fd5b634e487b7160e01b600052601160045260246000fd5b60006000198214156152e2576152e26152b8565b5060010190565b63ffffffff84168152604060208201819052810182905260006001600160fb1b0383111561531657600080fd5b8260051b8085606085013760009201606001918252509392505050565b6000602080838503121561534657600080fd5b82516001600160401b0381111561535c57600080fd5b8301601f8101851361536d57600080fd5b805161537b614a4e826149d0565b81815260059190911b8201830190838101908783111561539a57600080fd5b928401925b828410156152845783516153b2816145c3565b8252928401929084019061539f565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b63ffffffff8416815260406020820152600061540a6040830184866153c1565b95945050505050565b60006020828403121561542557600080fd5b81516001600160c01b0381168114613cfb57600080fd5b60006020828403121561544e57600080fd5b8151613cfb816145c3565b600060ff821660ff811415615470576154706152b8565b60010192915050565b60408152600061548d6040830185876153c1565b905063ffffffff83166020830152949350505050565b60006040820163ffffffff851683526020604081850152818551808452606086019150828701935060005b818110156154ea578451835293830193918301916001016154ce565b5090979650505050505050565b600181811c9082168061550b57607f821691505b6020821081141561552c57634e487b7160e01b600052602260045260246000fd5b50919050565b600063ffffffff8381169083168181101561554f5761554f6152b8565b039392505050565b6000815180845260005b8181101561557d57602081850181015186830182015201615561565b8181111561558f576000602083870101525b50601f01601f19169290920160200192915050565b60208152815160208201526000602083015163ffffffff80821660408501528060408601511660608501526060850151915060e060808501526155eb610100850183615557565b91508060808601511660a08501525060a0840151601f198483030160c08501526156158282615557565b91505060c084015161562f60e085018263ffffffff169052565b509392505050565b600063ffffffff808316818516808303821115615656576156566152b8565b01949350505050565b6000808335601e1984360301811261567657600080fd5b8301803591506001600160401b0382111561569057600080fd5b6020019150368190038213156147de57600080fd5b6000808335601e198436030181126156bc57600080fd5b83016020810192503590506001600160401b038111156156db57600080fd5b8036038313156147de57600080fd5b602081528135602082015260006020830135615705816145c3565b63ffffffff81166040840152506040830135615720816145c3565b63ffffffff811660608401525061573a60608401846156a5565b60e06080850152615750610100850182846153c1565b91505061575f608085016145d5565b63ffffffff811660a08501525061577960a08501856156a5565b848303601f190160c08601526157908382846153c1565b925050506157a060c085016145d5565b63ffffffff811660e085015261562f565b6020808252602c908201527f41676772656761746f722068617320616c726561647920726573706f6e64656460408201526b20746f20746865207461736b60a01b606082015260800190565b8035615808816145c3565b63ffffffff81168352506020810135602083015260408101356040830152606081013560608301526080810135608083015260a081013560a08301525050565b60c08101613cfe82846157fd565b61586081846157fd565b60e060c082015263ffffffff82511660e0820152602082015161010082015260006040830151608061012084015261589c61016084018261501d565b9050606084015160df19848303016101408501526158ba828261501d565b9695505050505050565b60006001600160601b03808316818516818304811182151516156158ea576158ea6152b8565b02949350505050565b600081600019048311821515161561590d5761590d6152b8565b500290565b60006020828403121561592457600080fd5b8151613cfb816149a4565b600082821015615941576159416152b8565b500390565b60006020828403121561595857600080fd5b5051919050565b60008219821115615972576159726152b8565b500190565b60006020828403121561598957600080fd5b815167ffffffffffffffff1981168114613cfb57600080fd5b60006001600160601b038381169083168181101561554f5761554f6152b8565b63ffffffff60e01b8360e01b1681526000600482018351602080860160005b838110156159fd578151855293820193908201906001016159e1565b5092979650505050505050565b600061ffff80831681811415615a2257615a226152b8565b600101939250505056fe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47424c535369676e6174757265436865636b65722e636865636b5369676e617475a2646970667358221220b2bffe6bed4e9ac4b060d01d4a30f19844f0937b767cc095ffd7e624ed50e60764736f6c634300080c0033",
}

// ContractFinalizerTaskManagerABI is the input ABI used to generate the binding from.
// Deprecated: Use ContractFinalizerTaskManagerMetaData.ABI instead.
var ContractFinalizerTaskManagerABI = ContractFinalizerTaskManagerMetaData.ABI

// ContractFinalizerTaskManagerBin is the compiled bytecode used for deploying new contracts.
// Deprecated: Use ContractFinalizerTaskManagerMetaData.Bin instead.
var ContractFinalizerTaskManagerBin = ContractFinalizerTaskManagerMetaData.Bin

// DeployContractFinalizerTaskManager deploys a new Ethereum contract, binding an instance of ContractFinalizerTaskManager to it.
func DeployContractFinalizerTaskManager(auth *bind.TransactOpts, backend bind.ContractBackend, _registryCoordinator common.Address, _taskResponseWindowBlock uint32) (common.Address, *types.Transaction, *ContractFinalizerTaskManager, error) {
	parsed, err := ContractFinalizerTaskManagerMetaData.GetAbi()
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	if parsed == nil {
		return common.Address{}, nil, nil, errors.New("GetABI returned nil")
	}

	address, tx, contract, err := bind.DeployContract(auth, *parsed, common.FromHex(ContractFinalizerTaskManagerBin), backend, _registryCoordinator, _taskResponseWindowBlock)
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

// AllTaskHashes is a free data retrieval call binding the contract method 0x2d89f6fc.
//
// Solidity: function allTaskHashes(uint32 ) view returns(bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) AllTaskHashes(opts *bind.CallOpts, arg0 uint32) ([32]byte, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "allTaskHashes", arg0)

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// AllTaskHashes is a free data retrieval call binding the contract method 0x2d89f6fc.
//
// Solidity: function allTaskHashes(uint32 ) view returns(bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) AllTaskHashes(arg0 uint32) ([32]byte, error) {
	return _ContractFinalizerTaskManager.Contract.AllTaskHashes(&_ContractFinalizerTaskManager.CallOpts, arg0)
}

// AllTaskHashes is a free data retrieval call binding the contract method 0x2d89f6fc.
//
// Solidity: function allTaskHashes(uint32 ) view returns(bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) AllTaskHashes(arg0 uint32) ([32]byte, error) {
	return _ContractFinalizerTaskManager.Contract.AllTaskHashes(&_ContractFinalizerTaskManager.CallOpts, arg0)
}

// AllTaskResponses is a free data retrieval call binding the contract method 0x2cb223d5.
//
// Solidity: function allTaskResponses(uint32 ) view returns(bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) AllTaskResponses(opts *bind.CallOpts, arg0 uint32) ([32]byte, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "allTaskResponses", arg0)

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// AllTaskResponses is a free data retrieval call binding the contract method 0x2cb223d5.
//
// Solidity: function allTaskResponses(uint32 ) view returns(bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) AllTaskResponses(arg0 uint32) ([32]byte, error) {
	return _ContractFinalizerTaskManager.Contract.AllTaskResponses(&_ContractFinalizerTaskManager.CallOpts, arg0)
}

// AllTaskResponses is a free data retrieval call binding the contract method 0x2cb223d5.
//
// Solidity: function allTaskResponses(uint32 ) view returns(bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) AllTaskResponses(arg0 uint32) ([32]byte, error) {
	return _ContractFinalizerTaskManager.Contract.AllTaskResponses(&_ContractFinalizerTaskManager.CallOpts, arg0)
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

// CheckSignatures is a free data retrieval call binding the contract method 0xdb67f777.
//
// Solidity: function checkSignatures(bytes32 msgHash, bytes quorumNumbers, uint32 referenceBlockNumber, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][],uint32[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)) params) view returns((uint96[],uint96[]), bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) CheckSignatures(opts *bind.CallOpts, msgHash [32]byte, quorumNumbers []byte, referenceBlockNumber uint32, params IBLSSignatureCheckerNonSignerStakesAndSignature) (IBLSSignatureCheckerQuorumStakeTotals, [32]byte, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "checkSignatures", msgHash, quorumNumbers, referenceBlockNumber, params)

	if err != nil {
		return *new(IBLSSignatureCheckerQuorumStakeTotals), *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new(IBLSSignatureCheckerQuorumStakeTotals)).(*IBLSSignatureCheckerQuorumStakeTotals)
	out1 := *abi.ConvertType(out[1], new([32]byte)).(*[32]byte)

	return out0, out1, err

}

// CheckSignatures is a free data retrieval call binding the contract method 0xdb67f777.
//
// Solidity: function checkSignatures(bytes32 msgHash, bytes quorumNumbers, uint32 referenceBlockNumber, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][],uint32[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)) params) view returns((uint96[],uint96[]), bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) CheckSignatures(msgHash [32]byte, quorumNumbers []byte, referenceBlockNumber uint32, params IBLSSignatureCheckerNonSignerStakesAndSignature) (IBLSSignatureCheckerQuorumStakeTotals, [32]byte, error) {
	return _ContractFinalizerTaskManager.Contract.CheckSignatures(&_ContractFinalizerTaskManager.CallOpts, msgHash, quorumNumbers, referenceBlockNumber, params)
}

// CheckSignatures is a free data retrieval call binding the contract method 0xdb67f777.
//
// Solidity: function checkSignatures(bytes32 msgHash, bytes quorumNumbers, uint32 referenceBlockNumber, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][],uint32[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)) params) view returns((uint96[],uint96[]), bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) CheckSignatures(msgHash [32]byte, quorumNumbers []byte, referenceBlockNumber uint32, params IBLSSignatureCheckerNonSignerStakesAndSignature) (IBLSSignatureCheckerQuorumStakeTotals, [32]byte, error) {
	return _ContractFinalizerTaskManager.Contract.CheckSignatures(&_ContractFinalizerTaskManager.CallOpts, msgHash, quorumNumbers, referenceBlockNumber, params)
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

// DummyForOperatorStateInfoType is a free data retrieval call binding the contract method 0xc3af8313.
//
// Solidity: function dummyForOperatorStateInfoType((bool,bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) _operatorStateInfo) view returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) DummyForOperatorStateInfoType(opts *bind.CallOpts, _operatorStateInfo IFinalizerTaskManagerOperatorStateInfo) error {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "dummyForOperatorStateInfoType", _operatorStateInfo)

	if err != nil {
		return err
	}

	return err

}

// DummyForOperatorStateInfoType is a free data retrieval call binding the contract method 0xc3af8313.
//
// Solidity: function dummyForOperatorStateInfoType((bool,bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) _operatorStateInfo) view returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) DummyForOperatorStateInfoType(_operatorStateInfo IFinalizerTaskManagerOperatorStateInfo) error {
	return _ContractFinalizerTaskManager.Contract.DummyForOperatorStateInfoType(&_ContractFinalizerTaskManager.CallOpts, _operatorStateInfo)
}

// DummyForOperatorStateInfoType is a free data retrieval call binding the contract method 0xc3af8313.
//
// Solidity: function dummyForOperatorStateInfoType((bool,bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) _operatorStateInfo) view returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) DummyForOperatorStateInfoType(_operatorStateInfo IFinalizerTaskManagerOperatorStateInfo) error {
	return _ContractFinalizerTaskManager.Contract.DummyForOperatorStateInfoType(&_ContractFinalizerTaskManager.CallOpts, _operatorStateInfo)
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

// GetLatestPendingStateHash is a free data retrieval call binding the contract method 0x0373408d.
//
// Solidity: function getLatestPendingStateHash() view returns(bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) GetLatestPendingStateHash(opts *bind.CallOpts) ([32]byte, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "getLatestPendingStateHash")

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// GetLatestPendingStateHash is a free data retrieval call binding the contract method 0x0373408d.
//
// Solidity: function getLatestPendingStateHash() view returns(bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) GetLatestPendingStateHash() ([32]byte, error) {
	return _ContractFinalizerTaskManager.Contract.GetLatestPendingStateHash(&_ContractFinalizerTaskManager.CallOpts)
}

// GetLatestPendingStateHash is a free data retrieval call binding the contract method 0x0373408d.
//
// Solidity: function getLatestPendingStateHash() view returns(bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) GetLatestPendingStateHash() ([32]byte, error) {
	return _ContractFinalizerTaskManager.Contract.GetLatestPendingStateHash(&_ContractFinalizerTaskManager.CallOpts)
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

// GetTaskResponseWindowBlock is a free data retrieval call binding the contract method 0xf5c9899d.
//
// Solidity: function getTaskResponseWindowBlock() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) GetTaskResponseWindowBlock(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "getTaskResponseWindowBlock")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// GetTaskResponseWindowBlock is a free data retrieval call binding the contract method 0xf5c9899d.
//
// Solidity: function getTaskResponseWindowBlock() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) GetTaskResponseWindowBlock() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.GetTaskResponseWindowBlock(&_ContractFinalizerTaskManager.CallOpts)
}

// GetTaskResponseWindowBlock is a free data retrieval call binding the contract method 0xf5c9899d.
//
// Solidity: function getTaskResponseWindowBlock() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) GetTaskResponseWindowBlock() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.GetTaskResponseWindowBlock(&_ContractFinalizerTaskManager.CallOpts)
}

// IndexToTaskStatus is a free data retrieval call binding the contract method 0x99dba0c4.
//
// Solidity: function indexToTaskStatus(uint32 ) view returns(uint8)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) IndexToTaskStatus(opts *bind.CallOpts, arg0 uint32) (uint8, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "indexToTaskStatus", arg0)

	if err != nil {
		return *new(uint8), err
	}

	out0 := *abi.ConvertType(out[0], new(uint8)).(*uint8)

	return out0, err

}

// IndexToTaskStatus is a free data retrieval call binding the contract method 0x99dba0c4.
//
// Solidity: function indexToTaskStatus(uint32 ) view returns(uint8)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) IndexToTaskStatus(arg0 uint32) (uint8, error) {
	return _ContractFinalizerTaskManager.Contract.IndexToTaskStatus(&_ContractFinalizerTaskManager.CallOpts, arg0)
}

// IndexToTaskStatus is a free data retrieval call binding the contract method 0x99dba0c4.
//
// Solidity: function indexToTaskStatus(uint32 ) view returns(uint8)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) IndexToTaskStatus(arg0 uint32) (uint8, error) {
	return _ContractFinalizerTaskManager.Contract.IndexToTaskStatus(&_ContractFinalizerTaskManager.CallOpts, arg0)
}

// LatestTaskNum is a free data retrieval call binding the contract method 0x8b00ce7c.
//
// Solidity: function latestTaskNum() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) LatestTaskNum(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "latestTaskNum")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// LatestTaskNum is a free data retrieval call binding the contract method 0x8b00ce7c.
//
// Solidity: function latestTaskNum() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) LatestTaskNum() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.LatestTaskNum(&_ContractFinalizerTaskManager.CallOpts)
}

// LatestTaskNum is a free data retrieval call binding the contract method 0x8b00ce7c.
//
// Solidity: function latestTaskNum() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) LatestTaskNum() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.LatestTaskNum(&_ContractFinalizerTaskManager.CallOpts)
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

// StaleStakesForbidden is a free data retrieval call binding the contract method 0xb98d0908.
//
// Solidity: function staleStakesForbidden() view returns(bool)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) StaleStakesForbidden(opts *bind.CallOpts) (bool, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "staleStakesForbidden")

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// StaleStakesForbidden is a free data retrieval call binding the contract method 0xb98d0908.
//
// Solidity: function staleStakesForbidden() view returns(bool)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) StaleStakesForbidden() (bool, error) {
	return _ContractFinalizerTaskManager.Contract.StaleStakesForbidden(&_ContractFinalizerTaskManager.CallOpts)
}

// StaleStakesForbidden is a free data retrieval call binding the contract method 0xb98d0908.
//
// Solidity: function staleStakesForbidden() view returns(bool)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) StaleStakesForbidden() (bool, error) {
	return _ContractFinalizerTaskManager.Contract.StaleStakesForbidden(&_ContractFinalizerTaskManager.CallOpts)
}

// TaskNumber is a free data retrieval call binding the contract method 0x72d18e8d.
//
// Solidity: function taskNumber() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) TaskNumber(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "taskNumber")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// TaskNumber is a free data retrieval call binding the contract method 0x72d18e8d.
//
// Solidity: function taskNumber() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) TaskNumber() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.TaskNumber(&_ContractFinalizerTaskManager.CallOpts)
}

// TaskNumber is a free data retrieval call binding the contract method 0x72d18e8d.
//
// Solidity: function taskNumber() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) TaskNumber() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.TaskNumber(&_ContractFinalizerTaskManager.CallOpts)
}

// TrySignatureAndApkVerification is a free data retrieval call binding the contract method 0x171f1d5b.
//
// Solidity: function trySignatureAndApkVerification(bytes32 msgHash, (uint256,uint256) apk, (uint256[2],uint256[2]) apkG2, (uint256,uint256) sigma) view returns(bool pairingSuccessful, bool siganatureIsValid)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) TrySignatureAndApkVerification(opts *bind.CallOpts, msgHash [32]byte, apk BN254G1Point, apkG2 BN254G2Point, sigma BN254G1Point) (struct {
	PairingSuccessful bool
	SiganatureIsValid bool
}, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "trySignatureAndApkVerification", msgHash, apk, apkG2, sigma)

	outstruct := new(struct {
		PairingSuccessful bool
		SiganatureIsValid bool
	})
	if err != nil {
		return *outstruct, err
	}

	outstruct.PairingSuccessful = *abi.ConvertType(out[0], new(bool)).(*bool)
	outstruct.SiganatureIsValid = *abi.ConvertType(out[1], new(bool)).(*bool)

	return *outstruct, err

}

// TrySignatureAndApkVerification is a free data retrieval call binding the contract method 0x171f1d5b.
//
// Solidity: function trySignatureAndApkVerification(bytes32 msgHash, (uint256,uint256) apk, (uint256[2],uint256[2]) apkG2, (uint256,uint256) sigma) view returns(bool pairingSuccessful, bool siganatureIsValid)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) TrySignatureAndApkVerification(msgHash [32]byte, apk BN254G1Point, apkG2 BN254G2Point, sigma BN254G1Point) (struct {
	PairingSuccessful bool
	SiganatureIsValid bool
}, error) {
	return _ContractFinalizerTaskManager.Contract.TrySignatureAndApkVerification(&_ContractFinalizerTaskManager.CallOpts, msgHash, apk, apkG2, sigma)
}

// TrySignatureAndApkVerification is a free data retrieval call binding the contract method 0x171f1d5b.
//
// Solidity: function trySignatureAndApkVerification(bytes32 msgHash, (uint256,uint256) apk, (uint256[2],uint256[2]) apkG2, (uint256,uint256) sigma) view returns(bool pairingSuccessful, bool siganatureIsValid)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) TrySignatureAndApkVerification(msgHash [32]byte, apk BN254G1Point, apkG2 BN254G2Point, sigma BN254G1Point) (struct {
	PairingSuccessful bool
	SiganatureIsValid bool
}, error) {
	return _ContractFinalizerTaskManager.Contract.TrySignatureAndApkVerification(&_ContractFinalizerTaskManager.CallOpts, msgHash, apk, apkG2, sigma)
}

// CreateNewTask is a paid mutator transaction binding the contract method 0x6b92787e.
//
// Solidity: function createNewTask(uint256 blockNumber, uint32 quorumThresholdPercentage, bytes quorumNumbers) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) CreateNewTask(opts *bind.TransactOpts, blockNumber *big.Int, quorumThresholdPercentage uint32, quorumNumbers []byte) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "createNewTask", blockNumber, quorumThresholdPercentage, quorumNumbers)
}

// CreateNewTask is a paid mutator transaction binding the contract method 0x6b92787e.
//
// Solidity: function createNewTask(uint256 blockNumber, uint32 quorumThresholdPercentage, bytes quorumNumbers) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) CreateNewTask(blockNumber *big.Int, quorumThresholdPercentage uint32, quorumNumbers []byte) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.CreateNewTask(&_ContractFinalizerTaskManager.TransactOpts, blockNumber, quorumThresholdPercentage, quorumNumbers)
}

// CreateNewTask is a paid mutator transaction binding the contract method 0x6b92787e.
//
// Solidity: function createNewTask(uint256 blockNumber, uint32 quorumThresholdPercentage, bytes quorumNumbers) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) CreateNewTask(blockNumber *big.Int, quorumThresholdPercentage uint32, quorumNumbers []byte) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.CreateNewTask(&_ContractFinalizerTaskManager.TransactOpts, blockNumber, quorumThresholdPercentage, quorumNumbers)
}

// Initialize is a paid mutator transaction binding the contract method 0xf8c8765e.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _aggregator, address _generator) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) Initialize(opts *bind.TransactOpts, _pauserRegistry common.Address, initialOwner common.Address, _aggregator common.Address, _generator common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "initialize", _pauserRegistry, initialOwner, _aggregator, _generator)
}

// Initialize is a paid mutator transaction binding the contract method 0xf8c8765e.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _aggregator, address _generator) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) Initialize(_pauserRegistry common.Address, initialOwner common.Address, _aggregator common.Address, _generator common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.Initialize(&_ContractFinalizerTaskManager.TransactOpts, _pauserRegistry, initialOwner, _aggregator, _generator)
}

// Initialize is a paid mutator transaction binding the contract method 0xf8c8765e.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _aggregator, address _generator) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) Initialize(_pauserRegistry common.Address, initialOwner common.Address, _aggregator common.Address, _generator common.Address) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.Initialize(&_ContractFinalizerTaskManager.TransactOpts, _pauserRegistry, initialOwner, _aggregator, _generator)
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

// RespondToTask is a paid mutator transaction binding the contract method 0xcab0e6fd.
//
// Solidity: function respondToTask((uint256,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32,bytes32,bytes32,bytes32) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][],uint32[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)) nonSignerStakesAndSignature) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) RespondToTask(opts *bind.TransactOpts, task IFinalizerTaskManagerTask, taskResponse IFinalizerTaskManagerTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "respondToTask", task, taskResponse, nonSignerStakesAndSignature)
}

// RespondToTask is a paid mutator transaction binding the contract method 0xcab0e6fd.
//
// Solidity: function respondToTask((uint256,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32,bytes32,bytes32,bytes32) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][],uint32[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)) nonSignerStakesAndSignature) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) RespondToTask(task IFinalizerTaskManagerTask, taskResponse IFinalizerTaskManagerTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.RespondToTask(&_ContractFinalizerTaskManager.TransactOpts, task, taskResponse, nonSignerStakesAndSignature)
}

// RespondToTask is a paid mutator transaction binding the contract method 0xcab0e6fd.
//
// Solidity: function respondToTask((uint256,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32,bytes32,bytes32,bytes32) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][],uint32[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)) nonSignerStakesAndSignature) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) RespondToTask(task IFinalizerTaskManagerTask, taskResponse IFinalizerTaskManagerTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.RespondToTask(&_ContractFinalizerTaskManager.TransactOpts, task, taskResponse, nonSignerStakesAndSignature)
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

// SetStaleStakesForbidden is a paid mutator transaction binding the contract method 0x416c7e5e.
//
// Solidity: function setStaleStakesForbidden(bool value) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) SetStaleStakesForbidden(opts *bind.TransactOpts, value bool) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "setStaleStakesForbidden", value)
}

// SetStaleStakesForbidden is a paid mutator transaction binding the contract method 0x416c7e5e.
//
// Solidity: function setStaleStakesForbidden(bool value) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) SetStaleStakesForbidden(value bool) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.SetStaleStakesForbidden(&_ContractFinalizerTaskManager.TransactOpts, value)
}

// SetStaleStakesForbidden is a paid mutator transaction binding the contract method 0x416c7e5e.
//
// Solidity: function setStaleStakesForbidden(bool value) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) SetStaleStakesForbidden(value bool) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.SetStaleStakesForbidden(&_ContractFinalizerTaskManager.TransactOpts, value)
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

// ContractFinalizerTaskManagerNewTaskCreatedIterator is returned from FilterNewTaskCreated and is used to iterate over the raw logs and unpacked data for NewTaskCreated events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerNewTaskCreatedIterator struct {
	Event *ContractFinalizerTaskManagerNewTaskCreated // Event containing the contract specifics and raw log

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
func (it *ContractFinalizerTaskManagerNewTaskCreatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerNewTaskCreated)
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
		it.Event = new(ContractFinalizerTaskManagerNewTaskCreated)
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
func (it *ContractFinalizerTaskManagerNewTaskCreatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerNewTaskCreatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerNewTaskCreated represents a NewTaskCreated event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerNewTaskCreated struct {
	TaskIndex uint32
	Task      IFinalizerTaskManagerTask
	Raw       types.Log // Blockchain specific contextual infos
}

// FilterNewTaskCreated is a free log retrieval operation binding the contract event 0x933fbdf390ba7173f9bf83f5459edf1ae41ae5edbde6c8f9b374d979d30900be.
//
// Solidity: event NewTaskCreated(uint32 indexed taskIndex, (uint256,uint32,uint32,bytes,uint32,bytes,uint32) task)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterNewTaskCreated(opts *bind.FilterOpts, taskIndex []uint32) (*ContractFinalizerTaskManagerNewTaskCreatedIterator, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "NewTaskCreated", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerNewTaskCreatedIterator{contract: _ContractFinalizerTaskManager.contract, event: "NewTaskCreated", logs: logs, sub: sub}, nil
}

// WatchNewTaskCreated is a free log subscription operation binding the contract event 0x933fbdf390ba7173f9bf83f5459edf1ae41ae5edbde6c8f9b374d979d30900be.
//
// Solidity: event NewTaskCreated(uint32 indexed taskIndex, (uint256,uint32,uint32,bytes,uint32,bytes,uint32) task)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchNewTaskCreated(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerNewTaskCreated, taskIndex []uint32) (event.Subscription, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "NewTaskCreated", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerNewTaskCreated)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "NewTaskCreated", log); err != nil {
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

// ParseNewTaskCreated is a log parse operation binding the contract event 0x933fbdf390ba7173f9bf83f5459edf1ae41ae5edbde6c8f9b374d979d30900be.
//
// Solidity: event NewTaskCreated(uint32 indexed taskIndex, (uint256,uint32,uint32,bytes,uint32,bytes,uint32) task)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParseNewTaskCreated(log types.Log) (*ContractFinalizerTaskManagerNewTaskCreated, error) {
	event := new(ContractFinalizerTaskManagerNewTaskCreated)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "NewTaskCreated", log); err != nil {
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

// ContractFinalizerTaskManagerTaskCompletedIterator is returned from FilterTaskCompleted and is used to iterate over the raw logs and unpacked data for TaskCompleted events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerTaskCompletedIterator struct {
	Event *ContractFinalizerTaskManagerTaskCompleted // Event containing the contract specifics and raw log

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
func (it *ContractFinalizerTaskManagerTaskCompletedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerTaskCompleted)
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
		it.Event = new(ContractFinalizerTaskManagerTaskCompleted)
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
func (it *ContractFinalizerTaskManagerTaskCompletedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerTaskCompletedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerTaskCompleted represents a TaskCompleted event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerTaskCompleted struct {
	TaskIndex uint32
	BlockHash [32]byte
	Raw       types.Log // Blockchain specific contextual infos
}

// FilterTaskCompleted is a free log retrieval operation binding the contract event 0x8378be8a33cf3a493910a16e275cd96af4f048c5eb1a2c2962d4066e697fea80.
//
// Solidity: event TaskCompleted(uint32 indexed taskIndex, bytes32 indexed blockHash)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterTaskCompleted(opts *bind.FilterOpts, taskIndex []uint32, blockHash [][32]byte) (*ContractFinalizerTaskManagerTaskCompletedIterator, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}
	var blockHashRule []interface{}
	for _, blockHashItem := range blockHash {
		blockHashRule = append(blockHashRule, blockHashItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "TaskCompleted", taskIndexRule, blockHashRule)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerTaskCompletedIterator{contract: _ContractFinalizerTaskManager.contract, event: "TaskCompleted", logs: logs, sub: sub}, nil
}

// WatchTaskCompleted is a free log subscription operation binding the contract event 0x8378be8a33cf3a493910a16e275cd96af4f048c5eb1a2c2962d4066e697fea80.
//
// Solidity: event TaskCompleted(uint32 indexed taskIndex, bytes32 indexed blockHash)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchTaskCompleted(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerTaskCompleted, taskIndex []uint32, blockHash [][32]byte) (event.Subscription, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}
	var blockHashRule []interface{}
	for _, blockHashItem := range blockHash {
		blockHashRule = append(blockHashRule, blockHashItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "TaskCompleted", taskIndexRule, blockHashRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerTaskCompleted)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "TaskCompleted", log); err != nil {
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

// ParseTaskCompleted is a log parse operation binding the contract event 0x8378be8a33cf3a493910a16e275cd96af4f048c5eb1a2c2962d4066e697fea80.
//
// Solidity: event TaskCompleted(uint32 indexed taskIndex, bytes32 indexed blockHash)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParseTaskCompleted(log types.Log) (*ContractFinalizerTaskManagerTaskCompleted, error) {
	event := new(ContractFinalizerTaskManagerTaskCompleted)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "TaskCompleted", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractFinalizerTaskManagerTaskRespondedIterator is returned from FilterTaskResponded and is used to iterate over the raw logs and unpacked data for TaskResponded events raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerTaskRespondedIterator struct {
	Event *ContractFinalizerTaskManagerTaskResponded // Event containing the contract specifics and raw log

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
func (it *ContractFinalizerTaskManagerTaskRespondedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerTaskManagerTaskResponded)
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
		it.Event = new(ContractFinalizerTaskManagerTaskResponded)
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
func (it *ContractFinalizerTaskManagerTaskRespondedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerTaskManagerTaskRespondedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerTaskManagerTaskResponded represents a TaskResponded event raised by the ContractFinalizerTaskManager contract.
type ContractFinalizerTaskManagerTaskResponded struct {
	TaskResponse         IFinalizerTaskManagerTaskResponse
	TaskResponseMetadata IFinalizerTaskManagerTaskResponseMetadata
	Raw                  types.Log // Blockchain specific contextual infos
}

// FilterTaskResponded is a free log retrieval operation binding the contract event 0xf986623b08031c29efcbf1098118412d9c71ef7d32f63c701988d6bba251ce66.
//
// Solidity: event TaskResponded((uint32,bytes32,bytes32,bytes32,bytes32,bytes32) taskResponse, (uint32,bytes32,uint96[],uint96[]) taskResponseMetadata)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterTaskResponded(opts *bind.FilterOpts) (*ContractFinalizerTaskManagerTaskRespondedIterator, error) {

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "TaskResponded")
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerTaskRespondedIterator{contract: _ContractFinalizerTaskManager.contract, event: "TaskResponded", logs: logs, sub: sub}, nil
}

// WatchTaskResponded is a free log subscription operation binding the contract event 0xf986623b08031c29efcbf1098118412d9c71ef7d32f63c701988d6bba251ce66.
//
// Solidity: event TaskResponded((uint32,bytes32,bytes32,bytes32,bytes32,bytes32) taskResponse, (uint32,bytes32,uint96[],uint96[]) taskResponseMetadata)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchTaskResponded(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerTaskResponded) (event.Subscription, error) {

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "TaskResponded")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerTaskManagerTaskResponded)
				if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "TaskResponded", log); err != nil {
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

// ParseTaskResponded is a log parse operation binding the contract event 0xf986623b08031c29efcbf1098118412d9c71ef7d32f63c701988d6bba251ce66.
//
// Solidity: event TaskResponded((uint32,bytes32,bytes32,bytes32,bytes32,bytes32) taskResponse, (uint32,bytes32,uint96[],uint96[]) taskResponseMetadata)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) ParseTaskResponded(log types.Log) (*ContractFinalizerTaskManagerTaskResponded, error) {
	event := new(ContractFinalizerTaskManagerTaskResponded)
	if err := _ContractFinalizerTaskManager.contract.UnpackLog(event, "TaskResponded", log); err != nil {
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

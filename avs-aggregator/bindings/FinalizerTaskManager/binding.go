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

// IFinalizerTaskManagerTask is an auto generated low-level Go binding around an user-defined struct.
type IFinalizerTaskManagerTask struct {
	TaskNum                                    uint32
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

// IGaspMultiRollupServicePrimitivesNonSignerStakesAndSignatureForOldState is an auto generated low-level Go binding around an user-defined struct.
type IGaspMultiRollupServicePrimitivesNonSignerStakesAndSignatureForOldState struct {
	NonSignerG1PubkeysForOldState []BN254G1Point
	ApkG2ForOldState              BN254G2Point
	SigmaForOldState              BN254G1Point
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
	ABI: "[{\"type\":\"constructor\",\"inputs\":[{\"name\":\"_registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"_taskResponseWindowBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"aggregator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allTaskHashes\",\"inputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allTaskResponses\",\"inputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"blsApkRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIBLSApkRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"checkSignatures\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"referenceBlockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"params\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.QuorumStakeTotals\",\"components\":[{\"name\":\"signedStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"totalStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"createNewTask\",\"inputs\":[{\"name\":\"blockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"delegation\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIDelegationManager\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"dummyForOperatorStateInfoType\",\"inputs\":[{\"name\":\"_operatorStateInfo\",\"type\":\"tuple\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorStateInfo\",\"components\":[{\"name\":\"operatorsStateChanged\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"quorumsRemoved\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsAdded[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"quorumsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsStakeUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]},{\"name\":\"quorumsApkUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsApkUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"operatorsRemoved\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"operatorsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsAdded[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]},{\"name\":\"operatorsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsStakeUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"operatorsQuorumCountUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsQuorumCountUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]}]}],\"outputs\":[],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"generator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getCheckSignaturesIndices\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"referenceBlockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"nonSignerOperatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structOperatorStateRetriever.CheckSignaturesIndices\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getLatestPendingStateHash\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getOperatorState\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple[][]\",\"internalType\":\"structOperatorStateRetriever.Operator[][]\",\"components\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"stake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getOperatorState\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"\",\"type\":\"tuple[][]\",\"internalType\":\"structOperatorStateRetriever.Operator[][]\",\"components\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"stake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getQuorumBitmapsAtBlockNumber\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256[]\",\"internalType\":\"uint256[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getTaskResponseWindowBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"indexToTaskStatus\",\"inputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumFinalizerTaskManager.TaskStatus\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"_pauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"initialOwner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_aggregator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_generator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"lastCompletedTaskCreatedBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastCompletedTaskNum\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastCompletedTaskQuorumNumbers\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"lastCompletedTaskQuorumThresholdPercentage\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestPendingStateHash\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestTaskNum\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"pauseAll\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[{\"name\":\"index\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pauserRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"registryCoordinator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"respondToTask\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.Task\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"blockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.TaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"storageProofHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"pendingStateHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]},{\"name\":\"NonSignerStakesAndSignatureForOldState\",\"type\":\"tuple\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.NonSignerStakesAndSignatureForOldState\",\"components\":[{\"name\":\"nonSignerG1PubkeysForOldState\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2ForOldState\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigmaForOldState\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setPauserRegistry\",\"inputs\":[{\"name\":\"newPauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setStaleStakesForbidden\",\"inputs\":[{\"name\":\"value\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"stakeRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIStakeRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"staleStakesForbidden\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"taskNumber\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"trySignatureAndApkVerification\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"apk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}],\"outputs\":[{\"name\":\"pairingSuccessful\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"siganatureIsValid\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"unpause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"Initialized\",\"inputs\":[{\"name\":\"version\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewTaskCreated\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"task\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.Task\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"blockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Paused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"PauserRegistrySet\",\"inputs\":[{\"name\":\"pauserRegistry\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"newPauserRegistry\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractIPauserRegistry\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"StaleStakesForbiddenUpdate\",\"inputs\":[{\"name\":\"value\",\"type\":\"bool\",\"indexed\":false,\"internalType\":\"bool\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"TaskCompleted\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"blockHash\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"TaskResponded\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.TaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"storageProofHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"pendingStateHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"taskResponseMetadata\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.TaskResponseMetadata\",\"components\":[{\"name\":\"taskResponsedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"hashOfNonSigners\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quroumStakeTotals\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quroumStakeSigned\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Unpaused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false}]",
	Bin: "0x6101206040523480156200001257600080fd5b5060405162005f6f38038062005f6f8339810160408190526200003591620001f7565b81806001600160a01b03166080816001600160a01b031681525050806001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa1580156200008f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000b591906200023e565b6001600160a01b031660a0816001600160a01b031681525050806001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa1580156200010d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200013391906200023e565b6001600160a01b031660c0816001600160a01b03168152505060a0516001600160a01b031663df5cf7236040518163ffffffff1660e01b8152600401602060405180830381865afa1580156200018d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001b391906200023e565b6001600160a01b031660e052506097805460ff1916600117905563ffffffff16610100525062000265565b6001600160a01b0381168114620001f457600080fd5b50565b600080604083850312156200020b57600080fd5b82516200021881620001de565b602084015190925063ffffffff811681146200023357600080fd5b809150509250929050565b6000602082840312156200025157600080fd5b81516200025e81620001de565b9392505050565b60805160a05160c05160e05161010051615c86620002e96000396000818161060c0152612e950152600081816105d5015261245b01526000818161044e015261263e0152600081816104850152818161281401526129d60152600081816104bf01528181610efe01528181612126015281816122be01526124f80152615c866000f3fe608060405234801561001057600080fd5b50600436106102485760003560e01c806366e4a1ca1161013b5780638da5cb5b116100b8578063df5cf7231161007c578063df5cf723146105d0578063f2fde38b146105f7578063f5c9899d1461060a578063f8c8765e14610630578063fabc1cbc1461064357600080fd5b80638da5cb5b1461054e57806399dba0c41461055f578063acfa13541461058f578063b98d0908146105a2578063cefdc1d4146105af57600080fd5b8063715018a6116100ff578063715018a61461050257806372d18e8d1461050a5780637afa1eed14610518578063886f11951461052b5780638b00ce7c1461053e57600080fd5b806366e4a1ca1461047057806368304835146104805780636b92787e146104a75780636d14a987146104ba5780636efb4636146104e157600080fd5b8063416c7e5e116101c9578063595c6a671161018d578063595c6a67146103e65780635ac86ab7146103ee5780635c155662146104215780635c975abb146104415780635df459461461044957600080fd5b8063416c7e5e1461038957806344d6ef391461039c5780634ae6b203146103ac5780634f739f74146103b557806354d127de146103d557600080fd5b80631ce7b2e5116102105780631ce7b2e5146102cb578063245a7bfc146102f75780632cb223d5146103295780632d89f6fc146103495780633563b0d11461036957600080fd5b80630373408d1461024d5780630e8e77c41461026457806310d67a2f14610279578063136439dd1461028e578063171f1d5b146102a1575b600080fd5b60d1545b6040519081526020015b60405180910390f35b61026c610656565b60405161025b9190614587565b61028c6102873660046145b6565b6106e4565b005b61028c61029c3660046145d3565b61079d565b6102b46102af366004614751565b6108dc565b60408051921515835290151560208301520161025b565b60cd546102e290600160201b900463ffffffff1681565b60405163ffffffff909116815260200161025b565b60cf5461031190600160201b90046001600160a01b031681565b6040516001600160a01b03909116815260200161025b565b6102516103373660046147bf565b60cb6020526000908152604090205481565b6102516103573660046147bf565b60ca6020526000908152604090205481565b61037c6103773660046147dc565b610a66565b60405161025b919061493a565b61028c61039736600461495b565b610efc565b60cf546102e29063ffffffff1681565b61025160d15481565b6103c86103c33660046149c0565b611071565b60405161025b9190614ac4565b61028c6103e3366004614b7f565b50565b61028c611797565b6104116103fc366004614bc9565b606654600160ff9092169190911b9081161490565b604051901515815260200161025b565b61043461042f366004614c09565b61185e565b60405161025b9190614cba565b606654610251565b6103117f000000000000000000000000000000000000000000000000000000000000000081565b60cd546102e29063ffffffff1681565b6103117f000000000000000000000000000000000000000000000000000000000000000081565b61028c6104b5366004614cfe565b611a26565b6103117f000000000000000000000000000000000000000000000000000000000000000081565b6104f46104ef366004614fd5565b611d73565b60405161025b929190615095565b61028c612c8b565b60c95463ffffffff166102e2565b60d054610311906001600160a01b031681565b606554610311906001600160a01b031681565b60c9546102e29063ffffffff1681565b6033546001600160a01b0316610311565b61058261056d3660046147bf565b60cc6020526000908152604090205460ff1681565b60405161025b91906150f4565b61028c61059d36600461511c565b612c9f565b6097546104119060ff1681565b6105c26105bd366004615234565b613212565b60405161025b929190615276565b6103117f000000000000000000000000000000000000000000000000000000000000000081565b61028c6106053660046145b6565b6133a4565b7f00000000000000000000000000000000000000000000000000000000000000006102e2565b61028c61063e366004615297565b61341a565b61028c6106513660046145d3565b61357f565b60ce8054610663906152f3565b80601f016020809104026020016040519081016040528092919081815260200182805461068f906152f3565b80156106dc5780601f106106b1576101008083540402835291602001916106dc565b820191906000526020600020905b8154815290600101906020018083116106bf57829003601f168201915b505050505081565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610737573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061075b919061532e565b6001600160a01b0316336001600160a01b0316146107945760405162461bcd60e51b815260040161078b9061534b565b60405180910390fd5b6103e3816136db565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa1580156107e5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108099190615395565b6108255760405162461bcd60e51b815260040161078b906153b2565b6066548181161461089e5760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e70617573653a20696e76616c696420617474656d70742060448201527f746f20756e70617573652066756e6374696f6e616c6974790000000000000000606482015260840161078b565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b60008060007f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f000000187876000015188602001518860000151600060028110610924576109246153fa565b60200201518951600160200201518a60200151600060028110610949576109496153fa565b60200201518b60200151600160028110610965576109656153fa565b602090810291909101518c518d8301516040516109c29a99989796959401988952602089019790975260408801959095526060870193909352608086019190915260a085015260c084015260e08301526101008201526101200190565b6040516020818303038152906040528051906020012060001c6109e59190615410565b9050610a586109fe6109f788846137d2565b8690613869565b610a066138fd565b610a4e610a3f85610a39604080518082018252600080825260209182015281518083019092526001825260029082015290565b906137d2565b610a488c6139bd565b90613869565b886201d4c0613a4d565b909890975095505050505050565b60606000846001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa158015610aa8573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610acc919061532e565b90506000856001600160a01b0316639e9923c26040518163ffffffff1660e01b8152600401602060405180830381865afa158015610b0e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b32919061532e565b90506000866001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa158015610b74573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b98919061532e565b9050600086516001600160401b03811115610bb557610bb56145ec565b604051908082528060200260200182016040528015610be857816020015b6060815260200190600190039081610bd35790505b50905060005b8751811015610ef0576000888281518110610c0b57610c0b6153fa565b0160200151604051638902624560e01b815260f89190911c6004820181905263ffffffff8a16602483015291506000906001600160a01b03871690638902624590604401600060405180830381865afa158015610c6c573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610c949190810190615432565b905080516001600160401b03811115610caf57610caf6145ec565b604051908082528060200260200182016040528015610cfa57816020015b6040805160608101825260008082526020808301829052928201528252600019909201910181610ccd5790505b50848481518110610d0d57610d0d6153fa565b602002602001018190525060005b8151811015610eda576040518060600160405280876001600160a01b03166347b314e8858581518110610d5057610d506153fa565b60200260200101516040518263ffffffff1660e01b8152600401610d7691815260200190565b602060405180830381865afa158015610d93573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610db7919061532e565b6001600160a01b03168152602001838381518110610dd757610dd76153fa565b60200260200101518152602001896001600160a01b031663fa28c627858581518110610e0557610e056153fa565b60209081029190910101516040516001600160e01b031960e084901b168152600481019190915260ff8816602482015263ffffffff8f166044820152606401602060405180830381865afa158015610e61573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610e8591906154c2565b6001600160601b0316815250858581518110610ea357610ea36153fa565b60200260200101518281518110610ebc57610ebc6153fa565b60200260200101819052508080610ed290615501565b915050610d1b565b5050508080610ee890615501565b915050610bee565b50979650505050505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610f5a573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610f7e919061532e565b6001600160a01b0316336001600160a01b03161461102a5760405162461bcd60e51b815260206004820152605c60248201527f424c535369676e6174757265436865636b65722e6f6e6c79436f6f7264696e6160448201527f746f724f776e65723a2063616c6c6572206973206e6f7420746865206f776e6560648201527f72206f6620746865207265676973747279436f6f7264696e61746f7200000000608482015260a40161078b565b6097805460ff19168215159081179091556040519081527f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc9060200160405180910390a150565b61109c6040518060800160405280606081526020016060815260200160608152602001606081525090565b6000876001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa1580156110dc573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611100919061532e565b905061112d6040518060800160405280606081526020016060815260200160608152602001606081525090565b6040516361c8a12f60e11b81526001600160a01b038a169063c391425e9061115d908b908990899060040161551c565b600060405180830381865afa15801561117a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526111a29190810190615566565b81526040516340e03a8160e11b81526001600160a01b038316906381c07502906111d4908b908b908b9060040161561d565b600060405180830381865afa1580156111f1573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526112199190810190615566565b6040820152856001600160401b03811115611236576112366145ec565b60405190808252806020026020018201604052801561126957816020015b60608152602001906001900390816112545790505b50606082015260005b60ff81168711156116a8576000856001600160401b03811115611297576112976145ec565b6040519080825280602002602001820160405280156112c0578160200160208202803683370190505b5083606001518360ff16815181106112da576112da6153fa565b602002602001018190525060005b868110156115a85760008c6001600160a01b03166304ec63518a8a85818110611313576113136153fa565b905060200201358e88600001518681518110611331576113316153fa565b60200260200101516040518463ffffffff1660e01b815260040161136e9392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa15801561138b573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906113af9190615646565b90506001600160c01b0381166114535760405162461bcd60e51b815260206004820152605c60248201527f4f70657261746f7253746174655265747269657665722e676574436865636b5360448201527f69676e617475726573496e64696365733a206f70657261746f72206d7573742060648201527f6265207265676973746572656420617420626c6f636b6e756d62657200000000608482015260a40161078b565b8a8a8560ff16818110611468576114686153fa565b6001600160c01b03841692013560f81c9190911c60019081161415905061159557856001600160a01b031663dd9846b98a8a858181106114aa576114aa6153fa565b905060200201358d8d8860ff168181106114c6576114c66153fa565b6040516001600160e01b031960e087901b1681526004810194909452919091013560f81c60248301525063ffffffff8f166044820152606401602060405180830381865afa15801561151c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611540919061566f565b85606001518560ff1681518110611559576115596153fa565b60200260200101518481518110611572576115726153fa565b63ffffffff909216602092830291909101909101528261159181615501565b9350505b50806115a081615501565b9150506112e8565b506000816001600160401b038111156115c3576115c36145ec565b6040519080825280602002602001820160405280156115ec578160200160208202803683370190505b50905060005b8281101561166d5784606001518460ff1681518110611613576116136153fa565b6020026020010151818151811061162c5761162c6153fa565b6020026020010151828281518110611646576116466153fa565b63ffffffff909216602092830291909101909101528061166581615501565b9150506115f2565b508084606001518460ff1681518110611688576116886153fa565b6020026020010181905250505080806116a09061568c565b915050611272565b506000896001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa1580156116e9573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061170d919061532e565b60405163354952a360e21b81529091506001600160a01b0382169063d5254a8c90611740908b908b908e906004016156ac565b600060405180830381865afa15801561175d573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526117859190810190615566565b60208301525098975050505050505050565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa1580156117df573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906118039190615395565b61181f5760405162461bcd60e51b815260040161078b906153b2565b600019606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b60606000846001600160a01b031663c391425e84866040518363ffffffff1660e01b81526004016118909291906156d6565b600060405180830381865afa1580156118ad573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526118d59190810190615566565b9050600084516001600160401b038111156118f2576118f26145ec565b60405190808252806020026020018201604052801561191b578160200160208202803683370190505b50905060005b8551811015611a1c57866001600160a01b03166304ec635187838151811061194b5761194b6153fa565b602002602001015187868581518110611966576119666153fa565b60200260200101516040518463ffffffff1660e01b81526004016119a39392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa1580156119c0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906119e49190615646565b6001600160c01b03168282815181106119ff576119ff6153fa565b602090810291909101015280611a1481615501565b915050611921565b5095945050505050565b60d0546001600160a01b03163314611a685760405162461bcd60e51b8152602060048201526005602482015264417574683160d81b604482015260640161078b565b60cd5463ffffffff600160201b909104164314801590611a8757504315155b611af95760405162461bcd60e51b815260206004820152603960248201527f43616e2774206372656174652061207461736b20696e207468652073616d652060448201527f626c6f636b206173206120636f6d706c65746564207461736b00000000000000606482015260840161078b565b6040805161010081018252600060608083018290526080830181905260c083015260e082015260c95463ffffffff908116825260208083018890524382168385015290861660a08301528251601f85018290048202810182019093528383529091908490849081908401838280828437600092019190915250505050608082015260cd54600160201b900463ffffffff16606082015260ce8054611b9c906152f3565b80601f0160208091040260200160405190810160405280929190818152602001828054611bc8906152f3565b8015611c155780601f10611bea57610100808354040283529160200191611c15565b820191906000526020600020905b815481529060010190602001808311611bf857829003601f168201915b505050505060c082015260cf5463ffffffff90811660e083015260c9541615611ca75760c954600090611c509060019063ffffffff1661572a565b9050600163ffffffff8216600090815260cc602052604090205460ff166003811115611c7e57611c7e6150de565b1415611ca55763ffffffff8116600090815260cc60205260409020805460ff191660021790555b505b80604051602001611cb8919061574f565b60408051808303601f19018152828252805160209182012060c9805463ffffffff908116600090815260ca85528581209390935581548116835260cc909352929020805460ff19166001179055905416907f840101e40ed68f9364ac588df45a8f81d45874b1c10a496d0a2129be9134639d90611d3690849061574f565b60405180910390a260c954611d529063ffffffff166001615800565b60c9805463ffffffff191663ffffffff929092169190911790555050505050565b6040805180820190915260608082526020820152600084611dea5760405162461bcd60e51b81526020600482015260376024820152600080516020615c3183398151915260448201527f7265733a20656d7074792071756f72756d20696e707574000000000000000000606482015260840161078b565b60408301515185148015611e02575060a08301515185145b8015611e12575060c08301515185145b8015611e22575060e08301515185145b611e8c5760405162461bcd60e51b81526020600482015260416024820152600080516020615c3183398151915260448201527f7265733a20696e7075742071756f72756d206c656e677468206d69736d6174636064820152600d60fb1b608482015260a40161078b565b82515160208401515114611f045760405162461bcd60e51b815260206004820152604460248201819052600080516020615c31833981519152908201527f7265733a20696e707574206e6f6e7369676e6572206c656e677468206d69736d6064820152630c2e8c6d60e31b608482015260a40161078b565b4363ffffffff168463ffffffff1610611f735760405162461bcd60e51b815260206004820152603c6024820152600080516020615c3183398151915260448201527f7265733a20696e76616c6964207265666572656e636520626c6f636b00000000606482015260840161078b565b6040805180820182526000808252602080830191909152825180840190935260608084529083015290866001600160401b03811115611fb457611fb46145ec565b604051908082528060200260200182016040528015611fdd578160200160208202803683370190505b506020820152866001600160401b03811115611ffb57611ffb6145ec565b604051908082528060200260200182016040528015612024578160200160208202803683370190505b50815260408051808201909152606080825260208201528560200151516001600160401b03811115612058576120586145ec565b604051908082528060200260200182016040528015612081578160200160208202803683370190505b5081526020860151516001600160401b038111156120a1576120a16145ec565b6040519080825280602002602001820160405280156120ca578160200160208202803683370190505b508160200181905250600061219c8a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152505060408051639aa1653d60e01b815290516001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169350639aa1653d925060048083019260209291908290030181865afa158015612173573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906121979190615828565b613c71565b905060005b876020015151811015612437576121e6886020015182815181106121c7576121c76153fa565b6020026020010151805160009081526020918201519091526040902090565b836020015182815181106121fc576121fc6153fa565b602090810291909101015280156122bc57602083015161221d600183615845565b8151811061222d5761222d6153fa565b602002602001015160001c8360200151828151811061224e5761224e6153fa565b602002602001015160001c116122bc576040805162461bcd60e51b8152602060048201526024810191909152600080516020615c3183398151915260448201527f7265733a206e6f6e5369676e65725075626b657973206e6f7420736f72746564606482015260840161078b565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166304ec635184602001518381518110612301576123016153fa565b60200260200101518b8b600001518581518110612320576123206153fa565b60200260200101516040518463ffffffff1660e01b815260040161235d9392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa15801561237a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061239e9190615646565b6001600160c01b0316836000015182815181106123bd576123bd6153fa565b6020026020010181815250506124236109f76123f784866000015185815181106123e9576123e96153fa565b602002602001015116613d04565b8a60200151848151811061240d5761240d6153fa565b6020026020010151613d2f90919063ffffffff16565b94508061242f81615501565b9150506121a1565b505061244283613e13565b60975490935060ff166000816124595760006124db565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663c448feb86040518163ffffffff1660e01b8152600401602060405180830381865afa1580156124b7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906124db919061585c565b905060005b8a811015612b5a57821561263c578963ffffffff16827f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663249a0c428f8f86818110612537576125376153fa565b60405160e085901b6001600160e01b031916815292013560f81c600483015250602401602060405180830381865afa158015612577573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061259b919061585c565b6125a59190615875565b101561263c5760405162461bcd60e51b81526020600482015260666024820152600080516020615c3183398151915260448201527f7265733a205374616b6552656769737472792075706461746573206d7573742060648201527f62652077697468696e207769746864726177616c44656c6179426c6f636b732060848201526577696e646f7760d01b60a482015260c40161078b565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166368bccaac8d8d8481811061267d5761267d6153fa565b9050013560f81c60f81b60f81c8c8c60a0015185815181106126a1576126a16153fa565b60209081029190910101516040516001600160e01b031960e086901b16815260ff909316600484015263ffffffff9182166024840152166044820152606401602060405180830381865afa1580156126fd573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612721919061588d565b6001600160401b0319166127448a6040015183815181106121c7576121c76153fa565b67ffffffffffffffff1916146127e05760405162461bcd60e51b81526020600482015260616024820152600080516020615c3183398151915260448201527f7265733a2071756f72756d41706b206861736820696e2073746f72616765206460648201527f6f6573206e6f74206d617463682070726f76696465642071756f72756d2061706084820152606b60f81b60a482015260c40161078b565b612810896040015182815181106127f9576127f96153fa565b60200260200101518761386990919063ffffffff16565b95507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663c8294c568d8d84818110612853576128536153fa565b9050013560f81c60f81b60f81c8c8c60c001518581518110612877576128776153fa565b60209081029190910101516040516001600160e01b031960e086901b16815260ff909316600484015263ffffffff9182166024840152166044820152606401602060405180830381865afa1580156128d3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906128f791906154c2565b8560200151828151811061290d5761290d6153fa565b6001600160601b03909216602092830291909101820152850151805182908110612939576129396153fa565b602002602001015185600001518281518110612957576129576153fa565b60200260200101906001600160601b031690816001600160601b0316815250506000805b8a6020015151811015612b45576129cf866000015182815181106129a1576129a16153fa565b60200260200101518f8f868181106129bb576129bb6153fa565b600192013560f81c9290921c811614919050565b15612b33577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f2be94ae8f8f86818110612a1557612a156153fa565b9050013560f81c60f81b60f81c8e89602001518581518110612a3957612a396153fa565b60200260200101518f60e001518881518110612a5757612a576153fa565b60200260200101518781518110612a7057612a706153fa565b60209081029190910101516040516001600160e01b031960e087901b16815260ff909416600485015263ffffffff92831660248501526044840191909152166064820152608401602060405180830381865afa158015612ad4573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612af891906154c2565b8751805185908110612b0c57612b0c6153fa565b60200260200101818151612b2091906158b8565b6001600160601b03169052506001909101905b80612b3d81615501565b91505061297b565b50508080612b5290615501565b9150506124e0565b505050600080612b748c868a606001518b608001516108dc565b9150915081612be55760405162461bcd60e51b81526020600482015260436024820152600080516020615c3183398151915260448201527f7265733a2070616972696e6720707265636f6d70696c652063616c6c206661696064820152621b195960ea1b608482015260a40161078b565b80612c465760405162461bcd60e51b81526020600482015260396024820152600080516020615c3183398151915260448201527f7265733a207369676e617475726520697320696e76616c696400000000000000606482015260840161078b565b50506000878260200151604051602001612c619291906158d8565b60408051808303601f190181529190528051602090910120929b929a509198505050505050505050565b612c93613eae565b612c9d6000613f08565b565b60cf54600160201b90046001600160a01b03163314612ce85760405162461bcd60e51b8152602060048201526005602482015264041757468360dc1b604482015260640161078b565b6000612cfa60608601604087016147bf565b9050366000612d0c6080880188615920565b90925090506000612d2360c0890160a08a016147bf565b905060ca6000612d3660208a018a6147bf565b63ffffffff1663ffffffff1681526020019081526020016000205488604051602001612d6291906159ab565b6040516020818303038152906040528051906020012014612deb5760405162461bcd60e51b815260206004820152603d60248201527f737570706c696564207461736b20646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e7472616374000000606482015260840161078b565b600160cc6000612dfe60208b018b6147bf565b63ffffffff16815260208101919091526040016000205460ff166003811115612e2957612e296150de565b14612e465760405162461bcd60e51b815260040161078b90615a8d565b600060cb81612e5860208b018b6147bf565b63ffffffff1663ffffffff1681526020019081526020016000205414612e905760405162461bcd60e51b815260040161078b90615a8d565b612eba7f000000000000000000000000000000000000000000000000000000000000000085615800565b63ffffffff164363ffffffff161115612f2b5760405162461bcd60e51b815260206004820152602d60248201527f41676772656761746f722068617320726573706f6e64656420746f207468652060448201526c7461736b20746f6f206c61746560981b606482015260840161078b565b600087604051602001612f3e9190615b24565b604051602081830303815290604052805190602001209050600080612f668387878a8d611d73565b6040805160808101825263ffffffff43168152602080820184905280850151828401528451606083015291519395509193509091612fa8918d91849101615b32565b6040516020818303038152906040528051906020012060cb60008d6000016020810190612fd591906147bf565b63ffffffff16815260208082019290925260400160002091909155612ffc908d018d6147bf565b63ffffffff167f82573b6e06a270f942a7da9b7d171ae407fc5035f31cdfd878fca0cb9ab67aab8c83604051613033929190615b32565b60405180910390a260005b868110156130d4578560ff1684602001518281518110613060576130606153fa565b60200260200101516130729190615ba0565b6001600160601b0316606485600001518381518110613093576130936153fa565b60200260200101516001600160601b03166130ae9190615bcf565b10156130c25750505050505050505061320c565b806130cc81615501565b91505061303e565b5060a08b013560d155600360cc60006130f060208f018f6147bf565b63ffffffff16815260208101919091526040016000205460ff16600381111561311b5761311b6150de565b5061312e905060608d0160408e016147bf565b60cd805463ffffffff92909216600160201b0267ffffffff000000001990921691909117905561316160808d018d615920565b61316d9160ce916143c7565b5061317e60c08d0160a08e016147bf565b60cf805463ffffffff191663ffffffff929092169190911790556131a560208d018d6147bf565b60cd805463ffffffff191663ffffffff9290921691909117905560608b01356131d160208d018d6147bf565b63ffffffff167f8378be8a33cf3a493910a16e275cd96af4f048c5eb1a2c2962d4066e697fea8060405160405180910390a350505050505050505b50505050565b604080516001808252818301909252600091606091839160208083019080368337019050509050848160008151811061324d5761324d6153fa565b60209081029190910101526040516361c8a12f60e11b81526000906001600160a01b0388169063c391425e9061328990889086906004016156d6565b600060405180830381865afa1580156132a6573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526132ce9190810190615566565b6000815181106132e0576132e06153fa565b60209081029190910101516040516304ec635160e01b81526004810188905263ffffffff87811660248301529091166044820181905291506000906001600160a01b038916906304ec635190606401602060405180830381865afa15801561334c573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906133709190615646565b6001600160c01b03169050600061338682613f5a565b9050816133948a838a610a66565b9550955050505050935093915050565b6133ac613eae565b6001600160a01b0381166134115760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b606482015260840161078b565b6103e381613f08565b600054610100900460ff161580801561343a5750600054600160ff909116105b806134545750303b158015613454575060005460ff166001145b6134b75760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b606482015260840161078b565b6000805460ff1916600117905580156134da576000805461ff0019166101001790555b6134e5856000614026565b6134ee84613f08565b60cf8054640100000000600160c01b031916600160201b6001600160a01b03868116919091029190911790915560d080546001600160a01b0319169184169190911790558015613578576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b5050505050565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156135d2573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906135f6919061532e565b6001600160a01b0316336001600160a01b0316146136265760405162461bcd60e51b815260040161078b9061534b565b6066541981196066541916146136a45760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e756e70617573653a20696e76616c696420617474656d7060448201527f7420746f2070617573652066756e6374696f6e616c6974790000000000000000606482015260840161078b565b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c906020016108d1565b6001600160a01b0381166137695760405162461bcd60e51b815260206004820152604960248201527f5061757361626c652e5f73657450617573657252656769737472793a206e657760448201527f50617573657252656769737472792063616e6e6f7420626520746865207a65726064820152686f206164647265737360b81b608482015260a40161078b565b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b60408051808201909152600080825260208201526137ee61444b565b835181526020808501519082015260408082018490526000908360608460076107d05a03fa905080801561382157613823565bfe5b50806138615760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b604482015260640161078b565b505092915050565b6040805180820190915260008082526020820152613885614469565b835181526020808501518183015283516040808401919091529084015160608301526000908360808460066107d05a03fa90508080156138215750806138615760405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b604482015260640161078b565b613905614487565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b6040805180820190915260008082526020820152600080806139ed600080516020615c1183398151915286615410565b90505b6139f981614110565b9093509150600080516020615c11833981519152828309831415613a33576040805180820190915290815260208101919091529392505050565b600080516020615c118339815191526001820890506139f0565b604080518082018252868152602080820186905282518084019093528683528201849052600091829190613a7f6144ac565b60005b6002811015613c44576000613a98826006615bcf565b9050848260028110613aac57613aac6153fa565b60200201515183613abe836000615875565b600c8110613ace57613ace6153fa565b6020020152848260028110613ae557613ae56153fa565b60200201516020015183826001613afc9190615875565b600c8110613b0c57613b0c6153fa565b6020020152838260028110613b2357613b236153fa565b6020020151515183613b36836002615875565b600c8110613b4657613b466153fa565b6020020152838260028110613b5d57613b5d6153fa565b6020020151516001602002015183613b76836003615875565b600c8110613b8657613b866153fa565b6020020152838260028110613b9d57613b9d6153fa565b602002015160200151600060028110613bb857613bb86153fa565b602002015183613bc9836004615875565b600c8110613bd957613bd96153fa565b6020020152838260028110613bf057613bf06153fa565b602002015160200151600160028110613c0b57613c0b6153fa565b602002015183613c1c836005615875565b600c8110613c2c57613c2c6153fa565b60200201525080613c3c81615501565b915050613a82565b50613c4d6144cb565b60006020826101808560088cfa9151919c9115159b50909950505050505050505050565b600080613c7d84614192565b9050808360ff166001901b11613cfb5760405162461bcd60e51b815260206004820152603f60248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206269746d61702065786365656473206d61782076616c756500606482015260840161078b565b90505b92915050565b6000805b8215613cfe57613d19600184615845565b9092169180613d2781615bee565b915050613d08565b60408051808201909152600080825260208201526102008261ffff1610613d8b5760405162461bcd60e51b815260206004820152601060248201526f7363616c61722d746f6f2d6c6172676560801b604482015260640161078b565b8161ffff1660011415613d9f575081613cfe565b6040805180820190915260008082526020820181905284906001905b8161ffff168661ffff1610613e0857600161ffff871660ff83161c81161415613deb57613de88484613869565b93505b613df58384613869565b92506201fffe600192831b169101613dbb565b509195945050505050565b60408051808201909152600080825260208201528151158015613e3857506020820151155b15613e56575050604080518082019091526000808252602082015290565b604051806040016040528083600001518152602001600080516020615c118339815191528460200151613e899190615410565b613ea190600080516020615c11833981519152615845565b905292915050565b919050565b6033546001600160a01b03163314612c9d5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604482015260640161078b565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6060600080613f6884613d04565b61ffff166001600160401b03811115613f8357613f836145ec565b6040519080825280601f01601f191660200182016040528015613fad576020820181803683370190505b5090506000805b825182108015613fc5575061010081105b1561401c576001811b93508584161561400c578060f81b838381518110613fee57613fee6153fa565b60200101906001600160f81b031916908160001a9053508160010191505b61401581615501565b9050613fb4565b5090949350505050565b6065546001600160a01b031615801561404757506001600160a01b03821615155b6140c95760405162461bcd60e51b815260206004820152604760248201527f5061757361626c652e5f696e697469616c697a655061757365723a205f696e6960448201527f7469616c697a6550617573657228292063616e206f6e6c792062652063616c6c6064820152666564206f6e636560c81b608482015260a40161078b565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a261410c826136db565b5050565b60008080600080516020615c118339815191526003600080516020615c1183398151915286600080516020615c11833981519152888909090890506000614186827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f52600080516020615c1183398151915261431f565b91959194509092505050565b60006101008251111561421b5760405162461bcd60e51b8152602060048201526044602482018190527f4269746d61705574696c732e6f72646572656442797465734172726179546f42908201527f69746d61703a206f7264657265644279746573417272617920697320746f6f206064820152636c6f6e6760e01b608482015260a40161078b565b815161422957506000919050565b6000808360008151811061423f5761423f6153fa565b0160200151600160f89190911c81901b92505b84518110156143165784818151811061426d5761426d6153fa565b0160200151600160f89190911c1b91508282116143025760405162461bcd60e51b815260206004820152604760248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f72646572656442797465734172726179206973206e6f74206064820152661bdc99195c995960ca1b608482015260a40161078b565b9181179161430f81615501565b9050614252565b50909392505050565b60008061432a6144cb565b6143326144e9565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa92508280156138215750826143bc5760405162461bcd60e51b815260206004820152601a60248201527f424e3235342e6578704d6f643a2063616c6c206661696c757265000000000000604482015260640161078b565b505195945050505050565b8280546143d3906152f3565b90600052602060002090601f0160209004810192826143f5576000855561443b565b82601f1061440e5782800160ff1982351617855561443b565b8280016001018555821561443b579182015b8281111561443b578235825591602001919060010190614420565b50614447929150614507565b5090565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b604051806040016040528061449a61451c565b81526020016144a761451c565b905290565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b5b808211156144475760008155600101614508565b60405180604001604052806002906020820280368337509192915050565b6000815180845260005b8181101561456057602081850181015186830182015201614544565b81811115614572576000602083870101525b50601f01601f19169290920160200192915050565b60208152600061459a602083018461453a565b9392505050565b6001600160a01b03811681146103e357600080fd5b6000602082840312156145c857600080fd5b8135613cfb816145a1565b6000602082840312156145e557600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b0381118282101715614624576146246145ec565b60405290565b60405161010081016001600160401b0381118282101715614624576146246145ec565b604051601f8201601f191681016001600160401b0381118282101715614675576146756145ec565b604052919050565b60006040828403121561468f57600080fd5b614697614602565b9050813581526020820135602082015292915050565b600082601f8301126146be57600080fd5b604051604081018181106001600160401b03821117156146e0576146e06145ec565b80604052508060408401858111156146f757600080fd5b845b81811015613e085780358352602092830192016146f9565b60006080828403121561472357600080fd5b61472b614602565b905061473783836146ad565b815261474683604084016146ad565b602082015292915050565b600080600080610120858703121561476857600080fd5b84359350614779866020870161467d565b92506147888660608701614711565b91506147978660e0870161467d565b905092959194509250565b63ffffffff811681146103e357600080fd5b8035613ea9816147a2565b6000602082840312156147d157600080fd5b8135613cfb816147a2565b6000806000606084860312156147f157600080fd5b83356147fc816145a1565b92506020848101356001600160401b038082111561481957600080fd5b818701915087601f83011261482d57600080fd5b81358181111561483f5761483f6145ec565b614851601f8201601f1916850161464d565b9150808252888482850101111561486757600080fd5b808484018584013760008482840101525080945050505061488a604085016147b4565b90509250925092565b600082825180855260208086019550808260051b8401018186016000805b8581101561492c57868403601f19018a52825180518086529086019086860190845b8181101561491757835180516001600160a01b03168452898101518a8501526040908101516001600160601b031690840152928801926060909201916001016148d3565b50509a86019a945050918401916001016148b1565b509198975050505050505050565b60208152600061459a6020830184614893565b80151581146103e357600080fd5b60006020828403121561496d57600080fd5b8135613cfb8161494d565b60008083601f84011261498a57600080fd5b5081356001600160401b038111156149a157600080fd5b6020830191508360208285010111156149b957600080fd5b9250929050565b600080600080600080608087890312156149d957600080fd5b86356149e4816145a1565b955060208701356149f4816147a2565b945060408701356001600160401b0380821115614a1057600080fd5b614a1c8a838b01614978565b90965094506060890135915080821115614a3557600080fd5b818901915089601f830112614a4957600080fd5b813581811115614a5857600080fd5b8a60208260051b8501011115614a6d57600080fd5b6020830194508093505050509295509295509295565b600081518084526020808501945080840160005b83811015614ab957815163ffffffff1687529582019590820190600101614a97565b509495945050505050565b600060208083528351608082850152614ae060a0850182614a83565b905081850151601f1980868403016040870152614afd8383614a83565b92506040870151915080868403016060870152614b1a8383614a83565b60608801518782038301608089015280518083529194508501925084840190600581901b8501860160005b82811015614b715784878303018452614b5f828751614a83565b95880195938801939150600101614b45565b509998505050505050505050565b600060208284031215614b9157600080fd5b81356001600160401b03811115614ba757600080fd5b82016101208185031215613cfb57600080fd5b60ff811681146103e357600080fd5b600060208284031215614bdb57600080fd5b8135613cfb81614bba565b60006001600160401b03821115614bff57614bff6145ec565b5060051b60200190565b600080600060608486031215614c1e57600080fd5b8335614c29816145a1565b92506020848101356001600160401b03811115614c4557600080fd5b8501601f81018713614c5657600080fd5b8035614c69614c6482614be6565b61464d565b81815260059190911b82018301908381019089831115614c8857600080fd5b928401925b82841015614ca657833582529284019290840190614c8d565b809650505050505061488a604085016147b4565b6020808252825182820181905260009190848201906040850190845b81811015614cf257835183529284019291840191600101614cd6565b50909695505050505050565b60008060008060608587031215614d1457600080fd5b843593506020850135614d26816147a2565b925060408501356001600160401b03811115614d4157600080fd5b614d4d87828801614978565b95989497509550505050565b600082601f830112614d6a57600080fd5b81356020614d7a614c6483614be6565b82815260059290921b84018101918181019086841115614d9957600080fd5b8286015b84811015614dbd578035614db0816147a2565b8352918301918301614d9d565b509695505050505050565b600082601f830112614dd957600080fd5b81356020614de9614c6483614be6565b82815260069290921b84018101918181019086841115614e0857600080fd5b8286015b84811015614dbd57614e1e888261467d565b835291830191604001614e0c565b600082601f830112614e3d57600080fd5b81356020614e4d614c6483614be6565b82815260059290921b84018101918181019086841115614e6c57600080fd5b8286015b84811015614dbd5780356001600160401b03811115614e8f5760008081fd5b614e9d8986838b0101614d59565b845250918301918301614e70565b60006101808284031215614ebe57600080fd5b614ec661462a565b905081356001600160401b0380821115614edf57600080fd5b614eeb85838601614d59565b83526020840135915080821115614f0157600080fd5b614f0d85838601614dc8565b60208401526040840135915080821115614f2657600080fd5b614f3285838601614dc8565b6040840152614f448560608601614711565b6060840152614f568560e0860161467d565b6080840152610120840135915080821115614f7057600080fd5b614f7c85838601614d59565b60a0840152610140840135915080821115614f9657600080fd5b614fa285838601614d59565b60c0840152610160840135915080821115614fbc57600080fd5b50614fc984828501614e2c565b60e08301525092915050565b600080600080600060808688031215614fed57600080fd5b8535945060208601356001600160401b038082111561500b57600080fd5b61501789838a01614978565b90965094506040880135915061502c826147a2565b9092506060870135908082111561504257600080fd5b5061504f88828901614eab565b9150509295509295909350565b600081518084526020808501945080840160005b83811015614ab95781516001600160601b031687529582019590820190600101615070565b60408152600083516040808401526150b0608084018261505c565b90506020850151603f198483030160608501526150cd828261505c565b925050508260208301529392505050565b634e487b7160e01b600052602160045260246000fd5b602081016004831061511657634e487b7160e01b600052602160045260246000fd5b91905290565b60008060008084860361012081121561513457600080fd5b85356001600160401b038082111561514b57600080fd5b818801915061010080838b03121561516257600080fd5b82975060c0601f198501121561517757600080fd5b60208901965060e089013593508184111561519157600080fd5b61519d8a858b01614eab565b95508801359250808311156151b157600080fd5b9187019160e0838a0312156151c557600080fd5b60405191506060820182811082821117156151e2576151e26145ec565b6040528235818111156151f457600080fd5b6152008a828601614dc8565b835250506152118860208401614711565b60208201526152238860a0840161467d565b604082015294979396509194505050565b60008060006060848603121561524957600080fd5b8335615254816145a1565b925060208401359150604084013561526b816147a2565b809150509250925092565b82815260406020820152600061528f6040830184614893565b949350505050565b600080600080608085870312156152ad57600080fd5b84356152b8816145a1565b935060208501356152c8816145a1565b925060408501356152d8816145a1565b915060608501356152e8816145a1565b939692955090935050565b600181811c9082168061530757607f821691505b6020821081141561532857634e487b7160e01b600052602260045260246000fd5b50919050565b60006020828403121561534057600080fd5b8151613cfb816145a1565b6020808252602a908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526939903ab73830bab9b2b960b11b606082015260800190565b6000602082840312156153a757600080fd5b8151613cfb8161494d565b60208082526028908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526739903830bab9b2b960c11b606082015260800190565b634e487b7160e01b600052603260045260246000fd5b60008261542d57634e487b7160e01b600052601260045260246000fd5b500690565b6000602080838503121561544557600080fd5b82516001600160401b0381111561545b57600080fd5b8301601f8101851361546c57600080fd5b805161547a614c6482614be6565b81815260059190911b8201830190838101908783111561549957600080fd5b928401925b828410156154b75783518252928401929084019061549e565b979650505050505050565b6000602082840312156154d457600080fd5b81516001600160601b0381168114613cfb57600080fd5b634e487b7160e01b600052601160045260246000fd5b6000600019821415615515576155156154eb565b5060010190565b63ffffffff84168152604060208201819052810182905260006001600160fb1b0383111561554957600080fd5b8260051b8085606085013760009201606001918252509392505050565b6000602080838503121561557957600080fd5b82516001600160401b0381111561558f57600080fd5b8301601f810185136155a057600080fd5b80516155ae614c6482614be6565b81815260059190911b820183019083810190878311156155cd57600080fd5b928401925b828410156154b75783516155e5816147a2565b825292840192908401906155d2565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b63ffffffff8416815260406020820152600061563d6040830184866155f4565b95945050505050565b60006020828403121561565857600080fd5b81516001600160c01b0381168114613cfb57600080fd5b60006020828403121561568157600080fd5b8151613cfb816147a2565b600060ff821660ff8114156156a3576156a36154eb565b60010192915050565b6040815260006156c06040830185876155f4565b905063ffffffff83166020830152949350505050565b60006040820163ffffffff851683526020604081850152818551808452606086019150828701935060005b8181101561571d57845183529383019391830191600101615701565b5090979650505050505050565b600063ffffffff83811690831681811015615747576157476154eb565b039392505050565b6020815263ffffffff82511660208201526020820151604082015260006040830151615783606084018263ffffffff169052565b50606083015163ffffffff811660808401525060808301516101008060a08501526157b261012085018361453a565b915060a08501516157cb60c086018263ffffffff169052565b5060c0850151848303601f190160e08601526157e7838261453a565b92505060e085015161401c8286018263ffffffff169052565b600063ffffffff80831681851680830382111561581f5761581f6154eb565b01949350505050565b60006020828403121561583a57600080fd5b8151613cfb81614bba565b600082821015615857576158576154eb565b500390565b60006020828403121561586e57600080fd5b5051919050565b60008219821115615888576158886154eb565b500190565b60006020828403121561589f57600080fd5b815167ffffffffffffffff1981168114613cfb57600080fd5b60006001600160601b0383811690831681811015615747576157476154eb565b63ffffffff60e01b8360e01b1681526000600482018351602080860160005b83811015615913578151855293820193908201906001016158f7565b5092979650505050505050565b6000808335601e1984360301811261593757600080fd5b8301803591506001600160401b0382111561595157600080fd5b6020019150368190038213156149b957600080fd5b6000808335601e1984360301811261597d57600080fd5b83016020810192503590506001600160401b0381111561599c57600080fd5b8036038313156149b957600080fd5b60208152600082356159bc816147a2565b63ffffffff8116602084015250602083013560408301526159df604084016147b4565b63ffffffff81166060840152506159f8606084016147b4565b63ffffffff8116608084015250615a126080840184615966565b6101008060a0860152615a2a610120860183856155f4565b9250615a3860a087016147b4565b63ffffffff811660c08701529150615a5360c0870187615966565b868503601f190160e08801529250615a6c8484836155f4565b935050615a7b60e087016147b4565b63ffffffff811686830152915061401c565b6020808252602c908201527f41676772656761746f722068617320616c726561647920726573706f6e64656460408201526b20746f20746865207461736b60a01b606082015260800190565b8035615ae4816147a2565b63ffffffff81168352506020810135602083015260408101356040830152606081013560608301526080810135608083015260a081013560a08301525050565b60c08101613cfe8284615ad9565b615b3c8184615ad9565b60e060c082015263ffffffff82511660e08201526020820151610100820152600060408301516080610120840152615b7861016084018261505c565b9050606084015160df1984830301610140850152615b96828261505c565b9695505050505050565b60006001600160601b0380831681851681830481118215151615615bc657615bc66154eb565b02949350505050565b6000816000190483118215151615615be957615be96154eb565b500290565b600061ffff80831681811415615c0657615c066154eb565b600101939250505056fe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47424c535369676e6174757265436865636b65722e636865636b5369676e617475a26469706673582212200411967ef3832f79ac4fe9caae9e900def6bd1b4df41b3a05bd6122f0857913564736f6c634300080c0033",
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

// CheckSignatures is a free data retrieval call binding the contract method 0x6efb4636.
//
// Solidity: function checkSignatures(bytes32 msgHash, bytes quorumNumbers, uint32 referenceBlockNumber, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) params) view returns((uint96[],uint96[]), bytes32)
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

// CheckSignatures is a free data retrieval call binding the contract method 0x6efb4636.
//
// Solidity: function checkSignatures(bytes32 msgHash, bytes quorumNumbers, uint32 referenceBlockNumber, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) params) view returns((uint96[],uint96[]), bytes32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) CheckSignatures(msgHash [32]byte, quorumNumbers []byte, referenceBlockNumber uint32, params IBLSSignatureCheckerNonSignerStakesAndSignature) (IBLSSignatureCheckerQuorumStakeTotals, [32]byte, error) {
	return _ContractFinalizerTaskManager.Contract.CheckSignatures(&_ContractFinalizerTaskManager.CallOpts, msgHash, quorumNumbers, referenceBlockNumber, params)
}

// CheckSignatures is a free data retrieval call binding the contract method 0x6efb4636.
//
// Solidity: function checkSignatures(bytes32 msgHash, bytes quorumNumbers, uint32 referenceBlockNumber, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) params) view returns((uint96[],uint96[]), bytes32)
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

// LastCompletedTaskCreatedBlock is a free data retrieval call binding the contract method 0x1ce7b2e5.
//
// Solidity: function lastCompletedTaskCreatedBlock() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) LastCompletedTaskCreatedBlock(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "lastCompletedTaskCreatedBlock")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// LastCompletedTaskCreatedBlock is a free data retrieval call binding the contract method 0x1ce7b2e5.
//
// Solidity: function lastCompletedTaskCreatedBlock() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) LastCompletedTaskCreatedBlock() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.LastCompletedTaskCreatedBlock(&_ContractFinalizerTaskManager.CallOpts)
}

// LastCompletedTaskCreatedBlock is a free data retrieval call binding the contract method 0x1ce7b2e5.
//
// Solidity: function lastCompletedTaskCreatedBlock() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) LastCompletedTaskCreatedBlock() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.LastCompletedTaskCreatedBlock(&_ContractFinalizerTaskManager.CallOpts)
}

// LastCompletedTaskNum is a free data retrieval call binding the contract method 0x66e4a1ca.
//
// Solidity: function lastCompletedTaskNum() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) LastCompletedTaskNum(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "lastCompletedTaskNum")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// LastCompletedTaskNum is a free data retrieval call binding the contract method 0x66e4a1ca.
//
// Solidity: function lastCompletedTaskNum() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) LastCompletedTaskNum() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.LastCompletedTaskNum(&_ContractFinalizerTaskManager.CallOpts)
}

// LastCompletedTaskNum is a free data retrieval call binding the contract method 0x66e4a1ca.
//
// Solidity: function lastCompletedTaskNum() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) LastCompletedTaskNum() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.LastCompletedTaskNum(&_ContractFinalizerTaskManager.CallOpts)
}

// LastCompletedTaskQuorumNumbers is a free data retrieval call binding the contract method 0x0e8e77c4.
//
// Solidity: function lastCompletedTaskQuorumNumbers() view returns(bytes)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) LastCompletedTaskQuorumNumbers(opts *bind.CallOpts) ([]byte, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "lastCompletedTaskQuorumNumbers")

	if err != nil {
		return *new([]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([]byte)).(*[]byte)

	return out0, err

}

// LastCompletedTaskQuorumNumbers is a free data retrieval call binding the contract method 0x0e8e77c4.
//
// Solidity: function lastCompletedTaskQuorumNumbers() view returns(bytes)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) LastCompletedTaskQuorumNumbers() ([]byte, error) {
	return _ContractFinalizerTaskManager.Contract.LastCompletedTaskQuorumNumbers(&_ContractFinalizerTaskManager.CallOpts)
}

// LastCompletedTaskQuorumNumbers is a free data retrieval call binding the contract method 0x0e8e77c4.
//
// Solidity: function lastCompletedTaskQuorumNumbers() view returns(bytes)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) LastCompletedTaskQuorumNumbers() ([]byte, error) {
	return _ContractFinalizerTaskManager.Contract.LastCompletedTaskQuorumNumbers(&_ContractFinalizerTaskManager.CallOpts)
}

// LastCompletedTaskQuorumThresholdPercentage is a free data retrieval call binding the contract method 0x44d6ef39.
//
// Solidity: function lastCompletedTaskQuorumThresholdPercentage() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCaller) LastCompletedTaskQuorumThresholdPercentage(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractFinalizerTaskManager.contract.Call(opts, &out, "lastCompletedTaskQuorumThresholdPercentage")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// LastCompletedTaskQuorumThresholdPercentage is a free data retrieval call binding the contract method 0x44d6ef39.
//
// Solidity: function lastCompletedTaskQuorumThresholdPercentage() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) LastCompletedTaskQuorumThresholdPercentage() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.LastCompletedTaskQuorumThresholdPercentage(&_ContractFinalizerTaskManager.CallOpts)
}

// LastCompletedTaskQuorumThresholdPercentage is a free data retrieval call binding the contract method 0x44d6ef39.
//
// Solidity: function lastCompletedTaskQuorumThresholdPercentage() view returns(uint32)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerCallerSession) LastCompletedTaskQuorumThresholdPercentage() (uint32, error) {
	return _ContractFinalizerTaskManager.Contract.LastCompletedTaskQuorumThresholdPercentage(&_ContractFinalizerTaskManager.CallOpts)
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

// RespondToTask is a paid mutator transaction binding the contract method 0xacfa1354.
//
// Solidity: function respondToTask((uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32,bytes32,bytes32,bytes32) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature, ((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)) NonSignerStakesAndSignatureForOldState) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactor) RespondToTask(opts *bind.TransactOpts, task IFinalizerTaskManagerTask, taskResponse IFinalizerTaskManagerTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature, NonSignerStakesAndSignatureForOldState IGaspMultiRollupServicePrimitivesNonSignerStakesAndSignatureForOldState) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.contract.Transact(opts, "respondToTask", task, taskResponse, nonSignerStakesAndSignature, NonSignerStakesAndSignatureForOldState)
}

// RespondToTask is a paid mutator transaction binding the contract method 0xacfa1354.
//
// Solidity: function respondToTask((uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32,bytes32,bytes32,bytes32) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature, ((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)) NonSignerStakesAndSignatureForOldState) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerSession) RespondToTask(task IFinalizerTaskManagerTask, taskResponse IFinalizerTaskManagerTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature, NonSignerStakesAndSignatureForOldState IGaspMultiRollupServicePrimitivesNonSignerStakesAndSignatureForOldState) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.RespondToTask(&_ContractFinalizerTaskManager.TransactOpts, task, taskResponse, nonSignerStakesAndSignature, NonSignerStakesAndSignatureForOldState)
}

// RespondToTask is a paid mutator transaction binding the contract method 0xacfa1354.
//
// Solidity: function respondToTask((uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32,bytes32,bytes32,bytes32) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature, ((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)) NonSignerStakesAndSignatureForOldState) returns()
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerTransactorSession) RespondToTask(task IFinalizerTaskManagerTask, taskResponse IFinalizerTaskManagerTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature, NonSignerStakesAndSignatureForOldState IGaspMultiRollupServicePrimitivesNonSignerStakesAndSignatureForOldState) (*types.Transaction, error) {
	return _ContractFinalizerTaskManager.Contract.RespondToTask(&_ContractFinalizerTaskManager.TransactOpts, task, taskResponse, nonSignerStakesAndSignature, NonSignerStakesAndSignatureForOldState)
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

// FilterNewTaskCreated is a free log retrieval operation binding the contract event 0x840101e40ed68f9364ac588df45a8f81d45874b1c10a496d0a2129be9134639d.
//
// Solidity: event NewTaskCreated(uint32 indexed taskIndex, (uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32) task)
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

// WatchNewTaskCreated is a free log subscription operation binding the contract event 0x840101e40ed68f9364ac588df45a8f81d45874b1c10a496d0a2129be9134639d.
//
// Solidity: event NewTaskCreated(uint32 indexed taskIndex, (uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32) task)
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

// ParseNewTaskCreated is a log parse operation binding the contract event 0x840101e40ed68f9364ac588df45a8f81d45874b1c10a496d0a2129be9134639d.
//
// Solidity: event NewTaskCreated(uint32 indexed taskIndex, (uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32) task)
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
	TaskIndex            uint32
	TaskResponse         IFinalizerTaskManagerTaskResponse
	TaskResponseMetadata IFinalizerTaskManagerTaskResponseMetadata
	Raw                  types.Log // Blockchain specific contextual infos
}

// FilterTaskResponded is a free log retrieval operation binding the contract event 0x82573b6e06a270f942a7da9b7d171ae407fc5035f31cdfd878fca0cb9ab67aab.
//
// Solidity: event TaskResponded(uint32 indexed taskIndex, (uint32,bytes32,bytes32,bytes32,bytes32,bytes32) taskResponse, (uint32,bytes32,uint96[],uint96[]) taskResponseMetadata)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) FilterTaskResponded(opts *bind.FilterOpts, taskIndex []uint32) (*ContractFinalizerTaskManagerTaskRespondedIterator, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.FilterLogs(opts, "TaskResponded", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerTaskManagerTaskRespondedIterator{contract: _ContractFinalizerTaskManager.contract, event: "TaskResponded", logs: logs, sub: sub}, nil
}

// WatchTaskResponded is a free log subscription operation binding the contract event 0x82573b6e06a270f942a7da9b7d171ae407fc5035f31cdfd878fca0cb9ab67aab.
//
// Solidity: event TaskResponded(uint32 indexed taskIndex, (uint32,bytes32,bytes32,bytes32,bytes32,bytes32) taskResponse, (uint32,bytes32,uint96[],uint96[]) taskResponseMetadata)
func (_ContractFinalizerTaskManager *ContractFinalizerTaskManagerFilterer) WatchTaskResponded(opts *bind.WatchOpts, sink chan<- *ContractFinalizerTaskManagerTaskResponded, taskIndex []uint32) (event.Subscription, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractFinalizerTaskManager.contract.WatchLogs(opts, "TaskResponded", taskIndexRule)
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

// ParseTaskResponded is a log parse operation binding the contract event 0x82573b6e06a270f942a7da9b7d171ae407fc5035f31cdfd878fca0cb9ab67aab.
//
// Solidity: event TaskResponded(uint32 indexed taskIndex, (uint32,bytes32,bytes32,bytes32,bytes32,bytes32) taskResponse, (uint32,bytes32,uint96[],uint96[]) taskResponseMetadata)
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

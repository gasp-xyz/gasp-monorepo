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
	ABI: "[{\"type\":\"constructor\",\"inputs\":[{\"name\":\"_registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"_taskResponseWindowBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"aggregator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allTaskHashes\",\"inputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allTaskResponses\",\"inputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"blsApkRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIBLSApkRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"checkSignatures\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"referenceBlockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"params\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.QuorumStakeTotals\",\"components\":[{\"name\":\"signedStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"totalStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"createNewTask\",\"inputs\":[{\"name\":\"blockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"delegation\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIDelegationManager\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"dummyForOperatorStateInfoType\",\"inputs\":[{\"name\":\"_operatorStateInfo\",\"type\":\"tuple\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorStateInfo\",\"components\":[{\"name\":\"operatorsStateChanged\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"quorumsRemoved\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsAdded[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"quorumsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsStakeUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]},{\"name\":\"quorumsApkUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsApkUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"OperatorsRemoved\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"operatorsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsAdded[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]},{\"name\":\"operatorsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsStakeUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"operatorsQuorumCountUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsQuorumCountUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]}]}],\"outputs\":[],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"generator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getCheckSignaturesIndices\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"referenceBlockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"nonSignerOperatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structOperatorStateRetriever.CheckSignaturesIndices\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getLatestPendingStateHash\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getOperatorState\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple[][]\",\"internalType\":\"structOperatorStateRetriever.Operator[][]\",\"components\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"stake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getOperatorState\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"\",\"type\":\"tuple[][]\",\"internalType\":\"structOperatorStateRetriever.Operator[][]\",\"components\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"stake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getQuorumBitmapsAtBlockNumber\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256[]\",\"internalType\":\"uint256[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getTaskResponseWindowBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"indexToTaskStatus\",\"inputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumFinalizerTaskManager.TaskStatus\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"_pauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"initialOwner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_aggregator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_generator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"latestTaskNum\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"pauseAll\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[{\"name\":\"index\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pauserRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"registryCoordinator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"respondToTask\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.Task\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"blockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.TaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"storageProofHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"pendingStateHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]},{\"name\":\"NonSignerStakesAndSignatureForOldState\",\"type\":\"tuple\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.NonSignerStakesAndSignatureForOldState\",\"components\":[{\"name\":\"nonSignerG1PubkeysForOldState\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2ForOldState\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigmaForOldState\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setPauserRegistry\",\"inputs\":[{\"name\":\"newPauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setStaleStakesForbidden\",\"inputs\":[{\"name\":\"value\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"stakeRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIStakeRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"staleStakesForbidden\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"taskNumber\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"trySignatureAndApkVerification\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"apk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}],\"outputs\":[{\"name\":\"pairingSuccessful\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"siganatureIsValid\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"unpause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"Initialized\",\"inputs\":[{\"name\":\"version\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewTaskCreated\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"task\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.Task\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"blockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Paused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"PauserRegistrySet\",\"inputs\":[{\"name\":\"pauserRegistry\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"newPauserRegistry\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractIPauserRegistry\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"StaleStakesForbiddenUpdate\",\"inputs\":[{\"name\":\"value\",\"type\":\"bool\",\"indexed\":false,\"internalType\":\"bool\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"TaskCompleted\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"blockHash\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"TaskResponded\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.TaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"storageProofHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"pendingStateHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"taskResponseMetadata\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIFinalizerTaskManager.TaskResponseMetadata\",\"components\":[{\"name\":\"taskResponsedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"hashOfNonSigners\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quroumStakeTotals\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quroumStakeSigned\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Unpaused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false}]",
	Bin: "0x6101206040523480156200001257600080fd5b5060405162005dfc38038062005dfc8339810160408190526200003591620001f7565b81806001600160a01b03166080816001600160a01b031681525050806001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa1580156200008f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000b591906200023e565b6001600160a01b031660a0816001600160a01b031681525050806001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa1580156200010d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200013391906200023e565b6001600160a01b031660c0816001600160a01b03168152505060a0516001600160a01b031663df5cf7236040518163ffffffff1660e01b8152600401602060405180830381865afa1580156200018d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001b391906200023e565b6001600160a01b031660e052506097805460ff1916600117905563ffffffff16610100525062000265565b6001600160a01b0381168114620001f457600080fd5b50565b600080604083850312156200020b57600080fd5b82516200021881620001de565b602084015190925063ffffffff811681146200023357600080fd5b809150509250929050565b6000602082840312156200025157600080fd5b81516200025e81620001de565b9392505050565b60805160a05160c05160e05161010051615b13620002e96000396000818161057d0152612d6a015260008181610546015261232f0152600081816103be01526125120152600081816103e5015281816126e801526128aa01526000818161041f01528181610de101528181611ffa0152818161219201526123cc0152615b136000f3fe608060405234801561001057600080fd5b50600436106102115760003560e01c80636b92787e1161012557806399dba0c4116100ad578063df5cf7231161007c578063df5cf72314610541578063f2fde38b14610568578063f5c9899d1461057b578063f8c8765e146105a1578063fabc1cbc146105b457600080fd5b806399dba0c4146104d0578063acfa135414610500578063b98d090814610513578063cefdc1d41461052057600080fd5b806372d18e8d116100f457806372d18e8d1461046a5780637afa1eed14610489578063886f11951461049c5780638b00ce7c146104af5780638da5cb5b146104bf57600080fd5b80636b92787e146104075780636d14a9871461041a5780636efb463614610441578063715018a61461046257600080fd5b8063416c7e5e116101a85780635ac86ab7116101775780635ac86ab71461035e5780635c155662146103915780635c975abb146103b15780635df45946146103b957806368304835146103e057600080fd5b8063416c7e5e146103125780634f739f741461032557806354d127de14610345578063595c6a671461035657600080fd5b8063245a7bfc116101e4578063245a7bfc1461027f5780632cb223d5146102b25780632d89f6fc146102d25780633563b0d1146102f257600080fd5b80630373408d1461021657806310d67a2f1461022d578063136439dd14610242578063171f1d5b14610255575b600080fd5b60d1545b6040519081526020015b60405180910390f35b61024061023b3660046143f2565b6105c7565b005b61024061025036600461440f565b610680565b61026861026336600461458d565b6107bf565b604080519215158352901515602083015201610224565b60cf5461029a9064010000000090046001600160a01b031681565b6040516001600160a01b039091168152602001610224565b61021a6102c03660046145fb565b60cb6020526000908152604090205481565b61021a6102e03660046145fb565b60ca6020526000908152604090205481565b610305610300366004614618565b610949565b6040516102249190614773565b61024061032036600461479b565b610ddf565b610338610333366004614800565b610f54565b6040516102249190614904565b6102406103533660046149bf565b50565b61024061167a565b61038161036c366004614a09565b606654600160ff9092169190911b9081161490565b6040519015158152602001610224565b6103a461039f366004614a49565b611741565b6040516102249190614afa565b60665461021a565b61029a7f000000000000000000000000000000000000000000000000000000000000000081565b61029a7f000000000000000000000000000000000000000000000000000000000000000081565b610240610415366004614b3e565b611909565b61029a7f000000000000000000000000000000000000000000000000000000000000000081565b61045461044f366004614e15565b611c47565b604051610224929190614ed5565b610240612b5f565b60c95463ffffffff165b60405163ffffffff9091168152602001610224565b60d05461029a906001600160a01b031681565b60655461029a906001600160a01b031681565b60c9546104749063ffffffff1681565b6033546001600160a01b031661029a565b6104f36104de3660046145fb565b60cc6020526000908152604090205460ff1681565b6040516102249190614f34565b61024061050e366004614f5c565b612b73565b6097546103819060ff1681565b61053361052e366004615074565b6130b4565b6040516102249291906150b6565b61029a7f000000000000000000000000000000000000000000000000000000000000000081565b6102406105763660046143f2565b613246565b7f0000000000000000000000000000000000000000000000000000000000000000610474565b6102406105af3660046150d7565b6132bc565b6102406105c236600461440f565b613422565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561061a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061063e9190615133565b6001600160a01b0316336001600160a01b0316146106775760405162461bcd60e51b815260040161066e90615150565b60405180910390fd5b6103538161357e565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa1580156106c8573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106ec919061519a565b6107085760405162461bcd60e51b815260040161066e906151b7565b606654818116146107815760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e70617573653a20696e76616c696420617474656d70742060448201527f746f20756e70617573652066756e6374696f6e616c6974790000000000000000606482015260840161066e565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b60008060007f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f000000187876000015188602001518860000151600060028110610807576108076151ff565b60200201518951600160200201518a6020015160006002811061082c5761082c6151ff565b60200201518b60200151600160028110610848576108486151ff565b602090810291909101518c518d8301516040516108a59a99989796959401988952602089019790975260408801959095526060870193909352608086019190915260a085015260c084015260e08301526101008201526101200190565b6040516020818303038152906040528051906020012060001c6108c89190615215565b905061093b6108e16108da8884613675565b869061370c565b6108e96137a0565b6109316109228561091c604080518082018252600080825260209182015281518083019092526001825260029082015290565b90613675565b61092b8c613860565b9061370c565b886201d4c06138f0565b909890975095505050505050565b60606000846001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa15801561098b573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109af9190615133565b90506000856001600160a01b0316639e9923c26040518163ffffffff1660e01b8152600401602060405180830381865afa1580156109f1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a159190615133565b90506000866001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa158015610a57573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a7b9190615133565b9050600086516001600160401b03811115610a9857610a98614428565b604051908082528060200260200182016040528015610acb57816020015b6060815260200190600190039081610ab65790505b50905060005b8751811015610dd3576000888281518110610aee57610aee6151ff565b0160200151604051638902624560e01b815260f89190911c6004820181905263ffffffff8a16602483015291506000906001600160a01b03871690638902624590604401600060405180830381865afa158015610b4f573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610b779190810190615237565b905080516001600160401b03811115610b9257610b92614428565b604051908082528060200260200182016040528015610bdd57816020015b6040805160608101825260008082526020808301829052928201528252600019909201910181610bb05790505b50848481518110610bf057610bf06151ff565b602002602001018190525060005b8151811015610dbd576040518060600160405280876001600160a01b03166347b314e8858581518110610c3357610c336151ff565b60200260200101516040518263ffffffff1660e01b8152600401610c5991815260200190565b602060405180830381865afa158015610c76573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c9a9190615133565b6001600160a01b03168152602001838381518110610cba57610cba6151ff565b60200260200101518152602001896001600160a01b031663fa28c627858581518110610ce857610ce86151ff565b60209081029190910101516040516001600160e01b031960e084901b168152600481019190915260ff8816602482015263ffffffff8f166044820152606401602060405180830381865afa158015610d44573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d6891906152c7565b6001600160601b0316815250858581518110610d8657610d866151ff565b60200260200101518281518110610d9f57610d9f6151ff565b60200260200101819052508080610db590615306565b915050610bfe565b5050508080610dcb90615306565b915050610ad1565b50979650505050505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610e3d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610e619190615133565b6001600160a01b0316336001600160a01b031614610f0d5760405162461bcd60e51b815260206004820152605c60248201527f424c535369676e6174757265436865636b65722e6f6e6c79436f6f7264696e6160448201527f746f724f776e65723a2063616c6c6572206973206e6f7420746865206f776e6560648201527f72206f6620746865207265676973747279436f6f7264696e61746f7200000000608482015260a40161066e565b6097805460ff19168215159081179091556040519081527f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc9060200160405180910390a150565b610f7f6040518060800160405280606081526020016060815260200160608152602001606081525090565b6000876001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa158015610fbf573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610fe39190615133565b90506110106040518060800160405280606081526020016060815260200160608152602001606081525090565b6040516361c8a12f60e11b81526001600160a01b038a169063c391425e90611040908b9089908990600401615321565b600060405180830381865afa15801561105d573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611085919081019061536b565b81526040516340e03a8160e11b81526001600160a01b038316906381c07502906110b7908b908b908b90600401615422565b600060405180830381865afa1580156110d4573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526110fc919081019061536b565b6040820152856001600160401b0381111561111957611119614428565b60405190808252806020026020018201604052801561114c57816020015b60608152602001906001900390816111375790505b50606082015260005b60ff811687111561158b576000856001600160401b0381111561117a5761117a614428565b6040519080825280602002602001820160405280156111a3578160200160208202803683370190505b5083606001518360ff16815181106111bd576111bd6151ff565b602002602001018190525060005b8681101561148b5760008c6001600160a01b03166304ec63518a8a858181106111f6576111f66151ff565b905060200201358e88600001518681518110611214576112146151ff565b60200260200101516040518463ffffffff1660e01b81526004016112519392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa15801561126e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611292919061544b565b90506001600160c01b0381166113365760405162461bcd60e51b815260206004820152605c60248201527f4f70657261746f7253746174655265747269657665722e676574436865636b5360448201527f69676e617475726573496e64696365733a206f70657261746f72206d7573742060648201527f6265207265676973746572656420617420626c6f636b6e756d62657200000000608482015260a40161066e565b8a8a8560ff1681811061134b5761134b6151ff565b6001600160c01b03841692013560f81c9190911c60019081161415905061147857856001600160a01b031663dd9846b98a8a8581811061138d5761138d6151ff565b905060200201358d8d8860ff168181106113a9576113a96151ff565b6040516001600160e01b031960e087901b1681526004810194909452919091013560f81c60248301525063ffffffff8f166044820152606401602060405180830381865afa1580156113ff573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906114239190615474565b85606001518560ff168151811061143c5761143c6151ff565b60200260200101518481518110611455576114556151ff565b63ffffffff909216602092830291909101909101528261147481615306565b9350505b508061148381615306565b9150506111cb565b506000816001600160401b038111156114a6576114a6614428565b6040519080825280602002602001820160405280156114cf578160200160208202803683370190505b50905060005b828110156115505784606001518460ff16815181106114f6576114f66151ff565b6020026020010151818151811061150f5761150f6151ff565b6020026020010151828281518110611529576115296151ff565b63ffffffff909216602092830291909101909101528061154881615306565b9150506114d5565b508084606001518460ff168151811061156b5761156b6151ff565b60200260200101819052505050808061158390615491565b915050611155565b506000896001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa1580156115cc573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115f09190615133565b60405163354952a360e21b81529091506001600160a01b0382169063d5254a8c90611623908b908b908e906004016154b1565b600060405180830381865afa158015611640573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611668919081019061536b565b60208301525098975050505050505050565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa1580156116c2573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906116e6919061519a565b6117025760405162461bcd60e51b815260040161066e906151b7565b600019606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b60606000846001600160a01b031663c391425e84866040518363ffffffff1660e01b81526004016117739291906154db565b600060405180830381865afa158015611790573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526117b8919081019061536b565b9050600084516001600160401b038111156117d5576117d5614428565b6040519080825280602002602001820160405280156117fe578160200160208202803683370190505b50905060005b85518110156118ff57866001600160a01b03166304ec635187838151811061182e5761182e6151ff565b602002602001015187868581518110611849576118496151ff565b60200260200101516040518463ffffffff1660e01b81526004016118869392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa1580156118a3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906118c7919061544b565b6001600160c01b03168282815181106118e2576118e26151ff565b6020908102919091010152806118f781615306565b915050611804565b5095945050505050565b60d0546001600160a01b0316331461194b5760405162461bcd60e51b8152602060048201526005602482015264417574683160d81b604482015260640161066e565b60cd5463ffffffff16431480159061196257504315155b6119d45760405162461bcd60e51b815260206004820152603960248201527f43616e2774206372656174652061207461736b20696e207468652073616d652060448201527f626c6f636b206173206120636f6d706c65746564207461736b00000000000000606482015260840161066e565b6040805161010081018252600060608083018290526080830181905260c083015260e082015260c95463ffffffff908116825260208083018890524382168385015290861660a08301528251601f85018290048202810182019093528383529091908490849081908401838280828437600092019190915250505050608082015260cd5463ffffffff16606082015260ce8054611a709061552f565b80601f0160208091040260200160405190810160405280929190818152602001828054611a9c9061552f565b8015611ae95780601f10611abe57610100808354040283529160200191611ae9565b820191906000526020600020905b815481529060010190602001808311611acc57829003601f168201915b505050505060c082015260cf5463ffffffff90811660e083015260c9541615611b7b5760c954600090611b249060019063ffffffff1661556a565b9050600163ffffffff8216600090815260cc602052604090205460ff166003811115611b5257611b52614f1e565b1415611b795763ffffffff8116600090815260cc60205260409020805460ff191660021790555b505b80604051602001611b8c91906155dc565b60408051808303601f19018152828252805160209182012060c9805463ffffffff908116600090815260ca85528581209390935581548116835260cc909352929020805460ff19166001179055905416907f840101e40ed68f9364ac588df45a8f81d45874b1c10a496d0a2129be9134639d90611c0a9084906155dc565b60405180910390a260c954611c269063ffffffff16600161568d565b60c9805463ffffffff191663ffffffff929092169190911790555050505050565b6040805180820190915260608082526020820152600084611cbe5760405162461bcd60e51b81526020600482015260376024820152600080516020615abe83398151915260448201527f7265733a20656d7074792071756f72756d20696e707574000000000000000000606482015260840161066e565b60408301515185148015611cd6575060a08301515185145b8015611ce6575060c08301515185145b8015611cf6575060e08301515185145b611d605760405162461bcd60e51b81526020600482015260416024820152600080516020615abe83398151915260448201527f7265733a20696e7075742071756f72756d206c656e677468206d69736d6174636064820152600d60fb1b608482015260a40161066e565b82515160208401515114611dd85760405162461bcd60e51b815260206004820152604460248201819052600080516020615abe833981519152908201527f7265733a20696e707574206e6f6e7369676e6572206c656e677468206d69736d6064820152630c2e8c6d60e31b608482015260a40161066e565b4363ffffffff168463ffffffff1610611e475760405162461bcd60e51b815260206004820152603c6024820152600080516020615abe83398151915260448201527f7265733a20696e76616c6964207265666572656e636520626c6f636b00000000606482015260840161066e565b6040805180820182526000808252602080830191909152825180840190935260608084529083015290866001600160401b03811115611e8857611e88614428565b604051908082528060200260200182016040528015611eb1578160200160208202803683370190505b506020820152866001600160401b03811115611ecf57611ecf614428565b604051908082528060200260200182016040528015611ef8578160200160208202803683370190505b50815260408051808201909152606080825260208201528560200151516001600160401b03811115611f2c57611f2c614428565b604051908082528060200260200182016040528015611f55578160200160208202803683370190505b5081526020860151516001600160401b03811115611f7557611f75614428565b604051908082528060200260200182016040528015611f9e578160200160208202803683370190505b50816020018190525060006120708a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152505060408051639aa1653d60e01b815290516001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169350639aa1653d925060048083019260209291908290030181865afa158015612047573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061206b91906156b5565b613b14565b905060005b87602001515181101561230b576120ba8860200151828151811061209b5761209b6151ff565b6020026020010151805160009081526020918201519091526040902090565b836020015182815181106120d0576120d06151ff565b602090810291909101015280156121905760208301516120f16001836156d2565b81518110612101576121016151ff565b602002602001015160001c83602001518281518110612122576121226151ff565b602002602001015160001c11612190576040805162461bcd60e51b8152602060048201526024810191909152600080516020615abe83398151915260448201527f7265733a206e6f6e5369676e65725075626b657973206e6f7420736f72746564606482015260840161066e565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166304ec6351846020015183815181106121d5576121d56151ff565b60200260200101518b8b6000015185815181106121f4576121f46151ff565b60200260200101516040518463ffffffff1660e01b81526004016122319392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa15801561224e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612272919061544b565b6001600160c01b031683600001518281518110612291576122916151ff565b6020026020010181815250506122f76108da6122cb84866000015185815181106122bd576122bd6151ff565b602002602001015116613ba7565b8a6020015184815181106122e1576122e16151ff565b6020026020010151613bd290919063ffffffff16565b94508061230381615306565b915050612075565b505061231683613cb6565b60975490935060ff1660008161232d5760006123af565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663c448feb86040518163ffffffff1660e01b8152600401602060405180830381865afa15801561238b573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906123af91906156e9565b905060005b8a811015612a2e578215612510578963ffffffff16827f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663249a0c428f8f8681811061240b5761240b6151ff565b60405160e085901b6001600160e01b031916815292013560f81c600483015250602401602060405180830381865afa15801561244b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061246f91906156e9565b6124799190615702565b10156125105760405162461bcd60e51b81526020600482015260666024820152600080516020615abe83398151915260448201527f7265733a205374616b6552656769737472792075706461746573206d7573742060648201527f62652077697468696e207769746864726177616c44656c6179426c6f636b732060848201526577696e646f7760d01b60a482015260c40161066e565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166368bccaac8d8d84818110612551576125516151ff565b9050013560f81c60f81b60f81c8c8c60a001518581518110612575576125756151ff565b60209081029190910101516040516001600160e01b031960e086901b16815260ff909316600484015263ffffffff9182166024840152166044820152606401602060405180830381865afa1580156125d1573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906125f5919061571a565b6001600160401b0319166126188a60400151838151811061209b5761209b6151ff565b67ffffffffffffffff1916146126b45760405162461bcd60e51b81526020600482015260616024820152600080516020615abe83398151915260448201527f7265733a2071756f72756d41706b206861736820696e2073746f72616765206460648201527f6f6573206e6f74206d617463682070726f76696465642071756f72756d2061706084820152606b60f81b60a482015260c40161066e565b6126e4896040015182815181106126cd576126cd6151ff565b60200260200101518761370c90919063ffffffff16565b95507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663c8294c568d8d84818110612727576127276151ff565b9050013560f81c60f81b60f81c8c8c60c00151858151811061274b5761274b6151ff565b60209081029190910101516040516001600160e01b031960e086901b16815260ff909316600484015263ffffffff9182166024840152166044820152606401602060405180830381865afa1580156127a7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906127cb91906152c7565b856020015182815181106127e1576127e16151ff565b6001600160601b0390921660209283029190910182015285015180518290811061280d5761280d6151ff565b60200260200101518560000151828151811061282b5761282b6151ff565b60200260200101906001600160601b031690816001600160601b0316815250506000805b8a6020015151811015612a19576128a386600001518281518110612875576128756151ff565b60200260200101518f8f8681811061288f5761288f6151ff565b600192013560f81c9290921c811614919050565b15612a07577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f2be94ae8f8f868181106128e9576128e96151ff565b9050013560f81c60f81b60f81c8e8960200151858151811061290d5761290d6151ff565b60200260200101518f60e00151888151811061292b5761292b6151ff565b60200260200101518781518110612944576129446151ff565b60209081029190910101516040516001600160e01b031960e087901b16815260ff909416600485015263ffffffff92831660248501526044840191909152166064820152608401602060405180830381865afa1580156129a8573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906129cc91906152c7565b87518051859081106129e0576129e06151ff565b602002602001018181516129f49190615745565b6001600160601b03169052506001909101905b80612a1181615306565b91505061284f565b50508080612a2690615306565b9150506123b4565b505050600080612a488c868a606001518b608001516107bf565b9150915081612ab95760405162461bcd60e51b81526020600482015260436024820152600080516020615abe83398151915260448201527f7265733a2070616972696e6720707265636f6d70696c652063616c6c206661696064820152621b195960ea1b608482015260a40161066e565b80612b1a5760405162461bcd60e51b81526020600482015260396024820152600080516020615abe83398151915260448201527f7265733a207369676e617475726520697320696e76616c696400000000000000606482015260840161066e565b50506000878260200151604051602001612b35929190615765565b60408051808303601f190181529190528051602090910120929b929a509198505050505050505050565b612b67613d51565b612b716000613dab565b565b60cf5464010000000090046001600160a01b03163314612bbd5760405162461bcd60e51b8152602060048201526005602482015264041757468360dc1b604482015260640161066e565b6000612bcf60608601604087016145fb565b9050366000612be160808801886157ad565b90925090506000612bf860c0890160a08a016145fb565b905060ca6000612c0b60208a018a6145fb565b63ffffffff1663ffffffff1681526020019081526020016000205488604051602001612c379190615838565b6040516020818303038152906040528051906020012014612cc05760405162461bcd60e51b815260206004820152603d60248201527f737570706c696564207461736b20646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e7472616374000000606482015260840161066e565b600160cc6000612cd360208b018b6145fb565b63ffffffff16815260208101919091526040016000205460ff166003811115612cfe57612cfe614f1e565b14612d1b5760405162461bcd60e51b815260040161066e9061591a565b600060cb81612d2d60208b018b6145fb565b63ffffffff1663ffffffff1681526020019081526020016000205414612d655760405162461bcd60e51b815260040161066e9061591a565b612d8f7f00000000000000000000000000000000000000000000000000000000000000008561568d565b63ffffffff164363ffffffff161115612e005760405162461bcd60e51b815260206004820152602d60248201527f41676772656761746f722068617320726573706f6e64656420746f207468652060448201526c7461736b20746f6f206c61746560981b606482015260840161066e565b600087604051602001612e1391906159b1565b604051602081830303815290604052805190602001209050600080612e3b8387878a8d611c47565b6040805160808101825263ffffffff43168152602080820184905280850151828401528451606083015291519395509193509091612e7d918d918491016159bf565b6040516020818303038152906040528051906020012060cb60008d6000016020810190612eaa91906145fb565b63ffffffff16815260208082019290925260400160002091909155612ed1908d018d6145fb565b63ffffffff167f82573b6e06a270f942a7da9b7d171ae407fc5035f31cdfd878fca0cb9ab67aab8c83604051612f089291906159bf565b60405180910390a260005b86811015612fa9578560ff1684602001518281518110612f3557612f356151ff565b6020026020010151612f479190615a2d565b6001600160601b0316606485600001518381518110612f6857612f686151ff565b60200260200101516001600160601b0316612f839190615a5c565b1015612f97575050505050505050506130ae565b80612fa181615306565b915050612f13565b5060a08b013560d155600360cc6000612fc560208f018f6145fb565b63ffffffff16815260208101919091526040016000205460ff166003811115612ff057612ff0614f1e565b50613003905060608d0160408e016145fb565b60cd805463ffffffff191663ffffffff9290921691909117905561302a60808d018d6157ad565b6130369160ce9161426a565b5061304760c08d0160a08e016145fb565b60cf805463ffffffff191663ffffffff9290921691909117905560608b013561307360208d018d6145fb565b63ffffffff167f8378be8a33cf3a493910a16e275cd96af4f048c5eb1a2c2962d4066e697fea8060405160405180910390a350505050505050505b50505050565b60408051600180825281830190925260009160609183916020808301908036833701905050905084816000815181106130ef576130ef6151ff565b60209081029190910101526040516361c8a12f60e11b81526000906001600160a01b0388169063c391425e9061312b90889086906004016154db565b600060405180830381865afa158015613148573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052613170919081019061536b565b600081518110613182576131826151ff565b60209081029190910101516040516304ec635160e01b81526004810188905263ffffffff87811660248301529091166044820181905291506000906001600160a01b038916906304ec635190606401602060405180830381865afa1580156131ee573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613212919061544b565b6001600160c01b03169050600061322882613dfd565b9050816132368a838a610949565b9550955050505050935093915050565b61324e613d51565b6001600160a01b0381166132b35760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b606482015260840161066e565b61035381613dab565b600054610100900460ff16158080156132dc5750600054600160ff909116105b806132f65750303b1580156132f6575060005460ff166001145b6133595760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b606482015260840161066e565b6000805460ff19166001179055801561337c576000805461ff0019166101001790555b613387856000613ec9565b61339084613dab565b60cf8054640100000000600160c01b0319166401000000006001600160a01b03868116919091029190911790915560d080546001600160a01b031916918416919091179055801561341b576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b5050505050565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613475573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906134999190615133565b6001600160a01b0316336001600160a01b0316146134c95760405162461bcd60e51b815260040161066e90615150565b6066541981196066541916146135475760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e756e70617573653a20696e76616c696420617474656d7060448201527f7420746f2070617573652066756e6374696f6e616c6974790000000000000000606482015260840161066e565b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c906020016107b4565b6001600160a01b03811661360c5760405162461bcd60e51b815260206004820152604960248201527f5061757361626c652e5f73657450617573657252656769737472793a206e657760448201527f50617573657252656769737472792063616e6e6f7420626520746865207a65726064820152686f206164647265737360b81b608482015260a40161066e565b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b60408051808201909152600080825260208201526136916142ee565b835181526020808501519082015260408082018490526000908360608460076107d05a03fa90508080156136c4576136c6565bfe5b50806137045760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b604482015260640161066e565b505092915050565b604080518082019091526000808252602082015261372861430c565b835181526020808501518183015283516040808401919091529084015160608301526000908360808460066107d05a03fa90508080156136c45750806137045760405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b604482015260640161066e565b6137a861432a565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b604080518082019091526000808252602082015260008080613890600080516020615a9e83398151915286615215565b90505b61389c81613fb3565b9093509150600080516020615a9e8339815191528283098314156138d6576040805180820190915290815260208101919091529392505050565b600080516020615a9e833981519152600182089050613893565b60408051808201825286815260208082018690528251808401909352868352820184905260009182919061392261434f565b60005b6002811015613ae757600061393b826006615a5c565b905084826002811061394f5761394f6151ff565b60200201515183613961836000615702565b600c8110613971576139716151ff565b6020020152848260028110613988576139886151ff565b6020020151602001518382600161399f9190615702565b600c81106139af576139af6151ff565b60200201528382600281106139c6576139c66151ff565b60200201515151836139d9836002615702565b600c81106139e9576139e96151ff565b6020020152838260028110613a0057613a006151ff565b6020020151516001602002015183613a19836003615702565b600c8110613a2957613a296151ff565b6020020152838260028110613a4057613a406151ff565b602002015160200151600060028110613a5b57613a5b6151ff565b602002015183613a6c836004615702565b600c8110613a7c57613a7c6151ff565b6020020152838260028110613a9357613a936151ff565b602002015160200151600160028110613aae57613aae6151ff565b602002015183613abf836005615702565b600c8110613acf57613acf6151ff565b60200201525080613adf81615306565b915050613925565b50613af061436e565b60006020826101808560088cfa9151919c9115159b50909950505050505050505050565b600080613b2084614035565b9050808360ff166001901b11613b9e5760405162461bcd60e51b815260206004820152603f60248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206269746d61702065786365656473206d61782076616c756500606482015260840161066e565b90505b92915050565b6000805b8215613ba157613bbc6001846156d2565b9092169180613bca81615a7b565b915050613bab565b60408051808201909152600080825260208201526102008261ffff1610613c2e5760405162461bcd60e51b815260206004820152601060248201526f7363616c61722d746f6f2d6c6172676560801b604482015260640161066e565b8161ffff1660011415613c42575081613ba1565b6040805180820190915260008082526020820181905284906001905b8161ffff168661ffff1610613cab57600161ffff871660ff83161c81161415613c8e57613c8b848461370c565b93505b613c98838461370c565b92506201fffe600192831b169101613c5e565b509195945050505050565b60408051808201909152600080825260208201528151158015613cdb57506020820151155b15613cf9575050604080518082019091526000808252602082015290565b604051806040016040528083600001518152602001600080516020615a9e8339815191528460200151613d2c9190615215565b613d4490600080516020615a9e8339815191526156d2565b905292915050565b919050565b6033546001600160a01b03163314612b715760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604482015260640161066e565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6060600080613e0b84613ba7565b61ffff166001600160401b03811115613e2657613e26614428565b6040519080825280601f01601f191660200182016040528015613e50576020820181803683370190505b5090506000805b825182108015613e68575061010081105b15613ebf576001811b935085841615613eaf578060f81b838381518110613e9157613e916151ff565b60200101906001600160f81b031916908160001a9053508160010191505b613eb881615306565b9050613e57565b5090949350505050565b6065546001600160a01b0316158015613eea57506001600160a01b03821615155b613f6c5760405162461bcd60e51b815260206004820152604760248201527f5061757361626c652e5f696e697469616c697a655061757365723a205f696e6960448201527f7469616c697a6550617573657228292063616e206f6e6c792062652063616c6c6064820152666564206f6e636560c81b608482015260a40161066e565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2613faf8261357e565b5050565b60008080600080516020615a9e8339815191526003600080516020615a9e83398151915286600080516020615a9e833981519152888909090890506000614029827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f52600080516020615a9e8339815191526141c2565b91959194509092505050565b6000610100825111156140be5760405162461bcd60e51b8152602060048201526044602482018190527f4269746d61705574696c732e6f72646572656442797465734172726179546f42908201527f69746d61703a206f7264657265644279746573417272617920697320746f6f206064820152636c6f6e6760e01b608482015260a40161066e565b81516140cc57506000919050565b600080836000815181106140e2576140e26151ff565b0160200151600160f89190911c81901b92505b84518110156141b957848181518110614110576141106151ff565b0160200151600160f89190911c1b91508282116141a55760405162461bcd60e51b815260206004820152604760248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f72646572656442797465734172726179206973206e6f74206064820152661bdc99195c995960ca1b608482015260a40161066e565b918117916141b281615306565b90506140f5565b50909392505050565b6000806141cd61436e565b6141d561438c565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa92508280156136c457508261425f5760405162461bcd60e51b815260206004820152601a60248201527f424e3235342e6578704d6f643a2063616c6c206661696c757265000000000000604482015260640161066e565b505195945050505050565b8280546142769061552f565b90600052602060002090601f01602090048101928261429857600085556142de565b82601f106142b15782800160ff198235161785556142de565b828001600101855582156142de579182015b828111156142de5782358255916020019190600101906142c3565b506142ea9291506143aa565b5090565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b604051806040016040528061433d6143bf565b815260200161434a6143bf565b905290565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b5b808211156142ea57600081556001016143ab565b60405180604001604052806002906020820280368337509192915050565b6001600160a01b038116811461035357600080fd5b60006020828403121561440457600080fd5b8135613b9e816143dd565b60006020828403121561442157600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b038111828210171561446057614460614428565b60405290565b60405161010081016001600160401b038111828210171561446057614460614428565b604051601f8201601f191681016001600160401b03811182821017156144b1576144b1614428565b604052919050565b6000604082840312156144cb57600080fd5b6144d361443e565b9050813581526020820135602082015292915050565b600082601f8301126144fa57600080fd5b604051604081018181106001600160401b038211171561451c5761451c614428565b806040525080604084018581111561453357600080fd5b845b81811015613cab578035835260209283019201614535565b60006080828403121561455f57600080fd5b61456761443e565b905061457383836144e9565b815261458283604084016144e9565b602082015292915050565b60008060008061012085870312156145a457600080fd5b843593506145b586602087016144b9565b92506145c4866060870161454d565b91506145d38660e087016144b9565b905092959194509250565b63ffffffff8116811461035357600080fd5b8035613d4c816145de565b60006020828403121561460d57600080fd5b8135613b9e816145de565b60008060006060848603121561462d57600080fd5b8335614638816143dd565b92506020848101356001600160401b038082111561465557600080fd5b818701915087601f83011261466957600080fd5b81358181111561467b5761467b614428565b61468d601f8201601f19168501614489565b915080825288848285010111156146a357600080fd5b80848401858401376000848284010152508094505050506146c6604085016145f0565b90509250925092565b600081518084526020808501808196508360051b810191508286016000805b86811015614765578385038a52825180518087529087019087870190845b8181101561475057835180516001600160a01b031684528a8101518b8501526040908101516001600160601b0316908401529289019260609092019160010161470c565b50509a87019a955050918501916001016146ee565b509298975050505050505050565b60208152600061478660208301846146cf565b9392505050565b801515811461035357600080fd5b6000602082840312156147ad57600080fd5b8135613b9e8161478d565b60008083601f8401126147ca57600080fd5b5081356001600160401b038111156147e157600080fd5b6020830191508360208285010111156147f957600080fd5b9250929050565b6000806000806000806080878903121561481957600080fd5b8635614824816143dd565b95506020870135614834816145de565b945060408701356001600160401b038082111561485057600080fd5b61485c8a838b016147b8565b9096509450606089013591508082111561487557600080fd5b818901915089601f83011261488957600080fd5b81358181111561489857600080fd5b8a60208260051b85010111156148ad57600080fd5b6020830194508093505050509295509295509295565b600081518084526020808501945080840160005b838110156148f957815163ffffffff16875295820195908201906001016148d7565b509495945050505050565b60006020808352835160808285015261492060a08501826148c3565b905081850151601f198086840301604087015261493d83836148c3565b9250604087015191508086840301606087015261495a83836148c3565b60608801518782038301608089015280518083529194508501925084840190600581901b8501860160005b828110156149b1578487830301845261499f8287516148c3565b95880195938801939150600101614985565b509998505050505050505050565b6000602082840312156149d157600080fd5b81356001600160401b038111156149e757600080fd5b82016101208185031215613b9e57600080fd5b60ff8116811461035357600080fd5b600060208284031215614a1b57600080fd5b8135613b9e816149fa565b60006001600160401b03821115614a3f57614a3f614428565b5060051b60200190565b600080600060608486031215614a5e57600080fd5b8335614a69816143dd565b92506020848101356001600160401b03811115614a8557600080fd5b8501601f81018713614a9657600080fd5b8035614aa9614aa482614a26565b614489565b81815260059190911b82018301908381019089831115614ac857600080fd5b928401925b82841015614ae657833582529284019290840190614acd565b80965050505050506146c6604085016145f0565b6020808252825182820181905260009190848201906040850190845b81811015614b3257835183529284019291840191600101614b16565b50909695505050505050565b60008060008060608587031215614b5457600080fd5b843593506020850135614b66816145de565b925060408501356001600160401b03811115614b8157600080fd5b614b8d878288016147b8565b95989497509550505050565b600082601f830112614baa57600080fd5b81356020614bba614aa483614a26565b82815260059290921b84018101918181019086841115614bd957600080fd5b8286015b84811015614bfd578035614bf0816145de565b8352918301918301614bdd565b509695505050505050565b600082601f830112614c1957600080fd5b81356020614c29614aa483614a26565b82815260069290921b84018101918181019086841115614c4857600080fd5b8286015b84811015614bfd57614c5e88826144b9565b835291830191604001614c4c565b600082601f830112614c7d57600080fd5b81356020614c8d614aa483614a26565b82815260059290921b84018101918181019086841115614cac57600080fd5b8286015b84811015614bfd5780356001600160401b03811115614ccf5760008081fd5b614cdd8986838b0101614b99565b845250918301918301614cb0565b60006101808284031215614cfe57600080fd5b614d06614466565b905081356001600160401b0380821115614d1f57600080fd5b614d2b85838601614b99565b83526020840135915080821115614d4157600080fd5b614d4d85838601614c08565b60208401526040840135915080821115614d6657600080fd5b614d7285838601614c08565b6040840152614d84856060860161454d565b6060840152614d968560e086016144b9565b6080840152610120840135915080821115614db057600080fd5b614dbc85838601614b99565b60a0840152610140840135915080821115614dd657600080fd5b614de285838601614b99565b60c0840152610160840135915080821115614dfc57600080fd5b50614e0984828501614c6c565b60e08301525092915050565b600080600080600060808688031215614e2d57600080fd5b8535945060208601356001600160401b0380821115614e4b57600080fd5b614e5789838a016147b8565b909650945060408801359150614e6c826145de565b90925060608701359080821115614e8257600080fd5b50614e8f88828901614ceb565b9150509295509295909350565b600081518084526020808501945080840160005b838110156148f95781516001600160601b031687529582019590820190600101614eb0565b6040815260008351604080840152614ef06080840182614e9c565b90506020850151603f19848303016060850152614f0d8282614e9c565b925050508260208301529392505050565b634e487b7160e01b600052602160045260246000fd5b6020810160048310614f5657634e487b7160e01b600052602160045260246000fd5b91905290565b600080600080848603610120811215614f7457600080fd5b85356001600160401b0380821115614f8b57600080fd5b818801915061010080838b031215614fa257600080fd5b82975060c0601f1985011215614fb757600080fd5b60208901965060e0890135935081841115614fd157600080fd5b614fdd8a858b01614ceb565b9550880135925080831115614ff157600080fd5b9187019160e0838a03121561500557600080fd5b604051915060608201828110828211171561502257615022614428565b60405282358181111561503457600080fd5b6150408a828601614c08565b83525050615051886020840161454d565b60208201526150638860a084016144b9565b604082015294979396509194505050565b60008060006060848603121561508957600080fd5b8335615094816143dd565b92506020840135915060408401356150ab816145de565b809150509250925092565b8281526040602082015260006150cf60408301846146cf565b949350505050565b600080600080608085870312156150ed57600080fd5b84356150f8816143dd565b93506020850135615108816143dd565b92506040850135615118816143dd565b91506060850135615128816143dd565b939692955090935050565b60006020828403121561514557600080fd5b8151613b9e816143dd565b6020808252602a908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526939903ab73830bab9b2b960b11b606082015260800190565b6000602082840312156151ac57600080fd5b8151613b9e8161478d565b60208082526028908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526739903830bab9b2b960c11b606082015260800190565b634e487b7160e01b600052603260045260246000fd5b60008261523257634e487b7160e01b600052601260045260246000fd5b500690565b6000602080838503121561524a57600080fd5b82516001600160401b0381111561526057600080fd5b8301601f8101851361527157600080fd5b805161527f614aa482614a26565b81815260059190911b8201830190838101908783111561529e57600080fd5b928401925b828410156152bc578351825292840192908401906152a3565b979650505050505050565b6000602082840312156152d957600080fd5b81516001600160601b0381168114613b9e57600080fd5b634e487b7160e01b600052601160045260246000fd5b600060001982141561531a5761531a6152f0565b5060010190565b63ffffffff84168152604060208201819052810182905260006001600160fb1b0383111561534e57600080fd5b8260051b8085606085013760009201606001918252509392505050565b6000602080838503121561537e57600080fd5b82516001600160401b0381111561539457600080fd5b8301601f810185136153a557600080fd5b80516153b3614aa482614a26565b81815260059190911b820183019083810190878311156153d257600080fd5b928401925b828410156152bc5783516153ea816145de565b825292840192908401906153d7565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b63ffffffff841681526040602082015260006154426040830184866153f9565b95945050505050565b60006020828403121561545d57600080fd5b81516001600160c01b0381168114613b9e57600080fd5b60006020828403121561548657600080fd5b8151613b9e816145de565b600060ff821660ff8114156154a8576154a86152f0565b60010192915050565b6040815260006154c56040830185876153f9565b905063ffffffff83166020830152949350505050565b60006040820163ffffffff851683526020604081850152818551808452606086019150828701935060005b8181101561552257845183529383019391830191600101615506565b5090979650505050505050565b600181811c9082168061554357607f821691505b6020821081141561556457634e487b7160e01b600052602260045260246000fd5b50919050565b600063ffffffff83811690831681811015615587576155876152f0565b039392505050565b6000815180845260005b818110156155b557602081850181015186830182015201615599565b818111156155c7576000602083870101525b50601f01601f19169290920160200192915050565b6020815263ffffffff82511660208201526020820151604082015260006040830151615610606084018263ffffffff169052565b50606083015163ffffffff811660808401525060808301516101008060a085015261563f61012085018361558f565b915060a085015161565860c086018263ffffffff169052565b5060c0850151848303601f190160e0860152615674838261558f565b92505060e0850151613ebf8286018263ffffffff169052565b600063ffffffff8083168185168083038211156156ac576156ac6152f0565b01949350505050565b6000602082840312156156c757600080fd5b8151613b9e816149fa565b6000828210156156e4576156e46152f0565b500390565b6000602082840312156156fb57600080fd5b5051919050565b60008219821115615715576157156152f0565b500190565b60006020828403121561572c57600080fd5b815167ffffffffffffffff1981168114613b9e57600080fd5b60006001600160601b0383811690831681811015615587576155876152f0565b63ffffffff60e01b8360e01b1681526000600482018351602080860160005b838110156157a057815185529382019390820190600101615784565b5092979650505050505050565b6000808335601e198436030181126157c457600080fd5b8301803591506001600160401b038211156157de57600080fd5b6020019150368190038213156147f957600080fd5b6000808335601e1984360301811261580a57600080fd5b83016020810192503590506001600160401b0381111561582957600080fd5b8036038313156147f957600080fd5b6020815260008235615849816145de565b63ffffffff81166020840152506020830135604083015261586c604084016145f0565b63ffffffff8116606084015250615885606084016145f0565b63ffffffff811660808401525061589f60808401846157f3565b6101008060a08601526158b7610120860183856153f9565b92506158c560a087016145f0565b63ffffffff811660c087015291506158e060c08701876157f3565b868503601f190160e088015292506158f98484836153f9565b93505061590860e087016145f0565b63ffffffff8116868301529150613ebf565b6020808252602c908201527f41676772656761746f722068617320616c726561647920726573706f6e64656460408201526b20746f20746865207461736b60a01b606082015260800190565b8035615971816145de565b63ffffffff81168352506020810135602083015260408101356040830152606081013560608301526080810135608083015260a081013560a08301525050565b60c08101613ba18284615966565b6159c98184615966565b60e060c082015263ffffffff82511660e08201526020820151610100820152600060408301516080610120840152615a05610160840182614e9c565b9050606084015160df1984830301610140850152615a238282614e9c565b9695505050505050565b60006001600160601b0380831681851681830481118215151615615a5357615a536152f0565b02949350505050565b6000816000190483118215151615615a7657615a766152f0565b500290565b600061ffff80831681811415615a9357615a936152f0565b600101939250505056fe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47424c535369676e6174757265436865636b65722e636865636b5369676e617475a2646970667358221220c36269fceb112d6cf5dfa72e1ca40119ccf591da088d0071e73205411c79bdb564736f6c634300080c0033",
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

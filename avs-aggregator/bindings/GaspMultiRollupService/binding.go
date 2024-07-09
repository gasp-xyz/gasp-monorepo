// Code generated - DO NOT EDIT.
// This file is a generated binding and any manual changes will be lost.

package contractGaspMultiRollupService

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

// ContractGaspMultiRollupServiceMetaData contains all meta data concerning the ContractGaspMultiRollupService contract.
var ContractGaspMultiRollupServiceMetaData = &bind.MetaData{
	ABI: "[{\"type\":\"function\",\"name\":\"checkSignatures\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"params\",\"type\":\"tuple\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.NonSignerStakesAndSignatureForOldState\",\"components\":[{\"name\":\"nonSignerG1PubkeysForOldState\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2ForOldState\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigmaForOldState\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.QuorumStakeTotals\",\"components\":[{\"name\":\"signedStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"totalStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"_pauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"initialOwner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_updater\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"lastUpdateBlockTimestamp\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestCompletedTaskCreatedBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestCompletedTaskNumber\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestPendingStateHash\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"operatorAndQuorumToStakes\",\"inputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint96\",\"internalType\":\"uint96\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"operatorIdQuorumCount\",\"inputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"pauseAll\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[{\"name\":\"index\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pauserRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"process_eigen_reinit\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.Task\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"blockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.TaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"storageProofHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"pendingStateHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"operatorStateInfo\",\"type\":\"tuple\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorStateInfo\",\"components\":[{\"name\":\"operatorsStateChanged\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"quorumsRemoved\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsAdded[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"quorumsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsStakeUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]},{\"name\":\"quorumsApkUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsApkUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"operatorsRemoved\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"operatorsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsAdded[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]},{\"name\":\"operatorsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsStakeUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"operatorsQuorumCountUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsQuorumCountUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"process_eigen_update\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.Task\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"blockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.TaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"storageProofHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"pendingStateHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"nonSignerStakesAndSignatureForOldState\",\"type\":\"tuple\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.NonSignerStakesAndSignatureForOldState\",\"components\":[{\"name\":\"nonSignerG1PubkeysForOldState\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2ForOldState\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigmaForOldState\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"operatorStateInfo\",\"type\":\"tuple\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorStateInfo\",\"components\":[{\"name\":\"operatorsStateChanged\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"quorumsRemoved\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsAdded[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"quorumsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsStakeUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]},{\"name\":\"quorumsApkUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsApkUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"operatorsRemoved\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"operatorsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsAdded[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]},{\"name\":\"operatorsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsStakeUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"operatorsQuorumCountUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsQuorumCountUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"qourumApk\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"quorumNumbers\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"quorumThresholdPercentage\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"quorumToStakes\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint96\",\"internalType\":\"uint96\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setPauserRegistry\",\"inputs\":[{\"name\":\"newPauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"set_updater\",\"inputs\":[{\"name\":\"_updater\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"stalled\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"trySignatureAndApkVerification\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"apk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}],\"outputs\":[{\"name\":\"pairingSuccessful\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"siganatureIsValid\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"unpause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updater\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"event\",\"name\":\"EigenReinitProcessed\",\"inputs\":[{\"name\":\"taskNumber\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"EigenUpdateProcessed\",\"inputs\":[{\"name\":\"taskNumber\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Initialized\",\"inputs\":[{\"name\":\"version\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Paused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"PauserRegistrySet\",\"inputs\":[{\"name\":\"pauserRegistry\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"newPauserRegistry\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractIPauserRegistry\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Unpaused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false}]",
	Bin: "0x608060405234801561001057600080fd5b5061493d806100206000396000f3fe608060405234801561001057600080fd5b50600436106101c45760003560e01c80635c975abb116100f9578063c927feef11610097578063ed5a04fe11610071578063ed5a04fe1461047a578063f2fde38b14610492578063fabc1cbc146104a5578063fc765dd5146104b857600080fd5b8063c927feef1461044b578063df034cd01461045e578063e61db1751461047157600080fd5b8063886f1195116100d3578063886f1195146103dc5780638da5cb5b14610407578063c0c53b8b14610418578063c4e1914c1461042b57600080fd5b80635c975abb146103a3578063715018a6146103ab5780637ad75561146103b357600080fd5b8063430d3b39116101665780634deabc21116101405780634deabc211461032f578063526e3e6414610354578063595c6a67146103785780635ac86ab71461038057600080fd5b8063430d3b3914610297578063499d6fb6146102cc5780634ae6b2031461031857600080fd5b8063136439dd116101a2578063136439dd14610232578063171f1d5b146102455780632a8414fd1461026f5780633e4049261461028457600080fd5b806303d097d2146101c957806310d67a2f1461020a578063124648c91461021f575b600080fd5b6101f06101d7366004613728565b609f602052600090815260409020805460019091015482565b604080519283526020830191909152015b60405180910390f35b61021d61021836600461375f565b6104c8565b005b61021d61022d36600461375f565b610584565b61021d61024036600461377c565b6105ae565b6102586102533660046138f9565b6106ed565b604080519215158352901515602083015201610201565b610277610877565b604051610201919061394a565b61021d6102923660046139dd565b610905565b6102ba6102a536600461377c565b60a06020526000908152604090205460ff1681565b60405160ff9091168152602001610201565b6103006102da366004613a80565b609e6020908152600092835260408084209091529082529020546001600160601b031681565b6040516001600160601b039091168152602001610201565b61032160995481565b604051908152602001610201565b609c5461033f9063ffffffff1681565b60405163ffffffff9091168152602001610201565b60975461036890600160a01b900460ff1681565b6040519015158152602001610201565b61021d611741565b61036861038e366004613728565b606654600160ff9092169190911b9081161490565b606654610321565b61021d611808565b6103006103c1366004613728565b609d602052600090815260409020546001600160601b031681565b6065546103ef906001600160a01b031681565b6040516001600160a01b039091168152602001610201565b6033546001600160a01b03166103ef565b61021d610426366004613aac565b61181c565b61043e610439366004613bd7565b61194d565b6040516102019190613c61565b61021d610459366004613ca3565b611f3a565b6097546103ef906001600160a01b031681565b61032160985481565b609a5461033f90640100000000900463ffffffff1681565b61021d6104a036600461375f565b6129fd565b61021d6104b336600461377c565b612a73565b609a5461033f9063ffffffff1681565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561051b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061053f9190613d18565b6001600160a01b0316336001600160a01b0316146105785760405162461bcd60e51b815260040161056f90613d35565b60405180910390fd5b61058181612bcf565b50565b61058c612cc6565b609780546001600160a01b0319166001600160a01b0392909216919091179055565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa1580156105f6573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061061a9190613d8d565b6106365760405162461bcd60e51b815260040161056f90613daa565b606654818116146106af5760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e70617573653a20696e76616c696420617474656d70742060448201527f746f20756e70617573652066756e6374696f6e616c6974790000000000000000606482015260840161056f565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b60008060007f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f00000018787600001518860200151886000015160006002811061073557610735613df2565b60200201518951600160200201518a6020015160006002811061075a5761075a613df2565b60200201518b6020015160016002811061077657610776613df2565b602090810291909101518c518d8301516040516107d39a99989796959401988952602089019790975260408801959095526060870193909352608086019190915260a085015260c084015260e08301526101008201526101200190565b6040516020818303038152906040528051906020012060001c6107f69190613e08565b905061086961080f6108088884612d20565b8690612db7565b610817612e4b565b61085f6108508561084a604080518082018252600080825260209182015281518083019092526001825260029082015290565b90612d20565b6108598c612f0b565b90612db7565b886201d4c0612f9b565b909890975095505050505050565b609b805461088490613e2a565b80601f01602080910402602001604051908101604052809291908181526020018280546108b090613e2a565b80156108fd5780601f106108d2576101008083540402835291602001916108fd565b820191906000526020600020905b8154815290600101906020018083116108e057829003601f168201915b505050505081565b6097546001600160a01b0316331461095f5760405162461bcd60e51b815260206004820152601a60248201527f55706461746572206d757374206265207468652063616c6c6572000000000000604482015260640161056f565b61096f6080850160608601613e73565b609a54640100000000900463ffffffff9081169116146109d15760405162461bcd60e51b815260206004820152601d60248201527f7265666572656e636520626c6f636b2068617368206d69736d61746368000000604482015260640161056f565b836040516020016109e29190613f03565b60405160208183030381529060405280519060200120836020013514610a4a5760405162461bcd60e51b815260206004820152601f60248201527f7265666572656e63655461736b486173682068617368206d69736d6174636800604482015260640161056f565b80604051602001610a5b919061446f565b60405160208183030381529060405280519060200120836040013514610ac35760405162461bcd60e51b815260206004820152601f60248201527f6f70657261746f725374617465496e666f2068617368206d69736d6174636800604482015260640161056f565b609a54640100000000900463ffffffff1615610c8357610ae96060850160408601613e73565b609a5463ffffffff91821691610b0a916401000000009004166138406145ec565b63ffffffff1611610b4d5760405162461bcd60e51b815260206004820152600d60248201526c07374616c65207374617465203609c1b604482015260640161056f565b426098546203f480610b5f9190614614565b11610b9c5760405162461bcd60e51b815260206004820152600d60248201526c7374616c65207374617465203160981b604482015260640161056f565b6000610bd284604051602001610bb2919061462c565b604051602081830303815290604052805190602001208461043990614679565b609c5490915063ffffffff1660005b609b8054610bee90613e2a565b9050811015610c7f578160ff1683602001518281518110610c1157610c11613df2565b6020026020010151610c239190614685565b6001600160601b0316606484600001518381518110610c4457610c44613df2565b60200260200101516001600160601b0316610c5f91906146b4565b1015610c6d5750505061173b565b80610c77816146d3565b915050610be1565b5050505b60005b610c9360208301836146ee565b9050811015610d5857609d6000610cad60208501856146ee565b84818110610cbd57610cbd613df2565b9050602002016020810190610cd29190613728565b60ff168152602080820192909252604001600090812080546001600160601b0319169055609f91610d05908501856146ee565b84818110610d1557610d15613df2565b9050602002016020810190610d2a9190613728565b60ff168152602081019190915260400160009081208181556001015580610d50816146d3565b915050610c86565b5060005b610d696040830183614737565b9050811015610ea457610d7f6040830183614737565b82818110610d8f57610d8f613df2565b9050608002016020016020810190610da79190614780565b609d6000610db86040860186614737565b85818110610dc857610dc8613df2565b610dde9260206080909202019081019150613728565b60ff1681526020810191909152604090810160002080546001600160601b0319166001600160601b039390931692909217909155610e1e90830183614737565b82818110610e2e57610e2e613df2565b905060800201604001609f6000848060400190610e4b9190614737565b85818110610e5b57610e5b613df2565b610e719260206080909202019081019150613728565b60ff1681526020808201929092526040016000208235815591013560019091015580610e9c816146d3565b915050610d5c565b5060005b610eb5606083018361479b565b9050811015610f6d57610ecb606083018361479b565b82818110610edb57610edb613df2565b9050604002016020016020810190610ef39190614780565b609d6000610f04606086018661479b565b85818110610f1457610f14613df2565b610f2a9260206040909202019081019150613728565b60ff168152602081019190915260400160002080546001600160601b0319166001600160601b039290921691909117905580610f65816146d3565b915050610ea8565b5060005b610f7e60808301836147e4565b905081101561101a57610f9460808301836147e4565b82818110610fa457610fa4613df2565b905060600201602001609f6000848060800190610fc191906147e4565b85818110610fd157610fd1613df2565b610fe79260206060909202019081019150613728565b60ff1681526020808201929092526040016000208235815591013560019091015580611012816146d3565b915050610f71565b5060005b61102b60a08301836146ee565b90508110156111515760005b609b805461104490613e2a565b90508110156110fd57609e600061105e60a08601866146ee565b8581811061106e5761106e613df2565b9050602002013581526020019081526020016000206000609b83815461109390613e2a565b81106110a1576110a1613df2565b8154600116156110c05790600052602060002090602091828204019190065b9054600160f81b911a0260f81c8152602081019190915260400160002080546001600160601b0319169055816110f5816146d3565b925050611037565b5060a0600061110e848301856146ee565b8481811061111e5761111e613df2565b60209081029290920135835250810191909152604001600020805460ff1916905580611149816146d3565b91505061101e565b5060005b61116260c08301836146ee565b90508110156113b25761117860c08301836146ee565b8281811061118857611188613df2565b905060200281019061119a919061482c565b6111ab906080810190606001613728565b60a060006111bc60c08601866146ee565b858181106111cc576111cc613df2565b90506020028101906111de919061482c565b60000135815260200190815260200160002060006101000a81548160ff021916908360ff16021790555060005b61121860c08401846146ee565b8381811061122857611228613df2565b905060200281019061123a919061482c565b6112489060208101906146ee565b905081101561139f5761125e60c08401846146ee565b8381811061126e5761126e613df2565b9050602002810190611280919061482c565b61128e9060408101906146ee565b8281811061129e5761129e613df2565b90506020020160208101906112b39190614780565b609e60006112c460c08701876146ee565b868181106112d4576112d4613df2565b90506020028101906112e6919061482c565b358152602081019190915260400160009081209061130760c08701876146ee565b8681811061131757611317613df2565b9050602002810190611329919061482c565b6113379060208101906146ee565b8581811061134757611347613df2565b905060200201602081019061135c9190613728565b60ff168152602081019190915260400160002080546001600160601b0319166001600160601b039290921691909117905580611397816146d3565b91505061120b565b50806113aa816146d3565b915050611155565b5060005b6113c360e08301836146ee565b90508110156115765760005b6113dc60e08401846146ee565b838181106113ec576113ec613df2565b90506020028101906113fe919061484c565b61140c9060208101906146ee565b90508110156115635761142260e08401846146ee565b8381811061143257611432613df2565b9050602002810190611444919061484c565b6114529060408101906146ee565b8281811061146257611462613df2565b90506020020160208101906114779190614780565b609e600061148860e08701876146ee565b8681811061149857611498613df2565b90506020028101906114aa919061484c565b35815260208101919091526040016000908120906114cb60e08701876146ee565b868181106114db576114db613df2565b90506020028101906114ed919061484c565b6114fb9060208101906146ee565b8581811061150b5761150b613df2565b90506020020160208101906115209190613728565b60ff168152602081019190915260400160002080546001600160601b0319166001600160601b03929092169190911790558061155b816146d3565b9150506113cf565b508061156e816146d3565b9150506113b6565b5060005b61158861010083018361479b565b905081101561162c5761159f61010083018361479b565b828181106115af576115af613df2565b90506040020160200160208101906115c79190613728565b60a060006115d961010086018661479b565b858181106115e9576115e9613df2565b90506040020160000135815260200190815260200160002060006101000a81548160ff021916908360ff1602179055508080611624906146d3565b91505061157a565b5060a08301356099556116426020850185613e73565b609a805463ffffffff191663ffffffff9290921691909117905561166c6060850160408601613e73565b609a805463ffffffff929092166401000000000267ffffffff0000000019909216919091179055426098556116a46080850185614862565b6116b091609b916135a4565b506116c160c0850160a08601613e73565b609c805463ffffffff191663ffffffff929092169190911790557f9a128fe7347f1e11ca22aa9deb632ec9abb09608c13a994c60f77a562f4601716117096020860186613e73565b6117196060870160408801613e73565b6040805163ffffffff9384168152929091166020830152015b60405180910390a15b50505050565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa158015611789573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906117ad9190613d8d565b6117c95760405162461bcd60e51b815260040161056f90613daa565b600019606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b611810612cc6565b61181a60006131bf565b565b600054610100900460ff161580801561183c5750600054600160ff909116105b806118565750303b158015611856575060005460ff166001145b6118b95760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b606482015260840161056f565b6000805460ff1916600117905580156118dc576000805461ff0019166101001790555b6118e7846000613211565b6118f0836131bf565b609780546001600160a01b0319166001600160a01b038416179055801561173b576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890602001611732565b604080518082019091526060808252602082015260408051808201909152600080825260208201819052609b805461198490613e2a565b905090506119a5604051806040016040528060608152602001606081525090565b816001600160401b038111156119bd576119bd613795565b6040519080825280602002602001820160405280156119e6578160200160208202803683370190505b506020820152816001600160401b03811115611a0457611a04613795565b604051908082528060200260200182016040528015611a2d578160200160208202803683370190505b5081528451516000906001600160401b03811115611a4d57611a4d613795565b604051908082528060200260200182016040528015611a76578160200160208202803683370190505b5090506000805b875151811015611c3157611abf88600001518281518110611aa057611aa0613df2565b6020026020010151805160009081526020918201519091526040902090565b838281518110611ad157611ad1613df2565b60209081029190910101528015611baf5782611aee6001836148a8565b81518110611afe57611afe613df2565b602002602001015160001c838281518110611b1b57611b1b613df2565b602002602001015160001c11611baf5760405162461bcd60e51b815260206004820152604d60248201527f424c535369676e6174757265436865636b65722e636865636b5369676e61747560448201527f7265733a206e6f6e5369676e657247315075626b657973466f724f6c6453746160648201526c1d19481b9bdd081cdbdc9d1959609a1b608482015260a40161056f565b611c1d611c1660a06000868581518110611bcb57611bcb613df2565b6020908102919091018101518252810191909152604001600020548a51805160ff9092169185908110611c0057611c00613df2565b60200260200101516132fb90919063ffffffff16565b8790612db7565b955080611c29816146d3565b915050611a7d565b50611c3b856133df565b945060005b84811015611e1c57609b818154611c5690613e2a565b8110611c6457611c64613df2565b815460011615611c835790600052602060002090602091828204019190065b9054600160f81b911a0260f81c6000818152609f60209081526040918290208251808401909352805483526001015490820152909250611cc4908790612db7565b60ff83166000908152609d60209081526040909120549086015180519298506001600160601b039091169183908110611cff57611cff613df2565b6001600160601b03909216602092830291909101820152840151805182908110611d2b57611d2b613df2565b602002602001015184600001518281518110611d4957611d49613df2565b60200260200101906001600160601b031690816001600160601b03168152505060005b885151811015611e0957609e6000858381518110611d8c57611d8c613df2565b6020908102919091018101518252818101929092526040908101600090812060ff87168252909252902054855180516001600160601b039092169184908110611dd757611dd7613df2565b60200260200101818151611deb91906148bf565b6001600160601b031690525080611e01816146d3565b915050611d6c565b5080611e14816146d3565b915050611c40565b50600080611e348a888b602001518c604001516106ed565b9150915081611eb75760405162461bcd60e51b815260206004820152604360248201527f424c535369676e6174757265436865636b65722e636865636b5369676e61747560448201527f7265733a2070616972696e6720707265636f6d70696c652063616c6c206661696064820152621b195960ea1b608482015260a40161056f565b80611f2a5760405162461bcd60e51b815260206004820152603960248201527f424c535369676e6174757265436865636b65722e636865636b5369676e61747560448201527f7265733a207369676e617475726520697320696e76616c696400000000000000606482015260840161056f565b5092955050505050505b92915050565b611f42612cc6565b60005b611f5260208301836146ee565b905081101561201757609d6000611f6c60208501856146ee565b84818110611f7c57611f7c613df2565b9050602002016020810190611f919190613728565b60ff168152602080820192909252604001600090812080546001600160601b0319169055609f91611fc4908501856146ee565b84818110611fd457611fd4613df2565b9050602002016020810190611fe99190613728565b60ff16815260208101919091526040016000908120818155600101558061200f816146d3565b915050611f45565b5060005b6120286040830183614737565b90508110156121635761203e6040830183614737565b8281811061204e5761204e613df2565b90506080020160200160208101906120669190614780565b609d60006120776040860186614737565b8581811061208757612087613df2565b61209d9260206080909202019081019150613728565b60ff1681526020810191909152604090810160002080546001600160601b0319166001600160601b0393909316929092179091556120dd90830183614737565b828181106120ed576120ed613df2565b905060800201604001609f600084806040019061210a9190614737565b8581811061211a5761211a613df2565b6121309260206080909202019081019150613728565b60ff168152602080820192909252604001600020823581559101356001909101558061215b816146d3565b91505061201b565b5060005b612174606083018361479b565b905081101561222c5761218a606083018361479b565b8281811061219a5761219a613df2565b90506040020160200160208101906121b29190614780565b609d60006121c3606086018661479b565b858181106121d3576121d3613df2565b6121e99260206040909202019081019150613728565b60ff168152602081019190915260400160002080546001600160601b0319166001600160601b039290921691909117905580612224816146d3565b915050612167565b5060005b61223d60808301836147e4565b90508110156122d95761225360808301836147e4565b8281811061226357612263613df2565b905060600201602001609f600084806080019061228091906147e4565b8581811061229057612290613df2565b6122a69260206060909202019081019150613728565b60ff16815260208082019290925260400160002082358155910135600190910155806122d1816146d3565b915050612230565b5060005b6122ea60a08301836146ee565b90508110156124105760005b609b805461230390613e2a565b90508110156123bc57609e600061231d60a08601866146ee565b8581811061232d5761232d613df2565b9050602002013581526020019081526020016000206000609b83815461235290613e2a565b811061236057612360613df2565b81546001161561237f5790600052602060002090602091828204019190065b9054600160f81b911a0260f81c8152602081019190915260400160002080546001600160601b0319169055816123b4816146d3565b9250506122f6565b5060a060006123cd848301856146ee565b848181106123dd576123dd613df2565b60209081029290920135835250810191909152604001600020805460ff1916905580612408816146d3565b9150506122dd565b5060005b61242160c08301836146ee565b90508110156126715761243760c08301836146ee565b8281811061244757612447613df2565b9050602002810190612459919061482c565b61246a906080810190606001613728565b60a0600061247b60c08601866146ee565b8581811061248b5761248b613df2565b905060200281019061249d919061482c565b60000135815260200190815260200160002060006101000a81548160ff021916908360ff16021790555060005b6124d760c08401846146ee565b838181106124e7576124e7613df2565b90506020028101906124f9919061482c565b6125079060208101906146ee565b905081101561265e5761251d60c08401846146ee565b8381811061252d5761252d613df2565b905060200281019061253f919061482c565b61254d9060408101906146ee565b8281811061255d5761255d613df2565b90506020020160208101906125729190614780565b609e600061258360c08701876146ee565b8681811061259357612593613df2565b90506020028101906125a5919061482c565b35815260208101919091526040016000908120906125c660c08701876146ee565b868181106125d6576125d6613df2565b90506020028101906125e8919061482c565b6125f69060208101906146ee565b8581811061260657612606613df2565b905060200201602081019061261b9190613728565b60ff168152602081019190915260400160002080546001600160601b0319166001600160601b039290921691909117905580612656816146d3565b9150506124ca565b5080612669816146d3565b915050612414565b5060005b61268260e08301836146ee565b90508110156128355760005b61269b60e08401846146ee565b838181106126ab576126ab613df2565b90506020028101906126bd919061484c565b6126cb9060208101906146ee565b9050811015612822576126e160e08401846146ee565b838181106126f1576126f1613df2565b9050602002810190612703919061484c565b6127119060408101906146ee565b8281811061272157612721613df2565b90506020020160208101906127369190614780565b609e600061274760e08701876146ee565b8681811061275757612757613df2565b9050602002810190612769919061484c565b358152602081019190915260400160009081209061278a60e08701876146ee565b8681811061279a5761279a613df2565b90506020028101906127ac919061484c565b6127ba9060208101906146ee565b858181106127ca576127ca613df2565b90506020020160208101906127df9190613728565b60ff168152602081019190915260400160002080546001600160601b0319166001600160601b03929092169190911790558061281a816146d3565b91505061268e565b508061282d816146d3565b915050612675565b5060005b61284761010083018361479b565b90508110156128eb5761285e61010083018361479b565b8281811061286e5761286e613df2565b90506040020160200160208101906128869190613728565b60a0600061289861010086018661479b565b858181106128a8576128a8613df2565b90506040020160000135815260200190815260200160002060006101000a81548160ff021916908360ff16021790555080806128e3906146d3565b915050612839565b5060a08201356099556129016020840184613e73565b609a805463ffffffff191663ffffffff9290921691909117905561292b6060840160408501613e73565b609a805463ffffffff929092166401000000000267ffffffff0000000019909216919091179055426098556129636080840184614862565b61296f91609b916135a4565b5061298060c0840160a08501613e73565b609c805463ffffffff191663ffffffff929092169190911790557f264965eb6bc436c6c473431d34af56e832ec344fdfd43ee6af6fce6d205e84af6129c86020850185613e73565b6129d86060860160408701613e73565b6040805163ffffffff93841681529290911660208301520160405180910390a1505050565b612a05612cc6565b6001600160a01b038116612a6a5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b606482015260840161056f565b610581816131bf565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612ac6573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612aea9190613d18565b6001600160a01b0316336001600160a01b031614612b1a5760405162461bcd60e51b815260040161056f90613d35565b606654198119606654191614612b985760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e756e70617573653a20696e76616c696420617474656d7060448201527f7420746f2070617573652066756e6374696f6e616c6974790000000000000000606482015260840161056f565b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c906020016106e2565b6001600160a01b038116612c5d5760405162461bcd60e51b815260206004820152604960248201527f5061757361626c652e5f73657450617573657252656769737472793a206e657760448201527f50617573657252656769737472792063616e6e6f7420626520746865207a65726064820152686f206164647265737360b81b608482015260a40161056f565b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b6033546001600160a01b0316331461181a5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604482015260640161056f565b6040805180820190915260008082526020820152612d3c613628565b835181526020808501519082015260408082018490526000908360608460076107d05a03fa9050808015612d6f57612d71565bfe5b5080612daf5760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b604482015260640161056f565b505092915050565b6040805180820190915260008082526020820152612dd3613646565b835181526020808501518183015283516040808401919091529084015160608301526000908360808460066107d05a03fa9050808015612d6f575080612daf5760405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b604482015260640161056f565b612e53613664565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b604080518082019091526000808252602082015260008080612f3b6000805160206148e883398151915286613e08565b90505b612f478161347a565b90935091506000805160206148e8833981519152828309831415612f81576040805180820190915290815260208101919091529392505050565b6000805160206148e8833981519152600182089050612f3e565b604080518082018252868152602080820186905282518084019093528683528201849052600091829190612fcd613689565b60005b6002811015613192576000612fe68260066146b4565b9050848260028110612ffa57612ffa613df2565b6020020151518361300c836000614614565b600c811061301c5761301c613df2565b602002015284826002811061303357613033613df2565b6020020151602001518382600161304a9190614614565b600c811061305a5761305a613df2565b602002015283826002811061307157613071613df2565b6020020151515183613084836002614614565b600c811061309457613094613df2565b60200201528382600281106130ab576130ab613df2565b60200201515160016020020151836130c4836003614614565b600c81106130d4576130d4613df2565b60200201528382600281106130eb576130eb613df2565b60200201516020015160006002811061310657613106613df2565b602002015183613117836004614614565b600c811061312757613127613df2565b602002015283826002811061313e5761313e613df2565b60200201516020015160016002811061315957613159613df2565b60200201518361316a836005614614565b600c811061317a5761317a613df2565b6020020152508061318a816146d3565b915050612fd0565b5061319b6136a8565b60006020826101808560088cfa9151919c9115159b50909950505050505050505050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6065546001600160a01b031615801561323257506001600160a01b03821615155b6132b45760405162461bcd60e51b815260206004820152604760248201527f5061757361626c652e5f696e697469616c697a655061757365723a205f696e6960448201527f7469616c697a6550617573657228292063616e206f6e6c792062652063616c6c6064820152666564206f6e636560c81b608482015260a40161056f565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a26132f782612bcf565b5050565b60408051808201909152600080825260208201526102008261ffff16106133575760405162461bcd60e51b815260206004820152601060248201526f7363616c61722d746f6f2d6c6172676560801b604482015260640161056f565b8161ffff166001141561336b575081611f34565b6040805180820190915260008082526020820181905284906001905b8161ffff168661ffff16106133d457600161ffff871660ff83161c811614156133b7576133b48484612db7565b93505b6133c18384612db7565b92506201fffe600192831b169101613387565b509195945050505050565b6040805180820190915260008082526020820152815115801561340457506020820151155b15613422575050604080518082019091526000808252602082015290565b6040518060400160405280836000015181526020016000805160206148e883398151915284602001516134559190613e08565b61346d906000805160206148e88339815191526148a8565b905292915050565b919050565b600080806000805160206148e883398151915260036000805160206148e8833981519152866000805160206148e88339815191528889090908905060006134f0827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f526000805160206148e88339815191526134fc565b91959194509092505050565b6000806135076136a8565b61350f6136c6565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa9250828015612d6f5750826135995760405162461bcd60e51b815260206004820152601a60248201527f424e3235342e6578704d6f643a2063616c6c206661696c757265000000000000604482015260640161056f565b505195945050505050565b8280546135b090613e2a565b90600052602060002090601f0160209004810192826135d25760008555613618565b82601f106135eb5782800160ff19823516178555613618565b82800160010185558215613618579182015b828111156136185782358255916020019190600101906135fd565b506136249291506136e4565b5090565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b60405180604001604052806136776136f9565b81526020016136846136f9565b905290565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b5b8082111561362457600081556001016136e5565b60405180604001604052806002906020820280368337509192915050565b803560ff8116811461347557600080fd5b60006020828403121561373a57600080fd5b61374382613717565b9392505050565b6001600160a01b038116811461058157600080fd5b60006020828403121561377157600080fd5b81356137438161374a565b60006020828403121561378e57600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b03811182821017156137cd576137cd613795565b60405290565b604051606081016001600160401b03811182821017156137cd576137cd613795565b604051601f8201601f191681016001600160401b038111828210171561381d5761381d613795565b604052919050565b60006040828403121561383757600080fd5b61383f6137ab565b9050813581526020820135602082015292915050565b600082601f83011261386657600080fd5b604051604081018181106001600160401b038211171561388857613888613795565b806040525080604084018581111561389f57600080fd5b845b818110156133d45780358352602092830192016138a1565b6000608082840312156138cb57600080fd5b6138d36137ab565b90506138df8383613855565b81526138ee8360408401613855565b602082015292915050565b600080600080610120858703121561391057600080fd5b843593506139218660208701613825565b925061393086606087016138b9565b915061393f8660e08701613825565b905092959194509250565b600060208083528351808285015260005b818110156139775785810183015185820160400152820161395b565b81811115613989576000604083870101525b50601f01601f1916929092016040019392505050565b600061010082840312156139b257600080fd5b50919050565b600060c082840312156139b257600080fd5b600061012082840312156139b257600080fd5b60008060008061012085870312156139f457600080fd5b84356001600160401b0380821115613a0b57600080fd5b613a178883890161399f565b9550613a2688602089016139b8565b945060e0870135915080821115613a3c57600080fd5b9086019060e08289031215613a5057600080fd5b9092506101008601359080821115613a6757600080fd5b50613a74878288016139ca565b91505092959194509250565b60008060408385031215613a9357600080fd5b82359150613aa360208401613717565b90509250929050565b600080600060608486031215613ac157600080fd5b8335613acc8161374a565b92506020840135613adc8161374a565b91506040840135613aec8161374a565b809150509250925092565b600060e08284031215613b0957600080fd5b613b116137d3565b905081356001600160401b0380821115613b2a57600080fd5b818401915084601f830112613b3e57600080fd5b8135602082821115613b5257613b52613795565b613b60818360051b016137f5565b828152818101935060069290921b840181019187831115613b8057600080fd5b938101935b82851015613ba957613b978886613825565b84528184019350604085019450613b85565b8552613bb7878783016138b9565b8186015250505050613bcc8360a08401613825565b604082015292915050565b60008060408385031215613bea57600080fd5b8235915060208301356001600160401b03811115613c0757600080fd5b613c1385828601613af7565b9150509250929050565b600081518084526020808501945080840160005b83811015613c565781516001600160601b031687529582019590820190600101613c31565b509495945050505050565b602081526000825160406020840152613c7d6060840182613c1d565b90506020840151601f19848303016040850152613c9a8282613c1d565b95945050505050565b60008060006101008486031215613cb957600080fd5b83356001600160401b0380821115613cd057600080fd5b613cdc8783880161399f565b9450613ceb87602088016139b8565b935060e0860135915080821115613d0157600080fd5b50613d0e868287016139ca565b9150509250925092565b600060208284031215613d2a57600080fd5b81516137438161374a565b6020808252602a908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526939903ab73830bab9b2b960b11b606082015260800190565b801515811461058157600080fd5b600060208284031215613d9f57600080fd5b815161374381613d7f565b60208082526028908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526739903830bab9b2b960c11b606082015260800190565b634e487b7160e01b600052603260045260246000fd5b600082613e2557634e487b7160e01b600052601260045260246000fd5b500690565b600181811c90821680613e3e57607f821691505b602082108114156139b257634e487b7160e01b600052602260045260246000fd5b803563ffffffff8116811461347557600080fd5b600060208284031215613e8557600080fd5b61374382613e5f565b6000808335601e19843603018112613ea557600080fd5b83016020810192503590506001600160401b03811115613ec457600080fd5b803603831315613ed357600080fd5b9250929050565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b6020815263ffffffff613f1583613e5f565b166020820152602082013560408201526000613f3360408401613e5f565b63ffffffff8116606084015250613f4c60608401613e5f565b63ffffffff8116608084015250613f666080840184613e8e565b6101008060a0860152613f7e61012086018385613eda565b9250613f8c60a08701613e5f565b63ffffffff811660c08701529150613fa760c0870187613e8e565b868503601f190160e08801529250613fc0848483613eda565b935050613fcf60e08701613e5f565b63ffffffff169401939093529392505050565b803561347581613d7f565b6000808335601e1984360301811261400457600080fd5b83016020810192503590506001600160401b0381111561402357600080fd5b8060051b3603831315613ed357600080fd5b8183526000602080850194508260005b85811015613c565760ff61405883613717565b1687529582019590820190600101614045565b6000808335601e1984360301811261408257600080fd5b83016020810192503590506001600160401b038111156140a157600080fd5b8060071b3603831315613ed357600080fd5b80356001600160601b038116811461347557600080fd5b8183526000602080850194508260005b85811015613c565760ff6140ed83613717565b1687526001600160601b036141038484016140b3565b16838801526040828101359088015260608083013590880152608096870196909101906001016140da565b6000808335601e1984360301811261414557600080fd5b83016020810192503590506001600160401b0381111561416457600080fd5b8060061b3603831315613ed357600080fd5b8183526000602080850194508260005b85811015613c565760ff61419983613717565b1687526001600160601b036141af8484016140b3565b16878401526040968701969190910190600101614186565b6000808335601e198436030181126141de57600080fd5b83016020810192503590506001600160401b038111156141fd57600080fd5b606081023603831315613ed357600080fd5b8183526000602080850194508260005b85811015613c565760ff61423283613717565b16875261424d83880184840180358252602090810135910152565b606096870196919091019060010161421f565b81835260006001600160fb1b0383111561427957600080fd5b8260051b8083602087013760009401602001938452509192915050565b8183526000602080850194508260005b85811015613c56576001600160601b036142bf836140b3565b16875295820195908201906001016142a6565b81835260006020808501808196508560051b81019150846000805b88811015614383578385038a528235607e1989360301811261430d578283fd5b880180358652608061432188830183613fed565b828a8a0152614333838a018284614035565b92505050604061434581840184613fed565b898403838b0152614357848284614296565b9350505050606060ff61436b828501613717565b169701969096525098850198918501916001016142ed565b509298975050505050505050565b81835260006020808501808196508560051b81019150846000805b88811015614383578385038a528235605e198936030181126143cc578283fd5b88018035865260606143e088830183613fed565b828a8a01526143f2838a018284614035565b92505050604061440481840184613fed565b9350888303828a0152614418838583614296565b9d8a019d985050509387019350506001016143ac565b8183526000602080850194508260005b85811015613c56578135875260ff614457848401613717565b1687840152604096870196919091019060010161443e565b602081526144896020820161448384613fe2565b15159052565b60006144986020840184613fed565b6101208060408601526144b061014086018385614035565b92506144bf604087018761406b565b9250601f19808786030160608801526144d98585846140ca565b94506144e8606089018961412e565b9450915080878603016080880152614501858584614176565b945061451060808901896141c7565b94509150808786030160a088015261452985858461420f565b945061453860a0890189613fed565b94509150808786030160c0880152614551858584614260565b945061456060c0890189613fed565b94509150808786030160e08801526145798585846142d2565b945061458860e0890189613fed565b945091506101008188870301818901526145a3868685614391565b95506145b1818a018a61412e565b9550925050808786030183880152506145cb84848361442e565b979650505050505050565b634e487b7160e01b600052601160045260246000fd5b600063ffffffff80831681851680830382111561460b5761460b6145d6565b01949350505050565b60008219821115614627576146276145d6565b500190565b60c0810163ffffffff61463e84613e5f565b1682526020830135602083015260408301356040830152606083013560608301526080830135608083015260a083013560a083015292915050565b6000611f343683613af7565b60006001600160601b03808316818516818304811182151516156146ab576146ab6145d6565b02949350505050565b60008160001904831182151516156146ce576146ce6145d6565b500290565b60006000198214156146e7576146e76145d6565b5060010190565b6000808335601e1984360301811261470557600080fd5b8301803591506001600160401b0382111561471f57600080fd5b6020019150600581901b3603821315613ed357600080fd5b6000808335601e1984360301811261474e57600080fd5b8301803591506001600160401b0382111561476857600080fd5b6020019150600781901b3603821315613ed357600080fd5b60006020828403121561479257600080fd5b613743826140b3565b6000808335601e198436030181126147b257600080fd5b8301803591506001600160401b038211156147cc57600080fd5b6020019150600681901b3603821315613ed357600080fd5b6000808335601e198436030181126147fb57600080fd5b8301803591506001600160401b0382111561481557600080fd5b6020019150606081023603821315613ed357600080fd5b60008235607e1983360301811261484257600080fd5b9190910192915050565b60008235605e1983360301811261484257600080fd5b6000808335601e1984360301811261487957600080fd5b8301803591506001600160401b0382111561489357600080fd5b602001915036819003821315613ed357600080fd5b6000828210156148ba576148ba6145d6565b500390565b60006001600160601b03838116908316818110156148df576148df6145d6565b03939250505056fe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a26469706673582212203b406214b292f3787665f457c420877824b47c9fd55bf14af9f989e1c7f5e43d64736f6c634300080c0033",
}

// ContractGaspMultiRollupServiceABI is the input ABI used to generate the binding from.
// Deprecated: Use ContractGaspMultiRollupServiceMetaData.ABI instead.
var ContractGaspMultiRollupServiceABI = ContractGaspMultiRollupServiceMetaData.ABI

// ContractGaspMultiRollupServiceBin is the compiled bytecode used for deploying new contracts.
// Deprecated: Use ContractGaspMultiRollupServiceMetaData.Bin instead.
var ContractGaspMultiRollupServiceBin = ContractGaspMultiRollupServiceMetaData.Bin

// DeployContractGaspMultiRollupService deploys a new Ethereum contract, binding an instance of ContractGaspMultiRollupService to it.
func DeployContractGaspMultiRollupService(auth *bind.TransactOpts, backend bind.ContractBackend) (common.Address, *types.Transaction, *ContractGaspMultiRollupService, error) {
	parsed, err := ContractGaspMultiRollupServiceMetaData.GetAbi()
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	if parsed == nil {
		return common.Address{}, nil, nil, errors.New("GetABI returned nil")
	}

	address, tx, contract, err := bind.DeployContract(auth, *parsed, common.FromHex(ContractGaspMultiRollupServiceBin), backend)
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	return address, tx, &ContractGaspMultiRollupService{ContractGaspMultiRollupServiceCaller: ContractGaspMultiRollupServiceCaller{contract: contract}, ContractGaspMultiRollupServiceTransactor: ContractGaspMultiRollupServiceTransactor{contract: contract}, ContractGaspMultiRollupServiceFilterer: ContractGaspMultiRollupServiceFilterer{contract: contract}}, nil
}

// ContractGaspMultiRollupService is an auto generated Go binding around an Ethereum contract.
type ContractGaspMultiRollupService struct {
	ContractGaspMultiRollupServiceCaller     // Read-only binding to the contract
	ContractGaspMultiRollupServiceTransactor // Write-only binding to the contract
	ContractGaspMultiRollupServiceFilterer   // Log filterer for contract events
}

// ContractGaspMultiRollupServiceCaller is an auto generated read-only Go binding around an Ethereum contract.
type ContractGaspMultiRollupServiceCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractGaspMultiRollupServiceTransactor is an auto generated write-only Go binding around an Ethereum contract.
type ContractGaspMultiRollupServiceTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractGaspMultiRollupServiceFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type ContractGaspMultiRollupServiceFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractGaspMultiRollupServiceSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type ContractGaspMultiRollupServiceSession struct {
	Contract     *ContractGaspMultiRollupService // Generic contract binding to set the session for
	CallOpts     bind.CallOpts                   // Call options to use throughout this session
	TransactOpts bind.TransactOpts               // Transaction auth options to use throughout this session
}

// ContractGaspMultiRollupServiceCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type ContractGaspMultiRollupServiceCallerSession struct {
	Contract *ContractGaspMultiRollupServiceCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts                         // Call options to use throughout this session
}

// ContractGaspMultiRollupServiceTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type ContractGaspMultiRollupServiceTransactorSession struct {
	Contract     *ContractGaspMultiRollupServiceTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts                         // Transaction auth options to use throughout this session
}

// ContractGaspMultiRollupServiceRaw is an auto generated low-level Go binding around an Ethereum contract.
type ContractGaspMultiRollupServiceRaw struct {
	Contract *ContractGaspMultiRollupService // Generic contract binding to access the raw methods on
}

// ContractGaspMultiRollupServiceCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type ContractGaspMultiRollupServiceCallerRaw struct {
	Contract *ContractGaspMultiRollupServiceCaller // Generic read-only contract binding to access the raw methods on
}

// ContractGaspMultiRollupServiceTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type ContractGaspMultiRollupServiceTransactorRaw struct {
	Contract *ContractGaspMultiRollupServiceTransactor // Generic write-only contract binding to access the raw methods on
}

// NewContractGaspMultiRollupService creates a new instance of ContractGaspMultiRollupService, bound to a specific deployed contract.
func NewContractGaspMultiRollupService(address common.Address, backend bind.ContractBackend) (*ContractGaspMultiRollupService, error) {
	contract, err := bindContractGaspMultiRollupService(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupService{ContractGaspMultiRollupServiceCaller: ContractGaspMultiRollupServiceCaller{contract: contract}, ContractGaspMultiRollupServiceTransactor: ContractGaspMultiRollupServiceTransactor{contract: contract}, ContractGaspMultiRollupServiceFilterer: ContractGaspMultiRollupServiceFilterer{contract: contract}}, nil
}

// NewContractGaspMultiRollupServiceCaller creates a new read-only instance of ContractGaspMultiRollupService, bound to a specific deployed contract.
func NewContractGaspMultiRollupServiceCaller(address common.Address, caller bind.ContractCaller) (*ContractGaspMultiRollupServiceCaller, error) {
	contract, err := bindContractGaspMultiRollupService(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupServiceCaller{contract: contract}, nil
}

// NewContractGaspMultiRollupServiceTransactor creates a new write-only instance of ContractGaspMultiRollupService, bound to a specific deployed contract.
func NewContractGaspMultiRollupServiceTransactor(address common.Address, transactor bind.ContractTransactor) (*ContractGaspMultiRollupServiceTransactor, error) {
	contract, err := bindContractGaspMultiRollupService(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupServiceTransactor{contract: contract}, nil
}

// NewContractGaspMultiRollupServiceFilterer creates a new log filterer instance of ContractGaspMultiRollupService, bound to a specific deployed contract.
func NewContractGaspMultiRollupServiceFilterer(address common.Address, filterer bind.ContractFilterer) (*ContractGaspMultiRollupServiceFilterer, error) {
	contract, err := bindContractGaspMultiRollupService(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupServiceFilterer{contract: contract}, nil
}

// bindContractGaspMultiRollupService binds a generic wrapper to an already deployed contract.
func bindContractGaspMultiRollupService(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := ContractGaspMultiRollupServiceMetaData.GetAbi()
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, *parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _ContractGaspMultiRollupService.Contract.ContractGaspMultiRollupServiceCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.ContractGaspMultiRollupServiceTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.ContractGaspMultiRollupServiceTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _ContractGaspMultiRollupService.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.contract.Transact(opts, method, params...)
}

// CheckSignatures is a free data retrieval call binding the contract method 0xc4e1914c.
//
// Solidity: function checkSignatures(bytes32 msgHash, ((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)) params) view returns((uint96[],uint96[]))
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) CheckSignatures(opts *bind.CallOpts, msgHash [32]byte, params IGaspMultiRollupServicePrimitivesNonSignerStakesAndSignatureForOldState) (IBLSSignatureCheckerQuorumStakeTotals, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "checkSignatures", msgHash, params)

	if err != nil {
		return *new(IBLSSignatureCheckerQuorumStakeTotals), err
	}

	out0 := *abi.ConvertType(out[0], new(IBLSSignatureCheckerQuorumStakeTotals)).(*IBLSSignatureCheckerQuorumStakeTotals)

	return out0, err

}

// CheckSignatures is a free data retrieval call binding the contract method 0xc4e1914c.
//
// Solidity: function checkSignatures(bytes32 msgHash, ((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)) params) view returns((uint96[],uint96[]))
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) CheckSignatures(msgHash [32]byte, params IGaspMultiRollupServicePrimitivesNonSignerStakesAndSignatureForOldState) (IBLSSignatureCheckerQuorumStakeTotals, error) {
	return _ContractGaspMultiRollupService.Contract.CheckSignatures(&_ContractGaspMultiRollupService.CallOpts, msgHash, params)
}

// CheckSignatures is a free data retrieval call binding the contract method 0xc4e1914c.
//
// Solidity: function checkSignatures(bytes32 msgHash, ((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)) params) view returns((uint96[],uint96[]))
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) CheckSignatures(msgHash [32]byte, params IGaspMultiRollupServicePrimitivesNonSignerStakesAndSignatureForOldState) (IBLSSignatureCheckerQuorumStakeTotals, error) {
	return _ContractGaspMultiRollupService.Contract.CheckSignatures(&_ContractGaspMultiRollupService.CallOpts, msgHash, params)
}

// LastUpdateBlockTimestamp is a free data retrieval call binding the contract method 0xe61db175.
//
// Solidity: function lastUpdateBlockTimestamp() view returns(uint256)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) LastUpdateBlockTimestamp(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "lastUpdateBlockTimestamp")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// LastUpdateBlockTimestamp is a free data retrieval call binding the contract method 0xe61db175.
//
// Solidity: function lastUpdateBlockTimestamp() view returns(uint256)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) LastUpdateBlockTimestamp() (*big.Int, error) {
	return _ContractGaspMultiRollupService.Contract.LastUpdateBlockTimestamp(&_ContractGaspMultiRollupService.CallOpts)
}

// LastUpdateBlockTimestamp is a free data retrieval call binding the contract method 0xe61db175.
//
// Solidity: function lastUpdateBlockTimestamp() view returns(uint256)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) LastUpdateBlockTimestamp() (*big.Int, error) {
	return _ContractGaspMultiRollupService.Contract.LastUpdateBlockTimestamp(&_ContractGaspMultiRollupService.CallOpts)
}

// LatestCompletedTaskCreatedBlock is a free data retrieval call binding the contract method 0xed5a04fe.
//
// Solidity: function latestCompletedTaskCreatedBlock() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) LatestCompletedTaskCreatedBlock(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "latestCompletedTaskCreatedBlock")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// LatestCompletedTaskCreatedBlock is a free data retrieval call binding the contract method 0xed5a04fe.
//
// Solidity: function latestCompletedTaskCreatedBlock() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) LatestCompletedTaskCreatedBlock() (uint32, error) {
	return _ContractGaspMultiRollupService.Contract.LatestCompletedTaskCreatedBlock(&_ContractGaspMultiRollupService.CallOpts)
}

// LatestCompletedTaskCreatedBlock is a free data retrieval call binding the contract method 0xed5a04fe.
//
// Solidity: function latestCompletedTaskCreatedBlock() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) LatestCompletedTaskCreatedBlock() (uint32, error) {
	return _ContractGaspMultiRollupService.Contract.LatestCompletedTaskCreatedBlock(&_ContractGaspMultiRollupService.CallOpts)
}

// LatestCompletedTaskNumber is a free data retrieval call binding the contract method 0xfc765dd5.
//
// Solidity: function latestCompletedTaskNumber() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) LatestCompletedTaskNumber(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "latestCompletedTaskNumber")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// LatestCompletedTaskNumber is a free data retrieval call binding the contract method 0xfc765dd5.
//
// Solidity: function latestCompletedTaskNumber() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) LatestCompletedTaskNumber() (uint32, error) {
	return _ContractGaspMultiRollupService.Contract.LatestCompletedTaskNumber(&_ContractGaspMultiRollupService.CallOpts)
}

// LatestCompletedTaskNumber is a free data retrieval call binding the contract method 0xfc765dd5.
//
// Solidity: function latestCompletedTaskNumber() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) LatestCompletedTaskNumber() (uint32, error) {
	return _ContractGaspMultiRollupService.Contract.LatestCompletedTaskNumber(&_ContractGaspMultiRollupService.CallOpts)
}

// LatestPendingStateHash is a free data retrieval call binding the contract method 0x4ae6b203.
//
// Solidity: function latestPendingStateHash() view returns(bytes32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) LatestPendingStateHash(opts *bind.CallOpts) ([32]byte, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "latestPendingStateHash")

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// LatestPendingStateHash is a free data retrieval call binding the contract method 0x4ae6b203.
//
// Solidity: function latestPendingStateHash() view returns(bytes32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) LatestPendingStateHash() ([32]byte, error) {
	return _ContractGaspMultiRollupService.Contract.LatestPendingStateHash(&_ContractGaspMultiRollupService.CallOpts)
}

// LatestPendingStateHash is a free data retrieval call binding the contract method 0x4ae6b203.
//
// Solidity: function latestPendingStateHash() view returns(bytes32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) LatestPendingStateHash() ([32]byte, error) {
	return _ContractGaspMultiRollupService.Contract.LatestPendingStateHash(&_ContractGaspMultiRollupService.CallOpts)
}

// OperatorAndQuorumToStakes is a free data retrieval call binding the contract method 0x499d6fb6.
//
// Solidity: function operatorAndQuorumToStakes(bytes32 , uint8 ) view returns(uint96)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) OperatorAndQuorumToStakes(opts *bind.CallOpts, arg0 [32]byte, arg1 uint8) (*big.Int, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "operatorAndQuorumToStakes", arg0, arg1)

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// OperatorAndQuorumToStakes is a free data retrieval call binding the contract method 0x499d6fb6.
//
// Solidity: function operatorAndQuorumToStakes(bytes32 , uint8 ) view returns(uint96)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) OperatorAndQuorumToStakes(arg0 [32]byte, arg1 uint8) (*big.Int, error) {
	return _ContractGaspMultiRollupService.Contract.OperatorAndQuorumToStakes(&_ContractGaspMultiRollupService.CallOpts, arg0, arg1)
}

// OperatorAndQuorumToStakes is a free data retrieval call binding the contract method 0x499d6fb6.
//
// Solidity: function operatorAndQuorumToStakes(bytes32 , uint8 ) view returns(uint96)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) OperatorAndQuorumToStakes(arg0 [32]byte, arg1 uint8) (*big.Int, error) {
	return _ContractGaspMultiRollupService.Contract.OperatorAndQuorumToStakes(&_ContractGaspMultiRollupService.CallOpts, arg0, arg1)
}

// OperatorIdQuorumCount is a free data retrieval call binding the contract method 0x430d3b39.
//
// Solidity: function operatorIdQuorumCount(bytes32 ) view returns(uint8)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) OperatorIdQuorumCount(opts *bind.CallOpts, arg0 [32]byte) (uint8, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "operatorIdQuorumCount", arg0)

	if err != nil {
		return *new(uint8), err
	}

	out0 := *abi.ConvertType(out[0], new(uint8)).(*uint8)

	return out0, err

}

// OperatorIdQuorumCount is a free data retrieval call binding the contract method 0x430d3b39.
//
// Solidity: function operatorIdQuorumCount(bytes32 ) view returns(uint8)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) OperatorIdQuorumCount(arg0 [32]byte) (uint8, error) {
	return _ContractGaspMultiRollupService.Contract.OperatorIdQuorumCount(&_ContractGaspMultiRollupService.CallOpts, arg0)
}

// OperatorIdQuorumCount is a free data retrieval call binding the contract method 0x430d3b39.
//
// Solidity: function operatorIdQuorumCount(bytes32 ) view returns(uint8)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) OperatorIdQuorumCount(arg0 [32]byte) (uint8, error) {
	return _ContractGaspMultiRollupService.Contract.OperatorIdQuorumCount(&_ContractGaspMultiRollupService.CallOpts, arg0)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) Owner(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "owner")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) Owner() (common.Address, error) {
	return _ContractGaspMultiRollupService.Contract.Owner(&_ContractGaspMultiRollupService.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) Owner() (common.Address, error) {
	return _ContractGaspMultiRollupService.Contract.Owner(&_ContractGaspMultiRollupService.CallOpts)
}

// Paused is a free data retrieval call binding the contract method 0x5ac86ab7.
//
// Solidity: function paused(uint8 index) view returns(bool)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) Paused(opts *bind.CallOpts, index uint8) (bool, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "paused", index)

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// Paused is a free data retrieval call binding the contract method 0x5ac86ab7.
//
// Solidity: function paused(uint8 index) view returns(bool)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) Paused(index uint8) (bool, error) {
	return _ContractGaspMultiRollupService.Contract.Paused(&_ContractGaspMultiRollupService.CallOpts, index)
}

// Paused is a free data retrieval call binding the contract method 0x5ac86ab7.
//
// Solidity: function paused(uint8 index) view returns(bool)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) Paused(index uint8) (bool, error) {
	return _ContractGaspMultiRollupService.Contract.Paused(&_ContractGaspMultiRollupService.CallOpts, index)
}

// Paused0 is a free data retrieval call binding the contract method 0x5c975abb.
//
// Solidity: function paused() view returns(uint256)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) Paused0(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "paused0")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// Paused0 is a free data retrieval call binding the contract method 0x5c975abb.
//
// Solidity: function paused() view returns(uint256)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) Paused0() (*big.Int, error) {
	return _ContractGaspMultiRollupService.Contract.Paused0(&_ContractGaspMultiRollupService.CallOpts)
}

// Paused0 is a free data retrieval call binding the contract method 0x5c975abb.
//
// Solidity: function paused() view returns(uint256)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) Paused0() (*big.Int, error) {
	return _ContractGaspMultiRollupService.Contract.Paused0(&_ContractGaspMultiRollupService.CallOpts)
}

// PauserRegistry is a free data retrieval call binding the contract method 0x886f1195.
//
// Solidity: function pauserRegistry() view returns(address)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) PauserRegistry(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "pauserRegistry")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// PauserRegistry is a free data retrieval call binding the contract method 0x886f1195.
//
// Solidity: function pauserRegistry() view returns(address)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) PauserRegistry() (common.Address, error) {
	return _ContractGaspMultiRollupService.Contract.PauserRegistry(&_ContractGaspMultiRollupService.CallOpts)
}

// PauserRegistry is a free data retrieval call binding the contract method 0x886f1195.
//
// Solidity: function pauserRegistry() view returns(address)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) PauserRegistry() (common.Address, error) {
	return _ContractGaspMultiRollupService.Contract.PauserRegistry(&_ContractGaspMultiRollupService.CallOpts)
}

// QourumApk is a free data retrieval call binding the contract method 0x03d097d2.
//
// Solidity: function qourumApk(uint8 ) view returns(uint256 X, uint256 Y)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) QourumApk(opts *bind.CallOpts, arg0 uint8) (struct {
	X *big.Int
	Y *big.Int
}, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "qourumApk", arg0)

	outstruct := new(struct {
		X *big.Int
		Y *big.Int
	})
	if err != nil {
		return *outstruct, err
	}

	outstruct.X = *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)
	outstruct.Y = *abi.ConvertType(out[1], new(*big.Int)).(**big.Int)

	return *outstruct, err

}

// QourumApk is a free data retrieval call binding the contract method 0x03d097d2.
//
// Solidity: function qourumApk(uint8 ) view returns(uint256 X, uint256 Y)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) QourumApk(arg0 uint8) (struct {
	X *big.Int
	Y *big.Int
}, error) {
	return _ContractGaspMultiRollupService.Contract.QourumApk(&_ContractGaspMultiRollupService.CallOpts, arg0)
}

// QourumApk is a free data retrieval call binding the contract method 0x03d097d2.
//
// Solidity: function qourumApk(uint8 ) view returns(uint256 X, uint256 Y)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) QourumApk(arg0 uint8) (struct {
	X *big.Int
	Y *big.Int
}, error) {
	return _ContractGaspMultiRollupService.Contract.QourumApk(&_ContractGaspMultiRollupService.CallOpts, arg0)
}

// QuorumNumbers is a free data retrieval call binding the contract method 0x2a8414fd.
//
// Solidity: function quorumNumbers() view returns(bytes)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) QuorumNumbers(opts *bind.CallOpts) ([]byte, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "quorumNumbers")

	if err != nil {
		return *new([]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([]byte)).(*[]byte)

	return out0, err

}

// QuorumNumbers is a free data retrieval call binding the contract method 0x2a8414fd.
//
// Solidity: function quorumNumbers() view returns(bytes)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) QuorumNumbers() ([]byte, error) {
	return _ContractGaspMultiRollupService.Contract.QuorumNumbers(&_ContractGaspMultiRollupService.CallOpts)
}

// QuorumNumbers is a free data retrieval call binding the contract method 0x2a8414fd.
//
// Solidity: function quorumNumbers() view returns(bytes)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) QuorumNumbers() ([]byte, error) {
	return _ContractGaspMultiRollupService.Contract.QuorumNumbers(&_ContractGaspMultiRollupService.CallOpts)
}

// QuorumThresholdPercentage is a free data retrieval call binding the contract method 0x4deabc21.
//
// Solidity: function quorumThresholdPercentage() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) QuorumThresholdPercentage(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "quorumThresholdPercentage")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// QuorumThresholdPercentage is a free data retrieval call binding the contract method 0x4deabc21.
//
// Solidity: function quorumThresholdPercentage() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) QuorumThresholdPercentage() (uint32, error) {
	return _ContractGaspMultiRollupService.Contract.QuorumThresholdPercentage(&_ContractGaspMultiRollupService.CallOpts)
}

// QuorumThresholdPercentage is a free data retrieval call binding the contract method 0x4deabc21.
//
// Solidity: function quorumThresholdPercentage() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) QuorumThresholdPercentage() (uint32, error) {
	return _ContractGaspMultiRollupService.Contract.QuorumThresholdPercentage(&_ContractGaspMultiRollupService.CallOpts)
}

// QuorumToStakes is a free data retrieval call binding the contract method 0x7ad75561.
//
// Solidity: function quorumToStakes(uint8 ) view returns(uint96)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) QuorumToStakes(opts *bind.CallOpts, arg0 uint8) (*big.Int, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "quorumToStakes", arg0)

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// QuorumToStakes is a free data retrieval call binding the contract method 0x7ad75561.
//
// Solidity: function quorumToStakes(uint8 ) view returns(uint96)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) QuorumToStakes(arg0 uint8) (*big.Int, error) {
	return _ContractGaspMultiRollupService.Contract.QuorumToStakes(&_ContractGaspMultiRollupService.CallOpts, arg0)
}

// QuorumToStakes is a free data retrieval call binding the contract method 0x7ad75561.
//
// Solidity: function quorumToStakes(uint8 ) view returns(uint96)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) QuorumToStakes(arg0 uint8) (*big.Int, error) {
	return _ContractGaspMultiRollupService.Contract.QuorumToStakes(&_ContractGaspMultiRollupService.CallOpts, arg0)
}

// Stalled is a free data retrieval call binding the contract method 0x526e3e64.
//
// Solidity: function stalled() view returns(bool)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) Stalled(opts *bind.CallOpts) (bool, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "stalled")

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// Stalled is a free data retrieval call binding the contract method 0x526e3e64.
//
// Solidity: function stalled() view returns(bool)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) Stalled() (bool, error) {
	return _ContractGaspMultiRollupService.Contract.Stalled(&_ContractGaspMultiRollupService.CallOpts)
}

// Stalled is a free data retrieval call binding the contract method 0x526e3e64.
//
// Solidity: function stalled() view returns(bool)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) Stalled() (bool, error) {
	return _ContractGaspMultiRollupService.Contract.Stalled(&_ContractGaspMultiRollupService.CallOpts)
}

// TrySignatureAndApkVerification is a free data retrieval call binding the contract method 0x171f1d5b.
//
// Solidity: function trySignatureAndApkVerification(bytes32 msgHash, (uint256,uint256) apk, (uint256[2],uint256[2]) apkG2, (uint256,uint256) sigma) view returns(bool pairingSuccessful, bool siganatureIsValid)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) TrySignatureAndApkVerification(opts *bind.CallOpts, msgHash [32]byte, apk BN254G1Point, apkG2 BN254G2Point, sigma BN254G1Point) (struct {
	PairingSuccessful bool
	SiganatureIsValid bool
}, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "trySignatureAndApkVerification", msgHash, apk, apkG2, sigma)

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
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) TrySignatureAndApkVerification(msgHash [32]byte, apk BN254G1Point, apkG2 BN254G2Point, sigma BN254G1Point) (struct {
	PairingSuccessful bool
	SiganatureIsValid bool
}, error) {
	return _ContractGaspMultiRollupService.Contract.TrySignatureAndApkVerification(&_ContractGaspMultiRollupService.CallOpts, msgHash, apk, apkG2, sigma)
}

// TrySignatureAndApkVerification is a free data retrieval call binding the contract method 0x171f1d5b.
//
// Solidity: function trySignatureAndApkVerification(bytes32 msgHash, (uint256,uint256) apk, (uint256[2],uint256[2]) apkG2, (uint256,uint256) sigma) view returns(bool pairingSuccessful, bool siganatureIsValid)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) TrySignatureAndApkVerification(msgHash [32]byte, apk BN254G1Point, apkG2 BN254G2Point, sigma BN254G1Point) (struct {
	PairingSuccessful bool
	SiganatureIsValid bool
}, error) {
	return _ContractGaspMultiRollupService.Contract.TrySignatureAndApkVerification(&_ContractGaspMultiRollupService.CallOpts, msgHash, apk, apkG2, sigma)
}

// Updater is a free data retrieval call binding the contract method 0xdf034cd0.
//
// Solidity: function updater() view returns(address)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) Updater(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "updater")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Updater is a free data retrieval call binding the contract method 0xdf034cd0.
//
// Solidity: function updater() view returns(address)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) Updater() (common.Address, error) {
	return _ContractGaspMultiRollupService.Contract.Updater(&_ContractGaspMultiRollupService.CallOpts)
}

// Updater is a free data retrieval call binding the contract method 0xdf034cd0.
//
// Solidity: function updater() view returns(address)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) Updater() (common.Address, error) {
	return _ContractGaspMultiRollupService.Contract.Updater(&_ContractGaspMultiRollupService.CallOpts)
}

// Initialize is a paid mutator transaction binding the contract method 0xc0c53b8b.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _updater) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) Initialize(opts *bind.TransactOpts, _pauserRegistry common.Address, initialOwner common.Address, _updater common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "initialize", _pauserRegistry, initialOwner, _updater)
}

// Initialize is a paid mutator transaction binding the contract method 0xc0c53b8b.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _updater) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) Initialize(_pauserRegistry common.Address, initialOwner common.Address, _updater common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.Initialize(&_ContractGaspMultiRollupService.TransactOpts, _pauserRegistry, initialOwner, _updater)
}

// Initialize is a paid mutator transaction binding the contract method 0xc0c53b8b.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _updater) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) Initialize(_pauserRegistry common.Address, initialOwner common.Address, _updater common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.Initialize(&_ContractGaspMultiRollupService.TransactOpts, _pauserRegistry, initialOwner, _updater)
}

// Pause is a paid mutator transaction binding the contract method 0x136439dd.
//
// Solidity: function pause(uint256 newPausedStatus) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) Pause(opts *bind.TransactOpts, newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "pause", newPausedStatus)
}

// Pause is a paid mutator transaction binding the contract method 0x136439dd.
//
// Solidity: function pause(uint256 newPausedStatus) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) Pause(newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.Pause(&_ContractGaspMultiRollupService.TransactOpts, newPausedStatus)
}

// Pause is a paid mutator transaction binding the contract method 0x136439dd.
//
// Solidity: function pause(uint256 newPausedStatus) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) Pause(newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.Pause(&_ContractGaspMultiRollupService.TransactOpts, newPausedStatus)
}

// PauseAll is a paid mutator transaction binding the contract method 0x595c6a67.
//
// Solidity: function pauseAll() returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) PauseAll(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "pauseAll")
}

// PauseAll is a paid mutator transaction binding the contract method 0x595c6a67.
//
// Solidity: function pauseAll() returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) PauseAll() (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.PauseAll(&_ContractGaspMultiRollupService.TransactOpts)
}

// PauseAll is a paid mutator transaction binding the contract method 0x595c6a67.
//
// Solidity: function pauseAll() returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) PauseAll() (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.PauseAll(&_ContractGaspMultiRollupService.TransactOpts)
}

// ProcessEigenReinit is a paid mutator transaction binding the contract method 0xc927feef.
//
// Solidity: function process_eigen_reinit((uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32,bytes32,bytes32,bytes32) taskResponse, (bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) operatorStateInfo) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) ProcessEigenReinit(opts *bind.TransactOpts, task IFinalizerTaskManagerTask, taskResponse IFinalizerTaskManagerTaskResponse, operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "process_eigen_reinit", task, taskResponse, operatorStateInfo)
}

// ProcessEigenReinit is a paid mutator transaction binding the contract method 0xc927feef.
//
// Solidity: function process_eigen_reinit((uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32,bytes32,bytes32,bytes32) taskResponse, (bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) operatorStateInfo) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) ProcessEigenReinit(task IFinalizerTaskManagerTask, taskResponse IFinalizerTaskManagerTaskResponse, operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.ProcessEigenReinit(&_ContractGaspMultiRollupService.TransactOpts, task, taskResponse, operatorStateInfo)
}

// ProcessEigenReinit is a paid mutator transaction binding the contract method 0xc927feef.
//
// Solidity: function process_eigen_reinit((uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32,bytes32,bytes32,bytes32) taskResponse, (bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) operatorStateInfo) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) ProcessEigenReinit(task IFinalizerTaskManagerTask, taskResponse IFinalizerTaskManagerTaskResponse, operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.ProcessEigenReinit(&_ContractGaspMultiRollupService.TransactOpts, task, taskResponse, operatorStateInfo)
}

// ProcessEigenUpdate is a paid mutator transaction binding the contract method 0x3e404926.
//
// Solidity: function process_eigen_update((uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32,bytes32,bytes32,bytes32) taskResponse, ((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)) nonSignerStakesAndSignatureForOldState, (bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) operatorStateInfo) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) ProcessEigenUpdate(opts *bind.TransactOpts, task IFinalizerTaskManagerTask, taskResponse IFinalizerTaskManagerTaskResponse, nonSignerStakesAndSignatureForOldState IGaspMultiRollupServicePrimitivesNonSignerStakesAndSignatureForOldState, operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "process_eigen_update", task, taskResponse, nonSignerStakesAndSignatureForOldState, operatorStateInfo)
}

// ProcessEigenUpdate is a paid mutator transaction binding the contract method 0x3e404926.
//
// Solidity: function process_eigen_update((uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32,bytes32,bytes32,bytes32) taskResponse, ((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)) nonSignerStakesAndSignatureForOldState, (bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) operatorStateInfo) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) ProcessEigenUpdate(task IFinalizerTaskManagerTask, taskResponse IFinalizerTaskManagerTaskResponse, nonSignerStakesAndSignatureForOldState IGaspMultiRollupServicePrimitivesNonSignerStakesAndSignatureForOldState, operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.ProcessEigenUpdate(&_ContractGaspMultiRollupService.TransactOpts, task, taskResponse, nonSignerStakesAndSignatureForOldState, operatorStateInfo)
}

// ProcessEigenUpdate is a paid mutator transaction binding the contract method 0x3e404926.
//
// Solidity: function process_eigen_update((uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32,bytes32,bytes32,bytes32) taskResponse, ((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)) nonSignerStakesAndSignatureForOldState, (bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) operatorStateInfo) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) ProcessEigenUpdate(task IFinalizerTaskManagerTask, taskResponse IFinalizerTaskManagerTaskResponse, nonSignerStakesAndSignatureForOldState IGaspMultiRollupServicePrimitivesNonSignerStakesAndSignatureForOldState, operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.ProcessEigenUpdate(&_ContractGaspMultiRollupService.TransactOpts, task, taskResponse, nonSignerStakesAndSignatureForOldState, operatorStateInfo)
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) RenounceOwnership(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "renounceOwnership")
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) RenounceOwnership() (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.RenounceOwnership(&_ContractGaspMultiRollupService.TransactOpts)
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) RenounceOwnership() (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.RenounceOwnership(&_ContractGaspMultiRollupService.TransactOpts)
}

// SetPauserRegistry is a paid mutator transaction binding the contract method 0x10d67a2f.
//
// Solidity: function setPauserRegistry(address newPauserRegistry) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) SetPauserRegistry(opts *bind.TransactOpts, newPauserRegistry common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "setPauserRegistry", newPauserRegistry)
}

// SetPauserRegistry is a paid mutator transaction binding the contract method 0x10d67a2f.
//
// Solidity: function setPauserRegistry(address newPauserRegistry) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) SetPauserRegistry(newPauserRegistry common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.SetPauserRegistry(&_ContractGaspMultiRollupService.TransactOpts, newPauserRegistry)
}

// SetPauserRegistry is a paid mutator transaction binding the contract method 0x10d67a2f.
//
// Solidity: function setPauserRegistry(address newPauserRegistry) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) SetPauserRegistry(newPauserRegistry common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.SetPauserRegistry(&_ContractGaspMultiRollupService.TransactOpts, newPauserRegistry)
}

// SetUpdater is a paid mutator transaction binding the contract method 0x124648c9.
//
// Solidity: function set_updater(address _updater) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) SetUpdater(opts *bind.TransactOpts, _updater common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "set_updater", _updater)
}

// SetUpdater is a paid mutator transaction binding the contract method 0x124648c9.
//
// Solidity: function set_updater(address _updater) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) SetUpdater(_updater common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.SetUpdater(&_ContractGaspMultiRollupService.TransactOpts, _updater)
}

// SetUpdater is a paid mutator transaction binding the contract method 0x124648c9.
//
// Solidity: function set_updater(address _updater) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) SetUpdater(_updater common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.SetUpdater(&_ContractGaspMultiRollupService.TransactOpts, _updater)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) TransferOwnership(opts *bind.TransactOpts, newOwner common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "transferOwnership", newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) TransferOwnership(newOwner common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.TransferOwnership(&_ContractGaspMultiRollupService.TransactOpts, newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) TransferOwnership(newOwner common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.TransferOwnership(&_ContractGaspMultiRollupService.TransactOpts, newOwner)
}

// Unpause is a paid mutator transaction binding the contract method 0xfabc1cbc.
//
// Solidity: function unpause(uint256 newPausedStatus) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) Unpause(opts *bind.TransactOpts, newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "unpause", newPausedStatus)
}

// Unpause is a paid mutator transaction binding the contract method 0xfabc1cbc.
//
// Solidity: function unpause(uint256 newPausedStatus) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) Unpause(newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.Unpause(&_ContractGaspMultiRollupService.TransactOpts, newPausedStatus)
}

// Unpause is a paid mutator transaction binding the contract method 0xfabc1cbc.
//
// Solidity: function unpause(uint256 newPausedStatus) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) Unpause(newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.Unpause(&_ContractGaspMultiRollupService.TransactOpts, newPausedStatus)
}

// ContractGaspMultiRollupServiceEigenReinitProcessedIterator is returned from FilterEigenReinitProcessed and is used to iterate over the raw logs and unpacked data for EigenReinitProcessed events raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceEigenReinitProcessedIterator struct {
	Event *ContractGaspMultiRollupServiceEigenReinitProcessed // Event containing the contract specifics and raw log

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
func (it *ContractGaspMultiRollupServiceEigenReinitProcessedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractGaspMultiRollupServiceEigenReinitProcessed)
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
		it.Event = new(ContractGaspMultiRollupServiceEigenReinitProcessed)
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
func (it *ContractGaspMultiRollupServiceEigenReinitProcessedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractGaspMultiRollupServiceEigenReinitProcessedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractGaspMultiRollupServiceEigenReinitProcessed represents a EigenReinitProcessed event raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceEigenReinitProcessed struct {
	TaskNumber       uint32
	TaskCreatedBlock uint32
	Raw              types.Log // Blockchain specific contextual infos
}

// FilterEigenReinitProcessed is a free log retrieval operation binding the contract event 0x264965eb6bc436c6c473431d34af56e832ec344fdfd43ee6af6fce6d205e84af.
//
// Solidity: event EigenReinitProcessed(uint32 taskNumber, uint32 taskCreatedBlock)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) FilterEigenReinitProcessed(opts *bind.FilterOpts) (*ContractGaspMultiRollupServiceEigenReinitProcessedIterator, error) {

	logs, sub, err := _ContractGaspMultiRollupService.contract.FilterLogs(opts, "EigenReinitProcessed")
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupServiceEigenReinitProcessedIterator{contract: _ContractGaspMultiRollupService.contract, event: "EigenReinitProcessed", logs: logs, sub: sub}, nil
}

// WatchEigenReinitProcessed is a free log subscription operation binding the contract event 0x264965eb6bc436c6c473431d34af56e832ec344fdfd43ee6af6fce6d205e84af.
//
// Solidity: event EigenReinitProcessed(uint32 taskNumber, uint32 taskCreatedBlock)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) WatchEigenReinitProcessed(opts *bind.WatchOpts, sink chan<- *ContractGaspMultiRollupServiceEigenReinitProcessed) (event.Subscription, error) {

	logs, sub, err := _ContractGaspMultiRollupService.contract.WatchLogs(opts, "EigenReinitProcessed")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractGaspMultiRollupServiceEigenReinitProcessed)
				if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "EigenReinitProcessed", log); err != nil {
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

// ParseEigenReinitProcessed is a log parse operation binding the contract event 0x264965eb6bc436c6c473431d34af56e832ec344fdfd43ee6af6fce6d205e84af.
//
// Solidity: event EigenReinitProcessed(uint32 taskNumber, uint32 taskCreatedBlock)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) ParseEigenReinitProcessed(log types.Log) (*ContractGaspMultiRollupServiceEigenReinitProcessed, error) {
	event := new(ContractGaspMultiRollupServiceEigenReinitProcessed)
	if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "EigenReinitProcessed", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractGaspMultiRollupServiceEigenUpdateProcessedIterator is returned from FilterEigenUpdateProcessed and is used to iterate over the raw logs and unpacked data for EigenUpdateProcessed events raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceEigenUpdateProcessedIterator struct {
	Event *ContractGaspMultiRollupServiceEigenUpdateProcessed // Event containing the contract specifics and raw log

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
func (it *ContractGaspMultiRollupServiceEigenUpdateProcessedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractGaspMultiRollupServiceEigenUpdateProcessed)
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
		it.Event = new(ContractGaspMultiRollupServiceEigenUpdateProcessed)
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
func (it *ContractGaspMultiRollupServiceEigenUpdateProcessedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractGaspMultiRollupServiceEigenUpdateProcessedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractGaspMultiRollupServiceEigenUpdateProcessed represents a EigenUpdateProcessed event raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceEigenUpdateProcessed struct {
	TaskNumber       uint32
	TaskCreatedBlock uint32
	Raw              types.Log // Blockchain specific contextual infos
}

// FilterEigenUpdateProcessed is a free log retrieval operation binding the contract event 0x9a128fe7347f1e11ca22aa9deb632ec9abb09608c13a994c60f77a562f460171.
//
// Solidity: event EigenUpdateProcessed(uint32 taskNumber, uint32 taskCreatedBlock)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) FilterEigenUpdateProcessed(opts *bind.FilterOpts) (*ContractGaspMultiRollupServiceEigenUpdateProcessedIterator, error) {

	logs, sub, err := _ContractGaspMultiRollupService.contract.FilterLogs(opts, "EigenUpdateProcessed")
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupServiceEigenUpdateProcessedIterator{contract: _ContractGaspMultiRollupService.contract, event: "EigenUpdateProcessed", logs: logs, sub: sub}, nil
}

// WatchEigenUpdateProcessed is a free log subscription operation binding the contract event 0x9a128fe7347f1e11ca22aa9deb632ec9abb09608c13a994c60f77a562f460171.
//
// Solidity: event EigenUpdateProcessed(uint32 taskNumber, uint32 taskCreatedBlock)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) WatchEigenUpdateProcessed(opts *bind.WatchOpts, sink chan<- *ContractGaspMultiRollupServiceEigenUpdateProcessed) (event.Subscription, error) {

	logs, sub, err := _ContractGaspMultiRollupService.contract.WatchLogs(opts, "EigenUpdateProcessed")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractGaspMultiRollupServiceEigenUpdateProcessed)
				if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "EigenUpdateProcessed", log); err != nil {
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

// ParseEigenUpdateProcessed is a log parse operation binding the contract event 0x9a128fe7347f1e11ca22aa9deb632ec9abb09608c13a994c60f77a562f460171.
//
// Solidity: event EigenUpdateProcessed(uint32 taskNumber, uint32 taskCreatedBlock)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) ParseEigenUpdateProcessed(log types.Log) (*ContractGaspMultiRollupServiceEigenUpdateProcessed, error) {
	event := new(ContractGaspMultiRollupServiceEigenUpdateProcessed)
	if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "EigenUpdateProcessed", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractGaspMultiRollupServiceInitializedIterator is returned from FilterInitialized and is used to iterate over the raw logs and unpacked data for Initialized events raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceInitializedIterator struct {
	Event *ContractGaspMultiRollupServiceInitialized // Event containing the contract specifics and raw log

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
func (it *ContractGaspMultiRollupServiceInitializedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractGaspMultiRollupServiceInitialized)
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
		it.Event = new(ContractGaspMultiRollupServiceInitialized)
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
func (it *ContractGaspMultiRollupServiceInitializedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractGaspMultiRollupServiceInitializedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractGaspMultiRollupServiceInitialized represents a Initialized event raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceInitialized struct {
	Version uint8
	Raw     types.Log // Blockchain specific contextual infos
}

// FilterInitialized is a free log retrieval operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) FilterInitialized(opts *bind.FilterOpts) (*ContractGaspMultiRollupServiceInitializedIterator, error) {

	logs, sub, err := _ContractGaspMultiRollupService.contract.FilterLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupServiceInitializedIterator{contract: _ContractGaspMultiRollupService.contract, event: "Initialized", logs: logs, sub: sub}, nil
}

// WatchInitialized is a free log subscription operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) WatchInitialized(opts *bind.WatchOpts, sink chan<- *ContractGaspMultiRollupServiceInitialized) (event.Subscription, error) {

	logs, sub, err := _ContractGaspMultiRollupService.contract.WatchLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractGaspMultiRollupServiceInitialized)
				if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "Initialized", log); err != nil {
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
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) ParseInitialized(log types.Log) (*ContractGaspMultiRollupServiceInitialized, error) {
	event := new(ContractGaspMultiRollupServiceInitialized)
	if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "Initialized", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractGaspMultiRollupServiceOwnershipTransferredIterator is returned from FilterOwnershipTransferred and is used to iterate over the raw logs and unpacked data for OwnershipTransferred events raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceOwnershipTransferredIterator struct {
	Event *ContractGaspMultiRollupServiceOwnershipTransferred // Event containing the contract specifics and raw log

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
func (it *ContractGaspMultiRollupServiceOwnershipTransferredIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractGaspMultiRollupServiceOwnershipTransferred)
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
		it.Event = new(ContractGaspMultiRollupServiceOwnershipTransferred)
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
func (it *ContractGaspMultiRollupServiceOwnershipTransferredIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractGaspMultiRollupServiceOwnershipTransferredIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractGaspMultiRollupServiceOwnershipTransferred represents a OwnershipTransferred event raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceOwnershipTransferred struct {
	PreviousOwner common.Address
	NewOwner      common.Address
	Raw           types.Log // Blockchain specific contextual infos
}

// FilterOwnershipTransferred is a free log retrieval operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) FilterOwnershipTransferred(opts *bind.FilterOpts, previousOwner []common.Address, newOwner []common.Address) (*ContractGaspMultiRollupServiceOwnershipTransferredIterator, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _ContractGaspMultiRollupService.contract.FilterLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupServiceOwnershipTransferredIterator{contract: _ContractGaspMultiRollupService.contract, event: "OwnershipTransferred", logs: logs, sub: sub}, nil
}

// WatchOwnershipTransferred is a free log subscription operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) WatchOwnershipTransferred(opts *bind.WatchOpts, sink chan<- *ContractGaspMultiRollupServiceOwnershipTransferred, previousOwner []common.Address, newOwner []common.Address) (event.Subscription, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _ContractGaspMultiRollupService.contract.WatchLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractGaspMultiRollupServiceOwnershipTransferred)
				if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
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
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) ParseOwnershipTransferred(log types.Log) (*ContractGaspMultiRollupServiceOwnershipTransferred, error) {
	event := new(ContractGaspMultiRollupServiceOwnershipTransferred)
	if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractGaspMultiRollupServicePausedIterator is returned from FilterPaused and is used to iterate over the raw logs and unpacked data for Paused events raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServicePausedIterator struct {
	Event *ContractGaspMultiRollupServicePaused // Event containing the contract specifics and raw log

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
func (it *ContractGaspMultiRollupServicePausedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractGaspMultiRollupServicePaused)
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
		it.Event = new(ContractGaspMultiRollupServicePaused)
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
func (it *ContractGaspMultiRollupServicePausedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractGaspMultiRollupServicePausedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractGaspMultiRollupServicePaused represents a Paused event raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServicePaused struct {
	Account         common.Address
	NewPausedStatus *big.Int
	Raw             types.Log // Blockchain specific contextual infos
}

// FilterPaused is a free log retrieval operation binding the contract event 0xab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d.
//
// Solidity: event Paused(address indexed account, uint256 newPausedStatus)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) FilterPaused(opts *bind.FilterOpts, account []common.Address) (*ContractGaspMultiRollupServicePausedIterator, error) {

	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}

	logs, sub, err := _ContractGaspMultiRollupService.contract.FilterLogs(opts, "Paused", accountRule)
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupServicePausedIterator{contract: _ContractGaspMultiRollupService.contract, event: "Paused", logs: logs, sub: sub}, nil
}

// WatchPaused is a free log subscription operation binding the contract event 0xab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d.
//
// Solidity: event Paused(address indexed account, uint256 newPausedStatus)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) WatchPaused(opts *bind.WatchOpts, sink chan<- *ContractGaspMultiRollupServicePaused, account []common.Address) (event.Subscription, error) {

	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}

	logs, sub, err := _ContractGaspMultiRollupService.contract.WatchLogs(opts, "Paused", accountRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractGaspMultiRollupServicePaused)
				if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "Paused", log); err != nil {
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
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) ParsePaused(log types.Log) (*ContractGaspMultiRollupServicePaused, error) {
	event := new(ContractGaspMultiRollupServicePaused)
	if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "Paused", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractGaspMultiRollupServicePauserRegistrySetIterator is returned from FilterPauserRegistrySet and is used to iterate over the raw logs and unpacked data for PauserRegistrySet events raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServicePauserRegistrySetIterator struct {
	Event *ContractGaspMultiRollupServicePauserRegistrySet // Event containing the contract specifics and raw log

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
func (it *ContractGaspMultiRollupServicePauserRegistrySetIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractGaspMultiRollupServicePauserRegistrySet)
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
		it.Event = new(ContractGaspMultiRollupServicePauserRegistrySet)
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
func (it *ContractGaspMultiRollupServicePauserRegistrySetIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractGaspMultiRollupServicePauserRegistrySetIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractGaspMultiRollupServicePauserRegistrySet represents a PauserRegistrySet event raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServicePauserRegistrySet struct {
	PauserRegistry    common.Address
	NewPauserRegistry common.Address
	Raw               types.Log // Blockchain specific contextual infos
}

// FilterPauserRegistrySet is a free log retrieval operation binding the contract event 0x6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6.
//
// Solidity: event PauserRegistrySet(address pauserRegistry, address newPauserRegistry)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) FilterPauserRegistrySet(opts *bind.FilterOpts) (*ContractGaspMultiRollupServicePauserRegistrySetIterator, error) {

	logs, sub, err := _ContractGaspMultiRollupService.contract.FilterLogs(opts, "PauserRegistrySet")
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupServicePauserRegistrySetIterator{contract: _ContractGaspMultiRollupService.contract, event: "PauserRegistrySet", logs: logs, sub: sub}, nil
}

// WatchPauserRegistrySet is a free log subscription operation binding the contract event 0x6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6.
//
// Solidity: event PauserRegistrySet(address pauserRegistry, address newPauserRegistry)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) WatchPauserRegistrySet(opts *bind.WatchOpts, sink chan<- *ContractGaspMultiRollupServicePauserRegistrySet) (event.Subscription, error) {

	logs, sub, err := _ContractGaspMultiRollupService.contract.WatchLogs(opts, "PauserRegistrySet")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractGaspMultiRollupServicePauserRegistrySet)
				if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "PauserRegistrySet", log); err != nil {
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
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) ParsePauserRegistrySet(log types.Log) (*ContractGaspMultiRollupServicePauserRegistrySet, error) {
	event := new(ContractGaspMultiRollupServicePauserRegistrySet)
	if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "PauserRegistrySet", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractGaspMultiRollupServiceUnpausedIterator is returned from FilterUnpaused and is used to iterate over the raw logs and unpacked data for Unpaused events raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceUnpausedIterator struct {
	Event *ContractGaspMultiRollupServiceUnpaused // Event containing the contract specifics and raw log

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
func (it *ContractGaspMultiRollupServiceUnpausedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractGaspMultiRollupServiceUnpaused)
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
		it.Event = new(ContractGaspMultiRollupServiceUnpaused)
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
func (it *ContractGaspMultiRollupServiceUnpausedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractGaspMultiRollupServiceUnpausedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractGaspMultiRollupServiceUnpaused represents a Unpaused event raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceUnpaused struct {
	Account         common.Address
	NewPausedStatus *big.Int
	Raw             types.Log // Blockchain specific contextual infos
}

// FilterUnpaused is a free log retrieval operation binding the contract event 0x3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c.
//
// Solidity: event Unpaused(address indexed account, uint256 newPausedStatus)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) FilterUnpaused(opts *bind.FilterOpts, account []common.Address) (*ContractGaspMultiRollupServiceUnpausedIterator, error) {

	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}

	logs, sub, err := _ContractGaspMultiRollupService.contract.FilterLogs(opts, "Unpaused", accountRule)
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupServiceUnpausedIterator{contract: _ContractGaspMultiRollupService.contract, event: "Unpaused", logs: logs, sub: sub}, nil
}

// WatchUnpaused is a free log subscription operation binding the contract event 0x3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c.
//
// Solidity: event Unpaused(address indexed account, uint256 newPausedStatus)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) WatchUnpaused(opts *bind.WatchOpts, sink chan<- *ContractGaspMultiRollupServiceUnpaused, account []common.Address) (event.Subscription, error) {

	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}

	logs, sub, err := _ContractGaspMultiRollupService.contract.WatchLogs(opts, "Unpaused", accountRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractGaspMultiRollupServiceUnpaused)
				if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "Unpaused", log); err != nil {
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
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) ParseUnpaused(log types.Log) (*ContractGaspMultiRollupServiceUnpaused, error) {
	event := new(ContractGaspMultiRollupServiceUnpaused)
	if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "Unpaused", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

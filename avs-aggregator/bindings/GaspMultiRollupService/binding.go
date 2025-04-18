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
	LastCompletedOpTaskNum                       uint32
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
	ChainId                                      uint64
	BatchId                                      uint32
	TaskCreatedBlock                             uint32
	LastCompletedOpTaskNum                       uint32
	LastCompletedOpTaskCreatedBlock              uint32
	LastCompletedOpTaskQuorumNumbers             []byte
	LastCompletedOpTaskQuorumThresholdPercentage uint32
}

// IFinalizerTaskManagerRdTaskResponse is an auto generated low-level Go binding around an user-defined struct.
type IFinalizerTaskManagerRdTaskResponse struct {
	ReferenceTaskIndex uint32
	ReferenceTaskHash  [32]byte
	ChainId            uint64
	BatchId            uint32
	RdUpdate           [32]byte
	RangeStart         *big.Int
	RangeEnd           *big.Int
	Updater            common.Address
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

// IRolldownPrimitivesRange is an auto generated low-level Go binding around an user-defined struct.
type IRolldownPrimitivesRange struct {
	Start *big.Int
	End   *big.Int
}

// ContractGaspMultiRollupServiceMetaData contains all meta data concerning the ContractGaspMultiRollupService contract.
var ContractGaspMultiRollupServiceMetaData = &bind.MetaData{
	ABI: "[{\"type\":\"function\",\"name\":\"allowNonRootInit\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"chainId\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"chainRdBatchNonce\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"checkSignatures\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"params\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.QuorumStakeTotals\",\"components\":[{\"name\":\"signedStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"totalStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"_pauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"initialOwner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_updater\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_allowNonRootInit\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"_rolldown\",\"type\":\"address\",\"internalType\":\"contractIRolldown\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"lastOpUpdateBlockTimestamp\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestCompletedOpTaskCreatedBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestCompletedOpTaskNumber\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestCompletedRdTaskCreatedBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestCompletedRdTaskNumber\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"operatorAndQuorumToStakes\",\"inputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint96\",\"internalType\":\"uint96\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"operatorIdQuorumCount\",\"inputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"pauseAll\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[{\"name\":\"index\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pauserRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"processEigenOpUpdate\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]},{\"name\":\"operatorStateInfo\",\"type\":\"tuple\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorStateInfo\",\"components\":[{\"name\":\"operatorsStateChanged\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"quorumsRemoved\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsAdded[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"quorumsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsStakeUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]},{\"name\":\"quorumsApkUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsApkUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"operatorsRemoved\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"operatorsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsAdded[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]},{\"name\":\"operatorsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsStakeUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"operatorsQuorumCountUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsQuorumCountUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"processEigenRdUpdate\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.RdTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"chainId\",\"type\":\"uint64\",\"internalType\":\"uint64\"},{\"name\":\"batchId\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.RdTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"chainId\",\"type\":\"uint64\",\"internalType\":\"uint64\"},{\"name\":\"batchId\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"rdUpdate\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"rangeStart\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"rangeEnd\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"updater\",\"type\":\"address\",\"internalType\":\"address\"}]},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"processEigenReinit\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"operatorStateInfo\",\"type\":\"tuple\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorStateInfo\",\"components\":[{\"name\":\"operatorsStateChanged\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"quorumsRemoved\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsAdded[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"quorumsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsStakeUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]},{\"name\":\"quorumsApkUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsApkUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"operatorsRemoved\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"operatorsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsAdded[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]},{\"name\":\"operatorsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsStakeUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"operatorsQuorumCountUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsQuorumCountUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]}]},{\"name\":\"merkleRoots\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"ranges\",\"type\":\"tuple[]\",\"internalType\":\"structIRolldownPrimitives.Range[]\",\"components\":[{\"name\":\"start\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"end\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"_chainRdBatchNonce\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"_latestCompletedRdTaskNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"_latestCompletedRdTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"qourumApk\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"quorumNumbers\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"quorumThresholdPercentage\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"quorumToStakes\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint96\",\"internalType\":\"uint96\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"rolldown\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIRolldown\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"setPauserRegistry\",\"inputs\":[{\"name\":\"newPauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setRolldown\",\"inputs\":[{\"name\":\"_rolldown\",\"type\":\"address\",\"internalType\":\"contractIRolldown\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setUpdater\",\"inputs\":[{\"name\":\"_updater\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"stalled\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"trySignatureAndApkVerification\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"apk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}],\"outputs\":[{\"name\":\"pairingSuccessful\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"siganatureIsValid\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"unpause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updater\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"event\",\"name\":\"EigenOpUpdateProcessed\",\"inputs\":[{\"name\":\"taskNumber\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"EigenRdUpdateProcessed\",\"inputs\":[{\"name\":\"taskNumber\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"EigenReinitProcessed\",\"inputs\":[{\"name\":\"taskNumber\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Initialized\",\"inputs\":[{\"name\":\"version\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Paused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"PauserRegistrySet\",\"inputs\":[{\"name\":\"pauserRegistry\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"newPauserRegistry\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractIPauserRegistry\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RolldownTargetUpdated\",\"inputs\":[{\"name\":\"rolldownAddress\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Unpaused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false}]",
	Bin: "0x608060405234801561001057600080fd5b50614d6d806100206000396000f3fe608060405234801561001057600080fd5b50600436106102115760003560e01c80636f0c30a411610125578063d03a07b2116100ad578063e2a7cb661161007c578063e2a7cb661461054d578063f2fde38b14610565578063f84e91fc14610578578063fabc1cbc14610581578063fdc15de81461059457600080fd5b8063d03a07b214610500578063d30270ef14610510578063deb4037d14610523578063df034cd01461053a57600080fd5b80637d978897116100f45780637d97889714610495578063886f1195146104b55780638da5cb5b146104c85780639a8a0592146104d95780639d54f419146104ed57600080fd5b80636f0c30a41461043a578063715018a614610451578063738491d9146104595780637ad755611461046c57600080fd5b80633d9fb00c116101a8578063526e3e6411610177578063526e3e64146103d657806356b933d9146103ea578063595c6a67146103fd5780635ac86ab7146104055780635c975abb1461042857600080fd5b80633d9fb00c1461031a578063430d3b3914610345578063499d6fb61461037a5780634deabc21146103c657600080fd5b8063136439dd116101e4578063136439dd146102b5578063171f1d5b146102c85780632a8414fd146102f2578063390f75701461030757600080fd5b806303d097d2146102165780630bf16410146102575780630ee0fdbd1461027c57806310d67a2f146102a0575b600080fd5b61023d610224366004613665565b609f602052600090815260409020805460019091015482565b604080519283526020830191909152015b60405180910390f35b60a1546102679063ffffffff1681565b60405163ffffffff909116815260200161024e565b60985461029090600160a81b900460ff1681565b604051901515815260200161024e565b6102b36102ae366004613695565b6105a7565b005b6102b36102c33660046136b2565b610663565b6102db6102d6366004613830565b6107a2565b60408051921515835290151560208301520161024e565b6102fa61092c565b60405161024e9190613881565b6102b36103153660046138ef565b6109ba565b60975461032d906001600160a01b031681565b6040516001600160a01b03909116815260200161024e565b6103686103533660046136b2565b60a06020526000908152604090205460ff1681565b60405160ff909116815260200161024e565b6103ae610388366004613960565b609e6020908152600092835260408084209091529082529020546001600160601b031681565b6040516001600160601b03909116815260200161024e565b609c546102679063ffffffff1681565b60985461029090600160a01b900460ff1681565b6102b36103f8366004613a5b565b610b29565b6102b3610d9d565b610290610413366004613665565b606654600160ff9092169190911b9081161490565b6066545b60405190815260200161024e565b609a5461026790600160401b900463ffffffff1681565b6102b3610e64565b6102b3610467366004613b57565b610e78565b6103ae61047a366004613665565b609d602052600090815260409020546001600160601b031681565b6104a86104a3366004613e6f565b611420565b60405161024e9190613ef9565b60655461032d906001600160a01b031681565b6033546001600160a01b031661032d565b60975461036890600160c01b900460ff1681565b6102b36104fb366004613695565b611a0a565b609a546102679063ffffffff1681565b6102b361051e366004613f3b565b611a34565b60975461026790600160a01b900463ffffffff1681565b60985461032d906001600160a01b031681565b609a5461026790640100000000900463ffffffff1681565b6102b3610573366004613695565b611f39565b61042c60995481565b6102b361058f3660046136b2565b611faf565b6102b36105a2366004613695565b61210b565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156105fa573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061061e9190613fdf565b6001600160a01b0316336001600160a01b0316146106575760405162461bcd60e51b815260040161064e90613ffc565b60405180910390fd5b61066081612167565b50565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa1580156106ab573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106cf9190614046565b6106eb5760405162461bcd60e51b815260040161064e90614063565b606654818116146107645760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e70617573653a20696e76616c696420617474656d70742060448201527f746f20756e70617573652066756e6374696f6e616c6974790000000000000000606482015260840161064e565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b60008060007f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001878760000151886020015188600001516000600281106107ea576107ea6140ab565b60200201518951600160200201518a6020015160006002811061080f5761080f6140ab565b60200201518b6020015160016002811061082b5761082b6140ab565b602090810291909101518c518d8301516040516108889a99989796959401988952602089019790975260408801959095526060870193909352608086019190915260a085015260c084015260e08301526101008201526101200190565b6040516020818303038152906040528051906020012060001c6108ab91906140c1565b905061091e6108c46108bd888461225e565b86906122ef565b6108cc612384565b610914610905856108ff604080518082018252600080825260209182015281518083019092526001825260029082015290565b9061225e565b61090e8c612444565b906122ef565b886201d4c06124d3565b909890975095505050505050565b609b8054610939906140e3565b80601f0160208091040260200160405190810160405280929190818152602001828054610965906140e3565b80156109b25780601f10610987576101008083540402835291602001916109b2565b820191906000526020600020905b81548152906001019060200180831161099557829003601f168201915b505050505081565b600054610100900460ff16158080156109da5750600054600160ff909116105b806109f45750303b1580156109f4575060005460ff166001145b610a575760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b606482015260840161064e565b6000805460ff191660011790558015610a7a576000805461ff0019166101001790555b610a858660006126f7565b610a8e856127e1565b609880546001600160a01b03868116600161ff0160a01b031990921691909117600160a81b861515021790915560978054600160a01b9285166001600160c01b0319909116179190911790558015610b21576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498906020015b60405180910390a15b505050505050565b610b31612833565b858414610b805760405162461bcd60e51b815260206004820152601d60248201527f726455706461746520696e666f206c656e677468206d69736d61746368000000604482015260640161064e565b610b898861288d565b610b9660208a018a614117565b609a805463ffffffff929092166401000000000267ffffffff0000000019909216919091179055610bcd60408a0160208b01614117565b609a805463ffffffff92909216600160401b0263ffffffff60401b1990921691909117905542609955610c0360808a018a614132565b610c0f91609b916134e1565b50610c2060c08a0160a08b01614117565b609c805463ffffffff191663ffffffff9290921691909117905560005b86811015610ce9576097546001600160a01b03166308f42d40898984818110610c6857610c686140ab565b90506020020135888885818110610c8157610c816140ab565b9050604002016040518363ffffffff1660e01b8152600401610ca4929190614178565b600060405180830381600087803b158015610cbe57600080fd5b505af1158015610cd2573d6000803e3d6000fd5b505050508080610ce1906141ac565b915050610c3d565b506097805463ffffffff808616600160a01b0263ffffffff60a01b1990921691909117909155609a805484831663ffffffff199182161790915560a18054928416929091169190911790557f264965eb6bc436c6c473431d34af56e832ec344fdfd43ee6af6fce6d205e84af610d6260208b018b614117565b610d7260408c0160208d01614117565b6040805163ffffffff93841681529290911660208301520160405180910390a1505050505050505050565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa158015610de5573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610e099190614046565b610e255760405162461bcd60e51b815260040161064e90614063565b600019606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b610e6c612833565b610e7660006127e1565b565b60665415610ec85760405162461bcd60e51b815260206004820152601c60248201527f5061757361626c653a20636f6e74726163742069732070617573656400000000604482015260640161064e565b6098546001600160a01b03163314610f225760405162461bcd60e51b815260206004820152601a60248201527f55706461746572206d757374206265207468652063616c6c6572000000000000604482015260640161064e565b609754600160a01b900463ffffffff16610f426080840160608501614117565b63ffffffff1614610f955760405162461bcd60e51b815260206004820152601a60248201527f636861696e526442617463684e6f6e6365206d69736d61746368000000000000604482015260640161064e565b610fa560408401602085016141dc565b6001600160401b03164614610fec5760405162461bcd60e51b815260206004820152600d60248201526c15dc9bdb99c818da185a5b9259609a1b604482015260640161064e565b609a5463ffffffff16158061101857506110096020840184614117565b609a5463ffffffff9182169116105b6110535760405162461bcd60e51b815260206004820152600c60248201526b5374616c652052645461736b60a01b604482015260640161064e565b609a54600160401b900463ffffffff166000036110a45760405162461bcd60e51b815260206004820152600f60248201526e13dc081cdd185d19481d5b9a5b9a5d608a1b604482015260640161064e565b6110b460c0840160a08501614117565b609a54600160401b900463ffffffff9081169116146111155760405162461bcd60e51b815260206004820152601d60248201527f7265666572656e636520626c6f636b2068617368206d69736d61746368000000604482015260640161064e565b826040516020016111269190614265565b6040516020818303038152906040528051906020012082602001351461118e5760405162461bcd60e51b815260206004820152601f60248201527f7265666572656e63655461736b486173682068617368206d69736d6174636800604482015260640161064e565b60006111c4836040516020016111a49190614343565b60405160208183030381529060405280519060200120836104a3906143d6565b609c5490915063ffffffff1660005b609b80546111e0906140e3565b90508110156112a9578160ff1683602001518281518110611203576112036140ab565b602002602001015161121591906143e2565b6001600160601b0316606484600001518381518110611236576112366140ab565b60200260200101516001600160601b03166112519190614411565b10156112975760405162461bcd60e51b81526020600482015260156024820152744661696c656420746f206d6565742071756f72756d60581b604482015260640161064e565b806112a1816141ac565b9150506111d3565b5060408051808201825260a0860135815260c08601356020820190815260975492516223d0b560e61b815260808801356004820152825160248201529051604482015290916001600160a01b0316906308f42d4090606401600060405180830381600087803b15801561131b57600080fd5b505af115801561132f573d6000803e3d6000fd5b50611344925050506080860160608701614117565b61134f906001614430565b6097805463ffffffff92909216600160a01b0263ffffffff60a01b199092169190911790556113816020870187614117565b609a805463ffffffff191663ffffffff929092169190911790556113ab6080870160608801614117565b60a1805463ffffffff191663ffffffff929092169190911790557fec68db391879b0f9f420d1cdf3476afbdf085a2462bf4d2b11df78466295cb176113f36020880188614117565b6114036080890160608a01614117565b6040805163ffffffff938416815292909116602083015201610b18565b604080518082019091526060808252602082015260408051808201909152600080825260208201819052609b8054611457906140e3565b90509050611478604051806040016040528060608152602001606081525090565b816001600160401b03811115611490576114906136cb565b6040519080825280602002602001820160405280156114b9578160200160208202803683370190505b506020820152816001600160401b038111156114d7576114d76136cb565b604051908082528060200260200182016040528015611500578160200160208202803683370190505b5081526020850151516000906001600160401b03811115611523576115236136cb565b60405190808252806020026020018201604052801561154c578160200160208202803683370190505b5090506000805b8760200151518110156116fe5761159888602001518281518110611579576115796140ab565b6020026020010151805160009081526020918201519091526040902090565b8382815181106115aa576115aa6140ab565b6020908102919091010152801561167457826115c7600183614458565b815181106115d7576115d76140ab565b602002602001015160001c8382815181106115f4576115f46140ab565b602002602001015160001c11611674576040805162461bcd60e51b81526020600482015260248101919091527f424c535369676e6174757265436865636b65722e636865636b5369676e61747560448201527f7265733a206e6f6e5369676e65725075626b657973206e6f7420736f72746564606482015260840161064e565b6116ea6116e360a06000868581518110611690576116906140ab565b6020026020010151815260200190815260200160002060009054906101000a900460ff1660ff168a6020015184815181106116cd576116cd6140ab565b602002602001015161323690919063ffffffff16565b87906122ef565b9550806116f6816141ac565b915050611553565b5061170885613319565b945060005b848110156118ec57609b818154611723906140e3565b8110611731576117316140ab565b8154600116156117505790600052602060002090602091828204019190065b9054600160f81b911a0260f81c6000818152609f602090815260409182902082518084019093528054835260010154908201529092506117919087906122ef565b60ff83166000908152609d60209081526040909120549086015180519298506001600160601b0390911691839081106117cc576117cc6140ab565b6001600160601b039092166020928302919091018201528401518051829081106117f8576117f86140ab565b602002602001015184600001518281518110611816576118166140ab565b60200260200101906001600160601b031690816001600160601b03168152505060005b8860200151518110156118d957609e600085838151811061185c5761185c6140ab565b6020908102919091018101518252818101929092526040908101600090812060ff87168252909252902054855180516001600160601b0390921691849081106118a7576118a76140ab565b602002602001018181516118bb919061446f565b6001600160601b0316905250806118d1816141ac565b915050611839565b50806118e4816141ac565b91505061170d565b506000806119048a888b606001518c608001516107a2565b91509150816119875760405162461bcd60e51b815260206004820152604360248201527f424c535369676e6174757265436865636b65722e636865636b5369676e61747560448201527f7265733a2070616972696e6720707265636f6d70696c652063616c6c206661696064820152621b195960ea1b608482015260a40161064e565b806119fa5760405162461bcd60e51b815260206004820152603960248201527f424c535369676e6174757265436865636b65722e636865636b5369676e61747560448201527f7265733a207369676e617475726520697320696e76616c696400000000000000606482015260840161064e565b5092955050505050505b92915050565b611a12612833565b609880546001600160a01b0319166001600160a01b0392909216919091179055565b60665415611a845760405162461bcd60e51b815260206004820152601c60248201527f5061757361626c653a20636f6e74726163742069732070617573656400000000604482015260640161064e565b6098546001600160a01b03163314611ade5760405162461bcd60e51b815260206004820152601a60248201527f55706461746572206d757374206265207468652063616c6c6572000000000000604482015260640161064e565b609a54600160401b900463ffffffff1680151580611b055750609854600160a81b900460ff165b15611b51576098546001600160a01b03163314611b4c5760405162461bcd60e51b8152602060048201526005602482015264041757468360dc1b604482015260640161064e565b611b93565b6033546001600160a01b03163314611b935760405162461bcd60e51b8152602060048201526005602482015264417574683160d81b604482015260640161064e565b84604051602001611ba49190614497565b60405160208183030381529060405280519060200120846020013514611c0c5760405162461bcd60e51b815260206004820152601f60248201527f7265666572656e63655461736b486173682068617368206d69736d6174636800604482015260640161064e565b81604051602001611c1d91906149f5565b60405160208183030381529060405280519060200120846040013514611c855760405162461bcd60e51b815260206004820152601f60248201527f6f70657261746f725374617465496e666f2068617368206d69736d6174636800604482015260640161064e565b63ffffffff811615611e1a57611ca16080860160608701614117565b63ffffffff168163ffffffff1614611cfb5760405162461bcd60e51b815260206004820152601860248201527f7265666572656e636520626c6f636b206d69736d617463680000000000000000604482015260640161064e565b6000611d3185604051602001611d119190614b5c565b60405160208183030381529060405280519060200120856104a3906143d6565b609c5490915063ffffffff1660005b609b8054611d4d906140e3565b9050811015611e16578160ff1683602001518281518110611d7057611d706140ab565b6020026020010151611d8291906143e2565b6001600160601b0316606484600001518381518110611da357611da36140ab565b60200260200101516001600160601b0316611dbe9190614411565b1015611e045760405162461bcd60e51b81526020600482015260156024820152744661696c656420746f206d6565742071756f72756d60581b604482015260640161064e565b80611e0e816141ac565b915050611d40565b5050505b611e238261288d565b611e306020860186614117565b609a805463ffffffff929092166401000000000267ffffffff0000000019909216919091179055611e676040860160208701614117565b609a805463ffffffff92909216600160401b0263ffffffff60401b1990921691909117905542609955611e9d6080860186614132565b611ea991609b916134e1565b50611eba60c0860160a08701614117565b609c805463ffffffff191663ffffffff929092169190911790557f36a1fd7bd554f5c428c9829c09c6606b4c893b1fadc8735a7a12795797447ded611f026020870187614117565b611f126040880160208901614117565b6040805163ffffffff93841681529290911660208301520160405180910390a15050505050565b611f41612833565b6001600160a01b038116611fa65760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b606482015260840161064e565b610660816127e1565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612002573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906120269190613fdf565b6001600160a01b0316336001600160a01b0316146120565760405162461bcd60e51b815260040161064e90613ffc565b6066541981196066541916146120d45760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e756e70617573653a20696e76616c696420617474656d7060448201527f7420746f2070617573652066756e6374696f6e616c6974790000000000000000606482015260840161064e565b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c90602001610797565b612113612833565b609780546001600160a01b0319166001600160a01b0383169081179091556040519081527f2f20cf1bda67739044c5bf577353970c3dbc183b2c7274d1e8584a10269232679060200160405180910390a150565b6001600160a01b0381166121f55760405162461bcd60e51b815260206004820152604960248201527f5061757361626c652e5f73657450617573657252656769737472793a206e657760448201527f50617573657252656769737472792063616e6e6f7420626520746865207a65726064820152686f206164647265737360b81b608482015260a40161064e565b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b604080518082019091526000808252602082015261227a613565565b835181526020808501519082015260408082018490526000908360608460076107d05a03fa905080806122a957fe5b50806122e75760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b604482015260640161064e565b505092915050565b604080518082019091526000808252602082015261230b613583565b835181526020808501518183015283516040808401919091529084015160608301526000908360808460066107d05a03fa9050808061234657fe5b50806122e75760405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b604482015260640161064e565b61238c6135a1565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b604080518082019091526000808252602082015260008080612474600080516020614d18833981519152866140c1565b90505b612480816133b4565b9093509150600080516020614d1883398151915282830983036124b9576040805180820190915290815260208101919091529392505050565b600080516020614d18833981519152600182089050612477565b6040805180820182528681526020808201869052825180840190935286835282018490526000918291906125056135c6565b60005b60028110156126ca57600061251e826006614411565b9050848260028110612532576125326140ab565b60200201515183612544836000614b8b565b600c8110612554576125546140ab565b602002015284826002811061256b5761256b6140ab565b602002015160200151838260016125829190614b8b565b600c8110612592576125926140ab565b60200201528382600281106125a9576125a96140ab565b60200201515151836125bc836002614b8b565b600c81106125cc576125cc6140ab565b60200201528382600281106125e3576125e36140ab565b60200201515160016020020151836125fc836003614b8b565b600c811061260c5761260c6140ab565b6020020152838260028110612623576126236140ab565b60200201516020015160006002811061263e5761263e6140ab565b60200201518361264f836004614b8b565b600c811061265f5761265f6140ab565b6020020152838260028110612676576126766140ab565b602002015160200151600160028110612691576126916140ab565b6020020151836126a2836005614b8b565b600c81106126b2576126b26140ab565b602002015250806126c2816141ac565b915050612508565b506126d36135e5565b60006020826101808560088cfa9151919c9115159b50909950505050505050505050565b6065546001600160a01b031615801561271857506001600160a01b03821615155b61279a5760405162461bcd60e51b815260206004820152604760248201527f5061757361626c652e5f696e697469616c697a655061757365723a205f696e6960448201527f7469616c697a6550617573657228292063616e206f6e6c792062652063616c6c6064820152666564206f6e636560c81b608482015260a40161064e565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a26127dd82612167565b5050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6033546001600160a01b03163314610e765760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604482015260640161064e565b60005b61289d6020830183614ba3565b905081101561296257609d60006128b76020850185614ba3565b848181106128c7576128c76140ab565b90506020020160208101906128dc9190613665565b60ff168152602080820192909252604001600090812080546001600160601b0319169055609f9161290f90850185614ba3565b8481811061291f5761291f6140ab565b90506020020160208101906129349190613665565b60ff16815260208101919091526040016000908120818155600101558061295a816141ac565b915050612890565b5060005b6129736040830183614bec565b9050811015612aae576129896040830183614bec565b82818110612999576129996140ab565b90506080020160200160208101906129b19190614c35565b609d60006129c26040860186614bec565b858181106129d2576129d26140ab565b6129e89260206080909202019081019150613665565b60ff1681526020810191909152604090810160002080546001600160601b0319166001600160601b039390931692909217909155612a2890830183614bec565b82818110612a3857612a386140ab565b905060800201604001609f6000848060400190612a559190614bec565b85818110612a6557612a656140ab565b612a7b9260206080909202019081019150613665565b60ff1681526020808201929092526040016000208235815591013560019091015580612aa6816141ac565b915050612966565b5060005b612abf6060830183614c50565b9050811015612b7757612ad56060830183614c50565b82818110612ae557612ae56140ab565b9050604002016020016020810190612afd9190614c35565b609d6000612b0e6060860186614c50565b85818110612b1e57612b1e6140ab565b612b349260206040909202019081019150613665565b60ff168152602081019190915260400160002080546001600160601b0319166001600160601b039290921691909117905580612b6f816141ac565b915050612ab2565b5060005b612b886080830183614c99565b9050811015612c2457612b9e6080830183614c99565b82818110612bae57612bae6140ab565b905060600201602001609f6000848060800190612bcb9190614c99565b85818110612bdb57612bdb6140ab565b612bf19260206060909202019081019150613665565b60ff1681526020808201929092526040016000208235815591013560019091015580612c1c816141ac565b915050612b7b565b5060005b612c3560a0830183614ba3565b9050811015612d5b5760005b609b8054612c4e906140e3565b9050811015612d0757609e6000612c6860a0860186614ba3565b85818110612c7857612c786140ab565b9050602002013581526020019081526020016000206000609b838154612c9d906140e3565b8110612cab57612cab6140ab565b815460011615612cca5790600052602060002090602091828204019190065b9054600160f81b911a0260f81c8152602081019190915260400160002080546001600160601b031916905580612cff816141ac565b915050612c41565b5060a06000612d1884830185614ba3565b84818110612d2857612d286140ab565b60209081029290920135835250810191909152604001600020805460ff1916905580612d53816141ac565b915050612c28565b5060005b612d6c60c0830183614ba3565b9050811015612fbc57612d8260c0830183614ba3565b82818110612d9257612d926140ab565b9050602002810190612da49190614ce1565b612db5906080810190606001613665565b60a06000612dc660c0860186614ba3565b85818110612dd657612dd66140ab565b9050602002810190612de89190614ce1565b60000135815260200190815260200160002060006101000a81548160ff021916908360ff16021790555060005b612e2260c0840184614ba3565b83818110612e3257612e326140ab565b9050602002810190612e449190614ce1565b612e52906020810190614ba3565b9050811015612fa957612e6860c0840184614ba3565b83818110612e7857612e786140ab565b9050602002810190612e8a9190614ce1565b612e98906040810190614ba3565b82818110612ea857612ea86140ab565b9050602002016020810190612ebd9190614c35565b609e6000612ece60c0870187614ba3565b86818110612ede57612ede6140ab565b9050602002810190612ef09190614ce1565b3581526020810191909152604001600090812090612f1160c0870187614ba3565b86818110612f2157612f216140ab565b9050602002810190612f339190614ce1565b612f41906020810190614ba3565b85818110612f5157612f516140ab565b9050602002016020810190612f669190613665565b60ff168152602081019190915260400160002080546001600160601b0319166001600160601b039290921691909117905580612fa1816141ac565b915050612e15565b5080612fb4816141ac565b915050612d5f565b5060005b612fcd60e0830183614ba3565b90508110156131805760005b612fe660e0840184614ba3565b83818110612ff657612ff66140ab565b90506020028101906130089190614d01565b613016906020810190614ba3565b905081101561316d5761302c60e0840184614ba3565b8381811061303c5761303c6140ab565b905060200281019061304e9190614d01565b61305c906040810190614ba3565b8281811061306c5761306c6140ab565b90506020020160208101906130819190614c35565b609e600061309260e0870187614ba3565b868181106130a2576130a26140ab565b90506020028101906130b49190614d01565b35815260208101919091526040016000908120906130d560e0870187614ba3565b868181106130e5576130e56140ab565b90506020028101906130f79190614d01565b613105906020810190614ba3565b85818110613115576131156140ab565b905060200201602081019061312a9190613665565b60ff168152602081019190915260400160002080546001600160601b0319166001600160601b039290921691909117905580613165816141ac565b915050612fd9565b5080613178816141ac565b915050612fc0565b5060005b613192610100830183614c50565b90508110156127dd576131a9610100830183614c50565b828181106131b9576131b96140ab565b90506040020160200160208101906131d19190613665565b60a060006131e3610100860186614c50565b858181106131f3576131f36140ab565b90506040020160000135815260200190815260200160002060006101000a81548160ff021916908360ff160217905550808061322e906141ac565b915050613184565b60408051808201909152600080825260208201526102008261ffff16106132925760405162461bcd60e51b815260206004820152601060248201526f7363616c61722d746f6f2d6c6172676560801b604482015260640161064e565b8161ffff166001036132a5575081611a04565b6040805180820190915260008082526020820181905284906001905b8161ffff168661ffff161061330e57600161ffff871660ff83161c811690036132f1576132ee84846122ef565b93505b6132fb83846122ef565b92506201fffe600192831b1691016132c1565b509195945050505050565b6040805180820190915260008082526020820152815115801561333e57506020820151155b1561335c575050604080518082019091526000808252602082015290565b604051806040016040528083600001518152602001600080516020614d18833981519152846020015161338f91906140c1565b6133a790600080516020614d18833981519152614458565b905292915050565b919050565b60008080600080516020614d188339815191526003600080516020614d1883398151915286600080516020614d1883398151915288890909089050600061342a827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f52600080516020614d18833981519152613436565b91959194509092505050565b6000806134416135e5565b613449613603565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa9250828061348657fe5b50826134d45760405162461bcd60e51b815260206004820152601a60248201527f424e3235342e6578704d6f643a2063616c6c206661696c757265000000000000604482015260640161064e565b50519150505b9392505050565b8280546134ed906140e3565b90600052602060002090601f01602090048101928261350f5760008555613555565b82601f106135285782800160ff19823516178555613555565b82800160010185558215613555579182015b8281111561355557823582559160200191906001019061353a565b50613561929150613621565b5090565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b60405180604001604052806135b4613636565b81526020016135c1613636565b905290565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b5b808211156135615760008155600101613622565b60405180604001604052806002906020820280368337509192915050565b803560ff811681146133af57600080fd5b60006020828403121561367757600080fd5b6134da82613654565b6001600160a01b038116811461066057600080fd5b6000602082840312156136a757600080fd5b81356134da81613680565b6000602082840312156136c457600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b0381118282101715613703576137036136cb565b60405290565b60405161010081016001600160401b0381118282101715613703576137036136cb565b604051601f8201601f191681016001600160401b0381118282101715613754576137546136cb565b604052919050565b60006040828403121561376e57600080fd5b6137766136e1565b9050813581526020820135602082015292915050565b600082601f83011261379d57600080fd5b604051604081018181106001600160401b03821117156137bf576137bf6136cb565b80604052508060408401858111156137d657600080fd5b845b8181101561330e5780358352602092830192016137d8565b60006080828403121561380257600080fd5b61380a6136e1565b9050613816838361378c565b8152613825836040840161378c565b602082015292915050565b600080600080610120858703121561384757600080fd5b84359350613858866020870161375c565b925061386786606087016137f0565b91506138768660e0870161375c565b905092959194509250565b600060208083528351808285015260005b818110156138ae57858101830151858201604001528201613892565b818111156138c0576000604083870101525b50601f01601f1916929092016040019392505050565b801515811461066057600080fd5b80356133af816138d6565b600080600080600060a0868803121561390757600080fd5b853561391281613680565b9450602086013561392281613680565b9350604086013561393281613680565b92506060860135613942816138d6565b9150608086013561395281613680565b809150509295509295909350565b6000806040838503121561397357600080fd5b8235915061398360208401613654565b90509250929050565b6000610100828403121561399f57600080fd5b50919050565b6000610120828403121561399f57600080fd5b60008083601f8401126139ca57600080fd5b5081356001600160401b038111156139e157600080fd5b6020830191508360208260051b85010111156139fc57600080fd5b9250929050565b60008083601f840112613a1557600080fd5b5081356001600160401b03811115613a2c57600080fd5b6020830191508360208260061b85010111156139fc57600080fd5b803563ffffffff811681146133af57600080fd5b600080600080600080600080600060e08a8c031215613a7957600080fd5b89356001600160401b0380821115613a9057600080fd5b613a9c8d838e0161398c565b9a5060208c0135915080821115613ab257600080fd5b613abe8d838e016139a5565b995060408c0135915080821115613ad457600080fd5b613ae08d838e016139b8565b909950975060608c0135915080821115613af957600080fd5b50613b068c828d01613a03565b9096509450613b19905060808b01613a47565b9250613b2760a08b01613a47565b9150613b3560c08b01613a47565b90509295985092959850929598565b6000610180828403121561399f57600080fd5b60008060006101408486031215613b6d57600080fd5b83356001600160401b0380821115613b8457600080fd5b613b908783880161398c565b9450613b9f876020880161398c565b9350610120860135915080821115613bb657600080fd5b50613bc386828701613b44565b9150509250925092565b60006001600160401b03821115613be657613be66136cb565b5060051b60200190565b600082601f830112613c0157600080fd5b81356020613c16613c1183613bcd565b61372c565b82815260059290921b84018101918181019086841115613c3557600080fd5b8286015b84811015613c5757613c4a81613a47565b8352918301918301613c39565b509695505050505050565b600082601f830112613c7357600080fd5b81356020613c83613c1183613bcd565b82815260069290921b84018101918181019086841115613ca257600080fd5b8286015b84811015613c5757613cb8888261375c565b835291830191604001613ca6565b600082601f830112613cd757600080fd5b81356020613ce7613c1183613bcd565b82815260059290921b84018101918181019086841115613d0657600080fd5b8286015b84811015613c575780356001600160401b03811115613d295760008081fd5b613d378986838b0101613bf0565b845250918301918301613d0a565b60006101808284031215613d5857600080fd5b613d60613709565b905081356001600160401b0380821115613d7957600080fd5b613d8585838601613bf0565b83526020840135915080821115613d9b57600080fd5b613da785838601613c62565b60208401526040840135915080821115613dc057600080fd5b613dcc85838601613c62565b6040840152613dde85606086016137f0565b6060840152613df08560e0860161375c565b6080840152610120840135915080821115613e0a57600080fd5b613e1685838601613bf0565b60a0840152610140840135915080821115613e3057600080fd5b613e3c85838601613bf0565b60c0840152610160840135915080821115613e5657600080fd5b50613e6384828501613cc6565b60e08301525092915050565b60008060408385031215613e8257600080fd5b8235915060208301356001600160401b03811115613e9f57600080fd5b613eab85828601613d45565b9150509250929050565b600081518084526020808501945080840160005b83811015613eee5781516001600160601b031687529582019590820190600101613ec9565b509495945050505050565b602081526000825160406020840152613f156060840182613eb5565b90506020840151601f19848303016040850152613f328282613eb5565b95945050505050565b60008060008084860360c0811215613f5257600080fd5b85356001600160401b0380821115613f6957600080fd5b613f7589838a0161398c565b96506060601f1984011215613f8957600080fd5b6020880195506080880135925080831115613fa357600080fd5b613faf89848a01613b44565b945060a0880135925080831115613fc557600080fd5b5050613fd3878288016139a5565b91505092959194509250565b600060208284031215613ff157600080fd5b81516134da81613680565b6020808252602a908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526939903ab73830bab9b2b960b11b606082015260800190565b60006020828403121561405857600080fd5b81516134da816138d6565b60208082526028908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526739903830bab9b2b960c11b606082015260800190565b634e487b7160e01b600052603260045260246000fd5b6000826140de57634e487b7160e01b600052601260045260246000fd5b500690565b600181811c908216806140f757607f821691505b60208210810361399f57634e487b7160e01b600052602260045260246000fd5b60006020828403121561412957600080fd5b6134da82613a47565b6000808335601e1984360301811261414957600080fd5b8301803591506001600160401b0382111561416357600080fd5b6020019150368190038213156139fc57600080fd5b828152606081016134da602083018480358252602090810135910152565b634e487b7160e01b600052601160045260246000fd5b6000600182016141be576141be614196565b5060010190565b80356001600160401b03811681146133af57600080fd5b6000602082840312156141ee57600080fd5b6134da826141c5565b6000808335601e1984360301811261420e57600080fd5b83016020810192503590506001600160401b0381111561422d57600080fd5b8036038313156139fc57600080fd5b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b6020815263ffffffff61427783613a47565b1660208201526001600160401b03614291602084016141c5565b16604082015260006142a560408401613a47565b63ffffffff81166060840152506142be60608401613a47565b63ffffffff81166080840152506142d760808401613a47565b63ffffffff811660a0840152506142f060a08401613a47565b63ffffffff811660c08401525061430a60c08401846141f7565b6101008060e08601526143226101208601838561423c565b925061433060e08701613a47565b63ffffffff169401939093529392505050565b610100810163ffffffff8061435785613a47565b168352602084013560208401526001600160401b03614378604086016141c5565b1660408401528061438b60608601613a47565b166060840152506080830135608083015260a083013560a083015260c083013560c083015260e08301356143be81613680565b6001600160a01b031660e09290920191909152919050565b6000611a043683613d45565b60006001600160601b038083168185168183048111821515161561440857614408614196565b02949350505050565b600081600019048311821515161561442b5761442b614196565b500290565b600063ffffffff80831681851680830382111561444f5761444f614196565b01949350505050565b60008282101561446a5761446a614196565b500390565b60006001600160601b038381169083168181101561448f5761448f614196565b039392505050565b6020815263ffffffff6144a983613a47565b16602082015260006144bd60208401613a47565b63ffffffff81166040840152506144d660408401613a47565b63ffffffff81166060840152506144ef60608401613a47565b63ffffffff811660808401525061450960808401846141f7565b6101008060a08601526145216101208601838561423c565b925061452f60a08701613a47565b63ffffffff811660c0870152915061454a60c08701876141f7565b868503601f190160e0880152925061456384848361423c565b93505061433060e08701613a47565b6000808335601e1984360301811261458957600080fd5b83016020810192503590506001600160401b038111156145a857600080fd5b8060051b36038313156139fc57600080fd5b8183526000602080850194508260005b85811015613eee5760ff6145dd83613654565b16875295820195908201906001016145ca565b6000808335601e1984360301811261460757600080fd5b83016020810192503590506001600160401b0381111561462657600080fd5b8060071b36038313156139fc57600080fd5b80356001600160601b03811681146133af57600080fd5b8183526000602080850194508260005b85811015613eee5760ff61467283613654565b1687526001600160601b03614688848401614638565b168388015260408281013590880152606080830135908801526080968701969091019060010161465f565b6000808335601e198436030181126146ca57600080fd5b83016020810192503590506001600160401b038111156146e957600080fd5b8060061b36038313156139fc57600080fd5b8183526000602080850194508260005b85811015613eee5760ff61471e83613654565b1687526001600160601b03614734848401614638565b1687840152604096870196919091019060010161470b565b6000808335601e1984360301811261476357600080fd5b83016020810192503590506001600160401b0381111561478257600080fd5b6060810236038313156139fc57600080fd5b8183526000602080850194508260005b85811015613eee5760ff6147b783613654565b1687526147d283880184840180358252602090810135910152565b60609687019691909101906001016147a4565b81835260006001600160fb1b038311156147fe57600080fd5b8260051b8083602087013760009401602001938452509192915050565b8183526000602080850194508260005b85811015613eee576001600160601b0361484483614638565b168752958201959082019060010161482b565b818352600060208085019450848460051b8601846000805b88811015614905578484038a528235607e1989360301811261488f578283fd5b88018035855260806148a388830183614572565b828a8901526148b583890182846145ba565b9250505060406148c781840184614572565b888403838a01526148d984828461481b565b9350505050606060ff6148ed828501613654565b1696019590955250988501989185019160010161486f565b509198975050505050505050565b60008383855260208086019550808560051b830101846000805b8881101561490557858403601f19018a52823536899003605e19018112614952578283fd5b880180358552606061496687830183614572565b828989015261497883890182846145ba565b92505050604061498a81840184614572565b93508783038289015261499e83858361481b565b9d89019d9750505093860193505060010161492d565b8183526000602080850194508260005b85811015613eee578135875260ff6149dd848401613654565b168784015260409687019691909101906001016149c4565b60208152614a0f60208201614a09846138e4565b15159052565b6000614a1e6020840184614572565b610120806040860152614a36610140860183856145ba565b9250614a4560408701876145f0565b9250601f1980878603016060880152614a5f85858461464f565b9450614a6e60608901896146b3565b9450915080878603016080880152614a878585846146fb565b9450614a96608089018961474c565b94509150808786030160a0880152614aaf858584614794565b9450614abe60a0890189614572565b94509150808786030160c0880152614ad78585846147e5565b9450614ae660c0890189614572565b94509150808786030160e0880152614aff858584614857565b9450614b0e60e0890189614572565b94509150610100818887030181890152614b29868685614913565b9550614b37818a018a6146b3565b955092505080878603018388015250614b518484836149b4565b979650505050505050565b6060810163ffffffff614b6e84613a47565b168252602083013560208301526040830135604083015292915050565b60008219821115614b9e57614b9e614196565b500190565b6000808335601e19843603018112614bba57600080fd5b8301803591506001600160401b03821115614bd457600080fd5b6020019150600581901b36038213156139fc57600080fd5b6000808335601e19843603018112614c0357600080fd5b8301803591506001600160401b03821115614c1d57600080fd5b6020019150600781901b36038213156139fc57600080fd5b600060208284031215614c4757600080fd5b6134da82614638565b6000808335601e19843603018112614c6757600080fd5b8301803591506001600160401b03821115614c8157600080fd5b6020019150600681901b36038213156139fc57600080fd5b6000808335601e19843603018112614cb057600080fd5b8301803591506001600160401b03821115614cca57600080fd5b60200191506060810236038213156139fc57600080fd5b60008235607e19833603018112614cf757600080fd5b9190910192915050565b60008235605e19833603018112614cf757600080fdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220afe104c5256db80872fb833a875aa7965656078c8b6f1df27a47081402bb966864736f6c634300080d0033",
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

// AllowNonRootInit is a free data retrieval call binding the contract method 0x0ee0fdbd.
//
// Solidity: function allowNonRootInit() view returns(bool)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) AllowNonRootInit(opts *bind.CallOpts) (bool, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "allowNonRootInit")

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// AllowNonRootInit is a free data retrieval call binding the contract method 0x0ee0fdbd.
//
// Solidity: function allowNonRootInit() view returns(bool)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) AllowNonRootInit() (bool, error) {
	return _ContractGaspMultiRollupService.Contract.AllowNonRootInit(&_ContractGaspMultiRollupService.CallOpts)
}

// AllowNonRootInit is a free data retrieval call binding the contract method 0x0ee0fdbd.
//
// Solidity: function allowNonRootInit() view returns(bool)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) AllowNonRootInit() (bool, error) {
	return _ContractGaspMultiRollupService.Contract.AllowNonRootInit(&_ContractGaspMultiRollupService.CallOpts)
}

// ChainId is a free data retrieval call binding the contract method 0x9a8a0592.
//
// Solidity: function chainId() view returns(uint8)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) ChainId(opts *bind.CallOpts) (uint8, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "chainId")

	if err != nil {
		return *new(uint8), err
	}

	out0 := *abi.ConvertType(out[0], new(uint8)).(*uint8)

	return out0, err

}

// ChainId is a free data retrieval call binding the contract method 0x9a8a0592.
//
// Solidity: function chainId() view returns(uint8)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) ChainId() (uint8, error) {
	return _ContractGaspMultiRollupService.Contract.ChainId(&_ContractGaspMultiRollupService.CallOpts)
}

// ChainId is a free data retrieval call binding the contract method 0x9a8a0592.
//
// Solidity: function chainId() view returns(uint8)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) ChainId() (uint8, error) {
	return _ContractGaspMultiRollupService.Contract.ChainId(&_ContractGaspMultiRollupService.CallOpts)
}

// ChainRdBatchNonce is a free data retrieval call binding the contract method 0xdeb4037d.
//
// Solidity: function chainRdBatchNonce() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) ChainRdBatchNonce(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "chainRdBatchNonce")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// ChainRdBatchNonce is a free data retrieval call binding the contract method 0xdeb4037d.
//
// Solidity: function chainRdBatchNonce() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) ChainRdBatchNonce() (uint32, error) {
	return _ContractGaspMultiRollupService.Contract.ChainRdBatchNonce(&_ContractGaspMultiRollupService.CallOpts)
}

// ChainRdBatchNonce is a free data retrieval call binding the contract method 0xdeb4037d.
//
// Solidity: function chainRdBatchNonce() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) ChainRdBatchNonce() (uint32, error) {
	return _ContractGaspMultiRollupService.Contract.ChainRdBatchNonce(&_ContractGaspMultiRollupService.CallOpts)
}

// CheckSignatures is a free data retrieval call binding the contract method 0x7d978897.
//
// Solidity: function checkSignatures(bytes32 msgHash, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) params) view returns((uint96[],uint96[]))
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) CheckSignatures(opts *bind.CallOpts, msgHash [32]byte, params IBLSSignatureCheckerNonSignerStakesAndSignature) (IBLSSignatureCheckerQuorumStakeTotals, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "checkSignatures", msgHash, params)

	if err != nil {
		return *new(IBLSSignatureCheckerQuorumStakeTotals), err
	}

	out0 := *abi.ConvertType(out[0], new(IBLSSignatureCheckerQuorumStakeTotals)).(*IBLSSignatureCheckerQuorumStakeTotals)

	return out0, err

}

// CheckSignatures is a free data retrieval call binding the contract method 0x7d978897.
//
// Solidity: function checkSignatures(bytes32 msgHash, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) params) view returns((uint96[],uint96[]))
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) CheckSignatures(msgHash [32]byte, params IBLSSignatureCheckerNonSignerStakesAndSignature) (IBLSSignatureCheckerQuorumStakeTotals, error) {
	return _ContractGaspMultiRollupService.Contract.CheckSignatures(&_ContractGaspMultiRollupService.CallOpts, msgHash, params)
}

// CheckSignatures is a free data retrieval call binding the contract method 0x7d978897.
//
// Solidity: function checkSignatures(bytes32 msgHash, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) params) view returns((uint96[],uint96[]))
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) CheckSignatures(msgHash [32]byte, params IBLSSignatureCheckerNonSignerStakesAndSignature) (IBLSSignatureCheckerQuorumStakeTotals, error) {
	return _ContractGaspMultiRollupService.Contract.CheckSignatures(&_ContractGaspMultiRollupService.CallOpts, msgHash, params)
}

// LastOpUpdateBlockTimestamp is a free data retrieval call binding the contract method 0xf84e91fc.
//
// Solidity: function lastOpUpdateBlockTimestamp() view returns(uint256)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) LastOpUpdateBlockTimestamp(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "lastOpUpdateBlockTimestamp")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// LastOpUpdateBlockTimestamp is a free data retrieval call binding the contract method 0xf84e91fc.
//
// Solidity: function lastOpUpdateBlockTimestamp() view returns(uint256)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) LastOpUpdateBlockTimestamp() (*big.Int, error) {
	return _ContractGaspMultiRollupService.Contract.LastOpUpdateBlockTimestamp(&_ContractGaspMultiRollupService.CallOpts)
}

// LastOpUpdateBlockTimestamp is a free data retrieval call binding the contract method 0xf84e91fc.
//
// Solidity: function lastOpUpdateBlockTimestamp() view returns(uint256)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) LastOpUpdateBlockTimestamp() (*big.Int, error) {
	return _ContractGaspMultiRollupService.Contract.LastOpUpdateBlockTimestamp(&_ContractGaspMultiRollupService.CallOpts)
}

// LatestCompletedOpTaskCreatedBlock is a free data retrieval call binding the contract method 0x6f0c30a4.
//
// Solidity: function latestCompletedOpTaskCreatedBlock() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) LatestCompletedOpTaskCreatedBlock(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "latestCompletedOpTaskCreatedBlock")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// LatestCompletedOpTaskCreatedBlock is a free data retrieval call binding the contract method 0x6f0c30a4.
//
// Solidity: function latestCompletedOpTaskCreatedBlock() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) LatestCompletedOpTaskCreatedBlock() (uint32, error) {
	return _ContractGaspMultiRollupService.Contract.LatestCompletedOpTaskCreatedBlock(&_ContractGaspMultiRollupService.CallOpts)
}

// LatestCompletedOpTaskCreatedBlock is a free data retrieval call binding the contract method 0x6f0c30a4.
//
// Solidity: function latestCompletedOpTaskCreatedBlock() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) LatestCompletedOpTaskCreatedBlock() (uint32, error) {
	return _ContractGaspMultiRollupService.Contract.LatestCompletedOpTaskCreatedBlock(&_ContractGaspMultiRollupService.CallOpts)
}

// LatestCompletedOpTaskNumber is a free data retrieval call binding the contract method 0xe2a7cb66.
//
// Solidity: function latestCompletedOpTaskNumber() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) LatestCompletedOpTaskNumber(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "latestCompletedOpTaskNumber")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// LatestCompletedOpTaskNumber is a free data retrieval call binding the contract method 0xe2a7cb66.
//
// Solidity: function latestCompletedOpTaskNumber() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) LatestCompletedOpTaskNumber() (uint32, error) {
	return _ContractGaspMultiRollupService.Contract.LatestCompletedOpTaskNumber(&_ContractGaspMultiRollupService.CallOpts)
}

// LatestCompletedOpTaskNumber is a free data retrieval call binding the contract method 0xe2a7cb66.
//
// Solidity: function latestCompletedOpTaskNumber() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) LatestCompletedOpTaskNumber() (uint32, error) {
	return _ContractGaspMultiRollupService.Contract.LatestCompletedOpTaskNumber(&_ContractGaspMultiRollupService.CallOpts)
}

// LatestCompletedRdTaskCreatedBlock is a free data retrieval call binding the contract method 0x0bf16410.
//
// Solidity: function latestCompletedRdTaskCreatedBlock() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) LatestCompletedRdTaskCreatedBlock(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "latestCompletedRdTaskCreatedBlock")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// LatestCompletedRdTaskCreatedBlock is a free data retrieval call binding the contract method 0x0bf16410.
//
// Solidity: function latestCompletedRdTaskCreatedBlock() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) LatestCompletedRdTaskCreatedBlock() (uint32, error) {
	return _ContractGaspMultiRollupService.Contract.LatestCompletedRdTaskCreatedBlock(&_ContractGaspMultiRollupService.CallOpts)
}

// LatestCompletedRdTaskCreatedBlock is a free data retrieval call binding the contract method 0x0bf16410.
//
// Solidity: function latestCompletedRdTaskCreatedBlock() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) LatestCompletedRdTaskCreatedBlock() (uint32, error) {
	return _ContractGaspMultiRollupService.Contract.LatestCompletedRdTaskCreatedBlock(&_ContractGaspMultiRollupService.CallOpts)
}

// LatestCompletedRdTaskNumber is a free data retrieval call binding the contract method 0xd03a07b2.
//
// Solidity: function latestCompletedRdTaskNumber() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) LatestCompletedRdTaskNumber(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "latestCompletedRdTaskNumber")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// LatestCompletedRdTaskNumber is a free data retrieval call binding the contract method 0xd03a07b2.
//
// Solidity: function latestCompletedRdTaskNumber() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) LatestCompletedRdTaskNumber() (uint32, error) {
	return _ContractGaspMultiRollupService.Contract.LatestCompletedRdTaskNumber(&_ContractGaspMultiRollupService.CallOpts)
}

// LatestCompletedRdTaskNumber is a free data retrieval call binding the contract method 0xd03a07b2.
//
// Solidity: function latestCompletedRdTaskNumber() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) LatestCompletedRdTaskNumber() (uint32, error) {
	return _ContractGaspMultiRollupService.Contract.LatestCompletedRdTaskNumber(&_ContractGaspMultiRollupService.CallOpts)
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

// Rolldown is a free data retrieval call binding the contract method 0x3d9fb00c.
//
// Solidity: function rolldown() view returns(address)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) Rolldown(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "rolldown")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Rolldown is a free data retrieval call binding the contract method 0x3d9fb00c.
//
// Solidity: function rolldown() view returns(address)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) Rolldown() (common.Address, error) {
	return _ContractGaspMultiRollupService.Contract.Rolldown(&_ContractGaspMultiRollupService.CallOpts)
}

// Rolldown is a free data retrieval call binding the contract method 0x3d9fb00c.
//
// Solidity: function rolldown() view returns(address)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) Rolldown() (common.Address, error) {
	return _ContractGaspMultiRollupService.Contract.Rolldown(&_ContractGaspMultiRollupService.CallOpts)
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

// Initialize is a paid mutator transaction binding the contract method 0x390f7570.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _updater, bool _allowNonRootInit, address _rolldown) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) Initialize(opts *bind.TransactOpts, _pauserRegistry common.Address, initialOwner common.Address, _updater common.Address, _allowNonRootInit bool, _rolldown common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "initialize", _pauserRegistry, initialOwner, _updater, _allowNonRootInit, _rolldown)
}

// Initialize is a paid mutator transaction binding the contract method 0x390f7570.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _updater, bool _allowNonRootInit, address _rolldown) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) Initialize(_pauserRegistry common.Address, initialOwner common.Address, _updater common.Address, _allowNonRootInit bool, _rolldown common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.Initialize(&_ContractGaspMultiRollupService.TransactOpts, _pauserRegistry, initialOwner, _updater, _allowNonRootInit, _rolldown)
}

// Initialize is a paid mutator transaction binding the contract method 0x390f7570.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _updater, bool _allowNonRootInit, address _rolldown) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) Initialize(_pauserRegistry common.Address, initialOwner common.Address, _updater common.Address, _allowNonRootInit bool, _rolldown common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.Initialize(&_ContractGaspMultiRollupService.TransactOpts, _pauserRegistry, initialOwner, _updater, _allowNonRootInit, _rolldown)
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

// ProcessEigenOpUpdate is a paid mutator transaction binding the contract method 0xd30270ef.
//
// Solidity: function processEigenOpUpdate((uint32,uint32,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature, (bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) operatorStateInfo) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) ProcessEigenOpUpdate(opts *bind.TransactOpts, task IFinalizerTaskManagerOpTask, taskResponse IFinalizerTaskManagerOpTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature, operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "processEigenOpUpdate", task, taskResponse, nonSignerStakesAndSignature, operatorStateInfo)
}

// ProcessEigenOpUpdate is a paid mutator transaction binding the contract method 0xd30270ef.
//
// Solidity: function processEigenOpUpdate((uint32,uint32,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature, (bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) operatorStateInfo) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) ProcessEigenOpUpdate(task IFinalizerTaskManagerOpTask, taskResponse IFinalizerTaskManagerOpTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature, operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.ProcessEigenOpUpdate(&_ContractGaspMultiRollupService.TransactOpts, task, taskResponse, nonSignerStakesAndSignature, operatorStateInfo)
}

// ProcessEigenOpUpdate is a paid mutator transaction binding the contract method 0xd30270ef.
//
// Solidity: function processEigenOpUpdate((uint32,uint32,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature, (bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) operatorStateInfo) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) ProcessEigenOpUpdate(task IFinalizerTaskManagerOpTask, taskResponse IFinalizerTaskManagerOpTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature, operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.ProcessEigenOpUpdate(&_ContractGaspMultiRollupService.TransactOpts, task, taskResponse, nonSignerStakesAndSignature, operatorStateInfo)
}

// ProcessEigenRdUpdate is a paid mutator transaction binding the contract method 0x738491d9.
//
// Solidity: function processEigenRdUpdate((uint32,uint64,uint32,uint32,uint32,uint32,bytes,uint32) task, (uint32,bytes32,uint64,uint32,bytes32,uint256,uint256,address) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) ProcessEigenRdUpdate(opts *bind.TransactOpts, task IFinalizerTaskManagerRdTask, taskResponse IFinalizerTaskManagerRdTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "processEigenRdUpdate", task, taskResponse, nonSignerStakesAndSignature)
}

// ProcessEigenRdUpdate is a paid mutator transaction binding the contract method 0x738491d9.
//
// Solidity: function processEigenRdUpdate((uint32,uint64,uint32,uint32,uint32,uint32,bytes,uint32) task, (uint32,bytes32,uint64,uint32,bytes32,uint256,uint256,address) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) ProcessEigenRdUpdate(task IFinalizerTaskManagerRdTask, taskResponse IFinalizerTaskManagerRdTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.ProcessEigenRdUpdate(&_ContractGaspMultiRollupService.TransactOpts, task, taskResponse, nonSignerStakesAndSignature)
}

// ProcessEigenRdUpdate is a paid mutator transaction binding the contract method 0x738491d9.
//
// Solidity: function processEigenRdUpdate((uint32,uint64,uint32,uint32,uint32,uint32,bytes,uint32) task, (uint32,bytes32,uint64,uint32,bytes32,uint256,uint256,address) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) ProcessEigenRdUpdate(task IFinalizerTaskManagerRdTask, taskResponse IFinalizerTaskManagerRdTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.ProcessEigenRdUpdate(&_ContractGaspMultiRollupService.TransactOpts, task, taskResponse, nonSignerStakesAndSignature)
}

// ProcessEigenReinit is a paid mutator transaction binding the contract method 0x56b933d9.
//
// Solidity: function processEigenReinit((uint32,uint32,uint32,uint32,bytes,uint32,bytes,uint32) task, (bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) operatorStateInfo, bytes32[] merkleRoots, (uint256,uint256)[] ranges, uint32 _chainRdBatchNonce, uint32 _latestCompletedRdTaskNumber, uint32 _latestCompletedRdTaskCreatedBlock) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) ProcessEigenReinit(opts *bind.TransactOpts, task IFinalizerTaskManagerOpTask, operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo, merkleRoots [][32]byte, ranges []IRolldownPrimitivesRange, _chainRdBatchNonce uint32, _latestCompletedRdTaskNumber uint32, _latestCompletedRdTaskCreatedBlock uint32) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "processEigenReinit", task, operatorStateInfo, merkleRoots, ranges, _chainRdBatchNonce, _latestCompletedRdTaskNumber, _latestCompletedRdTaskCreatedBlock)
}

// ProcessEigenReinit is a paid mutator transaction binding the contract method 0x56b933d9.
//
// Solidity: function processEigenReinit((uint32,uint32,uint32,uint32,bytes,uint32,bytes,uint32) task, (bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) operatorStateInfo, bytes32[] merkleRoots, (uint256,uint256)[] ranges, uint32 _chainRdBatchNonce, uint32 _latestCompletedRdTaskNumber, uint32 _latestCompletedRdTaskCreatedBlock) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) ProcessEigenReinit(task IFinalizerTaskManagerOpTask, operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo, merkleRoots [][32]byte, ranges []IRolldownPrimitivesRange, _chainRdBatchNonce uint32, _latestCompletedRdTaskNumber uint32, _latestCompletedRdTaskCreatedBlock uint32) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.ProcessEigenReinit(&_ContractGaspMultiRollupService.TransactOpts, task, operatorStateInfo, merkleRoots, ranges, _chainRdBatchNonce, _latestCompletedRdTaskNumber, _latestCompletedRdTaskCreatedBlock)
}

// ProcessEigenReinit is a paid mutator transaction binding the contract method 0x56b933d9.
//
// Solidity: function processEigenReinit((uint32,uint32,uint32,uint32,bytes,uint32,bytes,uint32) task, (bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) operatorStateInfo, bytes32[] merkleRoots, (uint256,uint256)[] ranges, uint32 _chainRdBatchNonce, uint32 _latestCompletedRdTaskNumber, uint32 _latestCompletedRdTaskCreatedBlock) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) ProcessEigenReinit(task IFinalizerTaskManagerOpTask, operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo, merkleRoots [][32]byte, ranges []IRolldownPrimitivesRange, _chainRdBatchNonce uint32, _latestCompletedRdTaskNumber uint32, _latestCompletedRdTaskCreatedBlock uint32) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.ProcessEigenReinit(&_ContractGaspMultiRollupService.TransactOpts, task, operatorStateInfo, merkleRoots, ranges, _chainRdBatchNonce, _latestCompletedRdTaskNumber, _latestCompletedRdTaskCreatedBlock)
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

// SetRolldown is a paid mutator transaction binding the contract method 0xfdc15de8.
//
// Solidity: function setRolldown(address _rolldown) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) SetRolldown(opts *bind.TransactOpts, _rolldown common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "setRolldown", _rolldown)
}

// SetRolldown is a paid mutator transaction binding the contract method 0xfdc15de8.
//
// Solidity: function setRolldown(address _rolldown) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) SetRolldown(_rolldown common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.SetRolldown(&_ContractGaspMultiRollupService.TransactOpts, _rolldown)
}

// SetRolldown is a paid mutator transaction binding the contract method 0xfdc15de8.
//
// Solidity: function setRolldown(address _rolldown) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) SetRolldown(_rolldown common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.SetRolldown(&_ContractGaspMultiRollupService.TransactOpts, _rolldown)
}

// SetUpdater is a paid mutator transaction binding the contract method 0x9d54f419.
//
// Solidity: function setUpdater(address _updater) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) SetUpdater(opts *bind.TransactOpts, _updater common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "setUpdater", _updater)
}

// SetUpdater is a paid mutator transaction binding the contract method 0x9d54f419.
//
// Solidity: function setUpdater(address _updater) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) SetUpdater(_updater common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.SetUpdater(&_ContractGaspMultiRollupService.TransactOpts, _updater)
}

// SetUpdater is a paid mutator transaction binding the contract method 0x9d54f419.
//
// Solidity: function setUpdater(address _updater) returns()
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

// ContractGaspMultiRollupServiceEigenOpUpdateProcessedIterator is returned from FilterEigenOpUpdateProcessed and is used to iterate over the raw logs and unpacked data for EigenOpUpdateProcessed events raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceEigenOpUpdateProcessedIterator struct {
	Event *ContractGaspMultiRollupServiceEigenOpUpdateProcessed // Event containing the contract specifics and raw log

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
func (it *ContractGaspMultiRollupServiceEigenOpUpdateProcessedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractGaspMultiRollupServiceEigenOpUpdateProcessed)
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
		it.Event = new(ContractGaspMultiRollupServiceEigenOpUpdateProcessed)
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
func (it *ContractGaspMultiRollupServiceEigenOpUpdateProcessedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractGaspMultiRollupServiceEigenOpUpdateProcessedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractGaspMultiRollupServiceEigenOpUpdateProcessed represents a EigenOpUpdateProcessed event raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceEigenOpUpdateProcessed struct {
	TaskNumber       uint32
	TaskCreatedBlock uint32
	Raw              types.Log // Blockchain specific contextual infos
}

// FilterEigenOpUpdateProcessed is a free log retrieval operation binding the contract event 0x36a1fd7bd554f5c428c9829c09c6606b4c893b1fadc8735a7a12795797447ded.
//
// Solidity: event EigenOpUpdateProcessed(uint32 taskNumber, uint32 taskCreatedBlock)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) FilterEigenOpUpdateProcessed(opts *bind.FilterOpts) (*ContractGaspMultiRollupServiceEigenOpUpdateProcessedIterator, error) {

	logs, sub, err := _ContractGaspMultiRollupService.contract.FilterLogs(opts, "EigenOpUpdateProcessed")
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupServiceEigenOpUpdateProcessedIterator{contract: _ContractGaspMultiRollupService.contract, event: "EigenOpUpdateProcessed", logs: logs, sub: sub}, nil
}

// WatchEigenOpUpdateProcessed is a free log subscription operation binding the contract event 0x36a1fd7bd554f5c428c9829c09c6606b4c893b1fadc8735a7a12795797447ded.
//
// Solidity: event EigenOpUpdateProcessed(uint32 taskNumber, uint32 taskCreatedBlock)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) WatchEigenOpUpdateProcessed(opts *bind.WatchOpts, sink chan<- *ContractGaspMultiRollupServiceEigenOpUpdateProcessed) (event.Subscription, error) {

	logs, sub, err := _ContractGaspMultiRollupService.contract.WatchLogs(opts, "EigenOpUpdateProcessed")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractGaspMultiRollupServiceEigenOpUpdateProcessed)
				if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "EigenOpUpdateProcessed", log); err != nil {
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

// ParseEigenOpUpdateProcessed is a log parse operation binding the contract event 0x36a1fd7bd554f5c428c9829c09c6606b4c893b1fadc8735a7a12795797447ded.
//
// Solidity: event EigenOpUpdateProcessed(uint32 taskNumber, uint32 taskCreatedBlock)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) ParseEigenOpUpdateProcessed(log types.Log) (*ContractGaspMultiRollupServiceEigenOpUpdateProcessed, error) {
	event := new(ContractGaspMultiRollupServiceEigenOpUpdateProcessed)
	if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "EigenOpUpdateProcessed", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractGaspMultiRollupServiceEigenRdUpdateProcessedIterator is returned from FilterEigenRdUpdateProcessed and is used to iterate over the raw logs and unpacked data for EigenRdUpdateProcessed events raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceEigenRdUpdateProcessedIterator struct {
	Event *ContractGaspMultiRollupServiceEigenRdUpdateProcessed // Event containing the contract specifics and raw log

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
func (it *ContractGaspMultiRollupServiceEigenRdUpdateProcessedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractGaspMultiRollupServiceEigenRdUpdateProcessed)
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
		it.Event = new(ContractGaspMultiRollupServiceEigenRdUpdateProcessed)
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
func (it *ContractGaspMultiRollupServiceEigenRdUpdateProcessedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractGaspMultiRollupServiceEigenRdUpdateProcessedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractGaspMultiRollupServiceEigenRdUpdateProcessed represents a EigenRdUpdateProcessed event raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceEigenRdUpdateProcessed struct {
	TaskNumber       uint32
	TaskCreatedBlock uint32
	Raw              types.Log // Blockchain specific contextual infos
}

// FilterEigenRdUpdateProcessed is a free log retrieval operation binding the contract event 0xec68db391879b0f9f420d1cdf3476afbdf085a2462bf4d2b11df78466295cb17.
//
// Solidity: event EigenRdUpdateProcessed(uint32 taskNumber, uint32 taskCreatedBlock)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) FilterEigenRdUpdateProcessed(opts *bind.FilterOpts) (*ContractGaspMultiRollupServiceEigenRdUpdateProcessedIterator, error) {

	logs, sub, err := _ContractGaspMultiRollupService.contract.FilterLogs(opts, "EigenRdUpdateProcessed")
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupServiceEigenRdUpdateProcessedIterator{contract: _ContractGaspMultiRollupService.contract, event: "EigenRdUpdateProcessed", logs: logs, sub: sub}, nil
}

// WatchEigenRdUpdateProcessed is a free log subscription operation binding the contract event 0xec68db391879b0f9f420d1cdf3476afbdf085a2462bf4d2b11df78466295cb17.
//
// Solidity: event EigenRdUpdateProcessed(uint32 taskNumber, uint32 taskCreatedBlock)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) WatchEigenRdUpdateProcessed(opts *bind.WatchOpts, sink chan<- *ContractGaspMultiRollupServiceEigenRdUpdateProcessed) (event.Subscription, error) {

	logs, sub, err := _ContractGaspMultiRollupService.contract.WatchLogs(opts, "EigenRdUpdateProcessed")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractGaspMultiRollupServiceEigenRdUpdateProcessed)
				if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "EigenRdUpdateProcessed", log); err != nil {
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

// ParseEigenRdUpdateProcessed is a log parse operation binding the contract event 0xec68db391879b0f9f420d1cdf3476afbdf085a2462bf4d2b11df78466295cb17.
//
// Solidity: event EigenRdUpdateProcessed(uint32 taskNumber, uint32 taskCreatedBlock)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) ParseEigenRdUpdateProcessed(log types.Log) (*ContractGaspMultiRollupServiceEigenRdUpdateProcessed, error) {
	event := new(ContractGaspMultiRollupServiceEigenRdUpdateProcessed)
	if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "EigenRdUpdateProcessed", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
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

// ContractGaspMultiRollupServiceRolldownTargetUpdatedIterator is returned from FilterRolldownTargetUpdated and is used to iterate over the raw logs and unpacked data for RolldownTargetUpdated events raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceRolldownTargetUpdatedIterator struct {
	Event *ContractGaspMultiRollupServiceRolldownTargetUpdated // Event containing the contract specifics and raw log

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
func (it *ContractGaspMultiRollupServiceRolldownTargetUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractGaspMultiRollupServiceRolldownTargetUpdated)
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
		it.Event = new(ContractGaspMultiRollupServiceRolldownTargetUpdated)
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
func (it *ContractGaspMultiRollupServiceRolldownTargetUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractGaspMultiRollupServiceRolldownTargetUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractGaspMultiRollupServiceRolldownTargetUpdated represents a RolldownTargetUpdated event raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceRolldownTargetUpdated struct {
	RolldownAddress common.Address
	Raw             types.Log // Blockchain specific contextual infos
}

// FilterRolldownTargetUpdated is a free log retrieval operation binding the contract event 0x2f20cf1bda67739044c5bf577353970c3dbc183b2c7274d1e8584a1026923267.
//
// Solidity: event RolldownTargetUpdated(address rolldownAddress)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) FilterRolldownTargetUpdated(opts *bind.FilterOpts) (*ContractGaspMultiRollupServiceRolldownTargetUpdatedIterator, error) {

	logs, sub, err := _ContractGaspMultiRollupService.contract.FilterLogs(opts, "RolldownTargetUpdated")
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupServiceRolldownTargetUpdatedIterator{contract: _ContractGaspMultiRollupService.contract, event: "RolldownTargetUpdated", logs: logs, sub: sub}, nil
}

// WatchRolldownTargetUpdated is a free log subscription operation binding the contract event 0x2f20cf1bda67739044c5bf577353970c3dbc183b2c7274d1e8584a1026923267.
//
// Solidity: event RolldownTargetUpdated(address rolldownAddress)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) WatchRolldownTargetUpdated(opts *bind.WatchOpts, sink chan<- *ContractGaspMultiRollupServiceRolldownTargetUpdated) (event.Subscription, error) {

	logs, sub, err := _ContractGaspMultiRollupService.contract.WatchLogs(opts, "RolldownTargetUpdated")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractGaspMultiRollupServiceRolldownTargetUpdated)
				if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "RolldownTargetUpdated", log); err != nil {
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
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) ParseRolldownTargetUpdated(log types.Log) (*ContractGaspMultiRollupServiceRolldownTargetUpdated, error) {
	event := new(ContractGaspMultiRollupServiceRolldownTargetUpdated)
	if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "RolldownTargetUpdated", log); err != nil {
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

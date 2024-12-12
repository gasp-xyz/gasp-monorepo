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
	ABI: "[{\"type\":\"function\",\"name\":\"allowNonRootInit\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"chainId\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"chainRdBatchNonce\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"checkSignatures\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"params\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.QuorumStakeTotals\",\"components\":[{\"name\":\"signedStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"totalStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"_pauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"initialOwner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_updater\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_allowNonRootInit\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"_rolldown\",\"type\":\"address\",\"internalType\":\"contractIRolldown\"},{\"name\":\"_chainId\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"lastOpUpdateBlockTimestamp\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestCompletedOpTaskCreatedBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestCompletedOpTaskNumber\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestCompletedRdTaskNumber\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"operatorAndQuorumToStakes\",\"inputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint96\",\"internalType\":\"uint96\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"operatorIdQuorumCount\",\"inputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"pauseAll\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[{\"name\":\"index\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pauserRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"processEigenOpUpdate\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]},{\"name\":\"operatorStateInfo\",\"type\":\"tuple\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorStateInfo\",\"components\":[{\"name\":\"operatorsStateChanged\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"quorumsRemoved\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsAdded[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"quorumsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsStakeUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]},{\"name\":\"quorumsApkUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsApkUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"operatorsRemoved\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"operatorsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsAdded[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]},{\"name\":\"operatorsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsStakeUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"operatorsQuorumCountUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsQuorumCountUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"processEigenRdUpdate\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.RdTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"chainId\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"},{\"name\":\"batchId\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.RdTaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"chainId\",\"type\":\"uint8\",\"internalType\":\"enumIRolldownPrimitives.ChainId\"},{\"name\":\"batchId\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"rdUpdate\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"rangeStart\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"rangeEnd\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"updater\",\"type\":\"address\",\"internalType\":\"address\"}]},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"processEigenReinit\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.OpTask\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedOpTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedOpTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"operatorStateInfo\",\"type\":\"tuple\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorStateInfo\",\"components\":[{\"name\":\"operatorsStateChanged\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"quorumsRemoved\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsAdded[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"quorumsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsStakeUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]},{\"name\":\"quorumsApkUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsApkUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"operatorsRemoved\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"operatorsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsAdded[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]},{\"name\":\"operatorsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsStakeUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"operatorsQuorumCountUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsQuorumCountUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]}]},{\"name\":\"merkleRoots\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"ranges\",\"type\":\"tuple[]\",\"internalType\":\"structIRolldownPrimitives.Range[]\",\"components\":[{\"name\":\"start\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"end\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"lastBatchId\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"qourumApk\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"quorumNumbers\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"quorumThresholdPercentage\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"quorumToStakes\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint96\",\"internalType\":\"uint96\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"rolldown\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIRolldown\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"setPauserRegistry\",\"inputs\":[{\"name\":\"newPauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setRolldown\",\"inputs\":[{\"name\":\"_rolldown\",\"type\":\"address\",\"internalType\":\"contractIRolldown\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setUpdater\",\"inputs\":[{\"name\":\"_updater\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"stalled\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"trySignatureAndApkVerification\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"apk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}],\"outputs\":[{\"name\":\"pairingSuccessful\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"siganatureIsValid\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"unpause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updater\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"event\",\"name\":\"EigenOpUpdateProcessed\",\"inputs\":[{\"name\":\"taskNumber\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"EigenRdUpdateProcessed\",\"inputs\":[{\"name\":\"taskNumber\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"EigenReinitProcessed\",\"inputs\":[{\"name\":\"taskNumber\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Initialized\",\"inputs\":[{\"name\":\"version\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Paused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"PauserRegistrySet\",\"inputs\":[{\"name\":\"pauserRegistry\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"newPauserRegistry\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractIPauserRegistry\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RolldownTargetUpdated\",\"inputs\":[{\"name\":\"rolldownAddress\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Unpaused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false}]",
	Bin: "0x608060405234801561001057600080fd5b50614d63806100206000396000f3fe608060405234801561001057600080fd5b50600436106102065760003560e01c80636f0c30a41161011a578063d03a07b2116100ad578063ed39e5021161007c578063ed39e50214610544578063f2fde38b14610557578063f84e91fc1461056a578063fabc1cbc14610573578063fdc15de81461058657600080fd5b8063d03a07b2146104f2578063deb4037d14610502578063df034cd014610519578063e2a7cb661461052c57600080fd5b8063886f1195116100e9578063886f11951461049a5780638da5cb5b146104ad5780639a8a0592146104be5780639d54f419146104df57600080fd5b80636f0c30a414610432578063715018a6146104495780637ad75561146104515780637d9788971461047a57600080fd5b806334fadbea1161019d5780634deabc211161016c5780634deabc21146103bc578063526e3e64146103e1578063595c6a67146103f55780635ac86ab7146103fd5780635c975abb1461042057600080fd5b806334fadbea146102fd5780633d9fb00c14610310578063430d3b391461033b578063499d6fb61461037057600080fd5b8063171f1d5b116101d9578063171f1d5b146102985780632635e74e146102c25780632a8414fd146102d557806330c47d8e146102ea57600080fd5b806303d097d21461020b5780630ee0fdbd1461024c57806310d67a2f14610270578063136439dd14610285575b600080fd5b61023261021936600461367d565b609f602052600090815260409020805460019091015482565b604080519283526020830191909152015b60405180910390f35b60985461026090600160a81b900460ff1681565b6040519015158152602001610243565b61028361027e3660046136ad565b610599565b005b6102836102933660046136ca565b610655565b6102ab6102a6366004613848565b610794565b604080519215158352901515602083015201610243565b6102836102d0366004613923565b61091e565b6102dd610b79565b6040516102439190613a1d565b6102836102f8366004613a9a565b610c07565b61028361030b366004613b2b565b610daf565b609754610323906001600160a01b031681565b6040516001600160a01b039091168152602001610243565b61035e6103493660046136ca565b60a06020526000908152604090205460ff1681565b60405160ff9091168152602001610243565b6103a461037e366004613bcf565b609e6020908152600092835260408084209091529082529020546001600160601b031681565b6040516001600160601b039091168152602001610243565b609c546103cc9063ffffffff1681565b60405163ffffffff9091168152602001610243565b60985461026090600160a01b900460ff1681565b6102836112b4565b61026061040b36600461367d565b606654600160ff9092169190911b9081161490565b6066545b604051908152602001610243565b609a546103cc90600160401b900463ffffffff1681565b61028361137b565b6103a461045f36600461367d565b609d602052600090815260409020546001600160601b031681565b61048d610488366004613e9d565b61138f565b6040516102439190613f27565b606554610323906001600160a01b031681565b6033546001600160a01b0316610323565b6097546104d290600160c01b900460ff1681565b6040516102439190613fa1565b6102836104ed3660046136ad565b611979565b609a546103cc9063ffffffff1681565b6097546103cc90600160a01b900463ffffffff1681565b609854610323906001600160a01b031681565b609a546103cc90640100000000900463ffffffff1681565b610283610552366004613faf565b6119a3565b6102836105653660046136ad565b611f51565b61042460995481565b6102836105813660046136ca565b611fc7565b6102836105943660046136ad565b612123565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156105ec573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106109190614031565b6001600160a01b0316336001600160a01b0316146106495760405162461bcd60e51b81526004016106409061404e565b60405180910390fd5b6106528161217f565b50565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa15801561069d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106c19190614098565b6106dd5760405162461bcd60e51b8152600401610640906140b5565b606654818116146107565760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e70617573653a20696e76616c696420617474656d70742060448201527f746f20756e70617573652066756e6374696f6e616c69747900000000000000006064820152608401610640565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b60008060007f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001878760000151886020015188600001516000600281106107dc576107dc6140fd565b60200201518951600160200201518a60200151600060028110610801576108016140fd565b60200201518b6020015160016002811061081d5761081d6140fd565b602090810291909101518c518d83015160405161087a9a99989796959401988952602089019790975260408801959095526060870193909352608086019190915260a085015260c084015260e08301526101008201526101200190565b6040516020818303038152906040528051906020012060001c61089d9190614113565b90506109106108b66108af8884612276565b8690612307565b6108be61239c565b6109066108f7856108f1604080518082018252600080825260209182015281518083019092526001825260029082015290565b90612276565b6109008c61245c565b90612307565b886201d4c06124eb565b909890975095505050505050565b61092661270f565b8382146109755760405162461bcd60e51b815260206004820152601d60248201527f726455706461746520696e666f206c656e677468206d69736d617463680000006044820152606401610640565b61097e86612769565b61098b6020880188614135565b609a805463ffffffff929092166401000000000267ffffffff00000000199092169190911790556109c26040880160208901614135565b609a805463ffffffff92909216600160401b0263ffffffff60401b19909216919091179055426099556109f86060880188614150565b610a0491609b916134f9565b50610a1560a0880160808901614135565b609c805463ffffffff191663ffffffff9290921691909117905560005b84811015610ade576097546001600160a01b03166308f42d40878784818110610a5d57610a5d6140fd565b90506020020135868685818110610a7657610a766140fd565b9050604002016040518363ffffffff1660e01b8152600401610a99929190614196565b600060405180830381600087803b158015610ab357600080fd5b505af1158015610ac7573d6000803e3d6000fd5b505050508080610ad6906141ca565b915050610a32565b508315610b1157610af08160016141e3565b609760146101000a81548163ffffffff021916908363ffffffff1602179055505b7f264965eb6bc436c6c473431d34af56e832ec344fdfd43ee6af6fce6d205e84af610b3f6020890189614135565b610b4f60408a0160208b01614135565b6040805163ffffffff9384168152929091166020830152015b60405180910390a150505050505050565b609b8054610b869061420b565b80601f0160208091040260200160405190810160405280929190818152602001828054610bb29061420b565b8015610bff5780601f10610bd457610100808354040283529160200191610bff565b820191906000526020600020905b815481529060010190602001808311610be257829003601f168201915b505050505081565b600054610100900460ff1615808015610c275750600054600160ff909116105b80610c415750303b158015610c41575060005460ff166001145b610ca45760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610640565b6000805460ff191660011790558015610cc7576000805461ff0019166101001790555b610cd2876000613116565b610cdb866131fc565b60988054851515600160a81b02600161ff0160a01b03199091166001600160a01b038089169190911791909117909155609780549185166001600160a01b03198316811782558492600164ff0000000160a01b03191617600160c01b836002811115610d4957610d49613f69565b02179055506097805463ffffffff60a01b1916600160a01b1790558015610da6576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890602001610b68565b50505050505050565b60665415610dff5760405162461bcd60e51b815260206004820152601c60248201527f5061757361626c653a20636f6e747261637420697320706175736564000000006044820152606401610640565b6098546001600160a01b03163314610e595760405162461bcd60e51b815260206004820152601a60248201527f55706461746572206d757374206265207468652063616c6c65720000000000006044820152606401610640565b609a54600160401b900463ffffffff1680151580610e805750609854600160a81b900460ff165b15610ecc576098546001600160a01b03163314610ec75760405162461bcd60e51b8152602060048201526005602482015264041757468360dc1b6044820152606401610640565b610f0e565b6033546001600160a01b03163314610f0e5760405162461bcd60e51b8152602060048201526005602482015264417574683160d81b6044820152606401610640565b84604051602001610f1f91906142ad565b60405160208183030381529060405280519060200120846020013514610f875760405162461bcd60e51b815260206004820152601f60248201527f7265666572656e63655461736b486173682068617368206d69736d61746368006044820152606401610640565b81604051602001610f9891906147f2565b604051602081830303815290604052805190602001208460400135146110005760405162461bcd60e51b815260206004820152601f60248201527f6f70657261746f725374617465496e666f2068617368206d69736d61746368006044820152606401610640565b63ffffffff8116156111955761101c6060860160408701614135565b63ffffffff168163ffffffff16146110765760405162461bcd60e51b815260206004820152601860248201527f7265666572656e636520626c6f636b206d69736d6174636800000000000000006044820152606401610640565b60006110ac8560405160200161108c9190614959565b604051602081830303815290604052805190602001208561048890614988565b609c5490915063ffffffff1660005b609b80546110c89061420b565b9050811015611191578160ff16836020015182815181106110eb576110eb6140fd565b60200260200101516110fd9190614994565b6001600160601b031660648460000151838151811061111e5761111e6140fd565b60200260200101516001600160601b031661113991906149c3565b101561117f5760405162461bcd60e51b81526020600482015260156024820152744661696c656420746f206d6565742071756f72756d60581b6044820152606401610640565b80611189816141ca565b9150506110bb565b5050505b61119e82612769565b6111ab6020860186614135565b609a805463ffffffff929092166401000000000267ffffffff00000000199092169190911790556111e26040860160208701614135565b609a805463ffffffff92909216600160401b0263ffffffff60401b19909216919091179055426099556112186060860186614150565b61122491609b916134f9565b5061123560a0860160808701614135565b609c805463ffffffff191663ffffffff929092169190911790557f36a1fd7bd554f5c428c9829c09c6606b4c893b1fadc8735a7a12795797447ded61127d6020870187614135565b61128d6040880160208901614135565b6040805163ffffffff93841681529290911660208301520160405180910390a15050505050565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa1580156112fc573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906113209190614098565b61133c5760405162461bcd60e51b8152600401610640906140b5565b600019606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b61138361270f565b61138d60006131fc565b565b604080518082019091526060808252602082015260408051808201909152600080825260208201819052609b80546113c69061420b565b905090506113e7604051806040016040528060608152602001606081525090565b816001600160401b038111156113ff576113ff6136e3565b604051908082528060200260200182016040528015611428578160200160208202803683370190505b506020820152816001600160401b03811115611446576114466136e3565b60405190808252806020026020018201604052801561146f578160200160208202803683370190505b5081526020850151516000906001600160401b03811115611492576114926136e3565b6040519080825280602002602001820160405280156114bb578160200160208202803683370190505b5090506000805b87602001515181101561166d57611507886020015182815181106114e8576114e86140fd565b6020026020010151805160009081526020918201519091526040902090565b838281518110611519576115196140fd565b602090810291909101015280156115e357826115366001836149e2565b81518110611546576115466140fd565b602002602001015160001c838281518110611563576115636140fd565b602002602001015160001c116115e3576040805162461bcd60e51b81526020600482015260248101919091527f424c535369676e6174757265436865636b65722e636865636b5369676e61747560448201527f7265733a206e6f6e5369676e65725075626b657973206e6f7420736f727465646064820152608401610640565b61165961165260a060008685815181106115ff576115ff6140fd565b6020026020010151815260200190815260200160002060009054906101000a900460ff1660ff168a60200151848151811061163c5761163c6140fd565b602002602001015161324e90919063ffffffff16565b8790612307565b955080611665816141ca565b9150506114c2565b5061167785613331565b945060005b8481101561185b57609b8181546116929061420b565b81106116a0576116a06140fd565b8154600116156116bf5790600052602060002090602091828204019190065b9054600160f81b911a0260f81c6000818152609f60209081526040918290208251808401909352805483526001015490820152909250611700908790612307565b60ff83166000908152609d60209081526040909120549086015180519298506001600160601b03909116918390811061173b5761173b6140fd565b6001600160601b03909216602092830291909101820152840151805182908110611767576117676140fd565b602002602001015184600001518281518110611785576117856140fd565b60200260200101906001600160601b031690816001600160601b03168152505060005b88602001515181101561184857609e60008583815181106117cb576117cb6140fd565b6020908102919091018101518252818101929092526040908101600090812060ff87168252909252902054855180516001600160601b039092169184908110611816576118166140fd565b6020026020010181815161182a91906149f9565b6001600160601b031690525080611840816141ca565b9150506117a8565b5080611853816141ca565b91505061167c565b506000806118738a888b606001518c60800151610794565b91509150816118f65760405162461bcd60e51b815260206004820152604360248201527f424c535369676e6174757265436865636b65722e636865636b5369676e61747560448201527f7265733a2070616972696e6720707265636f6d70696c652063616c6c206661696064820152621b195960ea1b608482015260a401610640565b806119695760405162461bcd60e51b815260206004820152603960248201527f424c535369676e6174757265436865636b65722e636865636b5369676e61747560448201527f7265733a207369676e617475726520697320696e76616c6964000000000000006064820152608401610640565b5092955050505050505b92915050565b61198161270f565b609880546001600160a01b0319166001600160a01b0392909216919091179055565b606654156119f35760405162461bcd60e51b815260206004820152601c60248201527f5061757361626c653a20636f6e747261637420697320706175736564000000006044820152606401610640565b6098546001600160a01b03163314611a4d5760405162461bcd60e51b815260206004820152601a60248201527f55706461746572206d757374206265207468652063616c6c65720000000000006044820152606401610640565b609754600160a01b900463ffffffff16611a6d6080840160608501614135565b63ffffffff1614611ac05760405162461bcd60e51b815260206004820152601a60248201527f636861696e526442617463684e6f6e6365206d69736d617463680000000000006044820152606401610640565b611ad06040840160208501614a21565b6002811115611ae157611ae1613f69565b609754600160c01b900460ff166002811115611aff57611aff613f69565b14611b3c5760405162461bcd60e51b815260206004820152600d60248201526c15dc9bdb99c818da185a5b9259609a1b6044820152606401610640565b609a5463ffffffff161580611b685750611b596020840184614135565b609a5463ffffffff9182169116105b611ba35760405162461bcd60e51b815260206004820152600c60248201526b5374616c652052645461736b60a01b6044820152606401610640565b609a54600160401b900463ffffffff16600003611bf45760405162461bcd60e51b815260206004820152600f60248201526e13dc081cdd185d19481d5b9a5b9a5d608a1b6044820152606401610640565b611c0460a0840160808501614135565b609a54600160401b900463ffffffff908116911614611c655760405162461bcd60e51b815260206004820152601d60248201527f7265666572656e636520626c6f636b2068617368206d69736d617463680000006044820152606401610640565b82604051602001611c769190614a3c565b60405160208183030381529060405280519060200120826020013514611cde5760405162461bcd60e51b815260206004820152601f60248201527f7265666572656e63655461736b486173682068617368206d69736d61746368006044820152606401610640565b6000611d1483604051602001611cf49190614aee565b604051602081830303815290604052805190602001208361048890614988565b609c5490915063ffffffff1660005b609b8054611d309061420b565b9050811015611df9578160ff1683602001518281518110611d5357611d536140fd565b6020026020010151611d659190614994565b6001600160601b0316606484600001518381518110611d8657611d866140fd565b60200260200101516001600160601b0316611da191906149c3565b1015611de75760405162461bcd60e51b81526020600482015260156024820152744661696c656420746f206d6565742071756f72756d60581b6044820152606401610640565b80611df1816141ca565b915050611d23565b5060408051808201825260a0860135815260c08601356020820190815260975492516223d0b560e61b815260808801356004820152825160248201529051604482015290916001600160a01b0316906308f42d4090606401600060405180830381600087803b158015611e6b57600080fd5b505af1158015611e7f573d6000803e3d6000fd5b50611e94925050506080860160608701614135565b611e9f9060016141e3565b6097805463ffffffff92909216600160a01b0263ffffffff60a01b19909216919091179055611ed16020870187614135565b609a805463ffffffff191663ffffffff929092169190911790557fec68db391879b0f9f420d1cdf3476afbdf085a2462bf4d2b11df78466295cb17611f196020880188614135565b611f296080890160608a01614135565b6040805163ffffffff93841681529290911660208301520160405180910390a1505050505050565b611f5961270f565b6001600160a01b038116611fbe5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610640565b610652816131fc565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561201a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061203e9190614031565b6001600160a01b0316336001600160a01b03161461206e5760405162461bcd60e51b81526004016106409061404e565b6066541981196066541916146120ec5760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e756e70617573653a20696e76616c696420617474656d7060448201527f7420746f2070617573652066756e6374696f6e616c69747900000000000000006064820152608401610640565b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c90602001610789565b61212b61270f565b609780546001600160a01b0319166001600160a01b0383169081179091556040519081527f2f20cf1bda67739044c5bf577353970c3dbc183b2c7274d1e8584a10269232679060200160405180910390a150565b6001600160a01b03811661220d5760405162461bcd60e51b815260206004820152604960248201527f5061757361626c652e5f73657450617573657252656769737472793a206e657760448201527f50617573657252656769737472792063616e6e6f7420626520746865207a65726064820152686f206164647265737360b81b608482015260a401610640565b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b604080518082019091526000808252602082015261229261357d565b835181526020808501519082015260408082018490526000908360608460076107d05a03fa905080806122c157fe5b50806122ff5760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b6044820152606401610640565b505092915050565b604080518082019091526000808252602082015261232361359b565b835181526020808501518183015283516040808401919091529084015160608301526000908360808460066107d05a03fa9050808061235e57fe5b50806122ff5760405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b6044820152606401610640565b6123a46135b9565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b60408051808201909152600080825260208201526000808061248c600080516020614d0e83398151915286614113565b90505b612498816133cc565b9093509150600080516020614d0e83398151915282830983036124d1576040805180820190915290815260208101919091529392505050565b600080516020614d0e83398151915260018208905061248f565b60408051808201825286815260208082018690528251808401909352868352820184905260009182919061251d6135de565b60005b60028110156126e25760006125368260066149c3565b905084826002811061254a5761254a6140fd565b6020020151518361255c836000614b81565b600c811061256c5761256c6140fd565b6020020152848260028110612583576125836140fd565b6020020151602001518382600161259a9190614b81565b600c81106125aa576125aa6140fd565b60200201528382600281106125c1576125c16140fd565b60200201515151836125d4836002614b81565b600c81106125e4576125e46140fd565b60200201528382600281106125fb576125fb6140fd565b6020020151516001602002015183612614836003614b81565b600c8110612624576126246140fd565b602002015283826002811061263b5761263b6140fd565b602002015160200151600060028110612656576126566140fd565b602002015183612667836004614b81565b600c8110612677576126776140fd565b602002015283826002811061268e5761268e6140fd565b6020020151602001516001600281106126a9576126a96140fd565b6020020151836126ba836005614b81565b600c81106126ca576126ca6140fd565b602002015250806126da816141ca565b915050612520565b506126eb6135fd565b60006020826101808560088cfa9151919c9115159b50909950505050505050505050565b6033546001600160a01b0316331461138d5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610640565b60005b6127796020830183614b99565b905081101561283e57609d60006127936020850185614b99565b848181106127a3576127a36140fd565b90506020020160208101906127b8919061367d565b60ff168152602080820192909252604001600090812080546001600160601b0319169055609f916127eb90850185614b99565b848181106127fb576127fb6140fd565b9050602002016020810190612810919061367d565b60ff168152602081019190915260400160009081208181556001015580612836816141ca565b91505061276c565b5060005b61284f6040830183614be2565b905081101561298a576128656040830183614be2565b82818110612875576128756140fd565b905060800201602001602081019061288d9190614c2b565b609d600061289e6040860186614be2565b858181106128ae576128ae6140fd565b6128c4926020608090920201908101915061367d565b60ff1681526020810191909152604090810160002080546001600160601b0319166001600160601b03939093169290921790915561290490830183614be2565b82818110612914576129146140fd565b905060800201604001609f60008480604001906129319190614be2565b85818110612941576129416140fd565b612957926020608090920201908101915061367d565b60ff1681526020808201929092526040016000208235815591013560019091015580612982816141ca565b915050612842565b5060005b61299b6060830183614c46565b9050811015612a53576129b16060830183614c46565b828181106129c1576129c16140fd565b90506040020160200160208101906129d99190614c2b565b609d60006129ea6060860186614c46565b858181106129fa576129fa6140fd565b612a10926020604090920201908101915061367d565b60ff168152602081019190915260400160002080546001600160601b0319166001600160601b039290921691909117905580612a4b816141ca565b91505061298e565b5060005b612a646080830183614c8f565b9050811015612b0057612a7a6080830183614c8f565b82818110612a8a57612a8a6140fd565b905060600201602001609f6000848060800190612aa79190614c8f565b85818110612ab757612ab76140fd565b612acd926020606090920201908101915061367d565b60ff1681526020808201929092526040016000208235815591013560019091015580612af8816141ca565b915050612a57565b5060005b612b1160a0830183614b99565b9050811015612c375760005b609b8054612b2a9061420b565b9050811015612be357609e6000612b4460a0860186614b99565b85818110612b5457612b546140fd565b9050602002013581526020019081526020016000206000609b838154612b799061420b565b8110612b8757612b876140fd565b815460011615612ba65790600052602060002090602091828204019190065b9054600160f81b911a0260f81c8152602081019190915260400160002080546001600160601b031916905580612bdb816141ca565b915050612b1d565b5060a06000612bf484830185614b99565b84818110612c0457612c046140fd565b60209081029290920135835250810191909152604001600020805460ff1916905580612c2f816141ca565b915050612b04565b5060005b612c4860c0830183614b99565b9050811015612e9857612c5e60c0830183614b99565b82818110612c6e57612c6e6140fd565b9050602002810190612c809190614cd7565b612c9190608081019060600161367d565b60a06000612ca260c0860186614b99565b85818110612cb257612cb26140fd565b9050602002810190612cc49190614cd7565b60000135815260200190815260200160002060006101000a81548160ff021916908360ff16021790555060005b612cfe60c0840184614b99565b83818110612d0e57612d0e6140fd565b9050602002810190612d209190614cd7565b612d2e906020810190614b99565b9050811015612e8557612d4460c0840184614b99565b83818110612d5457612d546140fd565b9050602002810190612d669190614cd7565b612d74906040810190614b99565b82818110612d8457612d846140fd565b9050602002016020810190612d999190614c2b565b609e6000612daa60c0870187614b99565b86818110612dba57612dba6140fd565b9050602002810190612dcc9190614cd7565b3581526020810191909152604001600090812090612ded60c0870187614b99565b86818110612dfd57612dfd6140fd565b9050602002810190612e0f9190614cd7565b612e1d906020810190614b99565b85818110612e2d57612e2d6140fd565b9050602002016020810190612e42919061367d565b60ff168152602081019190915260400160002080546001600160601b0319166001600160601b039290921691909117905580612e7d816141ca565b915050612cf1565b5080612e90816141ca565b915050612c3b565b5060005b612ea960e0830183614b99565b905081101561305c5760005b612ec260e0840184614b99565b83818110612ed257612ed26140fd565b9050602002810190612ee49190614cf7565b612ef2906020810190614b99565b905081101561304957612f0860e0840184614b99565b83818110612f1857612f186140fd565b9050602002810190612f2a9190614cf7565b612f38906040810190614b99565b82818110612f4857612f486140fd565b9050602002016020810190612f5d9190614c2b565b609e6000612f6e60e0870187614b99565b86818110612f7e57612f7e6140fd565b9050602002810190612f909190614cf7565b3581526020810191909152604001600090812090612fb160e0870187614b99565b86818110612fc157612fc16140fd565b9050602002810190612fd39190614cf7565b612fe1906020810190614b99565b85818110612ff157612ff16140fd565b9050602002016020810190613006919061367d565b60ff168152602081019190915260400160002080546001600160601b0319166001600160601b039290921691909117905580613041816141ca565b915050612eb5565b5080613054816141ca565b915050612e9c565b5060005b61306e610100830183614c46565b905081101561311257613085610100830183614c46565b82818110613095576130956140fd565b90506040020160200160208101906130ad919061367d565b60a060006130bf610100860186614c46565b858181106130cf576130cf6140fd565b90506040020160000135815260200190815260200160002060006101000a81548160ff021916908360ff160217905550808061310a906141ca565b915050613060565b5050565b6065546001600160a01b031615801561313757506001600160a01b03821615155b6131b95760405162461bcd60e51b815260206004820152604760248201527f5061757361626c652e5f696e697469616c697a655061757365723a205f696e6960448201527f7469616c697a6550617573657228292063616e206f6e6c792062652063616c6c6064820152666564206f6e636560c81b608482015260a401610640565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a26131128261217f565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b60408051808201909152600080825260208201526102008261ffff16106132aa5760405162461bcd60e51b815260206004820152601060248201526f7363616c61722d746f6f2d6c6172676560801b6044820152606401610640565b8161ffff166001036132bd575081611973565b6040805180820190915260008082526020820181905284906001905b8161ffff168661ffff161061332657600161ffff871660ff83161c81169003613309576133068484612307565b93505b6133138384612307565b92506201fffe600192831b1691016132d9565b509195945050505050565b6040805180820190915260008082526020820152815115801561335657506020820151155b15613374575050604080518082019091526000808252602082015290565b604051806040016040528083600001518152602001600080516020614d0e83398151915284602001516133a79190614113565b6133bf90600080516020614d0e8339815191526149e2565b905292915050565b919050565b60008080600080516020614d0e8339815191526003600080516020614d0e83398151915286600080516020614d0e833981519152888909090890506000613442827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f52600080516020614d0e83398151915261344e565b91959194509092505050565b6000806134596135fd565b61346161361b565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa9250828061349e57fe5b50826134ec5760405162461bcd60e51b815260206004820152601a60248201527f424e3235342e6578704d6f643a2063616c6c206661696c7572650000000000006044820152606401610640565b50519150505b9392505050565b8280546135059061420b565b90600052602060002090601f016020900481019282613527576000855561356d565b82601f106135405782800160ff1982351617855561356d565b8280016001018555821561356d579182015b8281111561356d578235825591602001919060010190613552565b50613579929150613639565b5090565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b60405180604001604052806135cc61364e565b81526020016135d961364e565b905290565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b5b80821115613579576000815560010161363a565b60405180604001604052806002906020820280368337509192915050565b803560ff811681146133c757600080fd5b60006020828403121561368f57600080fd5b6134f28261366c565b6001600160a01b038116811461065257600080fd5b6000602082840312156136bf57600080fd5b81356134f281613698565b6000602082840312156136dc57600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b038111828210171561371b5761371b6136e3565b60405290565b60405161010081016001600160401b038111828210171561371b5761371b6136e3565b604051601f8201601f191681016001600160401b038111828210171561376c5761376c6136e3565b604052919050565b60006040828403121561378657600080fd5b61378e6136f9565b9050813581526020820135602082015292915050565b600082601f8301126137b557600080fd5b604051604081018181106001600160401b03821117156137d7576137d76136e3565b80604052508060408401858111156137ee57600080fd5b845b818110156133265780358352602092830192016137f0565b60006080828403121561381a57600080fd5b6138226136f9565b905061382e83836137a4565b815261383d83604084016137a4565b602082015292915050565b600080600080610120858703121561385f57600080fd5b843593506138708660208701613774565b925061387f8660608701613808565b915061388e8660e08701613774565b905092959194509250565b600060e082840312156138ab57600080fd5b50919050565b600061012082840312156138ab57600080fd5b60008083601f8401126138d657600080fd5b5081356001600160401b038111156138ed57600080fd5b6020830191508360208260061b850101111561390857600080fd5b9250929050565b803563ffffffff811681146133c757600080fd5b600080600080600080600060a0888a03121561393e57600080fd5b87356001600160401b038082111561395557600080fd5b6139618b838c01613899565b985060208a013591508082111561397757600080fd5b6139838b838c016138b1565b975060408a013591508082111561399957600080fd5b818a0191508a601f8301126139ad57600080fd5b8135818111156139bc57600080fd5b8b60208260051b85010111156139d157600080fd5b6020830197508096505060608a01359150808211156139ef57600080fd5b506139fc8a828b016138c4565b9094509250613a0f90506080890161390f565b905092959891949750929550565b600060208083528351808285015260005b81811015613a4a57858101830151858201604001528201613a2e565b81811115613a5c576000604083870101525b50601f01601f1916929092016040019392505050565b801515811461065257600080fd5b80356133c781613a72565b8035600381106133c757600080fd5b60008060008060008060c08789031215613ab357600080fd5b8635613abe81613698565b95506020870135613ace81613698565b94506040870135613ade81613698565b93506060870135613aee81613a72565b92506080870135613afe81613698565b9150613b0c60a08801613a8b565b90509295509295509295565b600061018082840312156138ab57600080fd5b60008060008084860360c0811215613b4257600080fd5b85356001600160401b0380821115613b5957600080fd5b613b6589838a01613899565b96506060601f1984011215613b7957600080fd5b6020880195506080880135925080831115613b9357600080fd5b613b9f89848a01613b18565b945060a0880135925080831115613bb557600080fd5b5050613bc3878288016138b1565b91505092959194509250565b60008060408385031215613be257600080fd5b82359150613bf26020840161366c565b90509250929050565b60006001600160401b03821115613c1457613c146136e3565b5060051b60200190565b600082601f830112613c2f57600080fd5b81356020613c44613c3f83613bfb565b613744565b82815260059290921b84018101918181019086841115613c6357600080fd5b8286015b84811015613c8557613c788161390f565b8352918301918301613c67565b509695505050505050565b600082601f830112613ca157600080fd5b81356020613cb1613c3f83613bfb565b82815260069290921b84018101918181019086841115613cd057600080fd5b8286015b84811015613c8557613ce68882613774565b835291830191604001613cd4565b600082601f830112613d0557600080fd5b81356020613d15613c3f83613bfb565b82815260059290921b84018101918181019086841115613d3457600080fd5b8286015b84811015613c855780356001600160401b03811115613d575760008081fd5b613d658986838b0101613c1e565b845250918301918301613d38565b60006101808284031215613d8657600080fd5b613d8e613721565b905081356001600160401b0380821115613da757600080fd5b613db385838601613c1e565b83526020840135915080821115613dc957600080fd5b613dd585838601613c90565b60208401526040840135915080821115613dee57600080fd5b613dfa85838601613c90565b6040840152613e0c8560608601613808565b6060840152613e1e8560e08601613774565b6080840152610120840135915080821115613e3857600080fd5b613e4485838601613c1e565b60a0840152610140840135915080821115613e5e57600080fd5b613e6a85838601613c1e565b60c0840152610160840135915080821115613e8457600080fd5b50613e9184828501613cf4565b60e08301525092915050565b60008060408385031215613eb057600080fd5b8235915060208301356001600160401b03811115613ecd57600080fd5b613ed985828601613d73565b9150509250929050565b600081518084526020808501945080840160005b83811015613f1c5781516001600160601b031687529582019590820190600101613ef7565b509495945050505050565b602081526000825160406020840152613f436060840182613ee3565b90506020840151601f19848303016040850152613f608282613ee3565b95945050505050565b634e487b7160e01b600052602160045260246000fd5b60038110613f9d57634e487b7160e01b600052602160045260246000fd5b9052565b602081016119738284613f7f565b6000806000838503610140811215613fc657600080fd5b84356001600160401b0380821115613fdd57600080fd5b613fe988838901613899565b9550610100601f1984011215613ffe57600080fd5b60208701945061012087013592508083111561401957600080fd5b505061402786828701613b18565b9150509250925092565b60006020828403121561404357600080fd5b81516134f281613698565b6020808252602a908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526939903ab73830bab9b2b960b11b606082015260800190565b6000602082840312156140aa57600080fd5b81516134f281613a72565b60208082526028908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526739903830bab9b2b960c11b606082015260800190565b634e487b7160e01b600052603260045260246000fd5b60008261413057634e487b7160e01b600052601260045260246000fd5b500690565b60006020828403121561414757600080fd5b6134f28261390f565b6000808335601e1984360301811261416757600080fd5b8301803591506001600160401b0382111561418157600080fd5b60200191503681900382131561390857600080fd5b828152606081016134f2602083018480358252602090810135910152565b634e487b7160e01b600052601160045260246000fd5b6000600182016141dc576141dc6141b4565b5060010190565b600063ffffffff808316818516808303821115614202576142026141b4565b01949350505050565b600181811c9082168061421f57607f821691505b6020821081036138ab57634e487b7160e01b600052602260045260246000fd5b6000808335601e1984360301811261425657600080fd5b83016020810192503590506001600160401b0381111561427557600080fd5b80360383131561390857600080fd5b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b60208152600063ffffffff806142c28561390f565b166020840152806142d56020860161390f565b166040840152806142e86040860161390f565b166060840152506142fc606084018461423f565b60e0608085015261431261010085018284614284565b9150506143216080850161390f565b63ffffffff811660a08501525061433b60a085018561423f565b848303601f190160c0860152614352838284614284565b9250505061436260c0850161390f565b63ffffffff811660e0850152509392505050565b6000808335601e1984360301811261438d57600080fd5b83016020810192503590506001600160401b038111156143ac57600080fd5b8060051b360383131561390857600080fd5b8183526000602080850194508260005b85811015613f1c5760ff6143e18361366c565b16875295820195908201906001016143ce565b6000808335601e1984360301811261440b57600080fd5b83016020810192503590506001600160401b0381111561442a57600080fd5b8060071b360383131561390857600080fd5b80356001600160601b03811681146133c757600080fd5b8183526000602080850194508260005b85811015613f1c5760ff6144768361366c565b1687526001600160601b0361448c84840161443c565b1683880152604082810135908801526060808301359088015260809687019690910190600101614463565b6000808335601e198436030181126144ce57600080fd5b83016020810192503590506001600160401b038111156144ed57600080fd5b8060061b360383131561390857600080fd5b8183526000602080850194508260005b85811015613f1c5760ff6145228361366c565b1687526001600160601b0361453884840161443c565b1687840152604096870196919091019060010161450f565b6000808335601e1984360301811261456757600080fd5b83016020810192503590506001600160401b0381111561458657600080fd5b60608102360383131561390857600080fd5b8183526000602080850194508260005b85811015613f1c5760ff6145bb8361366c565b1687526145d683880184840180358252602090810135910152565b60609687019691909101906001016145a8565b81835260006001600160fb1b0383111561460257600080fd5b8260051b8083602087013760009401602001938452509192915050565b8183526000602080850194508260005b85811015613f1c576001600160601b036146488361443c565b168752958201959082019060010161462f565b818352600060208085019450848460051b8601846000805b88811015614709578484038a528235607e19893603018112614693578283fd5b88018035855260806146a788830183614376565b828a8901526146b983890182846143be565b9250505060406146cb81840184614376565b888403838a01526146dd84828461461f565b9350505050606060ff6146f182850161366c565b16960195909552509885019891850191600101614673565b509198975050505050505050565b818352600060208085019450848460051b8601846000805b88811015614709578484038a528235605e1989360301811261474f578283fd5b880180358552606061476388830183614376565b828a89015261477583890182846143be565b92505050604061478781840184614376565b93508783038289015261479b83858361461f565b9d8a019d9750505093870193505060010161472f565b8183526000602080850194508260005b85811015613f1c578135875260ff6147da84840161366c565b168784015260409687019691909101906001016147c1565b6020815261480c6020820161480684613a80565b15159052565b600061481b6020840184614376565b610120806040860152614833610140860183856143be565b925061484260408701876143f4565b9250601f198087860301606088015261485c858584614453565b945061486b60608901896144b7565b94509150808786030160808801526148848585846144ff565b94506148936080890189614550565b94509150808786030160a08801526148ac858584614598565b94506148bb60a0890189614376565b94509150808786030160c08801526148d48585846145e9565b94506148e360c0890189614376565b94509150808786030160e08801526148fc85858461465b565b945061490b60e0890189614376565b94509150610100818887030181890152614926868685614717565b9550614934818a018a6144b7565b95509250508087860301838801525061494e8484836147b1565b979650505050505050565b6060810163ffffffff61496b8461390f565b168252602083013560208301526040830135604083015292915050565b60006119733683613d73565b60006001600160601b03808316818516818304811182151516156149ba576149ba6141b4565b02949350505050565b60008160001904831182151516156149dd576149dd6141b4565b500290565b6000828210156149f4576149f46141b4565b500390565b60006001600160601b0383811690831681811015614a1957614a196141b4565b039392505050565b600060208284031215614a3357600080fd5b6134f282613a8b565b60208152600063ffffffff80614a518561390f565b166020840152614a6360208501613a8b565b614a706040850182613f7f565b5080614a7e6040860161390f565b16606084015280614a916060860161390f565b16608084015280614aa46080860161390f565b1660a0840152614ab760a085018561423f565b60e060c0860152614acd61010086018284614284565b91505081614add60c0870161390f565b1660e0850152809250505092915050565b610100810163ffffffff80614b028561390f565b16835260208401356020840152614b1b60408501613a8b565b614b286040850182613f7f565b5080614b366060860161390f565b166060840152506080830135608083015260a083013560a083015260c083013560c083015260e0830135614b6981613698565b6001600160a01b031660e09290920191909152919050565b60008219821115614b9457614b946141b4565b500190565b6000808335601e19843603018112614bb057600080fd5b8301803591506001600160401b03821115614bca57600080fd5b6020019150600581901b360382131561390857600080fd5b6000808335601e19843603018112614bf957600080fd5b8301803591506001600160401b03821115614c1357600080fd5b6020019150600781901b360382131561390857600080fd5b600060208284031215614c3d57600080fd5b6134f28261443c565b6000808335601e19843603018112614c5d57600080fd5b8301803591506001600160401b03821115614c7757600080fd5b6020019150600681901b360382131561390857600080fd5b6000808335601e19843603018112614ca657600080fd5b8301803591506001600160401b03821115614cc057600080fd5b602001915060608102360382131561390857600080fd5b60008235607e19833603018112614ced57600080fd5b9190910192915050565b60008235605e19833603018112614ced57600080fdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220d2801ac5e314d744ed197815f9ba119beb82114411ddf361bfc4c5d376b5e5af64736f6c634300080d0033",
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

// Initialize is a paid mutator transaction binding the contract method 0x30c47d8e.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _updater, bool _allowNonRootInit, address _rolldown, uint8 _chainId) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) Initialize(opts *bind.TransactOpts, _pauserRegistry common.Address, initialOwner common.Address, _updater common.Address, _allowNonRootInit bool, _rolldown common.Address, _chainId uint8) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "initialize", _pauserRegistry, initialOwner, _updater, _allowNonRootInit, _rolldown, _chainId)
}

// Initialize is a paid mutator transaction binding the contract method 0x30c47d8e.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _updater, bool _allowNonRootInit, address _rolldown, uint8 _chainId) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) Initialize(_pauserRegistry common.Address, initialOwner common.Address, _updater common.Address, _allowNonRootInit bool, _rolldown common.Address, _chainId uint8) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.Initialize(&_ContractGaspMultiRollupService.TransactOpts, _pauserRegistry, initialOwner, _updater, _allowNonRootInit, _rolldown, _chainId)
}

// Initialize is a paid mutator transaction binding the contract method 0x30c47d8e.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _updater, bool _allowNonRootInit, address _rolldown, uint8 _chainId) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) Initialize(_pauserRegistry common.Address, initialOwner common.Address, _updater common.Address, _allowNonRootInit bool, _rolldown common.Address, _chainId uint8) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.Initialize(&_ContractGaspMultiRollupService.TransactOpts, _pauserRegistry, initialOwner, _updater, _allowNonRootInit, _rolldown, _chainId)
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

// ProcessEigenOpUpdate is a paid mutator transaction binding the contract method 0x34fadbea.
//
// Solidity: function processEigenOpUpdate((uint32,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature, (bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) operatorStateInfo) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) ProcessEigenOpUpdate(opts *bind.TransactOpts, task IFinalizerTaskManagerOpTask, taskResponse IFinalizerTaskManagerOpTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature, operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "processEigenOpUpdate", task, taskResponse, nonSignerStakesAndSignature, operatorStateInfo)
}

// ProcessEigenOpUpdate is a paid mutator transaction binding the contract method 0x34fadbea.
//
// Solidity: function processEigenOpUpdate((uint32,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature, (bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) operatorStateInfo) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) ProcessEigenOpUpdate(task IFinalizerTaskManagerOpTask, taskResponse IFinalizerTaskManagerOpTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature, operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.ProcessEigenOpUpdate(&_ContractGaspMultiRollupService.TransactOpts, task, taskResponse, nonSignerStakesAndSignature, operatorStateInfo)
}

// ProcessEigenOpUpdate is a paid mutator transaction binding the contract method 0x34fadbea.
//
// Solidity: function processEigenOpUpdate((uint32,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature, (bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) operatorStateInfo) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) ProcessEigenOpUpdate(task IFinalizerTaskManagerOpTask, taskResponse IFinalizerTaskManagerOpTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature, operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.ProcessEigenOpUpdate(&_ContractGaspMultiRollupService.TransactOpts, task, taskResponse, nonSignerStakesAndSignature, operatorStateInfo)
}

// ProcessEigenRdUpdate is a paid mutator transaction binding the contract method 0xed39e502.
//
// Solidity: function processEigenRdUpdate((uint32,uint8,uint32,uint32,uint32,bytes,uint32) task, (uint32,bytes32,uint8,uint32,bytes32,uint256,uint256,address) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) ProcessEigenRdUpdate(opts *bind.TransactOpts, task IFinalizerTaskManagerRdTask, taskResponse IFinalizerTaskManagerRdTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "processEigenRdUpdate", task, taskResponse, nonSignerStakesAndSignature)
}

// ProcessEigenRdUpdate is a paid mutator transaction binding the contract method 0xed39e502.
//
// Solidity: function processEigenRdUpdate((uint32,uint8,uint32,uint32,uint32,bytes,uint32) task, (uint32,bytes32,uint8,uint32,bytes32,uint256,uint256,address) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) ProcessEigenRdUpdate(task IFinalizerTaskManagerRdTask, taskResponse IFinalizerTaskManagerRdTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.ProcessEigenRdUpdate(&_ContractGaspMultiRollupService.TransactOpts, task, taskResponse, nonSignerStakesAndSignature)
}

// ProcessEigenRdUpdate is a paid mutator transaction binding the contract method 0xed39e502.
//
// Solidity: function processEigenRdUpdate((uint32,uint8,uint32,uint32,uint32,bytes,uint32) task, (uint32,bytes32,uint8,uint32,bytes32,uint256,uint256,address) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) ProcessEigenRdUpdate(task IFinalizerTaskManagerRdTask, taskResponse IFinalizerTaskManagerRdTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.ProcessEigenRdUpdate(&_ContractGaspMultiRollupService.TransactOpts, task, taskResponse, nonSignerStakesAndSignature)
}

// ProcessEigenReinit is a paid mutator transaction binding the contract method 0x2635e74e.
//
// Solidity: function processEigenReinit((uint32,uint32,uint32,bytes,uint32,bytes,uint32) task, (bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) operatorStateInfo, bytes32[] merkleRoots, (uint256,uint256)[] ranges, uint32 lastBatchId) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) ProcessEigenReinit(opts *bind.TransactOpts, task IFinalizerTaskManagerOpTask, operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo, merkleRoots [][32]byte, ranges []IRolldownPrimitivesRange, lastBatchId uint32) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "processEigenReinit", task, operatorStateInfo, merkleRoots, ranges, lastBatchId)
}

// ProcessEigenReinit is a paid mutator transaction binding the contract method 0x2635e74e.
//
// Solidity: function processEigenReinit((uint32,uint32,uint32,bytes,uint32,bytes,uint32) task, (bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) operatorStateInfo, bytes32[] merkleRoots, (uint256,uint256)[] ranges, uint32 lastBatchId) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) ProcessEigenReinit(task IFinalizerTaskManagerOpTask, operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo, merkleRoots [][32]byte, ranges []IRolldownPrimitivesRange, lastBatchId uint32) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.ProcessEigenReinit(&_ContractGaspMultiRollupService.TransactOpts, task, operatorStateInfo, merkleRoots, ranges, lastBatchId)
}

// ProcessEigenReinit is a paid mutator transaction binding the contract method 0x2635e74e.
//
// Solidity: function processEigenReinit((uint32,uint32,uint32,bytes,uint32,bytes,uint32) task, (bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) operatorStateInfo, bytes32[] merkleRoots, (uint256,uint256)[] ranges, uint32 lastBatchId) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) ProcessEigenReinit(task IFinalizerTaskManagerOpTask, operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo, merkleRoots [][32]byte, ranges []IRolldownPrimitivesRange, lastBatchId uint32) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.ProcessEigenReinit(&_ContractGaspMultiRollupService.TransactOpts, task, operatorStateInfo, merkleRoots, ranges, lastBatchId)
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

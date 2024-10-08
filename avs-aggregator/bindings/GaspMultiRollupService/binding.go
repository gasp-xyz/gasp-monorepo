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
	Bin: "0x608060405234801561001057600080fd5b506156ff80620000216000396000f3fe608060405234801561001057600080fd5b50600436106102065760003560e01c80636f0c30a41161011a578063d03a07b2116100ad578063ed39e5021161007c578063ed39e50214610544578063f2fde38b14610557578063f84e91fc1461056a578063fabc1cbc14610573578063fdc15de81461058657600080fd5b8063d03a07b2146104f2578063deb4037d14610502578063df034cd014610519578063e2a7cb661461052c57600080fd5b8063886f1195116100e9578063886f11951461049a5780638da5cb5b146104ad5780639a8a0592146104be5780639d54f419146104df57600080fd5b80636f0c30a414610432578063715018a6146104495780637ad75561146104515780637d9788971461047a57600080fd5b806334fadbea1161019d5780634deabc211161016c5780634deabc21146103bc578063526e3e64146103e1578063595c6a67146103f55780635ac86ab7146103fd5780635c975abb1461042057600080fd5b806334fadbea146102fd5780633d9fb00c14610310578063430d3b391461033b578063499d6fb61461037057600080fd5b8063171f1d5b116101d9578063171f1d5b146102985780632635e74e146102c25780632a8414fd146102d557806330c47d8e146102ea57600080fd5b806303d097d21461020b5780630ee0fdbd1461024c57806310d67a2f14610270578063136439dd14610285575b600080fd5b610232610219366004614008565b609f602052600090815260409020805460019091015482565b604080519283526020830191909152015b60405180910390f35b60985461026090600160a81b900460ff1681565b6040519015158152602001610243565b61028361027e366004614038565b610599565b005b610283610293366004614055565b610655565b6102ab6102a63660046141d3565b610794565b604080519215158352901515602083015201610243565b6102836102d03660046142ae565b61091e565b6102dd61151a565b60405161024391906143a8565b6102836102f8366004614425565b6115a8565b61028361030b3660046144b6565b611750565b609754610323906001600160a01b031681565b6040516001600160a01b039091168152602001610243565b61035e610349366004614055565b60a06020526000908152604090205460ff1681565b60405160ff9091168152602001610243565b6103a461037e36600461455a565b609e6020908152600092835260408084209091529082529020546001600160601b031681565b6040516001600160601b039091168152602001610243565b609c546103cc9063ffffffff1681565b60405163ffffffff9091168152602001610243565b60985461026090600160a01b900460ff1681565b6102836125e5565b61026061040b366004614008565b606654600160ff9092169190911b9081161490565b6066545b604051908152602001610243565b609a546103cc90600160401b900463ffffffff1681565b6102836126ac565b6103a461045f366004614008565b609d602052600090815260409020546001600160601b031681565b61048d610488366004614828565b6126c0565b60405161024391906148b2565b606554610323906001600160a01b031681565b6033546001600160a01b0316610323565b6097546104d290600160c01b900460ff1681565b604051610243919061492c565b6102836104ed366004614038565b612caa565b609a546103cc9063ffffffff1681565b6097546103cc90600160a01b900463ffffffff1681565b609854610323906001600160a01b031681565b609a546103cc90640100000000900463ffffffff1681565b61028361055236600461493a565b612cd4565b610283610565366004614038565b61322f565b61042460995481565b610283610581366004614055565b6132a5565b610283610594366004614038565b613401565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156105ec573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061061091906149bc565b6001600160a01b0316336001600160a01b0316146106495760405162461bcd60e51b8152600401610640906149d9565b60405180910390fd5b610652816134ad565b50565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa15801561069d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106c19190614a23565b6106dd5760405162461bcd60e51b815260040161064090614a40565b606654818116146107565760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e70617573653a20696e76616c696420617474656d70742060448201527f746f20756e70617573652066756e6374696f6e616c69747900000000000000006064820152608401610640565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b60008060007f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001878760000151886020015188600001516000600281106107dc576107dc614a88565b60200201518951600160200201518a6020015160006002811061080157610801614a88565b60200201518b6020015160016002811061081d5761081d614a88565b602090810291909101518c518d83015160405161087a9a99989796959401988952602089019790975260408801959095526060870193909352608086019190915260a085015260c084015260e08301526101008201526101200190565b6040516020818303038152906040528051906020012060001c61089d9190614a9e565b90506109106108b66108af88846135a4565b869061363b565b6108be6136cf565b6109066108f7856108f1604080518082018252600080825260209182015281518083019092526001825260029082015290565b906135a4565b6109008c61378f565b9061363b565b886201d4c061381f565b909890975095505050505050565b610926613a43565b8382146109755760405162461bcd60e51b815260206004820152601d60248201527f726455706461746520696e666f206c656e677468206d69736d617463680000006044820152606401610640565b60005b6109856020880188614ac0565b9050811015610a4a57609d600061099f60208a018a614ac0565b848181106109af576109af614a88565b90506020020160208101906109c49190614008565b60ff168152602080820192909252604001600090812080546001600160601b0319169055609f916109f7908a018a614ac0565b84818110610a0757610a07614a88565b9050602002016020810190610a1c9190614008565b60ff168152602081019190915260400160009081208181556001015580610a4281614b1f565b915050610978565b5060005b610a5b6040880188614b3a565b9050811015610b9657610a716040880188614b3a565b82818110610a8157610a81614a88565b9050608002016020016020810190610a999190614b9a565b609d6000610aaa60408b018b614b3a565b85818110610aba57610aba614a88565b610ad09260206080909202019081019150614008565b60ff1681526020810191909152604090810160002080546001600160601b0319166001600160601b039390931692909217909155610b1090880188614b3a565b82818110610b2057610b20614a88565b905060800201604001609f6000898060400190610b3d9190614b3a565b85818110610b4d57610b4d614a88565b610b639260206080909202019081019150614008565b60ff1681526020808201929092526040016000208235815591013560019091015580610b8e81614b1f565b915050610a4e565b5060005b610ba76060880188614bb5565b9050811015610c5f57610bbd6060880188614bb5565b82818110610bcd57610bcd614a88565b9050604002016020016020810190610be59190614b9a565b609d6000610bf660608b018b614bb5565b85818110610c0657610c06614a88565b610c1c9260206040909202019081019150614008565b60ff168152602081019190915260400160002080546001600160601b0319166001600160601b039290921691909117905580610c5781614b1f565b915050610b9a565b5060005b610c706080880188614bfe565b9050811015610d0c57610c866080880188614bfe565b82818110610c9657610c96614a88565b905060600201602001609f6000898060800190610cb39190614bfe565b85818110610cc357610cc3614a88565b610cd99260206060909202019081019150614008565b60ff1681526020808201929092526040016000208235815591013560019091015580610d0481614b1f565b915050610c63565b5060005b610d1d60a0880188614ac0565b9050811015610e435760005b609b8054610d3690614c46565b9050811015610def57609e6000610d5060a08b018b614ac0565b85818110610d6057610d60614a88565b9050602002013581526020019081526020016000206000609b838154610d8590614c46565b8110610d9357610d93614a88565b815460011615610db25790600052602060002090602091828204019190065b9054600160f81b911a0260f81c8152602081019190915260400160002080546001600160601b031916905581610de781614b1f565b925050610d29565b5060a06000610e008983018a614ac0565b84818110610e1057610e10614a88565b60209081029290920135835250810191909152604001600020805460ff1916905580610e3b81614b1f565b915050610d10565b5060005b610e5460c0880188614ac0565b90508110156110a457610e6a60c0880188614ac0565b82818110610e7a57610e7a614a88565b9050602002810190610e8c9190614c7b565b610e9d906080810190606001614008565b60a06000610eae60c08b018b614ac0565b85818110610ebe57610ebe614a88565b9050602002810190610ed09190614c7b565b60000135815260200190815260200160002060006101000a81548160ff021916908360ff16021790555060005b610f0a60c0890189614ac0565b83818110610f1a57610f1a614a88565b9050602002810190610f2c9190614c7b565b610f3a906020810190614ac0565b905081101561109157610f5060c0890189614ac0565b83818110610f6057610f60614a88565b9050602002810190610f729190614c7b565b610f80906040810190614ac0565b82818110610f9057610f90614a88565b9050602002016020810190610fa59190614b9a565b609e6000610fb660c08c018c614ac0565b86818110610fc657610fc6614a88565b9050602002810190610fd89190614c7b565b3581526020810191909152604001600090812090610ff960c08c018c614ac0565b8681811061100957611009614a88565b905060200281019061101b9190614c7b565b611029906020810190614ac0565b8581811061103957611039614a88565b905060200201602081019061104e9190614008565b60ff168152602081019190915260400160002080546001600160601b0319166001600160601b03929092169190911790558061108981614b1f565b915050610efd565b508061109c81614b1f565b915050610e47565b5060005b6110b560e0880188614ac0565b90508110156112685760005b6110ce60e0890189614ac0565b838181106110de576110de614a88565b90506020028101906110f09190614c9b565b6110fe906020810190614ac0565b90508110156112555761111460e0890189614ac0565b8381811061112457611124614a88565b90506020028101906111369190614c9b565b611144906040810190614ac0565b8281811061115457611154614a88565b90506020020160208101906111699190614b9a565b609e600061117a60e08c018c614ac0565b8681811061118a5761118a614a88565b905060200281019061119c9190614c9b565b35815260208101919091526040016000908120906111bd60e08c018c614ac0565b868181106111cd576111cd614a88565b90506020028101906111df9190614c9b565b6111ed906020810190614ac0565b858181106111fd576111fd614a88565b90506020020160208101906112129190614008565b60ff168152602081019190915260400160002080546001600160601b0319166001600160601b03929092169190911790558061124d81614b1f565b9150506110c1565b508061126081614b1f565b9150506110a8565b5060005b61127a610100880188614bb5565b905081101561131e57611291610100880188614bb5565b828181106112a1576112a1614a88565b90506040020160200160208101906112b99190614008565b60a060006112cb6101008b018b614bb5565b858181106112db576112db614a88565b90506040020160000135815260200190815260200160002060006101000a81548160ff021916908360ff160217905550808061131690614b1f565b91505061126c565b5061132c6020880188614cb1565b609a805463ffffffff929092166401000000000267ffffffff00000000199092169190911790556113636040880160208901614cb1565b609a805463ffffffff92909216600160401b0263ffffffff60401b19909216919091179055426099556113996060880188614ccc565b6113a591609b91613e84565b506113b660a0880160808901614cb1565b609c805463ffffffff191663ffffffff9290921691909117905560005b8481101561147f576097546001600160a01b03166308f42d408787848181106113fe576113fe614a88565b9050602002013586868581811061141757611417614a88565b9050604002016040518363ffffffff1660e01b815260040161143a929190614d12565b600060405180830381600087803b15801561145457600080fd5b505af1158015611468573d6000803e3d6000fd5b50505050808061147790614b1f565b9150506113d3565b5083156114b257611491816001614d30565b609760146101000a81548163ffffffff021916908363ffffffff1602179055505b7f264965eb6bc436c6c473431d34af56e832ec344fdfd43ee6af6fce6d205e84af6114e06020890189614cb1565b6114f060408a0160208b01614cb1565b6040805163ffffffff9384168152929091166020830152015b60405180910390a150505050505050565b609b805461152790614c46565b80601f016020809104026020016040519081016040528092919081815260200182805461155390614c46565b80156115a05780601f10611575576101008083540402835291602001916115a0565b820191906000526020600020905b81548152906001019060200180831161158357829003601f168201915b505050505081565b600054610100900460ff16158080156115c85750600054600160ff909116105b806115e25750303b1580156115e2575060005460ff166001145b6116455760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610640565b6000805460ff191660011790558015611668576000805461ff0019166101001790555b611673876000613a9d565b61167c86613b87565b60988054851515600160a81b02600161ff0160a01b03199091166001600160a01b038089169190911791909117909155609780549185166001600160a01b03198316811782558492600164ff0000000160a01b03191617600160c01b8360018111156116ea576116ea6148f4565b02179055506097805463ffffffff60a01b1916600160a01b1790558015611747576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890602001611509565b50505050505050565b6098546001600160a01b031633146117aa5760405162461bcd60e51b815260206004820152601a60248201527f55706461746572206d757374206265207468652063616c6c65720000000000006044820152606401610640565b609a54600160401b900463ffffffff1615801561185c57609854600160a81b900460ff161561181a576098546001600160a01b031633146118155760405162461bcd60e51b8152602060048201526005602482015264041757468360dc1b6044820152606401610640565b61189e565b6033546001600160a01b031633146118155760405162461bcd60e51b815260206004820152600560248201526420baba341960d91b6044820152606401610640565b6098546001600160a01b0316331461189e5760405162461bcd60e51b8152602060048201526005602482015264041757468360dc1b6044820152606401610640565b846040516020016118af9190614dc6565b604051602081830303815290604052805190602001208460200135146119175760405162461bcd60e51b815260206004820152601f60248201527f7265666572656e63655461736b486173682068617368206d69736d61746368006044820152606401610640565b816040516020016119289190615302565b604051602081830303815290604052805190602001208460400135146119905760405162461bcd60e51b815260206004820152601f60248201527f6f70657261746f725374617465496e666f2068617368206d69736d61746368006044820152606401610640565b80611b25576119a56060860160408701614cb1565b609a54600160401b900463ffffffff908116911614611a065760405162461bcd60e51b815260206004820152601860248201527f7265666572656e636520626c6f636b206d69736d6174636800000000000000006044820152606401610640565b6000611a3c85604051602001611a1c9190615469565b604051602081830303815290604052805190602001208561048890615498565b609c5490915063ffffffff1660005b609b8054611a5890614c46565b9050811015611b21578160ff1683602001518281518110611a7b57611a7b614a88565b6020026020010151611a8d91906154a4565b6001600160601b0316606484600001518381518110611aae57611aae614a88565b60200260200101516001600160601b0316611ac991906154d3565b1015611b0f5760405162461bcd60e51b81526020600482015260156024820152744661696c656420746f206d6565742071756f72756d60581b6044820152606401610640565b80611b1981614b1f565b915050611a4b565b5050505b60005b611b356020840184614ac0565b9050811015611bfa57609d6000611b4f6020860186614ac0565b84818110611b5f57611b5f614a88565b9050602002016020810190611b749190614008565b60ff168152602080820192909252604001600090812080546001600160601b0319169055609f91611ba790860186614ac0565b84818110611bb757611bb7614a88565b9050602002016020810190611bcc9190614008565b60ff168152602081019190915260400160009081208181556001015580611bf281614b1f565b915050611b28565b5060005b611c0b6040840184614b3a565b9050811015611d4657611c216040840184614b3a565b82818110611c3157611c31614a88565b9050608002016020016020810190611c499190614b9a565b609d6000611c5a6040870187614b3a565b85818110611c6a57611c6a614a88565b611c809260206080909202019081019150614008565b60ff1681526020810191909152604090810160002080546001600160601b0319166001600160601b039390931692909217909155611cc090840184614b3a565b82818110611cd057611cd0614a88565b905060800201604001609f6000858060400190611ced9190614b3a565b85818110611cfd57611cfd614a88565b611d139260206080909202019081019150614008565b60ff1681526020808201929092526040016000208235815591013560019091015580611d3e81614b1f565b915050611bfe565b5060005b611d576060840184614bb5565b9050811015611e0f57611d6d6060840184614bb5565b82818110611d7d57611d7d614a88565b9050604002016020016020810190611d959190614b9a565b609d6000611da66060870187614bb5565b85818110611db657611db6614a88565b611dcc9260206040909202019081019150614008565b60ff168152602081019190915260400160002080546001600160601b0319166001600160601b039290921691909117905580611e0781614b1f565b915050611d4a565b5060005b611e206080840184614bfe565b9050811015611ebc57611e366080840184614bfe565b82818110611e4657611e46614a88565b905060600201602001609f6000858060800190611e639190614bfe565b85818110611e7357611e73614a88565b611e899260206060909202019081019150614008565b60ff1681526020808201929092526040016000208235815591013560019091015580611eb481614b1f565b915050611e13565b5060005b611ecd60a0840184614ac0565b9050811015611ff35760005b609b8054611ee690614c46565b9050811015611f9f57609e6000611f0060a0870187614ac0565b85818110611f1057611f10614a88565b9050602002013581526020019081526020016000206000609b838154611f3590614c46565b8110611f4357611f43614a88565b815460011615611f625790600052602060002090602091828204019190065b9054600160f81b911a0260f81c8152602081019190915260400160002080546001600160601b031916905581611f9781614b1f565b925050611ed9565b5060a06000611fb085830186614ac0565b84818110611fc057611fc0614a88565b60209081029290920135835250810191909152604001600020805460ff1916905580611feb81614b1f565b915050611ec0565b5060005b61200460c0840184614ac0565b90508110156122545761201a60c0840184614ac0565b8281811061202a5761202a614a88565b905060200281019061203c9190614c7b565b61204d906080810190606001614008565b60a0600061205e60c0870187614ac0565b8581811061206e5761206e614a88565b90506020028101906120809190614c7b565b60000135815260200190815260200160002060006101000a81548160ff021916908360ff16021790555060005b6120ba60c0850185614ac0565b838181106120ca576120ca614a88565b90506020028101906120dc9190614c7b565b6120ea906020810190614ac0565b90508110156122415761210060c0850185614ac0565b8381811061211057612110614a88565b90506020028101906121229190614c7b565b612130906040810190614ac0565b8281811061214057612140614a88565b90506020020160208101906121559190614b9a565b609e600061216660c0880188614ac0565b8681811061217657612176614a88565b90506020028101906121889190614c7b565b35815260208101919091526040016000908120906121a960c0880188614ac0565b868181106121b9576121b9614a88565b90506020028101906121cb9190614c7b565b6121d9906020810190614ac0565b858181106121e9576121e9614a88565b90506020020160208101906121fe9190614008565b60ff168152602081019190915260400160002080546001600160601b0319166001600160601b03929092169190911790558061223981614b1f565b9150506120ad565b508061224c81614b1f565b915050611ff7565b5060005b61226560e0840184614ac0565b90508110156124185760005b61227e60e0850185614ac0565b8381811061228e5761228e614a88565b90506020028101906122a09190614c9b565b6122ae906020810190614ac0565b9050811015612405576122c460e0850185614ac0565b838181106122d4576122d4614a88565b90506020028101906122e69190614c9b565b6122f4906040810190614ac0565b8281811061230457612304614a88565b90506020020160208101906123199190614b9a565b609e600061232a60e0880188614ac0565b8681811061233a5761233a614a88565b905060200281019061234c9190614c9b565b358152602081019190915260400160009081209061236d60e0880188614ac0565b8681811061237d5761237d614a88565b905060200281019061238f9190614c9b565b61239d906020810190614ac0565b858181106123ad576123ad614a88565b90506020020160208101906123c29190614008565b60ff168152602081019190915260400160002080546001600160601b0319166001600160601b0392909216919091179055806123fd81614b1f565b915050612271565b508061241081614b1f565b915050612258565b5060005b61242a610100840184614bb5565b90508110156124ce57612441610100840184614bb5565b8281811061245157612451614a88565b90506040020160200160208101906124699190614008565b60a0600061247b610100870187614bb5565b8581811061248b5761248b614a88565b90506040020160000135815260200190815260200160002060006101000a81548160ff021916908360ff16021790555080806124c690614b1f565b91505061241c565b506124dc6020860186614cb1565b609a805463ffffffff929092166401000000000267ffffffff00000000199092169190911790556125136040860160208701614cb1565b609a805463ffffffff92909216600160401b0263ffffffff60401b19909216919091179055426099556125496060860186614ccc565b61255591609b91613e84565b5061256660a0860160808701614cb1565b609c805463ffffffff191663ffffffff929092169190911790557f36a1fd7bd554f5c428c9829c09c6606b4c893b1fadc8735a7a12795797447ded6125ae6020870187614cb1565b6125be6040880160208901614cb1565b6040805163ffffffff93841681529290911660208301520160405180910390a15050505050565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa15801561262d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906126519190614a23565b61266d5760405162461bcd60e51b815260040161064090614a40565b600019606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b6126b4613a43565b6126be6000613b87565b565b604080518082019091526060808252602082015260408051808201909152600080825260208201819052609b80546126f790614c46565b90509050612718604051806040016040528060608152602001606081525090565b816001600160401b038111156127305761273061406e565b604051908082528060200260200182016040528015612759578160200160208202803683370190505b506020820152816001600160401b038111156127775761277761406e565b6040519080825280602002602001820160405280156127a0578160200160208202803683370190505b5081526020850151516000906001600160401b038111156127c3576127c361406e565b6040519080825280602002602001820160405280156127ec578160200160208202803683370190505b5090506000805b87602001515181101561299e576128388860200151828151811061281957612819614a88565b6020026020010151805160009081526020918201519091526040902090565b83828151811061284a5761284a614a88565b6020908102919091010152801561291457826128676001836154f2565b8151811061287757612877614a88565b602002602001015160001c83828151811061289457612894614a88565b602002602001015160001c11612914576040805162461bcd60e51b81526020600482015260248101919091527f424c535369676e6174757265436865636b65722e636865636b5369676e61747560448201527f7265733a206e6f6e5369676e65725075626b657973206e6f7420736f727465646064820152608401610640565b61298a61298360a0600086858151811061293057612930614a88565b6020026020010151815260200190815260200160002060009054906101000a900460ff1660ff168a60200151848151811061296d5761296d614a88565b6020026020010151613bd990919063ffffffff16565b879061363b565b95508061299681614b1f565b9150506127f3565b506129a885613cbd565b945060005b84811015612b8c57609b8181546129c390614c46565b81106129d1576129d1614a88565b8154600116156129f05790600052602060002090602091828204019190065b9054600160f81b911a0260f81c6000818152609f60209081526040918290208251808401909352805483526001015490820152909250612a3190879061363b565b60ff83166000908152609d60209081526040909120549086015180519298506001600160601b039091169183908110612a6c57612a6c614a88565b6001600160601b03909216602092830291909101820152840151805182908110612a9857612a98614a88565b602002602001015184600001518281518110612ab657612ab6614a88565b60200260200101906001600160601b031690816001600160601b03168152505060005b886020015151811015612b7957609e6000858381518110612afc57612afc614a88565b6020908102919091018101518252818101929092526040908101600090812060ff87168252909252902054855180516001600160601b039092169184908110612b4757612b47614a88565b60200260200101818151612b5b9190615509565b6001600160601b031690525080612b7181614b1f565b915050612ad9565b5080612b8481614b1f565b9150506129ad565b50600080612ba48a888b606001518c60800151610794565b9150915081612c275760405162461bcd60e51b815260206004820152604360248201527f424c535369676e6174757265436865636b65722e636865636b5369676e61747560448201527f7265733a2070616972696e6720707265636f6d70696c652063616c6c206661696064820152621b195960ea1b608482015260a401610640565b80612c9a5760405162461bcd60e51b815260206004820152603960248201527f424c535369676e6174757265436865636b65722e636865636b5369676e61747560448201527f7265733a207369676e617475726520697320696e76616c6964000000000000006064820152608401610640565b5092955050505050505b92915050565b612cb2613a43565b609880546001600160a01b0319166001600160a01b0392909216919091179055565b6098546001600160a01b03163314612d2e5760405162461bcd60e51b815260206004820152601a60248201527f55706461746572206d757374206265207468652063616c6c65720000000000006044820152606401610640565b609754600160a01b900463ffffffff16612d4e6080840160608501614cb1565b63ffffffff1614612da15760405162461bcd60e51b815260206004820152601a60248201527f636861696e526442617463684e6f6e6365206d69736d617463680000000000006044820152606401610640565b612db16040840160208501615531565b6001811115612dc257612dc26148f4565b609754600160c01b900460ff166001811115612de057612de06148f4565b14612e1d5760405162461bcd60e51b815260206004820152600d60248201526c15dc9bdb99c818da185a5b9259609a1b6044820152606401610640565b609a5463ffffffff161580612e495750612e3a6020840184614cb1565b609a5463ffffffff9182169116105b612e845760405162461bcd60e51b815260206004820152600c60248201526b5374616c652052645461736b60a01b6044820152606401610640565b609a54600160401b900463ffffffff16612ed25760405162461bcd60e51b815260206004820152600f60248201526e13dc081cdd185d19481d5b9a5b9a5d608a1b6044820152606401610640565b612ee260a0840160808501614cb1565b609a54600160401b900463ffffffff908116911614612f435760405162461bcd60e51b815260206004820152601d60248201527f7265666572656e636520626c6f636b2068617368206d69736d617463680000006044820152606401610640565b82604051602001612f54919061554c565b60405160208183030381529060405280519060200120826020013514612fbc5760405162461bcd60e51b815260206004820152601f60248201527f7265666572656e63655461736b486173682068617368206d69736d61746368006044820152606401610640565b6000612ff283604051602001612fd291906155fe565b604051602081830303815290604052805190602001208361048890615498565b609c5490915063ffffffff1660005b609b805461300e90614c46565b90508110156130d7578160ff168360200151828151811061303157613031614a88565b602002602001015161304391906154a4565b6001600160601b031660648460000151838151811061306457613064614a88565b60200260200101516001600160601b031661307f91906154d3565b10156130c55760405162461bcd60e51b81526020600482015260156024820152744661696c656420746f206d6565742071756f72756d60581b6044820152606401610640565b806130cf81614b1f565b915050613001565b5060408051808201825260a0860135815260c08601356020820190815260975492516223d0b560e61b815260808801356004820152825160248201529051604482015290916001600160a01b0316906308f42d4090606401600060405180830381600087803b15801561314957600080fd5b505af115801561315d573d6000803e3d6000fd5b50613172925050506080860160608701614cb1565b61317d906001614d30565b6097805463ffffffff92909216600160a01b0263ffffffff60a01b199092169190911790556131af6020870187614cb1565b609a805463ffffffff191663ffffffff929092169190911790557fec68db391879b0f9f420d1cdf3476afbdf085a2462bf4d2b11df78466295cb176131f76020880188614cb1565b6132076080890160608a01614cb1565b6040805163ffffffff93841681529290911660208301520160405180910390a1505050505050565b613237613a43565b6001600160a01b03811661329c5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610640565b61065281613b87565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156132f8573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061331c91906149bc565b6001600160a01b0316336001600160a01b03161461334c5760405162461bcd60e51b8152600401610640906149d9565b6066541981196066541916146133ca5760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e756e70617573653a20696e76616c696420617474656d7060448201527f7420746f2070617573652066756e6374696f6e616c69747900000000000000006064820152608401610640565b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c90602001610789565b606654156134515760405162461bcd60e51b815260206004820152601c60248201527f5061757361626c653a20636f6e747261637420697320706175736564000000006044820152606401610640565b613459613a43565b609780546001600160a01b0319166001600160a01b0383169081179091556040519081527f2f20cf1bda67739044c5bf577353970c3dbc183b2c7274d1e8584a10269232679060200160405180910390a150565b6001600160a01b03811661353b5760405162461bcd60e51b815260206004820152604960248201527f5061757361626c652e5f73657450617573657252656769737472793a206e657760448201527f50617573657252656769737472792063616e6e6f7420626520746865207a65726064820152686f206164647265737360b81b608482015260a401610640565b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b60408051808201909152600080825260208201526135c0613f08565b835181526020808501519082015260408082018490526000908360608460076107d05a03fa90508080156135f3576135f5565bfe5b50806136335760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b6044820152606401610640565b505092915050565b6040805180820190915260008082526020820152613657613f26565b835181526020808501518183015283516040808401919091529084015160608301526000908360808460066107d05a03fa90508080156135f35750806136335760405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b6044820152606401610640565b6136d7613f44565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b6040805180820190915260008082526020820152600080806137bf6000805160206156aa83398151915286614a9e565b90505b6137cb81613d58565b90935091506000805160206156aa833981519152828309831415613805576040805180820190915290815260208101919091529392505050565b6000805160206156aa8339815191526001820890506137c2565b604080518082018252868152602080820186905282518084019093528683528201849052600091829190613851613f69565b60005b6002811015613a1657600061386a8260066154d3565b905084826002811061387e5761387e614a88565b60200201515183613890836000615691565b600c81106138a0576138a0614a88565b60200201528482600281106138b7576138b7614a88565b602002015160200151838260016138ce9190615691565b600c81106138de576138de614a88565b60200201528382600281106138f5576138f5614a88565b6020020151515183613908836002615691565b600c811061391857613918614a88565b602002015283826002811061392f5761392f614a88565b6020020151516001602002015183613948836003615691565b600c811061395857613958614a88565b602002015283826002811061396f5761396f614a88565b60200201516020015160006002811061398a5761398a614a88565b60200201518361399b836004615691565b600c81106139ab576139ab614a88565b60200201528382600281106139c2576139c2614a88565b6020020151602001516001600281106139dd576139dd614a88565b6020020151836139ee836005615691565b600c81106139fe576139fe614a88565b60200201525080613a0e81614b1f565b915050613854565b50613a1f613f88565b60006020826101808560088cfa9151919c9115159b50909950505050505050505050565b6033546001600160a01b031633146126be5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610640565b6065546001600160a01b0316158015613abe57506001600160a01b03821615155b613b405760405162461bcd60e51b815260206004820152604760248201527f5061757361626c652e5f696e697469616c697a655061757365723a205f696e6960448201527f7469616c697a6550617573657228292063616e206f6e6c792062652063616c6c6064820152666564206f6e636560c81b608482015260a401610640565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2613b83826134ad565b5050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b60408051808201909152600080825260208201526102008261ffff1610613c355760405162461bcd60e51b815260206004820152601060248201526f7363616c61722d746f6f2d6c6172676560801b6044820152606401610640565b8161ffff1660011415613c49575081612ca4565b6040805180820190915260008082526020820181905284906001905b8161ffff168661ffff1610613cb257600161ffff871660ff83161c81161415613c9557613c92848461363b565b93505b613c9f838461363b565b92506201fffe600192831b169101613c65565b509195945050505050565b60408051808201909152600080825260208201528151158015613ce257506020820151155b15613d00575050604080518082019091526000808252602082015290565b6040518060400160405280836000015181526020016000805160206156aa8339815191528460200151613d339190614a9e565b613d4b906000805160206156aa8339815191526154f2565b905292915050565b919050565b600080806000805160206156aa83398151915260036000805160206156aa833981519152866000805160206156aa833981519152888909090890506000613dce827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f526000805160206156aa833981519152613dda565b91959194509092505050565b600080613de5613f88565b613ded613fa6565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa92508280156135f3575082613e775760405162461bcd60e51b815260206004820152601a60248201527f424e3235342e6578704d6f643a2063616c6c206661696c7572650000000000006044820152606401610640565b50519150505b9392505050565b828054613e9090614c46565b90600052602060002090601f016020900481019282613eb25760008555613ef8565b82601f10613ecb5782800160ff19823516178555613ef8565b82800160010185558215613ef8579182015b82811115613ef8578235825591602001919060010190613edd565b50613f04929150613fc4565b5090565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b6040518060400160405280613f57613fd9565b8152602001613f64613fd9565b905290565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b5b80821115613f045760008155600101613fc5565b60405180604001604052806002906020820280368337509192915050565b803560ff81168114613d5357600080fd5b60006020828403121561401a57600080fd5b613e7d82613ff7565b6001600160a01b038116811461065257600080fd5b60006020828403121561404a57600080fd5b8135613e7d81614023565b60006020828403121561406757600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b03811182821017156140a6576140a661406e565b60405290565b60405161010081016001600160401b03811182821017156140a6576140a661406e565b604051601f8201601f191681016001600160401b03811182821017156140f7576140f761406e565b604052919050565b60006040828403121561411157600080fd5b614119614084565b9050813581526020820135602082015292915050565b600082601f83011261414057600080fd5b604051604081018181106001600160401b03821117156141625761416261406e565b806040525080604084018581111561417957600080fd5b845b81811015613cb257803583526020928301920161417b565b6000608082840312156141a557600080fd5b6141ad614084565b90506141b9838361412f565b81526141c8836040840161412f565b602082015292915050565b60008060008061012085870312156141ea57600080fd5b843593506141fb86602087016140ff565b925061420a8660608701614193565b91506142198660e087016140ff565b905092959194509250565b600060e0828403121561423657600080fd5b50919050565b6000610120828403121561423657600080fd5b60008083601f84011261426157600080fd5b5081356001600160401b0381111561427857600080fd5b6020830191508360208260061b850101111561429357600080fd5b9250929050565b803563ffffffff81168114613d5357600080fd5b600080600080600080600060a0888a0312156142c957600080fd5b87356001600160401b03808211156142e057600080fd5b6142ec8b838c01614224565b985060208a013591508082111561430257600080fd5b61430e8b838c0161423c565b975060408a013591508082111561432457600080fd5b818a0191508a601f83011261433857600080fd5b81358181111561434757600080fd5b8b60208260051b850101111561435c57600080fd5b6020830197508096505060608a013591508082111561437a57600080fd5b506143878a828b0161424f565b909450925061439a90506080890161429a565b905092959891949750929550565b600060208083528351808285015260005b818110156143d5578581018301518582016040015282016143b9565b818111156143e7576000604083870101525b50601f01601f1916929092016040019392505050565b801515811461065257600080fd5b8035613d53816143fd565b803560028110613d5357600080fd5b60008060008060008060c0878903121561443e57600080fd5b863561444981614023565b9550602087013561445981614023565b9450604087013561446981614023565b93506060870135614479816143fd565b9250608087013561448981614023565b915061449760a08801614416565b90509295509295509295565b6000610180828403121561423657600080fd5b60008060008084860360c08112156144cd57600080fd5b85356001600160401b03808211156144e457600080fd5b6144f089838a01614224565b96506060601f198401121561450457600080fd5b602088019550608088013592508083111561451e57600080fd5b61452a89848a016144a3565b945060a088013592508083111561454057600080fd5b505061454e8782880161423c565b91505092959194509250565b6000806040838503121561456d57600080fd5b8235915061457d60208401613ff7565b90509250929050565b60006001600160401b0382111561459f5761459f61406e565b5060051b60200190565b600082601f8301126145ba57600080fd5b813560206145cf6145ca83614586565b6140cf565b82815260059290921b840181019181810190868411156145ee57600080fd5b8286015b84811015614610576146038161429a565b83529183019183016145f2565b509695505050505050565b600082601f83011261462c57600080fd5b8135602061463c6145ca83614586565b82815260069290921b8401810191818101908684111561465b57600080fd5b8286015b848110156146105761467188826140ff565b83529183019160400161465f565b600082601f83011261469057600080fd5b813560206146a06145ca83614586565b82815260059290921b840181019181810190868411156146bf57600080fd5b8286015b848110156146105780356001600160401b038111156146e25760008081fd5b6146f08986838b01016145a9565b8452509183019183016146c3565b6000610180828403121561471157600080fd5b6147196140ac565b905081356001600160401b038082111561473257600080fd5b61473e858386016145a9565b8352602084013591508082111561475457600080fd5b6147608583860161461b565b6020840152604084013591508082111561477957600080fd5b6147858583860161461b565b60408401526147978560608601614193565b60608401526147a98560e086016140ff565b60808401526101208401359150808211156147c357600080fd5b6147cf858386016145a9565b60a08401526101408401359150808211156147e957600080fd5b6147f5858386016145a9565b60c084015261016084013591508082111561480f57600080fd5b5061481c8482850161467f565b60e08301525092915050565b6000806040838503121561483b57600080fd5b8235915060208301356001600160401b0381111561485857600080fd5b614864858286016146fe565b9150509250929050565b600081518084526020808501945080840160005b838110156148a75781516001600160601b031687529582019590820190600101614882565b509495945050505050565b6020815260008251604060208401526148ce606084018261486e565b90506020840151601f198483030160408501526148eb828261486e565b95945050505050565b634e487b7160e01b600052602160045260246000fd5b6002811061492857634e487b7160e01b600052602160045260246000fd5b9052565b60208101612ca4828461490a565b600080600083850361014081121561495157600080fd5b84356001600160401b038082111561496857600080fd5b61497488838901614224565b9550610100601f198401121561498957600080fd5b6020870194506101208701359250808311156149a457600080fd5b50506149b2868287016144a3565b9150509250925092565b6000602082840312156149ce57600080fd5b8151613e7d81614023565b6020808252602a908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526939903ab73830bab9b2b960b11b606082015260800190565b600060208284031215614a3557600080fd5b8151613e7d816143fd565b60208082526028908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526739903830bab9b2b960c11b606082015260800190565b634e487b7160e01b600052603260045260246000fd5b600082614abb57634e487b7160e01b600052601260045260246000fd5b500690565b6000808335601e19843603018112614ad757600080fd5b8301803591506001600160401b03821115614af157600080fd5b6020019150600581901b360382131561429357600080fd5b634e487b7160e01b600052601160045260246000fd5b6000600019821415614b3357614b33614b09565b5060010190565b6000808335601e19843603018112614b5157600080fd5b8301803591506001600160401b03821115614b6b57600080fd5b6020019150600781901b360382131561429357600080fd5b80356001600160601b0381168114613d5357600080fd5b600060208284031215614bac57600080fd5b613e7d82614b83565b6000808335601e19843603018112614bcc57600080fd5b8301803591506001600160401b03821115614be657600080fd5b6020019150600681901b360382131561429357600080fd5b6000808335601e19843603018112614c1557600080fd5b8301803591506001600160401b03821115614c2f57600080fd5b602001915060608102360382131561429357600080fd5b600181811c90821680614c5a57607f821691505b6020821081141561423657634e487b7160e01b600052602260045260246000fd5b60008235607e19833603018112614c9157600080fd5b9190910192915050565b60008235605e19833603018112614c9157600080fd5b600060208284031215614cc357600080fd5b613e7d8261429a565b6000808335601e19843603018112614ce357600080fd5b8301803591506001600160401b03821115614cfd57600080fd5b60200191503681900382131561429357600080fd5b82815260608101613e7d602083018480358252602090810135910152565b600063ffffffff808316818516808303821115614d4f57614d4f614b09565b01949350505050565b6000808335601e19843603018112614d6f57600080fd5b83016020810192503590506001600160401b03811115614d8e57600080fd5b80360383131561429357600080fd5b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b60208152600063ffffffff80614ddb8561429a565b16602084015280614dee6020860161429a565b16604084015280614e016040860161429a565b16606084015250614e156060840184614d58565b60e06080850152614e2b61010085018284614d9d565b915050614e3a6080850161429a565b63ffffffff811660a085015250614e5460a0850185614d58565b848303601f190160c0860152614e6b838284614d9d565b92505050614e7b60c0850161429a565b63ffffffff811660e0850152509392505050565b6000808335601e19843603018112614ea657600080fd5b83016020810192503590506001600160401b03811115614ec557600080fd5b8060051b360383131561429357600080fd5b8183526000602080850194508260005b858110156148a75760ff614efa83613ff7565b1687529582019590820190600101614ee7565b6000808335601e19843603018112614f2457600080fd5b83016020810192503590506001600160401b03811115614f4357600080fd5b8060071b360383131561429357600080fd5b8183526000602080850194508260005b858110156148a75760ff614f7883613ff7565b1687526001600160601b03614f8e848401614b83565b1683880152604082810135908801526060808301359088015260809687019690910190600101614f65565b6000808335601e19843603018112614fd057600080fd5b83016020810192503590506001600160401b03811115614fef57600080fd5b8060061b360383131561429357600080fd5b8183526000602080850194508260005b858110156148a75760ff61502483613ff7565b1687526001600160601b0361503a848401614b83565b16878401526040968701969190910190600101615011565b6000808335601e1984360301811261506957600080fd5b83016020810192503590506001600160401b0381111561508857600080fd5b60608102360383131561429357600080fd5b8183526000602080850194508260005b858110156148a75760ff6150bd83613ff7565b1687526150d883880184840180358252602090810135910152565b60609687019691909101906001016150aa565b81835260006001600160fb1b0383111561510457600080fd5b8260051b8083602087013760009401602001938452509192915050565b8183526000602080850194508260005b858110156148a7576001600160601b0361514a83614b83565b1687529582019590820190600101615131565b60008383855260208086019550808560051b830101846000805b8881101561521257858403601f19018a52823536899003607e1901811261519c578283fd5b88018035855260806151b087830183614e8f565b82898901526151c28389018284614ed7565b9250505060406151d481840184614e8f565b888403838a01526151e6848284615121565b9350505050606060ff6151fa828501613ff7565b16960195909552509884019891840191600101615177565b509198975050505050505050565b60008383855260208086019550808560051b830101846000805b8881101561521257858403601f19018a52823536899003605e1901811261525f578283fd5b880180358552606061527387830183614e8f565b82898901526152858389018284614ed7565b92505050604061529781840184614e8f565b9350878303828901526152ab838583615121565b9d89019d9750505093860193505060010161523a565b8183526000602080850194508260005b858110156148a7578135875260ff6152ea848401613ff7565b168784015260409687019691909101906001016152d1565b6020815261531c602082016153168461440b565b15159052565b600061532b6020840184614e8f565b61012080604086015261534361014086018385614ed7565b92506153526040870187614f0d565b9250601f198087860301606088015261536c858584614f55565b945061537b6060890189614fb9565b9450915080878603016080880152615394858584615001565b94506153a36080890189615052565b94509150808786030160a08801526153bc85858461509a565b94506153cb60a0890189614e8f565b94509150808786030160c08801526153e48585846150eb565b94506153f360c0890189614e8f565b94509150808786030160e088015261540c85858461515d565b945061541b60e0890189614e8f565b94509150610100818887030181890152615436868685615220565b9550615444818a018a614fb9565b95509250508087860301838801525061545e8484836152c1565b979650505050505050565b6060810163ffffffff61547b8461429a565b168252602083013560208301526040830135604083015292915050565b6000612ca436836146fe565b60006001600160601b03808316818516818304811182151516156154ca576154ca614b09565b02949350505050565b60008160001904831182151516156154ed576154ed614b09565b500290565b60008282101561550457615504614b09565b500390565b60006001600160601b038381169083168181101561552957615529614b09565b039392505050565b60006020828403121561554357600080fd5b613e7d82614416565b60208152600063ffffffff806155618561429a565b16602084015261557360208501614416565b615580604085018261490a565b508061558e6040860161429a565b166060840152806155a16060860161429a565b166080840152806155b46080860161429a565b1660a08401526155c760a0850185614d58565b60e060c08601526155dd61010086018284614d9d565b915050816155ed60c0870161429a565b1660e0850152809250505092915050565b610100810163ffffffff806156128561429a565b1683526020840135602084015261562b60408501614416565b615638604085018261490a565b50806156466060860161429a565b166060840152506080830135608083015260a083013560a083015260c083013560c083015260e083013561567981614023565b6001600160a01b031660e09290920191909152919050565b600082198211156156a4576156a4614b09565b50019056fe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220c877901c8cd8c6c4eaff4ba1ae9afe64767e93be23e26127977355cb242fc20764736f6c634300080c0033",
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

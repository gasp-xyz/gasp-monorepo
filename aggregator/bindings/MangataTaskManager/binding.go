// Code generated - DO NOT EDIT.
// This file is a generated binding and any manual changes will be lost.

package contractMangataTaskManager

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

// BLSOperatorStateRetrieverCheckSignaturesIndices is an auto generated low-level Go binding around an user-defined struct.
type BLSOperatorStateRetrieverCheckSignaturesIndices struct {
	NonSignerQuorumBitmapIndices []uint32
	QuorumApkIndices             []uint32
	TotalStakeIndices            []uint32
	NonSignerStakeIndices        [][]uint32
}

// BLSOperatorStateRetrieverOperator is an auto generated low-level Go binding around an user-defined struct.
type BLSOperatorStateRetrieverOperator struct {
	OperatorId [32]byte
	Stake      *big.Int
}

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

// IMangataTaskManagerTask is an auto generated low-level Go binding around an user-defined struct.
type IMangataTaskManagerTask struct {
	BlockNumber               *big.Int
	TaskCreatedBlock          uint32
	QuorumNumbers             []byte
	QuorumThresholdPercentage uint32
}

// IMangataTaskManagerTaskResponse is an auto generated low-level Go binding around an user-defined struct.
type IMangataTaskManagerTaskResponse struct {
	ReferenceTaskIndex uint32
	BlockHash          [32]byte
}

// IMangataTaskManagerTaskResponseMetadata is an auto generated low-level Go binding around an user-defined struct.
type IMangataTaskManagerTaskResponseMetadata struct {
	TaskResponsedBlock uint32
	HashOfNonSigners   [32]byte
}

// ContractMangataTaskManagerMetaData contains all meta data concerning the ContractMangataTaskManager contract.
var ContractMangataTaskManagerMetaData = &bind.MetaData{
	ABI: "[{\"inputs\":[{\"internalType\":\"contractIBLSRegistryCoordinatorWithIndices\",\"name\":\"_registryCoordinator\",\"type\":\"address\"},{\"internalType\":\"uint32\",\"name\":\"_taskResponseWindowBlock\",\"type\":\"uint32\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\"}],\"name\":\"Initialized\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint32\",\"name\":\"taskIndex\",\"type\":\"uint32\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\"},{\"internalType\":\"uint32\",\"name\":\"taskCreatedBlock\",\"type\":\"uint32\"},{\"internalType\":\"bytes\",\"name\":\"quorumNumbers\",\"type\":\"bytes\"},{\"internalType\":\"uint32\",\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\"}],\"indexed\":false,\"internalType\":\"structIMangataTaskManager.Task\",\"name\":\"task\",\"type\":\"tuple\"}],\"name\":\"NewTaskCreated\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"OwnershipTransferred\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"newPausedStatus\",\"type\":\"uint256\"}],\"name\":\"Paused\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"contractIPauserRegistry\",\"name\":\"pauserRegistry\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"contractIPauserRegistry\",\"name\":\"newPauserRegistry\",\"type\":\"address\"}],\"name\":\"PauserRegistrySet\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint32\",\"name\":\"taskIndex\",\"type\":\"uint32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"challenger\",\"type\":\"address\"}],\"name\":\"TaskChallengedSuccessfully\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint32\",\"name\":\"taskIndex\",\"type\":\"uint32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"challenger\",\"type\":\"address\"}],\"name\":\"TaskChallengedUnsuccessfully\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint32\",\"name\":\"taskIndex\",\"type\":\"uint32\"}],\"name\":\"TaskCompleted\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"components\":[{\"internalType\":\"uint32\",\"name\":\"referenceTaskIndex\",\"type\":\"uint32\"},{\"internalType\":\"bytes32\",\"name\":\"blockHash\",\"type\":\"bytes32\"}],\"indexed\":false,\"internalType\":\"structIMangataTaskManager.TaskResponse\",\"name\":\"taskResponse\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"uint32\",\"name\":\"taskResponsedBlock\",\"type\":\"uint32\"},{\"internalType\":\"bytes32\",\"name\":\"hashOfNonSigners\",\"type\":\"bytes32\"}],\"indexed\":false,\"internalType\":\"structIMangataTaskManager.TaskResponseMetadata\",\"name\":\"taskResponseMetadata\",\"type\":\"tuple\"}],\"name\":\"TaskResponded\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"newPausedStatus\",\"type\":\"uint256\"}],\"name\":\"Unpaused\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"TASK_CHALLENGE_WINDOW_BLOCK\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"TASK_RESPONSE_WINDOW_BLOCK\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"aggregator\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"name\":\"allTaskHashes\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"name\":\"allTaskResponses\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"blsPubkeyRegistry\",\"outputs\":[{\"internalType\":\"contractIBLSPubkeyRegistry\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"msgHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"quorumNumbers\",\"type\":\"bytes\"},{\"internalType\":\"uint32\",\"name\":\"referenceBlockNumber\",\"type\":\"uint32\"},{\"components\":[{\"internalType\":\"uint32[]\",\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\"}],\"internalType\":\"structBN254.G1Point[]\",\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\"}],\"internalType\":\"structBN254.G1Point[]\",\"name\":\"quorumApks\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"uint256[2]\",\"name\":\"X\",\"type\":\"uint256[2]\"},{\"internalType\":\"uint256[2]\",\"name\":\"Y\",\"type\":\"uint256[2]\"}],\"internalType\":\"structBN254.G2Point\",\"name\":\"apkG2\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\"}],\"internalType\":\"structBN254.G1Point\",\"name\":\"sigma\",\"type\":\"tuple\"},{\"internalType\":\"uint32[]\",\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\"},{\"internalType\":\"uint32[]\",\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\"},{\"internalType\":\"uint32[][]\",\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\"}],\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\"}],\"name\":\"checkSignatures\",\"outputs\":[{\"components\":[{\"internalType\":\"uint96[]\",\"name\":\"signedStakeForQuorum\",\"type\":\"uint96[]\"},{\"internalType\":\"uint96[]\",\"name\":\"totalStakeForQuorum\",\"type\":\"uint96[]\"}],\"internalType\":\"structIBLSSignatureChecker.QuorumStakeTotals\",\"name\":\"\",\"type\":\"tuple\"},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\"},{\"internalType\":\"uint32\",\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\"},{\"internalType\":\"bytes\",\"name\":\"quorumNumbers\",\"type\":\"bytes\"}],\"name\":\"createNewTask\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"generator\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"contractIBLSRegistryCoordinatorWithIndices\",\"name\":\"registryCoordinator\",\"type\":\"address\"},{\"internalType\":\"uint32\",\"name\":\"referenceBlockNumber\",\"type\":\"uint32\"},{\"internalType\":\"bytes\",\"name\":\"quorumNumbers\",\"type\":\"bytes\"},{\"internalType\":\"bytes32[]\",\"name\":\"nonSignerOperatorIds\",\"type\":\"bytes32[]\"}],\"name\":\"getCheckSignaturesIndices\",\"outputs\":[{\"components\":[{\"internalType\":\"uint32[]\",\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\"},{\"internalType\":\"uint32[]\",\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\"},{\"internalType\":\"uint32[]\",\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\"},{\"internalType\":\"uint32[][]\",\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\"}],\"internalType\":\"structBLSOperatorStateRetriever.CheckSignaturesIndices\",\"name\":\"\",\"type\":\"tuple\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"contractIBLSRegistryCoordinatorWithIndices\",\"name\":\"registryCoordinator\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"quorumNumbers\",\"type\":\"bytes\"},{\"internalType\":\"uint32\",\"name\":\"blockNumber\",\"type\":\"uint32\"}],\"name\":\"getOperatorState\",\"outputs\":[{\"components\":[{\"internalType\":\"bytes32\",\"name\":\"operatorId\",\"type\":\"bytes32\"},{\"internalType\":\"uint96\",\"name\":\"stake\",\"type\":\"uint96\"}],\"internalType\":\"structBLSOperatorStateRetriever.Operator[][]\",\"name\":\"\",\"type\":\"tuple[][]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"contractIBLSRegistryCoordinatorWithIndices\",\"name\":\"registryCoordinator\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"operatorId\",\"type\":\"bytes32\"},{\"internalType\":\"uint32\",\"name\":\"blockNumber\",\"type\":\"uint32\"}],\"name\":\"getOperatorState\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"components\":[{\"internalType\":\"bytes32\",\"name\":\"operatorId\",\"type\":\"bytes32\"},{\"internalType\":\"uint96\",\"name\":\"stake\",\"type\":\"uint96\"}],\"internalType\":\"structBLSOperatorStateRetriever.Operator[][]\",\"name\":\"\",\"type\":\"tuple[][]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getTaskChallangeWindowBlock\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getTaskResponseWindowBlock\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"contractIPauserRegistry\",\"name\":\"_pauserRegistry\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"initialOwner\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_aggregator\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_generator\",\"type\":\"address\"}],\"name\":\"initialize\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"latestTaskNum\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newPausedStatus\",\"type\":\"uint256\"}],\"name\":\"pause\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"pauseAll\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"index\",\"type\":\"uint8\"}],\"name\":\"paused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"paused\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"pauserRegistry\",\"outputs\":[{\"internalType\":\"contractIPauserRegistry\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\"},{\"internalType\":\"uint32\",\"name\":\"taskCreatedBlock\",\"type\":\"uint32\"},{\"internalType\":\"bytes\",\"name\":\"quorumNumbers\",\"type\":\"bytes\"},{\"internalType\":\"uint32\",\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\"}],\"internalType\":\"structIMangataTaskManager.Task\",\"name\":\"task\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"uint32\",\"name\":\"referenceTaskIndex\",\"type\":\"uint32\"},{\"internalType\":\"bytes32\",\"name\":\"blockHash\",\"type\":\"bytes32\"}],\"internalType\":\"structIMangataTaskManager.TaskResponse\",\"name\":\"taskResponse\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"uint32\",\"name\":\"taskResponsedBlock\",\"type\":\"uint32\"},{\"internalType\":\"bytes32\",\"name\":\"hashOfNonSigners\",\"type\":\"bytes32\"}],\"internalType\":\"structIMangataTaskManager.TaskResponseMetadata\",\"name\":\"taskResponseMetadata\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\"}],\"internalType\":\"structBN254.G1Point[]\",\"name\":\"pubkeysOfNonSigningOperators\",\"type\":\"tuple[]\"}],\"name\":\"raiseAndResolveChallenge\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"registryCoordinator\",\"outputs\":[{\"internalType\":\"contractIRegistryCoordinator\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"renounceOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\"},{\"internalType\":\"uint32\",\"name\":\"taskCreatedBlock\",\"type\":\"uint32\"},{\"internalType\":\"bytes\",\"name\":\"quorumNumbers\",\"type\":\"bytes\"},{\"internalType\":\"uint32\",\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\"}],\"internalType\":\"structIMangataTaskManager.Task\",\"name\":\"task\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"uint32\",\"name\":\"referenceTaskIndex\",\"type\":\"uint32\"},{\"internalType\":\"bytes32\",\"name\":\"blockHash\",\"type\":\"bytes32\"}],\"internalType\":\"structIMangataTaskManager.TaskResponse\",\"name\":\"taskResponse\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"uint32[]\",\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\"}],\"internalType\":\"structBN254.G1Point[]\",\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\"}],\"internalType\":\"structBN254.G1Point[]\",\"name\":\"quorumApks\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"uint256[2]\",\"name\":\"X\",\"type\":\"uint256[2]\"},{\"internalType\":\"uint256[2]\",\"name\":\"Y\",\"type\":\"uint256[2]\"}],\"internalType\":\"structBN254.G2Point\",\"name\":\"apkG2\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\"}],\"internalType\":\"structBN254.G1Point\",\"name\":\"sigma\",\"type\":\"tuple\"},{\"internalType\":\"uint32[]\",\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\"},{\"internalType\":\"uint32[]\",\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\"},{\"internalType\":\"uint32[][]\",\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\"}],\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\"}],\"name\":\"respondToTask\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"contractIPauserRegistry\",\"name\":\"newPauserRegistry\",\"type\":\"address\"}],\"name\":\"setPauserRegistry\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"stakeRegistry\",\"outputs\":[{\"internalType\":\"contractIStakeRegistry\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"taskNumber\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"name\":\"taskSuccesfullyChallenged\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"transferOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"msgHash\",\"type\":\"bytes32\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\"}],\"internalType\":\"structBN254.G1Point\",\"name\":\"apk\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"uint256[2]\",\"name\":\"X\",\"type\":\"uint256[2]\"},{\"internalType\":\"uint256[2]\",\"name\":\"Y\",\"type\":\"uint256[2]\"}],\"internalType\":\"structBN254.G2Point\",\"name\":\"apkG2\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\"}],\"internalType\":\"structBN254.G1Point\",\"name\":\"sigma\",\"type\":\"tuple\"}],\"name\":\"trySignatureAndApkVerification\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"pairingSuccessful\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"siganatureIsValid\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newPausedStatus\",\"type\":\"uint256\"}],\"name\":\"unpause\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]",
	Bin: "0x6101006040523480156200001257600080fd5b50604051620051aa380380620051aa833981016040819052620000359162000169565b81806001600160a01b03166080816001600160a01b031681525050806001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa1580156200008f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000b59190620001b0565b6001600160a01b031660a0816001600160a01b031681525050806001600160a01b0316633561deb16040518163ffffffff1660e01b8152600401602060405180830381865afa1580156200010d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001339190620001b0565b6001600160a01b031660c0525063ffffffff1660e05250620001d7565b6001600160a01b03811681146200016657600080fd5b50565b600080604083850312156200017d57600080fd5b82516200018a8162000150565b602084015190925063ffffffff81168114620001a557600080fd5b809150509250929050565b600060208284031215620001c357600080fd5b8151620001d08162000150565b9392505050565b60805160a05160c05160e051614f68620002426000396000818161025c0152818161051b0152610ae30152600081816103240152818161150c0152611fc9015260008181610404015281816125ca015261276b01526000818161043e015261241e0152614f686000f3fe608060405234801561001057600080fd5b50600436106101fb5760003560e01c8063683048351161011a5780638b00ce7c116100ad578063f2fde38b1161007c578063f2fde38b14610506578063f5c9899d14610519578063f63c5bab1461053f578063f8c8765e14610547578063fabc1cbc1461055a57600080fd5b80638b00ce7c146104bd5780638da5cb5b146104cd578063cefdc1d4146104de578063e3b5ee04146104ff57600080fd5b8063715018a6116100e9578063715018a61461048157806372d18e8d146104895780637afa1eed14610497578063886f1195146104aa57600080fd5b806368304835146103ff5780636b92787e146104265780636d14a987146104395780636efb46361461046057600080fd5b80633561deb111610192578063595c6a6711610161578063595c6a67146103995780635ac86ab7146103a15780635c975abb146103d45780635decc3f5146103dc57600080fd5b80633561deb11461031f5780633563b0d1146103465780633cd27f64146103665780634f739f741461037957600080fd5b8063245a7bfc116101ce578063245a7bfc14610293578063277cb995146102be5780632cb223d5146102d15780632d89f6fc146102ff57600080fd5b806310d67a2f14610200578063136439dd14610215578063171f1d5b146102285780631ad4318914610257575b600080fd5b61021361020e366004613bc6565b61056d565b005b610213610223366004613be3565b610629565b61023b610236366004613d61565b610768565b6040805192151583529015156020830152015b60405180910390f35b61027e7f000000000000000000000000000000000000000000000000000000000000000081565b60405163ffffffff909116815260200161024e565b609b546102a6906001600160a01b031681565b6040516001600160a01b03909116815260200161024e565b6102136102cc36600461409d565b6108f2565b6102f16102df366004614111565b60996020526000908152604090205481565b60405190815260200161024e565b6102f161030d366004614111565b60986020526000908152604090205481565b6102a67f000000000000000000000000000000000000000000000000000000000000000081565b61035961035436600461412e565b610d71565b60405161024e9190614275565b610213610374366004614288565b6110ec565b61038c610387366004614356565b6116a6565b60405161024e919061445a565b610213611d2a565b6103c46103af366004614515565b606654600160ff9092169190911b9081161490565b604051901515815260200161024e565b6066546102f1565b6103c46103ea366004614111565b609a6020526000908152604090205460ff1681565b6102a67f000000000000000000000000000000000000000000000000000000000000000081565b610213610434366004614538565b611df1565b6102a67f000000000000000000000000000000000000000000000000000000000000000081565b61047361046e366004614593565b611f92565b60405161024e929190614653565b6102136129fe565b60975463ffffffff1661027e565b609c546102a6906001600160a01b031681565b6065546102a6906001600160a01b031681565b60975461027e9063ffffffff1681565b6033546001600160a01b03166102a6565b6104f16104ec36600461469c565b612a12565b60405161024e9291906146de565b606461027e565b610213610514366004613bc6565b612ba4565b7f000000000000000000000000000000000000000000000000000000000000000061027e565b61027e606481565b6102136105553660046146ff565b612c1a565b610213610568366004613be3565b612d6b565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156105c0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105e4919061475b565b6001600160a01b0316336001600160a01b03161461061d5760405162461bcd60e51b815260040161061490614778565b60405180910390fd5b61062681612ec7565b50565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa158015610671573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061069591906147c2565b6106b15760405162461bcd60e51b8152600401610614906147e4565b6066548181161461072a5760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e70617573653a20696e76616c696420617474656d70742060448201527f746f20756e70617573652066756e6374696f6e616c69747900000000000000006064820152608401610614565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b60008060007f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001878760000151886020015188600001516000600281106107b0576107b061482c565b60200201518951600160200201518a602001516000600281106107d5576107d561482c565b60200201518b602001516001600281106107f1576107f161482c565b602090810291909101518c518d83015160405161084e9a99989796959401988952602089019790975260408801959095526060870193909352608086019190915260a085015260c084015260e08301526101008201526101200190565b6040516020818303038152906040528051906020012060001c6108719190614842565b90506108e461088a6108838884612fbe565b8690613055565b6108926130e9565b6108da6108cb856108c5604080518082018252600080825260209182015281518083019092526001825260029082015290565b90612fbe565b6108d48c6131a9565b90613055565b886201d4c0613239565b909890975095505050505050565b609b546001600160a01b0316331461094c5760405162461bcd60e51b815260206004820152601d60248201527f41676772656761746f72206d757374206265207468652063616c6c65720000006044820152606401610614565b600061095e6040850160208601614111565b90503660006109706040870187614864565b909250905060006109876080880160608901614111565b90506098600061099a6020890189614111565b63ffffffff1663ffffffff16815260200190815260200160002054876040516020016109c691906148d3565b6040516020818303038152906040528051906020012014610a4f5760405162461bcd60e51b815260206004820152603d60248201527f737570706c696564207461736b20646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608401610614565b6000609981610a6160208a018a614111565b63ffffffff1663ffffffff1681526020019081526020016000205414610ade5760405162461bcd60e51b815260206004820152602c60248201527f41676772656761746f722068617320616c726561647920726573706f6e64656460448201526b20746f20746865207461736b60a01b6064820152608401610614565b610b087f00000000000000000000000000000000000000000000000000000000000000008561498a565b63ffffffff164363ffffffff161115610b795760405162461bcd60e51b815260206004820152602d60248201527f41676772656761746f722068617320726573706f6e64656420746f207468652060448201526c7461736b20746f6f206c61746560981b6064820152608401610614565b600086604051602001610b8c91906149d0565b604051602081830303815290604052805190602001209050600080610bb48387878a8c611f92565b9150915060005b85811015610cb3578460ff1683602001518281518110610bdd57610bdd61482c565b6020026020010151610bef91906149de565b6001600160601b0316606484600001518381518110610c1057610c1061482c565b60200260200101516001600160601b0316610c2b9190614a0d565b1015610ca1576040805162461bcd60e51b81526020600482015260248101919091527f5369676e61746f7269657320646f206e6f74206f776e206174206c656173742060448201527f7468726573686f6c642070657263656e74616765206f6620612071756f72756d6064820152608401610614565b80610cab81614a2c565b915050610bbb565b5060408051808201825263ffffffff43168152602080820184905291519091610ce0918c91849101614a47565b60405160208183030381529060405280519060200120609960008c6000016020810190610d0d9190614111565b63ffffffff1663ffffffff168152602001908152602001600020819055507ff2af11fad73d4349c99cf62f298d337641ea0bb7c0f5a8db92a98a275f734f588a82604051610d5c929190614a47565b60405180910390a15050505050505050505050565b60606000846001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa158015610db3573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610dd7919061475b565b90506000856001600160a01b0316639e9923c26040518163ffffffff1660e01b8152600401602060405180830381865afa158015610e19573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610e3d919061475b565b9050600085516001600160401b03811115610e5a57610e5a613bfc565b604051908082528060200260200182016040528015610e8d57816020015b6060815260200190600190039081610e785790505b50905060005b86518110156110df576000878281518110610eb057610eb061482c565b016020015160405163889ae3e560e01b815260f89190911c6004820181905263ffffffff8916602483015291506000906001600160a01b0386169063889ae3e590604401600060405180830381865afa158015610f11573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610f399190810190614a73565b905080516001600160401b03811115610f5457610f54613bfc565b604051908082528060200260200182016040528015610f9957816020015b6040805180820190915260008082526020820152815260200190600190039081610f725790505b50848481518110610fac57610fac61482c565b602002602001018190525060005b81518110156110c9576000828281518110610fd757610fd761482c565b6020908102919091018101516040805180820182528281529051631b32722560e01b81526004810183905260ff8816602482015263ffffffff8e166044820152919350918201906001600160a01b038b1690631b32722590606401602060405180830381865afa15801561104f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906110739190614b03565b6001600160601b03168152508686815181106110915761109161482c565b602002602001015183815181106110aa576110aa61482c565b60200260200101819052505080806110c190614a2c565b915050610fba565b50505080806110d790614a2c565b915050610e93565b50925050505b9392505050565b60006110fb6020850185614111565b63ffffffff811660009081526099602052604090205490915085359061116d5760405162461bcd60e51b815260206004820152602160248201527f5461736b206861736e2774206265656e20726573706f6e64656420746f2079656044820152601d60fa1b6064820152608401610614565b8484604051602001611180929190614b2c565b60408051601f19818403018152918152815160209283012063ffffffff8516600090815260999093529120541461121f5760405162461bcd60e51b815260206004820152603d60248201527f5461736b20726573706f6e736520646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608401610614565b63ffffffff82166000908152609a602052604090205460ff16156112b75760405162461bcd60e51b815260206004820152604360248201527f54686520726573706f6e736520746f2074686973207461736b2068617320616c60448201527f7265616479206265656e206368616c6c656e676564207375636365737366756c606482015262363c9760e91b608482015260a401610614565b60646112c66020860186614111565b6112d0919061498a565b63ffffffff164363ffffffff1611156113515760405162461bcd60e51b815260206004820152603760248201527f546865206368616c6c656e676520706572696f6420666f72207468697320746160448201527f736b2068617320616c726561647920657870697265642e0000000000000000006064820152608401610614565b6001604051339063ffffffff8516907ffd3e26beeb5967fc5a57a0446914eabc45b4aa474c67a51b4b5160cac60ddb0590600090a35050506116a0565b86518110156113e8576113b98782815181106113ac576113ac61482c565b602002602001015161345d565b8282815181106113cb576113cb61482c565b6020908102919091010152806113e081614a2c565b91505061138e565b5060006113fb60408b0160208c01614111565b8260405160200161140d929190614b47565b604051602081830303815290604052805190602001209050876020013581146114b75760405162461bcd60e51b815260206004820152605060248201527f546865207075626b657973206f66206e6f6e2d7369676e696e67206f7065726160448201527f746f727320737570706c69656420627920746865206368616c6c656e6765722060648201526f30b932903737ba1031b7b93932b1ba1760811b608482015260a401610614565b600087516001600160401b038111156114d2576114d2613bfc565b6040519080825280602002602001820160405280156114fb578160200160208202803683370190505b50905060005b885181101561164f577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663187548c86040518163ffffffff1660e01b8152600401602060405180830381865afa158015611568573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061158c919061475b565b6001600160a01b031663e8bb9ae68583815181106115ac576115ac61482c565b60200260200101516040518263ffffffff1660e01b81526004016115d291815260200190565b602060405180830381865afa1580156115ef573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611613919061475b565b8282815181106116255761162561482c565b6001600160a01b03909216602092830291909101909101528061164781614a2c565b915050611501565b5063ffffffff87166000818152609a6020526040808220805460ff19166001179055513392917fc20d1bb0f1623680306b83d4ff4bb99a2beb9d86d97832f3ca40fd13a29df1ec91a3505050505050505b50505050565b6116d16040518060800160405280606081526020016060815260200160608152602001606081525090565b6000876001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa158015611711573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611735919061475b565b90506117626040518060800160405280606081526020016060815260200160608152602001606081525090565b6040516385020d4960e01b81526001600160a01b038a16906385020d4990611792908b9089908990600401614b8f565b600060405180830381865afa1580156117af573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526117d79190810190614bd9565b815260405163e192e9ad60e01b81526001600160a01b0383169063e192e9ad90611809908b908b908b90600401614c67565b600060405180830381865afa158015611826573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261184e9190810190614bd9565b6040820152856001600160401b0381111561186b5761186b613bfc565b60405190808252806020026020018201604052801561189e57816020015b60608152602001906001900390816118895790505b50606082015260005b60ff8116871115611c3b576000856001600160401b038111156118cc576118cc613bfc565b6040519080825280602002602001820160405280156118f5578160200160208202803683370190505b5083606001518360ff168151811061190f5761190f61482c565b602002602001018190525060005b86811015611b3b5760008c6001600160a01b0316633064620d8a8a858181106119485761194861482c565b905060200201358e886000015186815181106119665761196661482c565b60200260200101516040518463ffffffff1660e01b81526004016119a39392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa1580156119c0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906119e49190614c90565b90508a8a8560ff168181106119fb576119fb61482c565b6001600160c01b03841692013560f81c9190911c600190811614159050611b2857856001600160a01b031663480858668a8a85818110611a3d57611a3d61482c565b905060200201358d8d8860ff16818110611a5957611a5961482c565b6040516001600160e01b031960e087901b1681526004810194909452919091013560f81c60248301525063ffffffff8f166044820152606401602060405180830381865afa158015611aaf573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611ad39190614cb9565b85606001518560ff1681518110611aec57611aec61482c565b60200260200101518481518110611b0557611b0561482c565b63ffffffff9092166020928302919091019091015282611b2481614a2c565b9350505b5080611b3381614a2c565b91505061191d565b506000816001600160401b03811115611b5657611b56613bfc565b604051908082528060200260200182016040528015611b7f578160200160208202803683370190505b50905060005b82811015611c005784606001518460ff1681518110611ba657611ba661482c565b60200260200101518181518110611bbf57611bbf61482c565b6020026020010151828281518110611bd957611bd961482c565b63ffffffff9092166020928302919091019091015280611bf881614a2c565b915050611b85565b508084606001518460ff1681518110611c1b57611c1b61482c565b602002602001018190525050508080611c3390614cd6565b9150506118a7565b506000896001600160a01b0316633561deb16040518163ffffffff1660e01b8152600401602060405180830381865afa158015611c7c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611ca0919061475b565b60405163eda1076360e01b81529091506001600160a01b0382169063eda1076390611cd3908b908b908e90600401614cf6565b600060405180830381865afa158015611cf0573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611d189190810190614bd9565b60208301525098975050505050505050565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa158015611d72573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611d9691906147c2565b611db25760405162461bcd60e51b8152600401610614906147e4565b600019606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b609c546001600160a01b03163314611e555760405162461bcd60e51b815260206004820152602160248201527f5461736b2067656e657261746f72206d757374206265207468652063616c6c656044820152603960f91b6064820152608401610614565b611e8c604051806080016040528060008152602001600063ffffffff16815260200160608152602001600063ffffffff1681525090565b84815263ffffffff438116602080840191909152908516606083015260408051601f850183900483028101830190915283815290849084908190840183828082843760009201919091525050505060408083019190915251611ef2908290602001614d4c565b60408051601f1981840301815282825280516020918201206097805463ffffffff9081166000908152609890945293909220555416907f1695b8d06ec800b4615e745cfb5bd00c1f2875615d42925c3b5afa543bb24c4890611f55908490614d4c565b60405180910390a2609754611f719063ffffffff16600161498a565b6097805463ffffffff191663ffffffff929092169190911790555050505050565b60408051808201909152606080825260208201526040805180820190915260008082526020820181905290815b868110156121af577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663c1af6b248989848181106120085761200861482c565b9050013560f81c60f81b60f81c888860a00151858151811061202c5761202c61482c565b60209081029190910101516040516001600160e01b031960e086901b16815260ff909316600484015263ffffffff9182166024840152166044820152606401602060405180830381865afa158015612088573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906120ac9190614db1565b6001600160401b0319166120cf866040015183815181106113ac576113ac61482c565b67ffffffffffffffff19161461216b5760405162461bcd60e51b81526020600482015260616024820152600080516020614f1383398151915260448201527f7265733a2071756f72756d41706b206861736820696e2073746f72616765206460648201527f6f6573206e6f74206d617463682070726f76696465642071756f72756d2061706084820152606b60f81b60a482015260c401610614565b61219b856040015182815181106121845761218461482c565b60200260200101518361305590919063ffffffff16565b9150806121a781614a2c565b915050611fbf565b506040805180820190915260608082526020820152866001600160401b038111156121dc576121dc613bfc565b604051908082528060200260200182016040528015612205578160200160208202803683370190505b506020820152866001600160401b0381111561222357612223613bfc565b60405190808252806020026020018201604052801561224c578160200160208202803683370190505b5081526020850151516000906001600160401b0381111561226f5761226f613bfc565b604051908082528060200260200182016040528015612298578160200160208202803683370190505b50905060008660200151516001600160401b038111156122ba576122ba613bfc565b6040519080825280602002602001820160405280156122e3578160200160208202803683370190505b50905060006123278b8b8080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152506134a092505050565b905060005b88602001515181101561259257612352896020015182815181106113ac576113ac61482c565b8482815181106123645761236461482c565b6020908102919091010152801561241c5783612381600183614ddc565b815181106123915761239161482c565b602002602001015160001c8482815181106123ae576123ae61482c565b602002602001015160001c1161241c576040805162461bcd60e51b8152602060048201526024810191909152600080516020614f1383398151915260448201527f7265733a206e6f6e5369676e65725075626b657973206e6f7420736f727465646064820152608401610614565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316633064620d85838151811061245d5761245d61482c565b60200260200101518c8c60000151858151811061247c5761247c61482c565b60200260200101516040518463ffffffff1660e01b81526004016124b99392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa1580156124d6573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906124fa9190614c90565b6001600160c01b03168382815181106125155761251561482c565b60200260200101818152505061257e61257761254b8486858151811061253d5761253d61482c565b602002602001015116613609565b6125718c6020015185815181106125645761256461482c565b602002602001015161363a565b906136d5565b8790613055565b95508061258a81614a2c565b91505061232c565b505060005b60ff81168a11156128d25760008b8b8360ff168181106125b9576125b961482c565b9050013560f81c60f81b60f81c90507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663c8294c56828c8c60c001518660ff16815181106126125761261261482c565b60209081029190910101516040516001600160e01b031960e086901b16815260ff909316600484015263ffffffff9182166024840152166044820152606401602060405180830381865afa15801561266e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906126929190614b03565b85602001518360ff16815181106126ab576126ab61482c565b6001600160601b03909216602092830291909101820152850151805160ff84169081106126da576126da61482c565b602002602001015185600001518360ff16815181106126fb576126fb61482c565b60200260200101906001600160601b031690816001600160601b03168152505060005b8960200151518163ffffffff1610156128c8576000612764858363ffffffff168151811061274e5761274e61482c565b60200260200101518460ff161c60019081161490565b156128b5577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663a43cde89848e898663ffffffff16815181106127b2576127b261482c565b60200260200101518f60e001518960ff16815181106127d3576127d361482c565b60200260200101518663ffffffff16815181106127f2576127f261482c565b60209081029190910101516040516001600160e01b031960e087901b16815260ff909416600485015263ffffffff92831660248501526044840191909152166064820152608401602060405180830381865afa158015612856573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061287a9190614b03565b8751805160ff87169081106128915761289161482c565b602002602001018181516128a59190614df3565b6001600160601b03169052506001015b50806128c081614e1b565b91505061271e565b5050600101612597565b50506000806128eb8c868a606001518b60800151610768565b915091508161295c5760405162461bcd60e51b81526020600482015260436024820152600080516020614f1383398151915260448201527f7265733a2070616972696e6720707265636f6d70696c652063616c6c206661696064820152621b195960ea1b608482015260a401610614565b806129bd5760405162461bcd60e51b81526020600482015260396024820152600080516020614f1383398151915260448201527f7265733a207369676e617475726520697320696e76616c6964000000000000006064820152608401610614565b5050600087826040516020016129d4929190614b47565b60408051808303601f190181529190528051602090910120929b929a509198505050505050505050565b612a066137ba565b612a106000613814565b565b6040805160018082528183019092526000916060918391602080830190803683370190505090508481600081518110612a4d57612a4d61482c565b60209081029190910101526040516385020d4960e01b81526000906001600160a01b038816906385020d4990612a899088908690600401614e3f565b600060405180830381865afa158015612aa6573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052612ace9190810190614bd9565b600081518110612ae057612ae061482c565b6020908102919091010151604051633064620d60e01b81526004810188905263ffffffff87811660248301529091166044820181905291506000906001600160a01b03891690633064620d90606401602060405180830381865afa158015612b4c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612b709190614c90565b6001600160c01b031690506000612b8682613866565b905081612b948a838a610d71565b9550955050505050935093915050565b612bac6137ba565b6001600160a01b038116612c115760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610614565b61062681613814565b600054610100900460ff1615808015612c3a5750600054600160ff909116105b80612c545750303b158015612c54575060005460ff166001145b612cb75760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610614565b6000805460ff191660011790558015612cda576000805461ff0019166101001790555b612ce58560006138c3565b612cee84613814565b609b80546001600160a01b038086166001600160a01b031992831617909255609c8054928516929091169190911790558015612d64576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b5050505050565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612dbe573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612de2919061475b565b6001600160a01b0316336001600160a01b031614612e125760405162461bcd60e51b815260040161061490614778565b606654198119606654191614612e905760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e756e70617573653a20696e76616c696420617474656d7060448201527f7420746f2070617573652066756e6374696f6e616c69747900000000000000006064820152608401610614565b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c9060200161075d565b6001600160a01b038116612f555760405162461bcd60e51b815260206004820152604960248201527f5061757361626c652e5f73657450617573657252656769737472793a206e657760448201527f50617573657252656769737472792063616e6e6f7420626520746865207a65726064820152686f206164647265737360b81b608482015260a401610614565b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b6040805180820190915260008082526020820152612fda613ad7565b835181526020808501519082015260408082018490526000908360608460076107d05a03fa905080801561300d5761300f565bfe5b508061304d5760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b6044820152606401610614565b505092915050565b6040805180820190915260008082526020820152613071613af5565b835181526020808501518183015283516040808401919091529084015160608301526000908360808460066107d05a03fa905080801561300d57508061304d5760405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b6044820152606401610614565b6130f1613b13565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b6040805180820190915260008082526020820152600080806131d9600080516020614ef383398151915286614842565b90505b6131e5816139ad565b9093509150600080516020614ef383398151915282830983141561321f576040805180820190915290815260208101919091529392505050565b600080516020614ef38339815191526001820890506131dc565b60408051808201825286815260208082018690528251808401909352868352820184905260009182919061326b613b38565b60005b6002811015613430576000613284826006614a0d565b90508482600281106132985761329861482c565b602002015151836132aa836000614e93565b600c81106132ba576132ba61482c565b60200201528482600281106132d1576132d161482c565b602002015160200151838260016132e89190614e93565b600c81106132f8576132f861482c565b602002015283826002811061330f5761330f61482c565b6020020151515183613322836002614e93565b600c81106133325761333261482c565b60200201528382600281106133495761334961482c565b6020020151516001602002015183613362836003614e93565b600c81106133725761337261482c565b60200201528382600281106133895761338961482c565b6020020151602001516000600281106133a4576133a461482c565b6020020151836133b5836004614e93565b600c81106133c5576133c561482c565b60200201528382600281106133dc576133dc61482c565b6020020151602001516001600281106133f7576133f761482c565b602002015183613408836005614e93565b600c81106134185761341861482c565b6020020152508061342881614a2c565b91505061326e565b50613439613b57565b60006020826101808560088cfa9151919c9115159b50909950505050505050505050565b600081600001518260200151604051602001613483929190918252602082015260400190565b604051602081830303815290604052805190602001209050919050565b6000610100825111156135145760405162461bcd60e51b815260206004820152603660248201527f4269746d61705574696c732e62797465734172726179546f4269746d61703a206044820152756279746573417272617920697320746f6f206c6f6e6760501b6064820152608401610614565b815161352257506000919050565b600080836000815181106135385761353861482c565b0160200151600160f89190911c81901b92505b8451811015613600578481815181106135665761356661482c565b0160200151600160f89190911c1b9150828216156135ec5760405162461bcd60e51b815260206004820152603a60248201527f4269746d61705574696c732e62797465734172726179546f4269746d61703a2060448201527f72657065617420656e74727920696e20627974657341727261790000000000006064820152608401610614565b918117916135f981614a2c565b905061354b565b50909392505050565b6000805b82156136345761361e600184614ddc565b909216918061362c81614eab565b91505061360d565b92915050565b6040805180820190915260008082526020820152815115801561365f57506020820151155b1561367d575050604080518082019091526000808252602082015290565b604051806040016040528083600001518152602001600080516020614ef383398151915284602001516136b09190614842565b6136c890600080516020614ef3833981519152614ddc565b905292915050565b919050565b60408051808201909152600080825260208201526102008261ffff16106137315760405162461bcd60e51b815260206004820152601060248201526f7363616c61722d746f6f2d6c6172676560801b6044820152606401610614565b8161ffff1660011415613745575081613634565b6040805180820190915260008082526020820181905284906001905b8161ffff168661ffff1611156137af57600161ffff871660ff83161c811614156137925761378f8484613055565b93505b61379c8384613055565b92506201fffe600192831b169101613761565b509195945050505050565b6033546001600160a01b03163314612a105760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610614565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b60606000805b6101008110156138bc576001811b9150838216156138ac57828160f81b60405160200161389a929190614ec3565b60405160208183030381529060405292505b6138b581614a2c565b905061386c565b5050919050565b6065546001600160a01b03161580156138e457506001600160a01b03821615155b6139665760405162461bcd60e51b815260206004820152604760248201527f5061757361626c652e5f696e697469616c697a655061757365723a205f696e6960448201527f7469616c697a6550617573657228292063616e206f6e6c792062652063616c6c6064820152666564206f6e636560c81b608482015260a401610614565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a26139a982612ec7565b5050565b60008080600080516020614ef38339815191526003600080516020614ef383398151915286600080516020614ef3833981519152888909090890506000613a23827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f52600080516020614ef3833981519152613a2f565b91959194509092505050565b600080613a3a613b57565b613a42613b75565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa925082801561300d575082613acc5760405162461bcd60e51b815260206004820152601a60248201527f424e3235342e6578704d6f643a2063616c6c206661696c7572650000000000006044820152606401610614565b505195945050505050565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b6040518060400160405280613b26613b93565b8152602001613b33613b93565b905290565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b60405180604001604052806002906020820280368337509192915050565b6001600160a01b038116811461062657600080fd5b600060208284031215613bd857600080fd5b81356110e581613bb1565b600060208284031215613bf557600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b0381118282101715613c3457613c34613bfc565b60405290565b60405161010081016001600160401b0381118282101715613c3457613c34613bfc565b604051601f8201601f191681016001600160401b0381118282101715613c8557613c85613bfc565b604052919050565b600060408284031215613c9f57600080fd5b613ca7613c12565b9050813581526020820135602082015292915050565b600082601f830112613cce57600080fd5b604051604081018181106001600160401b0382111715613cf057613cf0613bfc565b8060405250806040840185811115613d0757600080fd5b845b818110156137af578035835260209283019201613d09565b600060808284031215613d3357600080fd5b613d3b613c12565b9050613d478383613cbd565b8152613d568360408401613cbd565b602082015292915050565b6000806000806101208587031215613d7857600080fd5b84359350613d898660208701613c8d565b9250613d988660608701613d21565b9150613da78660e08701613c8d565b905092959194509250565b600060808284031215613dc457600080fd5b50919050565b600060408284031215613dc457600080fd5b60006001600160401b03821115613df557613df5613bfc565b5060051b60200190565b63ffffffff8116811461062657600080fd5b80356136d081613dff565b600082601f830112613e2d57600080fd5b81356020613e42613e3d83613ddc565b613c5d565b82815260059290921b84018101918181019086841115613e6157600080fd5b8286015b84811015613e85578035613e7881613dff565b8352918301918301613e65565b509695505050505050565b600082601f830112613ea157600080fd5b81356020613eb1613e3d83613ddc565b82815260069290921b84018101918181019086841115613ed057600080fd5b8286015b84811015613e8557613ee68882613c8d565b835291830191604001613ed4565b600082601f830112613f0557600080fd5b81356020613f15613e3d83613ddc565b82815260059290921b84018101918181019086841115613f3457600080fd5b8286015b84811015613e855780356001600160401b03811115613f575760008081fd5b613f658986838b0101613e1c565b845250918301918301613f38565b60006101808284031215613f8657600080fd5b613f8e613c3a565b905081356001600160401b0380821115613fa757600080fd5b613fb385838601613e1c565b83526020840135915080821115613fc957600080fd5b613fd585838601613e90565b60208401526040840135915080821115613fee57600080fd5b613ffa85838601613e90565b604084015261400c8560608601613d21565b606084015261401e8560e08601613c8d565b608084015261012084013591508082111561403857600080fd5b61404485838601613e1c565b60a084015261014084013591508082111561405e57600080fd5b61406a85838601613e1c565b60c084015261016084013591508082111561408457600080fd5b5061409184828501613ef4565b60e08301525092915050565b6000806000608084860312156140b257600080fd5b83356001600160401b03808211156140c957600080fd5b6140d587838801613db2565b94506140e48760208801613dca565b935060608601359150808211156140fa57600080fd5b5061410786828701613f73565b9150509250925092565b60006020828403121561412357600080fd5b81356110e581613dff565b60008060006060848603121561414357600080fd5b833561414e81613bb1565b92506020848101356001600160401b038082111561416b57600080fd5b818701915087601f83011261417f57600080fd5b81358181111561419157614191613bfc565b6141a3601f8201601f19168501613c5d565b915080825288848285010111156141b957600080fd5b80848401858401376000848284010152508094505050506141dc60408501613e11565b90509250925092565b600081518084526020808501808196508360051b810191508286016000805b86811015614267578385038a52825180518087529087019087870190845b81811015614252578351805184528a01516001600160601b03168a84015292890192604090920191600101614222565b50509a87019a95505091850191600101614204565b509298975050505050505050565b6020815260006110e560208301846141e5565b60008060008060c0858703121561429e57600080fd5b84356001600160401b03808211156142b557600080fd5b6142c188838901613db2565b95506142d08860208901613dca565b94506142df8860608901613dca565b935060a08701359150808211156142f557600080fd5b5061430287828801613e90565b91505092959194509250565b60008083601f84011261432057600080fd5b5081356001600160401b0381111561433757600080fd5b60208301915083602082850101111561434f57600080fd5b9250929050565b6000806000806000806080878903121561436f57600080fd5b863561437a81613bb1565b9550602087013561438a81613dff565b945060408701356001600160401b03808211156143a657600080fd5b6143b28a838b0161430e565b909650945060608901359150808211156143cb57600080fd5b818901915089601f8301126143df57600080fd5b8135818111156143ee57600080fd5b8a60208260051b850101111561440357600080fd5b6020830194508093505050509295509295509295565b600081518084526020808501945080840160005b8381101561444f57815163ffffffff168752958201959082019060010161442d565b509495945050505050565b60006020808352835160808285015261447660a0850182614419565b905081850151601f19808684030160408701526144938383614419565b925060408701519150808684030160608701526144b08383614419565b60608801518782038301608089015280518083529194508501925084840190600581901b8501860160005b8281101561450757848783030184526144f5828751614419565b958801959388019391506001016144db565b509998505050505050505050565b60006020828403121561452757600080fd5b813560ff811681146110e557600080fd5b6000806000806060858703121561454e57600080fd5b84359350602085013561456081613dff565b925060408501356001600160401b0381111561457b57600080fd5b6145878782880161430e565b95989497509550505050565b6000806000806000608086880312156145ab57600080fd5b8535945060208601356001600160401b03808211156145c957600080fd5b6145d589838a0161430e565b9096509450604088013591506145ea82613dff565b9092506060870135908082111561460057600080fd5b5061460d88828901613f73565b9150509295509295909350565b600081518084526020808501945080840160005b8381101561444f5781516001600160601b03168752958201959082019060010161462e565b604081526000835160408084015261466e608084018261461a565b90506020850151603f1984830301606085015261468b828261461a565b925050508260208301529392505050565b6000806000606084860312156146b157600080fd5b83356146bc81613bb1565b92506020840135915060408401356146d381613dff565b809150509250925092565b8281526040602082015260006146f760408301846141e5565b949350505050565b6000806000806080858703121561471557600080fd5b843561472081613bb1565b9350602085013561473081613bb1565b9250604085013561474081613bb1565b9150606085013561475081613bb1565b939692955090935050565b60006020828403121561476d57600080fd5b81516110e581613bb1565b6020808252602a908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526939903ab73830bab9b2b960b11b606082015260800190565b6000602082840312156147d457600080fd5b815180151581146110e557600080fd5b60208082526028908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526739903830bab9b2b960c11b606082015260800190565b634e487b7160e01b600052603260045260246000fd5b60008261485f57634e487b7160e01b600052601260045260246000fd5b500690565b6000808335601e1984360301811261487b57600080fd5b8301803591506001600160401b0382111561489557600080fd5b60200191503681900382131561434f57600080fd5b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b6020815281356020820152600060208301356148ee81613dff565b63ffffffff81166040840152506040830135601e1984360301811261491257600080fd5b830180356001600160401b0381111561492a57600080fd5b80360385131561493957600080fd5b6080606085015261495160a0850182602085016148aa565b91505061496060608501613e11565b63ffffffff81166080850152509392505050565b634e487b7160e01b600052601160045260246000fd5b600063ffffffff8083168185168083038211156149a9576149a9614974565b01949350505050565b80356149bd81613dff565b63ffffffff168252602090810135910152565b6040810161363482846149b2565b60006001600160601b0380831681851681830481118215151615614a0457614a04614974565b02949350505050565b6000816000190483118215151615614a2757614a27614974565b500290565b6000600019821415614a4057614a40614974565b5060010190565b60808101614a5582856149b2565b63ffffffff8351166040830152602083015160608301529392505050565b60006020808385031215614a8657600080fd5b82516001600160401b03811115614a9c57600080fd5b8301601f81018513614aad57600080fd5b8051614abb613e3d82613ddc565b81815260059190911b82018301908381019087831115614ada57600080fd5b928401925b82841015614af857835182529284019290840190614adf565b979650505050505050565b600060208284031215614b1557600080fd5b81516001600160601b03811681146110e557600080fd5b60808101614b3a82856149b2565b6110e560408301846149b2565b63ffffffff60e01b8360e01b1681526000600482018351602080860160005b83811015614b8257815185529382019390820190600101614b66565b5092979650505050505050565b63ffffffff84168152604060208201819052810182905260006001600160fb1b03831115614bbc57600080fd5b8260051b8085606085013760009201606001918252509392505050565b60006020808385031215614bec57600080fd5b82516001600160401b03811115614c0257600080fd5b8301601f81018513614c1357600080fd5b8051614c21613e3d82613ddc565b81815260059190911b82018301908381019087831115614c4057600080fd5b928401925b82841015614af8578351614c5881613dff565b82529284019290840190614c45565b63ffffffff84168152604060208201526000614c876040830184866148aa565b95945050505050565b600060208284031215614ca257600080fd5b81516001600160c01b03811681146110e557600080fd5b600060208284031215614ccb57600080fd5b81516110e581613dff565b600060ff821660ff811415614ced57614ced614974565b60010192915050565b604081526000614d0a6040830185876148aa565b905063ffffffff83166020830152949350505050565b60005b83811015614d3b578181015183820152602001614d23565b838111156116a05750506000910152565b60208152815160208201526000602083015163ffffffff8082166040850152604085015191506080606085015281518060a0860152614d928160c0870160208601614d20565b60609590950151166080840152505060c0601f909201601f1916010190565b600060208284031215614dc357600080fd5b815167ffffffffffffffff19811681146110e557600080fd5b600082821015614dee57614dee614974565b500390565b60006001600160601b0383811690831681811015614e1357614e13614974565b039392505050565b600063ffffffff80831681811415614e3557614e35614974565b6001019392505050565b60006040820163ffffffff851683526020604081850152818551808452606086019150828701935060005b81811015614e8657845183529383019391830191600101614e6a565b5090979650505050505050565b60008219821115614ea657614ea6614974565b500190565b600061ffff80831681811415614e3557614e35614974565b60008351614ed5818460208801614d20565b6001600160f81b031993909316919092019081526001019291505056fe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47424c535369676e6174757265436865636b65722e636865636b5369676e617475a2646970667358221220e27d6675a64669875b559691cdce0f0464b88d27000eb25ecc06e77cfc014f4164736f6c634300080c0033",
}

// ContractMangataTaskManagerABI is the input ABI used to generate the binding from.
// Deprecated: Use ContractMangataTaskManagerMetaData.ABI instead.
var ContractMangataTaskManagerABI = ContractMangataTaskManagerMetaData.ABI

// ContractMangataTaskManagerBin is the compiled bytecode used for deploying new contracts.
// Deprecated: Use ContractMangataTaskManagerMetaData.Bin instead.
var ContractMangataTaskManagerBin = ContractMangataTaskManagerMetaData.Bin

// DeployContractMangataTaskManager deploys a new Ethereum contract, binding an instance of ContractMangataTaskManager to it.
func DeployContractMangataTaskManager(auth *bind.TransactOpts, backend bind.ContractBackend, _registryCoordinator common.Address, _taskResponseWindowBlock uint32) (common.Address, *types.Transaction, *ContractMangataTaskManager, error) {
	parsed, err := ContractMangataTaskManagerMetaData.GetAbi()
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	if parsed == nil {
		return common.Address{}, nil, nil, errors.New("GetABI returned nil")
	}

	address, tx, contract, err := bind.DeployContract(auth, *parsed, common.FromHex(ContractMangataTaskManagerBin), backend, _registryCoordinator, _taskResponseWindowBlock)
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	return address, tx, &ContractMangataTaskManager{ContractMangataTaskManagerCaller: ContractMangataTaskManagerCaller{contract: contract}, ContractMangataTaskManagerTransactor: ContractMangataTaskManagerTransactor{contract: contract}, ContractMangataTaskManagerFilterer: ContractMangataTaskManagerFilterer{contract: contract}}, nil
}

// ContractMangataTaskManager is an auto generated Go binding around an Ethereum contract.
type ContractMangataTaskManager struct {
	ContractMangataTaskManagerCaller     // Read-only binding to the contract
	ContractMangataTaskManagerTransactor // Write-only binding to the contract
	ContractMangataTaskManagerFilterer   // Log filterer for contract events
}

// ContractMangataTaskManagerCaller is an auto generated read-only Go binding around an Ethereum contract.
type ContractMangataTaskManagerCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractMangataTaskManagerTransactor is an auto generated write-only Go binding around an Ethereum contract.
type ContractMangataTaskManagerTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractMangataTaskManagerFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type ContractMangataTaskManagerFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractMangataTaskManagerSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type ContractMangataTaskManagerSession struct {
	Contract     *ContractMangataTaskManager // Generic contract binding to set the session for
	CallOpts     bind.CallOpts               // Call options to use throughout this session
	TransactOpts bind.TransactOpts           // Transaction auth options to use throughout this session
}

// ContractMangataTaskManagerCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type ContractMangataTaskManagerCallerSession struct {
	Contract *ContractMangataTaskManagerCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts                     // Call options to use throughout this session
}

// ContractMangataTaskManagerTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type ContractMangataTaskManagerTransactorSession struct {
	Contract     *ContractMangataTaskManagerTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts                     // Transaction auth options to use throughout this session
}

// ContractMangataTaskManagerRaw is an auto generated low-level Go binding around an Ethereum contract.
type ContractMangataTaskManagerRaw struct {
	Contract *ContractMangataTaskManager // Generic contract binding to access the raw methods on
}

// ContractMangataTaskManagerCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type ContractMangataTaskManagerCallerRaw struct {
	Contract *ContractMangataTaskManagerCaller // Generic read-only contract binding to access the raw methods on
}

// ContractMangataTaskManagerTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type ContractMangataTaskManagerTransactorRaw struct {
	Contract *ContractMangataTaskManagerTransactor // Generic write-only contract binding to access the raw methods on
}

// NewContractMangataTaskManager creates a new instance of ContractMangataTaskManager, bound to a specific deployed contract.
func NewContractMangataTaskManager(address common.Address, backend bind.ContractBackend) (*ContractMangataTaskManager, error) {
	contract, err := bindContractMangataTaskManager(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &ContractMangataTaskManager{ContractMangataTaskManagerCaller: ContractMangataTaskManagerCaller{contract: contract}, ContractMangataTaskManagerTransactor: ContractMangataTaskManagerTransactor{contract: contract}, ContractMangataTaskManagerFilterer: ContractMangataTaskManagerFilterer{contract: contract}}, nil
}

// NewContractMangataTaskManagerCaller creates a new read-only instance of ContractMangataTaskManager, bound to a specific deployed contract.
func NewContractMangataTaskManagerCaller(address common.Address, caller bind.ContractCaller) (*ContractMangataTaskManagerCaller, error) {
	contract, err := bindContractMangataTaskManager(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &ContractMangataTaskManagerCaller{contract: contract}, nil
}

// NewContractMangataTaskManagerTransactor creates a new write-only instance of ContractMangataTaskManager, bound to a specific deployed contract.
func NewContractMangataTaskManagerTransactor(address common.Address, transactor bind.ContractTransactor) (*ContractMangataTaskManagerTransactor, error) {
	contract, err := bindContractMangataTaskManager(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &ContractMangataTaskManagerTransactor{contract: contract}, nil
}

// NewContractMangataTaskManagerFilterer creates a new log filterer instance of ContractMangataTaskManager, bound to a specific deployed contract.
func NewContractMangataTaskManagerFilterer(address common.Address, filterer bind.ContractFilterer) (*ContractMangataTaskManagerFilterer, error) {
	contract, err := bindContractMangataTaskManager(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &ContractMangataTaskManagerFilterer{contract: contract}, nil
}

// bindContractMangataTaskManager binds a generic wrapper to an already deployed contract.
func bindContractMangataTaskManager(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := ContractMangataTaskManagerMetaData.GetAbi()
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, *parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_ContractMangataTaskManager *ContractMangataTaskManagerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _ContractMangataTaskManager.Contract.ContractMangataTaskManagerCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_ContractMangataTaskManager *ContractMangataTaskManagerRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractMangataTaskManager.Contract.ContractMangataTaskManagerTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_ContractMangataTaskManager *ContractMangataTaskManagerRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _ContractMangataTaskManager.Contract.ContractMangataTaskManagerTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_ContractMangataTaskManager *ContractMangataTaskManagerCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _ContractMangataTaskManager.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_ContractMangataTaskManager *ContractMangataTaskManagerTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractMangataTaskManager.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_ContractMangataTaskManager *ContractMangataTaskManagerTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _ContractMangataTaskManager.Contract.contract.Transact(opts, method, params...)
}

// TASKCHALLENGEWINDOWBLOCK is a free data retrieval call binding the contract method 0xf63c5bab.
//
// Solidity: function TASK_CHALLENGE_WINDOW_BLOCK() view returns(uint32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCaller) TASKCHALLENGEWINDOWBLOCK(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractMangataTaskManager.contract.Call(opts, &out, "TASK_CHALLENGE_WINDOW_BLOCK")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// TASKCHALLENGEWINDOWBLOCK is a free data retrieval call binding the contract method 0xf63c5bab.
//
// Solidity: function TASK_CHALLENGE_WINDOW_BLOCK() view returns(uint32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) TASKCHALLENGEWINDOWBLOCK() (uint32, error) {
	return _ContractMangataTaskManager.Contract.TASKCHALLENGEWINDOWBLOCK(&_ContractMangataTaskManager.CallOpts)
}

// TASKCHALLENGEWINDOWBLOCK is a free data retrieval call binding the contract method 0xf63c5bab.
//
// Solidity: function TASK_CHALLENGE_WINDOW_BLOCK() view returns(uint32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCallerSession) TASKCHALLENGEWINDOWBLOCK() (uint32, error) {
	return _ContractMangataTaskManager.Contract.TASKCHALLENGEWINDOWBLOCK(&_ContractMangataTaskManager.CallOpts)
}

// TASKRESPONSEWINDOWBLOCK is a free data retrieval call binding the contract method 0x1ad43189.
//
// Solidity: function TASK_RESPONSE_WINDOW_BLOCK() view returns(uint32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCaller) TASKRESPONSEWINDOWBLOCK(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractMangataTaskManager.contract.Call(opts, &out, "TASK_RESPONSE_WINDOW_BLOCK")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// TASKRESPONSEWINDOWBLOCK is a free data retrieval call binding the contract method 0x1ad43189.
//
// Solidity: function TASK_RESPONSE_WINDOW_BLOCK() view returns(uint32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) TASKRESPONSEWINDOWBLOCK() (uint32, error) {
	return _ContractMangataTaskManager.Contract.TASKRESPONSEWINDOWBLOCK(&_ContractMangataTaskManager.CallOpts)
}

// TASKRESPONSEWINDOWBLOCK is a free data retrieval call binding the contract method 0x1ad43189.
//
// Solidity: function TASK_RESPONSE_WINDOW_BLOCK() view returns(uint32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCallerSession) TASKRESPONSEWINDOWBLOCK() (uint32, error) {
	return _ContractMangataTaskManager.Contract.TASKRESPONSEWINDOWBLOCK(&_ContractMangataTaskManager.CallOpts)
}

// Aggregator is a free data retrieval call binding the contract method 0x245a7bfc.
//
// Solidity: function aggregator() view returns(address)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCaller) Aggregator(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractMangataTaskManager.contract.Call(opts, &out, "aggregator")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Aggregator is a free data retrieval call binding the contract method 0x245a7bfc.
//
// Solidity: function aggregator() view returns(address)
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) Aggregator() (common.Address, error) {
	return _ContractMangataTaskManager.Contract.Aggregator(&_ContractMangataTaskManager.CallOpts)
}

// Aggregator is a free data retrieval call binding the contract method 0x245a7bfc.
//
// Solidity: function aggregator() view returns(address)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCallerSession) Aggregator() (common.Address, error) {
	return _ContractMangataTaskManager.Contract.Aggregator(&_ContractMangataTaskManager.CallOpts)
}

// AllTaskHashes is a free data retrieval call binding the contract method 0x2d89f6fc.
//
// Solidity: function allTaskHashes(uint32 ) view returns(bytes32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCaller) AllTaskHashes(opts *bind.CallOpts, arg0 uint32) ([32]byte, error) {
	var out []interface{}
	err := _ContractMangataTaskManager.contract.Call(opts, &out, "allTaskHashes", arg0)

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// AllTaskHashes is a free data retrieval call binding the contract method 0x2d89f6fc.
//
// Solidity: function allTaskHashes(uint32 ) view returns(bytes32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) AllTaskHashes(arg0 uint32) ([32]byte, error) {
	return _ContractMangataTaskManager.Contract.AllTaskHashes(&_ContractMangataTaskManager.CallOpts, arg0)
}

// AllTaskHashes is a free data retrieval call binding the contract method 0x2d89f6fc.
//
// Solidity: function allTaskHashes(uint32 ) view returns(bytes32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCallerSession) AllTaskHashes(arg0 uint32) ([32]byte, error) {
	return _ContractMangataTaskManager.Contract.AllTaskHashes(&_ContractMangataTaskManager.CallOpts, arg0)
}

// AllTaskResponses is a free data retrieval call binding the contract method 0x2cb223d5.
//
// Solidity: function allTaskResponses(uint32 ) view returns(bytes32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCaller) AllTaskResponses(opts *bind.CallOpts, arg0 uint32) ([32]byte, error) {
	var out []interface{}
	err := _ContractMangataTaskManager.contract.Call(opts, &out, "allTaskResponses", arg0)

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// AllTaskResponses is a free data retrieval call binding the contract method 0x2cb223d5.
//
// Solidity: function allTaskResponses(uint32 ) view returns(bytes32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) AllTaskResponses(arg0 uint32) ([32]byte, error) {
	return _ContractMangataTaskManager.Contract.AllTaskResponses(&_ContractMangataTaskManager.CallOpts, arg0)
}

// AllTaskResponses is a free data retrieval call binding the contract method 0x2cb223d5.
//
// Solidity: function allTaskResponses(uint32 ) view returns(bytes32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCallerSession) AllTaskResponses(arg0 uint32) ([32]byte, error) {
	return _ContractMangataTaskManager.Contract.AllTaskResponses(&_ContractMangataTaskManager.CallOpts, arg0)
}

// BlsPubkeyRegistry is a free data retrieval call binding the contract method 0x3561deb1.
//
// Solidity: function blsPubkeyRegistry() view returns(address)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCaller) BlsPubkeyRegistry(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractMangataTaskManager.contract.Call(opts, &out, "blsPubkeyRegistry")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// BlsPubkeyRegistry is a free data retrieval call binding the contract method 0x3561deb1.
//
// Solidity: function blsPubkeyRegistry() view returns(address)
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) BlsPubkeyRegistry() (common.Address, error) {
	return _ContractMangataTaskManager.Contract.BlsPubkeyRegistry(&_ContractMangataTaskManager.CallOpts)
}

// BlsPubkeyRegistry is a free data retrieval call binding the contract method 0x3561deb1.
//
// Solidity: function blsPubkeyRegistry() view returns(address)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCallerSession) BlsPubkeyRegistry() (common.Address, error) {
	return _ContractMangataTaskManager.Contract.BlsPubkeyRegistry(&_ContractMangataTaskManager.CallOpts)
}

// CheckSignatures is a free data retrieval call binding the contract method 0x6efb4636.
//
// Solidity: function checkSignatures(bytes32 msgHash, bytes quorumNumbers, uint32 referenceBlockNumber, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) view returns((uint96[],uint96[]), bytes32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCaller) CheckSignatures(opts *bind.CallOpts, msgHash [32]byte, quorumNumbers []byte, referenceBlockNumber uint32, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (IBLSSignatureCheckerQuorumStakeTotals, [32]byte, error) {
	var out []interface{}
	err := _ContractMangataTaskManager.contract.Call(opts, &out, "checkSignatures", msgHash, quorumNumbers, referenceBlockNumber, nonSignerStakesAndSignature)

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
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) CheckSignatures(msgHash [32]byte, quorumNumbers []byte, referenceBlockNumber uint32, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (IBLSSignatureCheckerQuorumStakeTotals, [32]byte, error) {
	return _ContractMangataTaskManager.Contract.CheckSignatures(&_ContractMangataTaskManager.CallOpts, msgHash, quorumNumbers, referenceBlockNumber, nonSignerStakesAndSignature)
}

// CheckSignatures is a free data retrieval call binding the contract method 0x6efb4636.
//
// Solidity: function checkSignatures(bytes32 msgHash, bytes quorumNumbers, uint32 referenceBlockNumber, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) view returns((uint96[],uint96[]), bytes32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCallerSession) CheckSignatures(msgHash [32]byte, quorumNumbers []byte, referenceBlockNumber uint32, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (IBLSSignatureCheckerQuorumStakeTotals, [32]byte, error) {
	return _ContractMangataTaskManager.Contract.CheckSignatures(&_ContractMangataTaskManager.CallOpts, msgHash, quorumNumbers, referenceBlockNumber, nonSignerStakesAndSignature)
}

// Generator is a free data retrieval call binding the contract method 0x7afa1eed.
//
// Solidity: function generator() view returns(address)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCaller) Generator(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractMangataTaskManager.contract.Call(opts, &out, "generator")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Generator is a free data retrieval call binding the contract method 0x7afa1eed.
//
// Solidity: function generator() view returns(address)
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) Generator() (common.Address, error) {
	return _ContractMangataTaskManager.Contract.Generator(&_ContractMangataTaskManager.CallOpts)
}

// Generator is a free data retrieval call binding the contract method 0x7afa1eed.
//
// Solidity: function generator() view returns(address)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCallerSession) Generator() (common.Address, error) {
	return _ContractMangataTaskManager.Contract.Generator(&_ContractMangataTaskManager.CallOpts)
}

// GetCheckSignaturesIndices is a free data retrieval call binding the contract method 0x4f739f74.
//
// Solidity: function getCheckSignaturesIndices(address registryCoordinator, uint32 referenceBlockNumber, bytes quorumNumbers, bytes32[] nonSignerOperatorIds) view returns((uint32[],uint32[],uint32[],uint32[][]))
func (_ContractMangataTaskManager *ContractMangataTaskManagerCaller) GetCheckSignaturesIndices(opts *bind.CallOpts, registryCoordinator common.Address, referenceBlockNumber uint32, quorumNumbers []byte, nonSignerOperatorIds [][32]byte) (BLSOperatorStateRetrieverCheckSignaturesIndices, error) {
	var out []interface{}
	err := _ContractMangataTaskManager.contract.Call(opts, &out, "getCheckSignaturesIndices", registryCoordinator, referenceBlockNumber, quorumNumbers, nonSignerOperatorIds)

	if err != nil {
		return *new(BLSOperatorStateRetrieverCheckSignaturesIndices), err
	}

	out0 := *abi.ConvertType(out[0], new(BLSOperatorStateRetrieverCheckSignaturesIndices)).(*BLSOperatorStateRetrieverCheckSignaturesIndices)

	return out0, err

}

// GetCheckSignaturesIndices is a free data retrieval call binding the contract method 0x4f739f74.
//
// Solidity: function getCheckSignaturesIndices(address registryCoordinator, uint32 referenceBlockNumber, bytes quorumNumbers, bytes32[] nonSignerOperatorIds) view returns((uint32[],uint32[],uint32[],uint32[][]))
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) GetCheckSignaturesIndices(registryCoordinator common.Address, referenceBlockNumber uint32, quorumNumbers []byte, nonSignerOperatorIds [][32]byte) (BLSOperatorStateRetrieverCheckSignaturesIndices, error) {
	return _ContractMangataTaskManager.Contract.GetCheckSignaturesIndices(&_ContractMangataTaskManager.CallOpts, registryCoordinator, referenceBlockNumber, quorumNumbers, nonSignerOperatorIds)
}

// GetCheckSignaturesIndices is a free data retrieval call binding the contract method 0x4f739f74.
//
// Solidity: function getCheckSignaturesIndices(address registryCoordinator, uint32 referenceBlockNumber, bytes quorumNumbers, bytes32[] nonSignerOperatorIds) view returns((uint32[],uint32[],uint32[],uint32[][]))
func (_ContractMangataTaskManager *ContractMangataTaskManagerCallerSession) GetCheckSignaturesIndices(registryCoordinator common.Address, referenceBlockNumber uint32, quorumNumbers []byte, nonSignerOperatorIds [][32]byte) (BLSOperatorStateRetrieverCheckSignaturesIndices, error) {
	return _ContractMangataTaskManager.Contract.GetCheckSignaturesIndices(&_ContractMangataTaskManager.CallOpts, registryCoordinator, referenceBlockNumber, quorumNumbers, nonSignerOperatorIds)
}

// GetOperatorState is a free data retrieval call binding the contract method 0x3563b0d1.
//
// Solidity: function getOperatorState(address registryCoordinator, bytes quorumNumbers, uint32 blockNumber) view returns((bytes32,uint96)[][])
func (_ContractMangataTaskManager *ContractMangataTaskManagerCaller) GetOperatorState(opts *bind.CallOpts, registryCoordinator common.Address, quorumNumbers []byte, blockNumber uint32) ([][]BLSOperatorStateRetrieverOperator, error) {
	var out []interface{}
	err := _ContractMangataTaskManager.contract.Call(opts, &out, "getOperatorState", registryCoordinator, quorumNumbers, blockNumber)

	if err != nil {
		return *new([][]BLSOperatorStateRetrieverOperator), err
	}

	out0 := *abi.ConvertType(out[0], new([][]BLSOperatorStateRetrieverOperator)).(*[][]BLSOperatorStateRetrieverOperator)

	return out0, err

}

// GetOperatorState is a free data retrieval call binding the contract method 0x3563b0d1.
//
// Solidity: function getOperatorState(address registryCoordinator, bytes quorumNumbers, uint32 blockNumber) view returns((bytes32,uint96)[][])
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) GetOperatorState(registryCoordinator common.Address, quorumNumbers []byte, blockNumber uint32) ([][]BLSOperatorStateRetrieverOperator, error) {
	return _ContractMangataTaskManager.Contract.GetOperatorState(&_ContractMangataTaskManager.CallOpts, registryCoordinator, quorumNumbers, blockNumber)
}

// GetOperatorState is a free data retrieval call binding the contract method 0x3563b0d1.
//
// Solidity: function getOperatorState(address registryCoordinator, bytes quorumNumbers, uint32 blockNumber) view returns((bytes32,uint96)[][])
func (_ContractMangataTaskManager *ContractMangataTaskManagerCallerSession) GetOperatorState(registryCoordinator common.Address, quorumNumbers []byte, blockNumber uint32) ([][]BLSOperatorStateRetrieverOperator, error) {
	return _ContractMangataTaskManager.Contract.GetOperatorState(&_ContractMangataTaskManager.CallOpts, registryCoordinator, quorumNumbers, blockNumber)
}

// GetOperatorState0 is a free data retrieval call binding the contract method 0xcefdc1d4.
//
// Solidity: function getOperatorState(address registryCoordinator, bytes32 operatorId, uint32 blockNumber) view returns(uint256, (bytes32,uint96)[][])
func (_ContractMangataTaskManager *ContractMangataTaskManagerCaller) GetOperatorState0(opts *bind.CallOpts, registryCoordinator common.Address, operatorId [32]byte, blockNumber uint32) (*big.Int, [][]BLSOperatorStateRetrieverOperator, error) {
	var out []interface{}
	err := _ContractMangataTaskManager.contract.Call(opts, &out, "getOperatorState0", registryCoordinator, operatorId, blockNumber)

	if err != nil {
		return *new(*big.Int), *new([][]BLSOperatorStateRetrieverOperator), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)
	out1 := *abi.ConvertType(out[1], new([][]BLSOperatorStateRetrieverOperator)).(*[][]BLSOperatorStateRetrieverOperator)

	return out0, out1, err

}

// GetOperatorState0 is a free data retrieval call binding the contract method 0xcefdc1d4.
//
// Solidity: function getOperatorState(address registryCoordinator, bytes32 operatorId, uint32 blockNumber) view returns(uint256, (bytes32,uint96)[][])
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) GetOperatorState0(registryCoordinator common.Address, operatorId [32]byte, blockNumber uint32) (*big.Int, [][]BLSOperatorStateRetrieverOperator, error) {
	return _ContractMangataTaskManager.Contract.GetOperatorState0(&_ContractMangataTaskManager.CallOpts, registryCoordinator, operatorId, blockNumber)
}

// GetOperatorState0 is a free data retrieval call binding the contract method 0xcefdc1d4.
//
// Solidity: function getOperatorState(address registryCoordinator, bytes32 operatorId, uint32 blockNumber) view returns(uint256, (bytes32,uint96)[][])
func (_ContractMangataTaskManager *ContractMangataTaskManagerCallerSession) GetOperatorState0(registryCoordinator common.Address, operatorId [32]byte, blockNumber uint32) (*big.Int, [][]BLSOperatorStateRetrieverOperator, error) {
	return _ContractMangataTaskManager.Contract.GetOperatorState0(&_ContractMangataTaskManager.CallOpts, registryCoordinator, operatorId, blockNumber)
}

// GetTaskChallangeWindowBlock is a free data retrieval call binding the contract method 0xe3b5ee04.
//
// Solidity: function getTaskChallangeWindowBlock() view returns(uint32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCaller) GetTaskChallangeWindowBlock(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractMangataTaskManager.contract.Call(opts, &out, "getTaskChallangeWindowBlock")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// GetTaskChallangeWindowBlock is a free data retrieval call binding the contract method 0xe3b5ee04.
//
// Solidity: function getTaskChallangeWindowBlock() view returns(uint32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) GetTaskChallangeWindowBlock() (uint32, error) {
	return _ContractMangataTaskManager.Contract.GetTaskChallangeWindowBlock(&_ContractMangataTaskManager.CallOpts)
}

// GetTaskChallangeWindowBlock is a free data retrieval call binding the contract method 0xe3b5ee04.
//
// Solidity: function getTaskChallangeWindowBlock() view returns(uint32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCallerSession) GetTaskChallangeWindowBlock() (uint32, error) {
	return _ContractMangataTaskManager.Contract.GetTaskChallangeWindowBlock(&_ContractMangataTaskManager.CallOpts)
}

// GetTaskResponseWindowBlock is a free data retrieval call binding the contract method 0xf5c9899d.
//
// Solidity: function getTaskResponseWindowBlock() view returns(uint32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCaller) GetTaskResponseWindowBlock(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractMangataTaskManager.contract.Call(opts, &out, "getTaskResponseWindowBlock")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// GetTaskResponseWindowBlock is a free data retrieval call binding the contract method 0xf5c9899d.
//
// Solidity: function getTaskResponseWindowBlock() view returns(uint32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) GetTaskResponseWindowBlock() (uint32, error) {
	return _ContractMangataTaskManager.Contract.GetTaskResponseWindowBlock(&_ContractMangataTaskManager.CallOpts)
}

// GetTaskResponseWindowBlock is a free data retrieval call binding the contract method 0xf5c9899d.
//
// Solidity: function getTaskResponseWindowBlock() view returns(uint32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCallerSession) GetTaskResponseWindowBlock() (uint32, error) {
	return _ContractMangataTaskManager.Contract.GetTaskResponseWindowBlock(&_ContractMangataTaskManager.CallOpts)
}

// LatestTaskNum is a free data retrieval call binding the contract method 0x8b00ce7c.
//
// Solidity: function latestTaskNum() view returns(uint32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCaller) LatestTaskNum(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractMangataTaskManager.contract.Call(opts, &out, "latestTaskNum")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// LatestTaskNum is a free data retrieval call binding the contract method 0x8b00ce7c.
//
// Solidity: function latestTaskNum() view returns(uint32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) LatestTaskNum() (uint32, error) {
	return _ContractMangataTaskManager.Contract.LatestTaskNum(&_ContractMangataTaskManager.CallOpts)
}

// LatestTaskNum is a free data retrieval call binding the contract method 0x8b00ce7c.
//
// Solidity: function latestTaskNum() view returns(uint32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCallerSession) LatestTaskNum() (uint32, error) {
	return _ContractMangataTaskManager.Contract.LatestTaskNum(&_ContractMangataTaskManager.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCaller) Owner(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractMangataTaskManager.contract.Call(opts, &out, "owner")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) Owner() (common.Address, error) {
	return _ContractMangataTaskManager.Contract.Owner(&_ContractMangataTaskManager.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCallerSession) Owner() (common.Address, error) {
	return _ContractMangataTaskManager.Contract.Owner(&_ContractMangataTaskManager.CallOpts)
}

// Paused is a free data retrieval call binding the contract method 0x5ac86ab7.
//
// Solidity: function paused(uint8 index) view returns(bool)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCaller) Paused(opts *bind.CallOpts, index uint8) (bool, error) {
	var out []interface{}
	err := _ContractMangataTaskManager.contract.Call(opts, &out, "paused", index)

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// Paused is a free data retrieval call binding the contract method 0x5ac86ab7.
//
// Solidity: function paused(uint8 index) view returns(bool)
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) Paused(index uint8) (bool, error) {
	return _ContractMangataTaskManager.Contract.Paused(&_ContractMangataTaskManager.CallOpts, index)
}

// Paused is a free data retrieval call binding the contract method 0x5ac86ab7.
//
// Solidity: function paused(uint8 index) view returns(bool)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCallerSession) Paused(index uint8) (bool, error) {
	return _ContractMangataTaskManager.Contract.Paused(&_ContractMangataTaskManager.CallOpts, index)
}

// Paused0 is a free data retrieval call binding the contract method 0x5c975abb.
//
// Solidity: function paused() view returns(uint256)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCaller) Paused0(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _ContractMangataTaskManager.contract.Call(opts, &out, "paused0")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// Paused0 is a free data retrieval call binding the contract method 0x5c975abb.
//
// Solidity: function paused() view returns(uint256)
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) Paused0() (*big.Int, error) {
	return _ContractMangataTaskManager.Contract.Paused0(&_ContractMangataTaskManager.CallOpts)
}

// Paused0 is a free data retrieval call binding the contract method 0x5c975abb.
//
// Solidity: function paused() view returns(uint256)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCallerSession) Paused0() (*big.Int, error) {
	return _ContractMangataTaskManager.Contract.Paused0(&_ContractMangataTaskManager.CallOpts)
}

// PauserRegistry is a free data retrieval call binding the contract method 0x886f1195.
//
// Solidity: function pauserRegistry() view returns(address)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCaller) PauserRegistry(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractMangataTaskManager.contract.Call(opts, &out, "pauserRegistry")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// PauserRegistry is a free data retrieval call binding the contract method 0x886f1195.
//
// Solidity: function pauserRegistry() view returns(address)
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) PauserRegistry() (common.Address, error) {
	return _ContractMangataTaskManager.Contract.PauserRegistry(&_ContractMangataTaskManager.CallOpts)
}

// PauserRegistry is a free data retrieval call binding the contract method 0x886f1195.
//
// Solidity: function pauserRegistry() view returns(address)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCallerSession) PauserRegistry() (common.Address, error) {
	return _ContractMangataTaskManager.Contract.PauserRegistry(&_ContractMangataTaskManager.CallOpts)
}

// RegistryCoordinator is a free data retrieval call binding the contract method 0x6d14a987.
//
// Solidity: function registryCoordinator() view returns(address)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCaller) RegistryCoordinator(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractMangataTaskManager.contract.Call(opts, &out, "registryCoordinator")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// RegistryCoordinator is a free data retrieval call binding the contract method 0x6d14a987.
//
// Solidity: function registryCoordinator() view returns(address)
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) RegistryCoordinator() (common.Address, error) {
	return _ContractMangataTaskManager.Contract.RegistryCoordinator(&_ContractMangataTaskManager.CallOpts)
}

// RegistryCoordinator is a free data retrieval call binding the contract method 0x6d14a987.
//
// Solidity: function registryCoordinator() view returns(address)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCallerSession) RegistryCoordinator() (common.Address, error) {
	return _ContractMangataTaskManager.Contract.RegistryCoordinator(&_ContractMangataTaskManager.CallOpts)
}

// StakeRegistry is a free data retrieval call binding the contract method 0x68304835.
//
// Solidity: function stakeRegistry() view returns(address)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCaller) StakeRegistry(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractMangataTaskManager.contract.Call(opts, &out, "stakeRegistry")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// StakeRegistry is a free data retrieval call binding the contract method 0x68304835.
//
// Solidity: function stakeRegistry() view returns(address)
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) StakeRegistry() (common.Address, error) {
	return _ContractMangataTaskManager.Contract.StakeRegistry(&_ContractMangataTaskManager.CallOpts)
}

// StakeRegistry is a free data retrieval call binding the contract method 0x68304835.
//
// Solidity: function stakeRegistry() view returns(address)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCallerSession) StakeRegistry() (common.Address, error) {
	return _ContractMangataTaskManager.Contract.StakeRegistry(&_ContractMangataTaskManager.CallOpts)
}

// TaskNumber is a free data retrieval call binding the contract method 0x72d18e8d.
//
// Solidity: function taskNumber() view returns(uint32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCaller) TaskNumber(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractMangataTaskManager.contract.Call(opts, &out, "taskNumber")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// TaskNumber is a free data retrieval call binding the contract method 0x72d18e8d.
//
// Solidity: function taskNumber() view returns(uint32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) TaskNumber() (uint32, error) {
	return _ContractMangataTaskManager.Contract.TaskNumber(&_ContractMangataTaskManager.CallOpts)
}

// TaskNumber is a free data retrieval call binding the contract method 0x72d18e8d.
//
// Solidity: function taskNumber() view returns(uint32)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCallerSession) TaskNumber() (uint32, error) {
	return _ContractMangataTaskManager.Contract.TaskNumber(&_ContractMangataTaskManager.CallOpts)
}

// TaskSuccesfullyChallenged is a free data retrieval call binding the contract method 0x5decc3f5.
//
// Solidity: function taskSuccesfullyChallenged(uint32 ) view returns(bool)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCaller) TaskSuccesfullyChallenged(opts *bind.CallOpts, arg0 uint32) (bool, error) {
	var out []interface{}
	err := _ContractMangataTaskManager.contract.Call(opts, &out, "taskSuccesfullyChallenged", arg0)

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// TaskSuccesfullyChallenged is a free data retrieval call binding the contract method 0x5decc3f5.
//
// Solidity: function taskSuccesfullyChallenged(uint32 ) view returns(bool)
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) TaskSuccesfullyChallenged(arg0 uint32) (bool, error) {
	return _ContractMangataTaskManager.Contract.TaskSuccesfullyChallenged(&_ContractMangataTaskManager.CallOpts, arg0)
}

// TaskSuccesfullyChallenged is a free data retrieval call binding the contract method 0x5decc3f5.
//
// Solidity: function taskSuccesfullyChallenged(uint32 ) view returns(bool)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCallerSession) TaskSuccesfullyChallenged(arg0 uint32) (bool, error) {
	return _ContractMangataTaskManager.Contract.TaskSuccesfullyChallenged(&_ContractMangataTaskManager.CallOpts, arg0)
}

// TrySignatureAndApkVerification is a free data retrieval call binding the contract method 0x171f1d5b.
//
// Solidity: function trySignatureAndApkVerification(bytes32 msgHash, (uint256,uint256) apk, (uint256[2],uint256[2]) apkG2, (uint256,uint256) sigma) view returns(bool pairingSuccessful, bool siganatureIsValid)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCaller) TrySignatureAndApkVerification(opts *bind.CallOpts, msgHash [32]byte, apk BN254G1Point, apkG2 BN254G2Point, sigma BN254G1Point) (struct {
	PairingSuccessful bool
	SiganatureIsValid bool
}, error) {
	var out []interface{}
	err := _ContractMangataTaskManager.contract.Call(opts, &out, "trySignatureAndApkVerification", msgHash, apk, apkG2, sigma)

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
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) TrySignatureAndApkVerification(msgHash [32]byte, apk BN254G1Point, apkG2 BN254G2Point, sigma BN254G1Point) (struct {
	PairingSuccessful bool
	SiganatureIsValid bool
}, error) {
	return _ContractMangataTaskManager.Contract.TrySignatureAndApkVerification(&_ContractMangataTaskManager.CallOpts, msgHash, apk, apkG2, sigma)
}

// TrySignatureAndApkVerification is a free data retrieval call binding the contract method 0x171f1d5b.
//
// Solidity: function trySignatureAndApkVerification(bytes32 msgHash, (uint256,uint256) apk, (uint256[2],uint256[2]) apkG2, (uint256,uint256) sigma) view returns(bool pairingSuccessful, bool siganatureIsValid)
func (_ContractMangataTaskManager *ContractMangataTaskManagerCallerSession) TrySignatureAndApkVerification(msgHash [32]byte, apk BN254G1Point, apkG2 BN254G2Point, sigma BN254G1Point) (struct {
	PairingSuccessful bool
	SiganatureIsValid bool
}, error) {
	return _ContractMangataTaskManager.Contract.TrySignatureAndApkVerification(&_ContractMangataTaskManager.CallOpts, msgHash, apk, apkG2, sigma)
}

// CreateNewTask is a paid mutator transaction binding the contract method 0x6b92787e.
//
// Solidity: function createNewTask(uint256 blockNumber, uint32 quorumThresholdPercentage, bytes quorumNumbers) returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerTransactor) CreateNewTask(opts *bind.TransactOpts, blockNumber *big.Int, quorumThresholdPercentage uint32, quorumNumbers []byte) (*types.Transaction, error) {
	return _ContractMangataTaskManager.contract.Transact(opts, "createNewTask", blockNumber, quorumThresholdPercentage, quorumNumbers)
}

// CreateNewTask is a paid mutator transaction binding the contract method 0x6b92787e.
//
// Solidity: function createNewTask(uint256 blockNumber, uint32 quorumThresholdPercentage, bytes quorumNumbers) returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) CreateNewTask(blockNumber *big.Int, quorumThresholdPercentage uint32, quorumNumbers []byte) (*types.Transaction, error) {
	return _ContractMangataTaskManager.Contract.CreateNewTask(&_ContractMangataTaskManager.TransactOpts, blockNumber, quorumThresholdPercentage, quorumNumbers)
}

// CreateNewTask is a paid mutator transaction binding the contract method 0x6b92787e.
//
// Solidity: function createNewTask(uint256 blockNumber, uint32 quorumThresholdPercentage, bytes quorumNumbers) returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerTransactorSession) CreateNewTask(blockNumber *big.Int, quorumThresholdPercentage uint32, quorumNumbers []byte) (*types.Transaction, error) {
	return _ContractMangataTaskManager.Contract.CreateNewTask(&_ContractMangataTaskManager.TransactOpts, blockNumber, quorumThresholdPercentage, quorumNumbers)
}

// Initialize is a paid mutator transaction binding the contract method 0xf8c8765e.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _aggregator, address _generator) returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerTransactor) Initialize(opts *bind.TransactOpts, _pauserRegistry common.Address, initialOwner common.Address, _aggregator common.Address, _generator common.Address) (*types.Transaction, error) {
	return _ContractMangataTaskManager.contract.Transact(opts, "initialize", _pauserRegistry, initialOwner, _aggregator, _generator)
}

// Initialize is a paid mutator transaction binding the contract method 0xf8c8765e.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _aggregator, address _generator) returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) Initialize(_pauserRegistry common.Address, initialOwner common.Address, _aggregator common.Address, _generator common.Address) (*types.Transaction, error) {
	return _ContractMangataTaskManager.Contract.Initialize(&_ContractMangataTaskManager.TransactOpts, _pauserRegistry, initialOwner, _aggregator, _generator)
}

// Initialize is a paid mutator transaction binding the contract method 0xf8c8765e.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _aggregator, address _generator) returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerTransactorSession) Initialize(_pauserRegistry common.Address, initialOwner common.Address, _aggregator common.Address, _generator common.Address) (*types.Transaction, error) {
	return _ContractMangataTaskManager.Contract.Initialize(&_ContractMangataTaskManager.TransactOpts, _pauserRegistry, initialOwner, _aggregator, _generator)
}

// Pause is a paid mutator transaction binding the contract method 0x136439dd.
//
// Solidity: function pause(uint256 newPausedStatus) returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerTransactor) Pause(opts *bind.TransactOpts, newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractMangataTaskManager.contract.Transact(opts, "pause", newPausedStatus)
}

// Pause is a paid mutator transaction binding the contract method 0x136439dd.
//
// Solidity: function pause(uint256 newPausedStatus) returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) Pause(newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractMangataTaskManager.Contract.Pause(&_ContractMangataTaskManager.TransactOpts, newPausedStatus)
}

// Pause is a paid mutator transaction binding the contract method 0x136439dd.
//
// Solidity: function pause(uint256 newPausedStatus) returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerTransactorSession) Pause(newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractMangataTaskManager.Contract.Pause(&_ContractMangataTaskManager.TransactOpts, newPausedStatus)
}

// PauseAll is a paid mutator transaction binding the contract method 0x595c6a67.
//
// Solidity: function pauseAll() returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerTransactor) PauseAll(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractMangataTaskManager.contract.Transact(opts, "pauseAll")
}

// PauseAll is a paid mutator transaction binding the contract method 0x595c6a67.
//
// Solidity: function pauseAll() returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) PauseAll() (*types.Transaction, error) {
	return _ContractMangataTaskManager.Contract.PauseAll(&_ContractMangataTaskManager.TransactOpts)
}

// PauseAll is a paid mutator transaction binding the contract method 0x595c6a67.
//
// Solidity: function pauseAll() returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerTransactorSession) PauseAll() (*types.Transaction, error) {
	return _ContractMangataTaskManager.Contract.PauseAll(&_ContractMangataTaskManager.TransactOpts)
}

// RaiseAndResolveChallenge is a paid mutator transaction binding the contract method 0x3cd27f64.
//
// Solidity: function raiseAndResolveChallenge((uint256,uint32,bytes,uint32) task, (uint32,bytes32) taskResponse, (uint32,bytes32) taskResponseMetadata, (uint256,uint256)[] pubkeysOfNonSigningOperators) returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerTransactor) RaiseAndResolveChallenge(opts *bind.TransactOpts, task IMangataTaskManagerTask, taskResponse IMangataTaskManagerTaskResponse, taskResponseMetadata IMangataTaskManagerTaskResponseMetadata, pubkeysOfNonSigningOperators []BN254G1Point) (*types.Transaction, error) {
	return _ContractMangataTaskManager.contract.Transact(opts, "raiseAndResolveChallenge", task, taskResponse, taskResponseMetadata, pubkeysOfNonSigningOperators)
}

// RaiseAndResolveChallenge is a paid mutator transaction binding the contract method 0x3cd27f64.
//
// Solidity: function raiseAndResolveChallenge((uint256,uint32,bytes,uint32) task, (uint32,bytes32) taskResponse, (uint32,bytes32) taskResponseMetadata, (uint256,uint256)[] pubkeysOfNonSigningOperators) returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) RaiseAndResolveChallenge(task IMangataTaskManagerTask, taskResponse IMangataTaskManagerTaskResponse, taskResponseMetadata IMangataTaskManagerTaskResponseMetadata, pubkeysOfNonSigningOperators []BN254G1Point) (*types.Transaction, error) {
	return _ContractMangataTaskManager.Contract.RaiseAndResolveChallenge(&_ContractMangataTaskManager.TransactOpts, task, taskResponse, taskResponseMetadata, pubkeysOfNonSigningOperators)
}

// RaiseAndResolveChallenge is a paid mutator transaction binding the contract method 0x3cd27f64.
//
// Solidity: function raiseAndResolveChallenge((uint256,uint32,bytes,uint32) task, (uint32,bytes32) taskResponse, (uint32,bytes32) taskResponseMetadata, (uint256,uint256)[] pubkeysOfNonSigningOperators) returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerTransactorSession) RaiseAndResolveChallenge(task IMangataTaskManagerTask, taskResponse IMangataTaskManagerTaskResponse, taskResponseMetadata IMangataTaskManagerTaskResponseMetadata, pubkeysOfNonSigningOperators []BN254G1Point) (*types.Transaction, error) {
	return _ContractMangataTaskManager.Contract.RaiseAndResolveChallenge(&_ContractMangataTaskManager.TransactOpts, task, taskResponse, taskResponseMetadata, pubkeysOfNonSigningOperators)
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerTransactor) RenounceOwnership(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractMangataTaskManager.contract.Transact(opts, "renounceOwnership")
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) RenounceOwnership() (*types.Transaction, error) {
	return _ContractMangataTaskManager.Contract.RenounceOwnership(&_ContractMangataTaskManager.TransactOpts)
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerTransactorSession) RenounceOwnership() (*types.Transaction, error) {
	return _ContractMangataTaskManager.Contract.RenounceOwnership(&_ContractMangataTaskManager.TransactOpts)
}

// RespondToTask is a paid mutator transaction binding the contract method 0x277cb995.
//
// Solidity: function respondToTask((uint256,uint32,bytes,uint32) task, (uint32,bytes32) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerTransactor) RespondToTask(opts *bind.TransactOpts, task IMangataTaskManagerTask, taskResponse IMangataTaskManagerTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractMangataTaskManager.contract.Transact(opts, "respondToTask", task, taskResponse, nonSignerStakesAndSignature)
}

// RespondToTask is a paid mutator transaction binding the contract method 0x277cb995.
//
// Solidity: function respondToTask((uint256,uint32,bytes,uint32) task, (uint32,bytes32) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) RespondToTask(task IMangataTaskManagerTask, taskResponse IMangataTaskManagerTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractMangataTaskManager.Contract.RespondToTask(&_ContractMangataTaskManager.TransactOpts, task, taskResponse, nonSignerStakesAndSignature)
}

// RespondToTask is a paid mutator transaction binding the contract method 0x277cb995.
//
// Solidity: function respondToTask((uint256,uint32,bytes,uint32) task, (uint32,bytes32) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerTransactorSession) RespondToTask(task IMangataTaskManagerTask, taskResponse IMangataTaskManagerTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractMangataTaskManager.Contract.RespondToTask(&_ContractMangataTaskManager.TransactOpts, task, taskResponse, nonSignerStakesAndSignature)
}

// SetPauserRegistry is a paid mutator transaction binding the contract method 0x10d67a2f.
//
// Solidity: function setPauserRegistry(address newPauserRegistry) returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerTransactor) SetPauserRegistry(opts *bind.TransactOpts, newPauserRegistry common.Address) (*types.Transaction, error) {
	return _ContractMangataTaskManager.contract.Transact(opts, "setPauserRegistry", newPauserRegistry)
}

// SetPauserRegistry is a paid mutator transaction binding the contract method 0x10d67a2f.
//
// Solidity: function setPauserRegistry(address newPauserRegistry) returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) SetPauserRegistry(newPauserRegistry common.Address) (*types.Transaction, error) {
	return _ContractMangataTaskManager.Contract.SetPauserRegistry(&_ContractMangataTaskManager.TransactOpts, newPauserRegistry)
}

// SetPauserRegistry is a paid mutator transaction binding the contract method 0x10d67a2f.
//
// Solidity: function setPauserRegistry(address newPauserRegistry) returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerTransactorSession) SetPauserRegistry(newPauserRegistry common.Address) (*types.Transaction, error) {
	return _ContractMangataTaskManager.Contract.SetPauserRegistry(&_ContractMangataTaskManager.TransactOpts, newPauserRegistry)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerTransactor) TransferOwnership(opts *bind.TransactOpts, newOwner common.Address) (*types.Transaction, error) {
	return _ContractMangataTaskManager.contract.Transact(opts, "transferOwnership", newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) TransferOwnership(newOwner common.Address) (*types.Transaction, error) {
	return _ContractMangataTaskManager.Contract.TransferOwnership(&_ContractMangataTaskManager.TransactOpts, newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerTransactorSession) TransferOwnership(newOwner common.Address) (*types.Transaction, error) {
	return _ContractMangataTaskManager.Contract.TransferOwnership(&_ContractMangataTaskManager.TransactOpts, newOwner)
}

// Unpause is a paid mutator transaction binding the contract method 0xfabc1cbc.
//
// Solidity: function unpause(uint256 newPausedStatus) returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerTransactor) Unpause(opts *bind.TransactOpts, newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractMangataTaskManager.contract.Transact(opts, "unpause", newPausedStatus)
}

// Unpause is a paid mutator transaction binding the contract method 0xfabc1cbc.
//
// Solidity: function unpause(uint256 newPausedStatus) returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerSession) Unpause(newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractMangataTaskManager.Contract.Unpause(&_ContractMangataTaskManager.TransactOpts, newPausedStatus)
}

// Unpause is a paid mutator transaction binding the contract method 0xfabc1cbc.
//
// Solidity: function unpause(uint256 newPausedStatus) returns()
func (_ContractMangataTaskManager *ContractMangataTaskManagerTransactorSession) Unpause(newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractMangataTaskManager.Contract.Unpause(&_ContractMangataTaskManager.TransactOpts, newPausedStatus)
}

// ContractMangataTaskManagerInitializedIterator is returned from FilterInitialized and is used to iterate over the raw logs and unpacked data for Initialized events raised by the ContractMangataTaskManager contract.
type ContractMangataTaskManagerInitializedIterator struct {
	Event *ContractMangataTaskManagerInitialized // Event containing the contract specifics and raw log

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
func (it *ContractMangataTaskManagerInitializedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractMangataTaskManagerInitialized)
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
		it.Event = new(ContractMangataTaskManagerInitialized)
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
func (it *ContractMangataTaskManagerInitializedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractMangataTaskManagerInitializedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractMangataTaskManagerInitialized represents a Initialized event raised by the ContractMangataTaskManager contract.
type ContractMangataTaskManagerInitialized struct {
	Version uint8
	Raw     types.Log // Blockchain specific contextual infos
}

// FilterInitialized is a free log retrieval operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) FilterInitialized(opts *bind.FilterOpts) (*ContractMangataTaskManagerInitializedIterator, error) {

	logs, sub, err := _ContractMangataTaskManager.contract.FilterLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return &ContractMangataTaskManagerInitializedIterator{contract: _ContractMangataTaskManager.contract, event: "Initialized", logs: logs, sub: sub}, nil
}

// WatchInitialized is a free log subscription operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) WatchInitialized(opts *bind.WatchOpts, sink chan<- *ContractMangataTaskManagerInitialized) (event.Subscription, error) {

	logs, sub, err := _ContractMangataTaskManager.contract.WatchLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractMangataTaskManagerInitialized)
				if err := _ContractMangataTaskManager.contract.UnpackLog(event, "Initialized", log); err != nil {
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
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) ParseInitialized(log types.Log) (*ContractMangataTaskManagerInitialized, error) {
	event := new(ContractMangataTaskManagerInitialized)
	if err := _ContractMangataTaskManager.contract.UnpackLog(event, "Initialized", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractMangataTaskManagerNewTaskCreatedIterator is returned from FilterNewTaskCreated and is used to iterate over the raw logs and unpacked data for NewTaskCreated events raised by the ContractMangataTaskManager contract.
type ContractMangataTaskManagerNewTaskCreatedIterator struct {
	Event *ContractMangataTaskManagerNewTaskCreated // Event containing the contract specifics and raw log

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
func (it *ContractMangataTaskManagerNewTaskCreatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractMangataTaskManagerNewTaskCreated)
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
		it.Event = new(ContractMangataTaskManagerNewTaskCreated)
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
func (it *ContractMangataTaskManagerNewTaskCreatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractMangataTaskManagerNewTaskCreatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractMangataTaskManagerNewTaskCreated represents a NewTaskCreated event raised by the ContractMangataTaskManager contract.
type ContractMangataTaskManagerNewTaskCreated struct {
	TaskIndex uint32
	Task      IMangataTaskManagerTask
	Raw       types.Log // Blockchain specific contextual infos
}

// FilterNewTaskCreated is a free log retrieval operation binding the contract event 0x1695b8d06ec800b4615e745cfb5bd00c1f2875615d42925c3b5afa543bb24c48.
//
// Solidity: event NewTaskCreated(uint32 indexed taskIndex, (uint256,uint32,bytes,uint32) task)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) FilterNewTaskCreated(opts *bind.FilterOpts, taskIndex []uint32) (*ContractMangataTaskManagerNewTaskCreatedIterator, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractMangataTaskManager.contract.FilterLogs(opts, "NewTaskCreated", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return &ContractMangataTaskManagerNewTaskCreatedIterator{contract: _ContractMangataTaskManager.contract, event: "NewTaskCreated", logs: logs, sub: sub}, nil
}

// WatchNewTaskCreated is a free log subscription operation binding the contract event 0x1695b8d06ec800b4615e745cfb5bd00c1f2875615d42925c3b5afa543bb24c48.
//
// Solidity: event NewTaskCreated(uint32 indexed taskIndex, (uint256,uint32,bytes,uint32) task)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) WatchNewTaskCreated(opts *bind.WatchOpts, sink chan<- *ContractMangataTaskManagerNewTaskCreated, taskIndex []uint32) (event.Subscription, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractMangataTaskManager.contract.WatchLogs(opts, "NewTaskCreated", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractMangataTaskManagerNewTaskCreated)
				if err := _ContractMangataTaskManager.contract.UnpackLog(event, "NewTaskCreated", log); err != nil {
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

// ParseNewTaskCreated is a log parse operation binding the contract event 0x1695b8d06ec800b4615e745cfb5bd00c1f2875615d42925c3b5afa543bb24c48.
//
// Solidity: event NewTaskCreated(uint32 indexed taskIndex, (uint256,uint32,bytes,uint32) task)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) ParseNewTaskCreated(log types.Log) (*ContractMangataTaskManagerNewTaskCreated, error) {
	event := new(ContractMangataTaskManagerNewTaskCreated)
	if err := _ContractMangataTaskManager.contract.UnpackLog(event, "NewTaskCreated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractMangataTaskManagerOwnershipTransferredIterator is returned from FilterOwnershipTransferred and is used to iterate over the raw logs and unpacked data for OwnershipTransferred events raised by the ContractMangataTaskManager contract.
type ContractMangataTaskManagerOwnershipTransferredIterator struct {
	Event *ContractMangataTaskManagerOwnershipTransferred // Event containing the contract specifics and raw log

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
func (it *ContractMangataTaskManagerOwnershipTransferredIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractMangataTaskManagerOwnershipTransferred)
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
		it.Event = new(ContractMangataTaskManagerOwnershipTransferred)
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
func (it *ContractMangataTaskManagerOwnershipTransferredIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractMangataTaskManagerOwnershipTransferredIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractMangataTaskManagerOwnershipTransferred represents a OwnershipTransferred event raised by the ContractMangataTaskManager contract.
type ContractMangataTaskManagerOwnershipTransferred struct {
	PreviousOwner common.Address
	NewOwner      common.Address
	Raw           types.Log // Blockchain specific contextual infos
}

// FilterOwnershipTransferred is a free log retrieval operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) FilterOwnershipTransferred(opts *bind.FilterOpts, previousOwner []common.Address, newOwner []common.Address) (*ContractMangataTaskManagerOwnershipTransferredIterator, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _ContractMangataTaskManager.contract.FilterLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return &ContractMangataTaskManagerOwnershipTransferredIterator{contract: _ContractMangataTaskManager.contract, event: "OwnershipTransferred", logs: logs, sub: sub}, nil
}

// WatchOwnershipTransferred is a free log subscription operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) WatchOwnershipTransferred(opts *bind.WatchOpts, sink chan<- *ContractMangataTaskManagerOwnershipTransferred, previousOwner []common.Address, newOwner []common.Address) (event.Subscription, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _ContractMangataTaskManager.contract.WatchLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractMangataTaskManagerOwnershipTransferred)
				if err := _ContractMangataTaskManager.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
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
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) ParseOwnershipTransferred(log types.Log) (*ContractMangataTaskManagerOwnershipTransferred, error) {
	event := new(ContractMangataTaskManagerOwnershipTransferred)
	if err := _ContractMangataTaskManager.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractMangataTaskManagerPausedIterator is returned from FilterPaused and is used to iterate over the raw logs and unpacked data for Paused events raised by the ContractMangataTaskManager contract.
type ContractMangataTaskManagerPausedIterator struct {
	Event *ContractMangataTaskManagerPaused // Event containing the contract specifics and raw log

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
func (it *ContractMangataTaskManagerPausedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractMangataTaskManagerPaused)
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
		it.Event = new(ContractMangataTaskManagerPaused)
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
func (it *ContractMangataTaskManagerPausedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractMangataTaskManagerPausedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractMangataTaskManagerPaused represents a Paused event raised by the ContractMangataTaskManager contract.
type ContractMangataTaskManagerPaused struct {
	Account         common.Address
	NewPausedStatus *big.Int
	Raw             types.Log // Blockchain specific contextual infos
}

// FilterPaused is a free log retrieval operation binding the contract event 0xab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d.
//
// Solidity: event Paused(address indexed account, uint256 newPausedStatus)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) FilterPaused(opts *bind.FilterOpts, account []common.Address) (*ContractMangataTaskManagerPausedIterator, error) {

	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}

	logs, sub, err := _ContractMangataTaskManager.contract.FilterLogs(opts, "Paused", accountRule)
	if err != nil {
		return nil, err
	}
	return &ContractMangataTaskManagerPausedIterator{contract: _ContractMangataTaskManager.contract, event: "Paused", logs: logs, sub: sub}, nil
}

// WatchPaused is a free log subscription operation binding the contract event 0xab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d.
//
// Solidity: event Paused(address indexed account, uint256 newPausedStatus)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) WatchPaused(opts *bind.WatchOpts, sink chan<- *ContractMangataTaskManagerPaused, account []common.Address) (event.Subscription, error) {

	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}

	logs, sub, err := _ContractMangataTaskManager.contract.WatchLogs(opts, "Paused", accountRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractMangataTaskManagerPaused)
				if err := _ContractMangataTaskManager.contract.UnpackLog(event, "Paused", log); err != nil {
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
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) ParsePaused(log types.Log) (*ContractMangataTaskManagerPaused, error) {
	event := new(ContractMangataTaskManagerPaused)
	if err := _ContractMangataTaskManager.contract.UnpackLog(event, "Paused", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractMangataTaskManagerPauserRegistrySetIterator is returned from FilterPauserRegistrySet and is used to iterate over the raw logs and unpacked data for PauserRegistrySet events raised by the ContractMangataTaskManager contract.
type ContractMangataTaskManagerPauserRegistrySetIterator struct {
	Event *ContractMangataTaskManagerPauserRegistrySet // Event containing the contract specifics and raw log

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
func (it *ContractMangataTaskManagerPauserRegistrySetIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractMangataTaskManagerPauserRegistrySet)
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
		it.Event = new(ContractMangataTaskManagerPauserRegistrySet)
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
func (it *ContractMangataTaskManagerPauserRegistrySetIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractMangataTaskManagerPauserRegistrySetIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractMangataTaskManagerPauserRegistrySet represents a PauserRegistrySet event raised by the ContractMangataTaskManager contract.
type ContractMangataTaskManagerPauserRegistrySet struct {
	PauserRegistry    common.Address
	NewPauserRegistry common.Address
	Raw               types.Log // Blockchain specific contextual infos
}

// FilterPauserRegistrySet is a free log retrieval operation binding the contract event 0x6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6.
//
// Solidity: event PauserRegistrySet(address pauserRegistry, address newPauserRegistry)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) FilterPauserRegistrySet(opts *bind.FilterOpts) (*ContractMangataTaskManagerPauserRegistrySetIterator, error) {

	logs, sub, err := _ContractMangataTaskManager.contract.FilterLogs(opts, "PauserRegistrySet")
	if err != nil {
		return nil, err
	}
	return &ContractMangataTaskManagerPauserRegistrySetIterator{contract: _ContractMangataTaskManager.contract, event: "PauserRegistrySet", logs: logs, sub: sub}, nil
}

// WatchPauserRegistrySet is a free log subscription operation binding the contract event 0x6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6.
//
// Solidity: event PauserRegistrySet(address pauserRegistry, address newPauserRegistry)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) WatchPauserRegistrySet(opts *bind.WatchOpts, sink chan<- *ContractMangataTaskManagerPauserRegistrySet) (event.Subscription, error) {

	logs, sub, err := _ContractMangataTaskManager.contract.WatchLogs(opts, "PauserRegistrySet")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractMangataTaskManagerPauserRegistrySet)
				if err := _ContractMangataTaskManager.contract.UnpackLog(event, "PauserRegistrySet", log); err != nil {
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
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) ParsePauserRegistrySet(log types.Log) (*ContractMangataTaskManagerPauserRegistrySet, error) {
	event := new(ContractMangataTaskManagerPauserRegistrySet)
	if err := _ContractMangataTaskManager.contract.UnpackLog(event, "PauserRegistrySet", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractMangataTaskManagerTaskChallengedSuccessfullyIterator is returned from FilterTaskChallengedSuccessfully and is used to iterate over the raw logs and unpacked data for TaskChallengedSuccessfully events raised by the ContractMangataTaskManager contract.
type ContractMangataTaskManagerTaskChallengedSuccessfullyIterator struct {
	Event *ContractMangataTaskManagerTaskChallengedSuccessfully // Event containing the contract specifics and raw log

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
func (it *ContractMangataTaskManagerTaskChallengedSuccessfullyIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractMangataTaskManagerTaskChallengedSuccessfully)
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
		it.Event = new(ContractMangataTaskManagerTaskChallengedSuccessfully)
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
func (it *ContractMangataTaskManagerTaskChallengedSuccessfullyIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractMangataTaskManagerTaskChallengedSuccessfullyIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractMangataTaskManagerTaskChallengedSuccessfully represents a TaskChallengedSuccessfully event raised by the ContractMangataTaskManager contract.
type ContractMangataTaskManagerTaskChallengedSuccessfully struct {
	TaskIndex  uint32
	Challenger common.Address
	Raw        types.Log // Blockchain specific contextual infos
}

// FilterTaskChallengedSuccessfully is a free log retrieval operation binding the contract event 0xc20d1bb0f1623680306b83d4ff4bb99a2beb9d86d97832f3ca40fd13a29df1ec.
//
// Solidity: event TaskChallengedSuccessfully(uint32 indexed taskIndex, address indexed challenger)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) FilterTaskChallengedSuccessfully(opts *bind.FilterOpts, taskIndex []uint32, challenger []common.Address) (*ContractMangataTaskManagerTaskChallengedSuccessfullyIterator, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}
	var challengerRule []interface{}
	for _, challengerItem := range challenger {
		challengerRule = append(challengerRule, challengerItem)
	}

	logs, sub, err := _ContractMangataTaskManager.contract.FilterLogs(opts, "TaskChallengedSuccessfully", taskIndexRule, challengerRule)
	if err != nil {
		return nil, err
	}
	return &ContractMangataTaskManagerTaskChallengedSuccessfullyIterator{contract: _ContractMangataTaskManager.contract, event: "TaskChallengedSuccessfully", logs: logs, sub: sub}, nil
}

// WatchTaskChallengedSuccessfully is a free log subscription operation binding the contract event 0xc20d1bb0f1623680306b83d4ff4bb99a2beb9d86d97832f3ca40fd13a29df1ec.
//
// Solidity: event TaskChallengedSuccessfully(uint32 indexed taskIndex, address indexed challenger)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) WatchTaskChallengedSuccessfully(opts *bind.WatchOpts, sink chan<- *ContractMangataTaskManagerTaskChallengedSuccessfully, taskIndex []uint32, challenger []common.Address) (event.Subscription, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}
	var challengerRule []interface{}
	for _, challengerItem := range challenger {
		challengerRule = append(challengerRule, challengerItem)
	}

	logs, sub, err := _ContractMangataTaskManager.contract.WatchLogs(opts, "TaskChallengedSuccessfully", taskIndexRule, challengerRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractMangataTaskManagerTaskChallengedSuccessfully)
				if err := _ContractMangataTaskManager.contract.UnpackLog(event, "TaskChallengedSuccessfully", log); err != nil {
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

// ParseTaskChallengedSuccessfully is a log parse operation binding the contract event 0xc20d1bb0f1623680306b83d4ff4bb99a2beb9d86d97832f3ca40fd13a29df1ec.
//
// Solidity: event TaskChallengedSuccessfully(uint32 indexed taskIndex, address indexed challenger)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) ParseTaskChallengedSuccessfully(log types.Log) (*ContractMangataTaskManagerTaskChallengedSuccessfully, error) {
	event := new(ContractMangataTaskManagerTaskChallengedSuccessfully)
	if err := _ContractMangataTaskManager.contract.UnpackLog(event, "TaskChallengedSuccessfully", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractMangataTaskManagerTaskChallengedUnsuccessfullyIterator is returned from FilterTaskChallengedUnsuccessfully and is used to iterate over the raw logs and unpacked data for TaskChallengedUnsuccessfully events raised by the ContractMangataTaskManager contract.
type ContractMangataTaskManagerTaskChallengedUnsuccessfullyIterator struct {
	Event *ContractMangataTaskManagerTaskChallengedUnsuccessfully // Event containing the contract specifics and raw log

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
func (it *ContractMangataTaskManagerTaskChallengedUnsuccessfullyIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractMangataTaskManagerTaskChallengedUnsuccessfully)
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
		it.Event = new(ContractMangataTaskManagerTaskChallengedUnsuccessfully)
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
func (it *ContractMangataTaskManagerTaskChallengedUnsuccessfullyIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractMangataTaskManagerTaskChallengedUnsuccessfullyIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractMangataTaskManagerTaskChallengedUnsuccessfully represents a TaskChallengedUnsuccessfully event raised by the ContractMangataTaskManager contract.
type ContractMangataTaskManagerTaskChallengedUnsuccessfully struct {
	TaskIndex  uint32
	Challenger common.Address
	Raw        types.Log // Blockchain specific contextual infos
}

// FilterTaskChallengedUnsuccessfully is a free log retrieval operation binding the contract event 0xfd3e26beeb5967fc5a57a0446914eabc45b4aa474c67a51b4b5160cac60ddb05.
//
// Solidity: event TaskChallengedUnsuccessfully(uint32 indexed taskIndex, address indexed challenger)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) FilterTaskChallengedUnsuccessfully(opts *bind.FilterOpts, taskIndex []uint32, challenger []common.Address) (*ContractMangataTaskManagerTaskChallengedUnsuccessfullyIterator, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}
	var challengerRule []interface{}
	for _, challengerItem := range challenger {
		challengerRule = append(challengerRule, challengerItem)
	}

	logs, sub, err := _ContractMangataTaskManager.contract.FilterLogs(opts, "TaskChallengedUnsuccessfully", taskIndexRule, challengerRule)
	if err != nil {
		return nil, err
	}
	return &ContractMangataTaskManagerTaskChallengedUnsuccessfullyIterator{contract: _ContractMangataTaskManager.contract, event: "TaskChallengedUnsuccessfully", logs: logs, sub: sub}, nil
}

// WatchTaskChallengedUnsuccessfully is a free log subscription operation binding the contract event 0xfd3e26beeb5967fc5a57a0446914eabc45b4aa474c67a51b4b5160cac60ddb05.
//
// Solidity: event TaskChallengedUnsuccessfully(uint32 indexed taskIndex, address indexed challenger)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) WatchTaskChallengedUnsuccessfully(opts *bind.WatchOpts, sink chan<- *ContractMangataTaskManagerTaskChallengedUnsuccessfully, taskIndex []uint32, challenger []common.Address) (event.Subscription, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}
	var challengerRule []interface{}
	for _, challengerItem := range challenger {
		challengerRule = append(challengerRule, challengerItem)
	}

	logs, sub, err := _ContractMangataTaskManager.contract.WatchLogs(opts, "TaskChallengedUnsuccessfully", taskIndexRule, challengerRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractMangataTaskManagerTaskChallengedUnsuccessfully)
				if err := _ContractMangataTaskManager.contract.UnpackLog(event, "TaskChallengedUnsuccessfully", log); err != nil {
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

// ParseTaskChallengedUnsuccessfully is a log parse operation binding the contract event 0xfd3e26beeb5967fc5a57a0446914eabc45b4aa474c67a51b4b5160cac60ddb05.
//
// Solidity: event TaskChallengedUnsuccessfully(uint32 indexed taskIndex, address indexed challenger)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) ParseTaskChallengedUnsuccessfully(log types.Log) (*ContractMangataTaskManagerTaskChallengedUnsuccessfully, error) {
	event := new(ContractMangataTaskManagerTaskChallengedUnsuccessfully)
	if err := _ContractMangataTaskManager.contract.UnpackLog(event, "TaskChallengedUnsuccessfully", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractMangataTaskManagerTaskCompletedIterator is returned from FilterTaskCompleted and is used to iterate over the raw logs and unpacked data for TaskCompleted events raised by the ContractMangataTaskManager contract.
type ContractMangataTaskManagerTaskCompletedIterator struct {
	Event *ContractMangataTaskManagerTaskCompleted // Event containing the contract specifics and raw log

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
func (it *ContractMangataTaskManagerTaskCompletedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractMangataTaskManagerTaskCompleted)
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
		it.Event = new(ContractMangataTaskManagerTaskCompleted)
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
func (it *ContractMangataTaskManagerTaskCompletedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractMangataTaskManagerTaskCompletedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractMangataTaskManagerTaskCompleted represents a TaskCompleted event raised by the ContractMangataTaskManager contract.
type ContractMangataTaskManagerTaskCompleted struct {
	TaskIndex uint32
	Raw       types.Log // Blockchain specific contextual infos
}

// FilterTaskCompleted is a free log retrieval operation binding the contract event 0x9a144f228a931b9d0d1696fbcdaf310b24b5d2d21e799db623fc986a0f547430.
//
// Solidity: event TaskCompleted(uint32 indexed taskIndex)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) FilterTaskCompleted(opts *bind.FilterOpts, taskIndex []uint32) (*ContractMangataTaskManagerTaskCompletedIterator, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractMangataTaskManager.contract.FilterLogs(opts, "TaskCompleted", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return &ContractMangataTaskManagerTaskCompletedIterator{contract: _ContractMangataTaskManager.contract, event: "TaskCompleted", logs: logs, sub: sub}, nil
}

// WatchTaskCompleted is a free log subscription operation binding the contract event 0x9a144f228a931b9d0d1696fbcdaf310b24b5d2d21e799db623fc986a0f547430.
//
// Solidity: event TaskCompleted(uint32 indexed taskIndex)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) WatchTaskCompleted(opts *bind.WatchOpts, sink chan<- *ContractMangataTaskManagerTaskCompleted, taskIndex []uint32) (event.Subscription, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractMangataTaskManager.contract.WatchLogs(opts, "TaskCompleted", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractMangataTaskManagerTaskCompleted)
				if err := _ContractMangataTaskManager.contract.UnpackLog(event, "TaskCompleted", log); err != nil {
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

// ParseTaskCompleted is a log parse operation binding the contract event 0x9a144f228a931b9d0d1696fbcdaf310b24b5d2d21e799db623fc986a0f547430.
//
// Solidity: event TaskCompleted(uint32 indexed taskIndex)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) ParseTaskCompleted(log types.Log) (*ContractMangataTaskManagerTaskCompleted, error) {
	event := new(ContractMangataTaskManagerTaskCompleted)
	if err := _ContractMangataTaskManager.contract.UnpackLog(event, "TaskCompleted", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractMangataTaskManagerTaskRespondedIterator is returned from FilterTaskResponded and is used to iterate over the raw logs and unpacked data for TaskResponded events raised by the ContractMangataTaskManager contract.
type ContractMangataTaskManagerTaskRespondedIterator struct {
	Event *ContractMangataTaskManagerTaskResponded // Event containing the contract specifics and raw log

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
func (it *ContractMangataTaskManagerTaskRespondedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractMangataTaskManagerTaskResponded)
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
		it.Event = new(ContractMangataTaskManagerTaskResponded)
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
func (it *ContractMangataTaskManagerTaskRespondedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractMangataTaskManagerTaskRespondedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractMangataTaskManagerTaskResponded represents a TaskResponded event raised by the ContractMangataTaskManager contract.
type ContractMangataTaskManagerTaskResponded struct {
	TaskResponse         IMangataTaskManagerTaskResponse
	TaskResponseMetadata IMangataTaskManagerTaskResponseMetadata
	Raw                  types.Log // Blockchain specific contextual infos
}

// FilterTaskResponded is a free log retrieval operation binding the contract event 0xf2af11fad73d4349c99cf62f298d337641ea0bb7c0f5a8db92a98a275f734f58.
//
// Solidity: event TaskResponded((uint32,bytes32) taskResponse, (uint32,bytes32) taskResponseMetadata)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) FilterTaskResponded(opts *bind.FilterOpts) (*ContractMangataTaskManagerTaskRespondedIterator, error) {

	logs, sub, err := _ContractMangataTaskManager.contract.FilterLogs(opts, "TaskResponded")
	if err != nil {
		return nil, err
	}
	return &ContractMangataTaskManagerTaskRespondedIterator{contract: _ContractMangataTaskManager.contract, event: "TaskResponded", logs: logs, sub: sub}, nil
}

// WatchTaskResponded is a free log subscription operation binding the contract event 0xf2af11fad73d4349c99cf62f298d337641ea0bb7c0f5a8db92a98a275f734f58.
//
// Solidity: event TaskResponded((uint32,bytes32) taskResponse, (uint32,bytes32) taskResponseMetadata)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) WatchTaskResponded(opts *bind.WatchOpts, sink chan<- *ContractMangataTaskManagerTaskResponded) (event.Subscription, error) {

	logs, sub, err := _ContractMangataTaskManager.contract.WatchLogs(opts, "TaskResponded")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractMangataTaskManagerTaskResponded)
				if err := _ContractMangataTaskManager.contract.UnpackLog(event, "TaskResponded", log); err != nil {
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

// ParseTaskResponded is a log parse operation binding the contract event 0xf2af11fad73d4349c99cf62f298d337641ea0bb7c0f5a8db92a98a275f734f58.
//
// Solidity: event TaskResponded((uint32,bytes32) taskResponse, (uint32,bytes32) taskResponseMetadata)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) ParseTaskResponded(log types.Log) (*ContractMangataTaskManagerTaskResponded, error) {
	event := new(ContractMangataTaskManagerTaskResponded)
	if err := _ContractMangataTaskManager.contract.UnpackLog(event, "TaskResponded", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractMangataTaskManagerUnpausedIterator is returned from FilterUnpaused and is used to iterate over the raw logs and unpacked data for Unpaused events raised by the ContractMangataTaskManager contract.
type ContractMangataTaskManagerUnpausedIterator struct {
	Event *ContractMangataTaskManagerUnpaused // Event containing the contract specifics and raw log

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
func (it *ContractMangataTaskManagerUnpausedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractMangataTaskManagerUnpaused)
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
		it.Event = new(ContractMangataTaskManagerUnpaused)
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
func (it *ContractMangataTaskManagerUnpausedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractMangataTaskManagerUnpausedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractMangataTaskManagerUnpaused represents a Unpaused event raised by the ContractMangataTaskManager contract.
type ContractMangataTaskManagerUnpaused struct {
	Account         common.Address
	NewPausedStatus *big.Int
	Raw             types.Log // Blockchain specific contextual infos
}

// FilterUnpaused is a free log retrieval operation binding the contract event 0x3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c.
//
// Solidity: event Unpaused(address indexed account, uint256 newPausedStatus)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) FilterUnpaused(opts *bind.FilterOpts, account []common.Address) (*ContractMangataTaskManagerUnpausedIterator, error) {

	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}

	logs, sub, err := _ContractMangataTaskManager.contract.FilterLogs(opts, "Unpaused", accountRule)
	if err != nil {
		return nil, err
	}
	return &ContractMangataTaskManagerUnpausedIterator{contract: _ContractMangataTaskManager.contract, event: "Unpaused", logs: logs, sub: sub}, nil
}

// WatchUnpaused is a free log subscription operation binding the contract event 0x3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c.
//
// Solidity: event Unpaused(address indexed account, uint256 newPausedStatus)
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) WatchUnpaused(opts *bind.WatchOpts, sink chan<- *ContractMangataTaskManagerUnpaused, account []common.Address) (event.Subscription, error) {

	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}

	logs, sub, err := _ContractMangataTaskManager.contract.WatchLogs(opts, "Unpaused", accountRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractMangataTaskManagerUnpaused)
				if err := _ContractMangataTaskManager.contract.UnpackLog(event, "Unpaused", log); err != nil {
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
func (_ContractMangataTaskManager *ContractMangataTaskManagerFilterer) ParseUnpaused(log types.Log) (*ContractMangataTaskManagerUnpaused, error) {
	event := new(ContractMangataTaskManagerUnpaused)
	if err := _ContractMangataTaskManager.contract.UnpackLog(event, "Unpaused", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

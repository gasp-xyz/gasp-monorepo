package core

import (
	"math/big"

	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	taskmanager "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/FinalizerTaskManager"
	"golang.org/x/crypto/sha3"
)

// this hardcodes abi.encode() for taskmanager.IFinalizerTaskManagerTaskResponse
// unclear why abigen doesn't provide this out of the box...
func AbiEncodeOpTaskResponse(h *taskmanager.IFinalizerTaskManagerOpTaskResponse) ([]byte, error) {

	parsedAbi, err := taskmanager.ContractFinalizerTaskManagerMetaData.GetAbi()
	inputParameters := parsedAbi.Methods["respondToOpTask"].Inputs
	args := inputParameters[1:2]

	bytes, err := args.Pack(h)
	if err != nil {
		return nil, err
	}

	return bytes, nil
}

// GetTaskResponseDigest returns the hash of the TaskResponse, which is what operators sign over
func GetOpTaskResponseDigest(h *taskmanager.IFinalizerTaskManagerOpTaskResponse) ([32]byte, error) {

	encodeTaskResponseByte, err := AbiEncodeOpTaskResponse(h)
	if err != nil {
		return [32]byte{}, err
	}

	var taskResponseDigest [32]byte
	hasher := sha3.NewLegacyKeccak256()
	hasher.Write(encodeTaskResponseByte)
	copy(taskResponseDigest[:], hasher.Sum(nil)[:32])

	return taskResponseDigest, nil
}

// this hardcodes abi.encode() for taskmanager.IFinalizerTaskManagerTaskResponse
// unclear why abigen doesn't provide this out of the box...
func AbiEncodeRdTaskResponse(h *taskmanager.IFinalizerTaskManagerRdTaskResponse) ([]byte, error) {

	parsedAbi, err := taskmanager.ContractFinalizerTaskManagerMetaData.GetAbi()
	inputParameters := parsedAbi.Methods["respondToRdTask"].Inputs
	args := inputParameters[1:2]

	bytes, err := args.Pack(h)
	if err != nil {
		return nil, err
	}

	return bytes, nil
}

// GetTaskResponseDigest returns the hash of the TaskResponse, which is what operators sign over
func GetRdTaskResponseDigest(h *taskmanager.IFinalizerTaskManagerRdTaskResponse) ([32]byte, error) {

	encodeTaskResponseByte, err := AbiEncodeRdTaskResponse(h)
	if err != nil {
		return [32]byte{}, err
	}

	var taskResponseDigest [32]byte
	hasher := sha3.NewLegacyKeccak256()
	hasher.Write(encodeTaskResponseByte)
	copy(taskResponseDigest[:], hasher.Sum(nil)[:32])

	return taskResponseDigest, nil
}

// BINDING UTILS - conversion from contract structs to golang structs

// BN254.sol is a library, so bindings for G1 Points and G2 Points are only generated
// in every contract that imports that library. Thus the output here will need to be
// type casted if G1Point is needed to interface with another contract (eg: BLSPublicKeyCompendium.sol)
func ConvertToBN254G1Point(input *bls.G1Point) taskmanager.BN254G1Point {
	output := taskmanager.BN254G1Point{
		X: input.X.BigInt(big.NewInt(0)),
		Y: input.Y.BigInt(big.NewInt(0)),
	}
	return output
}

func ConvertToBN254G2Point(input *bls.G2Point) taskmanager.BN254G2Point {
	output := taskmanager.BN254G2Point{
		X: [2]*big.Int{input.X.A1.BigInt(big.NewInt(0)), input.X.A0.BigInt(big.NewInt(0))},
		Y: [2]*big.Int{input.Y.A1.BigInt(big.NewInt(0)), input.Y.A0.BigInt(big.NewInt(0))},
	}
	return output
}

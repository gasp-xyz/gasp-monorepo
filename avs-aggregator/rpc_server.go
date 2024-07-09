package aggregator

import (
	"context"
	"encoding/json"
	"errors"
	"net/http"

	"github.com/ethereum/go-ethereum/accounts/abi"
	"encoding/hex"

	taskmanager "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/FinalizerTaskManager"
	"github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/core"

	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	"github.com/Layr-Labs/eigensdk-go/services/bls_aggregation"
	"github.com/Layr-Labs/eigensdk-go/types"
	sdktypes "github.com/Layr-Labs/eigensdk-go/types"
)

var (
	TaskNotFoundError400                     = errors.New("400. Task not found")
	OperatorNotPartOfTaskQuorum400           = errors.New("400. Operator not part of quorum")
	OperatorNotRegistered400                 = errors.New("400. Operator not registered in AVS")
	TaskResponseDigestNotFoundError500       = errors.New("500. Failed to get task response digest")
	UnknownErrorWhileVerifyingSignature400   = errors.New("400. Failed to verify signature")
	SignatureVerificationFailed400           = errors.New("400. Signature verification failed")
	CallToGetCheckSignaturesIndicesFailed500 = errors.New("500. Failed to get check signatures indices")
)

func (agg *Aggregator) startServer(ctx context.Context) error {
	http.HandleFunc("/", agg.handler)
	err := http.ListenAndServe(agg.serverIpPortAddr, nil)
	if err != nil {
		agg.logger.Fatal("ListenAndServe", "err", err)
	}

	return nil
}

func (agg *Aggregator) handler(w http.ResponseWriter, req *http.Request) {
	switch req.Method {
	case http.MethodConnect:
		http.Error(w, "Operator not supported, please upgrade to latest", http.StatusUpgradeRequired)
		return
	case http.MethodPost:
		break
	default:
		http.Error(w, "Method not supported", http.StatusMethodNotAllowed)
		return
	}

	var response SignedTaskResponse
	if err := json.NewDecoder(req.Body).Decode(&response); err != nil {
		http.Error(w, "Error parsing request body", http.StatusBadRequest)
		return
	}

	if err := agg.ProcessSignedTaskResponse(&response, nil); err != nil {
		var status int
		switch err {
		case TaskResponseDigestNotFoundError500, CallToGetCheckSignaturesIndicesFailed500:
			status = http.StatusInternalServerError
		default:
			switch err.Error() {
			// case blsagg.TaskNotFoundErrorFn(response.TaskResponse.ReferenceTaskIndex).Error():
			case blsagg.TaskNotFoundErrorFn(0).Error():
				status = http.StatusNotFound
			default:
				status = http.StatusBadRequest
			}
		}
		http.Error(w, err.Error(), status)
		return
	}
}

type SignedTaskResponse struct {
	TaskResponse string
	BlsSignature bls.Signature
	OperatorId   types.OperatorId
}

// rpc endpoint which is called by operator
// reply doesn't need to be checked. If there are no errors, the task response is accepted
// rpc framework forces a reply type to exist, so we put bool as a placeholder
func (agg *Aggregator) ProcessSignedTaskResponse(signedTaskResponse *SignedTaskResponse, reply *bool) error {
	agg.logger.Info("Received signed task response", "response", signedTaskResponse, "operatorId", signedTaskResponse.OperatorId.LogValue())

	task_response_bytes, err := hex.DecodeString(signedTaskResponse.TaskResponse[2:])
	if err != nil {
		agg.logger.Error("Failed to get task_response_bytes", "err", err)
		return TaskResponseDigestNotFoundError500
	}

	var taskResponse taskmanager.IFinalizerTaskManagerTaskResponse

	parsedAbi, err := taskmanager.ContractFinalizerTaskManagerMetaData.GetAbi()
	// TODO replace with dummy function?
	inputParameters := parsedAbi.Methods["respondToTask"].Inputs
	args := inputParameters[1:2]
	unpacked, err := args.Unpack(task_response_bytes)
	if err != nil {
		agg.logger.Error("Failed to get task response", "err", err)
		return TaskResponseDigestNotFoundError500
	}
	x := abi.ConvertType(unpacked[0], taskResponse)
	cx, ok := x.(taskmanager.IFinalizerTaskManagerTaskResponse)
	if !ok {
		agg.logger.Error("Failed to get task response cx", "cx", cx)
		return TaskResponseDigestNotFoundError500
	}

	taskResponse = cx

	taskIndex := taskResponse.ReferenceTaskIndex
	taskResponseDigest, err := core.GetTaskResponseDigest(&taskResponse)
	if err != nil {
		agg.logger.Error("Failed to get task response digest", "err", err)
		return TaskResponseDigestNotFoundError500
	}
	if signedTaskResponse.OperatorId == [32]byte{} {
		agg.logger.Error("Operator not registered", "err", err)
		return OperatorNotRegistered400
	}
	agg.taskResponsesMu.Lock()
	if _, ok := agg.taskResponses[taskIndex]; !ok {
		agg.taskResponses[taskIndex] = make(map[sdktypes.TaskResponseDigest]taskmanager.IFinalizerTaskManagerTaskResponse)
	}
	if _, ok := agg.taskResponses[taskIndex][taskResponseDigest]; !ok {
		agg.taskResponses[taskIndex][taskResponseDigest] = taskResponse
	}
	agg.taskResponsesMu.Unlock()

	err = agg.blsAggregationService.ProcessNewSignature(
		context.Background(), taskIndex, taskResponseDigest,
		&signedTaskResponse.BlsSignature, signedTaskResponse.OperatorId,
	)
	return err
}

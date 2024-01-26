package aggregator

import (
	"context"
	"encoding/json"
	"errors"
	"net/http"

	taskmanager "github.com/mangata-finance/eigen-layer-monorepo/aggregator/bindings/MangataTaskManager"
	"github.com/mangata-finance/eigen-layer-monorepo/aggregator/core"

	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
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
			status = http.StatusBadRequest
		}
		http.Error(w, err.Error(), status)
		return
	}
}

type SignedTaskResponse struct {
	TaskResponse taskmanager.IMangataTaskManagerTaskResponse
	BlsSignature bls.Signature
	OperatorId   bls.OperatorId
}

// rpc endpoint which is called by operator
// reply doesn't need to be checked. If there are no errors, the task response is accepted
// rpc framework forces a reply type to exist, so we put bool as a placeholder
func (agg *Aggregator) ProcessSignedTaskResponse(signedTaskResponse *SignedTaskResponse, reply *bool) error {
	agg.logger.Info("Received signed task response", "response", signedTaskResponse)
	taskIndex := signedTaskResponse.TaskResponse.ReferenceTaskIndex
	taskResponseDigest, err := core.GetTaskResponseDigest(&signedTaskResponse.TaskResponse)
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
		agg.taskResponses[taskIndex] = make(map[sdktypes.TaskResponseDigest]taskmanager.IMangataTaskManagerTaskResponse)
	}
	if _, ok := agg.taskResponses[taskIndex][taskResponseDigest]; !ok {
		agg.taskResponses[taskIndex][taskResponseDigest] = signedTaskResponse.TaskResponse
	}
	agg.taskResponsesMu.Unlock()

	err = agg.blsAggregationService.ProcessNewSignature(
		context.Background(), taskIndex, taskResponseDigest,
		&signedTaskResponse.BlsSignature, signedTaskResponse.OperatorId,
	)
	return err
}

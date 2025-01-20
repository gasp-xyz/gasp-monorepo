package aggregator

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"net/http"

	"encoding/hex"

	"github.com/ethereum/go-ethereum/accounts/abi"

	taskmanager "github.com/gasp-xyz/gasp-monorepo/avs-aggregator/bindings/FinalizerTaskManager"
	"github.com/gasp-xyz/gasp-monorepo/avs-aggregator/core"

	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	blsagg "github.com/Layr-Labs/eigensdk-go/services/bls_aggregation"
	"github.com/Layr-Labs/eigensdk-go/types"
	sdktypes "github.com/Layr-Labs/eigensdk-go/types"
)

var (
	TaskNotFoundError400                     = errors.New("400. Task not found")
	OperatorNotPartOfTaskQuorum400           = errors.New("400. Operator not part of quorum")
	OperatorNotRegistered400                 = errors.New("400. Operator not registered in AVS")
	BadTaskResponseError500       = errors.New("500. Bad Task Response")
	TaskResponseDigestNotFoundError500       = errors.New("500. Failed to get task response digest")
	UnknownErrorWhileVerifyingSignature400   = errors.New("400. Failed to verify signature")
	SignatureVerificationFailed400           = errors.New("400. Signature verification failed")
	CallToGetCheckSignaturesIndicesFailed500 = errors.New("500. Failed to get check signatures indices")
)

func (agg *Aggregator) startServer(ctx context.Context, apiKey string, runTrigger chan struct{} ) error {
	http.HandleFunc("/", agg.handler)
	http.HandleFunc("/isAwaitingRunTrigger", func(w http.ResponseWriter, r *http.Request) {

		var isAwaitingRunTrigger bool

		// If this is run in parallel then there will be a race condition between this /isAwaitingRunTrigger and /run
		// We don't want to receive the trigger here
		// But this rpc is not executed parallely so we should never have that
		// But still we check for it
		select {
		case _, ok := <-runTrigger: // if it doesn't block on receive then the channel is closed
			// if we receive an ok == true here we have a problem, this should never happen
			if ok {
				panic("received run trigger in isAwaitingRunTrigger check")
			}
			isAwaitingRunTrigger = false
		default: // if it blocks on receive rather than accept then the channel is still open
			isAwaitingRunTrigger = true
		}

		// Respond with JSON named data
		response := map[string]interface{}{
			"isAwaitingRunTrigger": isAwaitingRunTrigger,
			"status":  "OK",
		}
	
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(response) // Encode and send JSON response
	})
	http.HandleFunc("/run", func(w http.ResponseWriter, r *http.Request) {
		// Parse query parameters
		key := r.URL.Query().Get("SECRET_API_KEY")
	
		if key != apiKey {
			agg.logger.Error("Api key no match", "Received", key)
			http.Error(w, "Api key no match", http.StatusBadRequest)
			return
		}
	
		// Respond with JSON named data
		response := map[string]string{
			"message": fmt.Sprintf("Triggered run on agg"),
			"status":  "OK",
		}
	
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(response) // Encode and send JSON response
		runTrigger <- struct{}{}
		close(runTrigger)
	})
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
			case blsagg.TaskNotFoundErrorFn().Error():
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
	OpTaskResponse string
	RdTaskResponse string
	BlsSignature bls.Signature
	OperatorId   types.OperatorId
}

// rpc endpoint which is called by operator
// reply doesn't need to be checked. If there are no errors, the task response is accepted
// rpc framework forces a reply type to exist, so we put bool as a placeholder
func (agg *Aggregator) ProcessSignedTaskResponse(signedTaskResponse *SignedTaskResponse, reply *bool) error {
	agg.logger.Info("Received signed task response", "response", signedTaskResponse, "operatorId", signedTaskResponse.OperatorId.LogValue())

	if len(signedTaskResponse.OpTaskResponse) < 2 ||
		len(signedTaskResponse.RdTaskResponse) < 2 ||
		signedTaskResponse.BlsSignature.G1Point == nil {
		agg.logger.Error("Invalid task response")
		return BadTaskResponseError500
	}
	
	op_task_response_bytes, err := hex.DecodeString(signedTaskResponse.OpTaskResponse[2:])
	if err != nil {
		agg.logger.Error("Failed to get op_task_response_bytes", "err", err)
		return BadTaskResponseError500
	}

	rd_task_response_bytes, err := hex.DecodeString(signedTaskResponse.RdTaskResponse[2:])
	if err != nil {
		agg.logger.Error("Failed to get rd_task_response_bytes", "err", err)
		return BadTaskResponseError500
	}

	if len(op_task_response_bytes) !=0 && len(rd_task_response_bytes) !=0 {
		agg.logger.Error("Both op and rd task response are popoulated")
		return BadTaskResponseError500
	}

	if len(op_task_response_bytes) ==0 && len(rd_task_response_bytes) ==0 {
		agg.logger.Error("Neither op nor rd task response are popoulated")
		return BadTaskResponseError500
	}

	// due to above checks it is one of the two op or rd tasks
	// so the following should work

	var taskId types.TaskId
	var genericTaskResponse interface{}
	var taskResponseDigest [32]byte

	parsedAbi, err := taskmanager.ContractFinalizerTaskManagerMetaData.GetAbi()

	if len(op_task_response_bytes) !=0 {
		var taskResponse taskmanager.IFinalizerTaskManagerOpTaskResponse
	
		// TODO replace with dummy function?
		inputParameters := parsedAbi.Methods["respondToOpTask"].Inputs
		args := inputParameters[1:2]
		unpacked, err := args.Unpack(op_task_response_bytes)
		if err != nil {
			agg.logger.Error("Failed to get task response", "err", err)
			return TaskResponseDigestNotFoundError500
		}
		x := abi.ConvertType(unpacked[0], taskResponse)
		cx, ok := x.(taskmanager.IFinalizerTaskManagerOpTaskResponse)
		if !ok {
			agg.logger.Error("Failed to get task response cx", "cx", cx)
			return TaskResponseDigestNotFoundError500
		}
	
		taskResponse = cx
	
		taskId = types.TaskId{
			TaskType: types.TaskType(0),
			TaskIndex: types.TaskIndex(taskResponse.ReferenceTaskIndex),
			}
		taskResponseDigest, err = core.GetOpTaskResponseDigest(&taskResponse)
		if err != nil {
			agg.logger.Error("Failed to get task response digest", "err", err)
			return TaskResponseDigestNotFoundError500
		}
		genericTaskResponse = taskResponse

	}

	if len(rd_task_response_bytes) !=0 {
		var taskResponse taskmanager.IFinalizerTaskManagerRdTaskResponse
	
		// TODO replace with dummy function?
		inputParameters := parsedAbi.Methods["respondToRdTask"].Inputs
		args := inputParameters[1:2]
		unpacked, err := args.Unpack(rd_task_response_bytes)
		if err != nil {
			agg.logger.Error("Failed to get task response", "err", err)
			return TaskResponseDigestNotFoundError500
		}
		x := abi.ConvertType(unpacked[0], taskResponse)
		cx, ok := x.(taskmanager.IFinalizerTaskManagerRdTaskResponse)
		if !ok {
			agg.logger.Error("Failed to get task response cx", "cx", cx)
			return TaskResponseDigestNotFoundError500
		}
	
		taskResponse = cx
	
		taskId = types.TaskId{
			TaskType: types.TaskType(1),
			TaskIndex: types.TaskIndex(taskResponse.ReferenceTaskIndex),
			}
		taskResponseDigest, err = core.GetRdTaskResponseDigest(&taskResponse)
		if err != nil {
			agg.logger.Error("Failed to get task response digest", "err", err)
			return TaskResponseDigestNotFoundError500
		}
		genericTaskResponse = taskResponse

	}

	if signedTaskResponse.OperatorId == [32]byte{} {
		agg.logger.Error("Operator not registered", "err", err)
		return OperatorNotRegistered400
	}
	agg.taskResponsesMu.Lock()
	if _, ok := agg.taskResponses[taskId]; !ok {
		agg.taskResponses[taskId] = make(map[sdktypes.TaskResponseDigest]interface{})
	}
	if _, ok := agg.taskResponses[taskId][taskResponseDigest]; !ok {
		agg.taskResponses[taskId][taskResponseDigest] = genericTaskResponse
	}
	agg.taskResponsesMu.Unlock()

	err = agg.blsAggregationService.ProcessNewSignature(
		context.Background(), taskId, taskResponseDigest,
		&signedTaskResponse.BlsSignature, signedTaskResponse.OperatorId,
	)
	return err
}

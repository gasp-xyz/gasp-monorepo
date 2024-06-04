package aggregator

import (
	"context"
	"encoding/json"
	"errors"
	"net/http"

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
			case blsagg.TaskNotFoundErrorFn(response.TaskResponse.ReferenceTaskIndex).Error():
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
	TaskResponse taskmanager.IFinalizerTaskManagerTaskResponse
	BlsSignature bls.Signature
	OperatorId   types.OperatorId
}

// rpc endpoint which is called by operator
// reply doesn't need to be checked. If there are no errors, the task response is accepted
// rpc framework forces a reply type to exist, so we put bool as a placeholder
func (agg *Aggregator) ProcessSignedTaskResponse(signedTaskResponse *SignedTaskResponse, reply *bool) error {
	agg.logger.Info("Received signed task response", "response", signedTaskResponse, "operatorId", signedTaskResponse.OperatorId.LogValue())
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
		agg.taskResponses[taskIndex] = make(map[sdktypes.TaskResponseDigest]taskmanager.IFinalizerTaskManagerTaskResponse)
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

// OperatorStatus {
// 	avs-finalizer-1   |     eth_address: 0x2c9514c724515f5796bfcd5a3555d57493692922,
// 	avs-finalizer-1   |     registered_with_eigen: true,
// 	avs-finalizer-1   |     bls_key_registered: true,
// 	avs-finalizer-1   |     bls_g1: G1Point {
// 	avs-finalizer-1   |         x: 19103047014458333102771046173622446641964731795002409519915434034263897589057,
// 	avs-finalizer-1   |         y: 2295952366290670263472454533011486889943632186814447310844324824317313715255,
// 	avs-finalizer-1   |     },
// 	avs-finalizer-1   |     bls_g2: G2Point {
// 	avs-finalizer-1   |         x: [
// 	avs-finalizer-1   |             11507568495756053182134505605091225695586339633689926518759008653700327195340,
// 	avs-finalizer-1   |             547807548228344778563815376320635903509677052966523321561007854552401587467,
// 	avs-finalizer-1   |         ],
// 	avs-finalizer-1   |         y: [
// 	avs-finalizer-1   |             2836658439355967617809863679848518114974945973907740374226934555190660307553,
// 	avs-finalizer-1   |             1167547410576856564964743897663194384866764979835003390812493378548623435010,
// 	avs-finalizer-1   |         ],
// 	avs-finalizer-1   |     },
// 	avs-finalizer-1   |     registered_with_avs: true,
// 	avs-finalizer-1   |     operator_id: Some(
// 	avs-finalizer-1   |         0xbc851dedba931d453e011c4fb4eda6472e4ec545ca69693bce13a509dc7fdc5e,
// 	avs-finalizer-1   |     ),
// 	avs-finalizer-1   | }

// 	avs-finalizer-1   | SignedTaskResponse { task_response: TaskResponseWire { reference_task_index: 1, operators_state_hash: [7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7], block_hash: [179, 222, 63, 132, 199, 146, 116, 105, 248, 106, 89, 237, 168, 11, 188, 133, 222, 102, 193, 182, 91, 122, 135, 76, 107, 162, 21, 103, 78, 215, 158, 39], storage_proof_hash: [98, 67, 218, 167, 84, 200, 245, 6, 169, 15, 242, 61, 32, 137, 133, 104, 221, 159, 25, 220, 87, 148, 191, 188, 185, 22, 148, 123, 158, 113, 166, 91], pending_state_hash: [31, 188, 19, 31, 78, 175, 205, 220, 101, 13, 225, 81, 155, 55, 247, 31, 107, 154, 134, 69, 35, 200, 63, 22, 57, 47, 71, 152, 204, 46, 185, 25] }, bls_signature: BlsSignatureWire { g1_point: G1PointWire { x: BigInt([7465187858803810052, 11885315189215878424, 2931683236562232522, 3445309009331256907]), y: BigInt([345405741213586264, 3112153054911712264, 3170798046816384846, 2485196340116498254]) } }, operator_id: [188, 133, 29, 237, 186, 147, 29, 69, 62, 1, 28, 79, 180, 237, 166, 71, 46, 78, 197, 69, 202, 105, 105, 59, 206, 19, 165, 9, 220, 127, 220, 94] }
// 	avs-aggregator-1  | 2024-06-04T14:05:27.454Z	INFO	avs-aggregator/rpc_server.go:84	Received signed task response	{"response": {"TaskResponse":{"ReferenceTaskIndex":1,"OperatorsStateHash":[7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7],"BlockHash":[179,222,63,132,199,146,116,105,248,106,89,237,168,11,188,133,222,102,193,182,91,122,135,76,107,162,21,103,78,215,158,39],"StorageProofHash":[98,67,218,167,84,200,245,6,169,15,242,61,32,137,133,104,221,159,25,220,87,148,191,188,185,22,148,123,158,113,166,91],"PendingStateHash":[31,188,19,31,78,175,205,220,101,13,225,81,155,55,247,31,107,154,134,69,35,200,63,22,57,47,71,152,204,46,185,25]},"BlsSignature":{"g1_point":{"X":"21626555161416598641036571126028761885787781285664430506687716402728929287940","Y":"15599830259321898912464993223807627531962436563254314861362290908168788284248"}},"OperatorId":[188,133,29,237,186,147,29,69,62,1,28,79,180,237,166,71,46,78,197,69,202,105,105,59,206,19,165,9,220,127,220,94]}, "operatorId": "bc851dedba931d453e011c4fb4eda6472e4ec545ca69693bce13a509dc7fdc5e"}

// 	avs-aggregator-1  | 2024-06-04T14:05:27.454Z	DEBUG	bls_aggregation/blsagg.go:252	Task goroutine received new signed task response digest	{"taskIndex": 1, "signedTaskResponseDigest": {"TaskResponseDigest":[124,88,144,120,223,114,210,191,244,116,251,75,207,16,111,199,239,103,125,141,21,137,67,12,225,221,167,57,252,5,27,244],"BlsSignature":{"g1_point":{"X":"21626555161416598641036571126028761885787781285664430506687716402728929287940","Y":"15599830259321898912464993223807627531962436563254314861362290908168788284248"}},"OperatorId":[188,133,29,237,186,147,29,69,62,1,28,79,180,237,166,71,46,78,197,69,202,105,105,59,206,19,165,9,220,127,220,94]}}
// 	avs-aggregator-1  | 2024-06-04T14:05:27.454Z	DEBUG	bls_aggregation/blsagg.go:407	Verifying signed task response digest signature	{"operatorG2Pubkey": "E([547807548228344778563815376320635903509677052966523321561007854552401587467+11507568495756053182134505605091225695586339633689926518759008653700327195340*u,1167547410576856564964743897663194384866764979835003390812493378548623435010+2836658439355967617809863679848518114974945973907740374226934555190660307553*u])", "taskResponseDigest": [124,88,144,120,223,114,210,191,244,116,251,75,207,16,111,199,239,103,125,141,21,137,67,12,225,221,167,57,252,5,27,244], "blsSignature": "E([21626555161416598641036571126028761885787781285664430506687716402728929287940,15599830259321898912464993223807627531962436563254314861362290908168788284248])"}

//////////////////////////////////////////////////////////////////////////////////////

	// 0xf7d7444890efd642e2e950e80f464c435864e9e40069f6adc76d76618d85cbb6
	// avs-finalizer-1   | (16124680927130488407178248525583896693158375145599831488279344043531389803014, 11823540798131174526939843546396410136978352664340557338429507352260791128215)
	// avs-finalizer-1   | SignedTaskResponse { task_response: TaskResponseWire { reference_task_index: 1, operators_state_hash: [7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7],block_hash:[184,233,46,188,142,175,20,67,118,248,196,26,221,230,213,239,130,112,42,35,139,208,185,48,228,196,110,134,147,224,117,188],storage_proof_hash:[215,10,174,130,186,243,225,21,114,139,105,191,129,34,121,40,228,169,181,92,244,86,149,124,213,117,112,248,193,61,122,208],pending_state_hash:[31,188,19,31,78,175,205,220,101,13,225,81,155,55,247,31,107,154,134,69,35,200,63,22,57,47,71,152,204,46,185,25] }, bls_signature: BlsSignatureWire { g1_point: G1PointWire { x: BigInt([12809293238116456966, 12381830049062475940, 14974206346316353316, 2568809875460331222]), y: BigInt([9909276949556383895, 4830677788045925473, 13011330337496357747, 1883598720644731295]) } }, operator_id: [64, 194, 20, 129, 120, 127, 114, 185, 148, 67, 54, 218, 91, 82, 54, 146, 42, 182, 121, 199, 62, 21, 136, 136, 221, 5, 137, 20, 48, 196, 195, 165] }
	// avs-aggregator-1  | 2024-06-04T14:39:57.477Z	INFO	avs-aggregator/rpc_server.go:84	Received signed task response	{"response": {"TaskResponse":{"ReferenceTaskIndex":1,"OperatorsStateHash":[7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7],"BlockHash":[184,233,46,188,142,175,20,67,118,248,196,26,221,230,213,239,130,112,42,35,139,208,185,48,228,196,110,134,147,224,117,188],"StorageProofHash":[215,10,174,130,186,243,225,21,114,139,105,191,129,34,121,40,228,169,181,92,244,86,149,124,213,117,112,248,193,61,122,208],"PendingStateHash":[31,188,19,31,78,175,205,220,101,13,225,81,155,55,247,31,107,154,134,69,35,200,63,22,57,47,71,152,204,46,185,25]},"BlsSignature":{"g1_point":{"X":"16124680927130488407178248525583896693158375145599831488279344043531389803014","Y":"11823540798131174526939843546396410136978352664340557338429507352260791128215"}},"OperatorId":[64,194,20,129,120,127,114,185,148,67,54,218,91,82,54,146,42,182,121,199,62,21,136,136,221,5,137,20,48,196,195,165]}, "operatorId": "40c21481787f72b9944336da5b5236922ab679c73e158888dd05891430c4c3a5"}
	// avs-aggregator-1  | 2024-06-04T14:39:57.478Z	DEBUG	bls_aggregation/blsagg.go:252	Task goroutine received new signed task response digest	{"taskIndex": 1, "signedTaskResponseDigest": {"TaskResponseDigest":[243,198,93,230,220,137,169,246,81,35,176,128,203,185,244,139,59,169,136,156,160,78,213,4,146,85,245,238,202,147,74,222],"BlsSignature":{"g1_point":{"X":"16124680927130488407178248525583896693158375145599831488279344043531389803014","Y":"11823540798131174526939843546396410136978352664340557338429507352260791128215"}},"OperatorId":[64,194,20,129,120,127,114,185,148,67,54,218,91,82,54,146,42,182,121,199,62,21,136,136,221,5,137,20,48,196,195,165]}}
	// avs-aggregator-1  | 2024-06-04T14:39:57.478Z	DEBUG	bls_aggregation/blsagg.go:407	Verifying signed task response digest signature	{"operatorG2Pubkey": "E([1204848068762182265191850104053210228094357639931323501744012507352651191715+3437905672118404638218691649552543807038332409265562544360613743931029465701*u,15562454116867540509343966090314294754420286176618755869518765676848225112837+8065639762024274629012319107489312720603226320349447376560419440986343714665*u])", "taskResponseDigest": [243,198,93,230,220,137,169,246,81,35,176,128,203,185,244,139,59,169,136,156,160,78,213,4,146,85,245,238,202,147,74,222], "blsSignature": "E([16124680927130488407178248525583896693158375145599831488279344043531389803014,11823540798131174526939843546396410136978352664340557338429507352260791128215])"}

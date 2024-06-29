package chainio

import (
	"context"
	"math/big"
	"strings"

	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	logging "github.com/Layr-Labs/eigensdk-go/logging"

	taskmanager "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/FinalizerTaskManager"
)

type AvsReaderer interface {
	CheckSignatures(
		ctx context.Context, msgHash [32]byte, quorumNumbers []byte, referenceBlockNumber uint32, nonSignerStakesAndSignature taskmanager.IBLSSignatureCheckerNonSignerStakesAndSignature,
	) (taskmanager.IBLSSignatureCheckerQuorumStakeTotals, error)

	GetTaskRespondedEvents(ctx context.Context, blocksAgo uint32) ([]taskmanager.ContractFinalizerTaskManagerTaskResponded, error)

	GetNonSigningOperatorPubKeys(event taskmanager.ContractFinalizerTaskManagerTaskResponded) ([]*bls.G1Point, error)
}

type AvsReader struct {
	AvsServiceBindings *AvsServiceBindings
	logger             logging.Logger
}

var _ AvsReaderer = (*AvsReader)(nil)

func NewAvsReaderFromConfig(
	registry common.Address,
	ethClient eth.Client,
	logger logging.Logger,
) (*AvsReader, error) {
	avsServiceBindings, err := NewAvsServiceBindings(registry, ethClient, logger)
	if err != nil {
		return nil, err
	}

	return NewAvsReader(avsServiceBindings, logger)
}

func NewAvsReader(avsServiceBindings *AvsServiceBindings, logger logging.Logger) (*AvsReader, error) {
	return &AvsReader{
		AvsServiceBindings: avsServiceBindings,
		logger:             logger,
	}, nil
}

func (r *AvsReader) CheckSignatures(
	ctx context.Context, msgHash [32]byte, quorumNumbers []byte, referenceBlockNumber uint32, nonSignerStakesAndSignature taskmanager.IBLSSignatureCheckerNonSignerStakesAndSignature,
) (taskmanager.IBLSSignatureCheckerQuorumStakeTotals, error) {
	stakeTotalsPerQuorum, _, err := r.AvsServiceBindings.TaskManager.CheckSignatures(
		&bind.CallOpts{}, msgHash, quorumNumbers, referenceBlockNumber, nonSignerStakesAndSignature,
	)
	if err != nil {
		return taskmanager.IBLSSignatureCheckerQuorumStakeTotals{}, err
	}
	return stakeTotalsPerQuorum, nil
}

func (r *AvsReader) GetTaskRespondedEvents(ctx context.Context, blocksAgo uint32) ([]taskmanager.ContractFinalizerTaskManagerTaskResponded, error) {
	events := []taskmanager.ContractFinalizerTaskManagerTaskResponded{}

	currentBlock, err := r.AvsServiceBindings.ethClient.BlockNumber(ctx)
	if err != nil {
		r.logger.Error("Cannot get current block number", "err", err)
		return nil, err
	}
	opts := bind.FilterOpts{Start: currentBlock - uint64(blocksAgo), End: &currentBlock, Context: ctx}
	r.logger.Debug("Getting FilterTaskResponded", "opts", opts)
	it, err := r.AvsServiceBindings.TaskManager.FilterTaskResponded(&opts, []uint32{})
	if err != nil {
		return nil, err
	}
	for it.Next() {
		events = append(events, *it.Event)
	}
	if it.Error() != nil {
		return nil, err
	}

	return events, nil
}

func (r *AvsReader) GetNonSigningOperatorPubKeys(event taskmanager.ContractFinalizerTaskManagerTaskResponded) ([]*bls.G1Point, error) {
	// r.logger.Debug("event.Raw is", "event.Raw", event.Raw)

	// get the nonSignerStakesAndSignature
	txHash := event.Raw.TxHash
	// r.logger.Debug("txHash", "txHash", txHash)
	tx, _, err := r.AvsServiceBindings.ethClient.TransactionByHash(context.Background(), txHash)
	_ = tx
	if err != nil {
		r.logger.Error("Error getting transaction by hash",
			"txHash", txHash,
			"err", err,
		)
		return nil, err
	}
	calldata := tx.Data()
	// r.logger.Debug("calldata", "calldata", calldata)
	cstmAbi, err := abi.JSON(strings.NewReader(taskmanager.ContractFinalizerTaskManagerABI))
	if err != nil {
		r.logger.Error("Error getting Abi", "err", err)
		return nil, err
	}
	methodSig := calldata[:4]
	// r.logger.Debug("methodSig", "methodSig", methodSig)
	method, err := cstmAbi.MethodById(methodSig)
	if err != nil {
		r.logger.Error("Error getting method", "err", err)
		return nil, err
	}
	
	inputs, err := method.Inputs.Unpack(calldata[4:])
	if err != nil {
		r.logger.Error("Error unpacking calldata", "err", err)
		return nil, err
	}

	// r.logger.Debug("inputs", "inputs", inputs)
	nonSignerStakesAndSignatureInput := inputs[2].(struct {
		NonSignerQuorumBitmapIndices []uint32 "json:\"nonSignerQuorumBitmapIndices\""
		NonSignerPubkeys             []struct {
			X *big.Int "json:\"X\""
			Y *big.Int "json:\"Y\""
		} "json:\"nonSignerPubkeys\""
		QuorumApks []struct {
			X *big.Int "json:\"X\""
			Y *big.Int "json:\"Y\""
		} "json:\"quorumApks\""
		ApkG2 struct {
			X [2]*big.Int "json:\"X\""
			Y [2]*big.Int "json:\"Y\""
		} "json:\"apkG2\""
		Sigma struct {
			X *big.Int "json:\"X\""
			Y *big.Int "json:\"Y\""
		} "json:\"sigma\""
		QuorumApkIndices      []uint32   "json:\"quorumApkIndices\""
		TotalStakeIndices     []uint32   "json:\"totalStakeIndices\""
		NonSignerStakeIndices [][]uint32 "json:\"nonSignerStakeIndices\""
	})

	nonSigningOperatorPubKeys := make([]*bls.G1Point, len(nonSignerStakesAndSignatureInput.NonSignerPubkeys))
	for i, pubkey := range nonSignerStakesAndSignatureInput.NonSignerPubkeys {
		nonSigningOperatorPubKeys[i] = bls.NewG1Point(pubkey.X, pubkey.Y)
	}

	// r.logger.Debug("nonSigningOperatorPubKeys", "nonSigningOperatorPubKeys", nonSigningOperatorPubKeys)
	return nonSigningOperatorPubKeys, nil
}
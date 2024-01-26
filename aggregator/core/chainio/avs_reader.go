package chainio

import (
	"context"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	logging "github.com/Layr-Labs/eigensdk-go/logging"

	taskmanager "github.com/mangata-finance/eigen-layer-monorepo/aggregator/bindings/MangataTaskManager"
)

type AvsReaderer interface {
	CheckSignatures(
		ctx context.Context, msgHash [32]byte, quorumNumbers []byte, referenceBlockNumber uint32, nonSignerStakesAndSignature taskmanager.IBLSSignatureCheckerNonSignerStakesAndSignature,
	) (taskmanager.IBLSSignatureCheckerQuorumStakeTotals, error)
}

type AvsReader struct {
	AvsServiceBindings *AvsServiceBindings
	logger             logging.Logger
}

var _ AvsReaderer = (*AvsReader)(nil)

func NewAvsReaderFromConfig(
	serviceManager common.Address,
	blsOperatorStateRetriever common.Address,
	ethClient eth.EthClient,
	logger logging.Logger,
) (*AvsReader, error) {
	avsServiceBindings, err := NewAvsServiceBindings(serviceManager, blsOperatorStateRetriever, ethClient, logger)
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

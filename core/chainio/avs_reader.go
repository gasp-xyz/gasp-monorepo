package chainio

import (
	"context"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"

	sdkavsregistry "github.com/Layr-Labs/eigensdk-go/chainio/avsregistry"
	sdkclients "github.com/Layr-Labs/eigensdk-go/chainio/clients"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	logging "github.com/Layr-Labs/eigensdk-go/logging"

	taskmanager "github.com/mangata-finance/eigen-layer-monorepo/contracts/bindings/MangataTaskManager"
)

type AvsReaderer interface {
	sdkavsregistry.AvsRegistryReader

	CheckSignatures(
		ctx context.Context, msgHash [32]byte, quorumNumbers []byte, referenceBlockNumber uint32, nonSignerStakesAndSignature taskmanager.IBLSSignatureCheckerNonSignerStakesAndSignature,
	) (taskmanager.IBLSSignatureCheckerQuorumStakeTotals, error)
}

type AvsReader struct {
	sdkavsregistry.AvsRegistryReader
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

	avsContractBindings, err := NewAvsServiceBindings(serviceManager, blsOperatorStateRetriever, ethClient, logger)
	if err != nil {
		return nil, err
	}
	blsRegistryCoordinatorAddr, err := avsContractBindings.ServiceManager.RegistryCoordinator(&bind.CallOpts{})
	if err != nil {
		return nil, err
	}
	stakeRegistryAddr, err := avsContractBindings.ServiceManager.StakeRegistry(&bind.CallOpts{})
	if err != nil {
		return nil, err
	}
	blsPubkeyRegistryAddr, err := avsContractBindings.ServiceManager.BlsPubkeyRegistry(&bind.CallOpts{})
	if err != nil {
		return nil, err
	}
	avsRegistryContractClient, err := sdkclients.NewAvsRegistryContractsChainClient(blsRegistryCoordinatorAddr, blsOperatorStateRetriever, stakeRegistryAddr, blsPubkeyRegistryAddr, ethClient, logger)
	if err != nil {
		return nil, err
	}

	avsRegistryReader, err := sdkavsregistry.NewAvsRegistryReader(avsRegistryContractClient, logger, ethClient)
	if err != nil {
		return nil, err
	}
	avsServiceBindings, err := NewAvsServiceBindings(serviceManager, blsOperatorStateRetriever, ethClient, logger)
	if err != nil {
		return nil, err
	}

	return NewAvsReader(avsRegistryReader, avsServiceBindings, logger)
}

func NewAvsReader(avsRegistryReader sdkavsregistry.AvsRegistryReader, avsServiceBindings *AvsServiceBindings, logger logging.Logger) (*AvsReader, error) {
	return &AvsReader{
		AvsRegistryReader:  avsRegistryReader,
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

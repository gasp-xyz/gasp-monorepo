package chainio

import (
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/logging"
	regcoord "github.com/Layr-Labs/eigensdk-go/contracts/bindings/RegistryCoordinator"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"

	servicemanager "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/FinalizerServiceManager"
	taskmanager "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/FinalizerTaskManager"
	blsSignatureChecker "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/BLSSignatureChecker"
)

type AvsServiceBindings struct {
	TaskManager            *taskmanager.ContractFinalizerTaskManager
	ServiceManager         *servicemanager.ContractFinalizerServiceManager
	BlsSignatureChecker         *blsSignatureChecker.ContractBLSSignatureChecker
	OperatorStateRetriever common.Address
	ethClient              eth.Client
	logger                 logging.Logger
}

func NewAvsServiceBindings(registryCoordinatorAddr common.Address, ethclient eth.Client, logger logging.Logger) (*AvsServiceBindings, error) {
	contractRegistryCoordinator, err := regcoord.NewContractRegistryCoordinator(registryCoordinatorAddr, ethclient)
	if err != nil {
		return nil, err
	}

	serviceManagerAddr, err := contractRegistryCoordinator.ServiceManager(&bind.CallOpts{})
	if err != nil {
		logger.Error("Failed to fetch ServiceManager address", "err", err)
		return nil, err
	}
	contractServiceManager, err := servicemanager.NewContractFinalizerServiceManager(serviceManagerAddr, ethclient)
	if err != nil {
		logger.Error("Failed to fetch IServiceManager contract", "err", err)
		return nil, err
	}

	taskManagerAddr, err := contractServiceManager.TaskManager(&bind.CallOpts{})
	if err != nil {
		logger.Error("Failed to fetch TaskManager address", "err", err)
		return nil, err
	}
	contractTaskManager, err := taskmanager.NewContractFinalizerTaskManager(taskManagerAddr, ethclient)
	if err != nil {
		logger.Error("Failed to fetch IIncredibleSquaringTaskManager contract", "err", err)
		return nil, err
	}


	blsSignatureCheckerAddr, err := contractTaskManager.BlsSignatureChecker(&bind.CallOpts{})
	if err != nil {
		logger.Error("Failed to fetch TaskManager address", "err", err)
		return nil, err
	}
	contractBlsSignatureChecker, err := blsSignatureChecker.NewContractBLSSignatureChecker(blsSignatureCheckerAddr, ethclient)
	if err != nil {
		logger.Error("Failed to fetch IIncredibleSquaringTaskManager contract", "err", err)
		return nil, err
	}

	return &AvsServiceBindings{
		ServiceManager:         contractServiceManager,
		TaskManager:            contractTaskManager,
		BlsSignatureChecker:            contractBlsSignatureChecker,
		OperatorStateRetriever: taskManagerAddr,
		ethClient:              ethclient,
		logger:                 logger,
	}, nil
}

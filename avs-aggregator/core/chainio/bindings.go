package chainio

import (
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/logging"
	regcoord "github.com/Layr-Labs/eigensdk-go/contracts/bindings/RegistryCoordinator"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"

	servicemanager "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/FinalizerServiceManager"
	taskmanager "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/FinalizerTaskManager"
)

type AvsServiceBindings struct {
	TaskManager            *taskmanager.ContractFinalizerTaskManager
	ServiceManager         *servicemanager.ContractFinalizerServiceManager
	OperatorStateRetriever common.Address
	ethClient              eth.EthClient
	logger                 logging.Logger
}

func NewAvsServiceBindings(registryCoordinatorAddr common.Address, ethclient eth.EthClient, logger logging.Logger) (*AvsServiceBindings, error) {
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

	return &AvsServiceBindings{
		ServiceManager:         contractServiceManager,
		TaskManager:            contractTaskManager,
		OperatorStateRetriever: taskManagerAddr,
		ethClient:              ethclient,
		logger:                 logger,
	}, nil
}

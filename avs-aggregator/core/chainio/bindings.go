package chainio

import (
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/logging"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	gethcommon "github.com/ethereum/go-ethereum/common"

	servicemanager "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/MangataServiceManager"
	taskmanager "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/MangataTaskManager"
)

type AvsServiceBindings struct {
	TaskManager         *taskmanager.ContractMangataTaskManager
	ServiceManager      *servicemanager.ContractMangataServiceManager
	ethClient           eth.EthClient
	logger              logging.Logger
	RegistryCoordinator common.Address
}

func NewAvsServiceBindings(serviceManagerAddr, blsOperatorStateRetrieverAddr gethcommon.Address, ethclient eth.EthClient, logger logging.Logger) (*AvsServiceBindings, error) {
	contractServiceManager, err := servicemanager.NewContractMangataServiceManager(serviceManagerAddr, ethclient)
	if err != nil {
		logger.Error("Failed to fetch IServiceManager contract", "err", err)
		return nil, err
	}

	taskManagerAddr, err := contractServiceManager.TaskManager(&bind.CallOpts{})
	if err != nil {
		logger.Error("Failed to fetch TaskManager address", "err", err)
		return nil, err
	}
	contractTaskManager, err := taskmanager.NewContractMangataTaskManager(taskManagerAddr, ethclient)
	if err != nil {
		logger.Error("Failed to fetch IIncredibleSquaringTaskManager contract", "err", err)
		return nil, err
	}

	registryCoordinatorAddr, err := contractServiceManager.RegistryCoordinator(&bind.CallOpts{})
	if err != nil {
		logger.Error("Failed to fetch RegistryCoordinator address", "err", err)
		return nil, err
	}

	return &AvsServiceBindings{
		ServiceManager:      contractServiceManager,
		TaskManager:         contractTaskManager,
		ethClient:           ethclient,
		logger:              logger,
		RegistryCoordinator: registryCoordinatorAddr,
	}, nil
}

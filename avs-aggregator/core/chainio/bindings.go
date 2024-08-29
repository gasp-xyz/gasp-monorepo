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
	delegationManager "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/DelegationManager"
	stakeRegistry "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/StakeRegistry"
	operatorStateRetrieverExtended "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/OperatorStateRetrieverExtended"
)

type AvsServiceBindings struct {
	RegistryCoordinator    *regcoord.ContractRegistryCoordinator
	TaskManager            *taskmanager.ContractFinalizerTaskManager
	ServiceManager         *servicemanager.ContractFinalizerServiceManager
	BlsSignatureChecker         *blsSignatureChecker.ContractBLSSignatureChecker
	StakeRegistry 			*stakeRegistry.ContractStakeRegistry
	DelegationManager      *delegationManager.ContractDelegationManager
	OperatorStateRetrieverExtended *operatorStateRetrieverExtended.ContractOperatorStateRetrieverExtended
	OperatorStateRetriever common.Address
	DelegationManagerAddress common.Address
	TaskManagerAddress common.Address
	StakeRegistryAddress common.Address 
	RegistryCoordinatorAddress common.Address
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
		logger.Error("Failed to fetch TaskManager contract", "err", err)
		return nil, err
	}


	stakeRegistryAddr, err := contractRegistryCoordinator.StakeRegistry(&bind.CallOpts{})
	if err != nil {
		logger.Error("Failed to fetch StakeRegistry address", "err", err)
		return nil, err
	}
	contractStakeRegistry, err := stakeRegistry.NewContractStakeRegistry(stakeRegistryAddr, ethclient)
	if err != nil {
		logger.Error("Failed to fetch StakeRegistry contract", "err", err)
		return nil, err
	}

	delegationManagerAddr, err := contractStakeRegistry.Delegation(&bind.CallOpts{})
	if err != nil {
		logger.Error("Failed to fetch DelegationManager address", "err", err)
		return nil, err
	}
	contractDelegationManager, err := delegationManager.NewContractDelegationManager(delegationManagerAddr, ethclient)
	if err != nil {
		logger.Error("Failed to fetch DelegationManager contract", "err", err)
		return nil, err
	}

	operatorStateRetrieverExtendedAddr, err := contractTaskManager.OperatorStateRetrieverExtended(&bind.CallOpts{})
	if err != nil {
		logger.Error("Failed to fetch OperatorStateRetrieverExtended address", "err", err)
		return nil, err
	}
	contractOperatorStateRetrieverExtended, err := operatorStateRetrieverExtended.NewContractOperatorStateRetrieverExtended(operatorStateRetrieverExtendedAddr, ethclient)
	if err != nil {
		logger.Error("Failed to fetch OperatorStateRetrieverExtended contract", "err", err)
		return nil, err
	}

	return &AvsServiceBindings{
		RegistryCoordinator: contractRegistryCoordinator,
		ServiceManager:         contractServiceManager,
		TaskManager:            contractTaskManager,
		BlsSignatureChecker:            contractBlsSignatureChecker,
		StakeRegistry:            contractStakeRegistry,
		DelegationManager:            contractDelegationManager,
		OperatorStateRetrieverExtended: contractOperatorStateRetrieverExtended,
		OperatorStateRetriever: taskManagerAddr,
		DelegationManagerAddress: delegationManagerAddr,
		TaskManagerAddress: taskManagerAddr,
		StakeRegistryAddress: stakeRegistryAddr,
		RegistryCoordinatorAddress: registryCoordinatorAddr,
		ethClient:              ethclient,
		logger:                 logger,
	}, nil
}

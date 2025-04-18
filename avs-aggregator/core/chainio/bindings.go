package chainio

import (
	regcoord "github.com/Layr-Labs/eigensdk-go/contracts/bindings/RegistryCoordinator"
	"github.com/Layr-Labs/eigensdk-go/logging"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/ethclient"

	blsSignatureChecker "github.com/gasp-xyz/gasp-monorepo/avs-aggregator/bindings/BLSSignatureChecker"
	delegationManager "github.com/gasp-xyz/gasp-monorepo/avs-aggregator/bindings/DelegationManager"
	servicemanager "github.com/gasp-xyz/gasp-monorepo/avs-aggregator/bindings/FinalizerServiceManager"
	taskmanager "github.com/gasp-xyz/gasp-monorepo/avs-aggregator/bindings/FinalizerTaskManager"
	indexRegistry "github.com/gasp-xyz/gasp-monorepo/avs-aggregator/bindings/IndexRegistry"
	operatorStateRetrieverExtended "github.com/gasp-xyz/gasp-monorepo/avs-aggregator/bindings/OperatorStateRetrieverExtended"
	stakeRegistry "github.com/gasp-xyz/gasp-monorepo/avs-aggregator/bindings/StakeRegistry"
)

type AvsServiceBindings struct {
	RegistryCoordinator    *regcoord.ContractRegistryCoordinator
	TaskManager            *taskmanager.ContractFinalizerTaskManager
	ServiceManager         *servicemanager.ContractFinalizerServiceManager
	BlsSignatureChecker         *blsSignatureChecker.ContractBLSSignatureChecker
	StakeRegistry 			*stakeRegistry.ContractStakeRegistry
	IndexRegistry          *indexRegistry.ContractIndexRegistry 
	DelegationManager      *delegationManager.ContractDelegationManager
	OperatorStateRetrieverExtended *operatorStateRetrieverExtended.ContractOperatorStateRetrieverExtended
	OperatorStateRetriever common.Address
	DelegationManagerAddress common.Address
	TaskManagerAddress common.Address
	StakeRegistryAddress common.Address 
	RegistryCoordinatorAddress common.Address
	EthClient              *ethclient.Client
	logger                 logging.Logger
}

func NewAvsServiceBindings(registryCoordinatorAddr common.Address, EthClient *ethclient.Client, logger logging.Logger) (*AvsServiceBindings, error) {
	contractRegistryCoordinator, err := regcoord.NewContractRegistryCoordinator(registryCoordinatorAddr, EthClient)
	if err != nil {
		return nil, err
	}

	serviceManagerAddr, err := contractRegistryCoordinator.ServiceManager(&bind.CallOpts{})
	if err != nil {
		logger.Error("Failed to fetch ServiceManager address", "err", err)
		return nil, err
	}
	contractServiceManager, err := servicemanager.NewContractFinalizerServiceManager(serviceManagerAddr, EthClient)
	if err != nil {
		logger.Error("Failed to fetch IServiceManager contract", "err", err)
		return nil, err
	}

	taskManagerAddr, err := contractServiceManager.TaskManager(&bind.CallOpts{})
	if err != nil {
		logger.Error("Failed to fetch TaskManager address", "err", err)
		return nil, err
	}
	contractTaskManager, err := taskmanager.NewContractFinalizerTaskManager(taskManagerAddr, EthClient)
	if err != nil {
		logger.Error("Failed to fetch IIncredibleSquaringTaskManager contract", "err", err)
		return nil, err
	}


	blsSignatureCheckerAddr, err := contractTaskManager.BlsSignatureChecker(&bind.CallOpts{})
	if err != nil {
		logger.Error("Failed to fetch TaskManager address", "err", err)
		return nil, err
	}
	contractBlsSignatureChecker, err := blsSignatureChecker.NewContractBLSSignatureChecker(blsSignatureCheckerAddr, EthClient)
	if err != nil {
		logger.Error("Failed to fetch TaskManager contract", "err", err)
		return nil, err
	}


	stakeRegistryAddr, err := contractRegistryCoordinator.StakeRegistry(&bind.CallOpts{})
	if err != nil {
		logger.Error("Failed to fetch StakeRegistry address", "err", err)
		return nil, err
	}
	contractStakeRegistry, err := stakeRegistry.NewContractStakeRegistry(stakeRegistryAddr, EthClient)
	if err != nil {
		logger.Error("Failed to fetch StakeRegistry contract", "err", err)
		return nil, err
	}
	
	indexRegistryAddr, err := contractRegistryCoordinator.IndexRegistry(&bind.CallOpts{})
	if err != nil {
		logger.Error("Failed to fetch IndexRegistry address", "err", err)
		return nil, err
	}
	contractIndexRegistry, err := indexRegistry.NewContractIndexRegistry(indexRegistryAddr, EthClient)
	if err != nil {
		logger.Error("Failed to fetch IndexRegistry contract", "err", err)
		return nil, err
	}

	delegationManagerAddr, err := contractStakeRegistry.Delegation(&bind.CallOpts{})
	if err != nil {
		logger.Error("Failed to fetch DelegationManager address", "err", err)
		return nil, err
	}
	contractDelegationManager, err := delegationManager.NewContractDelegationManager(delegationManagerAddr, EthClient)
	if err != nil {
		logger.Error("Failed to fetch DelegationManager contract", "err", err)
		return nil, err
	}

	operatorStateRetrieverExtendedAddr, err := contractTaskManager.OperatorStateRetrieverExtended(&bind.CallOpts{})
	if err != nil {
		logger.Error("Failed to fetch OperatorStateRetrieverExtended address", "err", err)
		return nil, err
	}
	contractOperatorStateRetrieverExtended, err := operatorStateRetrieverExtended.NewContractOperatorStateRetrieverExtended(operatorStateRetrieverExtendedAddr, EthClient)
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
		IndexRegistry:            contractIndexRegistry,
		DelegationManager:            contractDelegationManager,
		OperatorStateRetrieverExtended: contractOperatorStateRetrieverExtended,
		OperatorStateRetriever: taskManagerAddr,
		DelegationManagerAddress: delegationManagerAddr,
		TaskManagerAddress: taskManagerAddr,
		StakeRegistryAddress: stakeRegistryAddr,
		RegistryCoordinatorAddress: registryCoordinatorAddr,
		EthClient:              EthClient,
		logger:                 logger,
	}, nil
}

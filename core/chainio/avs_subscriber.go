package chainio

import (
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	gethcommon "github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/event"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"

	taskmanager "github.com/mangata-finance/eigen-layer-monorepo/contracts/bindings/MangataTaskManager"
)

type AvsSubscriberer interface {
	SubscribeToNewTasks(newTaskCreatedChan chan *taskmanager.ContractMangataTaskManagerNewTaskCreated) event.Subscription
	SubscribeToTaskResponses(taskResponseLogs chan *taskmanager.ContractMangataTaskManagerTaskResponded) event.Subscription
	ParseTaskResponded(rawLog types.Log) (*taskmanager.ContractMangataTaskManagerTaskResponded, error)
}

// Subscribers use a ws connection instead of http connection like Readers
// kind of stupid that the geth client doesn't have a unified interface for both...
// it takes a single url, so the bindings, even though they have watcher functions, those can't be used
// with the http connection... seems very very stupid. Am I missing something?
type AvsSubscriber struct {
	AvsContractBindings *AvsServiceBindings
	logger              sdklogging.Logger
}

func NewAvsSubscriber(serviceManagerAddr, blsOperatorStateRetrieverAddr gethcommon.Address, ethclient eth.EthClient, logger sdklogging.Logger) (*AvsSubscriber, error) {
	avsContractBindings, err := NewAvsServiceBindings(serviceManagerAddr, blsOperatorStateRetrieverAddr, ethclient, logger)
	if err != nil {
		logger.Errorf("Failed to create contract bindings", "err", err)
		return nil, err
	}
	return &AvsSubscriber{
		AvsContractBindings: avsContractBindings,
		logger:              logger,
	}, nil
}

func (s *AvsSubscriber) SubscribeToNewTasks(newTaskCreatedChan chan *taskmanager.ContractMangataTaskManagerNewTaskCreated) event.Subscription {
	sub, err := s.AvsContractBindings.TaskManager.WatchNewTaskCreated(
		&bind.WatchOpts{}, newTaskCreatedChan, nil,
	)
	if err != nil {
		s.logger.Error("Failed to subscribe to new TaskManager tasks", "err", err)
	}
	s.logger.Infof("Subscribed to new TaskManager tasks")
	return sub
}

func (s *AvsSubscriber) SubscribeToTaskResponses(taskResponseChan chan *taskmanager.ContractMangataTaskManagerTaskResponded) event.Subscription {
	sub, err := s.AvsContractBindings.TaskManager.WatchTaskResponded(
		&bind.WatchOpts{}, taskResponseChan,
	)
	if err != nil {
		s.logger.Error("Failed to subscribe to TaskResponded events", "err", err)
	}
	s.logger.Infof("Subscribed to TaskResponded events")
	return sub
}

func (s *AvsSubscriber) ParseTaskResponded(rawLog types.Log) (*taskmanager.ContractMangataTaskManagerTaskResponded, error) {
	return s.AvsContractBindings.TaskManager.ContractMangataTaskManagerFilterer.ParseTaskResponded(rawLog)
}

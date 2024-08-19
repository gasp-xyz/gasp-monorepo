package chainio

import (
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	gethcommon "github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/event"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"

	taskmanager "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/FinalizerTaskManager"
)

type AvsSubscriberer interface {
	SubscribeToNewRdTasks(newTaskCreatedChan chan *taskmanager.ContractFinalizerTaskManagerNewRdTaskCreated) event.Subscription
	SubscribeToRdTaskResponses(taskResponseLogs chan *taskmanager.ContractFinalizerTaskManagerRdTaskResponded) event.Subscription
	ParseRdTaskResponded(rawLog types.Log) (*taskmanager.ContractFinalizerTaskManagerRdTaskResponded, error)
	SubscribeToOpTaskCompleted(opTaskCompletionLogs chan *taskmanager.ContractFinalizerTaskManagerOpTaskCompleted) event.Subscription
	SubscribeToResumeTrackingOpState(resumeLogs chan *taskmanager.ContractFinalizerTaskManagerResumeTrackingOpState) event.Subscription 
}

// Subscribers use a ws connection instead of http connection like Readers
// kind of stupid that the geth client doesn't have a unified interface for both...
// it takes a single url, so the bindings, even though they have watcher functions, those can't be used
// with the http connection... seems very very stupid. Am I missing something?
type AvsSubscriber struct {
	AvsContractBindings *AvsServiceBindings
	logger              sdklogging.Logger
}

func NewAvsSubscriber(registryAddr gethcommon.Address, ethclient eth.Client, logger sdklogging.Logger) (*AvsSubscriber, error) {
	avsContractBindings, err := NewAvsServiceBindings(registryAddr, ethclient, logger)
	if err != nil {
		logger.Errorf("Failed to create contract bindings", "err", err)
		return nil, err
	}
	return &AvsSubscriber{
		AvsContractBindings: avsContractBindings,
		logger:              logger,
	}, nil
}

func (s *AvsSubscriber) SubscribeToNewRdTasks(newTaskCreatedChan chan *taskmanager.ContractFinalizerTaskManagerNewRdTaskCreated) event.Subscription {
	sub, err := s.AvsContractBindings.TaskManager.WatchNewRdTaskCreated(
		&bind.WatchOpts{}, newTaskCreatedChan, nil,
	)
	if err != nil {
		s.logger.Error("Failed to subscribe to new TaskManager tasks", "err", err)
	}
	s.logger.Infof("Subscribed to new TaskManager tasks")
	return sub
}

func (s *AvsSubscriber) SubscribeToRdTaskResponses(taskResponseLogs chan *taskmanager.ContractFinalizerTaskManagerRdTaskResponded) event.Subscription {
	sub, err := s.AvsContractBindings.TaskManager.WatchRdTaskResponded(
		&bind.WatchOpts{}, taskResponseLogs, []uint32{},
	)
	if err != nil {
		s.logger.Error("Failed to subscribe to TaskResponded events", "err", err)
	}
	s.logger.Infof("Subscribed to TaskResponded events")
	return sub
}

func (s *AvsSubscriber) SubscribeToOpTaskCompleted(fromBlock uint64,opTaskCompletionLogs chan *taskmanager.ContractFinalizerTaskManagerOpTaskCompleted) event.Subscription {
	sub, err := s.AvsContractBindings.TaskManager.WatchOpTaskCompleted(
		&bind.WatchOpts{Start: fromBlock}, opTaskCompletionLogs, []uint32{},
	)
	if err != nil {
		s.logger.Error("Failed to subscribe to OpTaskCompleted events", "err", err)
	}
	s.logger.Infof("Subscribed to OpTaskCompleted events")
	return sub
}

func (s *AvsSubscriber) SubscribeToResumeTrackingOpState(resumeLogs chan *taskmanager.ContractFinalizerTaskManagerResumeTrackingOpState) event.Subscription {
	sub, err := s.AvsContractBindings.TaskManager.WatchResumeTrackingOpState(
		&bind.WatchOpts{}, resumeLogs, []uint32{},
	)
	if err != nil {
		s.logger.Error("Failed to subscribe to ResumeTrackingOpState events", "err", err)
	}
	s.logger.Infof("Subscribed to ResumeTrackingOpState events")
	return sub
}

func (s *AvsSubscriber) ParseRdTaskResponded(rawLog types.Log) (*taskmanager.ContractFinalizerTaskManagerRdTaskResponded, error) {
	return s.AvsContractBindings.TaskManager.ContractFinalizerTaskManagerFilterer.ParseRdTaskResponded(rawLog)
}

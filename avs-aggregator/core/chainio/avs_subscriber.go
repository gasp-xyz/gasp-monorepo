package chainio

import (
	"time"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	gethcommon "github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/ethereum/go-ethereum/event"

	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"

	taskmanager "github.com/gasp-xyz/gasp-monorepo/avs-aggregator/bindings/FinalizerTaskManager"
	stakeRegistry "github.com/gasp-xyz/gasp-monorepo/avs-aggregator/bindings/StakeRegistry"

	"context"
	"github.com/ethereum/go-ethereum"
	chain "github.com/ethereum/go-ethereum/core/types"
)

type AvsSubscriberer interface {
	StreamQueryWithHistory(ctx context.Context, q *ethereum.FilterQuery) (chan chain.Log, ethereum.Subscription, error)
	
	SubscribeToNewRdTasks(newTaskCreatedChan chan *taskmanager.ContractFinalizerTaskManagerNewRdTaskCreated) event.Subscription
	
	SubscribeToRdTaskResponses(taskResponseLogs chan *taskmanager.ContractFinalizerTaskManagerRdTaskResponded) event.Subscription
	
	SubscribeToOpTaskCompleted(opTaskCompletionLogs chan *taskmanager.ContractFinalizerTaskManagerOpTaskCompleted) (event.Subscription, error)
	
	SubscribeToOperatorStakeUpdate(updateLogs chan *stakeRegistry.ContractStakeRegistryOperatorStakeUpdate) (event.Subscription, error)
	
	ParseRdTaskResponded(rawLog types.Log) (*taskmanager.ContractFinalizerTaskManagerRdTaskResponded, error)
}

var _ AvsSubscriberer = (*AvsSubscriber)(nil)

// Subscribers use a ws connection instead of http connection like Readers
// kind of stupid that the geth client doesn't have a unified interface for both...
// it takes a single url, so the bindings, even though they have watcher functions, those can't be used
// with the http connection... seems very very stupid. Am I missing something?
type AvsSubscriber struct {
	AvsContractBindings *AvsServiceBindings
	StreamSubscriber *StreamReader
	logger              sdklogging.Logger
}

func NewAvsSubscriber(registryAddr gethcommon.Address, ethclient *ethclient.Client, logger sdklogging.Logger, aggSSFetchTimeout int) (*AvsSubscriber, error) {
	avsContractBindings, err := NewAvsServiceBindings(registryAddr, ethclient, logger)
	if err != nil {
		logger.Errorf("Failed to create contract bindings", "err", err)
		return nil, err
	}
	streamSubscriber := StreamReader{
		Backend: ethclient,
		FetchTimeout: time.Duration(aggSSFetchTimeout) * time.Second,
	}
	return &AvsSubscriber{
		AvsContractBindings: avsContractBindings,
		StreamSubscriber:  &streamSubscriber,
		logger:              logger,
	}, nil
}

func (s *AvsSubscriber) StreamQueryWithHistory(ctx context.Context, q *ethereum.FilterQuery) (chan chain.Log, ethereum.Subscription, error) {
	rawLogsC, sub, err := s.StreamSubscriber.StreamQueryWithHistory(ctx, q)
	return rawLogsC, sub, err
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

func (s *AvsSubscriber) SubscribeToOpTaskCompleted(opTaskCompletionLogs chan *taskmanager.ContractFinalizerTaskManagerOpTaskCompleted) (event.Subscription, error) {
	sub, err := s.AvsContractBindings.TaskManager.WatchOpTaskCompleted(
		&bind.WatchOpts{}, opTaskCompletionLogs, []uint32{},
	)
	if err != nil {
		s.logger.Error("Failed to subscribe to OpTaskCompleted events", "err", err)
	}
	s.logger.Infof("Subscribed to OpTaskCompleted events")
	return sub, err
}

func (s *AvsSubscriber) SubscribeToOperatorStakeUpdate(updateLogs chan *stakeRegistry.ContractStakeRegistryOperatorStakeUpdate) (event.Subscription, error) {
	sub, err := s.AvsContractBindings.StakeRegistry.WatchOperatorStakeUpdate(
		&bind.WatchOpts{}, updateLogs, [][32]byte{},
	)
	if err != nil {
		s.logger.Error("Failed to subscribe to OperatorStakeUpdate events", "err", err)
	}
	s.logger.Infof("Subscribed to OperatorStakeUpdate events")
	return sub, err
}

func (s *AvsSubscriber) ParseRdTaskResponded(rawLog types.Log) (*taskmanager.ContractFinalizerTaskManagerRdTaskResponded, error) {
	return s.AvsContractBindings.TaskManager.ContractFinalizerTaskManagerFilterer.ParseRdTaskResponded(rawLog)
}

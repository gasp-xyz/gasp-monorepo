package chainio

import (
	"context"
	"errors"
	"math/big"

	"github.com/ethereum/go-ethereum/common"
	gethcommon "github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/chainio/txmgr"
	logging "github.com/Layr-Labs/eigensdk-go/logging"
	sdktypes "github.com/Layr-Labs/eigensdk-go/types"

	taskmanager "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/FinalizerTaskManager"
)

type AvsWriterer interface {
	SendNewRdTask(
		ctx context.Context,
		blockNumber *big.Int,
	) (taskmanager.IFinalizerTaskManagerRdTask, uint32, error)
	SendNewOpTask(ctx context.Context,
		quorumThresholdPercentage uint32,
		quorumNumbers sdktypes.QuorumNums,
	) (taskmanager.IFinalizerTaskManagerOpTask, uint32, error)
	SendAggregatedOpTaskResponse(ctx context.Context, task taskmanager.IFinalizerTaskManagerOpTask, taskResponse taskmanager.IFinalizerTaskManagerOpTaskResponse, nonSignerStakesAndSignature taskmanager.IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Receipt, error)
	SendAggregatedRdTaskResponse(ctx context.Context, task taskmanager.IFinalizerTaskManagerRdTask, taskResponse taskmanager.IFinalizerTaskManagerRdTaskResponse, nonSignerStakesAndSignature taskmanager.IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Receipt, error)
	EjectOperators(
		ctx context.Context,
		operators []common.Address,
		quorumNumbers [][]byte,
	) (*types.Receipt, error)
}

type AvsWriter struct {
	AvsContractBindings *AvsServiceBindings
	txMgr               txmgr.TxManager
	logger              logging.Logger
}

func NewAvsWriter(txMgr txmgr.TxManager, registryAddr gethcommon.Address, ethHttpClient eth.Client, logger logging.Logger) (*AvsWriter, error) {
	avsServiceBindings, err := NewAvsServiceBindings(registryAddr, ethHttpClient, logger)
	if err != nil {
		logger.Error("Failed to create contract bindings", "err", err)
		return nil, err
	}

	return &AvsWriter{
		AvsContractBindings: avsServiceBindings,
		logger:              logger,
		txMgr:               txMgr,
	}, nil
}

// returns the tx receipt, as well as the task index (which it gets from parsing the tx receipt logs)
func (w *AvsWriter) SendNewOpTask(ctx context.Context, quorumThresholdPercentage uint32, quorumNumbers sdktypes.QuorumNums) (taskmanager.IFinalizerTaskManagerOpTask, uint32, error) {
	w.logger.Info("creating new task with AVS's task manager")
	noSendTxOpts, err := w.txMgr.GetNoSendTxOpts()
	if err != nil {
		return taskmanager.IFinalizerTaskManagerOpTask{}, 0, err
	}
	tx, err := w.AvsContractBindings.TaskManager.CreateNewOpTask(noSendTxOpts, quorumThresholdPercentage, quorumNumbers.UnderlyingType())
	if err != nil {
		w.logger.Errorf("Error assembling CreateNewOpTask tx")
		return taskmanager.IFinalizerTaskManagerOpTask{}, 0, err
	}

	receipt, err := w.txMgr.Send(ctx, tx)
	if err != nil {
		return taskmanager.IFinalizerTaskManagerOpTask{}, 0, errors.New("failed to send tx with err: " + err.Error())
	}
	if r.status == 0{
		return taskmanager.IFinalizerTaskManagerOpTask{}, 0, errors.New("Txn failed with status failure (0): " + r)
	}
	w.logger.Infof("tx hash: %s", receipt.TxHash.String())
	w.logger.Info("sent new task with the AVS's task manager")
	newTaskCreatedEvent, err := w.AvsContractBindings.TaskManager.ContractFinalizerTaskManagerFilterer.ParseNewOpTaskCreated(*receipt.Logs[0])
	if err != nil {
		w.logger.Error("Aggregator failed to parse new task created event", "err", err)
		return taskmanager.IFinalizerTaskManagerOpTask{}, 0, err
	}
	return newTaskCreatedEvent.Task, newTaskCreatedEvent.TaskIndex, nil
}

// returns the tx receipt, as well as the task index (which it gets from parsing the tx receipt logs)
func (w *AvsWriter) SendNewRdTask(ctx context.Context, blockNumber *big.Int) (taskmanager.IFinalizerTaskManagerRdTask, uint32, error) {
	w.logger.Info("creating new task with AVS's task manager")
	noSendTxOpts, err := w.txMgr.GetNoSendTxOpts()
	if err != nil {
		return taskmanager.IFinalizerTaskManagerRdTask{}, 0, err
	}
	tx, err := w.AvsContractBindings.TaskManager.CreateNewRdTask(noSendTxOpts, blockNumber)
	if err != nil {
		w.logger.Errorf("Error assembling CreateNewRdTask tx")
		return taskmanager.IFinalizerTaskManagerRdTask{}, 0, err
	}

	receipt, err := w.txMgr.Send(ctx, tx)
	if err != nil {
		return taskmanager.IFinalizerTaskManagerRdTask{}, 0, errors.New("failed to send tx with err: " + err.Error())
	}
	if r.status == 0{
		return taskmanager.IFinalizerTaskManagerRdTask{}, 0, errors.New("Txn failed with status failure (0): " + r)
	}
	w.logger.Infof("tx hash: %s", receipt.TxHash.String())
	w.logger.Info("sent new task with the AVS's task manager")
	newTaskCreatedEvent, err := w.AvsContractBindings.TaskManager.ContractFinalizerTaskManagerFilterer.ParseNewRdTaskCreated(*receipt.Logs[0])
	if err != nil {
		w.logger.Error("Aggregator failed to parse new task created event", "err", err)
		return taskmanager.IFinalizerTaskManagerRdTask{}, 0, err
	}
	return newTaskCreatedEvent.Task, newTaskCreatedEvent.TaskIndex, nil
}

func (w *AvsWriter) SendAggregatedOpTaskResponse(ctx context.Context, task taskmanager.IFinalizerTaskManagerOpTask, taskResponse taskmanager.IFinalizerTaskManagerOpTaskResponse, nonSignerStakesAndSignature taskmanager.IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Receipt, error) {
	w.logger.Info("sending aggregated task response with the AVS's task manager")
	noSendTxOpts, err := w.txMgr.GetNoSendTxOpts()
	if err != nil {
		return nil, err
	}
	tx, err := w.AvsContractBindings.TaskManager.RespondToOpTask(noSendTxOpts, task, taskResponse, nonSignerStakesAndSignature)
	if err != nil {
		w.logger.Errorf("Error assembling RespondToOpTask tx")
		return nil, err
	}

	receipt, err := w.txMgr.Send(ctx, tx)
	if err != nil {
		return nil, errors.New("failed to send tx with err: " + err.Error())
	}
	if r.status == 0{
		return nil, errors.New("Txn failed with status failure (0): " + r)
	}
	w.logger.Infof("tx hash: %s", receipt.TxHash.String())
	w.logger.Info("sent aggregated response with the AVS's task manager")
	return receipt, nil
}


func (w *AvsWriter) SendAggregatedRdTaskResponse(ctx context.Context, task taskmanager.IFinalizerTaskManagerRdTask, taskResponse taskmanager.IFinalizerTaskManagerRdTaskResponse, nonSignerStakesAndSignature taskmanager.IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Receipt, error) {
	w.logger.Info("sending aggregated task response with the AVS's task manager")
	noSendTxOpts, err := w.txMgr.GetNoSendTxOpts()
	if err != nil {
		return nil, err
	}
	tx, err := w.AvsContractBindings.TaskManager.RespondToRdTask(noSendTxOpts, task, taskResponse, nonSignerStakesAndSignature)
	if err != nil {
		w.logger.Errorf("Error assembling RespondToRdTask tx")
		return nil, err
	}

	receipt, err := w.txMgr.Send(ctx, tx)
	if err != nil {
		return nil, errors.New("failed to send tx with err: " + err.Error())
	}
	if r.status == 0{
		return nil, errors.New("Txn failed with status failure (0): " + r)
	}
	w.logger.Infof("tx hash: %s", receipt.TxHash.String())
	w.logger.Info("sent aggregated response with the AVS's task manager")
	return receipt, nil
}

func (w *AvsWriter) EjectOperators(ctx context.Context, operators []common.Address, quorumNumbers[][]uint8) (*types.Receipt, error) {
	w.logger.Info("sending eject operators with AVS's service manager")
	noSendTxOpts, err := w.txMgr.GetNoSendTxOpts()
	if err != nil {
		return nil, err
	}
	tx, err := w.AvsContractBindings.ServiceManager.EjectOperators(noSendTxOpts, operators, quorumNumbers)
	if err != nil {
		w.logger.Errorf("Error assembling RespondToTask tx")
		return nil, err
	}

	receipt, err := w.txMgr.Send(ctx, tx)
	if err != nil {
		return nil, errors.New("failed to send tx with err: " + err.Error())
	}
	if r.status == 0{
		return nil, errors.New("Txn failed with status failure (0): " + r)
	}
	w.logger.Infof("tx hash: %s", receipt.TxHash.String())
	w.logger.Info("sent eject operators with AVS's service manager")
	return receipt, nil
}

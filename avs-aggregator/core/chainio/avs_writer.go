package chainio

import (
	"context"
	"errors"
	"math/big"
	"fmt"

	"github.com/ethereum/go-ethereum/common"
	gethcommon "github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/ethclient"

	"github.com/Layr-Labs/eigensdk-go/chainio/txmgr"
	logging "github.com/Layr-Labs/eigensdk-go/logging"
	sdktypes "github.com/Layr-Labs/eigensdk-go/types"

	taskmanager "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/FinalizerTaskManager"
)

const waitForReceipt = bool(true)

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

func NewAvsWriter(txMgr txmgr.TxManager, registryAddr gethcommon.Address, ethHttpClient *ethclient.Client, logger logging.Logger) (*AvsWriter, error) {
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

	receipt, err := w.txMgr.Send(ctx, tx, waitForReceipt)
	if err != nil {
		return taskmanager.IFinalizerTaskManagerOpTask{}, 0, errors.New("failed to send tx with err: " + err.Error())
	}
	if receipt.Status == 0{
		return taskmanager.IFinalizerTaskManagerOpTask{}, 0, fmt.Errorf("Txn failed with status failure (0): %v", receipt)
	}
	w.logger.Infof("tx hash: %s", receipt.TxHash.String())
	w.logger.Info("sent new task with the AVS's task manager")

	var event *taskmanager.ContractFinalizerTaskManagerNewOpTaskCreated
	var foundEvent bool
	for _, log := range receipt.Logs{
		event, err = w.AvsContractBindings.TaskManager.ContractFinalizerTaskManagerFilterer.ParseNewOpTaskCreated(*log)
		if err == nil {
			foundEvent = true
		}
	}
	
	if foundEvent == false {
		w.logger.Error("Aggregator failed to parse new task created op event", "err", err)
		return taskmanager.IFinalizerTaskManagerOpTask{}, 0, fmt.Errorf("Aggregator failed to parse new task op created event - err: %v", err)
	}
	return event.Task, event.TaskIndex, nil
}

// returns the tx receipt, as well as the task index (which it gets from parsing the tx receipt logs)
func (w *AvsWriter) SendNewRdTask(ctx context.Context, chainToUpdate uint8, chainBatchIdToUpdate uint32) (taskmanager.IFinalizerTaskManagerRdTask, uint32, error) {
	w.logger.Info("creating new task with AVS's task manager")
	noSendTxOpts, err := w.txMgr.GetNoSendTxOpts()
	if err != nil {
		return taskmanager.IFinalizerTaskManagerRdTask{}, 0, err
	}
	tx, err := w.AvsContractBindings.TaskManager.CreateNewRdTask(noSendTxOpts, chainToUpdate, chainBatchIdToUpdate)
	if err != nil {
		w.logger.Errorf("Error assembling CreateNewRdTask tx")
		return taskmanager.IFinalizerTaskManagerRdTask{}, 0, err
	}

	receipt, err := w.txMgr.Send(ctx, tx, waitForReceipt)
	if err != nil {
		return taskmanager.IFinalizerTaskManagerRdTask{}, 0, errors.New("failed to send tx with err: " + err.Error())
	}
	if receipt.Status == 0{
		return taskmanager.IFinalizerTaskManagerRdTask{}, 0, fmt.Errorf("Txn failed with status failure (0): %v", receipt)
	}
	w.logger.Infof("tx hash: %s", receipt.TxHash.String())
	w.logger.Info("sent new task with the AVS's task manager")

	var event *taskmanager.ContractFinalizerTaskManagerNewRdTaskCreated
	var foundEvent bool
	for _, log := range receipt.Logs{
		event, err = w.AvsContractBindings.TaskManager.ContractFinalizerTaskManagerFilterer.ParseNewRdTaskCreated(*log)
		if err == nil {
			foundEvent = true
		}
	}
	
	if foundEvent == false {
		w.logger.Error("Aggregator failed to parse new task rd created event", "err", err)
		return taskmanager.IFinalizerTaskManagerRdTask{}, 0, fmt.Errorf("Aggregator failed to parse new task rd created event - err: %v", err)
	}
	return event.Task, event.TaskIndex, nil
}

func (w *AvsWriter) CancelPendingTask(ctx context.Context) (error) {
	w.logger.Info("Cancelling pending task with AVS's task manager")
	noSendTxOpts, err := w.txMgr.GetNoSendTxOpts()
	if err != nil {
		return err
	}
	tx, err := w.AvsContractBindings.TaskManager.CancelPendingTasks(noSendTxOpts)
	if err != nil {
		w.logger.Errorf("Error assembling cancelPendingTasks tx")
		return err
	}

	receipt, err := w.txMgr.Send(ctx, tx, waitForReceipt)
	if err != nil {
		return errors.New("failed to send tx with err: " + err.Error())
	}
	if receipt.Status == 0{
		return fmt.Errorf("Txn failed with status failure (0): %v", receipt)
	}
	w.logger.Infof("tx hash: %s", receipt.TxHash.String())
	w.logger.Info("Pending task cancelled with the AVS's task manager")

	return nil
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

	receipt, err := w.txMgr.Send(ctx, tx, waitForReceipt)
	if err != nil {
		return nil, errors.New("failed to send tx with err: " + err.Error())
	}
	if receipt.Status == 0{
		return nil, fmt.Errorf("Txn failed with status failure (0): %v", receipt)
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

	receipt, err := w.txMgr.Send(ctx, tx, waitForReceipt)
	if err != nil {
		return nil, errors.New("failed to send tx with err: " + err.Error())
	}
	if receipt.Status == 0{
		return nil, fmt.Errorf("Txn failed with status failure (0): %v", receipt)
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

	receipt, err := w.txMgr.Send(ctx, tx, waitForReceipt)
	if err != nil {
		return nil, errors.New("failed to send tx with err: " + err.Error())
	}
	if receipt.Status == 0{
		return nil, fmt.Errorf("Txn failed with status failure (0): %v", receipt)
	}
	w.logger.Infof("tx hash: %s", receipt.TxHash.String())
	w.logger.Info("sent eject operators with AVS's service manager")
	return receipt, nil
}

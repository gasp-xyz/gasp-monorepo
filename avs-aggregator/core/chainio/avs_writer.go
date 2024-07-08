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
	SendNewTaskVerifyBlock(
		ctx context.Context,
		blockNumber *big.Int,
		quorumThresholdPercentage uint32,
		quorumNumbers sdktypes.QuorumNums,
	) (taskmanager.IFinalizerTaskManagerTask, uint32, error)
	SendAggregatedResponse(ctx context.Context,
		task taskmanager.IFinalizerTaskManagerTask,
		taskResponse taskmanager.IFinalizerTaskManagerTaskResponse,
		nonSignerStakesAndSignature taskmanager.IBLSSignatureCheckerNonSignerStakesAndSignature,
	) (*types.Receipt, error)
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
func (w *AvsWriter) SendNewTaskVerifyBlock(ctx context.Context, blockNumber *big.Int, quorumThresholdPercentage uint32, quorumNumbers sdktypes.QuorumNums) (taskmanager.IFinalizerTaskManagerTask, uint32, error) {
	w.logger.Info("creating new task with AVS's task manager")
	noSendTxOpts, err := w.txMgr.GetNoSendTxOpts()
	if err != nil {
		return taskmanager.IFinalizerTaskManagerTask{}, 0, err
	}
	tx, err := w.AvsContractBindings.TaskManager.CreateNewTask(noSendTxOpts, blockNumber, quorumThresholdPercentage, quorumNumbers.UnderlyingType())
	if err != nil {
		w.logger.Errorf("Error assembling CreateNewTask tx")
		return taskmanager.IFinalizerTaskManagerTask{}, 0, err
	}

	receipt, err := w.txMgr.Send(ctx, tx)
	if err != nil {
		return taskmanager.IFinalizerTaskManagerTask{}, 0, errors.New("failed to send tx with err: " + err.Error())
	}
	w.logger.Infof("tx hash: %s", receipt.TxHash.String())
	w.logger.Info("sent new task with the AVS's task manager")
	newTaskCreatedEvent, err := w.AvsContractBindings.TaskManager.ContractFinalizerTaskManagerFilterer.ParseNewTaskCreated(*receipt.Logs[0])
	if err != nil {
		w.logger.Error("Aggregator failed to parse new task created event", "err", err)
		return taskmanager.IFinalizerTaskManagerTask{}, 0, err
	}
	return newTaskCreatedEvent.Task, newTaskCreatedEvent.TaskIndex, nil
}

func (w *AvsWriter) SendAggregatedResponse(ctx context.Context, task taskmanager.IFinalizerTaskManagerTask, taskResponse taskmanager.IFinalizerTaskManagerTaskResponse, nonSignerStakesAndSignature taskmanager.IBLSSignatureCheckerNonSignerStakesAndSignature, NonSignerStakesAndSignatureForOldState taskmanager.IGaspMultiRollupServicePrimitivesNonSignerStakesAndSignatureForOldState) (*types.Receipt, error) {
	w.logger.Info("sending aggregated task response with the AVS's task manager")
	noSendTxOpts, err := w.txMgr.GetNoSendTxOpts()
	if err != nil {
		return nil, err
	}
	tx, err := w.AvsContractBindings.TaskManager.RespondToTask(noSendTxOpts, task, taskResponse, nonSignerStakesAndSignature, NonSignerStakesAndSignatureForOldState)
	if err != nil {
		w.logger.Errorf("Error assembling RespondToTask tx")
		return nil, err
	}

	receipt, err := w.txMgr.Send(ctx, tx)
	if err != nil {
		return nil, errors.New("failed to send tx with err: " + err.Error())
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
	w.logger.Infof("tx hash: %s", receipt.TxHash.String())
	w.logger.Info("sent eject operators with AVS's service manager")
	return receipt, nil
}

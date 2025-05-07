package aggregator

import (
	"context"
	"math/big"
	"time"
	"fmt"

	sdktypes "github.com/Layr-Labs/eigensdk-go/types"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"

	"github.com/gasp-xyz/gasp-monorepo/avs-aggregator/core/chainio"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
)

var balance = promauto.NewGauge(prometheus.GaugeOpts{
	Name: "account_balance",
	Help: "balance of aggregator account",
})

var latest_batch_on_l2_per_l1 = promauto.NewGaugeVec(prometheus.GaugeOpts{
	Name: "latest_batch_on_l2_per_l1",
	Help: "Latest batch on L2 per L1",
}, []string{"for_l1"})

var latest_batch_processed_on_l1_per_l1 = promauto.NewGaugeVec(prometheus.GaugeOpts{
	Name: "latest_batch_processed_on_l1_per_l1",
	Help: "Latest batch processed on L1 per L1",
}, []string{"for_l1"})

var last_task_created = promauto.NewGaugeVec(prometheus.GaugeOpts{
	Name: "last_task_created",
	Help: "Last task created for each task type",
}, []string{"task_type"})

var last_task_responded = promauto.NewGaugeVec(prometheus.GaugeOpts{
	Name: "last_task_responded",
	Help: "Last task responded for each task type",
}, []string{"task_type"})

var last_task_completed = promauto.NewGaugeVec(prometheus.GaugeOpts{
	Name: "last_task_completed",
	Help: "Last task completed for each task type",
}, []string{"task_type"})

var aggregator_run_trigger = promauto.NewGaugeVec(prometheus.GaugeOpts{
	Name: "aggregator_run_trigger",
	Help: "Timestamps for when the aggregator started and when it received the run trigger",
}, []string{"event"})

var tasks_responded_but_not_completed = promauto.NewCounter(prometheus.CounterOpts{
	Name: "tasks_responded_but_not_completed",
	Help: "Tasks that did not meet quorum",
})

var kicked_operators = promauto.NewCounter(prometheus.CounterOpts{
	Name: "kicked_operators",
	Help: "Number of operators kicked for inactivity",
})

func recordMetrics(logger sdklogging.Logger, rpc *chainio.EthRpc) {
	go func() {
		for {

			address, err := rpc.Clients.SenderAddress(context.Background())
			if err != nil {
				logger.Error("Could not get account address", "err", err)
				time.Sleep(60 * time.Second)
				continue
			}

			latest, err := rpc.Clients.BlockNumber(context.Background())
			if err != nil {
				logger.Error("Could not get latest block number", "err", err)
				time.Sleep(60 * time.Second)
				continue
			}

			account_balance, err := rpc.AvsReader.BalanceAt(context.Background(), address, big.NewInt(int64(latest)))
			if err != nil {
				logger.Error("could not fetch account balance", "err", err)
				time.Sleep(60 * time.Second)
				continue
			}

			balanceF64, _ := account_balance.Float64()
			balance.Set(balanceF64 / 1_000_000_000_000_000_000)
			time.Sleep(60 * time.Second)
		}
	}()
}

func recordLatestBatchOnL2PerL1Metric(chainId uint8, BatchId uint32) {
	latest_batch_on_l2_per_l1.WithLabelValues(fmt.Sprintf("%v", chainId)).Set(float64(BatchId))
}

func recordLatestBatchProcessedOnL1PerL1Metric(chainId uint8, BatchId uint32) {
	latest_batch_processed_on_l1_per_l1.WithLabelValues(fmt.Sprintf("%v", chainId)).Set(float64(BatchId))
}

func recordLastTaskCreatedMetric(lastTaskCreatedType sdktypes.TaskType, lastTaskCreatedIndex sdktypes.TaskIndex ) {
	if lastTaskCreatedType == sdktypes.TaskType(0) {
		last_task_created.WithLabelValues("op_task").Set(float64(lastTaskCreatedIndex))
	} else {
		last_task_created.WithLabelValues("rd_task").Set(float64(lastTaskCreatedIndex))
	}
}

func recordLastTaskRespondedMetric(lastTaskRespondedType sdktypes.TaskType, lastTaskRespondedIndex sdktypes.TaskIndex ) {
	if lastTaskRespondedType == sdktypes.TaskType(0) {
		last_task_responded.WithLabelValues("op_task").Set(float64(lastTaskRespondedIndex))
	} else {
		last_task_responded.WithLabelValues("rd_task").Set(float64(lastTaskRespondedIndex))
	}
}

func recordLastTaskCompletedMetric(lastTaskCompletedType sdktypes.TaskType, lastTaskCompletedIndex sdktypes.TaskIndex ) {
	if lastTaskCompletedType == sdktypes.TaskType(0) {
		last_task_completed.WithLabelValues("op_task").Set(float64(lastTaskCompletedIndex))
	} else {
		last_task_completed.WithLabelValues("rd_task").Set(float64(lastTaskCompletedIndex))
	}
}

func recordRunTriggerTimesEventMetric(set bool) {
	if set {
		aggregator_run_trigger.WithLabelValues("started_awaiting_run_trigger").Set(float64(time.Now().Unix()))
	} else {
		aggregator_run_trigger.WithLabelValues("received_run_trigger").Set(float64(time.Now().Unix()))
	}
}

func recordRespondedButUncompletedTasksMetric() {
	tasks_responded_but_not_completed.Inc()
}

func recordKickedOperatorMetric(val uint32) {
	kicked_operators.Add(float64(val))
}

package aggregator

import (
	"context"
	"math/big"
	"time"

	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/gasp-xyz/gasp-monorepo/avs-aggregator/core/chainio"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
)

var balance = promauto.NewGauge(prometheus.GaugeOpts{
	Name: "account balance",
	Help: "balance of aggregator account",
})

func recordMetrics(logger sdklogging.Logger, rpc *chainio.EthRpc) {
	go func() {
		for {

			address, err := rpc.Clients.Wallet.SenderAddress(context.Background())
			if err != nil {
				logger.Error("Could not get account address", "err", err)
				time.Sleep(60 * time.Second)
				continue
			}

			latest, err := rpc.Clients.EthHttpClient.BlockNumber(context.Background())
			if err != nil {
				logger.Error("Could not get latest block number", "err", err)
				time.Sleep(60 * time.Second)
				continue
			}

			account_balance, err := rpc.AvsReader.AvsServiceBindings.EthClient.BalanceAt(context.Background(), address, big.NewInt(int64(latest)))
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

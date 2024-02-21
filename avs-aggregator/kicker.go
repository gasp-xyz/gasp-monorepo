package aggregator

import (
	"context"

	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	"github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/core/chainio"
	"github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/types"
)

type Kicker struct {
	logger      logging.Logger
	kickPeriod  uint32
	blockPeriod uint32
	ethRpc      chainio.EthRpc
}

func NewKicker(logger logging.Logger, ethRpc chainio.EthRpc, kickPeriod uint32, blockPeriod uint32) (*Kicker, error) {
	return &Kicker{
		logger:      logger,
		kickPeriod:  kickPeriod,
		blockPeriod: blockPeriod,
		ethRpc:      ethRpc,
	}, nil
}

func (k *Kicker) TriggerNewTask(taskIndex uint32) {
	if !isTimeToKick(taskIndex, k.kickPeriod) {
		return
	}
	go k.CheckStateAndKick()
}

func (k *Kicker) CheckStateAndKick() error {
	k.logger.Info("Running Operator Active List check")
	// get N last TaskResponses
	// our block ~= eth block time, so this should roughly match; new task for every Nth block * evry Mth task to kick
	events, err := k.ethRpc.AvsReader.GetTaskRespondedEvents(context.Background(), k.kickPeriod*k.blockPeriod)
	if err != nil {
		k.logger.Error("Cannot get list of past TaskResponded events", "err", err)
		return err
	}

	k.logger.Debug("Got last events", "eventsCount", len(events))
	// get non signers present in every trx
	hash := make(map[bls.OperatorId]*int)
	nonSigningOperatorPubKeys := make([]*bls.G1Point, 0)
	for _, ev := range events {
		keys, err := k.ethRpc.AvsReader.GetNonSigningOperatorPubKeys(ev)
		if err != nil {
			k.logger.Error("Cannot get list of NonSigningOperatorPubKeys for TaskResponded event", "err", err)
			return err
		}
		for _, key := range keys {
			id := key.GetOperatorID()
			if counter := hash[id]; counter != nil {
				if *counter++; *counter >= len(events) {
					nonSigningOperatorPubKeys = append(nonSigningOperatorPubKeys, key)
				}
			} else {
				i := 1
				hash[id] = &i
			}
		}
	}

	k.logger.Info("OAL check found list of pubkeys", "pubkeys", nonSigningOperatorPubKeys)
	// fetch address and eject
	for _, key := range nonSigningOperatorPubKeys {
		address, err := k.ethRpc.Clients.AvsRegistryChainReader.GetOperatorFromId(&bind.CallOpts{}, key.GetOperatorID())
		if err != nil {
			k.logger.Error("Cannot get operator address", "operatorId", key.GetOperatorID(), "err", err)
			return err
		}

		_, err = k.ethRpc.Clients.AvsRegistryChainWriter.EjectOperator(context.Background(), address, types.QUORUM_NUMBERS)
		if err != nil {
			k.logger.Error("Cannot eject operator", "operatorAddress", address, "err", err)
			return err
		}
		k.logger.Info("Operator ejected successfuly", "address", address)
	}

	return nil
}

func isTimeToKick(index uint32, period uint32) bool {
	return index%period == 0
}

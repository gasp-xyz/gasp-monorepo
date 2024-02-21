package aggregator

import (
	"context"
	"math/big"
	"sort"

	"github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/core/chainio"
	"github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/types"
)

type StakeUpdate struct {
	logger       logging.Logger
	updatePeriod uint32
	blockPeriod  uint32
	ethRpc       chainio.EthRpc
	lastBlock    uint32
}

func NewStakeUpdate(logger logging.Logger, ethRpc chainio.EthRpc, updatePeriod uint32, blockPeriod uint32) (*StakeUpdate, error) {
	return &StakeUpdate{
		logger:       logger,
		updatePeriod: updatePeriod,
		blockPeriod:  blockPeriod,
		ethRpc:       ethRpc,
		lastBlock:    0,
	}, nil
}

func (k *StakeUpdate) TriggerNewTask(taskIndex uint32) {
	if !isTimeToUpdate(taskIndex, k.updatePeriod) {
		return
	}
	go k.CheckStateAndupdate()
}

func (k *StakeUpdate) CheckStateAndupdate() error {
	k.logger.Info("Running Operator Stake Update check")

	currentBlock, err := k.ethRpc.Clients.EthHttpClient.BlockNumber(context.Background())
	if err != nil {
		k.logger.Error("Cannot get current block number", "err", err)
		return err
	}

	operatorIds, err := k.ethRpc.Clients.AvsRegistryChainReader.GetOperatorIdList(&bind.CallOpts{}, types.QUORUM_NUMBER, uint32(currentBlock))
	if err != nil {
		k.logger.Error("Cannot get current operator list", "err", err)
		return err
	}

	operatorAdrresses := []common.Address{}
	for _, id := range operatorIds {
		address, err := k.ethRpc.Clients.AvsRegistryChainReader.GetOperatorFromId(&bind.CallOpts{}, id)
		if err != nil {
			k.logger.Error("Cannot get operator address", "operatorId", id, "err", err)
			return err
		}
		operatorAdrresses = append(operatorAdrresses, address)
	}

	sort.Slice(operatorAdrresses, func(i, j int) bool {
		a := big.NewInt(0).SetBytes(operatorAdrresses[i][:])
		b := big.NewInt(0).SetBytes(operatorAdrresses[j][:])
		return a.Cmp(b) < 0
	})

	if len(operatorAdrresses) > 0 {
		_, err = k.ethRpc.Clients.AvsRegistryChainWriter.UpdateStakesOfEntireOperatorSetForQuorums(context.Background(), [][]common.Address{operatorAdrresses}, types.QUORUM_NUMBERS)
		if err != nil {
			k.logger.Error("Cannot update stakes", "err", err)
			return err
		}
	}

	k.logger.Info("Operator stakes update successfull", "addresses", operatorAdrresses)

	return nil
}

func isTimeToUpdate(index uint32, period uint32) bool {
	return index%period == 0
}

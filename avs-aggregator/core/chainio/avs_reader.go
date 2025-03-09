package chainio

import (
	"context"
	"errors"
	"math/big"
	"strings"

	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/ethclient"

	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	logging "github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/utils"

	delegationManager "github.com/gasp-xyz/gasp-monorepo/avs-aggregator/bindings/DelegationManager"
	taskmanager "github.com/gasp-xyz/gasp-monorepo/avs-aggregator/bindings/FinalizerTaskManager"
	stakeRegistry "github.com/gasp-xyz/gasp-monorepo/avs-aggregator/bindings/StakeRegistry"
	// blsSignatureChecker "github.com/gasp-xyz/gasp-monorepo/avs-aggregator/bindings/BLSSignatureChecker"
	// opstateretriever "github.com/Layr-Labs/eigensdk-go/contracts/bindings/OperatorStateRetriever"

	sdktypes "github.com/Layr-Labs/eigensdk-go/types"
	"github.com/gasp-xyz/gasp-monorepo/avs-aggregator/types"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
)

type AvsReaderer interface {
	CheckSignatures(
		ctx context.Context, msgHash [32]byte, quorumNumbers []byte, referenceBlockNumber uint32, nonSignerStakesAndSignature taskmanager.IBLSSignatureCheckerNonSignerStakesAndSignature,
	) (taskmanager.IBLSSignatureCheckerQuorumStakeTotals, error)

	GetRdTaskRespondedEvents(ctx context.Context, blocksAgo uint32) ([]taskmanager.ContractFinalizerTaskManagerRdTaskResponded, error)

	GetNonSigningOperatorPubKeys(event taskmanager.ContractFinalizerTaskManagerRdTaskResponded) ([]*bls.G1Point, error)
	
	IsTaskPending(ctx context.Context) (bool, error)
	LastCompletedOpTaskCreatedBlock(ctx context.Context) (uint32, error)
	LatestOpTaskNum(ctx context.Context) (uint32, error)
	LatestRdTaskNum(ctx context.Context) (uint32, error)
	LastOpTaskCreatedBlock(ctx context.Context) (uint32, error)
	LastRdTaskCreatedBlock(ctx context.Context) (uint32, error)
	IdToTaskStatus(ctx context.Context, taskType uint8, taskIndex uint32) (uint8, error)
	ChainRdBatchNonce(ctx context.Context, chainIndex uint8) (uint32, error)
	LastCompletedOpTaskCreatedBlockAtBlock(ctx context.Context, atBlock uint64) (uint32, error)
	GetOperatorsFromIds(opts *bind.CallOpts, registryCoordinatorAddr common.Address, operatorIds []sdktypes.OperatorId) ([]common.Address, error)
	GetTypedOperatorsStakesForQuorumAtBlock(ctx context.Context, registryCoordinatorAddr common.Address, quorumNumbers sdktypes.QuorumNums, operatorAddr []common.Address, blockNumber sdktypes.BlockNum) (map[sdktypes.OperatorId]types.OperatorAvsState, error)
	GetOperatorsAvsStateAtBlock(ctx context.Context, registryCoordinatorAddr common.Address, quorumNumbers sdktypes.QuorumNums, blockNumber sdktypes.BlockNum) (map[sdktypes.OperatorId]types.OperatorAvsState, error)
	GetOperatorIdList(opts *bind.CallOpts, quorum sdktypes.QuorumNum, blockNumber uint32) ([]sdktypes.OperatorId, error)

	ChainID(ctx context.Context) (*big.Int, error)
	FilterNewOpTaskCreated(opts *bind.FilterOpts, taskIndex []uint32) (*taskmanager.ContractFinalizerTaskManagerNewOpTaskCreatedIterator, error)
	FilterNewRdTaskCreated(opts *bind.FilterOpts, taskIndex []uint32) (*taskmanager.ContractFinalizerTaskManagerNewRdTaskCreatedIterator, error)
	TaskResponseWindowBlock(opts *bind.CallOpts) (uint32, error)
	ParseOpTaskCompleted(log ethtypes.Log) (*taskmanager.ContractFinalizerTaskManagerOpTaskCompleted, error)
	ParseRdTaskCompleted(log ethtypes.Log) (*taskmanager.ContractFinalizerTaskManagerRdTaskCompleted, error)
	BalanceAt(ctx context.Context, account common.Address, blockNumber *big.Int) (*big.Int, error)

	TaskManagerAddress() common.Address	
	StakeRegistryAddress() common.Address	
	DelegationManagerAddress() common.Address	
	RegistryCoordinatorAddress() common.Address	
	ParseOperatorStakeUpdate(log ethtypes.Log) (*stakeRegistry.ContractStakeRegistryOperatorStakeUpdate, error)	
	ParseResumeTrackingOpState(log ethtypes.Log) (*taskmanager.ContractFinalizerTaskManagerResumeTrackingOpState, error)	
	ParsePauseTrackingOpState(log ethtypes.Log) (*taskmanager.ContractFinalizerTaskManagerPauseTrackingOpState, error)	
	ParseOperatorSharesIncreased(log ethtypes.Log) (*delegationManager.ContractDelegationManagerOperatorSharesIncreased, error)	
	ParseOperatorSharesDecreased(log ethtypes.Log) (*delegationManager.ContractDelegationManagerOperatorSharesDecreased, error)	
	ParseStrategyMultiplierUpdated(log ethtypes.Log) (*stakeRegistry.ContractStakeRegistryStrategyMultiplierUpdated, error)	
	ParseMinimumStakeForQuorumUpdated(log ethtypes.Log) (*stakeRegistry.ContractStakeRegistryMinimumStakeForQuorumUpdated, error)
}

var _ AvsReaderer = (*AvsReader)(nil)

type AvsReader struct {
	AvsServiceBindings *AvsServiceBindings
	logger             logging.Logger
}

func NewAvsReaderFromConfig(
	registry common.Address,
	ethClient *ethclient.Client,
	logger logging.Logger,
) (*AvsReader, error) {
	avsServiceBindings, err := NewAvsServiceBindings(registry, ethClient, logger)
	if err != nil {
		return nil, err
	}

	return NewAvsReader(avsServiceBindings, logger)
}

func NewAvsReader(avsServiceBindings *AvsServiceBindings, logger logging.Logger) (*AvsReader, error) {
	return &AvsReader{
		AvsServiceBindings: avsServiceBindings,
		logger:             logger,
	}, nil
}

func (r *AvsReader) CheckSignatures(
	ctx context.Context, msgHash [32]byte, quorumNumbers []byte, referenceBlockNumber uint32, nonSignerStakesAndSignature taskmanager.IBLSSignatureCheckerNonSignerStakesAndSignature,
) (taskmanager.IBLSSignatureCheckerQuorumStakeTotals, error) {
	stakeTotalsPerQuorum, _, err := r.AvsServiceBindings.TaskManager.CheckSignatures(
		&bind.CallOpts{}, msgHash, quorumNumbers, referenceBlockNumber, nonSignerStakesAndSignature,
	)
	if err != nil {
		return taskmanager.IBLSSignatureCheckerQuorumStakeTotals{}, err
	}
	return stakeTotalsPerQuorum, nil
}

func (r *AvsReader) IsTaskPending(
	ctx context.Context,
) (bool, error) {
	v, err := r.AvsServiceBindings.TaskManager.IsTaskPending(
		&bind.CallOpts{},
	)
	if err != nil {
		return false, err
	}
	return v, nil
}

func (r *AvsReader) LastCompletedOpTaskCreatedBlock(
	ctx context.Context,
) (uint32, error) {
	v, err := r.AvsServiceBindings.TaskManager.LastCompletedOpTaskCreatedBlock(
		&bind.CallOpts{},
	)
	if err != nil {
		return uint32(0), err
	}
	return v, nil
}

func (r *AvsReader) LatestOpTaskNum(
	ctx context.Context,
) (uint32, error) {
	v, err := r.AvsServiceBindings.TaskManager.LatestOpTaskNum(
		&bind.CallOpts{},
	)
	if err != nil {
		return uint32(0), err
	}
	return v, nil
}

func (r *AvsReader) LatestRdTaskNum(
	ctx context.Context,
) (uint32, error) {
	v, err := r.AvsServiceBindings.TaskManager.LatestRdTaskNum(
		&bind.CallOpts{},
	)
	if err != nil {
		return uint32(0), err
	}
	return v, nil
}

func (r *AvsReader) LastOpTaskCreatedBlock(
	ctx context.Context,
) (uint32, error) {
	v, err := r.AvsServiceBindings.TaskManager.LastOpTaskCreatedBlock(
		&bind.CallOpts{},
	)
	if err != nil {
		return uint32(0), err
	}
	return v, nil
}

func (r *AvsReader) LastRdTaskCreatedBlock(
	ctx context.Context,
) (uint32, error) {
	v, err := r.AvsServiceBindings.TaskManager.LastRdTaskCreatedBlock(
		&bind.CallOpts{},
	)
	if err != nil {
		return uint32(0), err
	}
	return v, nil
}


func (r *AvsReader) IdToTaskStatus(
	ctx context.Context,
	taskType uint8,
	taskIndex uint32,
) (uint8, error) {
	v, err := r.AvsServiceBindings.TaskManager.IdToTaskStatus(
		&bind.CallOpts{},
		taskType,
		taskIndex,
	)
	if err != nil {
		return uint8(0), err
	}
	return v, nil
}

func (r *AvsReader) ChainRdBatchNonce(
	ctx context.Context,
	chainIndex uint8,
) (uint32, error) {
	v, err := r.AvsServiceBindings.TaskManager.ChainRdBatchNonce(
		&bind.CallOpts{},
		chainIndex,
	)
	if err != nil {
		return uint32(0), err
	}
	return v, nil
}

func (r *AvsReader) LastCompletedOpTaskCreatedBlockAtBlock(
	ctx context.Context, atBlock uint64,
) (uint32, error) {
	v, err := r.AvsServiceBindings.TaskManager.LastCompletedOpTaskCreatedBlock(
		&bind.CallOpts{BlockNumber: big.NewInt(int64(atBlock))},
	)
	if err != nil {
		return uint32(0), err
	}
	return v, nil
}

func (r *AvsReader) GetRdTaskRespondedEvents(ctx context.Context, blocksAgo uint32) ([]taskmanager.ContractFinalizerTaskManagerRdTaskResponded, error) {
	events := []taskmanager.ContractFinalizerTaskManagerRdTaskResponded{}

	currentBlock, err := r.AvsServiceBindings.EthClient.BlockNumber(ctx)
	if err != nil {
		r.logger.Error("Cannot get current block number", "err", err)
		return nil, err
	}
	opts := bind.FilterOpts{Start: currentBlock - uint64(blocksAgo), End: &currentBlock, Context: ctx}
	r.logger.Debug("Getting FilterRdTaskResponded", "opts", opts)
	it, err := r.AvsServiceBindings.TaskManager.FilterRdTaskResponded(&opts, []uint32{})
	if err != nil {
		return nil, err
	}
	for it.Next() {
		events = append(events, *it.Event)
	}
	if it.Error() != nil {
		return nil, err
	}

	return events, nil
}

func (r *AvsReader) GetNonSigningOperatorPubKeys(event taskmanager.ContractFinalizerTaskManagerRdTaskResponded) ([]*bls.G1Point, error) {
	// r.logger.Debug("event.Raw is", "event.Raw", event.Raw)

	// get the nonSignerStakesAndSignature
	txHash := event.Raw.TxHash
	// r.logger.Debug("txHash", "txHash", txHash)
	tx, _, err := r.AvsServiceBindings.EthClient.TransactionByHash(context.Background(), txHash)
	_ = tx
	if err != nil {
		r.logger.Error("Error getting transaction by hash",
			"txHash", txHash,
			"err", err,
		)
		return nil, err
	}
	calldata := tx.Data()
	// r.logger.Debug("calldata", "calldata", calldata)
	cstmAbi, err := abi.JSON(strings.NewReader(taskmanager.ContractFinalizerTaskManagerABI))
	if err != nil {
		r.logger.Error("Error getting Abi", "err", err)
		return nil, err
	}
	methodSig := calldata[:4]
	// r.logger.Debug("methodSig", "methodSig", methodSig)
	method, err := cstmAbi.MethodById(methodSig)
	if err != nil {
		r.logger.Error("Error getting method", "err", err)
		return nil, err
	}
	
	inputs, err := method.Inputs.Unpack(calldata[4:])
	if err != nil {
		r.logger.Error("Error unpacking calldata", "err", err)
		return nil, err
	}

	// r.logger.Debug("inputs", "inputs", inputs)
	nonSignerStakesAndSignatureInput := inputs[2].(struct {
		NonSignerQuorumBitmapIndices []uint32 "json:\"nonSignerQuorumBitmapIndices\""
		NonSignerPubkeys             []struct {
			X *big.Int "json:\"X\""
			Y *big.Int "json:\"Y\""
		} "json:\"nonSignerPubkeys\""
		QuorumApks []struct {
			X *big.Int "json:\"X\""
			Y *big.Int "json:\"Y\""
		} "json:\"quorumApks\""
		ApkG2 struct {
			X [2]*big.Int "json:\"X\""
			Y [2]*big.Int "json:\"Y\""
		} "json:\"apkG2\""
		Sigma struct {
			X *big.Int "json:\"X\""
			Y *big.Int "json:\"Y\""
		} "json:\"sigma\""
		QuorumApkIndices      []uint32   "json:\"quorumApkIndices\""
		TotalStakeIndices     []uint32   "json:\"totalStakeIndices\""
		NonSignerStakeIndices [][]uint32 "json:\"nonSignerStakeIndices\""
	})

	nonSigningOperatorPubKeys := make([]*bls.G1Point, len(nonSignerStakesAndSignatureInput.NonSignerPubkeys))
	for i, pubkey := range nonSignerStakesAndSignatureInput.NonSignerPubkeys {
		nonSigningOperatorPubKeys[i] = bls.NewG1Point(pubkey.X, pubkey.Y)
	}

	// r.logger.Debug("nonSigningOperatorPubKeys", "nonSigningOperatorPubKeys", nonSigningOperatorPubKeys)
	return nonSigningOperatorPubKeys, nil
}

func (r *AvsReader) GetOperatorsFromIds(
	opts *bind.CallOpts,
	registryCoordinatorAddr common.Address,
	operatorIds []sdktypes.OperatorId,
) ([]common.Address, error) {
	operatorIdsBytes := make([][32]byte, len(operatorIds))
	for i, id := range operatorIds {
		operatorIdsBytes[i] = id
	}
	operators, err := r.AvsServiceBindings.OperatorStateRetrieverExtended.GetOperatorsFromIds(opts, registryCoordinatorAddr, operatorIdsBytes)
	if err != nil {
		r.logger.Error("Cannot get operators from ids", "err", err)
		return nil, err
	}
	return operators, nil
}


// func (r *AvsReader) GetOperatorsStakesForQuorum(
// 	opts *bind.CallOpts,
// 	registryCoordinatorAddr common.Address,
// 	quorumNumbers sdktypes.QuorumNums,
// 	operatorAddr []common.Address,
// ) ([][]opstateretriever.OperatorStateRetrieverOperator, error) {
// 	operatorStakes, err := r.AvsServiceBindings.OperatorStateRetrieverExtended.GetOperatorsStakesForQuorum(
// 		opts,
// 		registryCoordinatorAddr,
// 		quorumNumbers.UnderlyingType(),
// 		operatorAddr)
// 	if err != nil {
// 		return nil, sdktypes.WrapError(errors.New("Failed to get operators state"), err)
// 	}
// 	return operatorStakes, nil
// }

func (r *AvsReader) GetTypedOperatorsStakesForQuorumAtBlock(ctx context.Context, registryCoordinatorAddr common.Address, quorumNumbers sdktypes.QuorumNums, operatorAddr []common.Address, blockNumber sdktypes.BlockNum) (map[sdktypes.OperatorId]types.OperatorAvsState, error) {
	operatorsAvsState := make(map[sdktypes.OperatorId]types.OperatorAvsState)
	operatorsStakesInQuorums, err := r.AvsServiceBindings.OperatorStateRetrieverExtended.GetOperatorsStakesForQuorum(&bind.CallOpts{Context: ctx, BlockNumber: big.NewInt(int64(blockNumber))}, registryCoordinatorAddr, quorumNumbers.UnderlyingType(), operatorAddr)
	if err != nil {
		return nil, utils.WrapError(errors.New("Failed to get operator state"), err)
	}
	numquorums := len(quorumNumbers)
	if len(operatorsStakesInQuorums) != numquorums {
		r.logger.Fatal("Number of quorums returned from GetOperatorsStakeInQuorumsAtBlock does not match number of quorums requested. Probably pointing to old contract or wrong implementation.", "service", "AvsRegistryServiceChainCaller")
	}

	for quorumIdx, quorumNum := range quorumNumbers {
		for _, operator := range operatorsStakesInQuorums[quorumIdx] {
			if _, ok := operatorsAvsState[operator.OperatorId]; ok {
				operatorsAvsState[operator.OperatorId].StakePerQuorum[quorumNum] = operator.Stake
			} else {
				stakePerQuorum := make(map[sdktypes.QuorumNum]sdktypes.StakeAmount)
				operatorsAvsState[operator.OperatorId] = types.OperatorAvsState{
					OperatorId:     operator.OperatorId,
					Operator:   operator.Operator,
					StakePerQuorum: stakePerQuorum,
				}
				operatorsAvsState[operator.OperatorId].StakePerQuorum[quorumNum] = operator.Stake
			}
		}
	}

	return operatorsAvsState, nil
}


func (r *AvsReader) GetOperatorsAvsStateAtBlock(ctx context.Context, registryCoordinatorAddr common.Address, quorumNumbers sdktypes.QuorumNums, blockNumber sdktypes.BlockNum) (map[sdktypes.OperatorId]types.OperatorAvsState, error) {
	operatorsAvsState := make(map[sdktypes.OperatorId]types.OperatorAvsState)
	// Get operator state for each quorum by querying BLSOperatorStateRetriever (this call is why this service implementation is called ChainCaller)
	operatorsStakesInQuorums, err := r.AvsServiceBindings.OperatorStateRetrieverExtended.GetOperatorState(&bind.CallOpts{Context: ctx}, registryCoordinatorAddr, quorumNumbers.UnderlyingType(), blockNumber)
	if err != nil {
		return nil, utils.WrapError(errors.New("Failed to get operator state"), err)
	}
	numquorums := len(quorumNumbers)
	if len(operatorsStakesInQuorums) != numquorums {
		r.logger.Fatal("Number of quorums returned from GetOperatorsStakeInQuorumsAtBlock does not match number of quorums requested. Probably pointing to old contract or wrong implementation.", "service", "AvsRegistryServiceChainCaller")
	}

	for quorumIdx, quorumNum := range quorumNumbers {
		for _, operator := range operatorsStakesInQuorums[quorumIdx] {
			if _, ok := operatorsAvsState[operator.OperatorId]; ok {
				operatorsAvsState[operator.OperatorId].StakePerQuorum[quorumNum] = operator.Stake
			} else {
				stakePerQuorum := make(map[sdktypes.QuorumNum]sdktypes.StakeAmount)
				operatorsAvsState[operator.OperatorId] = types.OperatorAvsState{
					OperatorId:     operator.OperatorId,
					Operator:   operator.Operator,
					StakePerQuorum: stakePerQuorum,
				}
				operatorsAvsState[operator.OperatorId].StakePerQuorum[quorumNum] = operator.Stake
			}
		}
	}

	return operatorsAvsState, nil
}

func (r *AvsReader) GetOperatorIdList(
	opts *bind.CallOpts,
	quorum sdktypes.QuorumNum,
	blockNumber uint32,
) ([]sdktypes.OperatorId, error) {
	ids, err := r.AvsServiceBindings.IndexRegistry.GetOperatorListAtBlockNumber(opts, quorum.UnderlyingType(), blockNumber)
	if err != nil {
		r.logger.Error("Cannot get operator list", "err", err)
		return nil, err
	}
	operatorIds := make([]sdktypes.OperatorId, 0)
	for _, id := range ids {
		operatorIds = append(operatorIds, id)
	}
	return operatorIds, nil
}

func (r *AvsReader) ChainID(ctx context.Context) (*big.Int, error) {
	chainId, err := r.AvsServiceBindings.EthClient.ChainID(ctx)
	return chainId, err 
}

func (r *AvsReader) FilterNewOpTaskCreated(opts *bind.FilterOpts, taskIndex []uint32) (*taskmanager.ContractFinalizerTaskManagerNewOpTaskCreatedIterator, error) {
	eventIter, err := r.AvsServiceBindings.TaskManager.FilterNewOpTaskCreated(
		opts, taskIndex,
	)
	return eventIter, err 
}

func (r *AvsReader) FilterNewRdTaskCreated(opts *bind.FilterOpts, taskIndex []uint32) (*taskmanager.ContractFinalizerTaskManagerNewRdTaskCreatedIterator, error) {

	eventIter, err := r.AvsServiceBindings.TaskManager.FilterNewRdTaskCreated(
		opts, taskIndex,
	)
	return eventIter, err 
}

func (r *AvsReader) TaskResponseWindowBlock(opts *bind.CallOpts) (uint32, error) {
	
	taskResponseWindowBlock, err := r.AvsServiceBindings.TaskManager.TaskResponseWindowBlock(opts)
	return taskResponseWindowBlock, err

}

func (r *AvsReader) ParseOpTaskCompleted(log ethtypes.Log) (*taskmanager.ContractFinalizerTaskManagerOpTaskCompleted, error) {
	
	v, err := r.AvsServiceBindings.TaskManager.ContractFinalizerTaskManagerFilterer.ParseOpTaskCompleted(log)
	return v, err
}

func (r *AvsReader) ParseRdTaskCompleted(log ethtypes.Log) (*taskmanager.ContractFinalizerTaskManagerRdTaskCompleted, error) {
	v, err := r.AvsServiceBindings.TaskManager.ContractFinalizerTaskManagerFilterer.ParseRdTaskCompleted(log)
	return v, err
}

func (r *AvsReader) BalanceAt(ctx context.Context, account common.Address, blockNumber *big.Int) (*big.Int, error) {
	account_balance, err := r.AvsServiceBindings.EthClient.BalanceAt(ctx, account, blockNumber)
	return account_balance, err
}

func (r *AvsReader) TaskManagerAddress() common.Address {
	return r.AvsServiceBindings.TaskManagerAddress
}

func (r *AvsReader) StakeRegistryAddress() common.Address {
	return r.AvsServiceBindings.StakeRegistryAddress
}

func (r *AvsReader) DelegationManagerAddress() common.Address {
	return r.AvsServiceBindings.DelegationManagerAddress
}

func (r *AvsReader) RegistryCoordinatorAddress() common.Address {
	return r.AvsServiceBindings.RegistryCoordinatorAddress
}

func (r *AvsReader) ParseOperatorStakeUpdate(log ethtypes.Log) (*stakeRegistry.ContractStakeRegistryOperatorStakeUpdate, error) {
	
	event, err := r.AvsServiceBindings.StakeRegistry.ContractStakeRegistryFilterer.ParseOperatorStakeUpdate(log)
	return event, err
}

func (r *AvsReader) ParseResumeTrackingOpState(log ethtypes.Log) (*taskmanager.ContractFinalizerTaskManagerResumeTrackingOpState, error) {
	event, err := r.AvsServiceBindings.TaskManager.ContractFinalizerTaskManagerFilterer.ParseResumeTrackingOpState(log)
	return event, err
}

func (r *AvsReader) ParsePauseTrackingOpState(log ethtypes.Log) (*taskmanager.ContractFinalizerTaskManagerPauseTrackingOpState, error) {
	event, err := r.AvsServiceBindings.TaskManager.ContractFinalizerTaskManagerFilterer.ParsePauseTrackingOpState(log)
	return event, err
}

func (r *AvsReader) ParseOperatorSharesIncreased(log ethtypes.Log) (*delegationManager.ContractDelegationManagerOperatorSharesIncreased, error) {
	event, err := r.AvsServiceBindings.DelegationManager.ContractDelegationManagerFilterer.ParseOperatorSharesIncreased(log)
	return event, err
}

func (r *AvsReader) ParseOperatorSharesDecreased(log ethtypes.Log) (*delegationManager.ContractDelegationManagerOperatorSharesDecreased, error) {
	event, err := r.AvsServiceBindings.DelegationManager.ContractDelegationManagerFilterer.ParseOperatorSharesDecreased(log)
	return event, err
}

func (r *AvsReader) ParseStrategyMultiplierUpdated(log ethtypes.Log) (*stakeRegistry.ContractStakeRegistryStrategyMultiplierUpdated, error) {
	
	event, err := r.AvsServiceBindings.StakeRegistry.ContractStakeRegistryFilterer.ParseStrategyMultiplierUpdated(log)
	return event, err
}

func (r *AvsReader) ParseMinimumStakeForQuorumUpdated(log ethtypes.Log) (*stakeRegistry.ContractStakeRegistryMinimumStakeForQuorumUpdated, error) {
	
	event, err := r.AvsServiceBindings.StakeRegistry.ContractStakeRegistryFilterer.ParseMinimumStakeForQuorumUpdated(log)
	return event, err
}
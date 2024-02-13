# Mangata AVS

<b> Do not use it in Production, testnet only. </b>

PoC Mangata AVS implementation

## Dependencies

You will need [foundry](https://book.getfoundry.sh/getting-started/installation) to run local testnet. You will need Rust toolchain to build avs-finalizer binary. And [golang toolchain](https://go.dev/doc/install) with [zap-pretty](https://github.com/maoueh/zap-pretty) to run the aggregator.
```
curl -L https://foundry.paradigm.xyz | bash
foundryup

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

go install github.com/maoueh/zap-pretty@latest
```

## Running via make

This simple session illustrates the basic flow of the AVS. The makefile commands are hardcoded for a single avs-finalizer. For more avs-finalizers you would need to modify setup script to distribute mock ERC20 tokens and deposit into AVS per avs-finalizer.
See [0_AnvilSetup.s.sol](contracts/script/0_AnvilSetup.s.sol#L88) script for more info.

```bash
  git submodule init
  git submodule update
```


Start anvil in a separate terminal:

```bash
make start-anvil-chain-with-el-and-avs-deployed
```

The above command starts a local anvil chain from a [saved state](tests/integration/avs-and-eigenlayer-deployed-anvil-state.json) with eigenlayer and mangata-avs contracts already deployed (but no avs-finalizer registered).

Then start the aggregator: 
```bash
make start-avs-aggregator
```
And lately, the avs-finalizer,
```bash
make start-avs-finalizer
```


## Avs Task Description

The architecture of the AVS contains:

- [Eigenlayer core](https://github.com/Layr-Labs/eigenlayer-contracts/tree/master) contracts
- AVS contracts
  - [ServiceManager](contracts/src/MangataServiceManager.sol) which will eventually contain slashing logic but for M2 is just a placeholder.
  - [TaskManager](contracts/src/MangataTaskManager.sol) which contains [task creation](contracts/src/MangataTaskManager.sol#L83) and [task response](contracts/src/MangataTaskManager.sol#L102) logic.
  - Set of [registry contracts](https://github.com/Layr-Labs/eigenlayer-middleware) to manage avs-finalizers opted in to this avs
- Task Generator
  - in a real world scenario, this could be a separate entity, but for this simple demo, the aggregator also acts as the task generator
- Aggregator
  - aggregates BLS signatures from avs-finalizers and posts the aggregated response to the task manager
  - For this simple demo, the aggregator is not an avs-finalizer, and thus does not need to register with eigenlayer or the AVS contract. It's IP address is simply hardcoded into the finalizers' config.
- Finalizers
  - Execute a block sent to the task manager by the task generator, sign it, and send it to the aggregator


1. A task generator (in our case, same as the aggregator) publishes tasks once every regular interval (say 10 blocks, you are free to set your own interval) to the Mangata contract's [createNewTask](contracts/src/MangataTaskManager.sol#L83) function. Each task specifies an integer `blockNumber` for which it wants the currently opted-in finalizers to execute it. `createNewTask` also takes `quorumNumbers` and `quorumThresholdPercentage` which requests that each listed quorum (we only use quorumNumber 0) needs to reach at least thresholdPercentage of finalizer signatures.

2. A [registry](https://github.com/Layr-Labs/eigenlayer-middleware/blob/master/src/BLSRegistryCoordinatorWithIndices.sol) contract is deployed that allows any eigenlayer finalizer with at least 1 delegated [mockerc20](contracts/src/ERC20Mock.sol) token to opt-in to this AVS and also de-register from this AVS.

3. [Finalizer] The finalizers who are currently opted-in with the AVS need to read the task number from the Task contract, execute the block, sign on that computed result (over the BN254 curve) and send their taskResponse and signature to the aggregator.

4. [Aggregator] The aggregator collects the signatures from the finalizers and aggregates them using BLS aggregation. If any response passes the [quorumThresholdPercentage](contracts/src/IMangataTaskManager.sol#L36) set by the task generator when posting the task, the aggregator posts the aggregated response to the Task contract.

5. If a response was sent within the [response window](contracts/src/MangataTaskManager.sol#L122), we enter the [Dispute resolution] period.
   - Not yet implemented

## Avs node spec compliance

Every AVS node implementation is required to abide by the [Eigenlayer AVS Node Specification](https://eigen.nethermind.io/). We suggest reading through the whole spec, including the keys management section, but the hard requirements are currently only to:
- implement the [AVS Node API](https://eigen.nethermind.io/docs/category/avs-node-api)
- implement the [eigen prometheus metrics](https://eigen.nethermind.io/docs/category/metrics)

Work in progress for the current implementation of Finalizer.

## StakeUpdates Cronjob

AVS Registry contracts have a stale view of finalizer shares in the delegation manager contract. In order to update their stake table, they need to periodically call the [StakeRegistry.updateStakes()](https://github.com/Layr-Labs/eigenlayer-middleware/blob/f171a0812126bbb0bb6d44f53c622591a643e987/src/StakeRegistry.sol#L76) function. We are currently using internal script to get finalizer adresses and call the contract manually.

## Integration Tests

See the integration tests [README](tests/integration/README.md) for more details.

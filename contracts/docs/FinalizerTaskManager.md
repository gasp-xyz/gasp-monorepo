## FinalizerTaskManager

| File                                                          | Type      | Proxy             |
| ------------------------------------------------------------- | --------- | ----------------- |
| [`FinalizerTaskManager.sol`](../src/FinalizerTaskManager.sol) | Singleton | Transparent proxy |

The `FinalizerTaskManager` is responsible for orchestrating the Task lifecycle. New task are created by the Aggregator service. Upon receiving the responses from Operators in Aggregator service, it submits the aggregated response. Serves as an entry point for Operators to pick up new tasks.

Events & structs can be check in the [interface definition](../src/IFinalizerTaskManager.sol)

We also include the functionality of [`OperatorStateRetriever`](../lib/eigenlayer-middleware/src/OperatorStateRetriever.sol) and [`BLSSignatureChecker`](../lib/eigenlayer-middleware/docs/BLSSignatureChecker.md) contracts.

---

#### `createNewTask`

```solidity
function createNewTask(
    uint256 blockNumber,
    uint32 quorumThresholdPercentage,
    bytes calldata quorumNumbers
) external onlyTaskGenerator
```

Task generator, currently the Aggregator service, is responsible for creating new tasks. Aggregator subscribes to new blocks on L2 and submits new task at a given period, eg. every 50th block gets a new task. The emitted `NewTaskCreated` is used to trigger OPs block reexecution implementation.

_Requirements_:

- Caller MUST be the task generator

####

```solidity
function respondToTask(
    Task calldata task,
    TaskResponse calldata taskResponse,
    NonSignerStakesAndSignature memory nonSignerStakesAndSignature
) external onlyAggregator
```

Aggregator listens for OP's responses on dedicated RPC. Each response is checked for a valid signature, OP's public keys needs to be registered to the AVS. Once the aggregated signed stake reaches the threshold, or before expiry, the task response gets submited to contract. The original task & keys of the non signers are also attached to verify the quorum threshold against the criteria set in task. `TaskResponded` is emitted each time, if the criteria is met, `TaskCompleted` is emitted too.

`TaskResponded` emission is used for OPs active list filtering. If for all N of previous events an OP keys are present in the non signer struct, it gets ejected as inactive. No further restrictions are placed to prevent the OP from registring again, yet.

`TaskCompleted` marks a finalized block on L2. Used by other rollup entities to trigger updates.

_Requirements_:

- Caller MUST be the task aggregator

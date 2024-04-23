## FinalizerServiceManager

| File | Type | Proxy |
| -------- | -------- | -------- |
| [`FinalizerServiceManager.sol`](../src/FinalizerServiceManager.sol) | Singleton | Transparent proxy |

This is our implementation of [`ServiceManagerBase.sol`](../lib/eigenlayer-middleware/docs/ServiceManagerBase.md)

For docs see the base contract.

Our implementation extends the base with an extra reference to [`FinalizerTaskManager`](../src/FinalizerTaskManager.sol)
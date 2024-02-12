use sc_cli::{
    execution_method_from_cli, DEFAULT_WASMTIME_INSTANTIATION_STRATEGY,
    DEFAULT_WASM_EXECUTION_METHOD,
};
use sc_executor::{sp_wasm_interface::HostFunctions, WasmExecutor, DEFAULT_HEAP_ALLOC_STRATEGY};

pub(crate) fn build_executor<H: HostFunctions>() -> WasmExecutor<H> {
    WasmExecutor::builder()
        .with_execution_method(execution_method_from_cli(
            DEFAULT_WASM_EXECUTION_METHOD,
            DEFAULT_WASMTIME_INSTANTIATION_STRATEGY,
        ))
        .with_onchain_heap_alloc_strategy(DEFAULT_HEAP_ALLOC_STRATEGY)
        .with_offchain_heap_alloc_strategy(DEFAULT_HEAP_ALLOC_STRATEGY)
        .build()
}

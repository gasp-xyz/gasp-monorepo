use sc_executor::{sp_wasm_interface::HostFunctions, WasmExecutor};
use sp_core::{
    offchain::{
        testing::{TestOffchainExt, TestTransactionPoolExt},
        OffchainDbExt, OffchainWorkerExt, TransactionPoolExt,
    },
    traits::{CallContext, ReadRuntimeVersionExt},
};
use sp_externalities::Extensions;
use sp_keystore::{testing::MemoryKeystore, KeystoreExt};
use sp_runtime::traits::{Block as BlockT, HashingFor};
use sp_state_machine::{
    OverlayedChanges, StateMachine, StorageProof, TestExternalities, TrieBackendBuilder,
};
use std::{fmt::Debug, path::PathBuf, str::FromStr};

pub mod execute;
mod setup;
mod state;

pub(crate) const LOG_TARGET: &str = "operator::cli";

/// Get the hash type of the generic `Block` from a `hash_str`.
pub(crate) fn hash_of<Block: BlockT>(hash_str: &str) -> sc_cli::Result<Block::Hash>
where
    <Block::Hash as FromStr>::Err: Debug,
{
    hash_str
        .parse::<<Block as BlockT>::Hash>()
        .map_err(|e| format!("Could not parse block hash: {:?}", e).into())
}

/// Same as [`state_machine_call`], but it also computes and returns the storage proof and ref time
/// information.
///
/// Make sure [`LOG_TARGET`] is enabled in logging.
pub(crate) fn state_machine_call_with_proof<Block: BlockT, HostFns: HostFunctions>(
    ext: &TestExternalities<HashingFor<Block>>,
    storage_overlay: &mut OverlayedChanges<HashingFor<Block>>,
    executor: &WasmExecutor<HostFns>,
    method: &'static str,
    data: &[u8],
    mut extensions: Extensions,
    maybe_export_proof: Option<PathBuf>,
) -> sc_cli::Result<(StorageProof, Vec<u8>)> {
    let runtime_code_backend = sp_state_machine::backend::BackendRuntimeCode::new(&ext.backend);
    let proving_backend = TrieBackendBuilder::wrap(&ext.backend)
        .with_recorder(Default::default())
        .build();
    let runtime_code = runtime_code_backend.runtime_code()?;

    let encoded_result = StateMachine::new(
        &proving_backend,
        storage_overlay,
        executor,
        method,
        data,
        &mut extensions,
        &runtime_code,
        CallContext::Offchain,
    )
    .execute()
    .map_err(|e| format!("failed to execute {}: {}", method, e))
    .map_err::<sc_cli::Error, _>(Into::into)?;

    let proof = proving_backend
        .extract_proof()
        .expect("A recorder was set and thus, a storage proof can be extracted; qed");

    if let Some(path) = maybe_export_proof {
        let mut file = std::fs::File::create(&path).map_err(|e| {
            log::error!(
                target: LOG_TARGET,
                "Failed to create file {}: {:?}",
                path.to_string_lossy(),
                e
            );
            e
        })?;

        log::info!(target: LOG_TARGET, "Writing storage proof to {}", path.to_string_lossy());

        use std::io::Write as _;
        file.write_all(storage_proof_to_raw_json(&proof).as_bytes())
            .map_err(|e| {
                log::error!(
                    target: LOG_TARGET,
                    "Failed to write storage proof to {}: {:?}",
                    path.to_string_lossy(),
                    e
                );
                e
            })?;
    }

    Ok((proof, encoded_result))
}

/// Converts a [`sp_state_machine::StorageProof`] into a JSON string.
fn storage_proof_to_raw_json(storage_proof: &sp_state_machine::StorageProof) -> String {
    serde_json::Value::Object(
        storage_proof
            .to_memory_db::<sp_runtime::traits::BlakeTwo256>()
            .drain()
            .iter()
            .map(|(key, (value, _n))| {
                (
                    format!("0x{}", hex::encode(key.as_bytes())),
                    serde_json::Value::String(format!("0x{}", hex::encode(value))),
                )
            })
            .collect(),
    )
    .to_string()
}

pub(crate) fn rpc_err_handler(error: impl Debug) -> &'static str {
    log::error!(target: LOG_TARGET, "rpc error: {:?}", error);
    "rpc error."
}

/// Build all extensions that we typically use.
pub(crate) fn full_extensions<H: HostFunctions>(wasm_executor: WasmExecutor<H>) -> Extensions {
    let mut extensions = Extensions::default();
    let (offchain, _offchain_state) = TestOffchainExt::new();
    let (pool, _pool_state) = TestTransactionPoolExt::new();
    let keystore = MemoryKeystore::new();
    extensions.register(OffchainDbExt::new(offchain.clone()));
    extensions.register(OffchainWorkerExt::new(offchain));
    extensions.register(KeystoreExt::new(keystore));
    extensions.register(TransactionPoolExt::new(pool));
    extensions.register(ReadRuntimeVersionExt::new(wasm_executor));

    extensions
}

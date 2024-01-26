use super::{
    full_extensions, rpc_err_handler, setup::build_executor, state::State,
    state_machine_call_with_proof,
};
use node_primitives::BlockNumber;
use sc_executor::sp_wasm_interface::HostFunctions;
use sp_runtime::{
    generic::SignedBlock,
    traits::{Block as BlockT, Header as HeaderT, NumberFor},
};
use std::{fmt::Debug, str::FromStr};
use substrate_rpc_client::{ws_client, ChainApi};
use tracing::{instrument};

#[instrument(skip(uri))]
pub async fn execute_block<Block, HostFns>(
    uri: &str,
    at: BlockNumber,
) -> sc_cli::Result<Block::Hash>
where
    Block: BlockT + serde::de::DeserializeOwned,
    <Block::Hash as FromStr>::Err: Debug,
    Block::Hash: serde::de::DeserializeOwned,
    Block::Header: serde::de::DeserializeOwned,
    <NumberFor<Block> as TryInto<u64>>::Error: Debug,
    HostFns: HostFunctions,
{
    let executor = build_executor::<HostFns>();
    let rpc = ws_client(uri).await?;

    let execute_at_state = State::for_block_number::<Block>(uri, at).await?;
    let execute_at = execute_at_state.at::<Block>()?;
    let prev_block_state = execute_at_state.into_prev_block_state::<Block>().await?;

    let ext = prev_block_state.to_ext::<Block>().await?;

    // Execute the desired block on top of it
    let block = ChainApi::<(), Block::Hash, Block::Header, SignedBlock<Block>>::block(
        &rpc,
        Some(execute_at),
    )
    .await
    .map_err(rpc_err_handler)?
    .expect("header exists, block should also exist; qed")
    .block;

    // A digest item gets added when the runtime is processing the block, so we need to pop
    // the last one to be consistent with what a gossiped block would contain.
    let (mut header, extrinsics) = block.deconstruct();
    header.digest_mut().pop();
    let block = Block::new(header, extrinsics);

    // for now, hardcoded for the sake of simplicity. We might customize them one day.
    let payload = block.clone().encode();

    let _ = state_machine_call_with_proof::<Block, HostFns>(
        &ext,
        &mut Default::default(),
        &executor,
        "Core_execute_block",
        &payload,
        full_extensions(executor.clone()),
        None,
    )?;

    Ok(block.hash())
}

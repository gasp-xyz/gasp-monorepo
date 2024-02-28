use super::{
    full_extensions, rpc_err_handler, setup::build_executor, state::State,
    state_machine_call_with_proof,
};
use eyre::eyre;
use node_primitives::BlockNumber;
use sc_executor::sp_wasm_interface::HostFunctions;
use sp_core::H256;
use sp_runtime::{
    generic::SignedBlock,
    traits::{Block as BlockT, Hash, Header as HeaderT, Keccak256, NumberFor},
};
use std::{fmt::Debug, str::FromStr};
use substrate_rpc_client::{ws_client, ChainApi};
use tracing::instrument;

#[instrument(skip(uri))]
pub async fn execute_block<Block, HostFns>(uri: &str, at: BlockNumber) -> eyre::Result<(H256, H256)>
where
    Block: BlockT + serde::de::DeserializeOwned,
    <Block::Hash as FromStr>::Err: Debug,
    Block::Hash: serde::de::DeserializeOwned + Into<H256>,
    Block::Header: serde::de::DeserializeOwned,
    <NumberFor<Block> as TryInto<u64>>::Error: Debug,
    HostFns: HostFunctions,
{
    let executor = build_executor::<HostFns>();
    let rpc = ws_client(uri).await.map_err(|e| eyre!(e))?;

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
    .map_err(rpc_err_handler)
    .map_err(|e| eyre!(e))?
    .expect("header exists, block should also exist; qed")
    .block;
    let block_hash = block.hash();

    // A digest item gets added when the runtime is processing the block, so we need to pop
    // the last one to be consistent with what a gossiped block would contain.
    let (mut header, extrinsics) = block.deconstruct();
    header.digest_mut().pop();
    let block = Block::new(header, extrinsics);

    // for now, hardcoded for the sake of simplicity. We might customize them one day.
    let payload = block.clone().encode();

    let (proof, _) = state_machine_call_with_proof::<Block, HostFns>(
        &ext,
        &mut Default::default(),
        &executor,
        "Core_execute_block",
        &payload,
        full_extensions(executor.clone()),
        None,
    )?;
    let hash = Keccak256::hash_of(&proof);

    Ok((block_hash.into(), hash))
}

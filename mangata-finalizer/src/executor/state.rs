use super::{hash_of, rpc_err_handler};
use frame_remote_externalities::{Builder, Mode, OnlineConfig, RemoteExternalities};
use node_primitives::BlockNumber;

use sp_core::{storage::well_known_keys, twox_128};
use sp_rpc::{list::ListOrValue::Value, number::NumberOrHex::Number};
use sp_runtime::{
    traits::{Block as BlockT, Header},
    DeserializeOwned,
};
use std::{fmt::Debug, str::FromStr};
use substrate_rpc_client::{ws_client, ChainApi};

/// The source of runtime *state* to use.
#[derive(Debug, Clone)]
pub struct State {
    pub uri: String,
    pub at: String,
}

impl State {
    /// Return the `at` block hash as a `Hash`, if it exists.
    pub async fn for_block_number<Block: BlockT>(uri: &str, at: BlockNumber) -> sc_cli::Result<Self>
    where
        <Block::Hash as FromStr>::Err: Debug,
    {
        let rpc = ws_client(uri).await?;

        let hash = ChainApi::<(), Block::Hash, Block::Header, ()>::block_hash(
            &rpc,
            Some(Value(Number(at.into()))),
        )
        .await
        .map_err(rpc_err_handler)
        .and_then(|maybe_hash| {
            if let Value(Some(hash)) = maybe_hash {
                Ok(hex::encode(hash))
            } else {
                Err("Block hash not found")
            }
        })?;

        Ok(State {
            at: hash,
            uri: uri.to_owned(),
        })
    }

    /// Return the `at` block hash as a `Hash`, if it exists.
    pub fn at<Block: BlockT>(&self) -> sc_cli::Result<<Block>::Hash>
    where
        <Block::Hash as FromStr>::Err: Debug,
    {
        hash_of::<Block>(self.at.as_str())
    }

    pub async fn into_prev_block_state<Block: BlockT>(self) -> sc_cli::Result<State>
    where
        <Block::Hash as FromStr>::Err: Debug,
    {
        // We want to execute the block `at`, therefore need the state of the block *before* it.
        let at = self.at::<Block>()?;

        // Get the block number requested by the user, or the current block number if they
        // didn't specify one.
        let rpc = ws_client(&self.uri).await?;
        let previous_hash = ChainApi::<(), Block::Hash, Block::Header, ()>::header(&rpc, Some(at))
            .await
            .map_err(rpc_err_handler)
            .and_then(|maybe_header| {
                maybe_header
                    .ok_or("header_not_found")
                    .map(|h| *h.parent_hash())
            })?;

        Ok(State {
            at: hex::encode(previous_hash),
            ..self
        })
    }

    /// Create the [`RemoteExternalities`].
    ///
    /// This will override the code as it sees fit based on [`Runtime`]. It will also check the
    /// spec-version and name.
    pub async fn to_ext<Block: BlockT + DeserializeOwned>(
        &self,
    ) -> sc_cli::Result<RemoteExternalities<Block>>
    where
        Block::Header: DeserializeOwned,
        <Block::Hash as FromStr>::Err: Debug,
    {
        // get all keys
        let builder = Builder::<Block>::new().mode(Mode::Online(OnlineConfig {
            at: Some(self.at::<Block>()?),
            transport: self.uri.to_owned().into(),
            state_snapshot: None,
            pallets: vec![],
            child_trie: false,
            hashed_keys: vec![
                // we always download the code, but we almost always won't use it, based on
                // `Runtime`.
                well_known_keys::CODE.to_vec(),
                // we will always download this key, since it helps detect if we should do
                // runtime migration or not.
                [twox_128(b"System"), twox_128(b"LastRuntimeUpgrade")].concat(),
                [twox_128(b"System"), twox_128(b"Number")].concat(),
            ],
            hashed_prefixes: vec![],
        }));

        // build the main ext.
        let ext = builder.build().await?;

        Ok(ext)
    }
}

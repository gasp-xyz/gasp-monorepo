use crate::chainio::{avs::AvsContracts, build_client, eigen::ElContracts, Client};
use crate::cli::CliArgs;
use crate::crypto::bn254::BlsKeypair;
use crate::crypto::keystore::decrypt_keystore;
use crate::executor::execute::execute_block;

use bindings::mangata_task_manager::NewTaskCreatedFilter;
use ethers::prelude::*;
use eyre::eyre;
use node_executor::ExecutorDispatch;
use node_primitives::BlockNumber;
use sp_runtime::traits::BlakeTwo256;
use sp_runtime::{generic, OpaqueExtrinsic};
use std::sync::Arc;
use tracing::{debug, info, instrument};

pub type Header = generic::HeaderVer<node_primitives::BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, OpaqueExtrinsic>;

#[derive(Debug)]
pub struct Operator {
    client: Arc<Client>,
    avs_contracts: AvsContracts,
    el_contracts: ElContracts,
    bls_keypair: BlsKeypair,
    substrate_client_uri: String,
    chain_id: u64,
}
impl Operator {
    pub async fn from_cli(cfg: &CliArgs) -> eyre::Result<Self> {
        let client = Arc::new(build_client(&cfg.into()).await?);
        let avs_contracts = AvsContracts::build(&cfg.into(), client.clone()).await?;
        let slasher = avs_contracts.slasher_address().await?;
        let el_contracts = ElContracts::build(&cfg.into(), slasher, client.clone()).await?;

        info!("Decryting BLS keypair at: {:?}", cfg.bls_key_file);
        let bls_key = decrypt_keystore(&cfg.bls_key_file, &cfg.bls_key_password)?;
        debug!("Bls Keypair decrytped");

        Ok(Self {
            avs_contracts,
            el_contracts,
            substrate_client_uri: cfg.substrate_rpc_url.to_owned(),
            client,
            bls_keypair: bls_key,
            chain_id: cfg.chain_id,
        })
    }

    pub async fn watch_new_tasks(&self) -> eyre::Result<()> {
        let evs = self.avs_contracts.new_task_stream();
        let mut stream: stream::EventStream<'_, _, NewTaskCreatedFilter, _> =
            evs.subscribe().await?;
        while let Some(Ok(event)) = stream.next().await {
            println!("{:?}", event);
            self.execute_block(event.task.block_number.as_u32()).await?;
            println!("executed");
        }
        Ok(())
    }

    pub(crate) async fn execute_block(&self, block_number: BlockNumber) -> eyre::Result<()> {
        use sc_executor::{sp_wasm_interface::ExtendedHostFunctions, NativeExecutionDispatch};
        execute_block::<
            Block,
            ExtendedHostFunctions<
                sp_io::SubstrateHostFunctions,
                <ExecutorDispatch as NativeExecutionDispatch>::ExtendHostFunctions,
            >,
        >(&self.substrate_client_uri, block_number)
        .await?;

        Ok(())
    }

    pub(crate) async fn check_registration(&self) -> eyre::Result<()> {
        let status = self
            .el_contracts
            .is_operator_registered(self.client.address())
            .await?;

        let id = self.avs_contracts.operator_id().await?;
        let local_id = self.bls_keypair.operator_id();

        match (status, id, local_id) {
            (false, _, _) => Err(eyre!("Operator not registered with EigenLayer")),
            (true, None, _) => Err(eyre!("Operator not registered with AVS")),
            (true, Some(id), local) if id == local => Ok(()),
            _ => Err(eyre!(
                "Registered operator id ({:x}) & BlsKeypair.operator_id() ({:x}) mismatch",
                id.unwrap(),
                local_id
            )),
        }
    }

    #[instrument(skip_all)]
    pub(crate) async fn register(&self) -> eyre::Result<()> {
        let status = self
            .el_contracts
            .is_operator_registered(self.client.address())
            .await?;

        if !status {
            info!(
                "Registering Operator {:x} with EigenLayer",
                self.client.address()
            );

            self.el_contracts
                .register_bls_pub_key(&self.bls_keypair, self.chain_id)
                .await?;

            self.el_contracts
                .register_as_operator_with_el(self.client.address())
                .await?;

            info!("Sucessfully registered with EigenLayer")
        }

        if let Some(id) = self.avs_contracts.operator_id().await? {
            info!("Operator already registered, with id {:x}", id);
        } else {
            info!("Registering Operator {:x} with AVS", self.client.address());
            self.avs_contracts
                .register_with_avs(&self.bls_keypair)
                .await?;
            let id = self
                .avs_contracts
                .operator_id()
                .await?
                .expect("should have operator id after success trx");
            info!("Sucessfully registered with AVS with id {:x}", id);
        }

        Ok(())
    }
}

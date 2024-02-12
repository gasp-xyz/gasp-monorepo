use crate::chainio::{avs::AvsContracts, build_eth_client, eigen::ElContracts, Client};
use crate::cli::CliArgs;
use crate::crypto::bn254::{BlsKeypair, OperatorId};
use crate::crypto::EthConvert;
use crate::executor::execute::execute_block;
use crate::rpc::Rpc;

use bindings::{
    mangata_task_manager::NewTaskCreatedFilter,
    shared_types::{G1Point, G2Point, TaskResponse},
};
use ethers::prelude::*;
use node_executor::ExecutorDispatch;
use node_primitives::BlockNumber;

use serde::Serialize;
use sp_runtime::traits::BlakeTwo256;
use sp_runtime::{generic, OpaqueExtrinsic};
use std::sync::Arc;
use tracing::{debug, error, info, instrument};

pub type Header = generic::HeaderVer<node_primitives::BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, OpaqueExtrinsic>;

#[derive(Debug, Serialize)]
pub struct OperatorStatus {
    pub eth_address: Address,
    pub registered_with_eigen: bool,
    pub bls_key_registered: bool,
    pub bls_g1: G1Point,
    pub bls_g2: G2Point,
    pub registered_with_avs: bool,
    pub operator_id: Option<OperatorId>,
    // opted_in_salshing_by_avs: bool,
    // frozen: bool,
}

#[derive(Debug)]
pub struct Operator {
    client: Arc<Client>,
    avs_contracts: AvsContracts,
    el_contracts: ElContracts,
    bls_keypair: BlsKeypair,
    substrate_client_uri: String,
    chain_id: u64,
    rpc: Rpc,
}
impl Operator {
    #[instrument(name = "create_operator", skip_all)]
    pub async fn from_cli(cfg: &CliArgs) -> eyre::Result<Self> {
        let client = Arc::new(build_eth_client(cfg).await?);
        let avs_contracts = AvsContracts::build(cfg, client.clone()).await?;
        let slasher = avs_contracts.slasher_address().await?;
        let el_contracts = ElContracts::build(cfg, slasher, client.clone()).await?;

        info!("Decrypting BLS keypair...");
        let bls_key = cfg.get_bls_keystore()?.into_bls_keypair()?;
        info!(
            "Bls Keypair decrypted with operator id: {:x}",
            bls_key.operator_id()
        );

        let rpc = Rpc::build(cfg);

        Ok(Self {
            avs_contracts,
            el_contracts,
            substrate_client_uri: cfg.substrate_rpc_url.to_owned(),
            client,
            bls_keypair: bls_key,
            chain_id: cfg.chain_id,
            rpc,
        })
    }

    #[instrument(skip_all)]
    pub async fn watch_new_tasks(&self) -> eyre::Result<()> {
        let evs = self.avs_contracts.new_task_stream();
        let mut stream: stream::EventStream<'_, _, NewTaskCreatedFilter, _> =
            evs.subscribe().await?;

        while let Some(Ok(event)) = stream.next().await {
            info!("Executing a Block for task: {:?}", event);
            let block = self.execute_block(event.task.block_number.as_u32()).await?;
            debug!("Block executed successfully");

            let payload = TaskResponse {
                reference_task_index: event.task_index,
                block_hash: block.as_fixed_bytes().to_owned(),
            };

            let response = self
                .rpc
                .send_task_response(payload, &self.bls_keypair)
                .await?;

            match response.error_for_status_ref() {
                Err(e) => error!("{} - {}", e, response.text().await?),
                Ok(_) => info!("Task finished successfuly and sent to AVS service"),
            }
        }
        Ok(())
    }

    pub(crate) async fn execute_block(&self, block_number: BlockNumber) -> eyre::Result<H256> {
        use sc_executor::{sp_wasm_interface::ExtendedHostFunctions, NativeExecutionDispatch};
        let block = execute_block::<
            Block,
            ExtendedHostFunctions<
                sp_io::SubstrateHostFunctions,
                <ExecutorDispatch as NativeExecutionDispatch>::ExtendHostFunctions,
            >,
        >(&self.substrate_client_uri, block_number)
        .await?;

        Ok(block)
    }

    pub(crate) fn operator_id(&self) -> OperatorId {
        self.bls_keypair.operator_id()
    }

    #[instrument(skip_all)]
    pub(crate) async fn get_status(&self) -> eyre::Result<OperatorStatus> {
        let el_status = self
            .el_contracts
            .is_operator_registered(self.client.address())
            .await?;

        let pubkey_status = self
            .el_contracts
            .has_operator_pubkey(self.client.address())
            .await?;

        let id = self.avs_contracts.operator_id().await?;

        Ok(OperatorStatus {
            eth_address: self.client.address(),
            registered_with_eigen: el_status,
            bls_key_registered: pubkey_status,
            bls_g1: EthConvert::to_g1(self.bls_keypair.public).unwrap_or_default(),
            bls_g2: EthConvert::to_g2(self.bls_keypair.public_g2()).unwrap_or_default(),
            operator_id: id,
            registered_with_avs: id.is_some(),
        })
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
        } else {
            info!("Operator already registered in EigenLayer");
        }

        Ok(())
    }

    #[instrument(skip_all)]
    pub(crate) async fn opt_in_avs(&self) -> eyre::Result<()> {
        if self.avs_contracts.operator_id().await?.is_some() {
            info!("Operator already opt-in AVS");
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

    #[instrument(skip_all)]
    pub(crate) async fn opt_out_avs(&self) -> eyre::Result<()> {
        if self.avs_contracts.operator_id().await?.is_some() {
            self.avs_contracts
                .deregister_with_avs(&self.bls_keypair)
                .await?;
            info!("Operator opted out with AVS sucessfully");
        } else {
            info!("Operator not opt in with AVS");
        }

        Ok(())
    }
}

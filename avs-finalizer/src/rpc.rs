use crate::{
    cli::CliArgs,
    crypto::bn254::{BlsKeypair, BlsSignature, PrivateKey},
};
use ark_ec::AffineRepr;
use ark_ff::PrimeField;
use bindings::shared_types::TaskResponse;
use ethers::abi::AbiEncode;
use reqwest::Response;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{
    default_on_request_failure, default_on_request_success, policies::ExponentialBackoff,
    RetryTransientMiddleware, Retryable, RetryableStrategy,
};
use serde::{ser::SerializeStruct, Serialize};
use sp_runtime::traits::{Hash, Keccak256};
use tracing::instrument;

type Bytes32 = [u8; 32];

#[derive(Serialize, Debug)]
struct SignedTaskResponse {
    #[serde(rename = "TaskResponse")]
    task_response: TaskResponseWire,
    #[serde(rename = "BlsSignature")]
    bls_signature: BlsSignatureWire,
    #[serde(rename = "OperatorId")]
    operator_id: Bytes32,
}

#[derive(Serialize, Debug)]
struct TaskResponseWire {
    #[serde(rename = "ReferenceTaskIndex")]
    pub reference_task_index: u32,
    #[serde(rename = "operatorsStateHash")]
    pub operators_state_hash: Bytes32,
    #[serde(rename = "BlockHash")]
    pub block_hash: Bytes32,
    #[serde(rename = "StorageProofHash")]
    pub storage_proof_hash: Bytes32,
    #[serde(rename = "PendingStateHash")]
    pub pending_state_hash: Bytes32,
}

impl From<TaskResponse> for TaskResponseWire {
    fn from(value: TaskResponse) -> Self {
        Self {
            reference_task_index: value.reference_task_index,
            operators_state_hash: value.operators_state_hash,
            block_hash: value.block_hash,
            storage_proof_hash: value.storage_proof_hash,
            pending_state_hash: value.pending_state_hash,
        }
    }
}

#[derive(Serialize, Debug)]
struct BlsSignatureWire {
    g1_point: G1PointWire,
}

impl From<BlsSignature> for BlsSignatureWire {
    fn from(value: BlsSignature) -> Self {
        Self {
            g1_point: G1PointWire {
                x: value.x().unwrap().into_bigint(),
                y: value.y().unwrap().into_bigint(),
            },
        }
    }
}

#[derive(Debug)]
struct G1PointWire {
    x: <PrivateKey as PrimeField>::BigInt,
    y: <PrivateKey as PrimeField>::BigInt,
}

impl Serialize for G1PointWire {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("g1_point", 2)?;
        s.serialize_field("X", &self.x.to_string())?;
        s.serialize_field("Y", &self.y.to_string())?;
        s.end()
    }
}

#[derive(Debug)]
pub struct Rpc {
    client: ClientWithMiddleware,
    avs_url: String,
}

struct RetryFailed;
impl RetryableStrategy for RetryFailed {
    fn handle(&self, res: &reqwest_middleware::Result<reqwest::Response>) -> Option<Retryable> {
        match res {
            // retry if 404 task not initialized, in case block reexecution is faster the aggr task initialization, usually on local testnet
            Ok(success) if success.status() == 404 => Some(Retryable::Transient),
            Ok(success) => default_on_request_success(success),
            Err(error) => default_on_request_failure(error),
        }
    }
}

impl Rpc {
    pub fn build(cfg: &CliArgs) -> Self {
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
        let client = ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy_and_strategy(
                retry_policy,
                RetryFailed,
            ))
            .build();
        Self {
            client,
            avs_url: cfg.avs_rpc_url.to_owned(),
        }
    }

    #[instrument(skip(self, keypair))]
    pub async fn send_task_response(
        &self,
        task_response: TaskResponse,
        keypair: &BlsKeypair,
    ) -> eyre::Result<Response> {
        let req = create_response(task_response, keypair)?;
        println!("{:?}", req);
        let json: String = serde_json::to_string(&req)?;

        Ok(self.client.post(&self.avs_url).body(json).send().await?)
    }
}

fn create_response(task: TaskResponse, keypair: &BlsKeypair) -> eyre::Result<SignedTaskResponse> {
    let encoded = task.clone().encode();

    let hash = Keccak256::hash(encoded.as_ref());
    let sig = keypair.sign(hash.as_bytes())?;

    println!("{:?}", hash);
    println!("{:?}", sig);
    Ok(SignedTaskResponse {
        bls_signature: sig.into(),
        task_response: task.into(),
        operator_id: keypair.operator_id().to_fixed_bytes(),
    })
}

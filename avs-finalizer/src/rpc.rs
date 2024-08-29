use crate::{
    cli::CliArgs,
    crypto::bn254::{BlsKeypair, BlsSignature, PrivateKey},
};
use ark_ec::AffineRepr;
use ark_ff::PrimeField;
use bindings::{
    finalizer_task_manager::OperatorStateInfo,
    shared_types::{OpTask, OpTaskResponse, RdTask, RdTaskResponse, *},
};
use ethers::abi::AbiEncode;
use eyre::eyre;
use reqwest::Response;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{
    default_on_request_failure, default_on_request_success, policies::ExponentialBackoff,
    RetryTransientMiddleware, Retryable, RetryableStrategy,
};
use serde::{ser::SerializeStruct, Serialize};
use sp_core::Bytes;
use sp_runtime::traits::{Hash, Keccak256};
use tracing::instrument;

type Bytes32 = [u8; 32];

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct SignedTaskResponse {
    op_task_response: Bytes,
    rd_task_response: Bytes,
    bls_signature: BlsSignatureWire,
    operator_id: Bytes32,
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
        op_task_response: Option<OpTaskResponse>,
        rd_task_response: Option<RdTaskResponse>,
        keypair: &BlsKeypair,
    ) -> eyre::Result<Response> {
        let req = create_response(op_task_response, rd_task_response, keypair)?;
        println!("req: {:?}", req);
        let json: String = serde_json::to_string(&req)?;

        Ok(self.client.post(&self.avs_url).body(json).send().await?)
    }
}

fn create_response(
    op_task_response: Option<OpTaskResponse>,
    rd_task_response: Option<RdTaskResponse>,
    keypair: &BlsKeypair,
) -> eyre::Result<SignedTaskResponse> {
    match (op_task_response, rd_task_response) {
        (Some(task), None) => {
            let encoded = task.clone().encode();

            let hash = Keccak256::hash(encoded.as_ref());
            let sig = keypair.sign(hash.as_bytes())?;

            return Ok(SignedTaskResponse {
                bls_signature: sig.into(),
                op_task_response: encoded.into(),
                rd_task_response: vec![].into(),
                operator_id: keypair.operator_id().to_fixed_bytes(),
            });
        }
        (None, Some(task)) => {
            let encoded = task.clone().encode();

            let hash = Keccak256::hash(encoded.as_ref());
            let sig = keypair.sign(hash.as_bytes())?;

            Ok(SignedTaskResponse {
                bls_signature: sig.into(),
                op_task_response: vec![].into(),
                rd_task_response: encoded.into(),
                operator_id: keypair.operator_id().to_fixed_bytes(),
            })
        }
        (None, None) => {
            return Err(eyre!("Neither of op and rd task response populated"));
        }
        (Some(_), Some(_)) => {
            return Err(eyre!("Both of op and rd task response populated"));
        }
    }
}

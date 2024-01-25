use ark_ec::AffineRepr;
use ark_ff::PrimeField;
use bindings::shared_types::TaskResponse;
use ethers::abi::AbiEncode;
use reqwest::{Client, Response};
use serde::{ser::SerializeStruct, Serialize};
use sha3::{Digest, Keccak256};
use sp_core::H256;
use tracing::instrument;


use crate::{
    cli::CliArgs,
    crypto::bn254::{BlsKeypair, BlsSignature, PrivateKey},
};

#[derive(Serialize)]
struct SignedTaskResponse {
    #[serde(rename = "TaskResponse")]
    task_response: TaskResponseWire,
    #[serde(rename = "BlsSignature")]
    bls_signature: BlsSignatureWire,
    #[serde(rename = "OperatorId")]
    operator_id: [u8; 32],
}

#[derive(Serialize)]
struct TaskResponseWire {
    #[serde(rename = "ReferenceTaskIndex")]
    pub reference_task_index: u32,
    #[serde(rename = "BlockHash")]
    pub block_hash: [u8; 32],
}

impl From<TaskResponse> for TaskResponseWire {
    fn from(value: TaskResponse) -> Self {
        Self {
            reference_task_index: value.reference_task_index,
            block_hash: value.block_hash,
        }
    }
}

#[derive(Serialize)]
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
    client: Client,
    avs_url: String,
}

impl Rpc {
    pub fn build(cfg: &CliArgs) -> Self {
        Self {
            client: reqwest::Client::new(),
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
        let json: String = serde_json::to_string(&req)?;

        Ok(self.client.post(&self.avs_url).body(json).send().await?)
    }
}

fn create_response(task: TaskResponse, keypair: &BlsKeypair) -> eyre::Result<SignedTaskResponse> {
    let encoded = task.clone().encode();

    let hash = Keccak256::new().chain_update(encoded).finalize();
    let slice: H256 = TryInto::<[u8; 32]>::try_into(hash.as_slice())
        .expect("incorrect size for Keccak256 and H256")
        .into();

    let sig = keypair.sign(slice.as_bytes())?;

    Ok(SignedTaskResponse {
        bls_signature: sig.into(),
        task_response: task.into(),
        operator_id: keypair.operator_id().to_fixed_bytes(),
    })
}

use ark_bn254::{Fq, G1Affine, G2Affine};
use ark_ec::AffineRepr;
use ark_ff::{BigInteger, PrimeField};
use bindings::shared_types::{G1Point, G2Point};
use ethers::core::types::U256;

pub mod bn254;
pub mod keystore;

pub struct EthConvert;
impl EthConvert {
    pub fn to_u256(p: &Fq) -> U256 {
        U256::from_little_endian(&p.into_bigint().to_bytes_le())
    }

    pub fn to_g1(xy: G1Affine) -> Option<G1Point> {
        xy.xy().map(|(x, y)| G1Point {
            x: EthConvert::to_u256(x),
            y: EthConvert::to_u256(y),
        })
    }

    pub fn to_g2(xy: G2Affine) -> Option<G2Point> {
        xy.xy().map(|(x, y)| G2Point {
            x: [EthConvert::to_u256(&x.c1), EthConvert::to_u256(&x.c0)],
            y: [EthConvert::to_u256(&y.c1), EthConvert::to_u256(&y.c0)],
        })
    }
}


use hex_literal::hex;
use hex as hex_encoder;
use rs_merkle::{MerkleProof, MerkleTree, algorithms::Sha256, Hasher};
use serde::{Serialize, Serializer};
use serde_json;
use sha3::{Keccak256, Digest};

#[derive(Clone)]
pub struct Keccak256Hasher {}

impl Hasher for Keccak256Hasher {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> [u8; 32] {
        let mut output = [0u8; 32];
        let hash = Keccak256::digest(&data[..]);
        output.copy_from_slice(&hash[..]);
        output
    }
}

#[derive(Debug)]
struct Hash32([u8; 32]);

impl Serialize for Hash32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let str = format!("0x{}",hex_encoder::encode(&self.0));
        serializer.serialize_str(&str)
    }
}

impl From<[u8; 32]> for Hash32 {
    fn from(bytes: [u8; 32]) -> Self {
        Hash32(bytes)
    }
}

impl From<&[u8; 32]> for Hash32 {
    fn from(bytes: &[u8; 32]) -> Self {
        Hash32(bytes.clone())
    }
}

#[derive(Debug, Serialize)]
struct TestCase {
    leaves: Vec<Hash32>,
    expected_root: Hash32,
    leave_pos: usize, 
    leave_hash: Hash32, 
    proof: Vec<Hash32>,
}

#[derive(Debug, Serialize)]
struct TestData{
    pub cases: Vec<TestCase>,
    pub cases_count: usize
}

fn main() {

        let leaves = [
                hex!("0000000000000000000000000000000000000000000000000000000000000000"), 
                hex!("1111111111111111111111111111111111111111111111111111111111111111"),
                hex!("2222222222222222222222222222222222222222222222222222222222222222"),
                hex!("3333333333333333333333333333333333333333333333333333333333333333"),
                hex!("4444444444444444444444444444444444444444444444444444444444444444"),
                hex!("5555555555555555555555555555555555555555555555555555555555555555"),
                hex!("6666666666666666666666666666666666666666666666666666666666666666"),
                hex!("7777777777777777777777777777777777777777777777777777777777777777"),
        ];
        
        let mut data = TestData{
            cases: Vec::new(),
            cases_count: 0
        };

        for i in 1..=8 {
            let merkle_tree = MerkleTree::<Keccak256Hasher>::from_leaves(
                &leaves[0..i]
            );

            for j in 0..i{
                data.cases.push(
                    TestCase {
                        leaves: merkle_tree.leaves().unwrap().iter().map(Into::into).collect(),
                        expected_root: merkle_tree.root().unwrap().into(),
                        leave_pos: j,
                        leave_hash: leaves[j].into(),
                        proof: merkle_tree.proof(&[j]).proof_hashes().iter().map(Into::into).collect()
                    }
                );
                data.cases_count += 1;
            }
        }
        println!("{}", serde_json::to_string_pretty(&data).unwrap());

    }

use sha3::{Digest, Keccak256};

use secp256k1::{Message, Secp256k1, SecretKey};
use subxt::tx::signer::Signer as SignerT;

use super::GaspAddress;
use super::GaspConfig;
use super::GaspSignature;

use secp256k1::SECP256K1;

pub struct Keypair(pub secp256k1::Keypair);

impl Keypair {
    pub fn from_secret_key(secret: [u8; 32]) -> Self {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&secret[..]).expect("32 bytes, within curve order");
        Keypair(secp256k1::Keypair::from_secret_key(&secp, &secret_key))
    }

    pub fn address(&self) -> GaspAddress {
        let mut res = [0u8; 64];
        res.copy_from_slice(&self.0.public_key().serialize_uncompressed()[1..]);
        let mut buffer = [0u8; 20];
        buffer.copy_from_slice(&Keccak256::digest(res)[12..32]);
        buffer.into()
    }

    pub fn sign_prehashed(&self, message: &[u8; 32]) -> GaspSignature {
        let message = Message::from_digest_slice(message).expect("Message is 32 bytes; qed");
        let secret_key =
            SecretKey::from_slice(&self.0.secret_bytes()).expect("Secret key is 32 bytes; qed");
        let recsig = SECP256K1.sign_ecdsa_recoverable(&message, &secret_key);
        let (recid, sig): (_, [u8; 64]) = recsig.serialize_compact();
        let mut signature_bytes: [u8; 65] = [0; 65];
        signature_bytes[..64].copy_from_slice(&sig);
        signature_bytes[64] = (recid.to_i32() & 0xFF) as u8;
        signature_bytes.into()
    }
}

impl SignerT<GaspConfig> for Keypair {
    fn account_id(&self) -> GaspAddress {
        self.address()
    }

    fn address(&self) -> GaspAddress {
        self.address()
    }

    fn sign(&self, signer_payload: &[u8]) -> GaspSignature {
        let hashed = Keccak256::digest(signer_payload);
        let signature = self.sign_prehashed(&hashed.into());
        signature
    }
}

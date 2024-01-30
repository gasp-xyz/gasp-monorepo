use aes::{
    cipher::{self, InnerIvInit, KeyInit, StreamCipherCore},
    Aes128,
};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::fields::PrimeField;
use eth_keystore::{CryptoJson, KdfparamsType};
use eyre::{eyre, Report};
use scrypt::{scrypt, Params as ScryptParams};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::{fmt::Debug, fs::File, io::Read, path::Path};

use crate::crypto::bn254::{BlsKeypair, PrivateKey, PublicKey};

#[derive(Debug, Deserialize, Serialize)]
struct BlsKeystore {
    pub crypto: CryptoJson,
}

/// Decrypts an encrypted JSON from the provided path to construct a BN254 key
pub fn decrypt_keystore<P, S>(keypath: &P, password: &S) -> eyre::Result<BlsKeypair>
where
    P: AsRef<Path>,
    S: AsRef<[u8]>,
{
    let secret = decrypt_key(keypath, password)?;
    let fr = PrivateKey::from_be_bytes_mod_order(&secret);
    let p = PublicKey::generator() * fr;

    Ok(BlsKeypair {
        private: fr,
        public: p.into_affine(),
    })
}

fn decrypt_key<P, S>(path: P, password: S) -> eyre::Result<Vec<u8>>
where
    P: AsRef<Path>,
    S: AsRef<[u8]>,
{
    // Read the file contents as string and deserialize it.
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let keystore: BlsKeystore = serde_json::from_str(&contents)?;

    // Derive the key.
    let key = match keystore.crypto.kdfparams {
        KdfparamsType::Pbkdf2 { .. } => {
            return Err(Report::msg("Pbkdf2 not supported"));
        }
        KdfparamsType::Scrypt {
            dklen,
            n,
            p,
            r,
            salt,
        } => {
            let mut key = vec![0u8; dklen as usize];
            let log_n = (n as f32).log2() as u8;
            let scrypt_params = ScryptParams::new(log_n, r, p)?;
            scrypt(password.as_ref(), &salt, &scrypt_params, key.as_mut_slice())?;
            key
        }
    };

    // Derive the MAC from the derived key and ciphertext.
    let derived_mac = Keccak256::new()
        .chain_update(&key[16..32])
        .chain_update(&keystore.crypto.ciphertext)
        .finalize();

    if derived_mac.as_slice() != keystore.crypto.mac.as_slice() {
        return Err(eyre!("MacMismatch"));
    }

    // Decrypt the private key bytes using AES-128-CTR
    let decryptor =
        Aes128Ctr::new(&key[..16], &keystore.crypto.cipherparams.iv[..16]).expect("invalid length");

    let mut pk = keystore.crypto.ciphertext;
    decryptor.apply_keystream(&mut pk);

    Ok(pk)
}

struct Aes128Ctr {
    inner: ctr::CtrCore<Aes128, ctr::flavors::Ctr128BE>,
}

impl Aes128Ctr {
    fn new(key: &[u8], iv: &[u8]) -> Result<Self, cipher::InvalidLength> {
        let cipher = aes::Aes128::new_from_slice(key).unwrap();
        let inner = ctr::CtrCore::inner_iv_slice_init(cipher, iv).unwrap();
        Ok(Self { inner })
    }

    fn apply_keystream(self, buf: &mut [u8]) {
        self.inner.apply_keystream_partial(buf.into());
    }
}

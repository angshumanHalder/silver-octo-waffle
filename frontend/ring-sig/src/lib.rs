//! Rust part of Ed25519 Quirks.

mod keygen;
mod signature;
pub mod traits;

use curve25519_dalek::{ristretto::CompressedRistretto, scalar::Scalar};
use digest::Digest;
use keygen::{EDKeyGen, KeyGen};
use rand::AsByteSliceMut;
use rand_core::{CryptoRng, RngCore};
use serde::{Deserialize, Serialize};
use sha3::Keccak512;
use signature::BLSAG;
use traits::Sign;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(feature = "std")]
pub use std::vec::Vec;

////////// Binding to a JavaScript CSPRNG. ////////////

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = getRandomValues, js_namespace = crypto)]
    fn random_bytes(dest: &mut [u8]);
}

/// RNG based on `window.crypto.getRandomValues()`.
pub struct RandomValuesRng;

impl RngCore for RandomValuesRng {
    fn next_u32(&mut self) -> u32 {
        let mut bytes = [0_u8; 4];
        random_bytes(&mut bytes);
        let mut result = bytes[0] as u32;
        for (i, &byte) in bytes.iter().enumerate().skip(1) {
            result += (byte as u32) << (i * 8);
        }
        result
    }

    fn next_u64(&mut self) -> u64 {
        let mut bytes = [0_u8; 8];
        random_bytes(&mut bytes);
        let mut result = bytes[0] as u64;
        for (i, &byte) in bytes.iter().enumerate().skip(1) {
            result += (byte as u64) << (i * 8);
        }
        result
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        random_bytes(dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl CryptoRng for RandomValuesRng {}

#[derive(Serialize, Deserialize)]
pub struct Candidate {
    secret: [u8; 32],
    public: [u8; 32],
}

#[derive(Serialize, Deserialize)]
pub struct Voter {
    secret: [u8; 32],
    public: [u8; 32],
    image: [u8; 32],
}

#[derive(Serialize, Deserialize)]
pub struct Signature {
    pub challenge: [u8; 32],
    pub responses: Vec<[u8; 32]>,
    pub ring: Vec<[u8; 32]>,
    pub key_image: [u8; 32],
}

#[wasm_bindgen(js_name = "genKey")]
pub fn gen_key() -> JsValue {
    let pair = EDKeyGen::new();
    let keys = Candidate {
        secret: pair.private_key.to_bytes(),
        public: pair.public_key.compress().to_bytes(),
    };
    JsValue::from_serde(&keys).unwrap()
}

#[wasm_bindgen(js_name = "genVoter")]
pub fn gen_voter() -> JsValue {
    let keys = KeyGen::new::<Keccak512>();
    let voter = Voter {
        secret: keys.private_key.to_bytes(),
        public: keys.public_key.compress().to_bytes(),
        image: keys.key_image.compress().to_bytes(),
    };
    JsValue::from_serde(&voter).unwrap()
}

#[wasm_bindgen(js_name = "genSignature")]
pub fn gen_signature(
    private_key: JsValue,
    public_key: JsValue,
    key_image: JsValue,
    ring: JsValue,
    secret_index: u32,
    message: String,
) -> JsValue {
    let private: [u8; 32] = private_key.into_serde().unwrap();
    let public: [u8; 32] = public_key.into_serde().unwrap();
    let image: [u8; 32] = key_image.into_serde().unwrap();
    let ring: Vec<[u8; 32]> = ring.into_serde().unwrap();

    let mut message_hash = Keccak512::default();
    message_hash.update(message.as_bytes());

    let mut message_gen: Vec<u8> = message_hash
        .finalize()
        .as_byte_slice_mut()
        .iter()
        .cloned()
        .collect();

    let generate_ring = ring
        .iter()
        .map(|x| CompressedRistretto(*x).decompress().unwrap())
        .collect();
    let blsag = BLSAG::sign::<Keccak512>(
        Scalar::from_bits(private),
        CompressedRistretto(public).decompress().unwrap(),
        CompressedRistretto(image).decompress().unwrap(),
        generate_ring,
        secret_index as usize,
        &mut message_gen,
    );

    // pub ring: Vec<[u8; 32]>,
    // pub key_image: [u8; 32],
    let sig_responses: Vec<[u8; 32]> = blsag.responses.iter().map(|x| x.to_bytes()).collect();
    let signature = Signature {
        challenge: blsag.challenge.to_bytes(),
        responses: sig_responses,
        ring,
        key_image: image,
    };
    JsValue::from_serde(&signature).unwrap()
}

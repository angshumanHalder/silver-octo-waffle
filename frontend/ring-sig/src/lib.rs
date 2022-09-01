mod ballot;
mod keygen;
mod random;
mod signature;
mod traits;

use ballot::Ballot;
use curve25519_dalek::{
    edwards::CompressedEdwardsY, ristretto::CompressedRistretto, scalar::Scalar,
};
use digest::Digest;
use keygen::{EDKeyGen, KeyGen};
use rand::AsByteSliceMut;
use rand_core::RngCore;
use random::RandomValuesRng;
use serde::{Deserialize, Serialize};
use sha3::Keccak512;
use signature::BLSAG;
use traits::{GenerateBallot, Sign};
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
    challenge: [u8; 32],
    responses: Vec<[u8; 32]>,
    ring: Vec<[u8; 32]>,
    key_image: [u8; 32],
}

#[derive(Serialize, Deserialize)]
pub struct CandidateBallot {
    sa: [u8; 32],
    r: [u8; 32],
}

#[wasm_bindgen(js_name = "genCandidate")]
pub fn gen_candidate() -> JsValue {
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

#[wasm_bindgen(js_name = "genVoterFromSecret")]
pub fn gen_voter_from_secret(secret: JsValue) -> JsValue {
    let secret_key: [u8; 32] = secret.into_serde().unwrap();
    let keys = KeyGen::gen_keys_from_secret::<Keccak512>(secret_key);
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
    ballot: JsValue,
) -> JsValue {
    let private: [u8; 32] = private_key.into_serde().unwrap();
    let public: [u8; 32] = public_key.into_serde().unwrap();
    let image: [u8; 32] = key_image.into_serde().unwrap();
    let ring: Vec<[u8; 32]> = ring.into_serde().unwrap();

    let de_ballot: CandidateBallot = ballot.into_serde().unwrap();

    let mut message_hash = Keccak512::default().chain(de_ballot.sa);
    message_hash.update(de_ballot.r);

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

    let sig_responses: Vec<[u8; 32]> = blsag.responses.iter().map(|x| x.to_bytes()).collect();
    let sig_ring = blsag.ring.iter().map(|x| x.compress().to_bytes()).collect();
    let signature = Signature {
        challenge: blsag.challenge.to_bytes(),
        responses: sig_responses,
        ring: sig_ring,
        key_image: image,
    };
    JsValue::from_serde(&signature).unwrap()
}

#[wasm_bindgen(js_name = "genBallot")]
pub fn gen_ballot(shared_public: JsValue, candidate_public: JsValue) -> JsValue {
    let shared_key: [u8; 32] = shared_public.into_serde().unwrap();
    let candidate_key: [u8; 32] = candidate_public.into_serde().unwrap();
    let shared_pk = CompressedEdwardsY(shared_key).decompress().unwrap();
    let candidate_pk = CompressedEdwardsY(candidate_key).decompress().unwrap();
    let ballot = Ballot::generate_ballot::<Keccak512>(shared_pk, candidate_pk);
    let candidate_ballot = CandidateBallot {
        sa: ballot.sa.compress().to_bytes(),
        r: ballot.r.compress().to_bytes(),
    };
    JsValue::from_serde(&candidate_ballot).unwrap()
}

use crate::prelude::*;
use curve25519_dalek::constants;
use curve25519_dalek::edwards::{CompressedEdwardsY, EdwardsPoint};
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::traits::MultiscalarMul;
use digest::generic_array::typenum::U64;
use digest::Digest;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId, Timestamp};
use rand::AsByteSliceMut;
use sha3::Keccak512;
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone, Copy)]
#[serde(crate = "near_sdk::serde")]
pub enum PollStatus {
    REGISTERED = 0,
    STARTED = 1,
    ENDED = 2,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Candidate {
    pub name: String,
    pub party_name: String,
    pub public_key: [u8; 32],
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Ballot {
    pub sa: [u8; 32],
    pub r: [u8; 32],
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Signature {
    pub challenge: [u8; 32],
    pub responses: Vec<[u8; 32]>,
    pub ring: Vec<[u8; 32]>,
    pub key_image: [u8; 32],
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Vote {
    signature: Signature,
    ballot: Ballot,
}

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct Poll {
    pub poll_id: u32,
    pub created_at: Timestamp,
    pub poll_owner: AccountId,
    pub poll_status: PollStatus,
    pub candidates: Vec<Candidate>,
    shared_sk: [u8; 32],
    pub shared_pk: [u8; 32],
    voters: HashSet<[u8; 32]>,
    pub votes: Vec<Vote>,
    pub seen_key_images: HashSet<[u8; 32]>,
    pub results: HashMap<String, u32>,
    pub defaulters: Vec<[u8; 32]>,
}

impl Poll {
    pub fn new(
        poll_id: u32,
        poll_owner: AccountId,
        candidates: Vec<Candidate>,
        shared_sk: [u8; 32],
        shared_pk: [u8; 32],
    ) -> Self {
        // let shared_sk = Scalar::random(&mut OsRng::default());
        // let shared_pk = constants::ED25519_BASEPOINT_POINT * shared_sk;

        let mut results = HashMap::<String, u32>::new();
        for c in candidates.iter() {
            results.insert(c.party_name.clone(), 0);
        }

        Poll {
            poll_id,
            created_at: env::block_timestamp(),
            poll_owner,
            shared_sk,
            shared_pk,
            poll_status: PollStatus::REGISTERED,
            candidates,
            voters: HashSet::<[u8; 32]>::new(),
            votes: Vec::<Vote>::new(),
            seen_key_images: HashSet::<[u8; 32]>::new(),
            results,
            defaulters: Vec::<[u8; 32]>::new(),
        }
    }

    pub fn change_poll_status(&mut self, status: PollStatus) -> bool {
        if env::signer_account_id() == self.poll_owner {
            self.poll_status = status;
            return true;
        }
        false
    }

    pub fn add_voter(&mut self, voter: [u8; 32]) -> bool {
        self.voters.insert(voter);
        true
    }

    pub fn vote(&mut self, ballot: Ballot, signature: Signature) -> bool {
        if self.seen_key_images.contains(&signature.key_image.clone()) {
            return false;
        }
        self.votes.push(Vote { ballot, signature });
        true
    }

    pub fn tally(&mut self) {
        let shared_sk = Scalar::from_bits(self.shared_sk);
        for vote in &self.votes {
            if !self.voters.contains(&vote.signature.key_image) {
                continue;
            }
            if self.seen_key_images.contains(&vote.signature.key_image) {
                self.defaulters.push(vote.signature.key_image);
                continue;
            }
            let challenge: Scalar = Scalar::from_bits(vote.signature.challenge);
            let responses: Vec<Scalar> = vote
                .signature
                .responses
                .iter()
                .map(|x| Scalar::from_bits(x.clone()))
                .collect();
            let ring: Vec<RistrettoPoint> = vote
                .signature
                .ring
                .iter()
                .map(|x| CompressedRistretto(x.clone()).decompress().unwrap())
                .collect();
            let key_image = CompressedRistretto(vote.signature.key_image.clone())
                .decompress()
                .unwrap();
            let mut message_hash =
                Keccak512::default().chain(CompressedEdwardsY(vote.ballot.r).as_bytes());
            message_hash.update(vote.ballot.sa);

            let message: Vec<u8> = message_hash
                .finalize()
                .as_byte_slice_mut()
                .iter()
                .cloned()
                .collect();

            let is_verified = verify::<Keccak512>(challenge, responses, ring, key_image, &message);
            if !is_verified {
                continue;
            }
            let r = CompressedEdwardsY(vote.ballot.r).decompress().unwrap();
            let sa = CompressedEdwardsY(vote.ballot.sa).decompress().unwrap();
            for candidate in self.candidates.iter() {
                let candidate_ballot = verify_ballot::<Keccak512>(
                    shared_sk,
                    CompressedEdwardsY(candidate.public_key)
                        .decompress()
                        .unwrap(),
                    r,
                    sa,
                );
                if candidate_ballot {
                    let v = self.results.get(&candidate.party_name).unwrap();
                    //  find a better way to calculate the votes
                    self.results.insert(candidate.party_name.clone(), v + 1);
                }
            }
        }
    }
}

fn verify_ballot<Hash: Digest<OutputSize = U64> + Clone + Default>(
    shared_sk: Scalar,
    candidate_pk: EdwardsPoint,
    r: EdwardsPoint,
    sa: EdwardsPoint,
) -> bool {
    let ra = shared_sk * r;
    let h_ra = Hash::default().chain(ra.compress().as_bytes());
    let g_hra = constants::ED25519_BASEPOINT_POINT * Scalar::from_hash(h_ra);
    let sa_generated = g_hra + candidate_pk;
    sa == sa_generated
}

fn verify<Hash: Digest<OutputSize = U64> + Clone + Default>(
    challenge: Scalar,
    responses: Vec<Scalar>,
    ring: Vec<RistrettoPoint>,
    key_image: RistrettoPoint,
    message: &Vec<u8>,
) -> bool {
    let mut reconstructed_c: Scalar = challenge;
    let n = ring.len();
    for j in 0..n {
        let mut h: Hash = Hash::default();
        h.update(message);
        h.update(
            RistrettoPoint::multiscalar_mul(
                &[responses[j], reconstructed_c],
                &[constants::RISTRETTO_BASEPOINT_POINT, ring[j]],
            )
            .compress()
            .as_bytes(),
        );

        h.update(
            RistrettoPoint::multiscalar_mul(
                &[responses[j], reconstructed_c],
                &[
                    RistrettoPoint::from_hash(Hash::default().chain(ring[j].compress().as_bytes())),
                    key_image,
                ],
            )
            .compress()
            .as_bytes(),
        );
        reconstructed_c = Scalar::from_hash(h);
    }

    return challenge == reconstructed_c;
}

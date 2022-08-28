use crate::prelude::*;
use digest::generic_array::typenum::U64;
use digest::Digest;

pub trait Verify {
    fn verify<Hash: Digest<OutputSize = U64> + Clone + Default>(
        signature: Self,
        message: &Vec<u8>,
    ) -> bool;
}

pub trait VerifyBallot<SharedKey, CandidatePublicKey, Ballot> {
    fn verify_ballot<Hash: Digest<OutputSize = U64> + Clone + Default>(
        shared_sk: SharedKey,
        candidate_pk: CandidatePublicKey,
        ballot: Ballot,
    ) -> bool;
}

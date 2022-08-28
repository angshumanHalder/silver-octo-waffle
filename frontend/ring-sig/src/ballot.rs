use curve25519_dalek::{constants, edwards::EdwardsPoint, scalar::Scalar};

use crate::{random::RandomValuesRng, traits::GenerateBallot};
use digest::{consts::U64, Digest};
use rand_core::RngCore;

pub struct Ballot {
    pub sa: EdwardsPoint,
    pub r: EdwardsPoint,
}

impl GenerateBallot<EdwardsPoint, EdwardsPoint> for Ballot {
    fn generate_ballot<Hash: Digest<OutputSize = U64> + Clone + Default>(
        shared_pk: EdwardsPoint,
        candidate_pk: EdwardsPoint,
    ) -> Ballot {
        let mut secret = [0_u8; 32];
        RandomValuesRng.fill_bytes(&mut secret);

        let random = Scalar::from_bytes_mod_order(secret);

        let r = constants::ED25519_BASEPOINT_POINT * random;

        let rs = shared_pk * random;

        let h_rs = Hash::default().chain(rs.compress().as_bytes());
        let g_hrs = constants::ED25519_BASEPOINT_POINT * Scalar::from_hash(h_rs);
        let sa = g_hrs + candidate_pk;
        return Ballot { sa, r };
    }
}

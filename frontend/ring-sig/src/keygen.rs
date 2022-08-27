use curve25519_dalek::{
    constants, edwards::EdwardsPoint, ristretto::RistrettoPoint, scalar::Scalar,
};
use digest::{consts::U64, Digest};
use rand_core::RngCore;
use sha3::Keccak512;

use crate::RandomValuesRng;

pub struct KeyGen {
    pub private_key: Scalar,
    pub public_key: RistrettoPoint,
    pub key_image: RistrettoPoint,
}

impl KeyGen {
    pub fn new<Hash: Digest<OutputSize = U64> + Clone + Default>() -> Self {
        let mut secret = [0_u8; 32];
        RandomValuesRng.fill_bytes(&mut secret);
        let secret: Scalar = Scalar::from_bytes_mod_order(secret);
        let public: RistrettoPoint = secret * constants::RISTRETTO_BASEPOINT_POINT;
        let key_img: RistrettoPoint = secret
            * RistrettoPoint::from_hash(Keccak512::default().chain(public.compress().as_bytes()));
        Self {
            private_key: secret,
            public_key: public,
            key_image: key_img,
        }
    }
}

pub struct EDKeyGen {
    pub private_key: Scalar,
    pub public_key: EdwardsPoint,
}

impl EDKeyGen {
    pub fn new() -> Self {
        let mut secret = [0_u8; 32];
        RandomValuesRng.fill_bytes(&mut secret);
        let private_key: Scalar = Scalar::from_bytes_mod_order(secret);
        let public_key = private_key * constants::ED25519_BASEPOINT_POINT;
        return EDKeyGen {
            private_key,
            public_key,
        };
    }
}

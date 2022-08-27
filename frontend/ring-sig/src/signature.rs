use curve25519_dalek::constants;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::traits::MultiscalarMul;
use digest::generic_array::typenum::U64;
use digest::Digest;
use rand_core::RngCore;

use crate::traits::Sign;
use crate::RandomValuesRng;

pub struct BLSAG {
    pub challenge: Scalar,
    pub responses: Vec<Scalar>,
    pub ring: Vec<RistrettoPoint>,
    pub key_image: RistrettoPoint,
}

impl Sign<Scalar, RistrettoPoint, RistrettoPoint, Vec<RistrettoPoint>> for BLSAG {
    /// To sign you need `k` your private key, and `ring` which is the public keys of everyone
    /// except you. You are signing the `message`
    fn sign<Hash: Digest<OutputSize = U64> + Clone + Default>(
        private_key: Scalar,
        public_key: RistrettoPoint,
        key_image: RistrettoPoint,
        mut ring: Vec<RistrettoPoint>,
        secret_index: usize,
        message: &Vec<u8>,
    ) -> BLSAG {
        let n = ring.len() + 1;

        ring.insert(secret_index, public_key);

        let mut secret = [0_u8; 32];
        RandomValuesRng.fill_bytes(&mut secret);
        let a: Scalar = Scalar::from_bytes_mod_order(secret);

        let mut rs: Vec<Scalar> = (0..n)
            .map(|_| Scalar::from_bytes_mod_order(secret))
            .collect();

        let mut cs: Vec<Scalar> = (0..n).map(|_| Scalar::zero()).collect();

        // Hash of message is shared by all challenges H_n(m, ....)
        let mut message_hash = Hash::default();

        message_hash.update(message);

        let mut hashes: Vec<Hash> = (0..n).map(|_| message_hash.clone()).collect();
        hashes[(secret_index + 1) % n].update(
            (a * constants::RISTRETTO_BASEPOINT_POINT)
                .compress()
                .as_bytes(),
        );
        hashes[(secret_index + 1) % n].update(
            (a * RistrettoPoint::from_hash(
                Hash::default().chain(public_key.compress().as_bytes()),
            ))
            .compress()
            .as_bytes(),
        );
        cs[(secret_index + 1) % n] = Scalar::from_hash(hashes[(secret_index + 1) % n].clone());

        let mut i = (secret_index + 1) % n;

        loop {
            hashes[(i + 1) % n].update(
                RistrettoPoint::multiscalar_mul(
                    &[rs[i % n], cs[i % n]],
                    &[constants::RISTRETTO_BASEPOINT_POINT, ring[i % n]],
                )
                .compress()
                .as_bytes(),
            );
            hashes[(i + 1) % n].update(
                RistrettoPoint::multiscalar_mul(
                    &[rs[i % n], cs[i % n]],
                    &[
                        RistrettoPoint::from_hash(
                            Hash::default().chain(ring[i % n].compress().as_bytes()),
                        ),
                        key_image,
                    ],
                )
                .compress()
                .as_bytes(),
            );
            cs[(i + 1) % n] = Scalar::from_hash(hashes[(i + 1) % n].clone());

            if secret_index >= 1 && i % n == (secret_index - 1) % n {
                break;
            } else if secret_index == 0 && i % n == n - 1 {
                break;
            } else {
                i = (i + 1) % n;
            }
        }

        rs[secret_index] = a - (cs[secret_index] * private_key);

        return BLSAG {
            challenge: cs[0],
            responses: rs,
            ring,
            key_image,
        };
    }
}

use digest::generic_array::typenum::U64;
use digest::Digest;

pub trait Sign<PrivateKey, PublicKey, KeyImage, Ring> {
    fn sign<Hash: Digest<OutputSize = U64> + Clone + Default>(
        private_key: PrivateKey,
        public_key: PublicKey,
        key_image: KeyImage,
        ring: Ring,
        secret_index: usize,
        message: &Vec<u8>,
    ) -> Self;
}

pub trait GenerateBallot<SharedKey, CandidatePublicKey> {
    fn generate_ballot<Hash: Digest<OutputSize = U64> + Clone + Default>(
        shared_key: SharedKey,
        candidate_pk: CandidatePublicKey,
    ) -> Self;
}

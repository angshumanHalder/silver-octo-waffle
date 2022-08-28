use crate::random_bytes;
use rand_core::{CryptoRng, RngCore};

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

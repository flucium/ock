use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use crate::size::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct Rand;

impl rand_core::CryptoRng for Rand {}

impl rand_core::RngCore for Rand {
    fn next_u32(&mut self) -> u32 {
        let mut buf: [u8; 4] = [0; 4];

        self.fill_bytes(&mut buf);

        u32::from_le_bytes(buf)
    }

    fn next_u64(&mut self) -> u64 {
        let mut buf: [u8; 8] = [0; 8];

        self.fill_bytes(&mut buf);

        u64::from_be_bytes(buf)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.try_fill_bytes(dest).unwrap();
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        dest.copy_from_slice(&gen_32());
        Ok(())
    }
}

/// ChaCha20Rng
//
/// Generates a 32-byte cryptographic pseudo random number.
/// Same as pub fn gen_32() -> [u8;SIZE_U32]{}
pub fn generate() -> [u8; SIZE_U32] {
    gen_32()
}

/// ChaCha20Rng
///
/// Generates a 32-byte cryptographic pseudo random number.
pub(crate) fn gen_32() -> [u8; SIZE_U32] {
    let mut rng = ChaCha20Rng::from_entropy();

    let bytes = rng.gen::<[u8; 32]>();

    bytes
}

// ChaCha20Rng
//
// Generates a 24-byte cryptographic pseudo random number.
// pub(crate) fn gen_24() -> [u8; SIZE_U24] {
//     let mut rng = ChaCha20Rng::from_entropy();

//     let bytes = rng.gen::<[u8; 24]>();

//     bytes
// }

// ChaCha20Rng
//
// Generates a 16-byte cryptographic pseudo random number.
// pub(crate) fn gen_16() -> [u8; SIZE_U16] {
//     let mut rng = ChaCha20Rng::from_entropy();

//     let bytes = rng.gen::<[u8; 16]>();

//     bytes
// }

// ChaCha20Rng
//
// Generates a 12-byte cryptographic pseudo random number.
// pub(crate) fn gen_12() -> [u8; SIZE_U12] {
//     let mut rng = ChaCha20Rng::from_entropy();

//     let bytes = rng.gen::<[u8; 12]>();

//     bytes
// }
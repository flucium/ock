use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
// pub use rand_chacha::rand_core::OsRng;
use crate::size::*;

pub struct Rand;

/// ChaCha20Rng
///
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

/// ChaCha20Rng
///
/// Generates a 24-byte cryptographic pseudo random number.
pub(crate) fn gen_24() -> [u8; SIZE_U24] {
    let mut rng = ChaCha20Rng::from_entropy();

    let bytes = rng.gen::<[u8; 24]>();

    bytes
}

/// ChaCha20Rng
///
/// Generates a 16-byte cryptographic pseudo random number.
pub(crate) fn gen_16() -> [u8; SIZE_U16] {
    let mut rng = ChaCha20Rng::from_entropy();

    let bytes = rng.gen::<[u8; 16]>();

    bytes
}

/// ChaCha20Rng
///
/// Generates a 12-byte cryptographic pseudo random number.
pub(crate) fn gen_12() -> [u8; SIZE_U12] {
    let mut rng = ChaCha20Rng::from_entropy();

    let bytes = rng.gen::<[u8; 12]>();

    bytes
}

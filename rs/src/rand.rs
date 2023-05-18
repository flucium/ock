use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
// pub use rand_chacha::rand_core::OsRng;

pub fn generate() -> [u8; 32] {
    let mut rng = ChaCha20Rng::from_entropy();

    let bytes = rng.gen::<[u8; 32]>();

    bytes
}

pub fn gen_32() -> [u8; 32] {
    let mut rng = ChaCha20Rng::from_entropy();

    let bytes = rng.gen::<[u8; 32]>();

    bytes
}

pub fn gen_24() -> [u8; 24] {
    let mut rng = ChaCha20Rng::from_entropy();

    let bytes = rng.gen::<[u8; 24]>();

    bytes
}

pub fn gen_16() -> [u8; 16] {
    let mut rng = ChaCha20Rng::from_entropy();

    let bytes = rng.gen::<[u8; 16]>();

    bytes
}

pub fn gen_12() -> [u8; 12] {
    let mut rng = ChaCha20Rng::from_entropy();

    let bytes = rng.gen::<[u8; 12]>();

    bytes
}

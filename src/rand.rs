use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

//Rust Crypto
// pub use aes_gcm::aead::OsRng;

pub fn generate() -> [u8; 32] {
    let mut rng = ChaCha20Rng::from_entropy();

    let bytes = rng.gen::<[u8; 32]>();

    bytes
}
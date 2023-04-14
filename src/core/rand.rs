// https://github.com/rust-random
// https://github.com/rust-random/rand

pub use rand::*;
pub use rand_chacha::*;

pub fn random() ->[u8;32]{
    chacha20_rng()
}

pub fn chacha20_rng() -> [u8; 32] {
    ChaCha20Rng::from_entropy().gen()
}
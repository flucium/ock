// https://github.com/rust-random
// https://github.com/rust-random/rand

pub use rand::*;
pub use rand_chacha::*;

// #[inline]
// pub fn random() ->[u8;32]{
//     chacha20_rng()
// }

// #[inline]
// pub fn chacha20_rng() -> [u8; 32] {
//     ChaCha20Rng::from_entropy().gen()
// }

// #[inline]
// pub fn random(size:usize){}

// #[inline]
// pub fn random_32bytes(){}

// #[inline]
// pub fn random_16bytes(){}
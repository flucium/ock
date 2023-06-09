use crate::size::*;
use digest::Digest;
use sha2::{Sha256, Sha512, Sha512_256};

/// BLAKE3 KDF default context
pub const BLAKE3_DEFAULT_CONTEXT: &str = "OPENCK BLAKE3 VERSION 0.0.1";

/// BLAKE3
//pub fn blake3(bytes: &[u8]) -> [u8; BLAKE3_LENGTH] {
pub fn blake3(bytes: &[u8]) -> [u8; SIZE_U32] {
    blake3::hash(bytes).into()
}

/// BLAKE3 XOF(extend)
pub fn blake3_xof(bytes: &[u8], output: &mut [u8]) {
    blake3::Hasher::new()
        .update(bytes)
        .finalize_xof()
        .fill(output);
}

/// BLAKE3 Key Derivation
// pub fn blake3_kdf(ikm: &[u8], context: &str) -> [u8; BLAKE3_KDF_LENGTH] {
pub fn blake3_kdf(ikm: &[u8], context: &str) -> [u8; SIZE_U32] {
    blake3::derive_key(context, ikm)
}

/// BLAKE3 MAC
// pub fn blake3_mac(key: &[u8; 32], msg: &[u8]) -> [u8; BLAKE3_MAC_LENGTH] {
pub fn blake3_mac(key: &[u8; 32], msg: &[u8]) -> [u8; SIZE_U32] {
    blake3::keyed_hash(key, msg).into()
}

/// SHA256
pub fn sha256(bytes: &[u8]) -> [u8; SIZE_U32] {
    Sha256::digest(bytes).into()
}

/// SHA512
pub fn sha512(bytes: &[u8]) -> [u8; SIZE_U64] {
    Sha512::digest(bytes).into()
}

/// SHA512/256
pub fn sha512_256(bytes: &[u8]) -> [u8; SIZE_U32] {
    Sha512_256::digest(bytes).into()
}

pub fn hkdf_sha256() {}

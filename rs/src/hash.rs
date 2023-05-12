use digest::Digest;

pub const BLAKE3_DEFAULT_CONTEXT: &str = "";


// BLAKE3
pub fn blake3(bytes: &[u8]) -> [u8; 32] {
    blake3::hash(bytes).into()
}

/// BLAKE3 XOF
pub fn blake3_extend(bytes: &[u8], output: &mut [u8]) {
    blake3::Hasher::new()
        .update(bytes)
        .finalize_xof()
        .fill(output);
}

/// BLAKE3 Key Derivation
pub fn blake3_derive_key(ikm: &[u8], context: &str) -> [u8; 32] {
    blake3::derive_key(context, ikm)
}

/// BLAKE3 MAC
pub fn blake3_mac(key: &[u8; 32], msg: &[u8]) -> [u8; 32] {
    blake3::keyed_hash(key, msg).into()
}

/// SHA2 256
pub fn sha256(bytes: &[u8]) -> [u8; 32] {
    sha2::Sha256::digest(bytes).into()
}

/// SHA2 512
pub fn sha512(bytes: &[u8]) -> [u8; 64] {
    sha2::Sha512::digest(bytes).into()
}

/// SHA2 512/256
pub fn sha512_256(bytes: &[u8]) -> [u8; 32] {
    sha2::Sha512_256::digest(bytes).into()
}

mod chacha;
use crate::{size::*, Error, ErrorKind, Result};
use aead::{Aead, KeyInit, Payload};
use chacha::ChaCha20Poly1305;

pub fn sym_encrypt(
    key: &[u8; SIZE_U32],
    nonce: &[u8; SIZE_U12],
    aad: &[u8],
    plain: &[u8],
) -> Result<Vec<u8>> {
    let mac = crate::hash::blake3_mac(key, nonce);
    let hash: &[u8; SIZE_U12] = mac.get(0..12).unwrap().try_into().unwrap();

    let mut cipher = ChaCha20Poly1305::new_from_slice(key)
        .map_err(|err| Error::new(ErrorKind::Unknown, err.to_string()))?
        .encrypt(
            hash.into(),
            Payload {
                msg: plain,
                aad: aad,
            },
        )
        .map_err(|err| Error::new(ErrorKind::Unknown, err.to_string()))?;
    cipher.append(&mut nonce.to_vec());

    Ok(cipher)
}

pub fn sym_decrypt(key: &[u8; SIZE_U32], aad: &[u8], cipher: &[u8]) -> Result<Vec<u8>> {
    let len = cipher.len() - SIZE_U12;

    let c = cipher.get(0..len).unwrap();

    let n = cipher.get(len..).unwrap();

    let mac = crate::hash::blake3_mac(key, n);
    let hash: &[u8; SIZE_U12] = mac.get(0..12).unwrap().try_into().unwrap();

    ChaCha20Poly1305::new_from_slice(key)
        .map_err(|err| Error::new(ErrorKind::Unknown, err.to_string()))?
        .decrypt(hash.into(), Payload { msg: c, aad: aad })
        .map_err(|err| Error::new(ErrorKind::Unknown, err.to_string()))
}
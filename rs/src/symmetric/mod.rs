mod chacha;
use self::chacha::{ChaCha20Blake3, XChaCha20Blake3};
use crate::{size::*, Error, ErrorKind, Result};
use aead::{Aead, KeyInit, Payload};
use chacha20poly1305::ChaCha20Poly1305;

pub fn sym_encrypt(
    key: &[u8; SIZE_U32],
    nonce: &[u8; SIZE_U12],
    aad: &[u8],
    plain: &[u8],
) -> Result<Vec<u8>> {
    match ChaCha20Poly1305::new_from_slice(key) {
        Err(err) => Err(Error::new(ErrorKind::Unknown, err.to_string())),
        Ok(ok) => match ok.encrypt(
            nonce.into(),
            Payload {
                msg: plain,
                aad: aad,
            },
        ) {
            Err(err) => Err(Error::new(ErrorKind::Unknown, err.to_string())),
            Ok(cipher) => Ok(cipher),
        },
    }
}

pub fn sym_decrypt(
    key: &[u8; SIZE_U32],
    nonce: &[u8; SIZE_U12],
    aad: &[u8],
    cipher: &[u8],
) -> Result<Vec<u8>> {
    match ChaCha20Poly1305::new_from_slice(key) {
        Err(err) => Err(Error::new(ErrorKind::Unknown, err.to_string())),
        Ok(ok) => match ok.encrypt(
            nonce.into(),
            Payload {
                msg: cipher,
                aad: aad,
            },
        ) {
            Err(err) => Err(Error::new(ErrorKind::Unknown, err.to_string())),
            Ok(plain) => Ok(plain),
        },
    }
}

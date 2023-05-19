use crate::{size::*, Error, ErrorKind, Result};
use aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, XChaCha20Poly1305};

pub fn chacha20_poly1305_encrypt(
    key: &[u8; SIZE_U32],
    nonce: &[u8; SIZE_U12],
    plain: &[u8],
) -> Result<Vec<u8>> {
    match ChaCha20Poly1305::new_from_slice(key) {
        Err(err) => Err(Error::new(ErrorKind::Unknown, err.to_string())),
        Ok(ok) => match ok.encrypt(nonce.into(), plain) {
            Err(err) => Err(Error::new(ErrorKind::Unknown, err.to_string())),
            Ok(cipher) => Ok(cipher),
        },
    }
}

pub fn chacha20_poly1305_decrypt(
    key: &[u8; SIZE_U32],
    nonce: &[u8; SIZE_U12],
    cipher: &[u8],
) -> Result<Vec<u8>> {
    match ChaCha20Poly1305::new_from_slice(key) {
        Err(err) => Err(Error::new(ErrorKind::Unknown, err.to_string())),
        Ok(ok) => match ok.decrypt(nonce.into(), cipher) {
            Err(err) => Err(Error::new(ErrorKind::Unknown, err.to_string())),
            Ok(plain) => Ok(plain),
        },
    }
}

pub fn xchacha20_poly1305_encrypt(
    key: &[u8; SIZE_U32],
    nonce: &[u8; SIZE_U24],
    plain: &[u8],
) -> Result<Vec<u8>> {
    match XChaCha20Poly1305::new_from_slice(key) {
        Err(err) => Err(Error::new(ErrorKind::Unknown, err.to_string())),
        Ok(ok) => match ok.encrypt(nonce.into(), plain) {
            Err(err) => Err(Error::new(ErrorKind::Unknown, err.to_string())),
            Ok(cipher) => Ok(cipher),
        },
    }
}

pub fn xchacha20_poly1305_decrypt(
    key: &[u8; SIZE_U32],
    nonce: &[u8; SIZE_U24],
    cipher: &[u8],
) -> Result<Vec<u8>> {
    match XChaCha20Poly1305::new_from_slice(key) {
        Err(err) => Err(Error::new(ErrorKind::Unknown, err.to_string())),
        Ok(ok) => match ok.decrypt(nonce.into(), cipher) {
            Err(err) => Err(Error::new(ErrorKind::Unknown, err.to_string())),
            Ok(plain) => Ok(plain),
        },
    }
}

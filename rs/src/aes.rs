use crate::size::*;
use crate::Error;
use crate::ErrorKind;
use crate::Result;
use aead::{Aead, KeyInit};
use aes_gcm::{Aes128Gcm, Aes256Gcm};
type Aes192Gcm = aes_gcm::AesGcm<aes_gcm::aes::Aes192, aead::consts::U12>;

// pub fn aes_128_gcm_encrypt(key: &[u8; 16], nonce: &[u8; 12], plain: &[u8]) -> Result<Vec<u8>> {
pub fn aes_128_gcm_encrypt(
    key: &[u8; SIZE_U16],
    nonce: &[u8; SIZE_U12],
    plain: &[u8],
) -> Result<Vec<u8>> {
    match Aes128Gcm::new_from_slice(key) {
        Err(err) => return Err(Error::new(ErrorKind::Unknown, err.to_string())),
        Ok(ok) => match ok.encrypt(nonce.into(), plain) {
            Err(err) => return Err(Error::new(ErrorKind::Unknown, err.to_string())),
            Ok(cipher) => Ok(cipher),
        },
    }
}

// pub fn aes_128_gcm_decrypt(key: &[u8; 16], nonce: &[u8; 12], cipher: &[u8]) -> Result<Vec<u8>> {
pub fn aes_128_gcm_decrypt(
    key: &[u8; SIZE_U16],
    nonce: &[u8; SIZE_U12],
    cipher: &[u8],
) -> Result<Vec<u8>> {
    match Aes128Gcm::new_from_slice(key) {
        Err(err) => return Err(Error::new(ErrorKind::Unknown, err.to_string())),
        Ok(ok) => match ok.decrypt(nonce.into(), cipher) {
            Err(err) => return Err(Error::new(ErrorKind::Unknown, err.to_string())),
            Ok(plain) => Ok(plain),
        },
    }
}

// pub fn aes_192_gcm_encrypt(key: &[u8; 24], nonce: &[u8; 12], plain: &[u8]) -> Result<Vec<u8>> {
pub fn aes_192_gcm_encrypt(
    key: &[u8; SIZE_U24],
    nonce: &[u8; SIZE_U12],
    plain: &[u8],
) -> Result<Vec<u8>> {
    match Aes192Gcm::new_from_slice(key) {
        Err(err) => return Err(Error::new(ErrorKind::Unknown, err.to_string())),
        Ok(ok) => match ok.encrypt(nonce.into(), plain) {
            Err(err) => return Err(Error::new(ErrorKind::Unknown, err.to_string())),
            Ok(cipher) => Ok(cipher),
        },
    }
}

// pub fn aes_192_gcm_decrypt(key: &[u8; 24], nonce: &[u8; 12], cipher: &[u8]) -> Result<Vec<u8>> {
pub fn aes_192_gcm_decrypt(
    key: &[u8; SIZE_U24],
    nonce: &[u8; SIZE_U12],
    cipher: &[u8],
) -> Result<Vec<u8>> {
    match Aes192Gcm::new_from_slice(key) {
        Err(err) => return Err(Error::new(ErrorKind::Unknown, err.to_string())),
        Ok(ok) => match ok.decrypt(nonce.into(), cipher) {
            Err(err) => return Err(Error::new(ErrorKind::Unknown, err.to_string())),
            Ok(plain) => Ok(plain),
        },
    }
}

// pub fn aes_256_gcm_encrypt(key: &[u8; 32], nonce: &[u8; 12], plain: &[u8]) -> Result<Vec<u8>> {
pub fn aes_256_gcm_encrypt(
    key: &[u8; SIZE_U32],
    nonce: &[u8; SIZE_U12],
    plain: &[u8],
) -> Result<Vec<u8>> {
    match Aes256Gcm::new_from_slice(key) {
        Err(err) => return Err(Error::new(ErrorKind::Unknown, err.to_string())),
        Ok(ok) => match ok.encrypt(nonce.into(), plain) {
            Err(err) => return Err(Error::new(ErrorKind::Unknown, err.to_string())),
            Ok(cipher) => Ok(cipher),
        },
    }
}

// pub fn aes_256_gcm_decrypt(key: &[u8; 32], nonce: &[u8; 12], cipher: &[u8]) -> Result<Vec<u8>> {
pub fn aes_256_gcm_decrypt(
    key: &[u8; SIZE_U32],
    nonce: &[u8; SIZE_U12],
    cipher: &[u8],
) -> Result<Vec<u8>> {
    match Aes256Gcm::new_from_slice(key) {
        Err(err) => return Err(Error::new(ErrorKind::Unknown, err.to_string())),
        Ok(ok) => match ok.decrypt(nonce.into(), cipher) {
            Err(err) => return Err(Error::new(ErrorKind::Unknown, err.to_string())),
            Ok(plain) => Ok(plain),
        },
    }
}

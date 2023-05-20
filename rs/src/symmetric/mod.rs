// mod aes;
// mod chacha;
use crate::{size::*, Error, ErrorKind, Result};
use aead::{Aead, KeyInit};

use chacha20poly1305::ChaCha20Poly1305;
pub struct Symmetric(ChaCha20Poly1305);

impl Symmetric {
    pub fn new(key: &[u8; SIZE_U32]) -> Result<Self> {
        match ChaCha20Poly1305::new_from_slice(key) {
            Err(err) => Err(Error::new(ErrorKind::Unknown, err.to_string())),
            Ok(ok) => Ok(Self(ok)),
        }
    }

    pub fn encrypt(&mut self, nonce: &[u8; SIZE_U12], plain: &[u8]) -> Result<Vec<u8>> {
        match self.0.encrypt(nonce.into(), plain) {
            Err(err) => Err(Error::new(ErrorKind::Unknown, err.to_string())),
            Ok(cipher) => Ok(cipher),
        }
    }

    pub fn decrypt(&mut self, nonce: &[u8; SIZE_U12], cipher: &[u8]) -> Result<Vec<u8>> {
        match self.0.decrypt(nonce.into(), cipher) {
            Err(err) => Err(Error::new(ErrorKind::Unknown, err.to_string())),
            Ok(plain) => Ok(plain),
        }
    }
}

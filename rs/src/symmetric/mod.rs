mod chacha;

use crate::{rand::generate, size::*, Error, ErrorKind, Result};
use aead::{Aead, KeyInit, Payload};
use aes_gcm::{Aes128Gcm, Aes256Gcm};
use chacha::{ChaCha20Poly1305, XChaCha20Poly1305};

type Aes192Gcm = aes_gcm::AesGcm<aes_gcm::aes::Aes192, aead::consts::U12>;
pub enum Symmetric<'a> {
    Aes128Gcm {
        key: Box<[u8; SIZE_16]>,
        aad: &'a [u8],
        data: &'a [u8],
    },

    Aes192Gcm {
        key: Box<[u8; SIZE_24]>,
        aad: &'a [u8],
        data: &'a [u8],
    },

    Aes256Gcm {
        key: Box<[u8; SIZE_32]>,
        aad: &'a [u8],
        data: &'a [u8],
    },

    ChaCha20Poly1305 {
        key: Box<[u8; SIZE_32]>,
        aad: &'a [u8],
        data: &'a [u8],
    },
    XChaCha20Poly1305 {
        key: Box<[u8; SIZE_32]>,
        aad: &'a [u8],
        data: &'a [u8],
    },
}

impl Symmetric<'_> {
    // fn key_size(&self) -> usize {
    //     match self {
    //         Self::Aes128Gcm {
    //             key: _,
    //             aad: _,
    //             data: _,
    //         } => SIZE_U16,
    //         Self::Aes192Gcm {
    //             key: _,
    //             aad: _,
    //             data: _,
    //         } => SIZE_U24,
    //         Self::Aes256Gcm {
    //             key: _,
    //             aad: _,
    //             data: _,
    //         } => SIZE_U32,
    //         Self::ChaCha20Poly1305 {
    //             key: _,
    //             aad: _,
    //             data: _,
    //         } => SIZE_U32,
    //         Self::XChaCha20Poly1305 {
    //             key: _,
    //             aad: _,
    //             data: _,
    //         } => SIZE_U32,
    //     }
    // }
    fn nonce_size(&self) -> usize {
        match self {
            Self::Aes128Gcm {
                key: _,
                aad: _,
                data: _,
            } => SIZE_12,
            Self::Aes192Gcm {
                key: _,
                aad: _,
                data: _,
            } => SIZE_12,
            Self::Aes256Gcm {
                key: _,
                aad: _,
                data: _,
            } => SIZE_12,
            Self::ChaCha20Poly1305 {
                key: _,
                aad: _,
                data: _,
            } => SIZE_12,
            Self::XChaCha20Poly1305 {
                key: _,
                aad: _,
                data: _,
            } => SIZE_24,
        }
    }
}

impl Symmetric<'_> {
    pub fn encrypt(self) -> Result<Vec<u8>> {
        let nonce_size = self.nonce_size();
        match self {
            Self::Aes128Gcm { key, aad, data } => aead_encrypt(
                Aes128Gcm::new_from_slice(&*key)
                    .map_err(|err| Error::new(ErrorKind::Unknown, err.to_string()))?,
                nonce_size,
                aad,
                data,
            ),
            Self::Aes192Gcm { key, aad, data } => aead_encrypt(
                Aes192Gcm::new_from_slice(&*key)
                    .map_err(|err| Error::new(ErrorKind::Unknown, err.to_string()))?,
                nonce_size,
                aad,
                data,
            ),
            Self::Aes256Gcm { key, aad, data } => aead_encrypt(
                Aes256Gcm::new_from_slice(&*key)
                    .map_err(|err| Error::new(ErrorKind::Unknown, err.to_string()))?,
                nonce_size,
                aad,
                data,
            ),
            Self::ChaCha20Poly1305 { key, aad, data } => aead_encrypt(
                ChaCha20Poly1305::new_from_slice(&*key)
                    .map_err(|err| Error::new(ErrorKind::Unknown, err.to_string()))?,
                nonce_size,
                aad,
                data,
            ),
            Self::XChaCha20Poly1305 { key, aad, data } => aead_encrypt(
                XChaCha20Poly1305::new_from_slice(&*key)
                    .map_err(|err| Error::new(ErrorKind::Unknown, err.to_string()))?,
                nonce_size,
                aad,
                data,
            ),
        }
    }

    pub fn decrypt(self) -> Result<Vec<u8>> {
        let nonce_size = self.nonce_size();
        match self {
            Self::Aes128Gcm { key, aad, data } => aead_decrypt(
                Aes128Gcm::new_from_slice(&*key)
                    .map_err(|err| Error::new(ErrorKind::Unknown, err.to_string()))?,
                nonce_size,
                aad,
                data,
            ),
            Self::Aes192Gcm { key, aad, data } => aead_decrypt(
                Aes192Gcm::new_from_slice(&*key)
                    .map_err(|err| Error::new(ErrorKind::Unknown, err.to_string()))?,
                nonce_size,
                aad,
                data,
            ),
            Self::Aes256Gcm { key, aad, data } => aead_decrypt(
                Aes256Gcm::new_from_slice(&*key)
                    .map_err(|err| Error::new(ErrorKind::Unknown, err.to_string()))?,
                nonce_size,
                aad,
                data,
            ),
            Self::ChaCha20Poly1305 { key, aad, data } => aead_decrypt(
                ChaCha20Poly1305::new_from_slice(&*key)
                    .map_err(|err| Error::new(ErrorKind::Unknown, err.to_string()))?,
                nonce_size,
                aad,
                data,
            ),
            Self::XChaCha20Poly1305 { key, aad, data } => aead_decrypt(
                XChaCha20Poly1305::new_from_slice(&*key)
                    .map_err(|err| Error::new(ErrorKind::Unknown, err.to_string()))?,
                nonce_size,
                aad,
                data,
            ),
        }
    }
}

// fn aead_encrypt(aead: impl Aead, nonce: &[u8], aad: &[u8], plain: &[u8]) -> Result<Vec<u8>> {
// fn aead_encrypt(aead: impl Aead, nonce_size: u16, aad: &[u8], plain: &[u8]) -> Result<Vec<u8>> {
fn aead_encrypt(aead: impl Aead, nonce_size: usize, aad: &[u8], plain: &[u8]) -> Result<Vec<u8>> {
    let r = generate();
    let nonce = r.get(0..nonce_size).unwrap();

    let mut cipher = aead
        .encrypt(
            nonce.into(),
            Payload {
                msg: plain,
                aad: aad,
            },
        )
        .map_err(|err| Error::new(ErrorKind::Unknown, err.to_string()))?;

    cipher.append(&mut nonce.to_vec());

    Ok(cipher)
}

fn aead_decrypt(aead: impl Aead, nonce_size: usize, aad: &[u8], cipher: &[u8]) -> Result<Vec<u8>> {
    let len = cipher.len() - nonce_size;

    let nonce = match cipher.get(len..) {
        None => return Err(Error::new(ErrorKind::Unknown, "".to_string())),
        Some(nonce) => nonce,
    };

    let cipher = match cipher.get(..len) {
        None => return Err(Error::new(ErrorKind::Unknown, "".to_string())),
        Some(cipher) => cipher,
    };

    let plain = aead
        .decrypt(
            nonce.into(),
            Payload {
                msg: cipher,
                aad: aad,
            },
        )
        .map_err(|err| Error::new(ErrorKind::Unknown, err.to_string()))?;

    Ok(plain)
}

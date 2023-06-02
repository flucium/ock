mod chacha;

use crate::{hash::blake3, rand::generate, size::*, Error, ErrorKind, Result};
use aead::{Aead, KeyInit, Payload};
use aes_gcm::{Aes128Gcm, Aes256Gcm};
use chacha::ChaCha20Poly1305;

type Aes192Gcm = aes_gcm::AesGcm<aes_gcm::aes::Aes192, aead::consts::U12>;

pub enum Symmetric<'a> {
    Aes128Gcm {
        key: Box<[u8; SIZE_U16]>,
        aad: &'a [u8],
        data: &'a [u8],
    },

    Aes192Gcm {
        key: Box<[u8; SIZE_U24]>,
        aad: &'a [u8],
        data: &'a [u8],
    },

    Aes256Gcm {
        key: Box<[u8; SIZE_U32]>,
        aad: &'a [u8],
        data: &'a [u8],
    },

    ChaCha20Poly1305 {
        key: Box<[u8; SIZE_U32]>,
        aad: &'a [u8],
        data: &'a [u8],
    },
}

impl Symmetric<'_> {
    pub fn encrypt(self) -> Result<Vec<u8>> {
        match self {
            Self::Aes128Gcm { key, aad, data } => aead_encrypt(
                Aes128Gcm::new_from_slice(&*key)
                    .map_err(|err| Error::new(ErrorKind::Unknown, err.to_string()))?,
                &gen_nonce_12(),
                aad,
                data,
            ),
            Self::Aes192Gcm { key, aad, data } => aead_encrypt(
                Aes192Gcm::new_from_slice(&*key)
                    .map_err(|err| Error::new(ErrorKind::Unknown, err.to_string()))?,
                &gen_nonce_12(),
                aad,
                data,
            ),
            Self::Aes256Gcm { key, aad, data } => aead_encrypt(
                Aes256Gcm::new_from_slice(&*key)
                    .map_err(|err| Error::new(ErrorKind::Unknown, err.to_string()))?,
                &gen_nonce_12(),
                aad,
                data,
            ),
            Self::ChaCha20Poly1305 { key, aad, data } => aead_encrypt(
                ChaCha20Poly1305::new_from_slice(&*key)
                    .map_err(|err| Error::new(ErrorKind::Unknown, err.to_string()))?,
                &gen_nonce_12(),
                aad,
                data,
            ),
        }
    }

    pub fn decrypt(self) -> Result<Vec<u8>> {
        match self {
            Self::Aes128Gcm { key, aad, data } => aead_decrypt(
                Aes128Gcm::new_from_slice(&*key)
                    .map_err(|err| Error::new(ErrorKind::Unknown, err.to_string()))?,
                SIZE_U12,
                aad,
                data,
            ),
            Self::Aes192Gcm { key, aad, data } => aead_decrypt(
                Aes192Gcm::new_from_slice(&*key)
                    .map_err(|err| Error::new(ErrorKind::Unknown, err.to_string()))?,
                SIZE_U12,
                aad,
                data,
            ),
            Self::Aes256Gcm { key, aad, data } => aead_decrypt(
                Aes256Gcm::new_from_slice(&*key)
                    .map_err(|err| Error::new(ErrorKind::Unknown, err.to_string()))?,
                SIZE_U12,
                aad,
                data,
            ),
            Self::ChaCha20Poly1305 { key, aad, data } => aead_decrypt(
                ChaCha20Poly1305::new_from_slice(&*key)
                    .map_err(|err| Error::new(ErrorKind::Unknown, err.to_string()))?,
                SIZE_U12,
                aad,
                data,
            ),
        }
    }
}

fn aead_encrypt(aead: impl Aead, nonce: &[u8], aad: &[u8], plain: &[u8]) -> Result<Vec<u8>> {
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
        None => Err(Error::new(ErrorKind::Unknown, "".to_owned()))?,
        Some(n) => n,
    };

    let cipher = match cipher.get(..len) {
        None => Err(Error::new(ErrorKind::Unknown, "".to_owned()))?,
        Some(c) => c,
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

fn gen_nonce_12() -> [u8; SIZE_U12] {
    let r = generate();

    let h = blake3(&r);

    let nonce: [u8; SIZE_U12] = h.get(0..SIZE_U12).unwrap().try_into().unwrap();

    nonce
}

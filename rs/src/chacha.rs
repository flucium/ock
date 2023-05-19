use crate::{size::*, /*Error, ErrorKind,*/ Result};

pub fn chacha20_poly1305_encrypt(
    key: &[u8; SIZE_U32],
    nonce: &[u8; SIZE_U12],
    plain: &[u8],
) -> Result<Vec<u8>> {
    todo!()
}

pub fn chacha20_poly1305_decrypt(
    key: &[u8; SIZE_U32],
    nonce: &[u8; SIZE_U12],
    cipher: &[u8],
) -> Result<Vec<u8>> {
    todo!()
}
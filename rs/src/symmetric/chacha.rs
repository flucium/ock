// Reimplemented ChaChaPoly1305 to use the Zeroize ^1 version.
// https://github.com/RustCrypto/AEADs/tree/master/chacha20poly1305

use aead::{
    consts::{U0, U12, U16, U24, U32},
    generic_array::{ArrayLength, GenericArray},
    AeadCore, AeadInPlace, KeyInit, KeySizeUser,
};

use chacha20::{
    cipher::{KeyIvInit, StreamCipher, StreamCipherSeek},
    ChaCha20, XChaCha20,
};

use core::marker::PhantomData;

use poly1305::{universal_hash::UniversalHash, Key as Poly1305Key, Poly1305};

use zeroize::Zeroize;

pub(crate) type ChaCha20Poly1305 = ChaChaPoly1305<ChaCha20, U12>;

pub(crate) type XChaCha20Poly1305 = ChaChaPoly1305<XChaCha20, U24>;

pub(crate) struct ChaChaPoly1305<C, N: ArrayLength<u8> = U12> {
    key: GenericArray<u8, U32>,
    cipher: PhantomData<C>,
    nonce_size: PhantomData<N>,
}

impl<C, N> KeySizeUser for ChaChaPoly1305<C, N>
where
    N: ArrayLength<u8>,
{
    type KeySize = U32;
}

impl<C, N> KeyInit for ChaChaPoly1305<C, N>
where
    N: ArrayLength<u8>,
{
    fn new(key: &GenericArray<u8, U32>) -> Self {
        Self {
            key: *key,
            cipher: PhantomData,
            nonce_size: PhantomData,
        }
    }
}

impl<C, N> AeadCore for ChaChaPoly1305<C, N>
where
    N: ArrayLength<u8>,
{
    type NonceSize = N;

    type TagSize = U16;

    type CiphertextOverhead = U0;
}

impl<C, N> AeadInPlace for ChaChaPoly1305<C, N>
where
    C: KeyIvInit<KeySize = U32, IvSize = N> + StreamCipher + StreamCipherSeek,
    N: ArrayLength<u8>,
{
    fn encrypt_in_place_detached(
        &self,
        nonce: &aead::Nonce<Self>,
        associated_data: &[u8],
        buffer: &mut [u8],
    ) -> aead::Result<aead::Tag<Self>> {
        let (mut cipher, mut hasher) = new_cipher(C::new(&self.key, nonce));
        if buffer.len() / BLOCK_SIZE as usize >= MAX_BLOCKS {
            return Err(aead::Error);
        }

        hasher.update_padded(associated_data);

        // cipher apply_keystream
        cipher.apply_keystream(buffer);

        hasher.update_padded(buffer);

        mac_auth_len(&mut hasher, associated_data, buffer)?;

        Ok(*GenericArray::from_slice(&hasher.finalize()))
    }

    fn decrypt_in_place_detached(
        &self,
        nonce: &aead::Nonce<Self>,
        associated_data: &[u8],
        buffer: &mut [u8],
        tag: &aead::Tag<Self>,
    ) -> aead::Result<()> {
        let (mut cipher, mut hasher) = new_cipher(C::new(&self.key, nonce));

        hasher.update_padded(associated_data);
        hasher.update_padded(buffer);

        mac_auth_len(&mut hasher, associated_data, buffer)?;

        if hasher.verify(tag).is_err() {
            Err(aead::Error)?
        }

        // cipher apply_keystream
        cipher.apply_keystream(buffer);

        Ok(())
    }
}

// Clone
impl<C, N> Clone for ChaChaPoly1305<C, N>
where
    N: ArrayLength<u8>,
{
    fn clone(&self) -> Self {
        Self {
            key: self.key,
            cipher: self.cipher,
            nonce_size: self.nonce_size,
        }
    }
}

// new stream cipher

const BLOCK_SIZE: u64 = 64;
const MAX_BLOCKS: usize = core::u32::MAX as usize;

fn new_cipher<C>(mut cipher: C) -> (C, Poly1305)
where
    C: StreamCipher + StreamCipherSeek,
{
    let mut key = Poly1305Key::default();

    cipher.apply_keystream(&mut key);

    let hasher = Poly1305::new(&key);

    key.zeroize();

    cipher.seek(BLOCK_SIZE);

    (cipher, hasher)
}

fn mac_auth_len(hasher: &mut Poly1305, aad: &[u8], buf: &[u8]) -> Result<(), aead::Error> {
    let aad_len: u64 = aad.len().try_into().map_err(|_| aead::Error)?;
    let buf_len: u64 = buf.len().try_into().map_err(|_| aead::Error)?;

    let mut generic_array: GenericArray<u8, U16> = GenericArray::default();
    generic_array[..8].copy_from_slice(&aad_len.to_le_bytes());
    generic_array[8..].copy_from_slice(&buf_len.to_le_bytes());
    hasher.update_padded(&generic_array);
    Ok(())
}

// Drop(zeroize)
impl<C, N> Drop for ChaChaPoly1305<C, N>
where
    N: ArrayLength<u8>,
{
    fn drop(&mut self) {
        self.key.as_mut_slice().zeroize();
    }
}

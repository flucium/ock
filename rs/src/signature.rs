// use crate::{size::*, Error, ErrorKind, Result};
// use ed25519_dalek::{ExpandedSecretKey, PublicKey, Signature, Verifier};
// use rand_core::{CryptoRng, RngCore};

// pub fn generate_keypair<R>(csprng: &mut R) -> ([u8; 32], [u8; 32])
// where
//     R: RngCore + CryptoRng,
// {
//     let keypair: ed25519_dalek::Keypair = ed25519_dalek::Keypair::generate(csprng);

//     (*keypair.secret.as_bytes(), *keypair.public.as_bytes())
// }

// pub fn sign(
//     private_key: &[u8; SIZE_U32],
//     public_key: &[u8; SIZE_U32],
//     msg: &[u8],
// ) -> Result<[u8; SIZE_U64]> {
//     let private_key = match ExpandedSecretKey::from_bytes(private_key) {
//         Err(err) => return Err(Error::new(ErrorKind::Unknown, err.to_string())),
//         Ok(key) => key,
//     };

//     let public_key = match PublicKey::from_bytes(public_key) {
//         Err(err) => return Err(Error::new(ErrorKind::Unknown, err.to_string())),
//         Ok(key) => key,
//     };

//     // let mut keypair: [u8; SIZE_U64] = [0u8; SIZE_U64];
//     // keypair.copy_from_slice(private_key);
//     // keypair.copy_from_slice(public_key);

//     Ok(private_key.sign(msg, &public_key).to_bytes())
// }

// pub fn verify(public_key: &[u8; SIZE_U32], msg: &[u8], signed: &[u8; SIZE_U64]) -> Result<()> {
//     let public_key = match PublicKey::from_bytes(public_key) {
//         Err(err) => return Err(Error::new(ErrorKind::Unknown, err.to_string())),
//         Ok(key) => key,
//     };

//     let signed = match Signature::from_bytes(signed) {
//         Err(err) => return Err(Error::new(ErrorKind::Unknown, err.to_string())),
//         Ok(signed) => signed,
//     };

//     if let Err(err) = public_key.verify(msg, &signed) {
//         Err(Error::new(ErrorKind::Unknown, err.to_string()))?
//     }

//     Ok(())
// }

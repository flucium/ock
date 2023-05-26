use crate::rand::Rand;
use crate::size::*;
use x25519_dalek::{PublicKey, StaticSecret};

// use x25519_dalek::EphemeralSecret;
/// Warning. i may make adjustments to this in the future.
pub fn generate_keypair() -> ([u8; SIZE_U32], [u8; SIZE_U32]) {
        
    let private_key = StaticSecret::new(Rand);

    let public_key = PublicKey::from(&private_key);

    (private_key.to_bytes(), public_key.to_bytes())
}

/// Warning. i may make adjustments to this in the future.
/// Currently unadjusted. not secure clone operations.
pub fn exchange(private_key: &[u8; SIZE_U32], their_public: &[u8; SIZE_U32]) -> [u8; SIZE_U32] {
    let private_key = StaticSecret::from(private_key.clone());

    let shared_key = private_key.diffie_hellman(&PublicKey::from(their_public.clone()));

    shared_key.to_bytes()
}
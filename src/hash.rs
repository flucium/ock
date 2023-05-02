pub use digest::Digest;
use sha2::{Sha224, Sha256, Sha384, Sha512};

const BLAKE3_DEFAULT_CONTEXT: &str = "";

pub enum Mac {
    Blake3(blake3::Hasher),
}

impl Mac {
    pub fn new() -> Self {
        Self::new_blake3()
    }

    pub fn new_blake3() -> Self {
        Self::Blake3(blake3::Hasher::new())
    }

    pub fn update(&mut self, bytes: &[u8]) -> &mut blake3::Hasher {
        match self {
            Mac::Blake3(f) => f.update(bytes),
        }
    }

    pub fn finalize(&mut self) -> Vec<u8> {
        match self {
            Mac::Blake3(f) => f.finalize().as_bytes().to_vec(),
        }
    }
}

impl Default for Mac {
    fn default() -> Self {
        Self::Blake3(blake3::Hasher::new())
    }
}

pub enum Hash {
    Blake3(blake3::Hasher),
    Sha224(Sha224),
    Sha256(Sha256),
    Sha384(Sha384),
    Sha512(Sha512),
}

impl Hash {
    pub fn new() -> Self {
        Self::new_blake3()
    }

    pub fn new_blake3() -> Self {
        Self::Blake3(blake3::Hasher::new())
    }

    pub fn new_sha224() -> Self {
        Self::Sha224(Sha224::new())
    }

    pub fn new_sha256() -> Self {
        Self::Sha256(Sha256::new())
    }

    pub fn new_sha384() -> Self {
        Self::Sha384(Sha384::new())
    }

    pub fn new_sha512() -> Self {
        Self::Sha512(Sha512::new())
    }

    pub fn update(&mut self, bytes: &[u8]) {
        match self {
            Self::Sha224(f) => f.update(bytes),
            Self::Sha256(f) => f.update(bytes),
            Self::Sha384(f) => f.update(bytes),
            Self::Sha512(f) => f.update(bytes),
            Self::Blake3(f) => {
                f.update(bytes);
            }
        }
    }

    pub fn finalize(&mut self) -> Vec<u8> {
        match self {
            Self::Sha224(f) => {
                todo!()
            }
            Self::Sha256(f) => {
                todo!()
            }
            Self::Sha384(f) => {
                todo!()
            }
            Self::Sha512(f) => {
                todo!()
            }
            Self::Blake3(f) => f.finalize().as_bytes().to_vec(),
        }
    }
}

impl Default for Hash {
    fn default() -> Self {
        Self::Blake3(blake3::Hasher::new())
    }
}

//
// pub enum Kdf {
//     Blake3(blake3::Hasher),
// }

// impl Kdf {
//     pub fn new_blake3() -> Self {
//         Self::Blake3(blake3::Hasher::new_derive_key(BLAKE3_DEFAULT_CONTEXT))
//     }

//     pub fn new_blake3_with_context(context: &str) -> Self {
//         Self::Blake3(blake3::Hasher::new_derive_key(context))
//     }

//     pub fn update(&mut self, bytes: &[u8]) {
//         match self {
//             Self::Blake3(f) => {
//                 f.update(bytes);
//             }
//         }
//     }

//     pub fn finalize(&mut self) -> [u8; 32] {
//         match self {
//             Self::Blake3(f) => f.finalize().into(),
//         }
//     }

//     // pub fn finalize_xof(&mut self, buffer: &mut [u8]) {
//     //     match self {
//     //         Self::Blake3(f) => {
//     //             f.finalize_xof().fill(buffer);
//     //         }
//     //     }
//     // }
// }

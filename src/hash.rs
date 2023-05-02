pub use digest::Digest;

// use sha2::{Sha224, Sha256, Sha384, Sha512};
// use sha2::{Sha256, Sha512, Sha512_256};

// const BLAKE3_DEFAULT_CONTEXT: &str = "";




// pub enum Hasher{}

// pub enum Mac{}

// pub enum Kdf{}
// pub enum Hasher {
//     Blake3(blake3::Hasher),
//     Sha256(sha2::Sha256),
// }

// impl Hasher {
//     pub fn new() -> Self {
//         Self::new_blake3()
//     }
//     pub fn new_blake3() -> Self {
//         Self::Blake3(blake3::Hasher::new())
//     }
//     pub fn new_sha256() -> Self {
//         Self::Sha256(Sha256::new())
//     }
//     pub fn update(&mut self, bytes: &[u8]) -> &mut Self {
//         match self {
//             Self::Blake3(f) => {
//                 f.update(bytes);
//                 self
//             }
//             Self::Sha256(f) => {
//                 f.update(bytes);
//                 self
//             }
//         }
//     }
//     pub fn finalize(&mut self) -> Vec<u8> {
//         match self {
//             Self::Blake3(f) => f.finalize().as_bytes().to_vec(),
//             Self::Sha256(f) => f.finalize_reset().to_vec(),
//         }
//     }
// }

pub trait Hasher {
    fn new() -> Self;
    fn update(&mut self, btyes: &[u8]) -> &mut Self;
    fn finalize(&mut self) -> Vec<u8>;
}


pub struct Sha256(sha2::Sha256);

impl Sha256 {
    pub fn new() -> Self {
        Self(sha2::Sha256::new())
    }
}

impl Hasher for Sha256 {
    fn new() -> Self {
        Self(sha2::Sha256::new())
    }

    fn update(&mut self, bytes: &[u8]) -> &mut Self {
        self.0.update(bytes);
        self
    }

    fn finalize(&mut self) -> Vec<u8> {
        self.0.finalize_reset().to_vec()
    }
}

pub struct Sha512(sha2::Sha512);

impl Sha512 {
    pub fn new() -> Self {
        Self(sha2::Sha512::new())
    }
}

impl Hasher for Sha512 {
    fn new() -> Self {
        Self(sha2::Sha512::new())
    }

    fn update(&mut self, bytes: &[u8]) -> &mut Self {
        self.0.update(bytes);
        self
    }

    fn finalize(&mut self) -> Vec<u8> {
        self.0.finalize_reset().to_vec()
    }
}

pub struct Sha512_256(sha2::Sha512_256);

impl Sha512_256 {
    pub fn new() -> Self {
        Self(sha2::Sha512_256::new())
    }
}

impl Hasher for Sha512_256 {
    fn new() -> Self {
        Self(sha2::Sha512_256::new())
    }

    fn update(&mut self, bytes: &[u8]) -> &mut Self {
        self.0.update(bytes);
        self
    }

    fn finalize(&mut self) -> Vec<u8> {
        self.0.finalize_reset().to_vec()
    }
}

pub struct Blake3(blake3::Hasher);

impl Blake3 {
    pub fn new() -> Self {
        Self(blake3::Hasher::new())
    }

    pub fn verify(&mut self, sign: &[u8]) -> bool {
        self.0.finalize().as_bytes() == sign
    }
}

impl Hasher for Blake3 {
    fn new() -> Self {
        Self(blake3::Hasher::new())
    }

    fn update(&mut self, bytes: &[u8]) -> &mut Self {
        self.0.update(bytes);
        self
    }

    fn finalize(&mut self) -> Vec<u8> {
        self.0.finalize().as_bytes().to_vec()
    }
}



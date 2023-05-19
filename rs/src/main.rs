use aead::KeyInit;
use aead::Aead;
trait Symmetric<T, N> {
    fn new(key: &[u8]) -> Result<T, ()>;
    fn encrypt(&mut self, nonce: N, plain: &[u8]) -> Result<Vec<u8>, ()>;
    fn decrypt(&mut self, nonce: N, cipher: &[u8]) -> Result<Vec<u8>, ()>;
}

struct Aes128Gcm(aes_gcm::Aes128Gcm);

impl Symmetric<Aes128Gcm, &[u8; 12]> for Aes128Gcm {
    fn new(key: &[u8]) -> Result<Self, ()> {
        match aes_gcm::Aes128Gcm::new_from_slice(key) {
            Err(_) => Err(()),
            Ok(ok) => Ok(Self(ok)),
        }
    }

    fn encrypt(&mut self, nonce: &[u8; 12], plain: &[u8]) -> Result<Vec<u8>, ()> {
        match self.0.encrypt(nonce.into(), plain) {
            Err(_) => Err(()),
            Ok(cipher) => Ok(cipher),
        }
    }

    fn decrypt(&mut self, nonce: &[u8; 12], cipher: &[u8]) -> Result<Vec<u8>, ()> {
        match self.0.decrypt(nonce.into(), cipher) {
            Err(_) => Err(()),
            Ok(plain) => Ok(plain),
        }
    }
}
fn main() {
}




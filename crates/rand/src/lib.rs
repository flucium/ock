

pub mod secure{

    pub use rand::*;
    
    pub use rand_chacha::*;

    pub fn random()->[u8;32]{
        chacha20()
    }
    
    pub fn chacha20() ->[u8;32]{
        let mut rng = ChaCha20Rng::from_entropy();
    
        rng.gen()
    }
}

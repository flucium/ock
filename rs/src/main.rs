use aead::{Aead, AeadCore, KeyInit};

fn main() {
    let payload = aead::Payload { msg: b"", aad: b"" };
    let key = aes_gcm::Aes256Gcm::generate_key(&mut aead::OsRng);
    let aes = aes_gcm::Aes256Gcm::new(&key);
    let nonce = &aes_gcm::Aes256Gcm::generate_nonce(&mut aead::OsRng);
    for i in 0..10 {
        let c = aes
            .encrypt(
                &nonce,
                aead::Payload {
                    msg: format!("hello - {}", i).as_bytes(),
                    aad: i.to_string().as_bytes(),
                },
            )
            .unwrap();
        println!(
            "{}",
            String::from_utf8_lossy(
                &aes.decrypt(
                    nonce,
                    aead::Payload {
                        msg: &c,
                        aad: i.to_string().as_bytes(),
                    }
                )
                .unwrap()
            )
            .to_string()
        );
    }
}

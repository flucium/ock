// pub fn bytes_to_hex(bytes: &[u8]) -> String {
//     let x = bytes.iter().map(|b| format!("{:02X}", b));

//     x.collect::<String>()
// }

pub mod format {

    pub struct Bytes(Vec<u8>);

    impl Bytes {}

    impl From<&[u8]> for Bytes {
        fn from(bytes: &[u8]) -> Self {
            Self(bytes.to_vec())
        }
    }

    impl From<Vec<u8>> for Bytes {
        fn from(bytes: Vec<u8>) -> Self {
            Self(bytes)
        }
    }
}

fn demo() {}

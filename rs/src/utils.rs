// pub(crate) fn hex(bytes: &[u8]) -> String {
//     bytes.iter().map(|byte| format!("{:02X}", byte)).collect()
// }


// pub(crate) fn type_of<T>(_: T) -> String {
//     core::any::type_name::<T>().to_string()
// }

// To avoid making silly mistakes.
// byte * 8 to get bit.
// pub(crate) fn byte_to_bit(n: usize) -> usize {
//     n * 8
// }

// To avoid making silly mistakes.
// bit / 8 to get byte.
// pub(crate) fn bit_to_bytes(n: usize) -> usize {
//     n / 8
// }

// u16 to bytes
// pub(crate) fn u16_to_bytes(n: u16) -> [u8; 2] {
//     n.to_be_bytes()
// }

// bytes to u16
// pub(crate) fn bytes_to_u16(bytes: [u8; 2]) -> u16 {
//     u16::from_be_bytes(bytes)
// }

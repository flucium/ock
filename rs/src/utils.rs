pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{:02X}", byte)).collect()
}

pub fn type_of<T>(_: T) -> String {
    core::any::type_name::<T>().to_string()
}

/// To avoid making silly mistakes.
/// byte * 8 to get bit.
pub fn byte_to_bit(n: usize) -> usize {
    n * 8
}

/// To avoid making silly mistakes.
/// bit / 8 to get byte.
pub fn bit_to_bytes(n: usize) -> usize {
    n / 8
}

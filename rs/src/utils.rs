pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{:02X}", byte)).collect()
}

pub fn type_of<T>(_: T) -> String {
    core::any::type_name::<T>().to_string()
}

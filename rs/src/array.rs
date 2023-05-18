// #[derive(Debug)]
// pub struct Array<const N: usize>([u8; N]);

// impl<const N: usize> Array<N> {
//     pub fn get(&self) -> &[u8] {
//         &self.0
//     }
// }

// impl<const N: usize> From<[u8; N]> for Array<N> {
//     fn from(value: [u8; N]) -> Self {
//         Array(value)
//     }
// }
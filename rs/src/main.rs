fn main() {
    // let bs = Bytes { data: [9u8; 100] };
    // print!("{:?}",bs);

    Bytes{data:[0u8;100]};
    Bytes{data:[0u8;200]};
}

#[derive(Debug)]
pub struct Bytes<const N: usize> {
    data: [u8; N],
}

type BS = Bytes<100>;

pub trait X {
    fn demo<const N:usize>(&self)->Bytes<N>;
}


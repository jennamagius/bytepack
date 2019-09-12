pub trait BytePack {
    const WIDTH: usize;

    fn to_slice(&self, dest: &mut [u8]);
    fn from_slice(from: &[u8]) -> Self;
}

pub use bytepack_derive::BytePack;

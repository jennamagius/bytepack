pub trait BytePack {
    const WIDTH: usize;

    fn to_slice(&self, dest: &mut [u8]);
    fn from_slice(from: &[u8]) -> Self;
}

pub use bytepack_derive::BytePack;

#[cfg(test)]
mod test {
    use crate as bytepack;
    use crate::BytePack;

    #[derive(bytepack_derive::BytePack, PartialEq, Debug)]
    struct Split {
        a: [u8; 5],
        b: [u8; 3],
        c: [u8; 2],
    }

    #[test]
    fn nominal() {
        let split = Split {
            a: [0; 5],
            b: [1; 3],
            c: [2; 2],
        };
        let mut dest = [3u8; Split::WIDTH];
        split.to_slice(&mut dest);
        assert_eq!(&dest[..], &[0, 0, 0, 0, 0, 1, 1, 1, 2, 2][..]);
        let src = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(
            Split {
                a: [0, 1, 2, 3, 4],
                b: [5, 6, 7],
                c: [8, 9],
            },
            <Split as crate::BytePack>::from_slice(&src[..])
        );
    }
}
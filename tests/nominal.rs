const MYWIDTH: usize = 5;

#[derive(bytepack_derive::BytePack, PartialEq, Debug)]
struct Split {
    a: [u8; MYWIDTH],
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
    let mut dest = [3u8; <Split as bytepack::BytePack>::WIDTH];
    bytepack::BytePack::to_slice(&split, &mut dest);
    assert_eq!(&dest[..], &[0, 0, 0, 0, 0, 1, 1, 1, 2, 2][..]);
    let src = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(
        Split {
            a: [0, 1, 2, 3, 4],
            b: [5, 6, 7],
            c: [8, 9],
        },
        <Split as bytepack::BytePack>::from_slice(&src[..])
    );
}

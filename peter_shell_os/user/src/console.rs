use crate::read;

const STDIN: usize = 0;

pub fn getchar() -> u8 {
    let mut c = [0u8; 1];
    read(STDIN, &mut c);
    c[0]
}
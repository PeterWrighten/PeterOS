

pub struct Stdin;

pub struct  Stdout;

impl File for Stdin {
    fn read(&self, mut user_bud: UserBuffer) -> usize {
        assert_eq!(user_buf.len(), 1);

    }
}
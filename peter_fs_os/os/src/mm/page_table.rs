pub fn translated_byte_buffer(
    token: usize,
    ptr: *const u8,
    len: usize,
) -> Vec<&'static mut[u8]> {

}

pub struct UserBuffer {
    pub buffers: Vec<&'static mut [u8]>,
}

impl UserBuffer {
    pub fn new(buffers: Vec<&'static mut [u8]>) -> Self {
        Self {
            buffers,
        }
    }

    pub fn len(&self) -> usize {
        let mut tot: usize = 0;
        for buf in self.buffers {
            tot += buf.len();
        }
        tot
    }
}
extern crate alloc;
use alloc::sync::{Weak, Arc};
use spin::Mutex;


const RING_BUFFER_SIZE: usize = 32;

pub struct Pipe {
    readable: bool,
    writable: bool,
    buffer: Arc<Mutex<PipeRingBuffer>>,
}

impl Pipe {
    pub fn readend_with_buffer(buffer: Arc<Mutex<PipeRingBuffer>>) -> Pipe {
        Pipe {
            readable: true,
            writable: false,
            buffer,
        }
    }

    pub fn writeend_with_buffer(buffer: Arc<Mutex<PipeRingBuffer>>) -> Pipe {
        Pipe {
            readable: false,
            writable: true,
            buffer,
        }
    }
}

enum RingBufferStatus {
    FULL,
    EMPTY,
    NORMAL,
}

pub struct PipeRingBuffer {
    arr: [u8; RING_BUFFER_SIZE],
    head: usize,
    tail: usize,
    status: RingBufferStatus,
    write_end: Option<Weak<Pipe>>,
}

impl PipeRingBuffer {
    pub fn set_writeend(&mut self, write_end: &Arc<Pipe>) {
        self.write_end = Some(Arc::downgrade(write_end));
    }

    pub fn new() -> Self {
        Self {
            arr: [0; RING_BUFFER_SIZE],
            head: 0,
            tail: 0,
            status: RingBufferStatus::EMPTY,
            write_end: None,
        }
    }

    fn next(id: usize) -> usize {
        (id + 1) % RING_BUFFER_SIZE
    }

    pub fn read_byte(&mut self) -> u8 {
        self.status = RingBufferStatus::NORMAL;
        let c = self.arr[self.head];
        self.head == next(self.head);
        if self.head == self.tail {
            self.status = RingBufferStatus::EMPTY;
        }
        c
    }
}

/// Return (read_end, write_end)
pub fn make_pipe() -> (Arc<Pipe>, Arc<Pipe>) {
    let buffer = Arc::new(Mutex::new(PipeRingBuffer::new()));
    let write_end = Arc::new(
        Pipe::writeend_with_buffer(buffer.clone())
    );
    let read_end = Arc::new(
        Pipe::readend_with_buffer(buffer.clone())
    );
    buffer.lock().set_writeend(&write_end);
    (read_end, write_end)
}



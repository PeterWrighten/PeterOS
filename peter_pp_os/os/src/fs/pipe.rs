extern crate alloc;
use alloc::sync::{Weak, Arc};
use spin::Mutex;


const RING_BUFFER_SIZE: usize = 32;

pub struct Pipe {
    readable: bool,
    writable: bool,
    buffer: Arc<Mutex<PipeRingBuffer>>,
}

impl File for Pipe {
    fn read(&self, buf: UserBuffer) -> usize {
        assert_eq!(self.readable, true);
        let mut buf_iter = buf.into_iter();
        let mut read_size = 0usize;
        loop {
            let mut ring_buffer = self.buffer.exclusive_access();
            let loop_read = ring_buffer.available_read();
            if loop_read == 0 {
                if ring_buffer.all_write_ends_closed() {
                    return read_size;
                }
                // still have unarrival data
                drop(ring_buffer);
                suspend_current_and_run_next();
                continue;
            }
            for _ in 0..loop_read {
                if let Some(byte_ref) = buf_iter.next() {
                    unsafe { *byte_ref = ring_buffer.read_byte(); }
                    read_size += 1;
                } else {
                    return read_size;
                }
            }

        }
    }
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

    pub fn available_read(&self) -> usize {
        if self.status == RingBufferStatus::EMPTY {
            0
        }  else {
            if self.tail > self.head {
                self.tail - self.head
            } else {
                self.tail + RING_BUFFER_SIZE - self.head
            }
        }
    }

    pub fn all_write_ends_closed(&self) -> bool {
        // if counter == 0: all write ends closed
        self.write_end.as_ref().unwrap().upgrade().is_none()
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



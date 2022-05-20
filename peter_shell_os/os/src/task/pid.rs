/// Based on RAII design pattern and stack-like allocation

pub struct PidHandle(pub size);

struct PidAllocator {
    current: usize,
    recycled: Vec<usize>,
}

impl PidAllocator {
    pub fn new() ->  Self {
        PidAllocator {
            current: 0,
            recycled: Vec::new(),
        }
    }

    pub fn alloc(&mut self) -> PidHandle {
        if let Some(pid) = self.recycled.pop() {
            PidHandle(pid)
        } else {
            self.current += 1;
            PidHandle(self.current - 1)
        }
    }

    pub fn dealloc(&mut self, pid: usize) {
        assert!(pid < self.current);

    }
}



pub trait File: Send + Sync {
    fn writable(&self) -> bool;
    fn readable(&self) -> bool;
    fn read(&self, buf: UserBuffer) -> usize;
    fn write(&self, buf: UserBuffer) -> usize;
}


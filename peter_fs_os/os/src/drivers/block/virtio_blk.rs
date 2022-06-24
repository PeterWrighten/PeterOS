// VirtIOHeader: MMIO supported registers
use virtio_drivers::{VirtIOBlk, VirtIOHeader};

// VirtIO MMIO's left side
const VIRTIO0: usize = 0x10001000;

/// Ensure concurrency, and use interior mutablity to implement BlockDevice trait
pub struct VirtIOBlock(Mutex<VirtIOBlk<'static>>);

impl VirtIOBlock {
    pub fn new() -> Self {
        Self(Mutex::new(VirtIOBlk::new(
            unsafe { &mut *(VIRTIO0 as *mut VirtIOHeader) }
        ).unwrap()))
    }
}

impl BlockDevice for VirtIOBlock {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        self.0.lock().read_block(block_id, buf).expect("Error when reading VirtIOBlk");
    }

    fn write_block(&self, block_id: usize, buf: &[u8]) {
        self.0.lock().write_block(block_id, buf).expect("Error when writing VirtIOBlk");
    }
}

lazy_static! {
    static ref QUEUE_FRAMES: Mutex<Vec<FrameTracker>> = Mutex::new(Vec::new());
}

#[no_mangle]
pub extern "C" fn virtio_dma_alloc(pages: usize) -> PhysAddr {
    let mut ppn_base = PhysPageNum(0);
    for i in 0..pages {
        let frame = frame_alloc().unwrap();
        if i == 0 { ppn_base = frame.ppn; }
        assert_eq!(ppn_base.0 + i, frame.ppn.0);
        QUEUE_FRAMES.lock().push(frame);
    }
    ppn_base.into()
}

#[no_mangle]
pub extern "C" fn virtio_phys_to_virt(physaddr: PhysAddr) -> VirtAddr {
    VirtAddr(physaddr.0)
}


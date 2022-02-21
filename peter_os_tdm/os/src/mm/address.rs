
const PA_WIDTH_SV39:usize = 56;

const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_SIZE_BITS;


#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysAddr(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtAddr(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PysPageNum(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtPageNum(pub usize);

impl From<usize> for PhysAddr {
    fn from(v: usize) -> Self {
        Self(v & ((1 << PA_WIDTH_SV39) - 1)) /* Prevent Overflow */
    }
}

impl PhysAddr {
    pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }

    pub fn floor(&self) -> PhysPageNum {
        PhysPageNum(self.0 / PAGE_SIZE)
    }

    pub fn ceil(&self) -> PysPageNum {
        PhysPageNum((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE)
    }

    pub fn aligned(&self) -> bool {
        self.page_offset() == 0
    }
}

impl From<PhysAddr> for PysPageNum {
    fn from(v: PhysAddr) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}

impl From<usize> for VirtAddr {
    fn from(v: usize) -> Self {
        Self(v & ((1 << VA_WIDTH_SV39) - 1 ))
    }
}

impl From<PhysAddr> for usize {
    fn from(v: PhysAddr) -> Self {
        v.0
    }
}

impl From<VirtAddr> for usize {
    fn from(v: VirtAddr) -> Self {
        Self(v.0)
    }
}




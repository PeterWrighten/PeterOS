use crate::config::MIMO;


pub struct MemorySet {

}

impl MemorySet {
    pub fn new_kernel() -> Self {
        // ..
        println!("mapping memory-mapped registers");
        for pair in MMIO {
            memory_set.push(MapArea::new(
                (*pair).0.into(),
                ((*pair).0 + (*pair).1).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ), None);
        }
        memory_set
    }
}
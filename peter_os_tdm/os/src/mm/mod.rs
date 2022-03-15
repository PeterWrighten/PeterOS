mod address;
mod frame_allocator;
mod memory_set;
mod page_table;
mod heap_allocator;

#[cfg(test)]
mod tests {
    use super::frame_allocator::{frame_alloc, FrameTracker};
    fn frame_allocator_test() {
        let mut v: Vec<FrameTracker> = Vec::new();
        for i in 0..5 {
            let frame = frame_alloc().unwrap();
            println!("{:?}", frame);
            v.push(frame);
        }
        v.clear();
        for i in 0..5 {
            let frame = frame_alloc().unwrap();
            println!("{:?}", frame);
            v.push(frame);
        }
        drop(v);
        println!("frame_allocator_test passed!");
    }
}
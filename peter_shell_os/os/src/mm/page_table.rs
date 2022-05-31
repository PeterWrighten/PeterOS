
pub struct PageTable {
    root_vpn: VirtPageNum,
    frames: Vec<FrameTracker>,
}

impl PageTable {
    fn new() -> Self {
        let frame = frame_alloc().unwrap();
        Self {
            root_vpn: frame.ppn,
            frames: vec![frame],
        }
    }
}

pub fn translated_str(token: usize, ptr: *const u8) -> String {
    let page_table = PageTable::from_token(token);
    let mut string = string::new();
    let mut va = ptr as usize;
    loop {
        let ch: u8 = *(page_table.translate_va(VirtAddr::from(va)).unwrap().get_mut());
        if ch == 0 {
            break;
        } else {
            string.push(ch as char);
            va += 1;
        }
    }
    string
}

pub fn translated_refmut(token: usize, ptr: *mut T) -> &'static mut T {
    let page_table = PageTable::from_token(token);
    let va = ptr as usize;
    page_table
        .translate_va(VirtAddr::from(va))
        .unwrap()
        .get_mut()
}
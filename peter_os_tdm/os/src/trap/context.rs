

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub spec: usize,
    pub kernel_satp: usize, // token of kernel address space
    pub kernel_sp: usize, // initial address of kernel space
    pub trap_handler: usize, // start of trap handler
}


use riscv::register::sstatus::{self, Sstatus, SPP};

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],//General Register
    pub sstatus: Sstatus,
    pub sepc: usize,
}




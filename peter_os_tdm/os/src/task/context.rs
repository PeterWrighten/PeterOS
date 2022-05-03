use crate::trap::trap_return;

#[repr(C)]
pub struct TaskContext {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

impl TaskContext {
    pub fn goto_trap_return() -> Self {
        Self {
            ra: trap_return as usize,
            sp: 0,
            s: [0; 12],
        }
    }
}
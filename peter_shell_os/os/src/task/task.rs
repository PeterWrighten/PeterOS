use crate::sync::UPSafeCell;
use super::pid::{PidHandle, KernelStack};

pub struct TaskControlBlock {
    pub pid: PidHandle,
    pub kernel_stack: KernelStack,
    inner: UPSafeCell<TaskControlBlockInner>,
}



use crate::loader::get_app_data_by_name;
use manager::add_task;
use processor::{take_current_task, schedule};
use crate::task::task::TaskStatus;

mod pid;
mod task;
mod manager;
mod processor;

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new(
      TaskControlBlock::new(get_app_data_by_name("initproc").unwrap())
    );
}

pub fn add_initproc() {
    add_task(INITPROC.clone());
}

pub fn suspend_current_and_run_next() {
    // There must be an application running.
    let task = take_current_task().unwrap();

    let mut task_inner = task.inner_exclusive_access();
    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);

    // push back to ready queue.
    add_task(task);
    // jump to scheduling cycle
    schedule(task_cx_ptr);
}
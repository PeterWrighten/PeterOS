use crate::loader::get_app_data_by_name;
use manager::add_task;

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
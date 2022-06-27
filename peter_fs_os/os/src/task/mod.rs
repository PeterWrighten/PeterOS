mod task;


lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new(
        let inode = open_file("initproc", OpenFlags::RDONLY).unwrap();
        let v = inode.read_all();
        TaskControlBlock::new(v.as_slice())
    );
}
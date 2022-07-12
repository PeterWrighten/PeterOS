fn  main() {
    let mut runtime = Runtime::new();
    runtime.init();
    runtime.spawn(|| {
        println!("Task1 Starting");
        let id = 1;
        for i in 0..10 {
            println!("task: {} counter: {}", id, i);
            yield_task();
        }
        println!("Task1 Finished");
    });

    runtime.spawn(|| {
        println!("Task2 Starting");
        let id = 2;
        for i in 0..15 {
            println!("task: {} counter: {}", id, i);
            yield_task();
        }
        println!("Task2 Finished");
    });
    runtime.run();
}
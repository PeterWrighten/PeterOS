use std::thread;

fn mythread(args: f64) -> f64 {
    println!("{}", args);
    return args + 1.0;
}

fn main() {
    let handle = thread::spawn(|| mythread(1.0));
    let rval = handle.join().unwrap();
    println!("{}", rval);
}

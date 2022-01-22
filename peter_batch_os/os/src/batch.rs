
use std::cell::RefCell;
// static A: RefCell<i32> = RefCell::new(3);



struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}
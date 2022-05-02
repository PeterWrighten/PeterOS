use riscv::register::{
    scause::{self, Trap, Exception, Interrupt},
    sie
};

match scause.cause() {
    Trap::Interrupt(Interrupt::SupervisorTimer) => {
        set_next_trigger();
        suspend_current_and_run_next();
    }
    Trap::Exception(Exception::Store)
}

pub fn enable_timer_interrupt() {
    unsafe{ sie::set_stimer(); }
}
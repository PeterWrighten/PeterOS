
#[no_mangle]
pub(crate) fn trap_handler() -> ! {
    // ...
    match scause.cause() {
        Trap::Exception(Exception::LoadPageFault) => {
            current_add_signal(SignalFlags::SIGSEGV);
        }

        Trap::Exception(Exception::IllegalInstruction) => {
            current_add_signal(SignalFlags::SIGILL);
        }
        // ..
    }

    handle_signals();

    if let Some((errno, msg)) = check_signals_error_of_current() {
        println!("[kernel error] {}", msg);
        exit_current_and_run_next(errno);
    }
    trap_return();
}


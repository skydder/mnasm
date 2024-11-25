pub fn emit_msg_and_exit(msg: String) -> ! {
    eprint!("{}", msg);
    std::process::exit(0)
}

#[macro_export]
macro_rules! emit_msg_and_exit {
    ($($msg: expr), *) => {
        $crate::emit_msg_and_exit(format!($($msg), *))
    };
}
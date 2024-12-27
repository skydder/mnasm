use crate::Location;

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

pub fn emit_error(location: Location, msg: String) -> ! {
    let text = format!("[ERROR]: {}\n-> {:?}\n", msg, location);
    emit_msg_and_exit(text);
}

#[macro_export]
macro_rules! emit_error {
    ($loc: expr,$($msg: expr), *) => {
        $crate::emit_error($loc, format!($($msg), *))
    };
}
pub fn emit_warning(location: Location, msg: String) {
    // eprint!("[WARNING]: {}\n-> {:?}\n", msg, location);
}

#[macro_export]
macro_rules! emit_warning {
    ($loc: expr,$($msg: expr), *) => {
        $crate::emit_warning($loc, format!($($msg), *))
    };
}

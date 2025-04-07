use crate::Location;
use std::sync::atomic::{AtomicBool, Ordering};

static IW: AtomicBool = AtomicBool::new(false);

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
    if IW.load(Ordering::Relaxed) {
        return;
    }
    eprint!("[WARNING]: {}\n-> {:?}\n", msg, location);
}

pub fn set_iw() {
    IW.fetch_or(true, Ordering::SeqCst);
}

#[macro_export]
macro_rules! emit_warning {
    ($loc: expr,$($msg: expr), *) => {
        $crate::emit_warning($loc, format!($($msg), *))
    };
}

#[derive(Debug)]
pub enum AsmError<'a> {
    ParseError(Location<'a>, String, String),
}

impl std::fmt::Display for AsmError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsmError::ParseError(location, msg, sub_msg) => {
                write!(
                    f,
                    "[Error in Parsing] {}\n-> {}\n: (tips) {}\n",
                    msg, location, sub_msg
                )
            }
        }
    }
}

impl std::error::Error for AsmError<'_> {}

pub type AsmResult<'a, T> = Result<T, AsmError<'a>>;

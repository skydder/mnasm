use std::fs::File;

mod error;
mod location;

pub use error::{emit_error, emit_msg_and_exit, emit_warning, set_iw, AsmError, AsmResult};
pub use location::{Location, Source};
pub fn open_safely(file: &str) -> File {
    File::open(file).unwrap_or_else(|_| {
        emit_msg_and_exit!("failed to open '{}'\n", file);
    })
}

use std::fs::File;

mod location;
mod error;

pub use location::{Location, Source};
pub use error::emit_msg_and_exit;
pub fn open_safely(file: &str) -> File {
    File::open(file).unwrap_or_else(|_| {
        emit_msg_and_exit!("failed to open '{}'", file);
    })
}
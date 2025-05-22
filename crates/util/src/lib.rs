use std::fs::File;

mod error;
mod location;
mod token;
mod tokenizer_requirement;

pub use error::{emit_error, emit_msg_and_exit, emit_warning, set_iw, AsmError, AsmResult, convert_to_asmerror};
pub use location::{Location, Source, Stream};
pub fn open_safely(file: &str) -> File {
    File::open(file).unwrap_or_else(|_| {
        emit_msg_and_exit!("failed to open '{}'\n", file);
    })
}

pub use token::{pair_end, Token, TokenKind};
pub use tokenizer_requirement::Tokenizer;

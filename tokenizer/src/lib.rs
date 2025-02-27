mod macro_related;
mod token;
mod tokenizer2;

use std::fmt::Debug;

pub(crate) use macro_related::{init_infix_macro, read_macro_call, read_macro_def, Macro};
pub use token::{Token, TokenKind};
pub use tokenizer2::{Tokenizer2, MacroStatus};
use util::Location;

#[derive(Debug, Clone, Copy)]
pub struct Stream<'a> {
    begin: Location<'a>,
    end: Location<'a>,
}

impl<'a> Stream<'a> {
    pub fn new(begin: Location<'a>, end: Location<'a>) -> Self {
        Self {
            begin: begin,
            end: end,
        }
    }

    pub fn begin(&self) -> Location<'a> {
        self.begin
    }

    pub fn end(&self) -> Location<'a> {
        self.end
    }
}

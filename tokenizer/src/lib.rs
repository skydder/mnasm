mod macro_related;
mod token;
mod tokenizer2;

use std::fmt::Debug;

pub(crate) use macro_related::{read_macro_call, read_macro_def, Macro};
pub use token::{Token, TokenKind};
pub use tokenizer2::Tokenizer2;
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

// todo: apply to Tokenizer and Macro
pub trait TokenGenerator<'a>: Debug {
    fn location(&self) -> Location<'a>;
    fn peek_token(&self) -> Token<'a>;
    fn next_token(&self) -> Token<'a>;
    fn skip_space(&self);
    fn consume_token(&self, consumeing_token: TokenKind);
    fn consume_newline(&self);
    fn consume_indent(&self);
    fn kind(&self) -> GenKind;
}

pub enum GenKind {
    Tokenizer,
    MacroTokenizer,
}

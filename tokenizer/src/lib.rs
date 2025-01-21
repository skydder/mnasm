mod token;
mod tokenizer;

use std::fmt::Debug;

pub use token::{Token, TokenKind};
pub use tokenizer::Tokenizer;
use util::Location;

// todo: apply to Tokenizer and Macro
pub trait TokenGenerator<'a>: Debug{
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
    MacroTokenizer
}

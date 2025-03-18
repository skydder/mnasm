use std::fmt::Debug;

use crate::token::{Token, TokenKind};

pub trait Tokenizer<'a>
where
    Self: Clone + Debug,
{
    fn location(&self) -> crate::Location<'a>;
    fn peek_token(&self, macro_expand: bool) -> Token<'a>;
    fn next_token(&self) -> Token<'a>;
    fn skip_space(&self, macro_expand: bool);
    fn skip_token(&self);
    fn consume_token(&self, consumeing_token: TokenKind<'a>);
    fn consume_newline(&self);
    fn consume_indent(&self);
    fn add_to_code(&self, tokenkind: TokenKind<'a>);
    fn code(&self) -> String;
}

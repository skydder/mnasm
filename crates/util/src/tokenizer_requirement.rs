use std::fmt::Debug;

use crate::{
    token::{Token, TokenKind},
    AsmResult,
};

pub trait Tokenizer<'code>
where
    Self: Clone + Debug,
{
    fn location(&self) -> crate::Location<'code>;
    fn peek_token(&self) -> Token<'code>;
    fn next_token(&self) -> Token<'code>;
    fn skip_space(&self) {
        while self.peek_token().is(&TokenKind::Space) {
            self.next_token();
        }
    }
    fn consume_token(&self, consumeing_token: TokenKind) -> AsmResult<'code, ()>;
}

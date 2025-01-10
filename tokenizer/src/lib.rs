mod token;
mod tokenizer;

pub use token::{Token, TokenKind};
pub use tokenizer::Tokenizer;
use util::Location;

// todo: apply to Tokenizer and Macro
pub trait TokenGenerator {
    fn location(&self) -> Location;
    fn peek_token(&self) -> Token;
    fn next_token(&self) -> Token;
    fn skip_space(&self);
    fn consume_token(&self, consumeing_token: TokenKind);
    fn consume_newline(&self);
    fn consume_indent(&self);
}

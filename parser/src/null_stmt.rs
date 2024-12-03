use tokenizer::{TokenKind, Tokenizer}; 
use data::NullStmt;
use util::emit_error;

pub fn parse_null_stmt<'a>(tokenizer: &'a Tokenizer<'a>) -> NullStmt<'a> {
    let loc = tokenizer.location();
    if !tokenizer.peek_token().is(TokenKind::Space) && !tokenizer.peek_token().is(TokenKind::NewLine) {
        emit_error!(loc, "expected Space!!")
    }
    tokenizer.skip_space();
    if tokenizer.peek_token().is(TokenKind::NewLine) {
        NullStmt::new(loc)
    } else {
        emit_error!(loc, "expected Space!!")
    }
}
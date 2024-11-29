use data::Stmt;
use tokenizer::{TokenKind, Tokenizer};
use util::emit_error;

use crate::parse_compound_ins;

pub fn parse_stmt<'a>(tokenizer: &'a Tokenizer<'a>) -> Option<Stmt<'a>> {
    let currrent_token = tokenizer.peek_token();
    if !currrent_token.is_identifier() {
        return None;
    } else if currrent_token.is(TokenKind::Space) {
        emit_error!(currrent_token.location, "Indent error")
    }

    Some(Stmt { line: parse_compound_ins(tokenizer).unwrap() })
}

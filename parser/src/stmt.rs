use data::Stmt;
use tokenizer::{TokenKind, Tokenizer};
use util::emit_error;

use crate::{parse_compound_ins, parse_null_stmt};

pub fn parse_stmt<'a>(tokenizer: &'a Tokenizer<'a>) -> Option<Stmt<'a>> {
    let currrent_token = tokenizer.peek_token();
    if !currrent_token.is_identifier() {
        return None;
    } else if currrent_token.is(TokenKind::Space) {
        return parse_null_stmt(tokenizer);
    }

    Some(Stmt { line: parse_compound_ins(tokenizer).unwrap() })
}

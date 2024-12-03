use data::Stmt;
use tokenizer::{TokenKind, Tokenizer};
use util::emit_error;

use crate::{parse_compound_ins, parse_null_stmt};

pub fn parse_stmt<'a>(tokenizer: &'a Tokenizer<'a>) -> Box<dyn Stmt + 'a> {
    let currrent_token = tokenizer.peek_token();
    let loc = tokenizer.location();
    match currrent_token.kind {
        TokenKind::Space | TokenKind::NewLine | TokenKind::EOF => {
            Box::new(parse_null_stmt(tokenizer))
        },
        TokenKind::Identifier(_) => {
            Box::new(parse_compound_ins(tokenizer).unwrap())
        },
        _ => {
            emit_error!(loc, "expected stmt, but found other!")
        }
    }
}

use data::Stmt;
use tokenizer::{TokenKind, Tokenizer};
use util::emit_error;

use crate::{parse_compound_ins, parse_null_stmt};

pub fn parse_stmt<'a>(tokenizer: &'a Tokenizer<'a>) -> Box<dyn Stmt + 'a> {
    let currrent_token = tokenizer.peek_token();
    let loc = tokenizer.location();
    if !currrent_token.is_identifier() {
        if currrent_token.is(TokenKind::Space) || currrent_token.is(TokenKind::NewLine) {
            return Box::new(parse_null_stmt(tokenizer));
            // return parse_null_stmt(tokenizer);
        }
        emit_error!(loc, "ehat")
        // todo!()
    }

    Box::new(parse_compound_ins(tokenizer).unwrap())
}

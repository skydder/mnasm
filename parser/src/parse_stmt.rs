use data::Stmt;
use tokenizer::{TokenKind, Tokenizer};
use util::emit_error;

use crate::{parse_block, parse_compound_ins, parse_label_def, parse_null_stmt};

pub fn parse_stmt<'a>(tokenizer: &'a Tokenizer<'a>, indent_depth: usize) -> Box<dyn Stmt + 'a> {
    let currrent_token = tokenizer.peek_token();
    match currrent_token.kind {
        TokenKind::Space | TokenKind::NewLine => Box::new(parse_null_stmt(tokenizer)),
        TokenKind::Identifier(_) => Box::new(parse_compound_ins(tokenizer)),
        TokenKind::OpenBrace => Box::new(parse_block(tokenizer, indent_depth)),
        TokenKind::LessThan => Box::new(parse_label_def(tokenizer, indent_depth)),
        _ => {
            emit_error!(currrent_token.location, "expected stmt, but found other!")
        }
    }
}

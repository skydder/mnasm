use tokenizer::{TokenKind, Tokenizer};
use util::emit_error;

use crate::{parse_stmt, read_indent_by_depth, Stmt};

#[derive(Debug)]
pub struct Block<'a> {
    indent_depth: usize,
    stmts: Vec<Stmt<'a>>,
}

pub fn parse_block<'a>(tokenizer: &'a Tokenizer<'a>, indent_depth: usize) -> Option<Block<'a>> {
    if !tokenizer.peek_symbol().is(TokenKind::OpenBrace) {
        return None;
    }
    tokenizer.next_token();
    tokenizer.expect_symbol(TokenKind::NewLine);
    // code
    let mut stmts = Vec::new();
    while !tokenizer.peek_symbol().is(TokenKind::CloseBrace) {
        read_indent_by_depth(tokenizer, indent_depth);
        stmts.push(parse_stmt(tokenizer).unwrap_or_else(|| {
            emit_error!(tokenizer.location(), "only stmts can be in block");
        }));
    }
    read_indent_by_depth(tokenizer, indent_depth);
    tokenizer.expect_symbol(TokenKind::CloseBrace);
    Some(Block {
        indent_depth: indent_depth,
        stmts: stmts,
    })
}

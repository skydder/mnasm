use data::{Block, Stmt};
use tokenizer::{TokenKind, Tokenizer};
use util::emit_error;

use crate::{parse_stmt, read_indent_by_depth};

pub fn parse_block<'a>(tokenizer: &'a Tokenizer<'a>, indent_depth: usize) -> Option<Block> {
    if !tokenizer.peek_symbol().is(TokenKind::OpenBrace) {
        return None;
    }
    tokenizer.next_token();
    // code
    let mut stmts = Vec::new();
    parse_inside(tokenizer, indent_depth, &mut stmts);
    // read_indent_by_depth(tokenizer, indent_depth);
    tokenizer.expect_token(TokenKind::CloseBrace);
    tokenizer.skip_space();
    Some(Block {
        indent_depth: indent_depth,
        stmts: stmts,
    })
}

fn parse_inside<'a>(tokenizer: &'a Tokenizer<'a>, indent_depth: usize, stmts: &mut Vec<Stmt>) {
    tokenizer.expect_symbol(TokenKind::NewLine);
    read_indent_by_depth(tokenizer, indent_depth);
    // if tokenizer.peek_token().is(TokenKind::CloseBrace) {
    //     return;
    // } 
    // read_indent_by_depth(tokenizer, 1);
    // stmts.push(parse_stmt(tokenizer).unwrap_or_else(|| {
    //     emit_error!(tokenizer.location(), "only stmts can be in block");
    // }));
    // parse_inside(tokenizer, indent_depth, stmts);

    match tokenizer.peek_token().kind {
        TokenKind::CloseBrace => {
            return;
        },
        TokenKind::NewLine => {
            parse_inside(tokenizer, indent_depth, stmts);
        },
        _ => {
            read_indent_by_depth(tokenizer, 1);
            stmts.push(parse_stmt(tokenizer).unwrap_or_else(|| {
                emit_error!(tokenizer.location(), "only stmts can be in block");
            }));
            parse_inside(tokenizer, indent_depth, stmts);
        }
    }
}

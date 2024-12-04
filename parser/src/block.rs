use data::{Block, Stmt};
use tokenizer::{TokenKind, Tokenizer};

use crate::{parse_stmt, read_indent_by_depth};

pub fn parse_block<'a>(tokenizer: &'a Tokenizer<'a>, indent_depth: usize) -> Option<Block<'a>> {
    if !tokenizer.peek_symbol().is(TokenKind::OpenBrace) {
        return None;
    }
    let loc = tokenizer.location();
    tokenizer.next_token();
    // code
    let mut stmts: Vec<Box<dyn Stmt + 'a>> = Vec::new();
    parse_inside(tokenizer, indent_depth, &mut stmts);
    // read_indent_by_depth(tokenizer, indent_depth);
    tokenizer.expect_token(TokenKind::CloseBrace);
    tokenizer.skip_space();
    Some(Block::new(indent_depth, stmts, loc))
}

fn parse_inside<'a>(
    tokenizer: &'a Tokenizer<'a>,
    indent_depth: usize,
    stmts: &mut Vec<Box<dyn Stmt + 'a>>,
) {
    tokenizer.expect_symbol(TokenKind::NewLine);
    read_indent_by_depth(tokenizer, indent_depth);

    match tokenizer.peek_token().kind {
        TokenKind::CloseBrace => {
            return;
        }
        TokenKind::NewLine => {
            parse_inside(tokenizer, indent_depth, stmts);
        }
        _ => {
            read_indent_by_depth(tokenizer, 1);
            stmts.push(parse_stmt(tokenizer, indent_depth + 1));
            parse_inside(tokenizer, indent_depth, stmts);
        }
    }
}

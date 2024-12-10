use data::{Block, Stmt};
use tokenizer::{TokenKind, Tokenizer};

use crate::{parse_stmt, read_indent_by_depth};

// <block> = "{" <stmt>* "}"
pub fn parse_block<'a>(tokenizer: &'a Tokenizer<'a>, indent_depth: usize) -> Block<'a> {
    let loc = tokenizer.location();

    // "{"
    assert!(tokenizer.peek_symbol().is(TokenKind::OpenBrace));
    tokenizer.next_token();

    // <stmt>*
    let mut stmts: Vec<Box<dyn Stmt + 'a>> = Vec::new();
    parse_inside(tokenizer, indent_depth, &mut stmts);

    // "}"
    tokenizer.expect_token(TokenKind::CloseBrace);
    tokenizer.skip_space();

    Block::new(indent_depth, stmts, loc)
}

// <stmts>*
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
        // <stmt>*
        _ => {
            read_indent_by_depth(tokenizer, 1);

            // <stmt>
            if !tokenizer.peek_token().is(TokenKind::Space) {
                stmts.push(parse_stmt(tokenizer, indent_depth + 1));
            }

            parse_inside(tokenizer, indent_depth, stmts);
        }
    }
}

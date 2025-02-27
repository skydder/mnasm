use std::{cell::RefCell, rc::Rc};

use data::{Block, Scope, Stmt};
use tokenizer::{TokenKind, Tokenizer2};
use util::AsmResult;

use crate::{parse_stmt, read_indent_by_depth};

// <block> = "{" <stmt>* "}"
pub fn parse_block<'a>(
    tokenizer: &'a Tokenizer2<'a>,
    indent_depth: usize,
    scope: Rc<RefCell<Scope<'a>>>,
) -> AsmResult<'a, Block<'a>> {
    let loc = tokenizer.location();

    // "{"
    tokenizer.consume_token(TokenKind::OpenBrace);
    tokenizer.add_to_code(TokenKind::NewLine);
    // <stmt>*
    let mut stmts: Vec<Box<dyn Stmt + 'a>> = Vec::new();

    parse_inside(tokenizer, indent_depth, &mut stmts, scope.clone())?;

    // "}"
    tokenizer.consume_token(TokenKind::CloseBrace);
    tokenizer.skip_space(true);

    Ok(Block::new(indent_depth, stmts, loc, scope))
}

// <stmts>*
fn parse_inside<'a>(
    tokenizer: &'a Tokenizer2<'a>,
    indent_depth: usize,
    stmts: &mut Vec<Box<dyn Stmt<'a> + 'a>>,
    scope: Rc<RefCell<Scope<'a>>>,
) -> AsmResult<'a, ()> {
    tokenizer.skip_space(true);
    tokenizer.consume_newline();
    read_indent_by_depth(tokenizer, indent_depth);

    match tokenizer.peek_token(true).kind {
        TokenKind::CloseBrace => Ok(()),
        TokenKind::NewLine | TokenKind::Semicolon | TokenKind::EOS => {
            tokenizer.add_to_code(TokenKind::NewLine);
            parse_inside(tokenizer, indent_depth, stmts, scope)
        }
        // <stmt>*
        _ => {
            read_indent_by_depth(tokenizer, 1);
            tokenizer.skip_space(true);
            // <stmt>
            if !(tokenizer.peek_token(true).is(TokenKind::Space)
                || tokenizer.peek_token(true).is(TokenKind::NewLine)
                || tokenizer.peek_token(true).is(TokenKind::Semicolon)
                || tokenizer.peek_token(true).is(TokenKind::EOS))
            {
                stmts.push(parse_stmt(tokenizer, indent_depth + 1, scope.clone())?);
            } else {
                tokenizer.add_to_code(TokenKind::NewLine);
            }

            parse_inside(tokenizer, indent_depth, stmts, scope)
        }
    }
}

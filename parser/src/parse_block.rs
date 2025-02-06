use std::{cell::RefCell, rc::Rc};

use data::{Block, Scope, Stmt};
use tokenizer::{TokenKind, Tokenizer2};

use crate::{parse_stmt, read_indent_by_depth};

// <block> = "{" <stmt>* "}"
pub fn parse_block<'a>(
    tokenizer: &'a Tokenizer2<'a>,
    indent_depth: usize,
    scope: Rc<RefCell<Scope<'a>>>,
) -> Block<'a> {
    let loc = tokenizer.location();

    // "{"
    assert!(tokenizer.peek_token().is(TokenKind::OpenBrace));
    tokenizer.next_token();
    tokenizer.add_to_code(TokenKind::NewLine);
    // <stmt>*
    let mut stmts: Vec<Box<dyn Stmt + 'a>> = Vec::new();
    // let inner_scope = Rc::new(RefCell::new(Scope::new(None, Some(scope))));
    // parse_inside(tokenizer, indent_depth, &mut stmts, inner_scope.clone());

    parse_inside(tokenizer, indent_depth, &mut stmts, scope.clone());

    // "}"
    tokenizer.consume_token(TokenKind::CloseBrace);
    tokenizer.skip_space();

    tokenizer.add_to_code(TokenKind::NewLine);

    // Block::new(indent_depth, stmts, loc, inner_scope)
    Block::new(indent_depth, stmts, loc, scope)
}

// <stmts>*
fn parse_inside<'a>(
    tokenizer: &'a Tokenizer2<'a>,
    indent_depth: usize,
    stmts: &mut Vec<Box<dyn Stmt<'a> + 'a>>,
    scope: Rc<RefCell<Scope<'a>>>,
) {
    tokenizer.skip_space();
    tokenizer.consume_newline();
    read_indent_by_depth(tokenizer, indent_depth);

    match tokenizer.peek_token().kind {
        TokenKind::CloseBrace => {
            return;
        }
        TokenKind::NewLine | TokenKind::Semicolon | TokenKind::EOS=> {
            tokenizer.add_to_code(TokenKind::NewLine);
            parse_inside(tokenizer, indent_depth, stmts, scope);
        }
        // <stmt>*
        _ => {
            read_indent_by_depth(tokenizer, 1);
            tokenizer.skip_space();
            // <stmt>
            if !(tokenizer.peek_token().is(TokenKind::Space)
                || tokenizer.peek_token().is(TokenKind::NewLine)
                || tokenizer.peek_token().is(TokenKind::Semicolon)
                || tokenizer.peek_token().is(TokenKind::EOS))
            {
                stmts.push(parse_stmt(tokenizer, indent_depth + 1, scope.clone()));
            } else {
                tokenizer.add_to_code(TokenKind::NewLine);
            }

            parse_inside(tokenizer, indent_depth, stmts, scope)
        }
    }
}

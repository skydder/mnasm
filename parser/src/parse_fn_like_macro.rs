use std::{cell::RefCell, rc::Rc};

use data::{Scope, Stmt};
use tokenizer::{TokenKind, Tokenizer2};
use util::{emit_error, Location};

use crate::{parse_label, parse_stmt};

// todo:
// read macro name, args(these are also stream)
// pass them to tokenizer
pub fn parse_fn_like_macro<'a>(
    tokenizer: &'a Tokenizer2<'a>,
    indent_depth: usize,
    scope: Rc<RefCell<Scope<'a>>>,
) -> Box<dyn Stmt<'a> + 'a> {
    let loc = tokenizer.location();
    tokenizer.consume_token(TokenKind::At);
    tokenizer.skip_space();
    tokenizer.add_to_code(TokenKind::At);
    // <instruction>
    let macro_name = parse_label(tokenizer, scope.clone()).0;

    // "("
    tokenizer.consume_token(TokenKind::OpenParenthesis);

    // <operands>?
    let mut args: Vec<(Location<'a>, Location<'a>)> = Vec::new();
    read_args(tokenizer, &mut args);
    // ")"
    tokenizer.consume_token(TokenKind::CloseParenthesis);
    tokenizer.add_to_code(TokenKind::At);
    if let Some(m) = scope.clone().borrow().find_macro(macro_name.ident()) {
        tokenizer.enter_macro(
            m.ingredients_of_tokenizer(),
            m.args.iter().map(|a| *a).zip(args).collect(),
        );
        let op = parse_stmt(tokenizer, indent_depth, scope.clone());
        tokenizer.leave_macro();
        return op;
    } else {
        emit_error!(loc, "undefined macro");
    }
}

fn read_args<'a>(tokenizer: &'a Tokenizer2<'a>, args: &mut Vec<(Location<'a>, Location<'a>)>) {
    tokenizer.skip_space();
    match tokenizer.peek_token().kind {
        TokenKind::CloseParenthesis => {
            return;
        }
        TokenKind::Comma => {
            tokenizer.next_token();
            read_args(tokenizer, args);
        }
        _ => {
            let s_start = tokenizer.location();
            let mut current = tokenizer.peek_token();
            let mut s_end = s_start;
            while !current.is(TokenKind::At) {
                tokenizer.next_token();
                tokenizer.skip_space();
                current = tokenizer.peek_token();
                s_end = tokenizer.location();
            }
            args.push((s_start, s_end));
            tokenizer.next_token();
            tokenizer.skip_space();
            current = tokenizer.peek_token();
            if !(current.is(TokenKind::CloseParenthesis) || current.is(TokenKind::Comma)) {
                emit_error!(current.location, "unexpected token?");
            }
            read_args(tokenizer, args);
        }
    }
}

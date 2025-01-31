use std::{cell::RefCell, rc::Rc};

use data::{Ident, Macro, Scope};
use tokenizer::{TokenKind, Tokenizer2};
use util::emit_error;

// eventually, make these not be recorded

pub fn parse_let_macro<'a>(
    tokenizer: &'a Tokenizer2<'a>,
    scope: Rc<RefCell<Scope<'a>>>,
) -> Macro<'a> {
    let loc = tokenizer.location();
    tokenizer.consume_token(TokenKind::Identifier("let"));
    tokenizer.skip_space();
    tokenizer.consume_token(TokenKind::OpenParenthesis);
    tokenizer.skip_space();
    let ident = match tokenizer.peek_token().kind {
        TokenKind::Identifier(ident) => Ident::new(ident, false),
        _ => {
            todo!();
        }
    };
    tokenizer.next_token();
    tokenizer.skip_space();
    tokenizer.consume_token(TokenKind::Comma);
    tokenizer.skip_space();

    if tokenizer.peek_token().is(TokenKind::CloseParenthesis) {
        emit_error!(tokenizer.location(), "unexpected token, expected stream");
    }

    let start_loc = tokenizer.location();
    let mut end = tokenizer.location();
    while !tokenizer.peek_token().is(TokenKind::MacroEnd) {
        tokenizer.next_token();
        tokenizer.skip_space();
        end = tokenizer.location();
    }

    tokenizer.consume_token(TokenKind::MacroEnd);
    tokenizer.skip_space();
    tokenizer.consume_token(TokenKind::CloseParenthesis);
    scope.borrow_mut().add_macro(
        ident,
        Rc::new(Macro::new(loc, Vec::new(), (start_loc, end))),
    );
    Macro::new(loc, Vec::new(), (start_loc, end))
}

pub fn parse_fn_like_macro_def<'a>(
    tokenizer: &'a Tokenizer2<'a>,
    scope: Rc<RefCell<Scope<'a>>>,
) -> Macro<'a> {
    let loc = tokenizer.location();
    tokenizer.consume_token(TokenKind::Identifier("macro"));
    tokenizer.skip_space();
    // get macro name
    let ident = match tokenizer.peek_token().kind {
        TokenKind::Identifier(ident) => Ident::new(ident, false),
        _ => {
            todo!();
        }
    };
    tokenizer.next_token();
    tokenizer.skip_space();

    tokenizer.consume_token(TokenKind::OpenParenthesis);

    // get args
    let mut args: Vec<&'a str> = Vec::new();
    read_args(tokenizer, &mut args);

    tokenizer.consume_token(TokenKind::CloseParenthesis);
    tokenizer.skip_space();

    tokenizer.skip_space();
    let m_start = tokenizer.location();
    tokenizer.consume_token(TokenKind::OpenBrace);
    while !tokenizer.peek_token_silently().is(TokenKind::CloseBrace) {
        tokenizer.skip_token();
        tokenizer.skip_space();
    }
    tokenizer.consume_token(TokenKind::CloseBrace);
    let m_end = tokenizer.location();
    scope.borrow_mut().add_macro(
        ident,
        Rc::new(Macro::new(loc, args.clone(), (m_start, m_end))),
    );
    Macro::new(loc, args, (m_start, m_end))
}

fn read_args<'a>(tokenizer: &'a Tokenizer2<'a>, args: &mut Vec<&'a str>) {
    tokenizer.skip_space();
    match tokenizer.peek_token().kind {
        TokenKind::CloseParenthesis => {
            return;
        }
        TokenKind::Identifier(ident) => {
            args.push(ident);
            tokenizer.next_token();
            read_args(tokenizer, args);
        }
        TokenKind::Comma => {
            tokenizer.next_token();
            read_args(tokenizer, args);
        }
        _ => {
            emit_error!(tokenizer.location(), "unexpected token");
        }
    }
}

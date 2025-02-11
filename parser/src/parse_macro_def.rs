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
        TokenKind::Identifier(ident) => Ident::new(ident),
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
    let mut counter = 1;
    while counter > 0 {
        tokenizer.next_token();
        match tokenizer.peek_token().kind {
            TokenKind::CloseParenthesis => {
                counter -= 1;
            },
            TokenKind::OpenParenthesis => {
                counter += 1;
            },
            _ => ()
        };
    }
    let end = tokenizer.location();
    
    // tokenizer.consume_token(TokenKind::MacroEnd);
    tokenizer.skip_space();
    tokenizer.consume_token(TokenKind::CloseParenthesis);
    scope.borrow_mut().add_macro(
        ident,
        Rc::new(Macro::new(loc, Vec::new(), (start_loc, end))),
    );
    Macro::new(loc, Vec::new(), (start_loc, end))
}

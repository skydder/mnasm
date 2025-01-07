use std::{cell::RefCell, rc::Rc};

use data::{Ident, Macro, Scope};
use tokenizer::{TokenKind, Tokenizer};

pub fn parse_let_macro<'a>(
    tokenizer: &'a Tokenizer<'a>,
    scope: Rc<RefCell<Scope<'a>>>,
) -> Macro<'a> {
    let loc = tokenizer.location();
    tokenizer.expect_token(TokenKind::Identifier("let"));
    tokenizer.expect_symbol(TokenKind::OpenParenthesis);
    tokenizer.skip_space();
    let ident = match tokenizer.peek_token().kind {
        TokenKind::Identifier(ident) => Ident::new(ident, false),
        _ => {
            todo!();
        }
    };
    tokenizer.next_token();
    tokenizer.expect_symbol(TokenKind::Comma);
    let mut macros = Vec::new();
    while !tokenizer.peek_token().is(TokenKind::CloseParenthesis) {
        macros.push(tokenizer.next_symbol());
    }
    tokenizer.expect_token(TokenKind::CloseParenthesis);
    let macros = Rc::new(macros);

    scope
        .borrow_mut()
        .add_macro(ident, Rc::new(Macro::new(loc, macros.clone())));
    Macro::new(loc, macros)
}

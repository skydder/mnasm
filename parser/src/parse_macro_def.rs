use std::{cell::RefCell, rc::Rc};

use data::{Ident, Macro, Scope};
use tokenizer::{TokenGenerator, TokenKind};

pub fn parse_let_macro<'a>(
    tokenizer: &'a (dyn TokenGenerator + 'a),
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
    let mut macros = Vec::new();
    while !tokenizer.peek_token().is(TokenKind::CloseParenthesis) {
        macros.push(tokenizer.next_token());
        tokenizer.skip_space();
    }
    tokenizer.consume_token(TokenKind::CloseParenthesis);
    let macros = Rc::new(macros);

    scope
        .borrow_mut()
        .add_macro(ident, Rc::new(Macro::new(loc, macros.clone())));
    Macro::new(loc, macros)
}

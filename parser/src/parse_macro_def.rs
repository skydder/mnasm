use std::{cell::RefCell, rc::Rc};

use data::{Ident, Macro, Scope};
use tokenizer::TokenKind;

use crate::tokenizer::Tokenizer2;

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

    let start_loc = tokenizer.location();
    let mut end = tokenizer.location();
    while !tokenizer.peek_token().is(TokenKind::At) {
        tokenizer.next_token();
        tokenizer.skip_space();
        end = tokenizer.location();
    }
    
    tokenizer.consume_token(TokenKind::At);
    tokenizer.skip_space();
    tokenizer.consume_token(TokenKind::CloseParenthesis);
    scope
        .borrow_mut()
        .add_macro(ident, Rc::new(Macro::new(loc, (start_loc, end))));
    Macro::new(loc, (start_loc, end))
}

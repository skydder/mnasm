use std::{cell::RefCell, rc::Rc};

use data::{Ident, Label, Path, Scope};
use tokenizer::{TokenKind, Tokenizer2};

// "."? <ident> ("." <ident>)*
pub fn parse_label<'a>(tokenizer: &'a Tokenizer2<'a>, scope: Rc<RefCell<Scope<'a>>>) -> Label<'a> {
    let location = tokenizer.location();
    let is_relative = if tokenizer.peek_token().is(TokenKind::Dot) {
        tokenizer.next_token();
        true
    } else {
        false
    };
    let mut path: Vec<Ident<'a>> = Vec::new();
    path.push(Ident::new(tokenizer.peek_token().get_identifier().unwrap())); // todo
    tokenizer.next_token();

    while tokenizer.peek_token().is(TokenKind::Dot) {
        tokenizer.next_token();
        path.push(Ident::new(tokenizer.peek_token().get_identifier().unwrap()));
        tokenizer.next_token();
    }

    Label::new(
        *path.last().unwrap(),
        scope,
        location,
        Path::new(is_relative, path),
    )
}

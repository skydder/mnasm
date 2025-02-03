use std::{cell::RefCell, panic::Location, rc::Rc};

use data::{Ident, Label, Path, Scope};
use tokenizer::{TokenKind, Tokenizer2};
use util::emit_error;

// pub fn parse_label__<'a>(
//     tokenizer: &'a Tokenizer2<'a>,
//     scope: Rc<RefCell<Scope<'a>>>,
// ) -> (Label<'a>, TokenKind<'a>) {
//     match tokenizer.peek_token().kind {
//         TokenKind::Dot => {
//             tokenizer.next_token();
//             let label = Label::new(
//                 Ident::new(
//                     tokenizer.peek_token().get_identifier().unwrap_or_else(|| {
//                         emit_error!(tokenizer.location(), "expected label here but found other");
//                     })
//                 ),
//                 scope,
//                 tokenizer.location(),
//             );
//             (label.clone(), tokenizer.next_token_silently().kind)
//         }
//         TokenKind::Identifier(ident) => {
//             let label = Label::new(Ident::new(ident), scope, tokenizer.location());
//             (label.clone(), tokenizer.next_token_silently().kind)
//         }
//         _ => {
//             emit_error!(tokenizer.location(), "expected label here but found other");
//         }
//     }
// }

// "."? <ident> ("." <ident>)*
pub fn parse_label<'a>(
    tokenizer: &'a Tokenizer2<'a>,
    scope: Rc<RefCell<Scope<'a>>>,
) -> Label<'a> {
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

    Label::new(*path.last().unwrap(), scope, location, Path::new(is_relative, path))
}

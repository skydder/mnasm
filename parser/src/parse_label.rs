use std::{cell::RefCell, rc::Rc};

use data::{Ident, Label, Scope};
use tokenizer::{TokenKind, Tokenizer2};
use util::emit_error;

pub fn parse_label<'a>(
    tokenizer: &'a Tokenizer2<'a>,
    scope: Rc<RefCell<Scope<'a>>>,
) -> (Label<'a>, TokenKind<'a>) {
    match tokenizer.peek_token().kind {
        TokenKind::Dot => {
            tokenizer.next_token();
            let label = Label::new(
                Ident::new(
                    tokenizer.peek_token().get_identifier().unwrap_or_else(|| {
                        emit_error!(tokenizer.location(), "expected label here but found other");
                    }),
                    true,
                ),
                scope,
                tokenizer.location(),
            );
            (label.clone(), tokenizer.next_token_with_out_record().kind)
        }
        TokenKind::Identifier(ident) => {
            let label = Label::new(Ident::new(ident, false), scope, tokenizer.location());
            (label.clone(), tokenizer.next_token_with_out_record().kind)
        }
        _ => {
            emit_error!(tokenizer.location(), "expected label here but found other");
        }
    }
}

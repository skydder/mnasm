use std::{cell::RefCell, rc::Rc};

use data::{Ident, Label, Scope};
use tokenizer::{TokenGenerator, TokenKind};
use util::emit_error;

use crate::tokenizer::Tokenizer2;

pub fn parse_label<'a>(tokenizer: &'a mut Tokenizer2, scope: Rc<RefCell<Scope<'a>>>) -> Label<'a> {
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
            tokenizer.next_token();
            label
        }
        TokenKind::Identifier(ident) => {
            let label = Label::new(Ident::new(ident, false), scope, tokenizer.location());
            tokenizer.next_token();
            label
        }
        _ => {
            emit_error!(tokenizer.location(), "expected label here but found other");
        }
    }
}

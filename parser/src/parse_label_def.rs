use std::{cell::RefCell, rc::Rc};

use data::{LabelDef, Name, Scope};
use tokenizer::{TokenKind, Tokenizer};
use util::emit_error;

use crate::parse_block;

// <label_def> = "<" <label> (":" "global")? (":" <section> )? ">" <block>?
pub fn parse_label_def<'a>(
    tokenizer: &'a Tokenizer<'a>,
    indent_depth: usize,
    scope: Rc<RefCell<Scope<'a>>>,
) -> LabelDef<'a> {
    let loc = tokenizer.location();

    // "<"
    assert!(tokenizer.peek_token().is(TokenKind::LessThan));
    tokenizer.next_token();

    // <label>
    let label = Name::new(tokenizer.peek_symbol().get_identifier().unwrap_or_else(|| {
        emit_error!(tokenizer.location(), "expected label here but found other");
    }));
    tokenizer.next_token();
    scope.borrow_mut().add_label(label);
    // kimokimo-nest :<

    // (":" "global")? (":" <section> )?
    let (is_global, section) = if tokenizer.peek_symbol().is(TokenKind::Colon) {
        tokenizer.next_token();
        // "global" (":" <section> )?
        if tokenizer.peek_symbol().is(TokenKind::Identifier("global")) {
            tokenizer.next_token();

            // (":" <section> )?
            let sec = if tokenizer.peek_symbol().is(TokenKind::Colon) {
                tokenizer.next_token();

                // <section>
                let s = tokenizer.peek_symbol().get_identifier().unwrap_or_else(|| {
                    emit_error!(tokenizer.location(), "expected label here but found other");
                });
                tokenizer.next_token();

                Some(Name::new(s))
            } else {
                None
            };
            (true, sec)

        // <section>
        } else {
            let sec = Some(Name::new(
                tokenizer.peek_symbol().get_identifier().unwrap_or_else(|| {
                    emit_error!(tokenizer.location(), "expected label here but found other");
                }),
            ));
            tokenizer.next_token();
            (false, sec)
        }
    } else {
        (false, None)
    };

    // ">"
    tokenizer.expect_symbol(TokenKind::GreaterThan);
    tokenizer.skip_space();

    // <block>?
    let block = match tokenizer.peek_token().kind {
        TokenKind::OpenBrace => Some(parse_block(
            tokenizer,
            indent_depth,
            Rc::new(RefCell::new(Scope::new(Some(label), Some(scope)))),
        )),
        TokenKind::NewLine | TokenKind::EOF => None,
        _ => {
            todo!()
        }
    };

    LabelDef::new(label, is_global, section, block, loc)
}

use std::{cell::RefCell, rc::Rc};

use data::{Ident, LabelDef, Scope};
use tokenizer::{TokenGenerator, TokenKind};
use util::emit_error;

use crate::{parse_block, parse_label};

// <label_def> = "<" <label> (":" "global")? (":" <section> )? ">" <block>?
pub fn parse_label_def<'a>(
    tokenizer: &'a (dyn TokenGenerator + 'a),
    indent_depth: usize,
    scope: Rc<RefCell<Scope<'a>>>,
) -> LabelDef<'a> {
    let loc = tokenizer.location();

    // "<"
    assert!(tokenizer.peek_token().is(TokenKind::LessThan));
    tokenizer.next_token();
    tokenizer.skip_space();

    // <label>
    let label = parse_label(tokenizer, scope.clone()).ident();

    if scope.borrow().find_label(label).is_some() {
        emit_error!(loc, "multiple difinition!!")
    }

    let gen_label = scope.borrow().gen_label(label);
    scope.borrow_mut().add_label(label);
    // kimokimo-nest :<

    // (":" "global")? (":" <section> )?
    tokenizer.skip_space();
    let (is_global, section) = if tokenizer.peek_token().is(TokenKind::Colon) {
        tokenizer.next_token();
        tokenizer.skip_space();
        // "global" (":" <section> )?
        if tokenizer.peek_token().is(TokenKind::Identifier("global")) {
            tokenizer.next_token();
            tokenizer.skip_space();

            // (":" <section> )?
            let sec = if tokenizer.peek_token().is(TokenKind::Colon) {
                tokenizer.next_token();
                tokenizer.skip_space();
                // <section>
                Some(parse_section(tokenizer))
            } else {
                None
            };
            (true, sec)

        // <section>
        } else {
            let sec = Some(parse_section(tokenizer));
            (false, sec)
        }
    } else {
        (false, None)
    };

    // ">"
    tokenizer.skip_space();
    tokenizer.consume_token(TokenKind::GreaterThan);
    tokenizer.skip_space();

    // <block>?
    let block = match tokenizer.peek_token().kind {
        TokenKind::OpenBrace => Some(parse_block(
            tokenizer,
            indent_depth,
            Rc::new(RefCell::new(Scope::new(Some(label), Some(scope)))),
        )),
        TokenKind::NewLine | TokenKind::EOS => None,
        _ => {
            todo!()
        }
    };

    LabelDef::new(label, gen_label, is_global, section, block, loc)
}

fn parse_section<'a>(tokenizer: &'a (dyn TokenGenerator + 'a)) -> Ident<'a> {
    let s = if tokenizer.peek_token().is(TokenKind::Dot) {
        tokenizer.next_token();
        // todo: add all reserved section
        match tokenizer.peek_token().kind {
            TokenKind::Identifier("text") => Ident::new("text", true),
            TokenKind::Identifier("data") => Ident::new("data", true),
            TokenKind::Identifier("bss") => Ident::new("bss", true),
            _ => {
                emit_error!(tokenizer.location(), "only special token can come here")
            }
        }
    } else {
        tokenizer.skip_space();
        Ident::new(
            tokenizer.peek_token().get_identifier().unwrap_or_else(|| {
                emit_error!(tokenizer.location(), "consumeed label here but found other");
            }),
            false,
        )
    };
    tokenizer.next_token();
    return s;
}

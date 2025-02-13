use std::{cell::RefCell, rc::Rc};

use data::{Ident, LabelDef, Scope};
use tokenizer::{TokenKind, Tokenizer2};
use util::emit_error;

use crate::{parse_block, parse_label};

// <label_def> = "<" <label> (":" "global")? (":" <section> )? ">" <block>?
pub fn parse_label_def<'a>(
    tokenizer: &'a Tokenizer2<'a>,
    indent_depth: usize,
    scope: Rc<RefCell<Scope<'a>>>,
) -> LabelDef<'a> {
    let loc = tokenizer.location();

    // "<"
    assert!(tokenizer.peek_token(true).is(TokenKind::LessThan));
    tokenizer.next_token();
    tokenizer.skip_space(true);

    // <label>
    let label_data = parse_label(tokenizer, scope.clone());
    let label = label_data.ident();
    if scope.borrow().find_label_local(&label_data.path).is_some() {
        emit_error!(loc, "multiple difinition!!")
    }

    let gen_label = scope.borrow().gen_label(label);
    // kimokimo-nest :<

    // (":" "global")? (":" <section> )?
    tokenizer.skip_space(true);
    let (is_global, section) = if tokenizer.peek_token(true).is(TokenKind::Colon) {
        tokenizer.next_token();
        tokenizer.skip_space(true);
        // "global" (":" <section> )?
        if tokenizer.peek_token(true).is(TokenKind::Identifier("global")) {
            tokenizer.next_token();
            tokenizer.skip_space(true);

            // (":" <section> )?
            let sec = if tokenizer.peek_token(true).is(TokenKind::Colon) {
                tokenizer.next_token();
                tokenizer.skip_space(true);
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
    tokenizer.skip_space(true);
    tokenizer.consume_token(TokenKind::GreaterThan);
    tokenizer.skip_space(true);

    // <block>?
    let block = match tokenizer.peek_token(true).kind {
        TokenKind::OpenBrace => {
            let s = Rc::new(RefCell::new(Scope::new(Some(label), Some(scope.clone()))));
            scope.borrow_mut().add_label(label, Some(s.clone()));
            Some(parse_block(tokenizer, indent_depth, s))
        }
        TokenKind::NewLine | TokenKind::EOS => {
            scope.borrow_mut().add_label(label, None);
            // tokenizer.add_to_code(TokenKind::NewLine);
            None
        }
        _ => {
            todo!()
        }
    };
    LabelDef::new(label, gen_label, is_global, section, block, loc)
}

fn parse_section<'a>(tokenizer: &'a Tokenizer2<'a>) -> Ident<'a> {
    let s = if tokenizer.peek_token(true).is(TokenKind::Dot) {
        tokenizer.next_token();
        // todo: add all reserved section
        match tokenizer.peek_token(true).kind {
            TokenKind::Identifier("text") => Ident::new(".text"),
            TokenKind::Identifier("data") => Ident::new(".data"),
            TokenKind::Identifier("bss") => Ident::new(".bss"),
            _ => {
                emit_error!(tokenizer.location(), "only special token can come here")
            }
        }
    } else {
        tokenizer.skip_space(true);
        Ident::new(tokenizer.peek_token(true).get_identifier().unwrap_or_else(|| {
            emit_error!(tokenizer.location(), "consumeed label here but found other");
        }))
    };
    tokenizer.next_token();
    return s;
}

use data::{LabelDef, Name};
use tokenizer::{TokenKind, Tokenizer};
use util::emit_error;

use crate::parse_block;

// <label_def> = "<" <label> (":" "global")? (":" <section> )? ">" <block>?
pub fn parse_label_def<'a>(tokenizer: &'a Tokenizer<'a>, indent_depth: usize) -> LabelDef {
    let loc = tokenizer.location();

    // "<"
    assert!(tokenizer.peek_token().is(TokenKind::LessThan));
    tokenizer.next_token();

    // <label>
    let label = tokenizer.peek_symbol().get_identifier().unwrap_or_else(|| {
        emit_error!(tokenizer.location(), "expected label here but found other");
    });
    tokenizer.next_token();

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
            let sec = Some(Name::new(tokenizer.peek_symbol().get_identifier().unwrap_or_else(|| {
                emit_error!(tokenizer.location(), "expected label here but found other");
            })));
            tokenizer.next_token();
            (false, sec)
        }
    } else {
        (false,  None)
    };

    // ">"
    tokenizer.expect_symbol(TokenKind::GreaterThan);
    tokenizer.skip_space();

    // <block>?
    let block = match tokenizer.peek_token().kind {
        TokenKind::OpenBrace => Some(parse_block(tokenizer, indent_depth)),
        TokenKind::NewLine | TokenKind::EOF => None,
        _ => {
            todo!()
        }
    };

    LabelDef::new(Name::new(label), is_global, section, block, loc)
}

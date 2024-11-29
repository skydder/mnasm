use data::LabelDef;
use tokenizer::{TokenKind, Tokenizer};
use util::emit_error;

use crate::parse_block;

pub fn parse_label_def<'a>(tokenizer: &'a Tokenizer<'a>) -> Option<LabelDef> {
    if !tokenizer.peek_token().is(TokenKind::LessThan) {
        return None;
    }
    tokenizer.next_token();
    let label = tokenizer.peek_symbol().get_identifier().unwrap_or_else(|| {
        emit_error!(tokenizer.location(), "expected label here but found other");
    });
    tokenizer.next_token();

    // kimokimo-nest::
    let (is_global, section) = if tokenizer.peek_symbol().is(TokenKind::Colon) {
        tokenizer.next_token();

        if tokenizer.peek_symbol().is(TokenKind::Identifier("global")) {
            tokenizer.next_token();
            let sec = if tokenizer.peek_symbol().is(TokenKind::Colon) {
                tokenizer.next_token();
                let s = tokenizer.peek_symbol().get_identifier().unwrap_or_else(|| {
                    emit_error!(tokenizer.location(), "expected label here but found other");
                });
                tokenizer.next_token();
                s
            } else {
                ""
            };
            (true, sec)
        } else {
            let sec = tokenizer.peek_symbol().get_identifier().unwrap_or_else(|| {
                emit_error!(tokenizer.location(), "expected label here but found other");
            });
            tokenizer.next_token();
            (false, sec)
        }
    } else {
        (false, "")
    };

    tokenizer.expect_symbol(TokenKind::GreaterThan);
    tokenizer.skip_space();
    let block = match tokenizer.peek_token().kind {
        TokenKind::OpenBrace => parse_block(tokenizer, 0),
        TokenKind::NewLine | TokenKind::EOF => None,
        _ => {
            todo!()
        }
    };
    Some(LabelDef::new(label, is_global, section, block))
}

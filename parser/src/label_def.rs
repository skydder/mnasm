use tokenizer::{TokenKind, Tokenizer};
use util::emit_error;

use crate::{parse_block, Block};

#[derive(Debug)]
pub struct LabelDef<'a> {
    label: &'a str,
    block: Option<Block<'a>>,
}

pub fn parse_label_def<'a>(tokenizer: &'a Tokenizer<'a>) -> Option<LabelDef> {
    if !tokenizer.peek_token().is(TokenKind::LessThan) {
        return None;
    }
    tokenizer.next_symbol();
    let label = tokenizer.peek_symbol().get_identifier().unwrap_or_else(|| {
        emit_error!(tokenizer.location(), "expected label here but found other");
    });
    tokenizer.next_symbol();
    tokenizer.expect_symbol(TokenKind::GreaterThan);
    let block = parse_block(tokenizer, 0);
    Some(LabelDef {
        label: label,
        block: block,
    })
}

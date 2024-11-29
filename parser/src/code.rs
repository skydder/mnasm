use data::{Code, LabelDef};
use tokenizer::{TokenKind, Tokenizer};

use crate::parse_label_def;

pub fn parse_code<'a>(tokenizer: &'a Tokenizer<'a>) -> Option<Code<'a>> {
    let mut labels = Vec::new();
    parse_code_inside(tokenizer, &mut labels);
    Some(Code { labels: labels })
}

fn parse_code_inside<'a>(tokenizer: &'a Tokenizer<'a>, labels:&mut Vec<LabelDef<'a>>) {
    labels.push(parse_label_def(tokenizer).unwrap());
    if tokenizer.peek_token().is(TokenKind::EOF) {
        return;
    }
    tokenizer.expect_token(TokenKind::NewLine);
    parse_code_inside(tokenizer, labels);
}
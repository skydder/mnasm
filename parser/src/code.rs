use data::{Code, LabelDef};
use tokenizer::{TokenKind, Tokenizer};
use util::emit_error;

use crate::parse_label_def;

pub fn parse_code<'a>(tokenizer: &'a Tokenizer<'a>) -> Option<Code<'a>> {
    let mut labels = Vec::new();
    parse_code_inside(tokenizer, &mut labels);
    Some(Code { labels: labels })
}

fn parse_code_inside<'a>(tokenizer: &'a Tokenizer<'a>, labels: &mut Vec<LabelDef<'a>>) {
    let loc = tokenizer.location();

    // <space>*<EOF> will be error so it should be fixed
    // if tokenizer.peek_token().is(TokenKind::EOF) {
    //     return;
    // }

    while tokenizer.peek_token().is(TokenKind::Space)
        || tokenizer.peek_token().is(TokenKind::NewLine)
        || tokenizer.peek_token().is(TokenKind::EOF)
    {
        if tokenizer.peek_token().is(TokenKind::EOF) {
            return;
        }
        skip_null_line(tokenizer);
    }

    labels.push(
        parse_label_def(tokenizer, 0)
            .unwrap_or_else(|| emit_error!(tokenizer.location(), "unexpected token!")),
    );

    parse_code_inside(tokenizer, labels);
}

fn skip_null_line<'a>(tokenizer: &'a Tokenizer<'a>) {
    tokenizer.skip_space();
    tokenizer.expect_token(TokenKind::NewLine);
}

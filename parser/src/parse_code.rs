use data::{Code, LabelDef};
use tokenizer::{TokenKind, Tokenizer};

use crate::parse_label_def;

pub fn parse_code<'a>(tokenizer: &'a Tokenizer<'a>) -> Option<Code<'a>> {
    let mut labels = Vec::new();
    parse_code_inside(tokenizer, &mut labels);
    Some(Code { labels: labels })
}

fn parse_code_inside<'a>(tokenizer: &'a Tokenizer<'a>, labels: &mut Vec<LabelDef<'a>>) {
    // <space>*<EOF> will be error so it should be fixed
    // => fixed, however, not good?

    if is_eof(tokenizer) {
        return;
    }

    labels.push(parse_label_def(tokenizer, 0));

    parse_code_inside(tokenizer, labels);
}

fn skip_null_line<'a>(tokenizer: &'a Tokenizer<'a>) {
    tokenizer.skip_space();
    tokenizer.expect_newline();
}

fn is_eof<'a>(tokenizer: &'a Tokenizer<'a>) -> bool {
    match tokenizer.peek_token().kind {
        TokenKind::EOF => true,
        TokenKind::NewLine | TokenKind::Space => {
            skip_null_line(tokenizer);
            is_eof(tokenizer)
        }
        _ => {
            // emit_error!(tokenizer.location(), "unexpected token");
            false
        }
    }
}

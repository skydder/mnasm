use std::{cell::RefCell, rc::Rc};

use data::{Code, LabelDef, Scope};
use tokenizer::{TokenKind, Tokenizer};

use crate::parse_label_def;

// <code> = <label_def>*
pub fn parse_code<'a>(tokenizer: &'a Tokenizer<'a>) -> Code<'a> {
    // <label_def>*
    eprintln!("!test");
    let mut labels = Vec::new();
    parse_code_inside(tokenizer, &mut labels);
    eprintln!("{:#?}", labels);

    Code { labels: labels }
}

// <label_def>*
fn parse_code_inside<'a>(tokenizer: &'a Tokenizer<'a>, labels: &mut Vec<LabelDef<'a>>) {
    // <space>*<EOF> will be error so it should be fixed
    // => fixed, however, not good?
    eprintln!("11!");
    if is_eof(tokenizer) {
        eprintln!("ok1");
        return;
    }

    // <label_def>
    labels.push(parse_label_def(
        tokenizer,
        0,
        Rc::new(RefCell::new(Scope::new(None, None))),
    ));
    eprintln!("!!test");

    // *
    parse_code_inside(tokenizer, labels);
}

fn skip_null_line<'a>(tokenizer: &'a Tokenizer<'a>) {
    tokenizer.skip_space();
    tokenizer.expect_newline();
}

fn is_eof<'a>(tokenizer: &'a Tokenizer<'a>) -> bool {
    eprintln!("ok2");
    match tokenizer.peek_token().kind {
        TokenKind::EOF => true,
        TokenKind::NewLine | TokenKind::Space => {
            skip_null_line(tokenizer);
            is_eof(tokenizer)
        }
        _ => false,
    }
}

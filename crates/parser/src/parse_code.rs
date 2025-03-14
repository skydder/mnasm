use std::{cell::RefCell, rc::Rc};

use data::{Code, Scope, Stmt};
use tokenizer::{TokenKind, Tokenizer2};
use util::AsmResult;

use crate::parse_stmt;

// <code> = <label_def>*
pub fn parse_code<'a>(tokenizer: &'a Tokenizer2<'a>) -> AsmResult<'a, Code<'a>> {
    // <label_def>*
    let mut codes = Vec::new();
    let root = Rc::new(RefCell::new(Scope::new(None, None)));
    parse_code_inside(tokenizer, &mut codes, root.clone())?;
    Ok(Code { codes: codes })
}

// <label_def>*
fn parse_code_inside<'a>(
    tokenizer: &'a Tokenizer2<'a>,
    labels: &mut Vec<Box<dyn Stmt<'a> + 'a>>,
    root: Rc<RefCell<Scope<'a>>>,
) -> AsmResult<'a, ()> {
    // <space>*<EOS> will be error so it should be fixed
    // => fixed, however, not good?
    if is_eos(tokenizer) {
        return Ok(());
    }

    // <label_def>
    labels.push(parse_stmt(tokenizer, 0, root.clone())?);
    // *
    parse_code_inside(tokenizer, labels, root)
}

fn skip_null_line<'a>(tokenizer: &'a Tokenizer2<'a>) {
    tokenizer.skip_space(true);
    tokenizer.consume_newline();
}

fn is_eos<'a>(tokenizer: &'a Tokenizer2<'a>) -> bool {
    match tokenizer.peek_token(true).kind {
        TokenKind::EOS => true,
        TokenKind::NewLine | TokenKind::Semicolon | TokenKind::Space => {
            // eprintln!("{:#?}", tokenizer);
            skip_null_line(tokenizer);
            is_eos(tokenizer)
        }
        _ => {
            tokenizer.add_to_code(TokenKind::NewLine);
            false
        }
    }
}

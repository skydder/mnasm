use std::{cell::RefCell, rc::Rc};

use data::{Code, Scope, Stmt};
use tokenizer::{TokenGenerator, TokenKind};

use crate::parse_stmt;

// <code> = <label_def>*
pub fn parse_code<'a>(tokenizer: &'a (dyn TokenGenerator + 'a)) -> Code<'a> {
    // <label_def>*
    let mut codes = Vec::new();
    let root = Rc::new(RefCell::new(Scope::new(None, None)));
    parse_code_inside(tokenizer, &mut codes, root.clone());
    // eprintln!("{:#?}", root.clone());
    Code { codes: codes }
}

// <label_def>*
fn parse_code_inside<'a>(
    tokenizer: &'a (dyn TokenGenerator + 'a),
    labels: &mut Vec<Box<dyn Stmt<'a> + 'a>>,
    root: Rc<RefCell<Scope<'a>>>,
) {
    // <space>*<EOS> will be error so it should be fixed
    // => fixed, however, not good?
    if is_eos(tokenizer) {
        return;
    }

    // <label_def>
    labels.push(parse_stmt(tokenizer, 0, root.clone()));

    // *
    parse_code_inside(tokenizer, labels, root);
}

fn skip_null_line<'a>(tokenizer: &(dyn TokenGenerator + 'a)) {
    tokenizer.skip_space();
    tokenizer.consume_newline();
}

fn is_eos<'a>(tokenizer: &(dyn TokenGenerator + 'a)) -> bool {
    match tokenizer.peek_token().kind {
        TokenKind::EOS => true,
        TokenKind::NewLine | TokenKind::Space => {
            skip_null_line(tokenizer);
            is_eos(tokenizer)
        }
        _ => false,
    }
}

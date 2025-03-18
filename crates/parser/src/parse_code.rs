use std::{cell::RefCell, rc::Rc};

use data::{Code, Scope, Stmt};
use util::{AsmResult, TokenKind, Tokenizer};

use crate::parse_stmt;

// <code> = <label_def>*
pub fn parse_code<'a, T>(tokenizer: &'a T) -> AsmResult<'a, Code<'a>>
where
    T: Tokenizer<'a>,
{
    // <label_def>*
    let mut codes = Vec::new();
    let root = Rc::new(RefCell::new(Scope::new(None, None)));
    parse_code_inside(tokenizer, &mut codes, root.clone())?;
    Ok(Code { codes: codes })
}

// <label_def>*
fn parse_code_inside<'a, T>(
    tokenizer: &'a T,
    labels: &mut Vec<Box<dyn Stmt<'a> + 'a>>,
    root: Rc<RefCell<Scope<'a>>>,
) -> AsmResult<'a, ()>
where
    T: Tokenizer<'a>,
{
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

fn skip_null_line<'a, T>(tokenizer: &'a T)
where
    T: Tokenizer<'a>,
{
    tokenizer.skip_space(true);
    tokenizer.consume_newline();
}

fn is_eos<'a, T>(tokenizer: &'a T) -> bool
where
    T: Tokenizer<'a>,
{
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

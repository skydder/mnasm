use std::{cell::RefCell, rc::Rc};

use data::{Scope, Stmt};
use util::{AsmError, AsmResult, TokenKind, Tokenizer};

use crate::{parse_block, parse_compound_ins, parse_label_def, parse_pseudo_ins};

// <stmt> = <compound_ins> | <block> | <label_def>
pub fn parse_stmt<'a, T>(
    tokenizer: Rc<T>,
    indent_depth: usize,
    scope: Rc<RefCell<Scope<'a>>>,
) -> AsmResult<'a, Box<dyn Stmt<'a> + 'a>>
where
    T: Tokenizer<'a>,
{
    let currrent_token = tokenizer.peek_token(true);
    let new: AsmResult<Box<dyn Stmt<'a> + 'a>> = match currrent_token.kind {
        TokenKind::Not => Ok(Box::new(parse_pseudo_ins(tokenizer.clone(), scope)?)),
        TokenKind::Identifier("db") => Ok(Box::new(parse_pseudo_ins(tokenizer.clone(), scope)?)),
        TokenKind::Identifier("nasm") => Ok(Box::new(parse_pseudo_ins(tokenizer.clone(), scope)?)),
        TokenKind::Identifier("resb") => Ok(Box::new(parse_pseudo_ins(tokenizer.clone(), scope)?)),
        TokenKind::Identifier("extern") | TokenKind::Identifier("include") => {
            Ok(Box::new(parse_pseudo_ins(tokenizer.clone(), scope)?))
        }
        // <compound_stmt>
        TokenKind::Identifier(_) => Ok(Box::new(parse_compound_ins(tokenizer.clone(), scope)?)),

        // <block>
        TokenKind::OpenBrace => Ok(Box::new(parse_block(
            tokenizer.clone(),
            indent_depth,
            Rc::new(RefCell::new(Scope::new(None, Some(scope)))),
        )?)),

        // <label_def>
        TokenKind::LessThan => Ok(Box::new(parse_label_def(tokenizer.clone(), indent_depth, scope)?)),
        _ => Err(AsmError::ParseError(
            currrent_token.location,
            format!(
                "expected stmt, but found other!:{:?}",
                tokenizer.peek_token(true)
            ),
            "look at Stmt bnfs".to_string(),
        )),
    };
    tokenizer.add_to_code(TokenKind::NewLine);
    new
}

use std::rc::Rc;

use data::Ast;
use util::{AsmResult, TokenKind, Tokenizer};

use crate::{parse_block, parse_label_def, parse_line};

pub fn parse<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Ast<'code>>
where
    T: Tokenizer<'code>,
{
    match tokenizer.peek_token().kind {
        TokenKind::LessThan => parse_label_def(tokenizer),
        TokenKind::Identifier(_) => parse_line(tokenizer),
        TokenKind::OpenBrace => parse_block(tokenizer),
        TokenKind::NewLine => {
            tokenizer.next_token();
            parse(tokenizer)
        }
        _ => Err(util::AsmError::ParseError(
            tokenizer.location(),
            "unexpected token".to_string(),
            String::new(),
        )),
    }
}

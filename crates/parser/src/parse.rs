use std::rc::Rc;

use data::Ast;
use util::{AsmResult, TokenKind, Tokenizer};

use crate::{parse_label_block, parse_line, parse_macro};

pub fn parse<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Ast<'code>>
where
    T: Tokenizer<'code>,
{
    match tokenizer.peek_token().kind {
        TokenKind::LessThan | TokenKind::OpenBrace => parse_label_block(tokenizer),
        TokenKind::Identifier(_) => parse_line(tokenizer),
        TokenKind::At => parse_macro(tokenizer),
        TokenKind::NewLine => {
            tokenizer.next_token();
            parse(tokenizer)
        }
        TokenKind::EOS => Ok(Ast::EOS),
        _ => Err(util::AsmError::ParseError(
            tokenizer.location(),
            "unexpected token".to_string(),
            String::new(),
        )),
    }
}

pub fn parse_code<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Vec<Ast<'code>>>
where
    T: Tokenizer<'code>,
{
    let mut code = Vec::new();
    loop {
        let ast = parse(tokenizer.clone())?;
        match ast {
            Ast::EOS => return Ok(code),
            _ => code.push(ast),
        }
    }
}
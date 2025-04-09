use std::rc::Rc;

use data::Ast;
use util::{AsmResult, TokenKind, Tokenizer};

use crate::{parse_ins, util::parse_list};

pub fn parse_line<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Ast<'code>>
where
    T: Tokenizer<'code>,
{
    let location = tokenizer.location();
    let ins = parse_ins(tokenizer.clone())?;
    tokenizer.skip_space();
    if tokenizer.peek_token().is(&TokenKind::NewLine) {
        return Ok(ins);
    } else if !tokenizer.peek_token().is(&TokenKind::Comma) {
        return Err(util::AsmError::ParseError(tokenizer.location(), "expected comma, but found other".to_string(), String::new()));
    }
    tokenizer.consume_token(TokenKind::Comma)?;
    let mut list = parse_list(tokenizer, TokenKind::Comma, TokenKind::NewLine, parse_ins)?;
    list.insert(0, ins);
    Ok(Ast::Block(list, location, false))
}

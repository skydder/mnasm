use std::rc::Rc;

use data::Ast;
use util::{AsmResult, Tokenizer, TokenKind};

use crate::{parse_ident, util::parse_list, parse_operand};

pub fn parse_ins<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Ast<'code>>
where
    T: Tokenizer<'code>,
{
    let ins_name = parse_ident(tokenizer.clone())?;
    tokenizer.skip_space();
    tokenizer.consume_token(TokenKind::OpenParenthesis)?;
    let list = parse_list(tokenizer.clone(), TokenKind::Comma, TokenKind::CloseParenthesis, parse_operand)?;
    tokenizer.consume_token(TokenKind::CloseParenthesis)?;
    Ok(Ast::Ins(ins_name, list))
}

use std::rc::Rc;

use data::Ast;
use util::{AsmResult, TokenKind, Tokenizer};

use crate::{parse_ident, parse_operand, util::parse_list};

pub fn parse_ins<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Ast<'code>>
where
    T: Tokenizer<'code>,
{
    let ins_name = parse_ident(tokenizer.clone())?;
    tokenizer.skip_space();
    tokenizer.consume_token(TokenKind::OpenParenthesis)?;
    let list = parse_list(
        tokenizer.clone(),
        TokenKind::Comma,
        TokenKind::CloseParenthesis,
        parse_operand,
    )?;
    eprintln!("{:?}", list);
    tokenizer.consume_token(TokenKind::CloseParenthesis)?;
    Ok(Ast::Ins(ins_name, Rc::new(list)))
}

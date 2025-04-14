use std::rc::Rc;

use data::Ast;
use util::{AsmResult, TokenKind, Tokenizer};

use crate::{parse, util::parse_list};

pub fn parse_block<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Ast<'code>>
where
    T: Tokenizer<'code>,
{
    let location = tokenizer.location();
    tokenizer.consume_token(TokenKind::OpenBrace)?;
    if tokenizer.peek_token().is(&TokenKind::NewLine) {
        tokenizer.next_token();
    }
    let list = parse_list(
        tokenizer.clone(),
        TokenKind::NewLine,
        TokenKind::CloseBrace,
        parse,
    )?;

    tokenizer.consume_token(TokenKind::CloseBrace)?;
    Ok(Ast::Block(list, location, true))
}

use std::rc::Rc;

use data::{Ast, Strings, WithLocation};
use util::{AsmError, AsmResult, TokenKind, Tokenizer};

use crate::parse_ident;

pub fn parse_nasm<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Ast<'code>>
where
    T: Tokenizer<'code>,
{
    let ins_name = parse_ident(tokenizer.clone())?;
    if ins_name.data().get_str() != "nasm" {
        return Err(AsmError::ParseError(tokenizer.location(), "expected \"nasm\", but found others".to_string(), String::new()));
    }
    tokenizer.skip_space();
    tokenizer.consume_token(TokenKind::OpenParenthesis)?;
    let nasm = if let Some(s) = tokenizer.peek_token().get_strings() {
        let token = tokenizer.next_token();
        Ok(Ast::Nasm(WithLocation::new(token.location, Strings::new(s))))
    } else {
        eprintln!("{:?}", tokenizer.peek_token());
        Err(AsmError::ParseError(
            tokenizer.location(),
            "expected label, but could not find it".to_string(),
            String::new(),
        ))
    };
    tokenizer.consume_token(TokenKind::CloseParenthesis)?;
    nasm
}

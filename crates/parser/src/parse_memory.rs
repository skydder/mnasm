use std::rc::Rc;

use data::{Ast, Register};
use util::{AsmError, AsmResult, TokenKind, Tokenizer};

use crate::{parse_ident, util::parse_list};

pub fn parse_memory<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Ast<'code>>
where
    T: Tokenizer<'code>,
{
    let location = tokenizer.location();
    let location = tokenizer.location();
    if tokenizer.peek_token().get_identifier().is_some_and(|s| s.as_str() == "ptr") {
        tokenizer.next_token();
    } else {
        return Err(util::AsmError::ParseError(location, "expected Memory, but there isn't".to_string(), String::new()));
    }
    tokenizer.consume_token(TokenKind::LessThan).map_err(|_| AsmError::ParseError(tokenizer.location(), "memory size is requiered to be explicit".to_string(), String::new()))?;
    let size = match tokenizer.peek_token().get_identifier() {
        Some(s) if s.as_str() == "byte" => {
            todo!()
        },
        Some(s) if s.as_str() == "word" => {
            todo!()
        },
        Some(s) if s.as_str() == "dword" => {
            todo!()
        },
        Some(s) if s.as_str() == "qword" => {
            todo!()
        },
        _ => return Err(util::AsmError::ParseError(tokenizer.location(), "expected Memory size, but there isn't".to_string(), String::new())),
    }
    todo!()
}
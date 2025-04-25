use std::rc::Rc;

use data::Ident;
use util::{AsmError, AsmResult, Tokenizer};

pub fn parse_ident<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Ident<'code>>
where
    T: Tokenizer<'code>,
{
    if tokenizer.peek_token().is_identifier() {
        let token = tokenizer.next_token();
        Ok(Ident::new(token.get_identifier().unwrap().to_string(), token.location))
    } else {
        Err(AsmError::ParseError(
            tokenizer.location(),
            "expected label, but could not find it".to_string(),
            String::new(),
        ))
    }
}

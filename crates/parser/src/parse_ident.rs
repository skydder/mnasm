use std::rc::Rc;

use data::{Ident, WithLocation};
use util::{AsmError, AsmResult, Tokenizer};

pub fn parse_ident<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, WithLocation<'code, Ident>>
where
    T: Tokenizer<'code>,
{
    if tokenizer.peek_token().is_identifier() {
        let token = tokenizer.next_token();
        let location = token.location.clone();
        Ok(WithLocation::new(
            location,
            Ident::new(token.get_identifier().unwrap().to_string()),
        ))
    } else {
        Err(AsmError::ParseError(
            tokenizer.location(),
            "expected label, but could not find it".to_string(),
            String::new(),
        ))
    }
}

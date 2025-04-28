use std::rc::Rc;

use data::{Strings, WithLocation};
use util::{AsmError, AsmResult, Tokenizer};

pub fn parse_strings<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, WithLocation<'code, Strings>>
where
    T: Tokenizer<'code>,
{
    if let Some(s) = tokenizer.peek_token().get_strings() {
        let token = tokenizer.next_token();
        Ok(WithLocation::new(token.location, Strings::new(s)))
    } else {
        Err(AsmError::ParseError(
            tokenizer.location(),
            "expected label, but could not find it".to_string(),
            String::new(),
        ))
    }
}

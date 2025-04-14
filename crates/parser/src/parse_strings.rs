use std::rc::Rc;

use data::Strings;
use util::{AsmError, AsmResult, Tokenizer};

pub fn parse_strings<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Strings<'code>>
where
    T: Tokenizer<'code>,
{
    if let Some(s) = tokenizer.peek_token().get_strings() {
        let token = tokenizer.next_token();
        Ok(Strings::new(s, token.location))
    } else {
        Err(AsmError::ParseError(
            tokenizer.location(),
            "expected label, but could not find it".to_string(),
            String::new(),
        ))
    }
}

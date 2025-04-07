use std::rc::Rc;

use data::{Ast, Register};
use util::{AsmResult, Tokenizer};

pub fn parse_register<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Ast<'code>>
where
    T: Tokenizer<'code>,
{
    let location = tokenizer.location();
    let s = tokenizer.peek_token().get_identifier().unwrap(); // todo
    if let Some((kind, value, size)) = Register::get_reg_val(s.as_str()) {
        tokenizer.next_token();
        Ok(Ast::Register(Register::new(kind, value, size, location)))
    } else {
        Err(util::AsmError::ParseError(location, "expected Register, but there isn't".to_string(), String::new()))
    }
}
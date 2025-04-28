use std::rc::Rc;

use data::{Ast, Register, WithLocation};
use util::{AsmResult, Location, Tokenizer};

pub fn parse_register<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Ast<'code>>
where
    T: Tokenizer<'code>,
{
    let location = tokenizer.location();
    let s = tokenizer.peek_token().get_identifier().unwrap(); // todo
    let reg = parse_register_from_str(&s, location.clone())?;
    tokenizer.next_token();
    Ok(Ast::Register(WithLocation::new(location, reg)))
}

pub fn parse_register_from_str<'code>(
    tokenizer: &str,
    location: Location<'code>,
) -> AsmResult<'code, Register> {
    if let Some((kind, value, size)) = Register::get_reg_val(tokenizer) {
        Ok(Register::new(kind, value, size))
    } else {
        Err(util::AsmError::ParseError(
            location,
            "expected Register, but there isn't".to_string(),
            String::new(),
        ))
    }
}

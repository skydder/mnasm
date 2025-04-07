mod parse_ins;
mod parse_ident;
mod parse_operand;
mod parse_register;
mod parse_immediate;
mod parse_memory;
mod util;

use std::rc::Rc;

pub use parse_ident::parse_ident;
pub use parse_ins::parse_ins;

pub use parse_operand::parse_operand;
pub use parse_register::parse_register;
pub use parse_immediate::parse_immediate;


use ::util::{Tokenizer, AsmResult};

trait Parser<'code, T, R>: Fn(Rc<T>) -> AsmResult<'code, R> {}

impl<'code, F, R, T> Parser<'code, T, R> for F
where 
    F: Fn(Rc<T>) -> AsmResult<'code, R>,
    T: Tokenizer<'code>
{}
mod parse_ins;
mod parse_ident;
mod parse_operand;
mod parse_register;
mod parse_immediate;
mod parse_memory;
mod parse_label;
mod parse_strings;
mod parse_line;
mod parse_block;
mod parse_label_def;
mod util;
mod parse;

use std::rc::Rc;

pub use parse_ident::parse_ident;
pub use parse_ins::parse_ins;

pub use parse_operand::parse_operand;
pub use parse_register::parse_register;
pub use parse_immediate::parse_immediate;
pub use parse_memory::parse_memory;
pub use parse_label::parse_label;
pub use parse_strings::parse_strings;

pub use parse_line::parse_line;
pub use parse_label_def::parse_label_def;
pub use parse_block::parse_block;

pub use parse::parse;

use ::util::{Tokenizer, AsmResult};



trait Parser<'code, T, R>: Fn(Rc<T>) -> AsmResult<'code, R> {}

impl<'code, F, R, T> Parser<'code, T, R> for F
where 
    F: Fn(Rc<T>) -> AsmResult<'code, R>,
    T: Tokenizer<'code>
{}
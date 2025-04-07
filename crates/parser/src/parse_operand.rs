
use std::rc::Rc;

use data::Ast;
use util::{AsmResult, Tokenizer, TokenKind};

use crate::{parse_ident, util::parse_list};

pub fn parse_operand<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Ast<'code>>
where
    T: Tokenizer<'code>,
{
    
    todo!()
}

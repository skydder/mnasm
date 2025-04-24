use std::{cell::RefCell, rc::Rc};

use util::{AsmError, Location};

use crate::{label_block::LabelBlock, Strings};

use super::{
    ident::Ident,
    operand::{Immediate, Memory, Path, Register},
};


#[derive(Debug)]
pub enum Ast<'code> {
    Ins(Ident<'code>, Vec<Ast<'code>>),
    Label(Path<'code>),
    LabelBlock(LabelBlock<'code>),
    Macro(Ident<'code>, Box<Ast<'code>>, Vec<Ast<'code>>), // 1 ->
    Register(Register<'code>),
    Memory(Memory<'code>),
    Immediate(Immediate<'code>),
    String(Strings<'code>),
}

#[allow(clippy::needless_lifetimes)]
impl<'code> Ast<'code> {
    pub fn is_operand(&self) -> bool {
        matches!(
            self,
            Ast::Label(..) | Ast::Immediate(..) | Ast::Memory(..) | Ast::Register(..)
        )
    }

    pub fn location(&self) -> Location<'code> {
        match self {
            Ast::Ins(label, _) => label.location(),
            Ast::Label(path) => path.location(),
            Ast::LabelBlock(label, ..) => label.location(),
            Ast::Macro(label, ..) => label.location(),
            Ast::Register(register) => register.location(),
            Ast::Memory(memory) => memory.location(),
            Ast::Immediate(immediate) => immediate.location(),
            Ast::String(_) => todo!(),
        }
    }
}

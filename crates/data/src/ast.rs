use std::{cell::RefCell, rc::Rc};

use util::{AsmError, Location};

use crate::Strings;

use super::{
    operand::{Immediate, Path, Memory, Register},
    ident::Ident,
};

#[derive(Debug)]
pub enum Section {
    Text,
    Data,
    Bss,
    Custom(Rc<String>)
}

#[derive(Debug)]
pub enum Ast<'code> {
    Ins(Ident<'code>, Vec<Ast<'code>>),
    Label(Path<'code>),
    LabelDef(Ident<'code>, Option<Section>, bool, Option<Box<Ast<'code>>>),
    Block(Vec<Ast<'code>>, Location<'code>, bool),
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

    pub fn is_block(&self) -> bool {
        matches!(self, Ast::Block(..))
    }

    pub fn location(&self) -> Location<'code> {
        match self {
            Ast::Ins(label, _) => label.location(),
            Ast::Label(path) => path.location(),
            Ast::LabelDef(label, ..) => label.location(),
            Ast::Block(_, loc, ..) => loc.clone(),
            Ast::Macro(label, ..) => label.location(),
            Ast::Register(register) => register.location(),
            Ast::Memory(memory) => memory.location(),
            Ast::Immediate(immediate) => immediate.location(),
            Ast::String(strings) => todo!(),
        }
    }

    pub fn print_ast(&self) -> String {
        match self {
            Ast::Ins(label, asts) => {
                        // format!("{}(", label, )
                    }
            Ast::Label(path) => todo!(),
            Ast::Block(asts, ..) => todo!(),
            Ast::Macro(label, ast, labels) => todo!(),
            Ast::Register(register) => todo!(),
            Ast::Memory(memory) => todo!(),
            Ast::Immediate(immediate) => todo!(),
            Ast::LabelDef(..) => todo!(),
            Ast::String(strings) => todo!(),
        }
        todo!()
    }
}

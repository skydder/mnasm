use util::{Location, Token, TokenKind};

use crate::{Strings, WithLocation, label_block::LabelBlock};

use super::{
    ident::Ident,
    operand::{Immediate, Memory, Path, Register},
};

#[derive(Debug)]
pub enum Ast<'code> {
    Ins(WithLocation<'code, Ident>, Vec<Ast<'code>>),
    Label(WithLocation<'code, Path>),
    LabelBlock(WithLocation<'code, LabelBlock<'code>>),
    Macro(WithLocation<'code, Ident>, Vec<TokenKind>), // 1 ->
    Register(WithLocation<'code, Register>),
    Memory(WithLocation<'code, Memory<'code>>),
    Immediate(WithLocation<'code, Immediate>),
    String(WithLocation<'code, Strings>),
    EOS,
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
            Ast::EOS => todo!(),
        }
    }
}

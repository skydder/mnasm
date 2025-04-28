use std::rc::Rc;

use crate::ast::Ast;

use super::{Operand, Register};

#[derive(Debug, Clone, Copy)]
pub enum Scale {
    S1 = 1,
    S2 = 2,
    S4 = 4,
    S8 = 8,
}
#[derive(Debug, Clone)]
pub struct Memory<'code> {
    pub size: u8,
    pub base: Option<Register>,
    pub index: Option<Register>,
    pub scale: Option<Scale>,
    pub disp: Option<Rc<Ast<'code>>>,
}

impl<'code> Memory<'code> {
    pub fn new(
        size: u8,
        base: Option<Register>,
        index: Option<Register>,
        scale: Option<Scale>,
        disp: Option<Rc<Ast<'code>>>,
    ) -> Self {
        Self {
            size,
            base,
            index,
            scale,
            disp,
        }
    }
}

impl Operand for Memory<'_> {}

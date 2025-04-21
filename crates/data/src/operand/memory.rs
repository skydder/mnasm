use util::Location;

use crate::ast::Ast;

use super::{Operand, Register};

#[derive(Debug, Clone, Copy)]
pub enum Scale {
    S1 = 1,
    S2 = 2,
    S4 = 4,
    S8 = 8,
}
#[derive(Debug)]
pub struct Memory<'code> {
    location: Location<'code>,
    pub size: u8,
    pub base: Option<Register<'code>>,
    pub index: Option<Register<'code>>,
    pub scale: Option<Scale>,
    pub disp: Option<Box<Ast<'code>>>,
}

impl<'code> Memory<'code> {
    pub fn location(&self) -> Location<'code> {
        self.location.clone()
    }

    pub fn new(
        location: Location<'code>,
        size: u8,
        base: Option<Register<'code>>,
        index: Option<Register<'code>>,
        scale: Option<Scale>,
        disp: Option<Box<Ast<'code>>>,
    ) -> Self {
        Self {
            location,
            size,
            base,
            index,
            scale,
            disp,
        }
    }
}

impl Operand for Memory<'_> {}

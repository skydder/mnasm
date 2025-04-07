use util::Location;

use crate::ast::Ast;

use super::{Operand, Register};

pub struct Memory<'code> {
    location: Location<'code>,
    base: Option<Register<'code>>,
    index: Option<Box<Ast<'code>>>,
    scale: Option<Box<Ast<'code>>>,
    disp: Option<Box<Ast<'code>>>,
}

impl<'code> Memory<'code> {
    pub fn location(&self) -> Location<'code> {
        self.location.clone()
    }
}

impl Operand for Memory<'_> {}
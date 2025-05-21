mod ast;
mod ident;
mod label_block;
mod operand;
mod scope;
use std::fmt::Debug;

pub use ast::Ast;
pub use ident::Ident;
pub use label_block::{LabelBlock, Section};
pub use operand::{
    Immediate, Memory, Path, REG8, REG16, REG32, REG64, Register, RegisterKind, Scale, Strings,
};
pub use scope::Scope;
use util::Location;

#[derive(Debug, Clone)]
pub struct WithLocation<'code, T>(Location<'code>, T)
where
    T: Clone + Debug;

impl<'code, T> WithLocation<'code, T>
where
    T: Clone + Debug,
{
    pub fn new(location: Location<'code>, data: T) -> Self {
        Self(location, data)
    }

    pub fn location(&self) -> Location<'code> {
        self.0.clone()
    }

    pub fn data(&self) -> T {
        self.1.clone()
    }
}

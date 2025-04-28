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

#[derive(Debug)]
pub struct WithLocation<'code, T>
where
    T: Clone + Debug,
{
    location: Location<'code>,
    data: T,
}

impl<'code, T> WithLocation<'code, T>
where
    T: Clone + Debug,
{
    pub fn new(location: Location<'code>, data: T) -> Self {
        Self { location, data }
    }
    
    pub fn location(&self) -> Location<'code> {
        self.location.clone()
    }

    pub fn data(&self) -> T {
        self.data.clone()
    }
}

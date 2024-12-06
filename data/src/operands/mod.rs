use std::fmt::Debug;

mod immediate;
mod label;
mod memory;
mod register;
pub use immediate::Immediate;
pub use label::Label;
pub use memory::{Memory, Scale};
pub use register::{Register, RegisterKind};

pub trait Operand: Debug {
    fn codegen(&self) -> String;
    // fn kind
    fn size(&self) -> usize;
}

use std::fmt::Debug;

mod immediate;
mod label;
mod memory;
mod register;
pub use immediate::Immediate;
pub use label::{Label, LabelState};
pub use memory::{Memory, Scale};
pub use register::{Register, RegisterKind};

pub enum OperandKind {
    Register(u8, RegisterKind),
    Memory,
    Immediate(bool),
    Label, // memory
}

pub trait Operand: Debug {
    fn codegen(&self) -> String;
    fn kind(&self) -> OperandKind;
    fn size(&self) -> usize;
}

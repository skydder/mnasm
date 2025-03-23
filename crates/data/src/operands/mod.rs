use std::fmt::Debug;

mod immediate;
mod label;
mod memory;
mod register;
pub use immediate::Immediate;
pub use label::{Label, LabelState};
pub use memory::{Memory, Scale};
pub use register::{Register, RegisterKind};

pub enum OperandKind<'a> {
    Register(u8, RegisterKind),
    Memory,
    Immediate(bool),
    Label, // memory,
    NASMOperand(&'a str),
}

#[derive(Debug)]
pub struct UnimplementedOperand<'a> {
    opreand: &'a str,
}

impl<'a> UnimplementedOperand<'a> {
    pub fn new(opreand: &'a str) -> Self {
        Self { opreand }
    }
}

impl Operand for UnimplementedOperand<'_> {
    fn codegen(&self) -> String {
        // should be run after analyzed
        self.opreand.to_string()
    }

    fn size(&self) -> usize {
        64
    }

    fn kind(&self) -> super::OperandKind {
        OperandKind::Label
    }

    fn analyze(&self) {}

    fn op(&self) -> (OperandKind, usize) {
        (OperandKind::Immediate(false), 64)
    }
}

pub trait Operand: Debug {
    fn codegen(&self) -> String;
    fn analyze(&self);
    fn kind(&self) -> OperandKind;
    fn size(&self) -> usize;
    fn op(&self) -> (OperandKind, usize);
}

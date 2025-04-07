use std::fmt::Debug;

mod immediate;
mod label;
mod memory;
mod register;
pub use immediate::Immediate;
pub use label::{Label, LabelState};
pub use memory::{Memory, Scale};
pub use register::{Register, RegisterKind};

use crate::{Analyze, Codegen, Object};

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
    fn size(&self) -> usize {
        64
    }

    fn kind_op(&self) -> super::OperandKind {
        OperandKind::Label
    }

    fn op(&self) -> (OperandKind, usize) {
        (OperandKind::Immediate(false), 64)
    }
}

impl Codegen for UnimplementedOperand<'_> {
    fn codegen(&self) -> String {
        self.opreand.to_string()
    }

    fn to_code(&self) -> String {
        self.opreand.to_string()
    }
}

impl Analyze for UnimplementedOperand<'_> {
    fn analyze(&self) {}
}

impl Object for UnimplementedOperand<'_> {}

pub trait Operand: Debug + Object {
    // fn codegen(&self) -> String;
    // fn analyze(&self);
    fn kind_op(&self) -> OperandKind;
    fn size(&self) -> usize;
    fn op(&self) -> (OperandKind, usize);
}

use util::Location;

use super::{Operand, OperandKind};

#[derive(Debug)]
pub struct Immediate<'a> {
    imm: u64,
    is_signed: bool,
    size: usize,
    pub location: Location<'a>,
}

impl<'a> Immediate<'a> {
    pub fn new(imm: u64, is_signed: bool, size: usize, location: Location<'a>) -> Self {
        Self {
            imm: imm,
            is_signed: is_signed,
            size: size,
            location: location,
        }
    }
    pub fn is_signed(&self) -> bool {
        self.is_signed
    }
    pub fn abs(&self) -> u64 {
        self.imm
    }
}

impl<'a> Operand for Immediate<'a> {
    // ad-hoc one
    fn codegen(&self) -> String {
        if self.is_signed {
            format!("-{}", self.imm)
        } else {
            format!("{}", self.imm)
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn kind(&self) -> super::OperandKind {
        OperandKind::Immediate(self.is_signed)
    }

    fn analyze(&self) {}
    fn op(&self) -> (OperandKind, usize) {
        (self.kind(), self.size)
    }
}

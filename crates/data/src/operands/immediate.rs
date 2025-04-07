use util::Location;

use crate::{Analyze, Codegen, Object};

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
            imm,
            is_signed,
            size,
            location,
        }
    }
    pub fn is_signed(&self) -> bool {
        self.is_signed
    }
    pub fn abs(&self) -> u64 {
        self.imm
    }
}

impl Operand for Immediate<'_> {
    // ad-hoc one

    fn size(&self) -> usize {
        self.size
    }

    fn kind_op(&self) -> super::OperandKind {
        OperandKind::Immediate(self.is_signed)
    }

    fn op(&self) -> (OperandKind, usize) {
        (self.kind_op(), self.size)
    }
}

impl Codegen for Immediate<'_> {
    fn codegen(&self) -> String {
        if self.is_signed {
            format!("-{}", self.imm)
        } else {
            format!("{}", self.imm)
        }
    }

    fn to_code(&self) -> String {
        if self.is_signed {
            format!("-{}", self.imm)
        } else {
            format!("{}", self.imm)
        }
    }
}

impl Analyze for Immediate<'_> {
    fn analyze(&self) {}
}

impl Object for Immediate<'_> {}

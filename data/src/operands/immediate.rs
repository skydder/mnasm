use util::Location;

use super::Operand;

#[derive(Debug)]
pub struct Immediate<'a> {
    imm: u64,
    is_signed: bool,
    size: usize,
    location: Location<'a>
}

impl<'a> Immediate<'a> {
    pub fn new(imm: u64, is_signed: bool, size: usize, location: Location<'a>) -> Self {
        Self { imm: imm, is_signed: is_signed, size: size, location: location }
    }
}

impl<'a> Operand for Immediate<'a> {
    fn codegen(&self) -> String {
        todo!()
    }

    fn size(&self) -> usize {
        self.size
    }
}
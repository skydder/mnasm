use super::Operand;

#[derive(Debug, Clone, Copy)]
pub struct Immediate {
    pub data: u64,
    pub signed: bool,
}

impl Immediate {
    pub fn new(data: u64, signed: bool) -> Self {
        Self { data, signed }
    }
}

impl Operand for Immediate {}

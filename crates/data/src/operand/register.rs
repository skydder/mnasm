use util::Location;

use super::Operand;

#[allow(clippy::upper_case_acronyms)]
pub enum RegisterKind {
    GR8,
    GR16,
    GR32,
    GR64,
    X87_80,
    MMX,
    XMM,
    YMM,
    SReg,
    CReg,
    DReg,
}

pub struct Register<'code> {
    kind: RegisterKind,
    value: u8,
    size: usize,
    location: Location<'code>,
}

impl<'code> Register<'code> {
    pub fn location(&self) -> Location<'code> {
        self.location.clone()
    }

    pub fn get_reg_val(s: &str) -> Option<(RegisterKind, u8, usize)> {
        todo!()
    }

    pub fn new(
        kind: RegisterKind,
        value: u8,
        size: usize,
        location: Location<'code>,
    ) -> Self {
        Self { kind, value, size, location }
    }
}

impl Operand for Register<'_> {}
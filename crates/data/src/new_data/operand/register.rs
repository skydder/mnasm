use util::Location;

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
    pub location: Location<'code>,
}

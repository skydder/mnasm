use util::Location;

use super::Operand;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
pub struct Register<'code> {
    kind: RegisterKind,
    pub value: u8,
    pub size: usize,
    location: Location<'code>,
}

impl<'code> Register<'code> {
    pub fn location(&self) -> Location<'code> {
        self.location.clone()
    }

    pub fn get_reg_val(s: &str) -> Option<(RegisterKind, u8, usize)> {
        for (i, reg8) in REG8.iter().enumerate() {
            if s == *reg8 {
                return Some((RegisterKind::GR8, i as u8, 8));
            }
        }

        for (i, reg16) in REG16.iter().enumerate() {
            if s == *reg16 {
                return Some((RegisterKind::GR16, i as u8, 16));
            }
        }

        for (i, reg32) in REG32.iter().enumerate() {
            if s == *reg32 {
                return Some((RegisterKind::GR32, i as u8, 32));
            }
        }

        for (i, reg64) in REG64.iter().enumerate() {
            if s == *reg64 {
                return Some((RegisterKind::GR64, i as u8, 64));
            }
        }
        None
    }

    pub fn new(kind: RegisterKind, value: u8, size: usize, location: Location<'code>) -> Self {
        Self {
            kind,
            value,
            size,
            location,
        }
    }
}

impl Operand for Register<'_> {}

pub const REG8: &[&str] = &[
    "al", "cl", "dl", "bl", "ah", "ch", "dh", "bh", "r8b", "r9b", "r10b", "r11b", "r12b", "r13b",
    "r14b", "r15b", "al", "cl", "dl", "bl", "spl", "bpl", "sil", "dil",
];
pub const REG16: &[&str] = &[
    "ax", "cx", "dx", "bx", "sp", "bp", "si", "di", "r8w", "r9w", "r10w", "r11w", "r12w", "r13w",
    "r14w", "r15w",
];
pub const REG32: &[&str] = &[
    "eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi", "r8d", "r9d", "r10d", "r11d", "r12d",
    "r13d", "r14d", "r15d",
];
pub const REG64: &[&str] = &[
    "rax", "rcx", "rdx", "rbx", "rsp", "rbp", "rsi", "rdi", "r8", "r9", "r10", "r11", "r12", "r13",
    "r14", "r15",
];

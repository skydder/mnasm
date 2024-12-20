use util::Location;

use super::{Operand, OperandKind};

#[derive(Debug, Clone, Copy)]
pub enum RegisterKind {
    GR8,
    GR16,
    GR32,
    GR64,
}

#[derive(Debug)]
pub struct Register<'a> {
    kind: RegisterKind,
    value: u8,
    size: usize,
    pub location: Location<'a>,
}

impl<'a> Register<'a> {
    pub fn new(kind: RegisterKind, value: u8, size: usize, location: Location<'a>) -> Self {
        Self {
            kind: kind,
            value: value,
            size: size,
            location: location,
        }
    }

    pub fn is_reg(s: &str) -> Option<(RegisterKind, u8, usize)> {
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
}

const REG8: &[&'static str] = &[
    "al", "cl", "dl", "bl", "ah", "ch", "dh", "bh", "r8b", "r9b", "r10b", "r11b", "r12b", "r13b",
    "r14b", "r15b", "al", "cl", "dl", "bl", "spl", "bpl", "sil", "dil",
];
const REG16: &[&'static str] = &[
    "ax", "cx", "dx", "bx", "sp", "bp", "si", "di", "r8w", "r9w", "r10w", "r11w", "r12w", "r13w",
    "r14w", "r15w",
];
const REG32: &[&'static str] = &[
    "eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi", "r8d", "r9d", "r10d", "r11d", "r12d",
    "r13d", "r14d", "r15d",
];
const REG64: &[&'static str] = &[
    "rax", "rcx", "rdx", "rbx", "rsp", "rbp", "rsi", "rdi", "r8", "r9", "r10", "r11", "r12", "r13",
    "r14", "r15",
];

impl<'a> std::fmt::Display for Register<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            RegisterKind::GR8 => {
                write!(f, "{}", REG8[self.value as usize])
            }
            RegisterKind::GR16 => {
                write!(f, "{}", REG16[self.value as usize])
            }
            RegisterKind::GR32 => {
                write!(f, "{}", REG32[self.value as usize])
            }
            RegisterKind::GR64 => {
                write!(f, "{}", REG64[self.value as usize])
            }
        }
    }
}

impl<'a> Operand for Register<'a> {
    fn codegen(&self) -> String {
        format!("{}", self)
    }
    fn size(&self) -> usize {
        self.size
    }

    fn kind(&self) -> OperandKind {
        OperandKind::Register(self.value, self.kind)
    }

    fn analyze(&self) {}
}

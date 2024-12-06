use util::{emit_error, Location};

use super::{Immediate, Operand, Register};

#[derive(Debug)]

pub enum Scale {
    S1,
    S2,
    S4,
    S8,
}

impl Scale {
    fn value(&self) -> u8 {
        match self {
            Scale::S1 => 1,
            Scale::S2 => 2,
            Scale::S4 => 4,
            Scale::S8 => 8,
        }
    }
}

#[derive(Debug)]

enum MemoryConstituents<'a> {
    D(Immediate<'a>),
    B(Register<'a>),
    BI(Register<'a>, Register<'a>),
    BD(Register<'a>, Immediate<'a>),
    BID(Register<'a>, Register<'a>, Immediate<'a>),
    BIS(Register<'a>, Register<'a>, Scale),
    ISD(Register<'a>, Scale, Immediate<'a>),
    BISD(Register<'a>, Register<'a>, Scale, Immediate<'a>),
}

impl<'a> MemoryConstituents<'a> {
    fn new(
        base: Option<Register<'a>>,
        index: Option<Register<'a>>,
        scale: Option<Scale>,
        disp: Option<Immediate<'a>>,
    ) -> Self {
        match (base, index, scale, disp) {
            (None, None, None, Some(d)) => Self::D(d),
            (Some(b), None, None, None) => Self::B(b),
            (Some(b), Some(i), None, None) => Self::BI(b, i),
            (Some(b), None, None, Some(d)) => Self::BD(b, d),
            (Some(b), Some(i), None, Some(d)) => Self::BID(b, i, d),
            (Some(b), Some(i), Some(s), None) => Self::BIS(b, i, s),
            (None, Some(i), Some(s), Some(d)) => Self::ISD(i, s, d),
            (Some(b), Some(i), Some(s), Some(d)) => Self::BISD(b, i, s, d),
            _ => todo!("considering"),
        }
    }
}

impl<'a> std::fmt::Display for MemoryConstituents<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryConstituents::D(disp) => write!(f, "[{}]", disp.codegen()),
            MemoryConstituents::B(base) => write!(f, "[{}]", base.codegen()),
            MemoryConstituents::BI(base, idx) => {
                write!(f, "[{} + {}]", base.codegen(), idx.codegen())
            }
            MemoryConstituents::BD(base, disp) => {
                if disp.is_signed() {
                    write!(f, "[{} - {}]", base.codegen(), disp.abs())
                } else {
                    write!(f, "[{} + {}]", base.codegen(), disp.codegen())
                }
            }
            MemoryConstituents::BID(base, idx, disp) => {
                if disp.is_signed() {
                    write!(
                        f,
                        "[{} + {} - {}]",
                        base.codegen(),
                        idx.codegen(),
                        disp.abs()
                    )
                } else {
                    write!(
                        f,
                        "[{} + {} + {}]",
                        base.codegen(),
                        idx.codegen(),
                        disp.codegen()
                    )
                }
            }
            MemoryConstituents::BIS(base, idx, scale) => write!(
                f,
                "[{} + {} * {}]",
                base.codegen(),
                idx.codegen(),
                scale.value()
            ),
            MemoryConstituents::ISD(idx, scale, disp) => {
                if disp.is_signed() {
                    write!(
                        f,
                        "[{} * {} - {}]",
                        idx.codegen(),
                        scale.value(),
                        disp.abs()
                    )
                } else {
                    write!(
                        f,
                        "[{} * {} + {}]",
                        idx.codegen(),
                        scale.value(),
                        disp.codegen()
                    )
                }
            }
            MemoryConstituents::BISD(base, idx, scale, disp) => {
                if disp.is_signed() {
                    write!(
                        f,
                        "[{} + {} * {} - {}]",
                        base.codegen(),
                        idx.codegen(),
                        scale.value(),
                        disp.abs()
                    )
                } else {
                    write!(
                        f,
                        "[{} + {} * {} + {}]",
                        base.codegen(),
                        idx.codegen(),
                        scale.value(),
                        disp.codegen()
                    )
                }
            }
        }
    }
}

// ptr<dword>(base, index, scale, displacement)
#[derive(Debug)]
pub struct Memory<'a> {
    constituents: MemoryConstituents<'a>,
    size: usize,
    pub location: Location<'a>,
}

impl<'a> Memory<'a> {
    pub fn new(
        args: (
            Option<Register<'a>>,
            Option<Register<'a>>,
            Option<Scale>,
            Option<Immediate<'a>>,
        ),
        size: usize,
        location: Location<'a>,
    ) -> Self {
        Self {
            constituents: MemoryConstituents::new(args.0, args.1, args.2, args.3),
            size: size,
            location: location,
        }
    }
}

impl<'a> Operand for Memory<'a> {
    fn codegen(&self) -> String {
        match self.size {
            0 => format!("{}", self.constituents),
            8 => format!("byte {}", self.constituents),
            16 => format!("word {}", self.constituents),
            32 => format!("dword {}", self.constituents),
            64 => format!("qword {}", self.constituents),
            _ => {
                emit_error!(self.location, "unexpected size")
            }
        }
    }

    fn size(&self) -> usize {
        self.size
    }
}

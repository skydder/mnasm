use super::analyze::Operands;
use crate::{OperandKind, RegisterKind};

#[allow(warnings)]
pub fn ins_analyzer(ins_name: &str, operands: Operands) -> Result<(), ()> {
    match (ins_name, &operands) {
        (
            "adc",
            Operands(
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "adc",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "adc",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "adc",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "adc",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "adc",
            Operands(
                Some((OperandKind::Register(0, _), 64)),
                Some((OperandKind::Immediate(true), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "adc",
            Operands(
                Some((OperandKind::Register(0, _), 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "adc",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(true), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "adc",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "add",
            Operands(
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "add",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "add",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "add",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "add",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "add",
            Operands(
                Some((OperandKind::Register(0, _), 64)),
                Some((OperandKind::Immediate(true), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "add",
            Operands(
                Some((OperandKind::Register(0, _), 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "add",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(true), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "add",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "and",
            Operands(
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "and",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "and",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "and",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "and",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "and",
            Operands(
                Some((OperandKind::Register(0, _), 64)),
                Some((OperandKind::Immediate(true), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "and",
            Operands(
                Some((OperandKind::Register(0, _), 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "and",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(true), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "and",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "bsf",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "bsf",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "bsr",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "bsr",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        ("bswap", Operands(Some((OperandKind::Register(_, _), 64)), None, None, None)) => Ok(()),
        (
            "bt",
            Operands(
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "bt",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "bt",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "btc",
            Operands(
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "btc",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "btc",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "btr",
            Operands(
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "btr",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "btr",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "bts",
            Operands(
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "bts",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "bts",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        ("call", Operands(Some((OperandKind::Immediate(false), 64)), None, None, None)) => Ok(()),
        ("call", Operands(Some((OperandKind::Immediate(false), _)), None, None, None)) => Ok(()),
        ("call", Operands(Some((OperandKind::Memory, _)), None, None, None)) => Ok(()),
        ("call", Operands(Some((OperandKind::Memory, _)), None, None, None)) => Ok(()),
        (
            "call",
            Operands(
                Some((OperandKind::Register(_, _), _) | (OperandKind::Memory, _)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "call",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        ("cdqe", Operands(None, None, None, None)) => Ok(()),
        (
            "cmp",
            Operands(
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "cmp",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "cmp",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "cmp",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "cmp",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "cmp",
            Operands(
                Some((OperandKind::Register(0, _), 64)),
                Some((OperandKind::Immediate(true), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "cmp",
            Operands(
                Some((OperandKind::Register(0, _), 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "cmp",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(true), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "cmp",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
                None,
            ),
        ) => Ok(()),
        ("cmpsq", Operands(None, None, None, None)) => Ok(()),
        (
            "cmpxchg",
            Operands(
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "cmpxchg",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        ("cmpxchg16b", Operands(Some((OperandKind::Memory, 128)), None, None, None)) => Ok(()),
        ("cqo", Operands(None, None, None, None)) => Ok(()),
        (
            "dec",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "div",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "idiv",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "imul",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "imul",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "imul",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "imul",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Immediate(false), 8)),
                None,
            ),
        ) => Ok(()),
        (
            "imul",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Immediate(true), _)),
                None,
            ),
        ) => Ok(()),
        (
            "imul",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Immediate(false), 32)),
                None,
            ),
        ) => Ok(()),
        (
            "imul",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Immediate(false), _)),
                None,
            ),
        ) => Ok(()),
        (
            "imul",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Immediate(false), 8)),
                None,
            ),
        ) => Ok(()),
        (
            "imul",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Immediate(true), _)),
                None,
            ),
        ) => Ok(()),
        (
            "imul",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Immediate(false), 32)),
                None,
            ),
        ) => Ok(()),
        (
            "imul",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
            ),
        ) => Ok(()),
        (
            "imul",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Immediate(false), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "imul",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Immediate(true), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "imul",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Immediate(false), 32)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "imul",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "inc",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        ("iretq", Operands(None, None, None, None)) => Ok(()),
        ("jrcxz", Operands(Some((OperandKind::Immediate(false), _)), None, None, None)) => Ok(()),
        ("jmp", Operands(Some((OperandKind::Immediate(false), 64)), None, None, None)) => Ok(()),
        ("jmp", Operands(Some((OperandKind::Immediate(false), _)), None, None, None)) => Ok(()),
        ("jmp", Operands(Some((OperandKind::Memory, _)), None, None, None)) => Ok(()),
        ("jmp", Operands(Some((OperandKind::Memory, _)), None, None, None)) => Ok(()),
        (
            "jmp",
            Operands(
                Some((OperandKind::Register(_, _), _) | (OperandKind::Memory, _)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "jmp",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "lar",
            Operands(
                Some((OperandKind::Register(_, _), 16)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "lar",
            Operands(
                Some((OperandKind::Register(_, _), 32)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "lar",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "lar",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 16)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "lar",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 32)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "lar",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "lea",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "lea",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "lfs",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "lgs",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        ("lodsq", Operands(None, None, None, None)) => Ok(()),
        (
            "loop",
            Operands(
                Some((OperandKind::Immediate(false), _)),
                Some((OperandKind::Register(1, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "loope",
            Operands(
                Some((OperandKind::Immediate(false), _)),
                Some((OperandKind::Register(1, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "loopne",
            Operands(
                Some((OperandKind::Immediate(false), _)),
                Some((OperandKind::Register(1, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "loopnz",
            Operands(
                Some((OperandKind::Immediate(false), _)),
                Some((OperandKind::Register(1, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "loopz",
            Operands(
                Some((OperandKind::Immediate(false), _)),
                Some((OperandKind::Register(1, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "lsl",
            Operands(
                Some((OperandKind::Register(_, _), 16)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "lsl",
            Operands(
                Some((OperandKind::Register(_, _), 32)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "lsl",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "lsl",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 16)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "lsl",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 32)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "lsl",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "lss",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "monitor",
            Operands(
                Some((OperandKind::Register(0, _), 64)),
                Some((OperandKind::Register(1, _), 32)),
                Some((OperandKind::Register(2, _), 32)),
                None,
            ),
        ) => Ok(()),
        (
            "mov",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, RegisterKind::SReg), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "mov",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Register(_, RegisterKind::SReg), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "mov",
            Operands(
                Some((OperandKind::Register(_, RegisterKind::SReg), _)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "mov",
            Operands(
                Some((OperandKind::Register(_, RegisterKind::SReg), _)),
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "mov",
            Operands(
                Some((OperandKind::Register(0, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "mov",
            Operands(
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Register(0, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "mov",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, RegisterKind::CReg), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "mov",
            Operands(
                Some((OperandKind::Register(_, RegisterKind::CReg), _)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "mov",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, RegisterKind::DReg), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "mov",
            Operands(
                Some((OperandKind::Register(_, RegisterKind::DReg), _)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "mov",
            Operands(
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "mov",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "mov",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "mov",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "mov",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "mov",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "mov",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), 32)),
                None,
                None,
            ),
        ) => Ok(()),
        ("movsq", Operands(None, None, None, None)) => Ok(()),
        (
            "movsx",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 8) | (OperandKind::Memory, 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "movsx",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 16) | (OperandKind::Memory, 16)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "movsxd",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 32) | (OperandKind::Memory, 32)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "movsx",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 32) | (OperandKind::Memory, 32)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "movzx",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 8) | (OperandKind::Memory, 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "movzx",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 16) | (OperandKind::Memory, 16)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "mul",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "neg",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "nop",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "not",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "or",
            Operands(
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "or",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "or",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "or",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "or",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "or",
            Operands(
                Some((OperandKind::Register(0, _), 64)),
                Some((OperandKind::Immediate(true), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "or",
            Operands(
                Some((OperandKind::Register(0, _), 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "or",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(true), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "or",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
                None,
            ),
        ) => Ok(()),
        ("pop", Operands(Some((OperandKind::Register(_, _), 64)), None, None, None)) => Ok(()),
        (
            "pop",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        ("popfq", Operands(None, None, None, None)) => Ok(()),
        ("push", Operands(Some((OperandKind::Register(_, _), 64)), None, None, None)) => Ok(()),
        (
            "push",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        ("push", Operands(Some((OperandKind::Immediate(true), _)), None, None, None)) => Ok(()),
        ("push", Operands(Some((OperandKind::Immediate(false), 64)), None, None, None)) => Ok(()),
        ("push", Operands(Some((OperandKind::Immediate(true), _)), None, None, None)) => Ok(()),
        ("push", Operands(Some((OperandKind::Immediate(false), 32)), None, None, None)) => Ok(()),
        ("pushfq", Operands(None, None, None, None)) => Ok(()),
        (
            "rcl",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Register(1, _), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "rcl",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "rcr",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Register(1, _), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "rcr",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        ("rdtscp", Operands(None, None, None, None)) => Ok(()),
        ("retq", Operands(None, None, None, None)) => Ok(()),
        ("retq", Operands(Some((OperandKind::Immediate(false), _)), None, None, None)) => Ok(()),
        ("retfq", Operands(None, None, None, None)) => Ok(()),
        ("retfq", Operands(Some((OperandKind::Immediate(false), _)), None, None, None)) => Ok(()),
        ("retnq", Operands(None, None, None, None)) => Ok(()),
        ("retnq", Operands(Some((OperandKind::Immediate(false), _)), None, None, None)) => Ok(()),
        (
            "rol",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Register(1, _), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "rol",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "ror",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Register(1, _), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "ror",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "sal",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Register(1, _), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "sal",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "sar",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Register(1, _), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "sar",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "sbb",
            Operands(
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "sbb",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "sbb",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "sbb",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "sbb",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "sbb",
            Operands(
                Some((OperandKind::Register(0, _), 64)),
                Some((OperandKind::Immediate(true), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "sbb",
            Operands(
                Some((OperandKind::Register(0, _), 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "sbb",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(true), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "sbb",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
                None,
            ),
        ) => Ok(()),
        ("scasq", Operands(None, None, None, None)) => Ok(()),
        (
            "shl",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Register(1, _), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "shl",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "shld",
            Operands(
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
            ),
        ) => Ok(()),
        (
            "shld",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
            ),
        ) => Ok(()),
        (
            "shld",
            Operands(
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(1, _), 8)),
                None,
            ),
        ) => Ok(()),
        (
            "shld",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(1, _), 8)),
                None,
            ),
        ) => Ok(()),
        (
            "shr",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Register(1, _), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "shr",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "shrd",
            Operands(
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
            ),
        ) => Ok(()),
        (
            "shrd",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
            ),
        ) => Ok(()),
        (
            "shrd",
            Operands(
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(1, _), 8)),
                None,
            ),
        ) => Ok(()),
        (
            "shrd",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(1, _), 8)),
                None,
            ),
        ) => Ok(()),
        ("sldt", Operands(Some((OperandKind::Register(_, _), 64)), None, None, None)) => Ok(()),
        ("sldt", Operands(Some((OperandKind::Register(_, _), 64)), None, None, None)) => Ok(()),
        ("skinit", Operands(None, None, None, None)) => Ok(()),
        ("smsw", Operands(Some((OperandKind::Register(_, _), 64)), None, None, None)) => Ok(()),
        ("stosq", Operands(None, None, None, None)) => Ok(()),
        ("str", Operands(Some((OperandKind::Register(_, _), 64)), None, None, None)) => Ok(()),
        (
            "sub",
            Operands(
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "sub",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "sub",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "sub",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "sub",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "sub",
            Operands(
                Some((OperandKind::Register(0, _), 64)),
                Some((OperandKind::Immediate(true), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "sub",
            Operands(
                Some((OperandKind::Register(0, _), 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "sub",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(true), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "sub",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
                None,
            ),
        ) => Ok(()),
        ("swapgs", Operands(None, None, None, None)) => Ok(()),
        (
            "test",
            Operands(
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "test",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "test",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "test",
            Operands(
                Some((OperandKind::Register(0, _), 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "test",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "xadd",
            Operands(
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "xadd",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "xchg",
            Operands(
                Some((OperandKind::Register(0, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "xchg",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(0, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "xchg",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "xchg",
            Operands(
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "xchg",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "xor",
            Operands(
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "xor",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "xor",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "xor",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "xor",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "xor",
            Operands(
                Some((OperandKind::Register(0, _), 64)),
                Some((OperandKind::Immediate(true), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "xor",
            Operands(
                Some((OperandKind::Register(0, _), 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "xor",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(true), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "xor",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Immediate(false), _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "cmovcc",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "cmovcc",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        ("jcc", Operands(Some((OperandKind::Immediate(false), _)), None, None, None)) => Ok(()),
        (
            "cvtss2si",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        ("fxrstor64", Operands(Some((OperandKind::Memory, _)), None, None, None)) => Ok(()),
        ("fxsave64", Operands(Some((OperandKind::Memory, _)), None, None, None)) => Ok(()),
        (
            "movnti",
            Operands(
                Some((OperandKind::Memory, _)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "cvtsd2si",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "cvttsd2si",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Memory, _)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "vmread",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                Some((OperandKind::Register(_, _), 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "vmwrite",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "crc32",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 8) | (OperandKind::Memory, 8)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "crc32",
            Operands(
                Some((OperandKind::Register(_, _), 64)),
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop0",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop1",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop2",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop3",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop4",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop5",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop6",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop7",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop8",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop9",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop10",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop11",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop12",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop13",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop14",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop15",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop16",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop17",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop18",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop19",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop20",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop21",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop22",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop23",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop24",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop25",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop26",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop27",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop28",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop29",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop30",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop31",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop32",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop33",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop34",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop35",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop36",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop37",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop38",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop39",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop40",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop41",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop42",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop43",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop44",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop45",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop46",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop47",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop48",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop49",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop50",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop51",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop52",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop53",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop54",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop55",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop56",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop57",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop58",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop59",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop60",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop61",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop62",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        (
            "hint_nop63",
            Operands(
                Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)),
                None,
                None,
                None,
            ),
        ) => Ok(()),
        _ => Err(()),
    }
}

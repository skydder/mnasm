use crate::{OperandKind, RegisterKind};
use super::analyze::Operands;

#[allow(warnings)]
pub fn ins_analyzer(ins_name: &str, operands: Operands) -> Result<(), ()> {
	match (ins_name, &operands) {
		("adc", Operands(Some((OperandKind::Memory, _)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("adc", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("adc", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("adc", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("adc", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), 8)), None, None)) => {
			return Ok(());
		},
		("adc", Operands(Some((OperandKind::Register(0, _), 64)), Some((OperandKind::Immediate(true), _)), None, None)) => {
			return Ok(());
		},
		("adc", Operands(Some((OperandKind::Register(0, _), 64)), Some((OperandKind::Immediate(false), _)), None, None)) => {
			return Ok(());
		},
		("adc", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(true), _)), None, None)) => {
			return Ok(());
		},
		("adc", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), _)), None, None)) => {
			return Ok(());
		},
		("add", Operands(Some((OperandKind::Memory, _)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("add", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("add", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("add", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("add", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), 8)), None, None)) => {
			return Ok(());
		},
		("add", Operands(Some((OperandKind::Register(0, _), 64)), Some((OperandKind::Immediate(true), _)), None, None)) => {
			return Ok(());
		},
		("add", Operands(Some((OperandKind::Register(0, _), 64)), Some((OperandKind::Immediate(false), _)), None, None)) => {
			return Ok(());
		},
		("add", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(true), _)), None, None)) => {
			return Ok(());
		},
		("add", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), _)), None, None)) => {
			return Ok(());
		},
		("and", Operands(Some((OperandKind::Memory, _)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("and", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("and", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("and", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("and", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), 8)), None, None)) => {
			return Ok(());
		},
		("and", Operands(Some((OperandKind::Register(0, _), 64)), Some((OperandKind::Immediate(true), _)), None, None)) => {
			return Ok(());
		},
		("and", Operands(Some((OperandKind::Register(0, _), 64)), Some((OperandKind::Immediate(false), _)), None, None)) => {
			return Ok(());
		},
		("and", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(true), _)), None, None)) => {
			return Ok(());
		},
		("and", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), _)), None, None)) => {
			return Ok(());
		},
		("bsf", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("bsf", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("bsr", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("bsr", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("bswap", Operands(Some((OperandKind::Register(_, _), 64)), None, None, None)) => {
			return Ok(());
		},
		("bt", Operands(Some((OperandKind::Memory, _)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("bt", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("bt", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), 8)), None, None)) => {
			return Ok(());
		},
		("btc", Operands(Some((OperandKind::Memory, _)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("btc", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("btc", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), 8)), None, None)) => {
			return Ok(());
		},
		("btr", Operands(Some((OperandKind::Memory, _)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("btr", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("btr", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), 8)), None, None)) => {
			return Ok(());
		},
		("bts", Operands(Some((OperandKind::Memory, _)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("bts", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("bts", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), 8)), None, None)) => {
			return Ok(());
		},
		("call", Operands(Some((OperandKind::Immediate(false), 64)), None, None, None)) => {
			return Ok(());
		},
		("call", Operands(Some((OperandKind::Immediate(false), _)), None, None, None)) => {
			return Ok(());
		},
		("call", Operands(Some((OperandKind::Memory, _)), None, None, None)) => {
			return Ok(());
		},
		("call", Operands(Some((OperandKind::Memory, _)), None, None, None)) => {
			return Ok(());
		},
		("call", Operands(Some((OperandKind::Register(_, _), _) | (OperandKind::Memory, _)), None, None, None)) => {
			return Ok(());
		},
		("call", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("cdqe", Operands(None, None, None, None)) => {
			return Ok(());
		},
		("cmp", Operands(Some((OperandKind::Memory, _)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("cmp", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("cmp", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("cmp", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("cmp", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), 8)), None, None)) => {
			return Ok(());
		},
		("cmp", Operands(Some((OperandKind::Register(0, _), 64)), Some((OperandKind::Immediate(true), _)), None, None)) => {
			return Ok(());
		},
		("cmp", Operands(Some((OperandKind::Register(0, _), 64)), Some((OperandKind::Immediate(false), _)), None, None)) => {
			return Ok(());
		},
		("cmp", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(true), _)), None, None)) => {
			return Ok(());
		},
		("cmp", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), _)), None, None)) => {
			return Ok(());
		},
		("cmpsq", Operands(None, None, None, None)) => {
			return Ok(());
		},
		("cmpxchg", Operands(Some((OperandKind::Memory, _)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("cmpxchg", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("cmpxchg16b", Operands(Some((OperandKind::Memory, 128)), None, None, None)) => {
			return Ok(());
		},
		("cqo", Operands(None, None, None, None)) => {
			return Ok(());
		},
		("dec", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("div", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("idiv", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("imul", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("imul", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("imul", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("imul", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), Some((OperandKind::Immediate(false), 8)), None)) => {
			return Ok(());
		},
		("imul", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), Some((OperandKind::Immediate(true), _)), None)) => {
			return Ok(());
		},
		("imul", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), Some((OperandKind::Immediate(false), 32)), None)) => {
			return Ok(());
		},
		("imul", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), Some((OperandKind::Immediate(false), _)), None)) => {
			return Ok(());
		},
		("imul", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Immediate(false), 8)), None)) => {
			return Ok(());
		},
		("imul", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Immediate(true), _)), None)) => {
			return Ok(());
		},
		("imul", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Immediate(false), 32)), None)) => {
			return Ok(());
		},
		("imul", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Immediate(false), _)), None)) => {
			return Ok(());
		},
		("imul", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Immediate(false), 8)), None, None)) => {
			return Ok(());
		},
		("imul", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Immediate(true), _)), None, None)) => {
			return Ok(());
		},
		("imul", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Immediate(false), 32)), None, None)) => {
			return Ok(());
		},
		("imul", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Immediate(false), _)), None, None)) => {
			return Ok(());
		},
		("inc", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("iretq", Operands(None, None, None, None)) => {
			return Ok(());
		},
		("jrcxz", Operands(Some((OperandKind::Immediate(false), _)), None, None, None)) => {
			return Ok(());
		},
		("jmp", Operands(Some((OperandKind::Immediate(false), 64)), None, None, None)) => {
			return Ok(());
		},
		("jmp", Operands(Some((OperandKind::Immediate(false), _)), None, None, None)) => {
			return Ok(());
		},
		("jmp", Operands(Some((OperandKind::Memory, _)), None, None, None)) => {
			return Ok(());
		},
		("jmp", Operands(Some((OperandKind::Memory, _)), None, None, None)) => {
			return Ok(());
		},
		("jmp", Operands(Some((OperandKind::Register(_, _), _) | (OperandKind::Memory, _)), None, None, None)) => {
			return Ok(());
		},
		("jmp", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("lar", Operands(Some((OperandKind::Register(_, _), 16)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("lar", Operands(Some((OperandKind::Register(_, _), 32)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("lar", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("lar", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 16)), None, None)) => {
			return Ok(());
		},
		("lar", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 32)), None, None)) => {
			return Ok(());
		},
		("lar", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("lea", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("lea", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Immediate(false), _)), None, None)) => {
			return Ok(());
		},
		("lfs", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("lgs", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("lodsq", Operands(None, None, None, None)) => {
			return Ok(());
		},
		("loop", Operands(Some((OperandKind::Immediate(false), _)), Some((OperandKind::Register(1, _), 64)), None, None)) => {
			return Ok(());
		},
		("loope", Operands(Some((OperandKind::Immediate(false), _)), Some((OperandKind::Register(1, _), 64)), None, None)) => {
			return Ok(());
		},
		("loopne", Operands(Some((OperandKind::Immediate(false), _)), Some((OperandKind::Register(1, _), 64)), None, None)) => {
			return Ok(());
		},
		("loopnz", Operands(Some((OperandKind::Immediate(false), _)), Some((OperandKind::Register(1, _), 64)), None, None)) => {
			return Ok(());
		},
		("loopz", Operands(Some((OperandKind::Immediate(false), _)), Some((OperandKind::Register(1, _), 64)), None, None)) => {
			return Ok(());
		},
		("lsl", Operands(Some((OperandKind::Register(_, _), 16)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("lsl", Operands(Some((OperandKind::Register(_, _), 32)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("lsl", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("lsl", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 16)), None, None)) => {
			return Ok(());
		},
		("lsl", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 32)), None, None)) => {
			return Ok(());
		},
		("lsl", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("lss", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("monitor", Operands(Some((OperandKind::Register(0, _), 64)), Some((OperandKind::Register(1, _), 32)), Some((OperandKind::Register(2, _), 32)), None)) => {
			return Ok(());
		},
		("mov", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, RegisterKind::SReg), _)), None, None)) => {
			return Ok(());
		},
		("mov", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Register(_, RegisterKind::SReg), _)), None, None)) => {
			return Ok(());
		},
		("mov", Operands(Some((OperandKind::Register(_, RegisterKind::SReg), _)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("mov", Operands(Some((OperandKind::Register(_, RegisterKind::SReg), _)), Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None)) => {
			return Ok(());
		},
		("mov", Operands(Some((OperandKind::Register(0, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("mov", Operands(Some((OperandKind::Memory, _)), Some((OperandKind::Register(0, _), 64)), None, None)) => {
			return Ok(());
		},
		("mov", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, RegisterKind::CReg), _)), None, None)) => {
			return Ok(());
		},
		("mov", Operands(Some((OperandKind::Register(_, RegisterKind::CReg), _)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("mov", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, RegisterKind::DReg), _)), None, None)) => {
			return Ok(());
		},
		("mov", Operands(Some((OperandKind::Register(_, RegisterKind::DReg), _)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("mov", Operands(Some((OperandKind::Memory, _)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("mov", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("mov", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("mov", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("mov", Operands(Some((OperandKind::Register(_, _), 64)), Some(None), None, None)) => {
			return Ok(());
		},
		("mov", Operands(Some((OperandKind::Register(_, _), 64)), Some(None), None, None)) => {
			return Ok(());
		},
		("mov", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Immediate(false), _)), None, None)) => {
			return Ok(());
		},
		("mov", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), _)), None, None)) => {
			return Ok(());
		},
		("mov", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), 32)), None, None)) => {
			return Ok(());
		},
		("movsq", Operands(None, None, None, None)) => {
			return Ok(());
		},
		("movsx", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 8) | (OperandKind::Memory, 8)), None, None)) => {
			return Ok(());
		},
		("movsx", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 16) | (OperandKind::Memory, 16)), None, None)) => {
			return Ok(());
		},
		("movsxd", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 32) | (OperandKind::Memory, 32)), None, None)) => {
			return Ok(());
		},
		("movsx", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 32) | (OperandKind::Memory, 32)), None, None)) => {
			return Ok(());
		},
		("movzx", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 8) | (OperandKind::Memory, 8)), None, None)) => {
			return Ok(());
		},
		("movzx", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 16) | (OperandKind::Memory, 16)), None, None)) => {
			return Ok(());
		},
		("mul", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("neg", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("nop", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("not", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("or", Operands(Some((OperandKind::Memory, _)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("or", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("or", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("or", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("or", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), 8)), None, None)) => {
			return Ok(());
		},
		("or", Operands(Some((OperandKind::Register(0, _), 64)), Some((OperandKind::Immediate(true), _)), None, None)) => {
			return Ok(());
		},
		("or", Operands(Some((OperandKind::Register(0, _), 64)), Some((OperandKind::Immediate(false), _)), None, None)) => {
			return Ok(());
		},
		("or", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(true), _)), None, None)) => {
			return Ok(());
		},
		("or", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), _)), None, None)) => {
			return Ok(());
		},
		("pop", Operands(Some((OperandKind::Register(_, _), 64)), None, None, None)) => {
			return Ok(());
		},
		("pop", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("popfq", Operands(None, None, None, None)) => {
			return Ok(());
		},
		("push", Operands(Some((OperandKind::Register(_, _), 64)), None, None, None)) => {
			return Ok(());
		},
		("push", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("push", Operands(Some((OperandKind::Immediate(true), _)), None, None, None)) => {
			return Ok(());
		},
		("push", Operands(Some((OperandKind::Immediate(false), 64)), None, None, None)) => {
			return Ok(());
		},
		("push", Operands(Some((OperandKind::Immediate(true), _)), None, None, None)) => {
			return Ok(());
		},
		("push", Operands(Some((OperandKind::Immediate(false), 32)), None, None, None)) => {
			return Ok(());
		},
		("pushfq", Operands(None, None, None, None)) => {
			return Ok(());
		},
		("rcl", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Register(1, _), 8)), None, None)) => {
			return Ok(());
		},
		("rcl", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), 8)), None, None)) => {
			return Ok(());
		},
		("rcr", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Register(1, _), 8)), None, None)) => {
			return Ok(());
		},
		("rcr", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), 8)), None, None)) => {
			return Ok(());
		},
		("rdtscp", Operands(None, None, None, None)) => {
			return Ok(());
		},
		("retq", Operands(None, None, None, None)) => {
			return Ok(());
		},
		("retq", Operands(Some((OperandKind::Immediate(false), _)), None, None, None)) => {
			return Ok(());
		},
		("retfq", Operands(None, None, None, None)) => {
			return Ok(());
		},
		("retfq", Operands(Some((OperandKind::Immediate(false), _)), None, None, None)) => {
			return Ok(());
		},
		("retnq", Operands(None, None, None, None)) => {
			return Ok(());
		},
		("retnq", Operands(Some((OperandKind::Immediate(false), _)), None, None, None)) => {
			return Ok(());
		},
		("rol", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Register(1, _), 8)), None, None)) => {
			return Ok(());
		},
		("rol", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), 8)), None, None)) => {
			return Ok(());
		},
		("ror", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Register(1, _), 8)), None, None)) => {
			return Ok(());
		},
		("ror", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), 8)), None, None)) => {
			return Ok(());
		},
		("sal", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Register(1, _), 8)), None, None)) => {
			return Ok(());
		},
		("sal", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), 8)), None, None)) => {
			return Ok(());
		},
		("sar", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Register(1, _), 8)), None, None)) => {
			return Ok(());
		},
		("sar", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), 8)), None, None)) => {
			return Ok(());
		},
		("sbb", Operands(Some((OperandKind::Memory, _)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("sbb", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("sbb", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("sbb", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("sbb", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), 8)), None, None)) => {
			return Ok(());
		},
		("sbb", Operands(Some((OperandKind::Register(0, _), 64)), Some((OperandKind::Immediate(true), _)), None, None)) => {
			return Ok(());
		},
		("sbb", Operands(Some((OperandKind::Register(0, _), 64)), Some((OperandKind::Immediate(false), _)), None, None)) => {
			return Ok(());
		},
		("sbb", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(true), _)), None, None)) => {
			return Ok(());
		},
		("sbb", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), _)), None, None)) => {
			return Ok(());
		},
		("scasq", Operands(None, None, None, None)) => {
			return Ok(());
		},
		("shl", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Register(1, _), 8)), None, None)) => {
			return Ok(());
		},
		("shl", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), 8)), None, None)) => {
			return Ok(());
		},
		("shld", Operands(Some((OperandKind::Memory, _)), Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Immediate(false), _)), None)) => {
			return Ok(());
		},
		("shld", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Immediate(false), _)), None)) => {
			return Ok(());
		},
		("shld", Operands(Some((OperandKind::Memory, _)), Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(1, _), 8)), None)) => {
			return Ok(());
		},
		("shld", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(1, _), 8)), None)) => {
			return Ok(());
		},
		("shr", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Register(1, _), 8)), None, None)) => {
			return Ok(());
		},
		("shr", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), 8)), None, None)) => {
			return Ok(());
		},
		("shrd", Operands(Some((OperandKind::Memory, _)), Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Immediate(false), _)), None)) => {
			return Ok(());
		},
		("shrd", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Immediate(false), _)), None)) => {
			return Ok(());
		},
		("shrd", Operands(Some((OperandKind::Memory, _)), Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(1, _), 8)), None)) => {
			return Ok(());
		},
		("shrd", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(1, _), 8)), None)) => {
			return Ok(());
		},
		("sldt", Operands(Some((OperandKind::Register(_, _), 64)), None, None, None)) => {
			return Ok(());
		},
		("sldt", Operands(Some((OperandKind::Register(_, _), 64)), None, None, None)) => {
			return Ok(());
		},
		("skinit", Operands(None, None, None, None)) => {
			return Ok(());
		},
		("smsw", Operands(Some((OperandKind::Register(_, _), 64)), None, None, None)) => {
			return Ok(());
		},
		("stosq", Operands(None, None, None, None)) => {
			return Ok(());
		},
		("str", Operands(Some((OperandKind::Register(_, _), 64)), None, None, None)) => {
			return Ok(());
		},
		("sub", Operands(Some((OperandKind::Memory, _)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("sub", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("sub", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("sub", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("sub", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), 8)), None, None)) => {
			return Ok(());
		},
		("sub", Operands(Some((OperandKind::Register(0, _), 64)), Some((OperandKind::Immediate(true), _)), None, None)) => {
			return Ok(());
		},
		("sub", Operands(Some((OperandKind::Register(0, _), 64)), Some((OperandKind::Immediate(false), _)), None, None)) => {
			return Ok(());
		},
		("sub", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(true), _)), None, None)) => {
			return Ok(());
		},
		("sub", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), _)), None, None)) => {
			return Ok(());
		},
		("swapgs", Operands(None, None, None, None)) => {
			return Ok(());
		},
		("test", Operands(Some((OperandKind::Memory, _)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("test", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("test", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("test", Operands(Some((OperandKind::Register(0, _), 64)), Some((OperandKind::Immediate(false), _)), None, None)) => {
			return Ok(());
		},
		("test", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), _)), None, None)) => {
			return Ok(());
		},
		("xadd", Operands(Some((OperandKind::Memory, _)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("xadd", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("xchg", Operands(Some((OperandKind::Register(0, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("xchg", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(0, _), 64)), None, None)) => {
			return Ok(());
		},
		("xchg", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("xchg", Operands(Some((OperandKind::Memory, _)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("xchg", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("xor", Operands(Some((OperandKind::Memory, _)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("xor", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("xor", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("xor", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("xor", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), 8)), None, None)) => {
			return Ok(());
		},
		("xor", Operands(Some((OperandKind::Register(0, _), 64)), Some((OperandKind::Immediate(true), _)), None, None)) => {
			return Ok(());
		},
		("xor", Operands(Some((OperandKind::Register(0, _), 64)), Some((OperandKind::Immediate(false), _)), None, None)) => {
			return Ok(());
		},
		("xor", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(true), _)), None, None)) => {
			return Ok(());
		},
		("xor", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), _)), None, None)) => {
			return Ok(());
		},
		("cmovcc", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("cmovcc", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("jcc", Operands(Some((OperandKind::Immediate(false), _)), None, None, None)) => {
			return Ok(());
		},
		("cvtsi2ss", Operands(Some(None), Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None)) => {
			return Ok(());
		},
		("cvtss2si", Operands(Some((OperandKind::Register(_, _), 64)), Some(None), None, None)) => {
			return Ok(());
		},
		("cvtss2si", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("cvttss2si", Operands(Some((OperandKind::Register(_, _), 64)), Some(None), None, None)) => {
			return Ok(());
		},
		("movmskps", Operands(Some((OperandKind::Register(_, _), 64)), Some(None), None, None)) => {
			return Ok(());
		},
		("fxrstor64", Operands(Some((OperandKind::Memory, _)), None, None, None)) => {
			return Ok(());
		},
		("fxsave64", Operands(Some((OperandKind::Memory, _)), None, None, None)) => {
			return Ok(());
		},
		("movnti", Operands(Some((OperandKind::Memory, _)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("movq", Operands(Some(None), Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None)) => {
			return Ok(());
		},
		("movq", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some(None), None, None)) => {
			return Ok(());
		},
		("pextrw", Operands(Some((OperandKind::Register(_, _), 64)), Some(None), Some((OperandKind::Immediate(false), _)), None)) => {
			return Ok(());
		},
		("pinsrw", Operands(Some(None), Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Immediate(false), _)), None)) => {
			return Ok(());
		},
		("cvtsd2si", Operands(Some((OperandKind::Register(_, _), 64)), Some(None), None, None)) => {
			return Ok(());
		},
		("cvtsd2si", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("cvtsi2sd", Operands(Some(None), Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None)) => {
			return Ok(());
		},
		("cvttsd2si", Operands(Some((OperandKind::Register(_, _), 64)), Some(None), None, None)) => {
			return Ok(());
		},
		("cvttsd2si", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Memory, _)), None, None)) => {
			return Ok(());
		},
		("movmskpd", Operands(Some((OperandKind::Register(_, _), 64)), Some(None), None, None)) => {
			return Ok(());
		},
		("vmread", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Register(_, _), 64)), None, None)) => {
			return Ok(());
		},
		("vmwrite", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None)) => {
			return Ok(());
		},
		("extractps", Operands(Some((OperandKind::Register(_, _), 64)), Some(None), Some((OperandKind::Immediate(false), 8)), None)) => {
			return Ok(());
		},
		("pextrb", Operands(Some((OperandKind::Register(_, _), 64)), Some(None), Some((OperandKind::Immediate(false), 8)), None)) => {
			return Ok(());
		},
		("pextrq", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some(None), Some((OperandKind::Immediate(false), 8)), None)) => {
			return Ok(());
		},
		("pextrw", Operands(Some((OperandKind::Register(_, _), 64)), Some(None), Some((OperandKind::Immediate(false), 8)), None)) => {
			return Ok(());
		},
		("pinsrq", Operands(Some(None), Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), Some((OperandKind::Immediate(false), 8)), None)) => {
			return Ok(());
		},
		("crc32", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 8) | (OperandKind::Memory, 8)), None, None)) => {
			return Ok(());
		},
		("crc32", Operands(Some((OperandKind::Register(_, _), 64)), Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None)) => {
			return Ok(());
		},
		("hint_nop0", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop1", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop2", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop3", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop4", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop5", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop6", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop7", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop8", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop9", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop10", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop11", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop12", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop13", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop14", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop15", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop16", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop17", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop18", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop19", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop20", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop21", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop22", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop23", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop24", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop25", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop26", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop27", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop28", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop29", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop30", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop31", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop32", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop33", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop34", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop35", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop36", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop37", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop38", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop39", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop40", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop41", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop42", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop43", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop44", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop45", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop46", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop47", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop48", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop49", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop50", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop51", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop52", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop53", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop54", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop55", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop56", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop57", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop58", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop59", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop60", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop61", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop62", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		("hint_nop63", Operands(Some((OperandKind::Register(_, _), 64) | (OperandKind::Memory, 64)), None, None, None)) => {
			return Ok(());
		},
		_ => return Err(());,
	}
}


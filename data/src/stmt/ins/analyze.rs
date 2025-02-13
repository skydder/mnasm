use util::emit_error;

use crate::{Operand, OperandKind};

use super::{ins_analyzer::ins_analyzer, Ins};

pub fn analyze_ins<'a>(ins: &Ins<'a>) {
    if ins.check {
        match ins_analyzer(ins.instruction, Operands::convert_operands(&ins.operands)) {
            Ok(_) => return,
            Err(_) => {
                emit_error!(ins.location, "unsuppoted instruction or operands.");
            }
        }
    }
}

pub struct Operands(
    pub Option<(OperandKind, usize)>,
    pub Option<(OperandKind, usize)>,
    pub Option<(OperandKind, usize)>,
    pub Option<(OperandKind, usize)>,
);

impl Operands {
    fn default() -> Self {
        Self(None, None, None, None)
    }
    fn set(&mut self, i: usize, value: Option<(OperandKind, usize)>) {
        match i {
            0 => self.0 = value,
            1 => self.1 = value,
            2 => self.2 = value,
            3 => self.3 = value,
            _ => todo!(),
        }
    }
    fn convert_operands<'a>(operands: &Vec<Box<dyn Operand + 'a>>) -> Self {
        let mut op = Operands::default();
        for i in 0..4 {
            op.set(i, operands.get(i).map_or(None, |o| Some(o.op())));
        }
        op
    }
}

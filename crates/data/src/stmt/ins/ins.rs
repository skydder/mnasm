use util::Location;

use super::analyze_ins;
use crate::Operand;
#[derive(Debug)]
pub struct Ins<'a> {
    pub instruction: &'a str,
    pub operands: Vec<Box<dyn Operand + 'a>>,
    pub location: Location<'a>,
}

impl<'a> Ins<'a> {
    pub fn new(
        instruction: &'a str,
        operands: Vec<Box<dyn Operand + 'a>>,
        location: Location<'a>,
    ) -> Self {
        Self {
            instruction,
            operands,
            location,
        }
    }

    pub fn codegen(&self) -> String {
        let mut code = self.instruction.to_string();
        if !self.operands.is_empty() {
            stringify_operands(&mut code, &self.operands, 0);
        }
        code
    }

    pub fn analyze(&self) {
        for op in &self.operands {
            op.analyze();
        }
        analyze_ins(self);
    }
}

fn stringify_operands<'a>(
    code: &'a mut String,
    operands: &'a Vec<Box<dyn Operand + 'a>>,
    n: usize,
) {
    if n >= operands.len() - 1 {
        code.push_str(&format!(" {}", operands[n].codegen()));
        return;
    }
    code.push_str(&format!(" {},", operands[n].codegen()));
    stringify_operands(code, operands, n + 1);
}

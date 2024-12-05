use util::Location;

use crate::{Operand, Stmt};

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
            instruction: instruction,
            operands: operands,
            location: location,
        }
    }

    fn codegen(&self) -> String {
        let mut code = format!("{}", self.instruction);
        if self.operands.len() != 0 {
            stringify_operands(&mut code, &self.operands, 0);
        }
        code
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

#[derive(Debug)]
pub struct CompoundIns<'a> {
    pub compound: Vec<Ins<'a>>,
    pub location: Location<'a>,
}

impl<'a> CompoundIns<'a> {
    pub fn new(compound: Vec<Ins<'a>>, location: Location<'a>) -> Self {
        Self {
            compound: compound,
            location: location,
        }
    }
}

impl<'a> Stmt for CompoundIns<'a> {
    fn codegen(&self) -> String {
        let mut code = String::new();
        for i in &self.compound {
            code.push_str(&format!("\t{}\n", i.codegen()));
        }
        code
    }
}

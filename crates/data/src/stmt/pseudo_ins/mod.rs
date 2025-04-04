use util::Location;

use crate::{Analyze, Codegen, Object, Operand};

use super::Stmt;

#[derive(Debug)]
pub struct PseudoIns<'a> {
    pub instruction: &'a str,
    pub operands: Vec<String>,
    pub nasm_op: Vec<Box<dyn Operand + 'a>>,
    pub location: Location<'a>,
    kind: bool,
}

impl<'a> PseudoIns<'a> {
    pub fn new(instruction: &'a str, operands: Vec<String>, location: Location<'a>) -> Self {
        Self {
            instruction,
            operands,
            location,
            nasm_op: Vec::new(),
            kind: false,
        }
    }
    pub fn new_nasm(
        instruction: &'a str,
        operands: Vec<Box<dyn Operand + 'a>>,
        location: Location<'a>,
    ) -> Self {
        Self {
            instruction,
            operands: Vec::new(),
            location,
            nasm_op: operands,
            kind: true,
        }
    }
    fn codegen_operands(&self) -> String {
        if self.kind {
            stringfy_vec(
                &self.nasm_op.iter().map(|op| op.codegen()).collect(),
                0,
                String::new(),
            )
        } else {
            stringfy_vec(&self.operands, 0, String::new())
        }
    }
}

fn stringfy_vec(v: &Vec<String>, mut n: usize, mut s: String) -> String {
    if v.is_empty() {
        return s;
    }
    if n == v.len() - 1 {
        s.push_str(&v[n]);
        s
    } else {
        s.push_str(&format!("{}, ", v[n]));
        n += 1;
        stringfy_vec(v, n, s)
    }
}

impl<'a> Stmt<'a> for PseudoIns<'a> {
    fn kind(&self) -> super::StmtKind {
        todo!()
    }
}
impl Object for PseudoIns<'_> {}
impl Analyze for PseudoIns<'_> {
    fn analyze(&self) {}
}

impl Codegen for PseudoIns<'_> {
    fn codegen(&self) -> String {
        match self.instruction {
            "include" => String::new(),
            "nasm" => format!("\t{}\n", self.operands[0]),
            _ => format!("\t{} {}\n", self.instruction, self.codegen_operands()),
        }
    }

    fn to_code(&self) -> String {
        if self.kind {
            format!(
                "{}({})",
                self.instruction,
                stringfy_vec(
                    &self.nasm_op.iter().map(|o| o.to_code()).collect(),
                    0,
                    String::new()
                )
            )
        } else {
            format!(
                "{}({})",
                self.instruction,
                stringfy_vec(&self.operands, 0, String::new())
            )
        }
    }
}

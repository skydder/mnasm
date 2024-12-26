use util::Location;

use crate::{Analyze, Codegen, Object};

use super::Stmt;

#[derive(Debug)]
pub struct PseudoIns<'a> {
    pub instruction: &'a str,
    pub operands: Vec<String>,
    pub location: Location<'a>,
}

impl<'a> PseudoIns<'a> {
    pub fn new(instruction: &'a str, operands: Vec<String>, location: Location<'a>) -> Self {
        Self {
            instruction: instruction,
            operands: operands,
            location: location,
        }
    }

    fn codegen_operands(&self) -> String {
        stringfy_vec(&self.operands, 0, String::new())
    }
}

fn stringfy_vec<'a>(v: &Vec<String>, mut n: usize, mut s: String) -> String {
    if n == v.len() - 1 {
        s.push_str(&v[n]);
        return s;
    } else {
        s.push_str(&format!("{}, ", v[n]));
        n += 1;
        return stringfy_vec(v, n, s);
    }
}

impl<'a> Stmt<'a> for PseudoIns<'a> {
    fn kind(&self) -> super::StmtKind {
        todo!()
    }
}
impl<'a> Object for PseudoIns<'a> {}
impl<'a> Analyze for PseudoIns<'a> {
    fn analyze(&self) {}
}

impl<'a> Codegen for PseudoIns<'a> {
    fn codegen(&self) -> String {
        match self.instruction {
            "include" => String::new(),
            _ => format!("\t{} {}\n", self.instruction, self.codegen_operands())
        }
    }
}

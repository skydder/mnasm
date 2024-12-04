use util::Location;

use crate::Stmt;

#[derive(Debug)]
pub struct Ins<'a> {
    pub instruction: &'a str,
    pub operand: (),
    pub location: Location<'a>,
}

impl<'a> Ins<'a> {
    pub fn new(instruction: &'a str, location: Location<'a>) -> Self {
        Self {
            instruction: instruction,
            operand: (),
            location: location,
        }
    }
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
            code.push_str(&format!("\t{}\n", i.instruction));
        }
        code
    }
}

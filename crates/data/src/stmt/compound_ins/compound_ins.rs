use util::Location;

use crate::{Analyze, Codegen, Ins, Object, Stmt, StmtKind};

use super::CompoundIns;

impl<'a> CompoundIns<'a> {
    pub fn new(compound: Vec<Ins<'a>>, location: Location<'a>) -> Self {
        Self { compound, location }
    }
}

impl Object for CompoundIns<'_> {}
impl Codegen for CompoundIns<'_> {
    fn codegen(&self) -> String {
        let mut code = String::new();
        for i in &self.compound {
            code.push_str(&format!("\t{}\n", i.codegen()));
        }
        code
    }

    fn to_code(&self) -> String {
        let mut code = String::new();
        for (n, ins) in self.compound.iter().enumerate() {
            if self.compound.len() - n == 1 {
                code.push_str(&ins.to_code());
            } else {
                code.push_str(&ins.to_code());
                code.push_str(", ");
            }
        }
        code
    }
}
impl Analyze for CompoundIns<'_> {
    fn analyze(&self) {
        for i in &self.compound {
            i.analyze();
        }
    }
}

impl<'a> Stmt<'a> for CompoundIns<'a> {
    fn kind(&self) -> crate::StmtKind {
        StmtKind::Ins
    }
}

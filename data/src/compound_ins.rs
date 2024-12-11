use util::Location;

use crate::{Ins, Stmt, StmtKind};


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

impl<'a> Stmt<'a> for CompoundIns<'a> {
    fn codegen(&self) -> String {
        let mut code = String::new();
        for i in &self.compound {
            code.push_str(&format!("\t{}\n", i.codegen()));
        }
        code
    }
    
    fn kind(&self) -> crate::StmtKind {
        StmtKind::Ins
    }

    fn analyze<'b>(&self, mut labels: &'b mut std::collections::HashMap<crate::Label<'a>, crate::LabelState>) -> &'b mut std::collections::HashMap<crate::Label<'a>, crate::LabelState> {
        for ins in &self.compound {
            labels = ins.analyze(labels);
        }
        labels
    }
}

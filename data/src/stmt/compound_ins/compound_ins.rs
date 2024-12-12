use util::Location;

use crate::{Ins, LabelInfo, Stmt, StmtKind};

use super::CompoundIns;


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

    fn analyze(
        &self,
        mut labels: &'a mut LabelInfo<'a>,
    ) -> &mut LabelInfo<'a> {
        for ins in &self.compound {
            labels = ins.analyze(labels);
        }
        labels
    }
}

use util::Location;

use crate::{Analyze, Codegen, Ins, Object, Stmt, StmtKind};

use super::CompoundIns;

impl<'a> CompoundIns<'a> {
    pub fn new(compound: Vec<Ins<'a>>, location: Location<'a>) -> Self {
        Self {
            compound: compound,
            location: location,
        }
    }
}

impl<'a> Object for CompoundIns<'a> {}
impl<'a> Codegen for CompoundIns<'a> {
    fn codegen(&self) -> String {
        let mut code = String::new();
        for i in &self.compound {
            code.push_str(&format!("\t{}\n", i.codegen()));
        }
        code
    }
}
impl<'a> Analyze for CompoundIns<'a> {
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

    // fn analyze(
    //     &self,
    //     mut labels: &'a mut LabelInfo<'a>,
    // ) -> &mut LabelInfo<'a> {
    //     for ins in &self.compound {
    //         labels = ins.analyze(labels);
    //     }
    //     labels
    // }
}

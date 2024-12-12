use util::emit_error;
use util::Location;

use crate::stmt::LabelInfo;
use crate::Name;
use crate::{Block, Label, LabelState, Stmt, StmtKind};

use super::LabelDef;


impl<'a> LabelDef<'a> {
    pub fn new(
        label: Name<'a>,
        is_global: bool,
        section: Option<Name<'a>>,
        block: Option<Block<'a>>,
        location: Location<'a>,
    ) -> Self {
        Self {
            label: label,
            is_global: is_global,
            section: section,
            block: block,
            location: location,
        }
    }

    pub fn label(&self) -> Label<'a> {
        Label::new(self.label, self.location)
    }
}

impl<'a> Stmt<'a> for LabelDef<'a> {
    fn codegen(&self) -> String {
        let mut code = String::new();

        if self.section != None {
            code.push_str(&format!("section {}\n", self.section.unwrap().get()));
        }

        if self.is_global {
            code.push_str(&format!("global {}\n", self.label.get()));
        }

        code.push_str(&format!("{}:\n", self.label.get()));
        if let Some(bl) = &self.block {
            code.push_str(&bl.codegen());
        }
        code.push('\n');
        code
    }

    fn kind(&self) -> crate::StmtKind {
        StmtKind::LabelDef
    }

    fn analyze(
        &self,
        mut labels: &'a mut LabelInfo<'a>,
    ) -> &'a mut LabelInfo<'a> {
        if let Some(data) = labels.get_mut(&self.label()) {
            *data = match data {
                LabelState::Used => LabelState::UsedAndDefined,
                _ => {
                    emit_error!(self.location, "multiple definition!!");
                }
            };
        } else {
            labels.insert(self.label(), LabelState::Defined);
        }
        if let Some(block) = &self.block {
           // labels = block.analyze(labels);
        }
        labels
    }
}

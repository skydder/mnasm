use util::Location;

use crate::{Block, Label, Stmt, StmtKind};

#[derive(Debug)]
pub struct LabelDef<'a> {
    pub label: &'a str,
    pub is_global: bool, // visibility
    pub section: &'a str,
    pub block: Option<Block<'a>>,
    pub location: Location<'a>,
}

impl<'a> LabelDef<'a> {
    pub fn new(
        label: &'a str,
        is_global: bool,
        section: &'a str,
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

impl<'a> Stmt for LabelDef<'a> {
    fn codegen(&self) -> String {
        let mut code = String::new();

        if self.section != "" {
            code.push_str(&format!("section {}\n", self.section));
        }

        if self.is_global {
            code.push_str(&format!("global {}\n", self.label));
        }

        code.push_str(&format!("{}:\n", self.label));
        if let Some(bl) = &self.block {
            code.push_str(&bl.codegen());
        }
        code.push('\n');
        code
    }
    
    fn kind(&self) -> crate::StmtKind {
        StmtKind::LabelDef
    }
}

use util::Location;

use crate::Analyze;
use crate::Codegen;
use crate::Ident;
use crate::Object;
use crate::{Block, Stmt, StmtKind};

use super::LabelDef;

impl<'a> LabelDef<'a> {
    pub fn new(
        label: Ident<'a>,
        gen_label: String,
        is_global: bool,
        section: Option<Ident<'a>>,
        block: Option<Block<'a>>,
        location: Location<'a>,
    ) -> Self {
        Self {
            label: label,
            gen_label: gen_label,
            is_global: is_global,
            section: section,
            block: block,
            location: location,
        }
    }
}

impl<'a> Object for LabelDef<'a> {}
impl<'a> Codegen for LabelDef<'a> {
    fn codegen(&self) -> String {
        let mut code = String::new();

        if self.section != None {
            code.push_str(&format!("section {}\n", self.section.unwrap().get()));
        }

        if self.is_global {
            code.push_str(&format!("global {}\n", self.label.get()));
        }

        code.push_str(&format!("{}:\n", self.gen_label));
        if let Some(bl) = &self.block {
            code.push_str(&bl.codegen());
        }
        code.push('\n');
        code
    }
}
impl<'a> Analyze for LabelDef<'a> {
    fn analyze(&self) {
        self.block.as_ref().and_then(|b| Some(b.analyze()));
    }
}

impl<'a> Stmt<'a> for LabelDef<'a> {
    fn kind(&self) -> crate::StmtKind {
        StmtKind::LabelDef
    }
}

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
            label,
            gen_label,
            is_global,
            section,
            block,
            location,
        }
    }
}

impl Object for LabelDef<'_> {}
impl Codegen for LabelDef<'_> {
    fn codegen(&self) -> String {
        let mut code = String::new();

        if self.section.is_some() {
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

    fn to_code(&self) -> String {
        let mut code = format!("<{}", self.label);
        if self.is_global {
            code.push_str("global");
        }
        if let Some(sec) = self.section {
            code.push_str(&sec.get());
        }
        code.push('>');
        if let Some(block) = &self.block {
            code.push_str(&block.to_code());
        } else {
            code.push('\n');
        }
        code
    }
}
impl Analyze for LabelDef<'_> {
    fn analyze(&self) {
        if let Some(b) = self.block.as_ref() {
            b.analyze()
        }
    }
}

impl<'a> Stmt<'a> for LabelDef<'a> {
    fn kind(&self) -> crate::StmtKind {
        StmtKind::LabelDef
    }
}

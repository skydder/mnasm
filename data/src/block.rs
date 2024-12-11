use util::Location;

use crate::{Stmt, StmtKind};

#[derive(Debug)]
pub struct Block<'a> {
    pub indent_depth: usize,
    pub stmts: Vec<Box<dyn Stmt<'a> + 'a>>,
    pub location: Location<'a>,
}

impl<'a> Block<'a> {
    pub fn new(
        indent_depth: usize,
        stmts: Vec<Box<dyn Stmt<'a> + 'a>>,
        location: Location<'a>,
    ) -> Self {
        Self {
            indent_depth: indent_depth,
            stmts: stmts,
            location: location,
        }
    }
}

impl<'a> Stmt<'a> for Block<'a> {
    fn codegen(&self) -> String {
        let mut code = String::new();
        for i in &self.stmts {
            code.push_str(&i.codegen());
        }
        code
    }
    
    fn kind(&self) -> StmtKind {
        StmtKind::Block
    }

    fn analyze<'b>(&self, mut labels: &'b mut std::collections::HashMap<crate::Label<'a>, crate::LabelState>) -> &'b mut std::collections::HashMap<crate::Label<'a>, crate::LabelState> {
        for stmt in &self.stmts {
            labels = stmt.analyze(labels);
        }
        labels
    }
}

use std::{cell::RefCell, rc::Rc};

use util::Location;

use crate::{Analyze, Codegen, Object, Stmt, StmtKind};

use super::{Block, Scope};

impl<'a> Block<'a> {
    pub fn new(
        indent_depth: usize,
        stmts: Vec<Box<dyn Stmt<'a> + 'a>>,
        location: Location<'a>,
        scope: Rc<RefCell<Scope<'a>>>,
    ) -> Self {
        Self {
            indent_depth: indent_depth,
            stmts: RefCell::new(stmts),
            location: location,
            scope: scope,
        }
    }
}

impl<'a> Object for Block<'a> {}
impl<'a> Codegen for Block<'a> {
    fn codegen(&self) -> String {
        let mut code = String::new();
        for i in self.stmts.borrow().iter() {
            code.push_str(&i.codegen());
        }
        code
    }
}
impl<'a> Analyze for Block<'a> {
    fn analyze(&self) {
        todo!()
    }
}

impl<'a> Stmt<'a> for Block<'a> {
    fn kind(&self) -> StmtKind {
        StmtKind::Block
    }

    // fn analyze(
    //     &self,
    //     labels: &'a mut LabelInfo<'a>,
    // ) -> &'a mut LabelInfo<'a> {
    //     // for stmt in &self.stmts {
    //     //     labels = stmt.analyze(labels);
    //     // }
    //     labels
    // }
}

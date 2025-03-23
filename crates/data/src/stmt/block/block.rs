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
            indent_depth,
            stmts: RefCell::new(stmts),
            location,
            scope,
        }
    }
}

impl Object for Block<'_> {}
impl Codegen for Block<'_> {
    fn codegen(&self) -> String {
        let mut code = String::new();
        for i in self.stmts.borrow().iter() {
            code.push_str(&i.codegen());
        }
        code
    }
}
impl Analyze for Block<'_> {
    fn analyze(&self) {
        for i in self.stmts.borrow().iter() {
            i.analyze();
        }
    }
}

impl<'a> Stmt<'a> for Block<'a> {
    fn kind(&self) -> StmtKind {
        StmtKind::Block
    }
}

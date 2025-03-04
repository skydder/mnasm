mod block;
mod scope;

use std::{cell::RefCell, rc::Rc};

use super::{Macro, Stmt};
use crate::Ident;
use util::Location;

pub struct Scope<'a> {
    scope_name: Ident<'a>,
    parent: Option<Rc<RefCell<Scope<'a>>>>,
    labels: RefCell<Vec<(Ident<'a>, Option<Rc<RefCell<Scope<'a>>>>)>>,
    macros: RefCell<Vec<(Ident<'a>, Rc<Macro<'a>>)>>,
    path_name: String,
}

impl<'a> std::fmt::Debug for Scope<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scope")
            .field("scope_name", &self.scope_name)
            .field("labels", &self.labels)
            .field("macros", &self.macros)
            .field("path_name", &self.path_name)
            .finish()
    }
}

#[derive(Debug)]
pub struct Block<'a> {
    pub indent_depth: usize,
    pub stmts: RefCell<Vec<Box<dyn Stmt<'a> + 'a>>>,
    pub location: Location<'a>,
    pub scope: Rc<RefCell<Scope<'a>>>,
}

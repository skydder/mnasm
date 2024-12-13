mod block;
mod scope;

use std::{cell::RefCell, rc::Rc};

use super::Stmt;
use crate::Name;
use util::Location;

#[derive(Debug)]
pub struct Scope<'a> {
    scope_name: Option<Name<'a>>,
    parent: Option<Rc<RefCell<Scope<'a>>>>,
    labels: RefCell<Vec<Name<'a>>>,
}

#[derive(Debug)]
pub struct Block<'a> {
    pub indent_depth: usize,
    pub stmts: RefCell<Vec<Box<dyn Stmt<'a> + 'a>>>,
    pub location: Location<'a>,
    pub scope: Rc<RefCell<Scope<'a>>>,
}

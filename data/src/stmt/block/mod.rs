mod block;
mod scope;

use std::{cell::RefCell, rc::Rc};

use super::{Macro, Stmt};
use crate::Ident;
use util::Location;

#[derive(Debug)]
pub struct Scope<'a> {
    scope_name: Ident<'a>,
    parent: Option<Rc<RefCell<Scope<'a>>>>,
    labels: RefCell<Vec<(Ident<'a>, Option<Rc<RefCell<Scope<'a>>>>)>>,
    macros: RefCell<Vec<(Ident<'a>, Rc<Macro<'a>>)>>,
    path_name: String,
}

#[derive(Debug)]
pub struct Block<'a> {
    pub indent_depth: usize,
    pub stmts: RefCell<Vec<Box<dyn Stmt<'a> + 'a>>>,
    pub location: Location<'a>,
    pub scope: Rc<RefCell<Scope<'a>>>,
}

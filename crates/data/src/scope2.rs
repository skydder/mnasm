use std::{
    cell::{Cell, RefCell}, collections::HashMap, rc::Rc
};

use crate::{Ident, Path, PathState};

use util::{AsmError, AsmResult, Location};

// Scope is a meta-data

pub struct ScopeManager<'code> {
    local: Scope<'code>,
    global: Scope<'code>,
}

pub enum DefinedStatus<'code> {
    Undefined(Rc<Vec<Location<'code>>>),
    Defined
}

pub struct Scope<'code> {
    manager: Rc<ScopeManager<'code>>,
    name: Ident,
    defined_status: DefinedStatus<'code>,
    children: RefCell<HashMap<Ident, Rc<Scope<'code>>>>,
    absolute_path: Path,  // this field is unneseary in the perspective of "DRY"
}

impl<'code> Scope<'code> {
    fn new(
        manager: Rc<ScopeManager<'code>>,
        name: Ident,
        defined_status: DefinedStatus<'code>,
        children: RefCell<HashMap<Ident, Rc<Scope<'code>>>>,
        absolute_path: Path,
    ) -> Rc<Self> {
        Rc::new(Self { manager, name, defined_status, children, absolute_path })
    }

    fn add_new_scope(self: &Rc<Self>, new_scope: Rc<Self>) -> bool {
        if let Some(scope) = self.children.borrow_mut().get(&new_scope.name) {
            return false;
        }
        self.children.borrow_mut().insert(new_scope.name.clone(), new_scope); 
        true
    }

    fn absolute_path(self: &Rc<Self>) -> Path {
        self.absolute_path.clone()
    }
}
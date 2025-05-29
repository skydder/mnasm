use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{Ident, Path, PathState};

use util::Location;

// Scope is a meta-data

pub struct ScopeManager<'code> {
    local: Rc<Scope<'code>>,
    global: Rc<Scope<'code>>,
}

#[derive(Clone)]
pub enum DefinedStatus<'code> {
    Undefined(Rc<RefCell<Vec<Location<'code>>>>),
    Defined,
}

pub struct Scope<'code> {
    manager: Rc<ScopeManager<'code>>,
    name: Ident,
    defined_status: RefCell<DefinedStatus<'code>>,
    children: RefCell<HashMap<Ident, Rc<Scope<'code>>>>,
    absolute_path: RefCell<Path>, // this field is unneseary in the perspective of "DRY"
}

impl<'code> Scope<'code> {
    pub fn new(
        manager: Rc<ScopeManager<'code>>,
        name: Ident,
        defined_status: RefCell<DefinedStatus<'code>>,
        children: RefCell<HashMap<Ident, Rc<Scope<'code>>>>,
        absolute_path: RefCell<Path>,
    ) -> Rc<Self> {
        Rc::new(Self {
            manager,
            name,
            defined_status,
            children,
            absolute_path,
        })
    }

    pub fn add_new_scope(self: &Rc<Self>, new_scope: Rc<Self>) -> bool {
        if let Some(scope) = self.children.borrow_mut().get(&new_scope.name) {
            return false;
        }
        self.children
            .borrow_mut()
            .insert(new_scope.name.clone(), new_scope);
        true
    }

    pub fn absolute_path(self: &Rc<Self>) -> Path {
        self.absolute_path.borrow().clone()
    }

    pub fn find_label(self: Rc<Self>, path: Path) -> Result<Rc<Scope<'code>>, Rc<Scope<'code>>> {
        if let Some(target) = self.children.borrow_mut().get(&path.get(0).unwrap()) {
            if let Some(next_path) = path.next_path() {
                target.clone().find_label(next_path)
            } else {
                Ok(target.clone())
            }
        } else {
            Err(self.clone())
        }
    }

    pub fn manager(&self) -> Rc<ScopeManager<'code>> {
        self.manager.clone()
    }

    pub fn get_defined_status(&self) -> DefinedStatus<'code> {
        self.defined_status.borrow().clone()
    }
}

impl<'code> ScopeManager<'code> {
    pub fn find_label(
        self: &Rc<Self>,
        path: Path,
    ) -> Result<Rc<Scope<'code>>, Result<Rc<Scope<'code>>, ()>> {
        match path.state() {
            PathState::GlobalRelative => self.global.clone().find_label(path).map_err(Ok),
            PathState::Absolute => self.local.clone().find_label(path).map_err(Ok),
            PathState::Relative => Err(Err(())),
        }
    }

    pub fn add_global_label(self: &Rc<Self>, scope: Rc<Scope<'code>>) -> bool {
        self.global.add_new_scope(scope)
    }
}

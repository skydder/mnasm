use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

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
    manager: RefCell<Weak<ScopeManager<'code>>>,
    name: Ident,
    defined_status: RefCell<DefinedStatus<'code>>,
    children: Rc<RefCell<HashMap<Ident, Rc<Scope<'code>>>>>,
    absolute_path: RefCell<Path>, // this field is unneseary in the perspective of "DRY"
}

impl<'code> Scope<'code> {
    pub fn new(
        manager: Weak<ScopeManager<'code>>,
        name: Ident,
        defined_status: DefinedStatus<'code>,
        children: HashMap<Ident, Rc<Scope<'code>>>,
        absolute_path: Path,
    ) -> Rc<Self> {
        Rc::new(Self {
            manager: RefCell::new(manager),
            name,
            defined_status: RefCell::new(defined_status),
            children: Rc::new(RefCell::new(children)),
            absolute_path: RefCell::new(absolute_path),
        })
    }

    pub fn add_new_scope(&self, new_scope: Rc<Self>) -> bool {
        // if self.children.borrow_mut().get(&new_scope.name).is_some() {
        //     return false;
        // }
        self.children
            .borrow_mut()
            .insert(new_scope.name.clone(), new_scope);
        true
    }

    pub fn absolute_path(&self) -> Path {
        self.absolute_path.borrow().clone()
    }

    pub fn find_label(self: Rc<Self>, path: &Path) -> Result<Rc<Scope<'code>>, Rc<Scope<'code>>> {
        if let Some(target) = self.children.borrow_mut().get(&path.get(0).unwrap()) {
            if let Some(next_path) = path.next_path() {
                target.clone().find_label(&next_path)
            } else {
                Ok(target.clone())
            }
        } else {
            Err(self)
        }
    }

    pub fn manager(&self) -> Rc<ScopeManager<'code>> {
        self.manager.clone().borrow().upgrade().unwrap()
    }

    pub fn get_defined_status(&self) -> DefinedStatus<'code> {
        self.defined_status.borrow().clone()
    }

    pub fn get_child(&self, name: &Ident) -> Option<Rc<Scope<'code>>> {
        self.children.borrow_mut().get(name).cloned()
    }

    pub fn get_children(&self) -> Rc<RefCell<HashMap<Ident, Rc<Scope<'code>>>>> {
        self.children.clone()
    }
}

impl<'code> ScopeManager<'code> {
    pub fn new() -> Rc<Self> {
        let new = Rc::new(Self {
            local: Scope::new(
                Weak::new(),
                Ident::new("local".to_string()),
                DefinedStatus::Defined,
                HashMap::new(),
                Path::new(Rc::new(Vec::new()), PathState::Absolute),
            ),
            global: Scope::new(
                Weak::new(),
                Ident::new("global".to_string()),
                DefinedStatus::Defined,
                HashMap::new(),
                Path::new(Rc::new(Vec::new()), PathState::Absolute),
            ),
        });
        *new.global.manager.borrow_mut() = Rc::downgrade(&new);
        *new.local.manager.borrow_mut() = Rc::downgrade(&new);
        new
    }
    pub fn find_label(
        self: &Rc<Self>,
        path: &Path,
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

    pub fn local(&self) -> Rc<Scope<'code>> {
        self.local.clone()
    }

    pub fn global(&self) -> Rc<Scope<'code>> {
        self.global.clone()
    }
}

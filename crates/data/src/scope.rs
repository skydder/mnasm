use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use crate::{Ident, Path};

#[allow(dead_code)]
pub struct Scope<'code> {
    parent: Option<Rc<Scope<'code>>>, // root of the tree
    is_global: bool,
    name: Ident,
    in_scope: RefCell<Vec<Rc<Scope<'code>>>>, // children
    is_defined: Cell<bool>, // to detect undefined label and multipule-defined label
    path: Path,             // the route to this Scope
}

// name isn't the label
// global label has two label? (not decided yet)

// ===
// 1. Path -> Scope
// 2.
// ===

impl<'code> Scope<'code> {
    fn new(
        parent: Option<Rc<Scope<'code>>>,
        name: Ident,
        is_global: bool,
        is_defined: bool,
        path: Path,
    ) -> Rc<Self> {
        Rc::new(Self {
            parent,
            name,
            in_scope: RefCell::new(Vec::new()),
            is_defined: Cell::new(is_defined),
            path,
            is_global,
        })
    }

    fn new_global_root(parent: Rc<Self>) -> Rc<Self> {
        Self::new(
            Some(parent),
            Ident::new("_global".to_string()),
            true,
            true,
            Path::new(Rc::new(Vec::new()), false),
        )
    }

    fn new_local_root(parent: Rc<Self>) -> Rc<Self> {
        Self::new(
            Some(parent),
            Ident::new("_local".to_string()),
            true,
            true,
            Path::new(Rc::new(Vec::new()), false),
        )
    }

    pub fn init_root() -> Rc<Self> {
        let root = Self::new(
            None,
            Ident::new("_root".to_string()),
            true,
            true,
            Path::new(Rc::new(Vec::new()), false),
        );
        let global = Self::new_global_root(root.clone());
        root.add_to_in_scope(global.clone());
        root.add_to_in_scope(Self::new_local_root(root.clone()));
        root
    }

    fn get_local_root(self: &Rc<Self>) -> Rc<Self> {
        let mut current = self.clone();
        // unsafe { // potentially has likelihood of unwrap() failed
        loop {
            for scope in current.in_scope.borrow().iter() {
                if scope.name.get_str() == "_local" {
                    return scope.clone();
                }
            }
            current = current.parent.clone().unwrap();
        }
        // }
    }

    fn get_global_root(self: &Rc<Self>) -> Rc<Self> {
        let mut current = self.clone();
        // unsafe { // potentially has likelihood of unwrap() failed
        loop {
            for scope in current.in_scope.borrow().iter() {
                if scope.name.get_str() == "_global" {
                    return scope.clone();
                }
            }
            current = current.parent.clone().unwrap();
        }
        // }
    }

    pub fn new_local(
        parent: Rc<Scope<'code>>,
        name: Ident,
        is_defined: bool,
        path: Path,
    ) -> Rc<Self> {
        Self::new(Some(parent), name, false, is_defined, path)
    }

    pub fn new_global(
        parent: Rc<Scope<'code>>,
        name: Ident,
        is_defined: bool,
        path: Path,
    ) -> Rc<Self> {
        let new = Self::new(Some(parent), name, false, is_defined, path);
        new.get_global_root().add_to_in_scope(new.clone());
        new
    }

    pub fn add_to_in_scope(&self, scope: Rc<Scope<'code>>) {
        self.in_scope.borrow_mut().push(scope);
    }

    pub fn has_path_of(self: &Rc<Self>, path: &Path) -> bool {
        let mut flag = true;
        let mut current = if path.is_relative() {
            self.clone()
        } else {
            self.get_local_root()
        };
        let route = path.clone().into_iter();
        for point in route {
            current = if let Some(scope) = current
                .in_scope
                .borrow()
                .iter()
                .find(|&scope| scope.name == point)
            {
                scope.clone()
            } else {
                flag = false;
                Self::new_local(
                    current.clone(),
                    point.clone(),
                    false,
                    current.path.append(point),
                )
            }
        }
        flag
    }

    pub fn get_label(self: &Rc<Self>) -> String {
        let mut label = String::new();
        for ident in self.path.path().iter() {
            label.push('_');
            label.push_str(&ident.get_str());
        }
        label
    }

    pub fn path(self: &Rc<Self>) -> Path {
        self.path.clone()
    }

    pub fn get_child(self: &Rc<Self>, name: &Ident) -> Option<Rc<Self>> {
        // eprintln!("{:#?}", self.in_scope);
        for child in self.in_scope.borrow().iter() {
            if child.name == *name {
                return Some(child.clone());
            }
        }
        None
    }

    pub fn name(&self) -> Ident {
        self.name.clone()
    }
}

impl std::fmt::Debug for Scope<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scope")
            .field("name", &self.name)
            .field("path", &self.path)
            .field("in_scope", &self.in_scope)
            .finish()
    }
}

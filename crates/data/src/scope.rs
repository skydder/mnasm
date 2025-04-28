use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use crate::{Ident, Path};

#[allow(dead_code)]
pub struct Scope<'code> {
    global: Option<Rc<Scope<'code>>>, // root of the tree
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
        global: Option<Rc<Scope<'code>>>,
        name: Ident,
        is_global: bool,
        is_defined: bool,
        path: Path,
    ) -> Rc<Self> {
        Rc::new(Self {
            global,
            name,
            in_scope: RefCell::new(Vec::new()),
            is_defined: Cell::new(is_defined),
            path,
            is_global,
        })
    }

    fn new_global_root() -> Rc<Self> {
        Self::new(
            None,
            Ident::new("_global".to_string()),
            true,
            true,
            Path::new(Rc::new(Vec::new()), false),
        )
    }

    fn new_local_root(global: Rc<Scope<'code>>) -> Rc<Self> {
        Self::new(
            Some(global),
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
        let global = Self::new_global_root();
        root.add_to_in_scope(global.clone());
        root.add_to_in_scope(Self::new_local_root(global));
        root
    }

    pub fn new_local(
        global: Rc<Scope<'code>>,
        name: Ident,
        is_defined: bool,
        path: Path,
    ) -> Rc<Self> {
        Self::new(Some(global), name, false, is_defined, path)
    }

    pub fn add_to_in_scope(&self, scope: Rc<Scope<'code>>) {
        self.in_scope.borrow_mut().push(scope);
    }

    pub fn has_path_of(self: &Rc<Self>, path: &Path) -> bool {
        if path.is_empty() {
            return false;
        }

        for label in self.in_scope.borrow().iter() {
            if label.name == path.current() {
                if path.is_last() {
                    return true;
                } else {
                    return label.clone().has_path_of(&path.next_path().unwrap());
                }
            }
        }
        let new = Self::new_local(
            self.global.clone().unwrap(),
            path.current(),
            false,
            path.next_path().unwrap().clone(),
        );
        self.add_to_in_scope(new.clone());

        new.has_path_of(&Path::new(Rc::new(Vec::new()), false));
        false
    }

    pub fn get_label(self: &Rc<Self>) -> String {
        let mut label = String::new();
        for ident in self.path.path().iter() {
            label.push('_');
            label.push_str(&ident.get_str());
        }
        label
    }

    pub fn global(self: &Rc<Self>) -> Option<Rc<Self>> {
        self.global.clone()
    }

    pub fn path(self: &Rc<Self>) -> Path {
        self.path.clone()
    }

    pub fn get_child(self: &Rc<Self>, name: &Ident) -> Option<Rc<Self>> {
        eprintln!("{:#?}", self.in_scope);
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

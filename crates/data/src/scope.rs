use std::{cell::RefCell, rc::Rc};

use crate::{Ident, Path};


pub struct Scope<'code> {
    global: Option<Rc<Scope<'code>>>,
    name: Ident<'code>,
    in_scope: RefCell<Vec<Rc<Scope<'code>>>>,
    is_defined: bool,
    path: Path<'code>
}

impl<'code> Scope<'code> {
    pub fn new(global: Rc<Scope<'code>>, name: Ident<'code>, is_defined: bool, path: Path<'code>) -> Rc<Self> {
        Rc::new(Self {
            global: Some(global),
            name,
            in_scope: RefCell::new(Vec::new()),
            is_defined,
            path
        })
    }

    pub fn has_path_of(self: &Rc<Self>, path: &Path<'code>) -> bool {
        for label in self.in_scope.borrow().iter() {
            if label.name == path.current() {
                if path.is_last() {
                    return true;
                } else {
                    return label.clone().has_path_of(&path.next_path().unwrap());
                }
            }
        }
        let new = self.add_new_scope(path.current(), false);
        new.has_path_of(&path.next_path().unwrap());
        false
    }

    pub fn get_label(self: &Rc<Self>) -> String {
        let mut label = String::new();
        for i in self.path.path().iter() {
            label.push_str(i.get_str());
        }
        label
    }
    pub fn add_new_scope(self: &Rc<Self>, name: Ident<'code>, is_defined: bool) -> Rc<Scope<'code>> {
        let mut path = self.path.path().to_vec();
        path.push(name.clone());
        let path = Path::new(name.location(), Rc::new(path), false);
        let new = Scope::new(
            if self.global.is_none() {
                self.clone()
            } else {
                self.global.clone().unwrap()
            },
            name,
            is_defined,
            path
        );
        self.in_scope.borrow_mut().push(new.clone());
        new
    }

    pub fn get_global(self: &Rc<Self>) -> Option<Rc<Self>> {
        self.global.clone()
    }
}
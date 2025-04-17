use std::{cell::RefCell, rc::Rc};

use crate::{Ast, Ident, Path};
use util::AsmError;


pub struct Scope<'code> {
    global: Option<Rc<Scope<'code>>>,
    name: Ident<'code>,
    in_scope: RefCell<Vec<Rc<Scope<'code>>>>,
    is_defined: bool,
}

impl<'code> Scope<'code> {
    pub fn new(global: Rc<Scope<'code>>, name: Ident<'code>, is_defined: bool) -> Rc<Self> {
        Rc::new(Self {
            global: Some(global),
            name,
            in_scope: RefCell::new(Vec::new()),
            is_defined,
        })
    }

    fn new_global(name: Ident<'code>) -> Rc<Self> {
        Rc::new(Self {
            global: None,
            name,
            in_scope: RefCell::new(Vec::new()),
            is_defined: true,
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

    pub fn add_new_scope(self: &Rc<Self>, name: Ident<'code>, is_defined: bool) -> Rc<Scope<'code>> {
        let new = Scope::new(
            if self.global.is_none() {
                self.clone()
            } else {
                self.global.clone().unwrap()
            },
            name,
            is_defined,
        );
        self.in_scope.borrow_mut().push(new.clone());
        new
    }

    pub fn get_global(self: &Rc<Self>) -> Option<Rc<Self>> {
        self.global.clone()
    }
}
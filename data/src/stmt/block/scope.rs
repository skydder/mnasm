use std::{cell::RefCell, rc::Rc};

use crate::{stmt::Macro, Ident, Path};

use super::Scope;

impl<'a> Scope<'a> {
    pub fn new(scope_name: Option<Ident<'a>>, parent: Option<Rc<RefCell<Scope<'a>>>>) -> Self {
        let scope_name = if scope_name.is_none() {
            Ident::new("N_L_L")
        } else {
            scope_name.unwrap()
        };
        Self {
            scope_name: scope_name,
            parent: parent.clone(),
            labels: RefCell::new(Vec::new()),
            macros: RefCell::new(Vec::new()),
            path_name: parent.map_or(format!("{}", scope_name), |p| format!("{}__{}", p.borrow().path_name.clone(), scope_name))
        }
    }

    pub fn add_label(&self, label: Ident<'a>) {
        self.labels.borrow_mut().push(label);
    }

    pub fn add_macro(&self, label: Ident<'a>, macros: Rc<Macro<'a>>) {
        self.labels.borrow_mut().push(label);
        self.macros.borrow_mut().push((label, macros));
    }

    pub fn find_macro(&self, label: Ident<'a>) -> Option<Rc<Macro<'a>>> {
        for (l, m) in self.macros.borrow().iter() {
            if label == *l {
                return Some(m.clone());
            }
        }
        if let Some(p) = &self.parent {
            return p.borrow().find_macro(label);
        }
        None
    }

    pub fn add_label_to_root(&self, label: Ident<'a>) {
        let parent: &mut Option<Rc<RefCell<Scope<'a>>>> = &mut self.parent_scope();
        if self.is_root() {
            self.add_label(label);
            return;
        }
        while parent
            .clone()
            .is_some_and(|s| !(s.clone().borrow().is_root()))
        {
            parent.replace(parent.clone().unwrap().borrow().parent_scope().unwrap());
        }
        parent.clone().unwrap().borrow_mut().add_label(label);
    }

    fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    fn parent_scope(&self) -> Option<Rc<RefCell<Scope<'a>>>> {
        self.parent.clone()
    }

    pub fn find_label(&self, label: &Path<'a>) -> Option<String> {
        for l in self.labels.borrow().iter() {
            if label.path_name() == self.gen_label(*l) {
                return Some(label.path_name());
            }
        }
        if let Some(p) = &self.parent {
            return p.borrow().find_label(label);
        }
        None
    }

    pub fn scope_name(&self) -> Ident<'a> {
        self.scope_name
    }

    pub fn gen_label(&self, label: Ident<'a>) -> String {
        format!("{}__{}", self.path_name, label)
    }

}

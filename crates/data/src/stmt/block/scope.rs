use std::{cell::RefCell, rc::Rc};

use crate::{stmt::Macro, Ident, Path};

use super::Scope;

impl<'a> Scope<'a> {
    pub fn new(scope_name: Option<Ident<'a>>, parent: Option<Rc<RefCell<Scope<'a>>>>) -> Self {
        let scope_name = if let Some(s) = scope_name {
            s
        } else if parent.is_none() {
            Ident::new("")
        } else {
            // Ident::new("N_L_L")
            Ident::new_unnamed()
        };
        Self {
            scope_name,
            parent: parent.clone(),
            labels: RefCell::new(Vec::new()),
            macros: RefCell::new(Vec::new()),
            path_name: parent.filter(|p| !p.borrow().path_name.is_empty()).map_or_else(
                || format!("{}", scope_name),
                |p| format!("{}__{}", p.borrow().path_name.clone(), scope_name),
            ),
        }
    }

    pub fn add_label(&self, label: Ident<'a>, scope: Option<Rc<RefCell<Scope<'a>>>>) {
        self.labels.borrow_mut().push((label, scope));
    }

    pub fn add_macro(&self, label: Ident<'a>, macros: Rc<Macro<'a>>) {
        // eprintln!("{:#?}", (label, macros.clone()));
        self.labels.borrow_mut().push((label, None));
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

    fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    fn parent_scope(&self) -> Option<Rc<RefCell<Scope<'a>>>> {
        self.parent.clone()
    }

    fn get_root(&self) -> Option<Rc<RefCell<Scope<'a>>>> {
        let parent: &mut Option<Rc<RefCell<Scope<'a>>>> = &mut self.parent_scope();
        if self.is_root() {
            return None;
        }
        while parent
            .clone()
            .is_some_and(|s| !(s.clone().borrow().is_root()))
        {
            parent.replace(parent.clone().unwrap().borrow().parent_scope().unwrap());
        }
        Some(parent.clone().unwrap())
    }

    pub fn add_label_to_root(&self, label: Ident<'a>) {
        let parent: &mut Option<Rc<RefCell<Scope<'a>>>> = &mut self.parent_scope();
        if self.is_root() {
            self.add_label(label, None);
            return;
        }
        while parent
            .clone()
            .is_some_and(|s| !(s.clone().borrow().is_root()))
        {
            parent.replace(parent.clone().unwrap().borrow().parent_scope().unwrap());
        }
        parent.clone().unwrap().borrow_mut().add_label(label, None);
    }

    fn find_label_from_root(&self, label: &Path<'a>) -> Option<String> {
        let parent = self.get_root();
        if parent.is_none() {
            if self._find_label(label) {
                Some(label.path_name())
            } else {
                None
            }
        } else if parent.unwrap().borrow()._find_label(label) {
            Some(label.path_name())
        } else {
            None
        }
    }

    fn _find_label(&self, label: &Path<'a>) -> bool {
        let (p, rest) = if let Some(data) = label.split() {
            data
        } else {
            return false;
        };
        for (l, s) in self.labels.borrow().iter() {
            if p == *l {
                if rest.split().is_none() {
                    return true;
                } else if s.is_none() {
                    return false;
                }
                return s.clone().unwrap().borrow()._find_label(&rest);
            }
        }
        false
    }

    pub fn find_label_local(&self, label: &Path<'a>) -> Option<String> {
        if self._find_label(label) {
            return Some(format!("{}__{}", self.path_name, label.path_name()));
        }
        None
    }

    pub fn find_label(&self, label: &Path<'a>) -> Option<String> {
        if label.is_relative() {
            self.find_label_local(label)
        } else {
            self.find_label_from_root(label)
        }
    }
    pub fn scope_name(&self) -> Ident<'a> {
        self.scope_name
    }

    pub fn gen_label(&self, label: Ident<'a>) -> String {
        if self.is_root() {
            return format!("{}", label);
        }
        format!("{}__{}", self.path_name, label)
    }
}

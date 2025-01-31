use std::{cell::RefCell, rc::Rc};

use crate::{stmt::Macro, Ident};

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
            parent: parent,
            labels: RefCell::new(Vec::new()),
            macros: RefCell::new(Vec::new()),
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

    pub fn find_label(&self, label: Ident<'a>) -> Option<String> {
        for l in self.labels.borrow().iter() {
            if label == *l {
                return Some(self.gen_label(label, false));
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

    pub fn gen_label(&self, label: Ident<'a>, is_global: bool) -> String {
        let mut l = String::new();
        if !is_global {
            self.gen_label_scope(&mut l);
            l.push_str("__");
            l.push_str(&label.get());
        } else {
            l.push_str(&label.get());
        }
        
        l
    }

    pub fn gen_label_scope(&self, name: &mut String) {
        // if self.parent.is_some() {
        //     format!(
        //         "{}__{}",
        //         self.parent.clone().unwrap().borrow().gen_label_scope(),
        //         self.scope_name().get(),
        //     )
        // } else {
        //     String::new()
        // }
        if self.parent.is_none() {
            return ;
        } else {
            self.parent.clone().unwrap().borrow().gen_label_scope(name);
            name.push_str("__");
            name.push_str(&self.scope_name().get());
        }
    }
}

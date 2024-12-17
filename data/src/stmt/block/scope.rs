use std::{cell::RefCell, rc::Rc};

use crate::{Ident, Label};

use super::Scope;

impl<'a> Scope<'a> {
    pub fn new(scope_name: Option<Ident<'a>>, parent: Option<Rc<RefCell<Scope<'a>>>>) -> Self {
        Self {
            scope_name: scope_name,
            parent: parent,
            labels: RefCell::new(Vec::new()),
        }
    }

    pub fn add_label(&self, label: Ident<'a>) {
        self.labels.borrow_mut().push(label);
    }

    pub fn find_label(&self, label: Ident<'a>) -> Option<String> {
        for l in self.labels.borrow().iter() {
            if label == *l {
                return Some(self.gen_label(label));
            }
        }
        if let Some(p) = &self.parent {
            return p.borrow().find_label(label);
        }
        None
    }

    pub fn scope_name(&self) -> Ident<'a> {
        self.scope_name.unwrap_or(Ident::new(""))
    }

    pub fn gen_label(&self, label: Ident<'a>) -> String {
        let mut l = String::new();
        l.push_str(label.get());
        l.push_str(&self.gen_label_scope());
        l
    }

    pub fn gen_label_scope(&self) -> String {
        if self.parent.is_some() {
            format!(
                "{}__{}",
                self.scope_name().get(),
                self.parent.clone().unwrap().borrow().gen_label_scope()
            )
        } else {
            String::new()
        }
    }
}

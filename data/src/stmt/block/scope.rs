use std::{cell::RefCell, rc::Rc};

use crate::Name;

use super::Scope;

impl<'a> Scope<'a> {
    pub fn new(scope_name: Option<Name<'a>>, parent: Option<Rc<RefCell<Scope<'a>>>>) -> Self {
        Self {
            scope_name: scope_name,
            parent: parent,
            labels: RefCell::new(Vec::new()),
        }
    }

    pub fn add_label(&self, label: Name<'a>) {
        self.labels.borrow_mut().push(label);
    }
}

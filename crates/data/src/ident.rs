use std::rc::Rc;

use util::Location;

#[derive(Clone, PartialEq)]
pub struct Ident<'code> {
    location: Location<'code>,
    label: Rc<String>,
}

impl<'code> Ident<'code> {
    pub fn new(label: Rc<String>, location: Location<'code>) -> Self {
        Self { location, label}
    }
    pub fn location(&self) -> Location<'code> {
        self.location.clone()
    }
}

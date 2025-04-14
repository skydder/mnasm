use std::rc::Rc;

use util::Location;
#[derive(Debug)]

pub struct Strings<'code> {
    location: Location<'code>,
    string: Rc<String>,
}

impl<'code> Strings<'code> {
    pub fn new(string: Rc<String>, location: Location<'code>) -> Self {
        Self { location, string }
    }
    pub fn location(&self) -> Location<'code> {
        self.location.clone()
    }

    pub fn get_str(&self) -> &str {
        self.string.as_str()
    }
}

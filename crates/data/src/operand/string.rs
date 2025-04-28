use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Strings {
    string: Rc<String>,
}

impl Strings {
    pub fn new(string: Rc<String>) -> Self {
        Self { string }
    }

    pub fn get_str(&self) -> &str {
        self.string.as_str()
    }
}

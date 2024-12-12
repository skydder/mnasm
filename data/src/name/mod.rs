#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Name<'a> {
    name: &'a str
}

impl<'a> Name<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name: name }
    }

    pub fn get(&self) -> &str {
        self.name
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Ident<'a> {
    name: &'a str,
}

impl<'a> Ident<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name: name }
    }

    pub fn get(&self) -> &str {
        &self.name[0..]
    }
}

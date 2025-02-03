#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Ident<'a> {
    name: &'a str,
}

impl<'a> Ident<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name: name,
        }
    }

    pub fn get(&self) -> String {
        format!("{}", &self.name[0..])
    }
}

impl<'a> std::fmt::Display for Ident<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

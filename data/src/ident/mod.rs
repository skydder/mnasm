#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Ident<'a> {
    name: &'a str,
    with_dot: bool,
}

impl<'a> Ident<'a> {
    pub fn new(name: &'a str, with_dot: bool) -> Self {
        Self { name: name , with_dot: with_dot}
    }

    pub fn get(&self) -> String {
        if self.with_dot {
            format!(".{}", &self.name[0..])
        } else {
            format!("{}", &self.name[0..])
        }
    }
}

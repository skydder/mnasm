use util::Location;

use super::Operand;

#[derive(Debug)]
pub struct Label<'a> {
    name: &'a str,
    pub location: Location<'a>
}

impl<'a> Label<'a> {
    pub fn new(name: &'a str, location: Location<'a>) -> Self {
        Self { name: name, location: location }
    }
}

impl<'a> Operand for Label<'a> {
    fn codegen(&self) -> String {
        format!("{}", self.name)        
    }

    fn size(&self) -> usize {
        64
    }
}
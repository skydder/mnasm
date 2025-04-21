use util::Location;

use super::Operand;

#[derive(Debug)]
pub struct Immediate<'code> {
    location: Location<'code>,
    pub data: u64,
    pub signed: bool,
}

impl<'code> Immediate<'code> {
    pub fn location(&self) -> Location<'code> {
        self.location.clone()
    }

    pub fn new(location: Location<'code>, data: u64, signed: bool) -> Self {
        Self {
            location,
            data,
            signed,
        }
    }
}

impl Operand for Immediate<'_> {}

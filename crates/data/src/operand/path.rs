use util::Location;

use crate::ident::Ident;

use super::Operand;

pub struct Path<'code> {
    is_relative: bool,
    path: Vec<Ident<'code>>,
    location: Location<'code>,
}

#[allow(clippy::needless_lifetimes)]
impl<'code> Path<'code> {
    pub fn is_relative(&self) -> bool {
        self.is_relative
    }

    pub fn path(&self) -> Vec<Ident<'code>> {
        self.path.clone()
    }

    pub fn location(&self) -> Location<'code> {
        self.location.clone()
    }
}

impl Operand for Path<'_> {}
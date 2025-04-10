use std::rc::Rc;

use util::Location;

use crate::ident::Ident;

use super::Operand;

#[derive(Debug)]
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

    pub fn new(
        location: Location<'code>,
        path: Vec<Ident<'code>>,
        is_relative: bool,
    ) -> Self {
        if path.is_empty() {
            unreachable!()
        }
        Self { is_relative, path, location }
    }

    pub fn next_path(&self) -> Option<Rc<Self>> {
        if self.path.len() > 1 {
            Some(Rc::new(Self { is_relative: self.is_relative, path: self.path[1..].to_vec(), location: self.path.get(1).unwrap().location() }))
        } else {
            None
        }
    }

    pub fn current(&self) -> Ident<'code> {
        self.path.first().unwrap().clone()
    }

    pub fn is_last(&self) -> bool {
        self.path.len() == 1
    }
}

impl Operand for Path<'_> {}
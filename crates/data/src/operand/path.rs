use std::{cell::RefCell, rc::Rc};

use util::Location;

use crate::ident::Ident;

use super::Operand;

#[derive(Debug, Clone)]
pub struct Path<'code> {
    is_relative: bool,
    path: Rc<Vec<Ident<'code>>>,
    location: Location<'code>,
    name: RefCell<String>
}

#[allow(clippy::needless_lifetimes)]
impl<'code> Path<'code> {
    pub fn is_relative(&self) -> bool {
        self.is_relative
    }

    pub fn path(&self) -> Rc<Vec<Ident<'code>>> {
        self.path.clone()
    }

    pub fn location(&self) -> Location<'code> {
        self.location.clone()
    }

    pub fn new(location: Location<'code>, path: Rc<Vec<Ident<'code>>>, is_relative: bool) -> Self {
        if path.is_empty() {
            unreachable!()
        }
        Self {
            is_relative,
            path,
            location,
            name: RefCell::new(String::new()),
        }
    }

    pub fn next_path(&self) -> Option<Rc<Self>> {
        if self.path.len() > 1 {
            Some(Rc::new(Self {
                is_relative: self.is_relative,
                path: Rc::new(self.path[1..].to_vec()),
                location: self.path.get(1).unwrap().location(),
                name: RefCell::new(String::new())
            }))
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

    pub fn len(&self) -> usize {
        self.path.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Operand for Path<'_> {}
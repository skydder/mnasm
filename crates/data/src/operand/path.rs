use std::rc::Rc;

use crate::ident::Ident;

use super::Operand;

#[derive(Debug, Clone)]
pub struct Path {
    is_relative: bool,
    path: Rc<Vec<Ident>>,
}

#[allow(clippy::needless_lifetimes)]
impl Path {
    pub fn is_relative(&self) -> bool {
        self.is_relative
    }

    pub fn path(&self) -> Rc<Vec<Ident>> {
        self.path.clone()
    }

    pub fn new(path: Rc<Vec<Ident>>, is_relative: bool) -> Self {
        // if path.is_empty() {
        //     unreachable!()
        // }
        Self { is_relative, path }
    }

    pub fn next_path(&self) -> Option<Self> {
        if self.path.len() >= 1 {
            Some(Self {
                is_relative: self.is_relative,
                path: Rc::new(self.path[1..].to_vec()),
            })
        } else {
            None
        }
    }

    pub fn current(&self) -> Ident {
        self.path
            .first()
            .expect("failed when using Path::current")
            .clone()
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

impl Operand for Path {}

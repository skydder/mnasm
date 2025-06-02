use std::{rc::Rc, thread::panicking};

use crate::ident::{self, Ident};

use super::Operand;

#[derive(Debug, Clone, Copy)]
pub enum PathState {
    Relative,
    Absolute,
    GlobalRelative,
}

#[derive(Debug, Clone)]
pub struct Path {
    state: PathState,
    path: Rc<Vec<Ident>>,
}

#[allow(clippy::needless_lifetimes)]
impl Path {
    pub fn is_relative(&self) -> bool {
        matches!(self.state, PathState::Relative)
    }

    pub fn path(&self) -> Rc<Vec<Ident>> {
        self.path.clone()
    }

    pub fn new(path: Rc<Vec<Ident>>, state: PathState) -> Self {
        // if path.is_empty() {
        //     unreachable!()
        // }
        Self { state, path }
    }

    pub fn append(&self, name: Ident) -> Self {
        let mut path = self.path.to_vec();
        path.push(name);
        Self::new(Rc::new(path), self.state)
    }

    pub fn next_path(&self) -> Option<Self> {
        if self.path.len() > 1 {
            Some(Self::new(Rc::new(self.path[1..].to_vec()), self.state))
        } else {
            None
        }
    }
    pub fn state(&self) -> PathState {
        self.state
    }

    pub fn get(&self, nth: usize) -> Option<Ident> {
        self.path.get(nth).cloned()
    }

    pub fn last(&self) -> Ident {
        self.path
            .last()
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

    pub fn diff(&self, path: Path) -> Path {
        let mut s = 0;
        for (i, now) in self.path.iter().enumerate() {
            if path.get(i).is_some_and(|i| i == *now) {
                continue;
            }
            s = i;
        }
        Path::new(Rc::new(self.path[s..].to_vec()), PathState::Relative)
    }

    pub fn absolutify(&self, path: Path) -> Path {
        if matches!(self.state, PathState::Relative) {
            Path::new(
                Rc::new([path.path.to_vec(), self.path.to_vec()].concat()),
                PathState::Absolute,
            )
        } else {
            self.clone()
        }
    }

    pub fn labelify(&self) -> String {
        let mut code = String::new();
        for ident in self.path.iter() {
            code.push('_');
            code.push_str(&ident.get_str());
        }
        code
    }
}

impl std::iter::IntoIterator for Path {
    type Item = Ident;

    type IntoIter = std::vec::IntoIter<Ident>;

    fn into_iter(self) -> Self::IntoIter {
        <std::vec::Vec<ident::Ident> as Clone>::clone(&self.path).into_iter()
    }
}

impl Operand for Path {}

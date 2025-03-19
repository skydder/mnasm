use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use super::AST;

#[derive(Debug, PartialEq)]
pub struct DSLFn {
    pub body: AST,
    pub params: Vec<AST>, // this should be the list of symbol
}

#[derive(Debug, Clone, PartialEq)]
pub enum Data {
    Integer(Rc<Cell<i64>>),
    String(Rc<RefCell<String>>),
    Symbol(Rc<String>),
    List(Rc<RefCell<Vec<Data>>>),
    Fn(Rc<DSLFn>),
    None,
}

impl Data {
    pub fn get_string(&self) -> Option<Rc<RefCell<String>>> {
        match self {
            Data::String(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn get_symbol(&self) -> Option<Rc<String>> {
        match self {
            Data::Symbol(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn get_list(&self) -> Option<Rc<RefCell<Vec<Data>>>> {
        match self {
            Data::List(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn get_integer(&self) -> Option<Rc<Cell<i64>>> {
        match self {
            Data::Integer(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn get_fn(&self) -> Option<Rc<DSLFn>> {
        match self {
            Data::Fn(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.get_integer().is_some_and(|i| i.get() == 0)
    }
}

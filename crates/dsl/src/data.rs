use std::rc::Rc;

use util::Token;

use super::AST;

#[derive(Debug, PartialEq)]
pub struct DSLFn<'a> {
    pub body: AST<'a>,
    pub params: Vec<AST<'a>>, // this should be the list of symbol
}

#[derive(Debug, Clone, PartialEq)]
pub enum Data<'a> {
    Integer(i64),
    String(Rc<String>),
    Symbol(Rc<String>),
    List(Rc<Vec<Data<'a>>>),
    Fn(Rc<DSLFn<'a>>),
    AsmToken(Token<'a>),
    None,
}

impl<'a> Data<'a> {
    pub fn get_string(&self) -> Option<Rc<String>> {
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

    pub fn get_list(&self) -> Option<Rc<Vec<Data<'a>>>> {
        match self {
            Data::List(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn get_list_nth(&self, nth: usize) -> Option<Data<'a>> {
        match self {
            Data::List(l) => l.clone().get(nth).cloned(),
            Data::String(s) => Some(Data::String(Rc::new(s.chars().nth(nth)?.to_string()))),
            _ => None,
        }
    }

    pub fn get_list_last(&self) -> Option<Data<'a>> {
        match self {
            Data::List(l) => l.clone().last().cloned(),
            Data::String(s) => Some(Data::String(Rc::new(s.chars().last()?.to_string()))),
            _ => None,
        }
    }

    pub fn get_integer(&self) -> Option<i64> {
        match self {
            Data::Integer(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn get_fn(&self) -> Option<Rc<DSLFn<'a>>> {
        match self {
            Data::Fn(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.get_integer().is_some_and(|i| i == 0)
    }
}

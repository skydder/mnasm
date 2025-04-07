use std::rc::Rc;

use data::Object;
use util::Token;

use crate::asm_tokenizer::TKNZR4ASM;

use super::AST;

#[derive(Debug, PartialEq)]
pub struct DSLFn<'a> {
    pub body: AST<'a>,
    pub params: Vec<AST<'a>>, // this should be the list of symbol
}

#[derive(Clone, Debug)]
pub enum Data<'a> {
    Integer(i64),
    String(Rc<String>),
    Symbol(Rc<String>),
    List(Rc<Vec<Data<'a>>>),
    Fn(Rc<DSLFn<'a>>),
    AsmTokenizer(Rc<TKNZR4ASM<'a>>),
    AsmToken(Token<'a>),
    AsmData(Rc<dyn Object + 'a>),
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
            Data::Integer(s) => Some(*s),
            _ => None,
        }
    }

    pub fn get_fn(&self) -> Option<Rc<DSLFn<'a>>> {
        match self {
            Data::Fn(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn get_tokenizer(&self) -> Option<Rc<TKNZR4ASM<'a>>> {
        match self {
            Data::AsmTokenizer(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.get_integer().is_some_and(|i| i == 0)
    }
}

impl std::cmp::PartialEq for Data<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Symbol(l0), Self::Symbol(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Fn(l0), Self::Fn(r0)) => l0 == r0,
            (Self::AsmTokenizer(l0), Self::AsmTokenizer(r0)) => l0 == r0,
            (Self::AsmToken(l0), Self::AsmToken(r0)) => l0 == r0,
            (Self::AsmData(_), Self::AsmData(_)) => false, //todo
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

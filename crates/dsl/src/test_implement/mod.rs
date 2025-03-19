use std::{cell::RefCell, collections::HashMap, rc::Rc};

mod data;
mod eval;

use data::Data;
use util::Source2;

use crate::{DSLError, DSLResult};

#[derive(Clone)]
pub struct Environment<'a> {
    local: RefCell<HashMap<String, Data>>,
    global: Rc<RefCell<HashMap<String, Data>>>,
    source: Rc<Source2<'a>>
}

impl<'a> Environment<'a> {
    pub fn new(source: Source2<'a>) -> Self {
        Self {
            local: RefCell::new(HashMap::new()),
            global: Rc::new(RefCell::new(HashMap::new())),
            source: Rc::new(source),
        }
    }
    pub fn get_variable(&self, name: &str) -> DSLResult<Data> {
        match name {
            n if self.global.borrow().contains_key(n) => {
                self.global.borrow().get(&n.to_string()).cloned()
            }
            n if self.local.borrow().contains_key(n) => {
                self.local.borrow().get(&n.to_string()).cloned()
            }
            _ => None,
        }
        .ok_or(DSLError::Eval(format!("{} is unfdefined", name)))
    }

    pub fn push_var(&self, name: String, constant: Data) {
        self.local.borrow_mut().insert(name, constant);
    }

    pub fn push_global(&self, name: String, constant: Data) {
        self.global.borrow_mut().insert(name, constant);
    }

    pub fn enter_fn(&self) -> Environment<'a>{
        Environment {
            local: RefCell::new(HashMap::new()),
            global: self.global.clone(),
            source: self.source.clone()
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    AddAssign,
    Add,
    CmpEqual,
    CmpLessThan,
    CmpNoMoreThan,
    Break,
    List,
    LOr,
    LAnd,
    Mul,
    MulAssign,
    FnCall, // fn_name goes to lhs and args go to rhs as list
}

#[derive(Clone, Debug)]
pub enum AST {
    Expr(
        Operator,
        Rc<AST>,
        Option<Rc<AST>>, // the reason type(rhs) == Option is for unary op
    ),
    Data(Rc<Data>),
    List(Rc<Vec<AST>>),
}

impl AST {
    pub fn get_data(&self) -> Option<Data> {
        match self {
            AST::Data(constant) => Some(constant.as_ref().clone()),
            _ => None,
        }
    }

    pub fn get_list(&self) -> Option<Rc<Vec<AST>>> {
        match self {
            AST::List(l) => Some(l.clone()),
            _ => None,
        }
    }
}
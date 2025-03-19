use std::{cell::RefCell, rc::Rc};

use crate::{AST, Environment};

// ===|todo|===
// - merge Variable and Constant into Data.

#[derive(Clone, Debug)]
pub enum Variable {
    String(RefCell<String>),
    List(RefCell<Vec<Variable>>),
    Integer(RefCell<i64>),
    Fn(Vec<AST>, AST),
}

impl Variable {
    pub fn add<'a>(&self, rhs: &Constant) {
        match (self, rhs) {
            (Variable::String(s1), Constant::String(s2)) => {
                s1.borrow_mut().push_str(s2);
            }
            (Variable::Integer(s1), Constant::Integer(s2)) => {
                *s1.borrow_mut() += *s2;
            }
            (Variable::List(s1), _) => s1.borrow_mut().push(rhs.to_type_evaled().unwrap()),
            _ => {
                todo!()
            }
        }
    }

    pub fn mul<'a>(&self, rhs: &Constant) {
        match (self, rhs) {
            (Variable::Integer(s1), Constant::Integer(s2)) => {
                *s1.borrow_mut() *= *s2;
            }
            _ => {
                todo!()
            }
        }
    }

    pub fn to_constant(&self) -> Constant {
        match self {
            Variable::String(ref_cell) => Constant::String(ref_cell.borrow().clone()),
            Variable::List(ref_cell) => Constant::List(
                ref_cell
                    .borrow()
                    .iter()
                    .map(|i| Rc::new(i.to_constant()))
                    .collect::<Vec<Rc<Constant>>>()
                    .clone(),
            ),
            Variable::Integer(ref_cell) => Constant::Integer(ref_cell.borrow().clone()),
            _ => todo!(),
        }
    }

    pub fn get_string(&self) -> Option<String> {
        match self {
            Variable::String(str) => Some(str.borrow().to_string()),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Constant {
    Integer(i64),
    String(String),
    List(Vec<Rc<Constant>>),
    Symbol(String),
    None,
}
impl Constant {
    pub fn get(&self, env: &Environment) -> Option<Rc<Variable>> {
        match self {
            Self::Symbol(name) => env.get_variable(name).ok(),
            _ => None,
        }
    }

    pub fn get_symbol(&self) -> Option<String> {
        match self {
            Self::Symbol(name) => Some(name.clone()),
            _ => None,
        }
    }

    pub fn to_type(&self, env: &Environment) -> Option<Variable> {
        match self {
            Constant::Integer(i) => Some(Variable::Integer(RefCell::new(*i))),
            Constant::String(s) => Some(Variable::String(RefCell::new(s.clone()))),
            Constant::List(constants) => {
                let mut ls = Vec::new();
                for i in constants {
                    ls.push(i.to_type(env)?);
                }
                Some(Variable::List(RefCell::new(ls)))
            }
            Constant::Symbol(_) => self.get(env).and_then(|s| Some(s.as_ref().clone())),
            Constant::None => None,
        }
    }

    pub fn to_type_evaled(&self) -> Option<Variable> {
        match self {
            Constant::Integer(i) => Some(Variable::Integer(RefCell::new(*i))),
            Constant::String(s) => Some(Variable::String(RefCell::new(s.clone()))),
            Constant::List(constants) => {
                let mut ls = Vec::new();
                for i in constants {
                    ls.push(i.to_type_evaled()?);
                }
                Some(Variable::List(RefCell::new(ls)))
            }
            Constant::Symbol(_) => None,
            Constant::None => None,
        }
    }

    pub fn _index(&self, nth: usize) -> Option<Rc<Constant>> {
        match self {
            Constant::List(lhs) => Some(lhs.get(nth)?.clone()),
            Constant::String(lhs) => {
                Some(Rc::new(Constant::String(lhs.chars().nth(nth)?.to_string())))
            }
            _ => None,
        }
    }

    pub fn _slice(&self, begin: usize, end: usize) -> Option<Rc<Constant>> {
        match self {
            Constant::List(lhs) => Some(Rc::new(Constant::List(
                lhs.get(begin..end)?
                    .iter()
                    .map(|i| i.clone())
                    .collect::<Vec<Rc<Constant>>>(),
            ))),
            Constant::String(lhs) => Some(Rc::new(Constant::String(
                lhs.get(begin..end)?.chars().collect::<String>(),
            ))),
            _ => None,
        }
    }

    pub fn is_zero(&self) -> bool {
        matches!(self, Self::Integer(0))
    }

    pub fn get_integer(&self) -> Option<i64> {
        match self {
            Self::Integer(i) => Some(*i),
            _ => None,
        }
    }

    pub fn get_string(&self) -> Option<String> {
        match self {
            Self::String(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn tail_of_list(&self) -> Rc<Constant> {
        match self {
            Constant::List(constants) => {
                constants.last().unwrap_or(&Rc::new(Constant::None)).clone()
            }
            _ => Rc::new(Constant::None),
        }
    }
}
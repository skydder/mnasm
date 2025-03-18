use std::{cell::RefCell, collections::HashMap, rc::Rc};

use eval::{eval, run};
use util::{Location, Source, Source2, Stream};

mod errors;
pub use errors::{DSLError, DSLResult};

mod tokenizer;
pub use tokenizer::{consume_token, peek_token, tokenize, KeyWord, Token};

mod parser;
pub use parser::parse;

mod eval;
// Constant ()
// - dsl code
// - raw stream
// - source info

// ====
// ++todo++
// enable dsl to use parser
// ====

pub struct DSLConstant<'a> {
    source: Source2<'a>,
    input: String,
}

impl<'a> DSLConstant<'a> {
    fn new(source: Source2<'a>, raw_stream: String) -> Self {
        Self {
            source: source,
            input: raw_stream,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Variable {
    String(RefCell<String>),
    List(RefCell<Vec<Variable>>),
    Integer(RefCell<i64>),
    Fn(Vec<AST>, AST),
}

impl Variable {
    fn add<'a>(&self, rhs: &Constant) {
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

    fn mul<'a>(&self, rhs: &Constant) {
        match (self, rhs) {
            (Variable::Integer(s1), Constant::Integer(s2)) => {
                *s1.borrow_mut() *= *s2;
            }
            _ => {
                todo!()
            }
        }
    }

    fn to_constant(&self) -> Constant {
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

    fn get_string(&self) -> Option<String> {
        match self {
            Variable::String(str) => Some(str.borrow().to_string()),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct Environment {
    local: RefCell<HashMap<String, Rc<Variable>>>,
    global: Rc<RefCell<HashMap<String, Rc<Variable>>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            local: RefCell::new(HashMap::new()),
            global: Rc::new(RefCell::new(HashMap::new())),
        }
    }
    pub fn get_variable(&self, name: &str) -> DSLResult<Rc<Variable>> {
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

    pub fn push_var(&self, name: String, constant: Rc<Variable>) {
        self.local.borrow_mut().insert(name, constant);
    }

    pub fn push_global(&self, name: String, constant: Rc<Variable>) {
        self.global.borrow_mut().insert(name, constant);
    }

    pub fn enter_fn(&self) -> Environment {
        Environment {
            local: RefCell::new(HashMap::new()),
            global: self.global.clone(),
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
    fn get(&self, env: &Environment) -> Option<Rc<Variable>> {
        match self {
            Self::Symbol(name) => env.get_variable(name).ok(),
            _ => None,
        }
    }

    fn get_symbol(&self) -> Option<String> {
        match self {
            Self::Symbol(name) => Some(name.clone()),
            _ => None,
        }
    }

    fn to_type(&self, env: &Environment) -> Option<Variable> {
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

    fn to_type_evaled(&self) -> Option<Variable> {
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

    fn _index(&self, nth: usize) -> Option<Rc<Constant>> {
        match self {
            Constant::List(lhs) => Some(lhs.get(nth)?.clone()),
            Constant::String(lhs) => {
                Some(Rc::new(Constant::String(lhs.chars().nth(nth)?.to_string())))
            }
            _ => None,
        }
    }

    fn _slice(&self, begin: usize, end: usize) -> Option<Rc<Constant>> {
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

    fn is_zero(&self) -> bool {
        matches!(self, Self::Integer(0))
    }

    fn get_integer(&self) -> Option<i64> {
        match self {
            Self::Integer(i) => Some(*i),
            _ => None,
        }
    }

    fn tail_of_list(&self) -> Rc<Constant> {
        match self {
            Constant::List(constants) => {
                constants.last().unwrap_or(&Rc::new(Constant::None)).clone()
            }
            _ => Rc::new(Constant::None),
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
    Data(Rc<Constant>),
    List(Vec<AST>),
}

impl AST {
    pub fn get_data(&self) -> Option<Rc<Constant>> {
        match self {
            AST::Data(constant) => Some(constant.clone()),
            _ => None,
        }
    }

    pub fn eval(&self, env: Rc<Environment>) -> DSLResult<Rc<Constant>> {
        eval(self, env) // todo
    }

    pub fn eval_list_nth(&self, env: Rc<Environment>, nth: usize) -> DSLResult<Rc<Constant>> {
        match self {
            AST::List(list) => list
                .get(nth)
                .ok_or(DSLError::Eval(format!("index range error")))?
                .eval(env),
            _ => Err(DSLError::Eval(format!(
                "expected list, but this has other type"
            ))),
        }
    }
    pub fn get_list_nth(&self, nth: usize) -> DSLResult<AST> {
        match self {
            AST::List(list) => Ok(list
                .get(nth)
                .ok_or(DSLError::Eval(format!("index range error")))?
                .clone()),
            _ => Err(DSLError::Eval(format!(
                "expected list, but this has other type"
            ))),
        }
    }
}

pub fn read_stream<'a>(stream: Stream<'a>) -> DSLConstant<'a> {
    let new = DSLConstant::new(stream.source(), stream.stringfiy().to_string());
    new
}

// todo: remove used stream in Source2
pub fn eval_macro<'a>(constant: DSLConstant<'a>, ast: AST) -> Stream<'a> {
    let env = Rc::new(Environment::new());
    let output = run(&ast, env.clone(), constant.input).unwrap();
    let begin = Location::new_source(constant.source, Source::new(output, "macro"));
    let end = begin.end();
    Stream::new(begin, end)
}

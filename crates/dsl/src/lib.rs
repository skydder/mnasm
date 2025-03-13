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
// data ()
// - dsl code
// - raw stream
// - source info

// ====
// ++todo++
// Data -> Constant
// Types -> Variable
// move Fn to Constantj from Types
// ====

pub struct DSLData<'a> {
    source: Source2<'a>,
    input: String,
}

impl<'a> DSLData<'a> {
    fn new(source: Source2<'a>, raw_stream: String) -> Self {
        Self {
            source: source,
            input: raw_stream,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Types {
    String(RefCell<String>),
    List(RefCell<Vec<Types>>),
    Integer(RefCell<i64>),
    Fn(Vec<AST>, AST),
}

impl Types {
    fn add<'a>(&self, rhs: &Data) {
        match (self, rhs) {
            (Types::String(s1), Data::String(s2)) => {
                s1.borrow_mut().push_str(s2);
            }
            _ => {
                todo!()
            }
        }
    }

    fn to_data(&self) -> Data {
        match self {
            Types::String(ref_cell) => Data::String(ref_cell.borrow().clone()),
            Types::List(ref_cell) => Data::List(
                ref_cell
                    .borrow()
                    .iter()
                    .map(|i| Rc::new(i.to_data()))
                    .collect::<Vec<Rc<Data>>>()
                    .clone(),
            ),
            Types::Integer(ref_cell) => Data::Integer(ref_cell.borrow().clone()),
            _ => todo!(),
        }
    }

    fn get_string(&self) -> Option<String> {
        match self {
            Types::String(str) => Some(str.borrow().to_string()),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct Environment {
    local: RefCell<HashMap<String, Rc<Types>>>,
    global: Rc<RefCell<HashMap<String, Rc<Types>>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            local: RefCell::new(HashMap::new()),
            global: Rc::new(RefCell::new(HashMap::new())),
        }
    }
    pub fn get_variable(&self, name: &str) -> DSLResult<Rc<Types>> {
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

    pub fn push_var(&self, name: String, data: Rc<Types>) {
        self.local.borrow_mut().insert(name, data);
    }

    pub fn push_global(&self, name: String, data: Rc<Types>) {
        self.global.borrow_mut().insert(name, data);
    }

    pub fn enter_fn(&self) -> Environment {
        Environment {
            local: RefCell::new(HashMap::new()),
            global: self.global.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Data {
    Integer(i64),
    String(String),
    List(Vec<Rc<Data>>),
    Symbol(String),
    None,
}
impl Data {
    fn get(&self, env: &Environment) -> Option<Rc<Types>> {
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

    fn to_type(&self, env: &Environment) -> Option<Types> {
        match self {
            Data::Integer(i) => Some(Types::Integer(RefCell::new(*i))),
            Data::String(s) => Some(Types::String(RefCell::new(s.clone()))),
            Data::List(datas) => {
                let mut ls = Vec::new();
                for i in datas {
                    ls.push(i.to_type(env)?);
                }
                Some(Types::List(RefCell::new(ls)))
            }
            Data::Symbol(_) => self.get(env).and_then(|s| Some(s.as_ref().clone())),
            Data::None => None,
        }
    }

    fn add(&self, rhs: Rc<Data>) -> Rc<Data> {
        match (self, rhs.as_ref()) {
            (Data::Integer(lhs), Data::Integer(rhs)) => Rc::new(Data::Integer(lhs + rhs)),
            (Data::String(lhs), Data::String(rhs)) => {
                Rc::new(Data::String(format!("{}{}", lhs, rhs)))
            }
            _ => {
                eprintln!("{:#?}", self);
                eprintln!("{:#?}", rhs);
                todo!()
            }
        }
    }

    fn _index(&self, nth: usize) -> Option<Rc<Data>> {
        match self {
            Data::List(lhs) => Some(lhs.get(nth)?.clone()),
            Data::String(lhs) => Some(Rc::new(Data::String(lhs.chars().nth(nth)?.to_string()))),
            _ => None,
        }
    }

    fn index(&self, rhs: Rc<Data>) -> Option<Rc<Data>> {
        match rhs.as_ref() {
            Data::Integer(nth) => self._index(*nth as usize),
            _ => None,
        }
    }

    fn _slice(&self, begin: usize, end: usize) -> Option<Rc<Data>> {
        match self {
            Data::List(lhs) => Some(Rc::new(Data::List(
                lhs.get(begin..end)?
                    .iter()
                    .map(|i| i.clone())
                    .collect::<Vec<Rc<Data>>>(),
            ))),
            Data::String(lhs) => Some(Rc::new(Data::String(
                lhs.get(begin..end)?.chars().collect::<String>(),
            ))),
            _ => None,
        }
    }

    fn slice(&self, begin: Rc<Data>, end: Rc<Data>) -> Option<Rc<Data>> {
        match (begin.as_ref(), end.as_ref()) {
            (Data::Integer(begin), Data::Integer(end)) => {
                self._slice(*begin as usize, *end as usize)
            }
            _ => None,
        }
    }

    fn cmp_equal(&self, rhs: Rc<Data>) -> Rc<Data> {
        match (self, rhs.as_ref()) {
            (Data::Integer(lhs), Data::Integer(rhs)) => {
                Rc::new(Data::Integer((*lhs == *rhs) as i64))
            }
            (Data::String(lhs), Data::String(rhs)) => Rc::new(Data::Integer((*lhs == *rhs) as i64)),
            _ => Rc::new(Data::Integer(0)),
        }
    }

    fn len(&self) -> Option<Rc<Data>> {
        match self {
            Data::List(list) => Some(Rc::new(Data::Integer(list.len() as i64))),
            Data::String(s) => Some(Rc::new(Data::Integer(s.len() as i64))),
            _ => None,
        }
    }

    fn is_zero(&self) -> bool {
        matches!(self, Self::Integer(0))
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    AddAssign,
    Add,
    CmpEqual,
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
    List(Vec<AST>),
}

impl AST {
    pub fn get_data(&self) -> Option<Rc<Data>> {
        match self {
            AST::Data(data) => Some(data.clone()),
            _ => None,
        }
    }

    pub fn eval(&self, env: Rc<Environment>) -> DSLResult<Rc<Data>> {
        eval(self, env) // todo
    }

    pub fn eval_list_nth(&self, env: Rc<Environment>, nth: usize) -> DSLResult<Rc<Data>> {
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

pub fn read_stream<'a>(stream: Stream<'a>) -> DSLData<'a> {
    let new = DSLData::new(stream.source(), stream.stringfiy().to_string());
    new
}

// todo: remove used stream in Source2
pub fn eval_macro<'a>(data: DSLData<'a>, ast: AST) -> Stream<'a> {
    let env = Rc::new(Environment::new());
    let output = run(&ast, env.clone(), data.input).unwrap();
    let begin = Location::new_source(data.source, Source::new(output, "macro"));
    let end = begin.end();
    Stream::new(begin, end)
}

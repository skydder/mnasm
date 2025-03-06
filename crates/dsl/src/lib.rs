use std::{cell::RefCell, collections::HashMap, rc::Rc, str};

use eval::eval;
use util::{Location, Source, Source2, Stream};

mod errors;
pub use errors::{DSLError, DSLResult};

mod tokenizer;
pub use tokenizer::{tokenize, KeyWord, Token, consume_token};

mod parser;
pub use parser::parse;

mod eval;
// data ()
// - dsl code
// - raw stream
// - source info

pub struct DSLData<'a> {
    source: Source2<'a>,
    env: Environment,
}

impl<'a> DSLData<'a> {
    fn new(source: Source2<'a>, raw_stream: String) -> Self {
        Self {
            source: source,
            env: Environment::new(raw_stream),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Types {
    String(RefCell<String>),
    List(RefCell<Vec<Types>>),
    Integer(RefCell<i64>),
    Char(RefCell<char>),
}

impl Types {
    fn add<'a>(&self, rhs: &Data) {
        match (self, rhs) {
            (Types::String(s1), Data::String(s2)) => {
                // eprintln!("{:?} +  {}", s1, s2);
                s1.borrow_mut().push_str(s2);
            }
            (Types::String(s1), Data::Char(s2)) => {
                s1.borrow_mut().push(*s2);
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
            Types::Char(ref_cell) => Data::Char(ref_cell.borrow().clone()),
        }
    }

    fn get_string(&self) -> Option<String> {
        match self {
            Types::String(str) => Some(str.borrow().to_string()),
            _ => None,
        }
    }
}

pub struct Environment {
    variables: RefCell<HashMap<String, Rc<Types>>>,
    input: (Rc<Types>, RefCell<usize>),
    output: Rc<Types>,
}

impl Environment {
    pub fn new(input: String) -> Self {
        Self {
            variables: RefCell::new(HashMap::new()),
            input: (Rc::new(Types::String(RefCell::new(input))), RefCell::new(0)),
            output: Rc::new(Types::String(RefCell::new(String::new()))),
        }
    }
    pub fn get_variable(&self, name: &str, env: &Environment) -> Option<Rc<Types>> {
        match name {
            "output" => Some(self.output.clone()),
            "input" => Some(self.input.0.clone()),
            n => {
                eprintln!("parce que: {}", name);
                env.variables.borrow().get(&n.to_string()).cloned()
            }
        }
    }

    pub fn get_output(&self) -> String {
        self.output.get_string().unwrap()
    }
}

#[derive(Clone, Debug)]
pub enum Data {
    Integer(i64),
    String(String),
    Char(char),
    List(Vec<Rc<Data>>),
    Symbol(String),
    None,
}
impl Data {
    fn get(&self, env: &Environment) -> Option<Rc<Types>> {
        match self {
            Self::Symbol(name) => env.get_variable(name, env),
            _ => None,
        }
    }

    fn add(&self, rhs: Rc<Data>) -> Rc<Data> {
        match (self, rhs.as_ref()) {
            (Data::Integer(lhs), Data::Integer(rhs)) => Rc::new(Data::Integer(lhs + rhs)),
            (Data::String(lhs), Data::String(rhs)) => {
                Rc::new(Data::String(format!("{}{}", lhs, rhs)))
            }
            (Data::String(lhs), Data::Char(rhs)) => {
                Rc::new(Data::String(format!("{}{}", lhs, rhs)))
            }
            _ => todo!(),
        }
    }

    fn _index(&self, nth: usize) -> Option<Rc<Data>> {
        match self {
            Data::List(lhs) => Some(lhs.get(nth)?.clone()),
            Data::String(lhs) => {
                Some(Rc::new(Data::Char(lhs.chars().nth(nth)?)))
            }
            _ => None,
        }
    }

    fn index(&self, rhs: Rc<Data>) -> Option<Rc<Data>> {
        match (self, rhs.as_ref()) {
            (Data::List(lhs), Data::Integer(rhs)) => Some(lhs.get(*rhs as usize)?.clone()),
            (Data::String(lhs), Data::Integer(rhs)) => {
                Some(Rc::new(Data::Char(lhs.chars().nth(*rhs as usize)?)))
            }
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    AddAssign,
    Add,
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

    pub fn eval(&self, env: &Environment) -> DSLResult<Rc<Data>> {
        eval(self, env)
    }
}

pub fn read_stream<'a>(stream: Stream<'a>) -> DSLData<'a> {
    let new = DSLData::new(stream.source(), stream.stringfiy().to_string());
    new
}

// todo: remove used stream in Source2
pub fn eval_macro<'a>(data: DSLData<'a>, ast: AST) -> Stream<'a> {
    let _ = ast.eval(&data.env);
    let output = data.env.get_output();
    eprintln!("out: {}", output);
    let begin = Location::new_source(data.source, Source::new(output, "macro"));
    let end = begin.end();
    Stream::new(begin, end)
}

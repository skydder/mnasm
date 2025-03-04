use std::{cell::RefCell, collections::HashMap, rc::Rc, str};

use util::{Location, Source, Source2, Stream};


mod tokenizer;
pub use tokenizer::{parse, tokenize, Token, KeyWord};

// data ()
// - dsl code
// - raw stream
// - source info

pub struct DSLData<'a> {
    source: Source2<'a>,
    env: Environment

}

impl<'a> DSLData<'a> {
    fn new(source: Source2<'a>, raw_stream: String) -> Self {
        Self {
            source: source,
            env: Environment::new(raw_stream)
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
            },
            (Types::String(s1), Data::Char(s2)) => {
                s1.borrow_mut().push(*s2);
            },
            _ => {
                todo!()
            }
        }
    }

    fn to_data(&self) -> Data {
        match self {
            Types::String(ref_cell) => Data::String(ref_cell.borrow().clone()),
            Types::List(ref_cell) => Data::List(ref_cell.borrow().iter().map(|i|i.to_data()).collect::<Vec<Data>>().clone()),
            Types::Integer(ref_cell) => Data::Integer(ref_cell.borrow().clone()),
            Types::Char(ref_cell) => Data::Char(ref_cell.borrow().clone()),
        }
    }

    fn get_string(&self) -> Option<String> {
        match self {
            Types::String(str) => Some(str.borrow().to_string()),
            _ => None
        }
    }
}


pub struct Environment {
    variables: RefCell<HashMap<String, Rc<Types>>>,
    input: (String, RefCell<usize>),
    output: Rc<Types>,
}

impl Environment {
    pub fn new(input: String) -> Self {
        Self {
            variables: RefCell::new(HashMap::new()),
            input: (input, RefCell::new(0)),
            output: Rc::new(Types::String(RefCell::new(String::new()))),
        }
    }
    pub fn get_variable(&self, name: &str, env: &Environment) -> Option<Rc<Types>> {
        match name {
            "output" => Some(self.output.clone()),
            n => {
                // eprintln!("parce que");
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
    List(Vec<Data>),
    Symbol(String),
    None,
}
impl Data {
    fn get(&self, env: &Environment) -> Option<Rc<Types>> {
        match self {
            Self::Symbol(name) => {
                env.get_variable(name, env)
            }
            _ => None
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    AddAssign,
}

#[derive(Clone, Debug)]
pub enum AST {
    Expr(
        Operator,
        Box<AST>,
        Option<Box<AST>>, // the reason type(rhs) == Option is for unary op
    ),
    Data(Rc<Data>),
}

impl AST {
    fn get_data(&self) -> Option<Rc<Data>> {
        match self {
            AST::Data(data) => Some(data.clone()),
            _ => None
        }
    }

    pub fn eval(&self, env: &Environment) -> Rc<Data> {
        match self {
            AST::Data(data) => {
                match data.as_ref() {
                    Data::Symbol(name) => {
                        Rc::new(env.get_variable(&name, env).map(|v| v.to_data()).unwrap())
                    },
                    _ => data.clone()
                }
            },
            AST::Expr(op, lhs, Some(rhs)) => {
                match op {
                    Operator::AddAssign => {
                        let evaled_lhs = lhs.get_data().unwrap().get(env).unwrap();
                        // eprintln!("{:?}", evaled_lhs);
                        evaled_lhs.add(&rhs.eval(env));

                        // eprintln!("{:?}", evaled_lhs);
                        Rc::new(Data::None)
                    }
                }
            }
            _ => todo!()
        }
    }
}

pub fn read_stream<'a>(stream: Stream<'a>) -> DSLData<'a> {
    let new = DSLData::new(stream.source(), stream.stringfiy().to_string());
    new
}

// todo: remove used stream in Source2
pub fn eval_macro<'a>(data: DSLData<'a>, ast: AST) -> Stream<'a> {
    ast.eval(&data.env);
    let output = data.env.get_output();
    // eprintln!("out: {}", output);
    let begin = Location::new_source(
        data.source,
        Source::new(output, "macro"),
    );
    let end = begin.end();
    Stream::new(begin, end)
}

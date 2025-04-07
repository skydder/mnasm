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
mod data;
mod test_implement;
pub use data::{Variable, Constant};

mod asm_tokenizer;
use asm_tokenizer::TKNZR4ASM;

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

#[derive(Clone)]
pub struct Environment<'a> {
    local: RefCell<HashMap<String, Rc<Variable>>>,
    global: Rc<RefCell<HashMap<String, Rc<Variable>>>>,
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
    let env = Rc::new(Environment::new(constant.source));
    let output = run(&ast, env.clone(), constant.input).unwrap();
    let begin = Location::new_source(constant.source, Source::new(output, "macro"));
    let end = begin.end();
    Stream::new(begin, end)
}

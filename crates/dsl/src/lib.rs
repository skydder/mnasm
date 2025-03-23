use std::{cell::RefCell, collections::HashMap, rc::Rc};

use eval::{eval, run};
use util::{Location, Source, Source2, Stream};

mod errors;
pub use errors::{DSLError, DSLResult};

mod tokenizer;
pub use tokenizer::{consume_token, peek_token, tokenize, KeyWord, Token};

mod parser;
pub use parser::parse;

mod data;
mod eval;
// mod test_implement;
pub use data::Data;

mod asm_tokenizer;
// use asm_tokenizer::TKNZR4ASM;

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
            source,
            input: raw_stream,
        }
    }
}

#[derive(Clone)]
pub struct Environment<'a> {
    local: RefCell<HashMap<Rc<String>, Data<'a>>>,
    global: Rc<RefCell<HashMap<Rc<String>, Data<'a>>>>,
    source: Rc<Source2<'a>>,
}

impl<'a> Environment<'a> {
    pub fn new(source: Source2<'a>) -> Self {
        Self {
            local: RefCell::new(HashMap::new()),
            global: Rc::new(RefCell::new(HashMap::new())),
            source: Rc::new(source),
        }
    }
    pub fn get_variable(&self, name: Rc<String>) -> DSLResult<Data<'a>> {
        match name.clone() {
            n if self.global.borrow().contains_key(&n) => {
                self.global.borrow().get(&n.to_string()).cloned()
            }
            n if self.local.borrow().contains_key(&n) => {
                self.local.borrow().get(&n.to_string()).cloned()
            }
            _ => None,
        }
        .ok_or(DSLError::Eval(format!("{} is unfdefined", name)))
    }

    pub fn push_var(&self, name: Rc<String>, constant: Data<'a>) {
        self.local.borrow_mut().insert(name, constant);
    }

    pub fn push_global(&self, name: Rc<String>, constant: Data<'a>) {
        self.global.borrow_mut().insert(name, constant);
    }

    pub fn enter_fn(&self) -> Environment<'a> {
        Environment {
            local: RefCell::new(HashMap::new()),
            global: self.global.clone(),
            source: self.source.clone(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    Assign,
    Add,
    CmpEqual,
    CmpLessThan,
    CmpNoMoreThan,
    Break,
    List,
    LOr,
    LAnd,
    Mul,
    Index,
    If,
    While,
    Len,
    Slice,
    Let,
    Print,
    IsDigit,
    GetDigit,
    Eval,
    TokenizerNew,
    TokenizerPeek,
    TokenizerNext,
    AsmParse,
    FnCall, // fn_name goes to lhs and args go to rhs as list
}

#[derive(Clone, Debug, PartialEq)]
pub enum AST<'a> {
    Expr(
        Operator,
        Rc<AST<'a>>,
        Option<Rc<AST<'a>>>, // the reason type(rhs) == Option is for unary op
    ),
    Data(Rc<Data<'a>>),
    List(Rc<Vec<AST<'a>>>),
}

impl<'a> AST<'a> {
    pub fn get_data(&self) -> Option<Rc<Data<'a>>> {
        match self {
            AST::Data(constant) => Some(constant.clone()),
            _ => None,
        }
    }

    pub fn get_list(&self) -> Option<Rc<Vec<AST<'a>>>> {
        match self {
            AST::List(l) => Some(l.clone()),
            _ => None,
        }
    }

    pub fn get_list_nth(&self, nth: usize) -> Option<AST<'a>> {
        match self {
            AST::List(l) => l.clone().get(nth).cloned(),
            _ => None,
        }
    }

    pub fn eval_list_nth(&self, env: &Environment<'a>, nth: usize) -> DSLResult<Data<'a>> {
        self.get_list()
            .and_then(|asts| Some(eval(env, asts.get(nth)?)))
            .ok_or(DSLError::Eval(format!("")))?
    }
}

pub fn read_stream<'a>(stream: Stream<'a>) -> DSLConstant<'a> {
    let new = DSLConstant::new(stream.source(), stream.stringfiy().to_string());
    new
}

// todo: remove used stream in Source2
pub fn eval_macro<'a>(constant: DSLConstant<'a>, ast: AST<'a>) -> Stream<'a> {
    let env = Rc::new(Environment::new(constant.source));
    let output = run(&ast, env.clone(), constant.input).unwrap();
    let begin = Location::new_source(constant.source, Source::new(output, "macro"));
    let end = begin.end();
    Stream::new(begin, end)
}

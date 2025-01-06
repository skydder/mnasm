use util::Location;

use tokenizer::{self, Token};

use crate::{Analyze, Codegen, Object};

use super::Stmt;

mod let_macro;

#[derive(Debug)]
pub struct Macro<'a> {
    stream: Vec<Token<'a>>,
    location: Location<'a>,
    // arg: Vec<()>
}

impl<'a> Macro<'a> {
    pub fn new(location: Location<'a>) -> Self {
        Self { stream: Vec::new() , location: location }
    }
}

impl<'a> Object for Macro<'a> {}
impl<'a> Analyze for Macro<'a> {
    fn analyze(&self) {
        todo!()
    }
}
impl<'a> Codegen for Macro<'a> {
    fn codegen(&self) -> String {
        String::new()
    }
}

impl<'a> Stmt<'a> for Macro<'a>  {
    fn kind(&self) -> super::StmtKind {
        super::StmtKind::Macro
    }
}
use util::Location;


use crate::{Analyze, Codegen, Label, Object};

use super::Stmt;

mod let_macro;

#[derive(Debug)]
pub struct Macro<'a> {
    stream: (Location<'a>, Location<'a>),
    args: Vec<Label<'a>>,
    location: Location<'a>,
}

impl<'a> Macro<'a> {
    pub fn new(
        location: Location<'a>,
        args: Vec<Label<'a>>,
        stream: (Location<'a>, Location<'a>),
    ) -> Self {
        Self {
            // stream: Box::new(stream.clone()),
            stream: stream,
            args: args,
            location: location,
        }
    }

    pub fn ingredients_of_tokenizer(&self) -> (Location<'a>, Location<'a>) {
        self.stream
    }

    pub fn location(&self) -> Location<'a> {
        self.location
    }
}

impl<'a> Object for Macro<'a> {}
impl<'a> Analyze for Macro<'a> {
    fn analyze(&self) {
        // eprintln!("analyzed");
    }
}
impl<'a> Codegen for Macro<'a> {
    fn codegen(&self) -> String {
        String::new()
    }
}

impl<'a> Stmt<'a> for Macro<'a> {
    fn kind(&self) -> super::StmtKind {
        super::StmtKind::Macro
    }
}

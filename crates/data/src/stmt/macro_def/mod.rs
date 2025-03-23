use util::{Location, Stream};

use crate::{Analyze, Codegen, Object};

use super::Stmt;

mod let_macro;

#[derive(Debug)]
pub struct Macro<'a> {
    stream: (Location<'a>, Location<'a>),
    pub args: Vec<&'a str>,
    location: Location<'a>,
}

impl<'a> Macro<'a> {
    pub fn new(
        location: Location<'a>,
        args: Vec<&'a str>,
        stream: (Location<'a>, Location<'a>),
    ) -> Self {
        Self {
            // stream: Box::new(stream.clone()),
            stream,
            args,
            location,
        }
    }

    pub fn ingredients_of_tokenizer(&self) -> Stream<'a> {
        Stream::new(self.stream.0, self.stream.1)
    }

    pub fn location(&self) -> Location<'a> {
        self.location
    }
}

impl Object for Macro<'_> {}
impl Analyze for Macro<'_> {
    fn analyze(&self) {
        // eprintln!("analyzed");
    }
}
impl Codegen for Macro<'_> {
    fn codegen(&self) -> String {
        String::new()
    }
}

impl<'a> Stmt<'a> for Macro<'a> {
    fn kind(&self) -> super::StmtKind {
        super::StmtKind::Macro
    }
}

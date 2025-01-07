use std::rc::Rc;

use util::Location;

use tokenizer::{self, Token};

use crate::{Analyze, Codegen, Object};

use super::Stmt;

mod let_macro;

#[derive(Debug)]
pub struct Macro<'a> {
    stream: Rc<Vec<Token<'a>>>,
    location: Location<'a>,
    // arg: Vec<()>
}

impl<'a> Macro<'a> {
    pub fn new(location: Location<'a>, stream: Rc<Vec<Token<'a>>>) -> Self {
        Self {
            stream: stream,
            location: location,
        }
    }
}

// impl TokenGenerator for MacroTokenizer {
//      fn location() {
//          self.location
//      }
//      fn peek_token() {self.stream.peek()}
//      fn next_token() {self.stream.next()}
//      fn skip_space() {while self.peek_token() == Tokenkind::Space {self.next_token()}}
// }

impl<'a> Object for Macro<'a> {}
impl<'a> Analyze for Macro<'a> {
    fn analyze(&self) {
        eprintln!("analyzed");
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

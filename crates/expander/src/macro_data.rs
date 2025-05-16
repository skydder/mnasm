use std::{cell::RefCell, collections::HashMap, env::Args, rc::Rc};

use data::{Ast, Ident};
use util::TokenKind;

#[derive(Debug, Clone)]
pub enum Expander {
    Replace((Rc<Vec<TokenKind>>, Rc<Vec<TokenKind>>)), // args, stream
    Definition
}

impl Expander {
    pub fn expand<'code>(self, stream: Vec<TokenKind>) -> Ast<'code> {
        match self {
            Expander::Definition => {
                todo!()
            }
            Expander::Replace((arg, def_stream)) => {
                todo!()
            }
        }
    }
}

pub struct MacroData {
    definition: RefCell<HashMap<Ident, Expander>>
}

impl MacroData {
    pub fn new() -> Self {
        Self { definition: RefCell::new(HashMap::new()) }
    }

    pub fn get(&self, name: Ident) -> Option<Expander> {
        self.definition.borrow().get(&name).cloned()
    }

    pub fn register_macro(&self, name: Ident, expander: Expander) {
        self.definition.borrow_mut().insert(name, expander);
    }
}
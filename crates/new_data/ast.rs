use std::{cell::RefCell, rc::{Rc, Weak}};

use util::{AsmError, Location};

use super::{
    operand::{Immediate, Label, Memory, Register},
    path::Path,
};

pub enum Ast<'code> {
    Ins(Label<'code>, Vec<Ast<'code>>),
    Label(Path<'code>),
    LabelDef(Label<'code>, Box<Ast<'code>>),
    Block(Vec<Ast<'code>>),
    BlockLessBlock(Vec<Ast<'code>>),
    Macro(Label<'code>, Box<Ast<'code>>, Vec<Label<'code>>),
    Register(Register<'code>),
    Memory(Memory<'code>),
    Immediate(Immediate<'code>),
}

#[allow(clippy::needless_lifetimes)]
impl<'code> Ast<'code> {
    pub fn is_operand(&self) -> bool {
        matches!(
            self,
            Ast::Label(..) | Ast::Immediate(..) | Ast::Memory(..) | Ast::Register(..)
        )
    }

    pub fn is_block(&self) -> bool {
        matches!(self, Ast::Block(..))
    }

    pub fn location(&self) -> Location<'code> {
        todo!()
    }

    pub fn print_ast(&self) -> String {
        match self {
            Ast::Ins(label, asts) => {
                // format!("{}(", label, )
            }
            Ast::Label(path) => todo!(),
            Ast::Block(asts) => todo!(),
            Ast::BlockLessBlock(asts) => todo!(),
            Ast::Macro(label, ast, labels) => todo!(),
            Ast::Register(register) => todo!(),
            Ast::Memory(memory) => todo!(),
            Ast::Immediate(immediate) => todo!(),
            Ast::LabelDef(..) => todo!(),
        }
        todo!()
    }
}

pub struct Scope<'code> {
    global: Option<Rc<Scope<'code>>>,
    name: Label<'code>,
    in_scope: RefCell<Vec<Rc<Scope<'code>>>>,
}

impl<'code> Scope<'code> {
    pub fn new(global: Rc<Scope<'code>>, name: Label<'code>) -> Rc<Self> {
        Rc::new(Self {
            global: Some(global),
            name,
            in_scope: RefCell::new(Vec::new()),
        })
    }

    fn new_global(name: Label<'code>) -> Rc<Self> {
        Rc::new(Self {
            global: None,
            name,
            in_scope: RefCell::new(Vec::new()),
        })
    }

    fn has(&self, name: Label<'code>) -> bool {
        for scope in self.in_scope.borrow().iter() {
            if scope.name == name {
                return true;
            }
        }
        false
    }

    pub fn has_path_of(self: Rc<Self>, path: &Path<'code>) -> bool {
        // recursive
        todo!()
    }
    pub fn add_new_scope(self: Rc<Self>, name: Label<'code>) -> Rc<Scope<'code>> {
        let new = Scope::new(if self.global.is_none() {self.clone()} else {self.global.clone().unwrap()}, name);
        self.in_scope.borrow_mut().push(new.clone());
        new
    }
}

pub fn analyze<'code>(ast: &Ast<'code>, scope: Rc<Scope<'code>>) -> Result<(), AsmError<'code>> {
    match ast {
        Ast::Ins(label, asts) => {
            for op in asts {
                if !op.is_operand() {
                    return Err(AsmError::ParseError(
                        op.location(),
                        String::new(),
                        String::new(),
                    ));
                }
                analyze(op, scope.clone())?;
            }
            Ok(())
        }
        Ast::Label(path) => {
            if !scope.has_path_of(path) {
                Err(AsmError::ParseError(
                    ast.location(),
                    String::new(),
                    String::new(),
                ))
            } else {
                Ok(())
            }
        }
        Ast::LabelDef(label, labeled_ast) => {
            let new = scope.clone().add_new_scope(label.clone()); // nl
            if labeled_ast.is_block() {
                analyze(labeled_ast, new)?;
            } else {
                analyze(labeled_ast, scope)?;
            }
            Ok(())
        }
        Ast::Block(asts) => {
            for a in asts {
                analyze(a, scope.clone())?;
            }
            Ok(())
        }
        Ast::BlockLessBlock(asts) => {
            for a in asts {
                analyze(a, scope.clone())?;
            }
            Ok(())
        }
        Ast::Macro(label, ast, labels) => todo!(),
        Ast::Register(register) => Ok(()),
        Ast::Memory(memory) => Ok(()),
        Ast::Immediate(immediate) => Ok(()),
    }
}

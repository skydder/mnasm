use crate::Stmt;

#[derive(Debug)]
pub struct Block {
    pub indent_depth: usize,
    pub stmts: Vec<Stmt>,
}

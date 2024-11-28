use crate::Stmt;

#[derive(Debug)]
pub struct Block<'a> {
    pub indent_depth: usize,
    pub stmts: Vec<Stmt<'a>>,
}
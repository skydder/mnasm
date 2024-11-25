use crate::Stmt;

pub struct Block<'a> {
    indent_depth: usize,
    stmts: Vec<Stmt<'a>>,
}
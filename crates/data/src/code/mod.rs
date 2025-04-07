use crate::Stmt;

pub struct Code<'a> {
    pub codes: Vec<Box<dyn Stmt<'a> + 'a>>,
}

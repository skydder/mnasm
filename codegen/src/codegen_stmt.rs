use data::Stmt;

pub fn codegen_stmt<'a>(stmt: &Box<dyn Stmt + 'a>) -> String {
    stmt.codegen()
}

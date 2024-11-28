use std::fmt::Write;
use parser::Stmt;

pub fn codegen_stmt(stmt: &Stmt) -> String {
    let mut code = String::new();
    write!(code, "\t{}", stmt.instruction);
    write!(code, "\n");
    code
}
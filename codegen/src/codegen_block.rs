use std::fmt::Write;

use data::Block;

use crate::codegen_stmt;

pub fn codegen_block<'a>(bl: &'a Block<'a>) -> String {
    let mut code = String::new();
    for i in &bl.stmts {
        write!(code, "{}", codegen_stmt(i));
    }
    code
}

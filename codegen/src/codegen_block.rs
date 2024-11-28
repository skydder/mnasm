use std::fmt::Write;

use parser::{parse_stmt, Block};

use crate::codegen_stmt;

pub fn codegen_block(bl: &Block) -> String {
    let mut code = String::new();
    for i in &bl.stmts {
        write!(code, "{}", codegen_stmt(i));
    }
    code
}
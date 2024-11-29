use data::{CompoundIns, Ins, Stmt};
use std::fmt::Write;

fn codegen_ins(ins: &Ins) -> String {
    format!("\t{}\n", ins.instruction)
}

fn codegen_compound_ins(c_ins: &CompoundIns) -> String {
    let mut code = String::new();
    for i in &c_ins.compound {
        code.push_str(&codegen_ins(i));
    }
    code
}

pub fn codegen_stmt(stmt: &Stmt) -> String {
    codegen_compound_ins(&stmt.line)
}

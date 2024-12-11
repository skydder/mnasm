use data::{Code, Stmt};

pub fn codegen_code(code: &Code) -> String {
    let mut res = String::new();
    for l in &code.labels {
        res.push_str(&l.codegen());
    }
    res
}

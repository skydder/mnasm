use data::{Code, Codegen};

pub fn codegen_code(code: &Code) -> String {
    let mut res = String::new();
    for l in &code.labels {
        res.push_str(&l.codegen());
    }
    res
}

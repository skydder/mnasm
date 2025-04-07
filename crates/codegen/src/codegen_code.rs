use data::Code;

pub fn codegen_code(code: &Code) -> String {
    let mut res = String::new();
    for l in &code.codes {
        res.push_str(&l.codegen());
    }
    res
}

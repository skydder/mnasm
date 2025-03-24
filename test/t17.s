@(
    fn parse(tokenizer) {
        let code = "";
        if asm_peek_token(tokenizer) == "@" {
            code += "@";
        }
        let lhs = asm_parse(Operand, tokenizer);
        print("lhs", lhs);
        asm_skip_space(tokenizer);
        let mid = asm_next_token(tokenizer);
        print(mid);
        if mid == "=" {
            asm_skip_space(tokenizer);
            let rhs = asm_parse(Operand, tokenizer);
            if is_none(rhs) {
                if asm_peek_token(tokenizer) == "`" {
                    asm_next_token(tokenizer);
                    let rhs = asm_parse(Operand, tokenizer);
                    if is_none(rhs) {
                        print("erroedfa");
                    } else {
                        code += "mov(" + lhs + ", `" + rhs + ")";
                    }
                } else {
                    print("error");
                }
            } else {
                code += "mov(" + lhs + ", " + rhs + ")";
            }
        } else if mid == "+" {
            if asm_next_token(tokenizer) == "=" {
                asm_skip_space(tokenizer);
                let rhs = asm_parse(Operand, tokenizer);
                if is_none(rhs) {
                    if asm_peek_token(tokenizer) == "`" {
                        asm_next_token(tokenizer);
                        let rhs = asm_parse(Operand, tokenizer);
                        if is_none(rhs) {
                        print("erroedfa");
                    } else {
                        code += "mov(" + lhs + ", `" + rhs + ")";
                    }
                    } else {
                        print("error");
                    }
                } else {
                    code += "add(" + lhs + ", " + rhs + ")";
                }
            } else {
                print("error1");
            }
        } else if mid == "-" {
            if asm_next_token(tokenizer) == "=" {
                asm_skip_space(tokenizer);
                let rhs = asm_parse(Operand, tokenizer);
                if is_none(rhs) {
                    if asm_peek_token(tokenizer) == "`" {
                        asm_next_token(tokenizer);
                        let rhs = asm_parse(Operand, tokenizer);
                        if is_none(rhs) {
                        print("erroedfa");
                    } else {
                        code += "mov(" + lhs + ", `" + rhs + ")";
                    }
                    } else {
                        print("error");
                    }
                } else {
                    code += "sub(" + lhs + ", " + rhs + ")";
                }
            } else {
                print("error");
            }
        } else {
            print("error?");
        }
        eval(code);
    }
    fn main() {
        output += parse(asm_tokenizer(input));
        print(output);
    }
)
<_start:global:.text> {
    @[rax += 200]
    @[rax = 200]
    @[rax -= 300]
}
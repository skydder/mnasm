@(
    fn parse(tokenizer) {
        let code = "";
        let m = "";
        if asm_peek_token(tokenizer) == "@" {
            m += "@";
            asm_next_token(tokenizer);
        }
        let lhs = m + asm_parse(Operand, tokenizer);
        print(lhs);
        asm_skip_space(tokenizer);
        let mid = asm_next_token(tokenizer);
        print(mid);
        if mid == "=" {
            asm_skip_space(tokenizer);
            let rhs = asm_parse(Operand, tokenizer);
            print(3, rhs);
            if is_none(rhs) {
                if asm_peek_token(tokenizer) == "`" {
                    print(1);
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
                print(2);
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

macro if(cond, then, else) {
    `cond
    !jne(.else)
    `then
    !jmp(.end)
    <else>
    `else
    <end>
}

<fizz:global:.data> {
    db("fizz")
    db(10)#5
}

<buzz:global:.data> {
    db("buzz")
    db(10)#5
}

macro print(len, str,) {
    @[rax = 1]
    @[rdi = 1]
    @[rsi = `str]
    @[rdx = `len]
    !syscall()
}

macro exit(code,) {
    @[rax = 60]
    @[rdi = `code]
    !syscall()
}

macro divide1(a, b,) {
    @[rax = `a]
    cqo()
    @[rdi = `b]
    div(rdi)
}

macro divide2(a, b,) {
    @divide1(`a)(`b)
}

macro divide3(a, b,) {
    @divide2(`a)(`b)
}

macro divide(a, b,) {
    @divide3(`a)(`b)
}

macro for(init, cond, inc, loop,) {
    `init
    <start>
    `cond
    !je(.end)
    `loop
    `inc
    !jmp(.start)
    <end>
    #`cond
}

macro l(lhs, rhs,) {
    cmp(`lhs, `rhs)
    #!setl(al)
    nasm("setl al")
    movsx(rax, al)
    cmp(rax, 0)
}

<_start:global:.text> {
    let(counter, r8) #r8:counter
    @for(@[@counter=1])(@l(@counter)(15))(@[@counter+=1]) {
        @divide(counter)(3)

        @if (cmp(rdx, 0)) {
            @print(5)(fizz)
        } {
            @divide(counter)(5)
            @if (cmp(rdx, 0)) {
                @print(5)(buzz)
            }()
        }
    }
    @exit(0)
}

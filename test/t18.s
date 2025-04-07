@(
    fn parse(tokenizer) {
        let code = "";
        let lhs = read_macro(tokenizer);
        asm_skip_space(tokenizer);
        let mid = asm_next_token(tokenizer);
        match mid {
            case "?" {
                asm_skip_space(tokenizer);
                let rhs = read_macro(tokenizer);
                code += "cmp(" + lhs + ", " + rhs + ")";
            }
            case "=" {
                asm_skip_space(tokenizer);
                let rhs = read_macro(tokenizer);
                code += "mov(" + lhs + ", " + rhs + ")";
            }
            case "+" {
                if asm_next_token(tokenizer) == "=" {
                    asm_skip_space(tokenizer);
                    let rhs = read_macro(tokenizer);
                    code += "add(" + lhs + ", " + rhs + ")";
                } else {
                    print("error1");
                }
            }
            case "-" {
                if asm_next_token(tokenizer) == "=" {
                    asm_skip_space(tokenizer);
                    let rhs = read_macro(tokenizer);
                    code += "sub(" + lhs + ", " + rhs + ")";
                } else {
                    print("error");
                }
            }
            case "|" {
                if asm_next_token(tokenizer) == "=" {
                    asm_skip_space(tokenizer);
                    let rhs = read_macro(tokenizer);
                    code += "or(" + lhs + ", " + rhs + ")";
                } else {
                    print("error");
                }
            }
        }

        eval(code);
    }

    fn read_macro(tokenizer) {
        let code = "";
        match asm_peek_token(tokenizer) {
            case "`" {
                asm_next_token(tokenizer);
                code += "`";
            }
            case "@" {
                asm_next_token(tokenizer);
                code += "@";
            }
        }
        let label = asm_parse(Operand, tokenizer);
        if is_none(label) {
            print("error");
        } else {
            code += label;
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

<fizz:global:.data> {
    db("fizz")
    db(10)#5
}

<buzz:global:.data> {
    db("buzz")
    db(10)#5
}

<fizzbuzz:global:.data> {
    db("fizzbuzz")
    db(10)#9
}

<num:global:.data> {
    db("num")
    db(10)#4
}


<_start:global:.text> {
    let(counter, r8) #r8:counter

    @for(@[@counter=1])(@l(@counter)(40))(@[@counter+=1]) {
        let(is_mul3, r9)
        @divide(@counter)(3)
        @[@is_mul3 = rdx]
        
        let(is_mul5, r10)
        @divide(@counter)(5)
        @[@is_mul5 = rdx]

        let(is_mul15, r11)
        @[@is_mul15 = rdx]
        @[@is_mul15 |= @is_mul3]

        @if (@[@is_mul15 ? 0]) {
            @print(9)(fizzbuzz)
        }{
            @if (@[@is_mul5 ? 0]) {
                @print(5)(buzz)
            }{
                @if (@[@is_mul3 ? 0]) {
                    @print(5)(fizz)
                }{
                    @print(4)(num)
                }
            }
        }
    }
    @exit(0)
}

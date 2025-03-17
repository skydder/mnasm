let(a, rax)
let(b, helloworld)
macro if(cond, then, else,) {
    `cond
    jne(else)
    `then
    jmp(end)
    <else>
    `else
    <end>
}

<helloworld:.data> {
    db("Hello world!", 10)
}

<_start:global:.text> {
    @(
        fn tokenize(in) {
            let i = 0;
            let lis = [];
            while i < len(in) {
                if in[i] == " " {
                    i += 1;
                } else if is_digit(in[i]) {
                    let num = 0;
                    while i < len(in) && is_digit(in[i]) {
                        print(i, in[i]);
                        num *= 10;
                        num += get_digit(in[i]);
                        i += 1;
                    }
                    lis += num;
                }
            }
            eval(lis);
        } 
        fn main() {
            print(tokenize(input));
        }
    )
    @[10 200 23 32]
}
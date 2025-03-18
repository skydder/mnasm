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
    @(fn test(x) {
        print("testT", x , 100);
    }

    fn test_while(in) {
        let i = 0;
        while i < len(in) {
            print(i, in[i]);
            i += 1;
        }
        let t = ["test", in, 0, 3, i];
        print(t);
    } 
    fn main() {
        let sys = "!syscall()";
        let mov = "!move()";
        let tes = "!test()";
        test(mov);
        test_while(input);
        if input == "syscall" {
            output += mov;
        } else if input == "move" {
            output += sys;
        } else {
            print(input);
            output += tes;
        }
        print(output);
    }
    )
    mov(@a, 1);mov(rdi, 1);
    mov(rsi, @b)
    mov(rdx, 14)
    @[syscall]
    @[move]
    @[adsfasd]
    mov(rax, 60)
    mov(rdi, 0)
    !syscall()
    #@if(mov(a, 1))(mov(a, 1))(mov(a, 1))
}
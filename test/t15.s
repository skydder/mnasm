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
    @(let sys = "!syscall()";
    let mov = "!move()";
    let tes = "!test()";
    print(len(sys), mov, tes);
    if input == "syscall" {
        output += mov;
    } else if input == "move" {
        output += sys;
    } else {
        print(input);
        output += tes;
    }
    print(output);
    )
    mov(a, 1);mov(rdi, 1);
    mov(rsi, helloworld)
    mov(rdx, 14)
    @[syscall]
    @[move]
    @[adsfasd]
    mov(rax, 60)
    mov(rdi, 0)
    !syscall()
    #@if(mov(a, 1))(mov(a, 1))(mov(a, 1))
}
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
    @(output += "!syscall()")
    mov(a, 1);mov(rdi, 1);
    mov(rsi, helloworld)
    mov(rdx, 14)
    @[test]
    mov(rax, 60)
    mov(rdi, 0)
    !syscall()
    #@if(mov(a, 1))(mov(a, 1))(mov(a, 1))
}
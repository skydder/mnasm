let(a, rax@)
let(b, helloworld@)
macro if(cond, then, else) {
    `cond
    jne(else)
    `then
    jmp(end)
    <else:_>
    `else
    <end:_>
}@:

<helloworld:.data> {
    db("Hello world!")
}

<_start:global:.text> {
    mov(a, 1);mov(rdi, 1);
    mov(rsi, b)
    mov(rdx, 13)
    syscall()
    mov(rax, 60)
    mov(rdi, 0)
        syscall()
    @if(mov(a, 1)@: mov(a, 1)@: mov(a, 1)@:)
}
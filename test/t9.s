let(a, rax@)
let(b, helloworld@)
<helloworld:.data> {
    db("Hello world!")
}

<_start:global:.text> {
    mov(a, 1)
    mov(rdi, 1)
    mov(rsi, b)
    mov(rdx, 13)
    syscall()
    mov(rax, 60)
    mov(rdi, 0)
    syscall()
}
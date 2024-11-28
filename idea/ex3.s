<helloworld:local:data> {
    db("Hello world!")
}

<_start:global:text> {
    mov(rax, 1)
    mov(rdi, 1)
    mov(rsi, helloworld)
    mov(rdx, 13)
    syscall()
    mov(rax, 60)
    mov(rdi, 0)
    syscall()
}
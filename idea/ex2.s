<_start:global:.text> {
    mov(rax, 0)
    mov(rbx, 1)
    <loop> {
        add(rax, rbx)
        add(rbx, 1)
        cmp(rbx, 10)
        jle(loop)
    }
    mov(rdi, rax)
    mov(rax, 60)
    syscall()
}
extern(main)

<_start: global:.text> {
    call(main)
}
<_fini: global:.text> {
    mov(rdi, rax)
    mov(rax, 60)
    syscall()
}
< test> {
    test(rax, rbx), nop(), ret()
    mov(rax, -1), mov(rbx, 100)
    mov(rax, helloworld)
    mov(rax, ptr(rax, _, _, -100))
    mov(rax, ptr(rax, rbx, _, 100))
    mov(rax, ptr(rax, rbx, 4, 100))
    mov(rax, ptr(_, rbx, 4, 100))
    mov(rax, ptr(rax, _, _, _))
    mov(rax, ptr(_, _, _, 100))
    mov(rax, ptr(rax, rbx, _,_ ))
    mov(rax, ptr(rax, rbx, 8, _))
    test()
}
<helloworld:.data> {
    db(1)
}
let(tsa, ptr<byte>(rax, _, _, -100)@)
< test> {
    test(rax, rbx), nop(), ret()
    mov(rax, -1), mov(rbx, 100)
    mov(rax, helloworld)
    mov(al, ptr<byte>(rax, _, _, -100))
    mov(al, tsa)
    mov(eax, ptr<dword>(rax, rbx, _, 100))
    mov(rax, ptr < qword > (rax, rbx, 4, 100))
    mov(rax, ptr(_, rbx, 4, 100))
    mov(rax, ptr(rax, _, _, _))
    mov(rax, ptr(_, _, _, 100))
    mov(rax, ptr(rax, rbx, _,_ ))
    mov(rax, ptr(rax, rbx, 8, _))
    test(rax, rbx)
}
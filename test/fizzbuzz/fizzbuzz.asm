extern(printf, assert_)
<GL_L_L_3 :global :.data> {
    db(110, 117, 109, 10, 0)
}
<GL_L_L_2 :global :.data> {
    db(102, 105, 122, 122, 10, 0)
}
<GL_L_L_1 :global :.data> {
    db(98, 117, 122, 122, 10, 0)
}
<GL_L_L_0 :global :.data> {
    db(102, 105, 122, 122, 98, 117, 122, 122, 10, 0)
}
<main :global :.text> {
    {
        push(rbp), mov(rbp, rsp), sub(rsp, 16)
    }
    #for
    {
        #initialize
        {
            lea(rax, ptr(rbp, _, _, -16))
            push(rax)
            {
                mov(rax, 1)
            }
            pop(rdi), mov(ptr<dword>(rdi, _, _, _), eax)
        }
        <L_begin_1>

        #condition
        {
            {
                mov(rax, 40)
            }
            push(rax)
            {
                lea(rax, ptr(rbp, _, _, -16))
                movsx(rax, ptr<dword>(rax, _, _, _))
            }
            pop(rdi)
            cmp(eax, edi), setl(al), movsx(rax, al)
        }
        cmp(rax, 0), je(.L_end_1)
        #loop
        {
            lea(rax, ptr(rbp, _, _, -12))
            push(rax)
            {
                {
                    mov(rax, 0)
                }
                push(rax)
                {
                    {
                        mov(rax, 3)
                    }
                    push(rax)
                    {
                        lea(rax, ptr(rbp, _, _, -16))
                        movsx(rax, ptr<dword>(rax, _, _, _))
                    }
                    pop(rdi)
                    cdq(), idiv(edi), mov(eax, edx)
                }
                pop(rdi)
                cmp(eax, edi), sete(al), movsx(rax, al)
            }
            pop(rdi), mov(ptr<dword>(rdi, _, _, _), eax)
        }
        {
            lea(rax, ptr(rbp, _, _, -8))
            push(rax)
            {
                {
                    mov(rax, 0)
                }
                push(rax)
                {
                    {
                        mov(rax, 5)
                    }
                    push(rax)
                    {
                        lea(rax, ptr(rbp, _, _, -16))
                        movsx(rax, ptr<dword>(rax, _, _, _))
                    }
                    pop(rdi)
                    cdq(), idiv(edi), mov(eax, edx)
                }
                pop(rdi)
                cmp(eax, edi), sete(al), movsx(rax, al)
            }
            pop(rdi), mov(ptr<dword>(rdi, _, _, _), eax)
        }
        {
            lea(rax, ptr(rbp, _, _, -4))
            push(rax)
            {
                {
                    lea(rax, ptr(rbp, _, _, -8))
                    movsx(rax, ptr<dword>(rax, _, _, _))
                }
                push(rax)
                {
                    lea(rax, ptr(rbp, _, _, -12))
                    movsx(rax, ptr<dword>(rax, _, _, _))
                }
                pop(rdi)
                and(eax, edi)

            }
            pop(rdi), mov(ptr<dword>(rdi, _, _, _), eax)
        }
        #if
        {
            {
                lea(rax, ptr(rbp, _, _, -4))
                movsx(rax, ptr<dword>(rax, _, _, _))
            }
            cmp(rax, 0), je(.L_else_2)
            {
                #function-call
                {
                    mov(rax, GL_L_L_0)
                }
                push(rax)
                pop(rdi)
                mov(rax, 0), call(printf)
            }
            jmp(.L_end_2)
            <L_else_2> {
                #if
                {
                    {
                        lea(rax, ptr(rbp, _, _, -8))
                        movsx(rax, ptr<dword>(rax, _, _, _))
                    }
                    cmp(rax, 0), je(.L_else_3)
                    {
                        #function-call
                        {
                            mov(rax, GL_L_L_1)
                        }
                        push(rax)
                        pop(rdi)
                        mov(rax, 0), call(printf)
                    }
                    jmp(.L_end_3)
                    <L_else_3> {
                        #if
                        {
                            {
                                lea(rax, ptr(rbp, _, _, -12))
                                movsx(rax, ptr<dword>(rax, _, _, _))
                            }
                            cmp(rax, 0), je(.L_else_4)
                            {
                                #function-call
                                {
                                    mov(rax, GL_L_L_2)
                                }
                                push(rax)
                                pop(rdi)
                                mov(rax, 0), call(printf)
                            }
                            jmp(.L_end_4)
                            <L_else_4> {
                                {
                                    #function-call
                                    {
                                        mov(rax, GL_L_L_3)
                                    }
                                    push(rax)
                                    pop(rdi)
                                    mov(rax, 0), call(printf)
                                }
                            }
                            <L_end_4>
                        }
                    }
                    <L_end_3>
                }
            }
            <L_end_2>
        }
        #inc
        {
            lea(rax, ptr(rbp, _, _, -16))
            push(rax)
            {
                {
                    mov(rax, 1)
                }
                push(rax)
                {
                    lea(rax, ptr(rbp, _, _, -16))
                    movsx(rax, ptr<dword>(rax, _, _, _))
                }
                pop(rdi)
                add(eax, edi)
            }
            pop(rdi), mov(ptr<dword>(rdi, _, _, _), eax)
        }
        jmp(.L_begin_1)
        <L_end_1>
    }
    <L_return_main> {
        mov(rsp, rbp), pop(rbp), ret()
    }
}

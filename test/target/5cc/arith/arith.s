<prep :global> {
    extern(printf, assert_)
}
<L_L_26 :global :.data> {
    db(79, 79, 79, 79)
}
<L_L_25 :global :.data> {
    db(49, 49, 49, 49, 49)
}
<L_L_24 :global :.data> {
    db(49, 49, 49, 49, 49)
}
<L_L_23 :global :.data> {
    db(49, 49, 49, 49, 49)
}
<L_L_22 :global :.data> {
    db(49, 49, 49, 49)
}
<L_L_21 :global :.data> {
    db(49, 49, 49, 49)
}
<L_L_20 :global :.data> {
    db(49, 49, 49, 49)
}
<L_L_19 :global :.data> {
    db(50, 50, 50, 50, 50)
}
<L_L_18 :global :.data> {
    db(49, 49, 49, 49, 49)
}
<L_L_17 :global :.data> {
    db(48, 48, 48, 48, 48)
}
<L_L_16 :global :.data> {
    db(50, 50, 50, 50)
}
<L_L_15 :global :.data> {
    db(49, 49, 49, 49)
}
<L_L_14 :global :.data> {
    db(48, 48, 48, 48)
}
<L_L_13 :global :.data> {
    db(52, 52, 52, 52, 52, 52, 52)
}
<L_L_12 :global :.data> {
    db(48, 48, 48, 48, 48)
}
<L_L_11 :global :.data> {
    db(52, 52, 52, 52, 52, 52, 52)
}
<L_L_10 :global :.data> {
    db(48, 48, 48, 48, 48)
}
<L_L_9 :global :.data> {
    db(45, 45, 45, 45, 45, 45, 45, 45)
}
<L_L_8 :global :.data> {
    db(45, 45, 45, 45, 45, 45)
}
<L_L_7 :global :.data> {
    db(45, 45, 45, 45, 45, 45, 45)
}
<L_L_6 :global :.data> {
    db(40, 40, 40, 40, 40, 40, 40, 40)
}
<L_L_5 :global :.data> {
    db(53, 53, 53, 53, 53, 53, 53, 53)
}
<L_L_4 :global :.data> {
    db(53, 53, 53, 53, 53, 53)
}
<L_L_3 :global :.data> {
    db(49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49)
}
<L_L_2 :global :.data> {
    db(53, 53, 53, 53, 53, 53, 53)
}
<L_L_1 :global :.data> {
    db(52, 52, 52)
}
<L_L_0 :global :.data> {
    db(48, 48)
}
<main :global :.text> {
    {
        push(rbp)
        mov(rbp, rsp)
        sub(rsp, 0)
    }
    {
    }
    #function-call
    {
        mov(rax, 0)
        push(rax)
        mov(rax, 0)
        push(rax)
        {
            mov(rax, L_L_0)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 42)
        push(rax)
        mov(rax, 42)
        push(rax)
        {
            mov(rax, L_L_1)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 21)
        push(rax)
        mov(rax, 4)
        push(rax)
        mov(rax, 20)
        push(rax)
        mov(rax, 5)
        pop(rdi)
        add(rax, rdi)
        pop(rdi)
        sub(rax, rdi)
        push(rax)
        {
            mov(rax, L_L_2)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 41)
        push(rax)
        mov(rax, 5)
        push(rax)
        mov(rax, 34)
        push(rax)
        mov(rax, 12)
        pop(rdi)
        add(rax, rdi)
        pop(rdi)
        sub(rax, rdi)
        push(rax)
        {
            mov(rax, L_L_3)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 47)
        push(rax)
        mov(rax, 7)
        push(rax)
        mov(rax, 6)
        pop(rdi)
        imul(rax, rdi)
        push(rax)
        mov(rax, 5)
        pop(rdi)
        add(rax, rdi)
        push(rax)
        {
            mov(rax, L_L_4)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 15)
        push(rax)
        mov(rax, 6)
        push(rax)
        mov(rax, 9)
        pop(rdi)
        sub(rax, rdi)
        push(rax)
        mov(rax, 5)
        pop(rdi)
        imul(rax, rdi)
        push(rax)
        {
            mov(rax, L_L_5)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 4)
        push(rax)
        mov(rax, 2)
        push(rax)
        mov(rax, 5)
        push(rax)
        mov(rax, 3)
        pop(rdi)
        add(rax, rdi)
        pop(rdi)
        cqo(), idiv(rdi)
        push(rax)
        {
            mov(rax, L_L_6)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 10)
        push(rax)
        mov(rax, 20)
        push(rax)
        mov(rax, 10)
        neg(rax)
        pop(rdi)
        add(rax, rdi)
        push(rax)
        {
            mov(rax, L_L_7)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 10)
        push(rax)
        mov(rax, 10)
        neg(rax)
        neg(rax)
        push(rax)
        {
            mov(rax, L_L_8)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 10)
        push(rax)
        mov(rax, 10)
        neg(rax)
        neg(rax)
        push(rax)
        {
            mov(rax, L_L_9)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 0)
        push(rax)
        mov(rax, 1)
        push(rax)
        mov(rax, 0)
        pop(rdi)
        cmp(rax, rdi), sete(al), movzb(rax, al)
        push(rax)
        {
            mov(rax, L_L_10)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 1)
        push(rax)
        mov(rax, 42)
        push(rax)
        mov(rax, 42)
        pop(rdi)
        cmp(rax, rdi), sete(al), movzb(rax, al)
        push(rax)
        {
            mov(rax, L_L_11)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 1)
        push(rax)
        mov(rax, 1)
        push(rax)
        mov(rax, 0)
        pop(rdi)
        cmp(rax, rdi), setne(al), movzb(rax, al)
        push(rax)
        {
            mov(rax, L_L_12)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 0)
        push(rax)
        mov(rax, 42)
        push(rax)
        mov(rax, 42)
        pop(rdi)
        cmp(rax, rdi), setne(al), movzb(rax, al)
        push(rax)
        {
            mov(rax, L_L_13)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 1)
        push(rax)
        mov(rax, 1)
        push(rax)
        mov(rax, 0)
        pop(rdi)
        cmp(rax, rdi), setl(al), movzb(rax, al)
        push(rax)
        {
            mov(rax, L_L_14)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 0)
        push(rax)
        mov(rax, 1)
        push(rax)
        mov(rax, 1)
        pop(rdi)
        cmp(rax, rdi), setl(al), movzb(rax, al)
        push(rax)
        {
            mov(rax, L_L_15)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 0)
        push(rax)
        mov(rax, 1)
        push(rax)
        mov(rax, 2)
        pop(rdi)
        cmp(rax, rdi), setl(al), movzb(rax, al)
        push(rax)
        {
            mov(rax, L_L_16)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 1)
        push(rax)
        mov(rax, 1)
        push(rax)
        mov(rax, 0)
        pop(rdi)
        cmp(rax, rdi), setle(al), movzb(rax, al)
        push(rax)
        {
            mov(rax, L_L_17)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 1)
        push(rax)
        mov(rax, 1)
        push(rax)
        mov(rax, 1)
        pop(rdi)
        cmp(rax, rdi), setle(al), movzb(rax, al)
        push(rax)
        {
            mov(rax, L_L_18)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 0)
        push(rax)
        mov(rax, 1)
        push(rax)
        mov(rax, 2)
        pop(rdi)
        cmp(rax, rdi), setle(al), movzb(rax, al)
        push(rax)
        {
            mov(rax, L_L_19)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 1)
        push(rax)
        mov(rax, 1)
        push(rax)
        mov(rax, 0)
        pop(rdi)
        cmp(rax, rdi), setl(al), movzb(rax, al)
        push(rax)
        {
            mov(rax, L_L_20)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 0)
        push(rax)
        mov(rax, 1)
        push(rax)
        mov(rax, 1)
        pop(rdi)
        cmp(rax, rdi), setl(al), movzb(rax, al)
        push(rax)
        {
            mov(rax, L_L_21)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 0)
        push(rax)
        mov(rax, 1)
        push(rax)
        mov(rax, 2)
        pop(rdi)
        cmp(rax, rdi), setl(al), movzb(rax, al)
        push(rax)
        {
            mov(rax, L_L_22)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 1)
        push(rax)
        mov(rax, 1)
        push(rax)
        mov(rax, 0)
        pop(rdi)
        cmp(rax, rdi), setle(al), movzb(rax, al)
        push(rax)
        {
            mov(rax, L_L_23)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 1)
        push(rax)
        mov(rax, 1)
        push(rax)
        mov(rax, 1)
        pop(rdi)
        cmp(rax, rdi), setle(al), movzb(rax, al)
        push(rax)
        {
            mov(rax, L_L_24)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        mov(rax, 0)
        push(rax)
        mov(rax, 1)
        push(rax)
        mov(rax, 2)
        pop(rdi)
        cmp(rax, rdi), setle(al), movzb(rax, al)
        push(rax)
        {
            mov(rax, L_L_25)
        }
        push(rax)
        pop(rdx)
        pop(rsi)
        pop(rdi)
        mov(rax, 0)
        call(assert_)
    }
    #function-call
    {
        {
            mov(rax, L_L_26)
        }
        push(rax)
        pop(rdi)
        mov(rax, 0)
        call(printf)
    }
    {
        mov(rax, 0)
        jmp(L_return_main)
    }
    <L_return_main> {
        mov(rsp, rbp)
        pop(rbp)
        ret()
    }
}

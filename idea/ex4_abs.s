<abs:global:text> {
    # -------------------
    # if (rdi < 0) {
    #     rdi = -rdi;
    # }
    # return rdi
    # -------------------
    cmp(rdi, 0)
    jl(minus)
    jmp(end)
    <minus:_:_> {
        neg(rdi)
    }
    <end:_:_>
    mov(rax, rdi)
    ret()
}

<abs2:global:text> {
    # -------------------
    # if (rdi < 0) {
    #     rdi = -rdi;
    # }
    # return rdi
    # -------------------
    {
        cmp(rdi, 0), jl(minus), jmp(end)
        <minus:_:_> {
            neg(rdi)
        }
        <end:_:_>
    }
    mov(rax, rdi)
    ret()
}

<abs:global:text> {
    # -------------------
    # if (rdi < 0) {
    #     rdi = -rdi;
    # }
    # return rdi
    # -------------------
    {
        cmp(rdi, 0)
        jl(minus)
        jmp(end)
        <minus:_:_> {
            neg(rdi)
        }
        <end:_:_>
    }
    mov(rax, rdi)
    ret()
}

// macro-def (grammar not determined)
if ($0, $1, $(cc)) {
    $(code)
}
=>
{
    cmp($0, $1)
    j$(cc) _if
    jmp(else)
    <_if:_:_> {
        $(code)
    }
    <_else:_:>
}
// macro above might be usefull

<abs4:global:text> {
    # -------------------
    # if (rdi < 0) {
    #     rdi = -rdi;
    # }
    # return rdi
    # -------------------
    if! (rdi, 0, le) {
        neg(rdi)
    }!
    mov(rax, rdi)
    ret()
}
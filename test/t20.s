@def_macro(if(cond, then, else,) => {
    `cond
    jne(.else)
    `then
    jmp(.end)
    <else>
    `else
    <end>
})

<fizz:global:.data> {
    db("fizz")
    db(10)#5
}

<buzz:global:.data> {
    db("buzz")
    db(10)#5
}

@def_macro(print(str, len,) => {
    mov(rax, 1)
    mov(rdi, 1)
    mov(rsi, `str)
    mov(rdx, `len)
    syscall()
})

@def_macro(exit() =>{
    mov(rax, 60)
    mov(rdi, 0)
    syscall()
})

@def_macro(divide(a, b,)=> {
    mov(rax, `a)
    cqo()
    mov(rdi, `b)
    div(rdi)
})

@def_macro(divide1(a, b,)=> {
    @divide(`a, `b)
})

@def_macro(divide2(a, b,)=> {
    @divide1(`a, `b)
})


<_start:global:.text> {
    #r8:counter
    @for( i in (test, mov, s) => {
        `i()
        `i(1)
        `i(1, 2)
    })
}
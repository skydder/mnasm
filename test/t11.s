macro if(cond, then, else,) {
    `cond
    jne(.else)
    `then
    jmp(.end)
    <else>
    `else
    <end>
}

<fizz:global:.data> {
    db("fizz")
    db(10)#5
}

<buzz:global:.data> {
    db("buzz")
    db(10)#5
}

macro print(len, str,) {
    mov(rax, 1);
    mov(rdi, 1);
    mov(rsi, `str)
    mov(rdx, `len)
    syscall()
}

macro exit() {
    mov(rax, 60)
    mov(rdi, 0)
    syscall()
}

macro exit2() mov(rax, 60);mov(rdi, 0);syscall()

macro divide(a, b,) {
    mov(rax, `a)
    cqo()
    mov(rdi, `b)
    div(rdi)
}

<_start:global:.text> {
    let(counter, r8) #r8:counter
    mov(counter, 1)
    <loop> {
        @divide(counter)(3)

        @if (cmp(rdx, 0)) {
            @print(5)(fizz)
        } {
            @divide(counter)(5)
            @if (cmp(rdx, 0)) {
                @print(5)(buzz)
            }()
        }
        add(counter, 1)
        cmp(counter, 15)
        jl(_start.loop)
    }
    @exit2()
}

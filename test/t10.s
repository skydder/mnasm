macro if(cond, then, else,) {
    `cond
    jne(.else)
    `then
    jmp(.end)
    <else>
    `else
    <end>
}@:

<fizz:global:.data> {
    db("fizz")
    db(10)#5
}

<buzz:global:.data> {
    db("buzz")
    db(10)#5
}

macro print(str, len,) {
    mov(rax, 1);
    mov(rdi, 1);
    mov(rsi, `str)
    mov(rdx, `len)
    syscall()
}@:

macro exit() {
    mov(rax, 60)
    mov(rdi, 0)
    syscall()
}@:

macro divide(a, b,) {
    mov(rax, `a)
    cqo()
    mov(rdi, `b)
    div(rdi)
}@:

<_start:global:.text> {
    #r8:counter
    mov(r8, 1)
    <loop> {
        @divide(r8@: 3@:)

        @if(cmp(rdx, 0)@: {
            @print(fizz@: 5@:)
        }@: {
            @divide(r8@: 5@:)
            @if(cmp(rdx, 0)@: {
                @print(buzz@: 5@:)
            }@:@:)
        }@:)
        add(r8, 1)
        cmp(r8, 15)
        jl(_start.loop)
    }
    @exit()
}
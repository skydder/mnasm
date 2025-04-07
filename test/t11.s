macro if(cond, then, else,) {
    `cond
    jne!(.else)
    `then
    jmp!(.end)
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
    @[rax = 1]
    @[rdi = 1]
    @[rsi = `str]
    @[rdx = `len]
    syscall!()
}

macro exit(code,) {
    @[rax = 60]
    @[rdi = `code]
    syscall!()
}

macro divide(a, b,) {
    @[rax = `a]
    cqo()
    @[rdi = `b]
    div(rdi)
}

macro for(init, cond, inc, loop,) {
    `init
    <start>
    `cond
    je!(.end)
    `loop
    `inc
    jmp!(.start)
    <end>
    #`cond
}

macro l(lhs, rhs,) {
    cmp(`lhs, `rhs)
    setl!(al)
    movsx(rax, al)
    cmp(rax, 0)
}

<_start:global:.text> {
    let(counter, r8) #r8:counter
    @for(@[counter=1])(@l(counter)(15))(@[counter+=1]) {
        @divide(counter)(3)

        @if (cmp(rdx, 0)) {
            @print(5)(fizz)
        } {
            @divide(counter)(5)
            @if (cmp(rdx, 0)) {
                @print(5)(buzz)
            }()
        }
    }
    @exit(0)
}

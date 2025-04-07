let(label, rax)
let(max, ptr(rbp, _, _, -20))
// just wandering
def-ins-macro if (cond, then, else) {
    !(cond)
    jne(else)
    !(then)
    jmp(end)
    <else>
    !(else)
    <end>
}
if!([rax >= 0], {
    add(rax, 1)
}, {
    sub(rax, 1)
})

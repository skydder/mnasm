@macro_def(test(a, b) => {
    test(`a, `b)
})

<test:.text> {
    <test1> {
        <test3>
    }
    <test2> {
        <test3>
        jump(::test::test1::test3)
    }
    @test(test, 1)
    extern(pyton)
}
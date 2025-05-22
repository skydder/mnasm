@macro_def(test(a, b) => {
    test(`a, `b)
})

@macro_def(test2() => {
    test()
})

<test:.text> {
    <test1> {
        <test3>
    }
    <test2> {
        <test3>
        jump(::test::test1::test3)
    }
    @test(::test, 1)
    @test2()
    extern(pyton)
}
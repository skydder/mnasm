<test:.text> {
    <test1> {
        <test3>
    }
    <test2> {
        <test3>
        jump(::test::test1::test3)
    }
    extern(pyton)
}
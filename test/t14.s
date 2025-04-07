# from https://en.wikibooks.org/wiki/X86_Assembly/SSE#Arithmetic_example_using_packed_singles
# this is a test for using not supported instructions by raw nasm pseudo-ins 
<v1:global:.data> {
    !dd("1.1", "2.2", "3.3", "4.4")
}
<v2:global:.data> {
    !dd("5.5", "6.6", "7.7", "8.8")
}
<v3:global:.bss> {
    !resd(4)
}

<_start:global:.text>{
    !movups("xmm0", "[v1]")
    !movups("xmm1", "[v2]")   ;#load v2 into xmm1
    
    !addps("xmm0", "xmm1")    ;#add the 4 numbers in xmm1 (from v2) to the 4 numbers in xmm0 (from v1), store in xmm0. for the first float the result will be 5.5+1.1=6.6
    !mulps("xmm0", "xmm1")    ;#multiply the four numbers in xmm1 (from v2, unchanged) with the results from the previous calculation (in xmm0), store in xmm0. for the first float the result will be 5.5*6.6=36.3
    !subps("xmm0", "xmm1")    ;#subtract the four numbers in v2 (in xmm1, still unchanged) from result from previous calculation (in xmm1). for the first float, the result will be 36.3-5.5=30.8
    
    !movups("[v3]", "xmm0")   ;#store v1 in v3
    
    #;end program
    !ret()
}
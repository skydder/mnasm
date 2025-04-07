compile() {
    ./test/5cc-n/5cc test/fizzbuzz/fizzbuzz.c > test/fizzbuzz/fizzbuzz.asm
    cargo run test/fizzbuzz/fizzbuzz.asm -S -o test/fizzbuzz/fizzbuzz.s
    printf "extern printf" >> test/fizzbuzz/fizzbuzz.s
    nasm -f elf64 test/fizzbuzz/fizzbuzz.s -o test/fizzbuzz/fizzbuzz.o
    ld -m elf_x86_64 -o test/fizzbuzz/fizzbuzz test/fizzbuzz/fizzbuzz.o test/fizzbuzz/t.o test/target/5cc/start.o -lc -static --dynamic-linker=/lib64/ld-linux-x86-64.so.2
}

compile
./test/fizzbuzz/fizzbuzz
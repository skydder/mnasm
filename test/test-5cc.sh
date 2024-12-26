# 5cc=test/5cc-n/5cc
# 5cc_test_dir=test/5cc-n/test
# 5cc_test_target=test/target/5cc
# common=test/5cc-n/target/test/common.o

assembl() {
    gcc -o- -E -P -C test/5cc-n/test/$1.c |./test/5cc-n/5cc  -o test/target/5cc/$1/$1.s -
    cargo run test/target/5cc/$1/$1.s -S -o test/target/5cc/$1/$1.nasm
    cargo run test/target/5cc/$1/$1.s -c -o test/target/5cc/$1/$1.o
    ld -m elf_x86_64 -o test/target/5cc/$1/$1  test/target/5cc/start.o test/target/5cc/$1/$1.o test/5cc-n/target/test/common.o -lc -static --dynamic-linker=/lib64/ld-linux-x86-64.so.2
}

prepare() {
    cd test/5cc-n
    make 5cc
    cd ../..
    gcc -c -o test/5cc-n/target/test/common.o test -xc test/5cc-n/test/common
    cargo run test/start.s -c -o test/target/5cc/start.o
}

check() {
    ./test/target/5cc/$1/$1
    if [ $? -eq 0 ]; then
        echo "test $1: PASS"
    else
        echo "test $1: FAIL"
        exit 1
    fi
}

test_5cc() {
    if [ ! -d test/target/5cc/$1 ]; then
        mkdir test/target/5cc/$1
    fi
    assembl $1
    check $1
}

main() {
    prepare
    for file in test/5cc-n/test/*.c; do
        file_name="$(basename $file .c)"
        test_5cc $file_name
    done
    echo "ALL PASS" 
}

main
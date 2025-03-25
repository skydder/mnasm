test: test/t1.s test/t2.s test/t3.s test/t4.s test/t5.s test/t6.s test/t7.s
	cargo run test/t1.s
	cargo run test/t2.s
	cargo run test/t3.s
	cargo run test/t4.s
	cargo run test/t5.s
	cargo run test/t6.s
	cargo run test/t7.s

5cc:
	./test/test-5cc.sh

fizzbuzz:
	./test/fizzbuzz/5cc.sh
.PHONY: test 5cc fizzbuzz
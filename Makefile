.PHONY: see_original see_result gen calc calc_tokio calc_mmap calc_go

see_original:
	hexdump -n 80 -e '1/8 "%.15f\n"' f64.bin

see_result:
	hexdump -n 80 -e '1/8 "%.15f\n"' result

gen:
	cargo run --bin gen --release -- f64.bin

calc:
	cargo run --bin calc --release -- f64.bin

calc_tokio:
	cargo run --bin calc_tokio --release -- f64.bin

calc_mmap:
	cargo run --bin calc_mmap --release -- f64.bin

calc_go:
	cd ./calc_go && go build
	./calc_go/calc_go f64.bin

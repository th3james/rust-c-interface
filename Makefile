all:
	cargo run

build_c_lib:
	clang src/extlib.c -o libext -c

hello: hello.o target/release/libunsafe_tut_core.so
	gcc hello.o -I -l target/release/libunsafe_tut_core.so -o bin/hello

target/release/libunsafe_tut_core.so:
	cargo build --release

hello.o: hello.c
	gcc -c hello.c -L.


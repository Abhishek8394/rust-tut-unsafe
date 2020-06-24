#include <stdio.h>


extern "rust" {
	hello_rust;
}

int main(){
	printf("Welcome to C land pilot!\n");
	printf("Meet my friend Rust\n");
	hello_rust();
}

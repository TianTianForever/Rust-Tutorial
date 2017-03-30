all:dead_code libcrates.rlib

dead_code: dead_code.rs
	rustc dead_code.rs && ./dead_code

libcrates.rlib: crates.rs
	rustc crates.rs

clean:
	rm -rf dead_code libcrates.rlib

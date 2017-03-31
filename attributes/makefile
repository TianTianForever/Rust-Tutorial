all:dead_code libcrates.rlib configuration

dead_code: dead_code.rs
	rustc dead_code.rs && ./dead_code

libcrates.rlib: crates.rs
	rustc crates.rs

configuration: configuration.rs
	rustc configuration.rs && ./configuration
clean:
	rm -rf dead_code libcrates.rlib configuration

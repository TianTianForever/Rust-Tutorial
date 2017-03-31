all:dead_code libcrates.rlib configuration custom

dead_code: dead_code.rs
	rustc dead_code.rs && ./dead_code

libcrates.rlib: crates.rs
	rustc crates.rs

configuration: configuration.rs
	rustc configuration.rs && ./configuration

custom: custom.rs
	rustc --cfg some_condition custom.rs && ./custom
clean:
	rm -rf dead_code libcrates.rlib configuration custom

all: libthree_days_of_rust.rlib executable
	./executable

libthree_days_of_rust.rlib: three_days_of_rust.rs
	rustc --crate-type=lib three_days_of_rust.rs

executable: executable.rs
	rustc executable.rs --extern three_days_of_rust=libthree_days_of_rust.rlib

clearn:
	rm -rf libthree_days_of_rust.rlib executable

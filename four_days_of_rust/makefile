all: libfour_days_of_rust.rlib executable
	./executable

libfour_days_of_rust.rlib: four_days_of_rust.rs
	rustc --crate-type=lib four_days_of_rust.rs

executable: executable.rs
	rustc executable.rs --extern four_days_of_rust=libfour_days_of_rust.rlib

clearn:
	rm -rf libfour_days_of_rust.rlib executable

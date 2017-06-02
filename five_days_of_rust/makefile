OBJECTS = libfive_days_of_rust.rlib beautiful_gril
FLAGS = --crate-type=lib
all: $(OBJECTS)
	./beautiful_gril

libfive_days_of_rust.rlib: five_days_of_rust.rs
	rustc $(FLAGS) five_days_of_rust.rs

beautiful_gril: beautiful_gril.rs
	rustc beautiful_gril.rs --extern five_days_of_rust=libfive_days_of_rust.rlib

.PHONY: clean
clean:
	-rm -rf $(OBJECTS)

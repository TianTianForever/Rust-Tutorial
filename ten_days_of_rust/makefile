OBJECTS = libmanually_implement.rlib ten_days_of_rust \
          generic_function

TARGETS = --crate-type=lib
LTGT = --extern
CC = rustc

all: $(OBJECTS)
	./ten_days_of_rust

libmanually_implement.rlib: manually_implement.rs
	$(CC) $(TARGETS) $<

ten_days_of_rust: ten_days_of_rust.rs
	$(CC) $< $(LTGT) manually_implement=libmanually_implement.rlib

generic_function: generic_function.rs
	$(CC) $< && ./$@
PHONY: clean
clean:
	-rm -rf $(OBJECTS)


OBJECTS = liblifetimes.rlib nine_days_of_rust
TARGETS = --crate-type=lib
LTARGETS = --extern
CC = rustc 

all: $(OBJECTS)
	./nine_days_of_rust

liblifetimes.rlib: lifetimes.rs
	$(CC) $(TARGETS) $<

nine_days_of_rust: nine_days_of_rust.rs
	$(CC) $< $(LTARGETS) lifetimes=liblifetimes.rlib

PHONY: clean
clean:
	-rm -rf $(OBJECTS)

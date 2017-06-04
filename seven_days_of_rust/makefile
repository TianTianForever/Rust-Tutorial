OBJECTS = libassociated_types.rlib seven_days_of_rust
FLAGS = --crate-type=lib
TARGET = --extern
CC = rustc

all: $(OBJECTS)
	./seven_days_of_rust

libassociated_types.rlib: associated_types.rs
	$(CC) $(FLAGS) associated_types.rs

seven_days_of_rust: seven_days_of_rust.rs
	$(CC) $@.rs $(TARGET) associated_types=libassociated_types.rlib

.PHONY: clean
clean:
	-rm -rf $(OBJECTS)


OBJECTS = libmy_lifetimes.rlib eight_days_of_rust
FLAGS = --crate-type=lib
LFLAGS = --extern
CC = rustc

all: $(OBJECTS)
	./eight_days_of_rust

libmy_lifetimes.rlib: my_lifetimes.rs
	$(CC) $(FLAGS) $<

eight_days_of_rust: eight_days_of_rust.rs
	$(CC) $@.rs $(LFLAGS) my_lifetimes=libmy_lifetimes.rlib 

.PHONY: clean
clean:
	-rm -rf $(OBJECTS)

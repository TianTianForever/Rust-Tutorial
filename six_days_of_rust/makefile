OBJECTS = six_days_of_rust
CC = rustc
all: six_days_of_rust
	./six_days_of_rust

six_days_of_rust: six_days_of_rust.rs
	$(CC) $@.rs

.PHONY: clean
clean:
	-rm -rf $(OBJECTS)

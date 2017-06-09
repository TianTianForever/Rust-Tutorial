OBJECTS = libhash_maps.rlib twelve_days_of_rust
TARGETS = --extern
RR = rustc

all: $(OBJECTS)
	./twelve_days_of_rust

libhash_maps.rlib: hash_maps.rs
	$(RR) $<

twelve_days_of_rust: twelve_days_of_rust.rs
	$(RR) $< $(TARGETS) hash_maps=libhash_maps.rlib

PHONY: clean
clean:
	-rm -rf $(OBJECTS)



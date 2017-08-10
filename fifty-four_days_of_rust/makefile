ARCH? = x86_64 linux
OBJECTS = libassociated_type.rlib fifty-four_days_of_rust
C = rustc
L = --extern
.PHONY: clean

run: $(OBJECTS)

libassociated_type.rlib: associated_type.rs
	$(C) $<

fifty-four_days_of_rust: fifty-four_days_of_rust.rs
	$(C) $< $(L) associated_type=libassociated_type.rlib && ./$@

clean:
	-rm -rf $(OBJECTS)

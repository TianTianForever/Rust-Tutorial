ARCH? = x86_64 linux
C = rustc
LINKS = --extern

OBJECTS = liberrors.rlib twenty-three_days_of_rust

.PHONY: run clean

run: $(OBJECTS)

liberrors.rlib: errors.rs
	$(C) $<

twenty-three_days_of_rust: twenty-three_days_of_rust.rs
	$(C) $< $(LINKS) errors=liberrors.rlib && ./$@

clean:
	-rm -rf $(OBJECTS)

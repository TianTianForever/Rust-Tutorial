ARCH? = x86_64
LINKS = --extern
C = rustc

.PHONY: run clean

OBJECTS = liboperators.rlib sixteen_days_of_rust

run: $(OBJECTS)

liboperators.rlib: operators.rs
	$(C) $<

sixteen_days_of_rust: sixteen_days_of_rust.rs
	$(C) $< $(LINKS) operators=liboperators.rlib && ./$@

clean:
	-rm -rf $(OBJECTS)

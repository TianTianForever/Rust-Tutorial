ARCH? = x86_64 linux
LINKS = --extern
R = rustc

.PHONY: run clean

OBJECTS = libclones.rlib nineteen_days_of_rust

run: $(OBJECTS)

libclones.rlib: clones.rs
	$(R) $<

nineteen_days_of_rust: nineteen_days_of_rust.rs
	$(R) $< $(LINKS) clones=libclones.rlib  && ./$@

clean:
	-rm -rf $(OBJECTS)

ARCH?= x86_64

OBJECTS = libiterator.rlib comparison fifteen_days_of_rust
RR = rustc
TARGETS = --crate-type=lib
LINKS = --extern

.PHONY: all clean

all: $(OBJECTS)

libiterator.rlib: iterator.rs
	$(RR) $< $(TARGETS)

comparison: comparison.rs
	$(RR) $< $(LINKS) iterator=libiterator.rlib && ./$@

fifteen_days_of_rust: fifteen_days_of_rust.rs
	$(RR) $< && ./$@

clean:
	-rm -rf $(OBJECTS)


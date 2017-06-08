OBJECTS = libclosure.rlib eleven_days_of_rust \
          match
TARGETS = --crate-type=lib
LTGTS = --extern
RR = rustc

all: $(OBJECTS)
	./eleven_days_of_rust

libclosure.rlib: closure.rs
	$(RR) $< $(TARGETS)

eleven_days_of_rust: eleven_days_of_rust.rs
	$(RR) $< $(LTGTS) closure=libclosure.rlib

match: match.rs
	$(RR) $< && ./$@

PHONT: clean
clean:
	-rm -rf $(OBJECTS)


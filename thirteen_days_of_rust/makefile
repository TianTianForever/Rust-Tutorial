OBJECTS = my_result thirteen_days_of_rust
TARGETS = --extern
LTARGETS = --crate-type=lib
RR = rustc

.PHONY: clean

all: $(OBJECTS)

my_result: my_result.rs
	$(RR) $< && ./$@

thirteen_days_of_rust: thirteen_days_of_rust.rs
	$(RR) $< && ./$@

clean:
	-rm -rf $(OBJECTS)

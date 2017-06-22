ARCH? = x86_64 linux
C = rustc
TAR = --crate-type=lib
L = --extern
.PHONY: run clean 

OBJECTS = liboptions_with_results.rlib twenty-four_days_of_rust

run: $(OBJECTS)

liboptions_with_results.rlib: options_with_results.rs
	$(C) $<

twenty-four_days_of_rust: twenty-four_days_of_rust.rs
	$(C) $< $(L) options_with_results=liboptions_with_results.rlib && ./$@

clean:
	-rm -rf $(OBJECTS)

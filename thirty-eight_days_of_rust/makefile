ARCH? = x86_64 linux

C = rustc

OBJECTS = thirty-eight_days_of_rust reference_cell

.PHONY: clean

all: $(OBJECTS)

thirty-eight_days_of_rust: thirty-eight_days_of_rust.rs
	$(C) $< && ./$@

reference_cell: reference_cell.rs
	$(C) $< && ./$@

clean:
	-rm -rf $(OBJECTS)

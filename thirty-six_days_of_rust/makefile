ARCH? = x86_64 linux

C = rustc

OBJECTS = thirty-six_days_of_rust

.PHONY: clean

run: $(OBJECTS)

thirty-six_days_of_rust: thirty-six_days_of_rust.rs
	$(C) $< && ./$@

clean:
	-rm -rf $(OBJECTS)
